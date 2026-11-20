//! The Miles Sound System's API (AIL 3, DOS), played by the runtime.
//!
//! A DOS game linked Miles, which loaded real-mode drivers for the player's
//! sound card (`.DIG` for samples, `.MDI` for music) chosen with `SETSOUND`.
//! None of that can run here, and players shouldn't set up drivers, so a port
//! replaces the API functions the game calls with mods that call these. They
//! play samples in the mixer, and XMIDI music on the same SoundFont
//! synthesizer as the Windows version's MIDI, as a General MIDI card would.
//!
//! Every function takes Miles' cdecl arguments from the stack, sets `eax` and
//! returns to the caller, who removes the arguments: a mod is one call, such as
//! `miles::start_sample(cpu)`.
//!
//! Callbacks the game registers (end of a stream buffer, XMIDI triggers) are
//! called from the mixer at the point where they fall, as Miles' interrupt
//! handler did, so a callback that loads the next buffer or picks a branch
//! takes effect at once.

use crate::cpu::Cpu;
use rustysynth::Synthesizer;

const RATE: f64 = 44_100.0;
/// XMIDI's fixed clock: Miles plays every sequence at 120 ticks a second.
const TICKS_PER_SECOND: f64 = 120.0;

pub const SMP_DONE: u32 = 2;
pub const SMP_PLAYING: u32 = 4;
pub const SMP_STOPPED: u32 = 8;
pub const SEQ_DONE: u32 = 2;
pub const SEQ_PLAYING: u32 = 4;
pub const SEQ_STOPPED: u32 = 8;

const DIG_F_16BITS: u32 = 1;
const DIG_F_STEREO: u32 = 2;
/// Miles' handles for the two drivers it pretends to have installed.
const DIG_DRIVER: u32 = 0x4d44_0001;
const MDI_DRIVER: u32 = 0x4d44_0002;
/// Handles are small numbers above these, so a stray one is easy to spot.
const SAMPLE_BASE: u32 = 0x5300_0000;
const SEQUENCE_BASE: u32 = 0x5100_0000;
/// The driver type `AIL_MDI_driver_type` reports: General MIDI, the kind of
/// card the built-in bank sounds like.
const GENERAL_MIDI: u32 = 3;

#[derive(Clone, Copy)]
struct Sample {
    status: u32,
    data: u32,
    length: u32,
    format: u32,
    rate: u32,
    volume: u32,
    pan: u32,
    /// Plays left: 0 is for ever.
    loops: u32,
    /// Bytes into the playing data, with the fraction between frames.
    position: f64,
    /// Double buffering: the two buffers' addresses and lengths, which plays,
    /// and the callback at each buffer's end.
    buffers: [(u32, u32); 2],
    current: Option<usize>,
    streaming: bool,
    end_of_buffer: u32,
}

impl Sample {
    fn new() -> Sample {
        Sample { status: SMP_DONE, data: 0, length: 0, format: 0, rate: 11_025, volume: 127, pan: 64, loops: 1, position: 0.0, buffers: [(0, 0); 2], current: None, streaming: false, end_of_buffer: 0 }
    }

    fn frame_bytes(&self) -> u32 {
        (if self.format & DIG_F_16BITS != 0 { 2 } else { 1 }) * (if self.format & DIG_F_STEREO != 0 { 2 } else { 1 })
    }

    /// Where the playing data is and how long, for one-shot and streamed samples alike.
    fn playing_data(&self) -> Option<(u32, u32)> {
        if self.streaming {
            self.current.map(|index| self.buffers[index]).filter(|(_, length)| *length > 0)
        } else {
            Some((self.data, self.length))
        }
    }
}

/// One event of an XMIDI sequence, with the ticks to wait before it and where
/// it is in the `EVNT` chunk, which branches point at.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    /// Channel, key, velocity and the ticks the note lasts.
    Note(u8, u8, u8, u32),
    /// Status and up to two data bytes.
    Message(u8, u8, u8),
    End,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Event {
    wait: u32,
    offset: u32,
    kind: Kind,
}

struct Sequence {
    status: u32,
    events: Vec<Event>,
    /// Branch numbers (controller 120's values) and the `EVNT` offsets they go to.
    branches: Vec<(u8, u32)>,
    next: usize,
    /// Ticks still to wait before `next`, with the fraction of one.
    waiting: f64,
    /// FOR loops open: the event after the FOR, and the passes left (0 for ever).
    loops: Vec<(usize, u32)>,
    /// Notes sounding: ticks left, channel and key.
    notes: Vec<(f64, u8, u8)>,
    synth: Option<Synthesizer>,
    volume: f32,
    /// A fade: the volume it ends at, and the change each mixed frame.
    fade: Option<(f32, f32)>,
    trigger: u32,
}

#[derive(Default)]
pub struct Miles {
    samples: Vec<Option<Sample>>,
    sequences: Vec<Option<Sequence>>,
    digital_volume: u32,
    music_volume: u32,
    volumes_set: bool,
    left: Vec<f32>,
    right: Vec<f32>,
}

/// Reads a standard MIDI variable-length number.
fn variable(data: &[u8], at: &mut usize) -> Option<u32> {
    let mut value = 0u32;
    for _ in 0..4 {
        let byte = *data.get(*at)?;
        *at += 1;
        value = value << 7 | (byte & 0x7f) as u32;
        if byte & 0x80 == 0 {
            return Some(value);
        }
    }
    Some(value)
}

/// Turns an `EVNT` chunk into events. XMIDI differs from a MIDI file in three
/// ways that matter here: a delay is a run of bytes below 0x80 that add up, a
/// note-on carries its duration and has no note-off, and there's no running status.
fn parse_events(data: &[u8]) -> Vec<Event> {
    let mut events = Vec::new();
    let mut at = 0;
    let mut wait = 0;
    while at < data.len() {
        let status = data[at];
        if status < 0x80 {
            wait += status as u32;
            at += 1;
            continue;
        }
        let offset = at as u32;
        at += 1;
        let byte = |at: usize| data.get(at).copied().unwrap_or(0) & 0x7f;
        let kind = match status >> 4 {
            0x9 => {
                let (key, velocity) = (byte(at), byte(at + 1));
                at += 2;
                let Some(duration) = variable(data, &mut at) else { break };
                Kind::Note(status & 0xf, key, velocity, duration)
            }
            0x8 | 0xa | 0xb | 0xe => {
                at += 2;
                Kind::Message(status, byte(at - 2), byte(at - 1))
            }
            0xc | 0xd => {
                at += 1;
                Kind::Message(status, byte(at - 1), 0)
            }
            _ => {
                // Meta events and system exclusive messages: only the end matters.
                let meta = if status == 0xff {
                    at += 1;
                    data.get(at - 1).copied()
                } else {
                    None
                };
                let Some(length) = variable(data, &mut at) else { break };
                at += length as usize;
                if meta == Some(0x2f) {
                    events.push(Event { wait, offset, kind: Kind::End });
                    break;
                }
                continue;
            }
        };
        events.push(Event { wait, offset, kind });
        wait = 0;
    }
    if events.last().map(|event| event.kind) != Some(Kind::End) {
        events.push(Event { wait, offset: data.len() as u32, kind: Kind::End });
    }
    events
}

fn be32(data: &[u8], at: usize) -> Option<usize> {
    Some(u32::from_be_bytes(data.get(at..at + 4)?.try_into().ok()?) as usize)
}

/// Finds sequence `number`'s `EVNT` and `RBRN` chunks in an XMIDI file:
/// `FORM XDIR` and `CAT XMID` holding a `FORM XMID` per sequence, or one
/// `FORM XMID` alone.
fn find_sequence(data: &[u8], number: usize) -> Option<(Vec<Event>, Vec<(u8, u32)>)> {
    let mut at = 0;
    let mut forms = Vec::new();
    while at + 12 <= data.len() {
        let (id, length) = (&data[at..at + 4], be32(data, at + 4)?);
        let kind = &data[at + 8..at + 12];
        match (id, kind) {
            (b"FORM", b"XDIR") => at += 8 + length + (length & 1),
            (b"CAT ", _) => at += 12,
            (b"FORM", b"XMID") => {
                forms.push((at + 12, at + 8 + length));
                at += 8 + length + (length & 1);
            }
            _ => break,
        }
    }
    let (mut at, end) = *forms.get(number)?;
    let (mut events, mut branches) = (None, Vec::new());
    while at + 8 <= end.min(data.len()) {
        let (id, length) = (&data[at..at + 4], be32(data, at + 4)?);
        let body = data.get(at + 8..(at + 8 + length).min(data.len()))?;
        match id {
            b"EVNT" => events = Some(parse_events(body)),
            b"RBRN" if body.len() >= 2 => {
                let count = u16::from_le_bytes([body[0], body[1]]) as usize;
                for index in 0..count {
                    let entry = body.get(2 + index * 6..8 + index * 6)?;
                    branches.push((entry[0], u32::from_le_bytes([entry[2], entry[3], entry[4], entry[5]])));
                }
            }
            _ => {}
        }
        at += 8 + length + (length & 1);
    }
    Some((events?, branches))
}

/// The size of the XMIDI data at a guest address, from its IFF headers.
fn xmidi_size(cpu: &Cpu, at: u32) -> u32 {
    let be = |at: u32| u32::from_be_bytes([cpu.r8(at), cpu.r8(at + 1), cpu.r8(at + 2), cpu.r8(at + 3)]);
    let mut size = 8 + be(at + 4);
    if be(at) == u32::from_be_bytes(*b"FORM") && be(at + 8) == u32::from_be_bytes(*b"XDIR") {
        let cat = at + size + (size & 1);
        size = cat - at + 8 + be(cat + 4);
    }
    size.min(4 << 20)
}

fn arg(cpu: &Cpu, index: u32) -> u32 {
    cpu.arg(index)
}

/// Returns to the caller with `value`, leaving the arguments for it to remove.
fn finish(cpu: &mut Cpu, value: u32) {
    cpu.eax = value;
    cpu.returned_to = cpu.pop();
}

fn miles(cpu: &mut Cpu) -> &mut Miles {
    let miles = &mut cpu.host.audio.miles;
    if !miles.volumes_set {
        miles.volumes_set = true;
        miles.digital_volume = 127;
        miles.music_volume = 127;
    }
    miles
}

fn sample_mut(cpu: &mut Cpu, handle: u32) -> Option<&mut Sample> {
    let index = handle.wrapping_sub(SAMPLE_BASE) as usize;
    miles(cpu).samples.get_mut(index)?.as_mut()
}

fn sequence_mut(cpu: &mut Cpu, handle: u32) -> Option<&mut Sequence> {
    let index = handle.wrapping_sub(SEQUENCE_BASE) as usize;
    miles(cpu).sequences.get_mut(index)?.as_mut()
}

fn trace(cpu: &Cpu, text: std::fmt::Arguments) {
    if std::env::var_os("RECOMP_TRACE").is_some() {
        let _ = cpu;
        println!("  miles: {text}");
    }
}

// The drivers.

/// `AIL_install_DIG_INI(driver *)` and `AIL_install_MDI_INI(driver *)`: as if
/// `SETSOUND` had set up a card and its driver loaded.
pub fn install_dig_ini(cpu: &mut Cpu) {
    let out = arg(cpu, 0);
    cpu.w32(out, DIG_DRIVER);
    miles(cpu);
    finish(cpu, 0);
}

pub fn install_mdi_ini(cpu: &mut Cpu) {
    let out = arg(cpu, 0);
    cpu.w32(out, MDI_DRIVER);
    miles(cpu);
    finish(cpu, 0);
}

pub fn mdi_driver_type(cpu: &mut Cpu) {
    finish(cpu, GENERAL_MIDI);
}

pub fn set_digital_master_volume(cpu: &mut Cpu) {
    let volume = arg(cpu, 1).min(127);
    miles(cpu).digital_volume = volume;
    finish(cpu, 0);
}

pub fn set_xmidi_master_volume(cpu: &mut Cpu) {
    let volume = arg(cpu, 1).min(127);
    miles(cpu).music_volume = volume;
    finish(cpu, 0);
}

// Samples.

pub fn allocate_sample_handle(cpu: &mut Cpu) {
    let samples = &mut miles(cpu).samples;
    let index = samples.iter().position(Option::is_none).unwrap_or_else(|| {
        samples.push(None);
        samples.len() - 1
    });
    samples[index] = Some(Sample::new());
    finish(cpu, SAMPLE_BASE + index as u32);
}

pub fn release_sample_handle(cpu: &mut Cpu) {
    let index = arg(cpu, 0).wrapping_sub(SAMPLE_BASE) as usize;
    if let Some(slot) = miles(cpu).samples.get_mut(index) {
        *slot = None;
    }
    finish(cpu, 0);
}

pub fn init_sample(cpu: &mut Cpu) {
    let handle = arg(cpu, 0);
    if let Some(sample) = sample_mut(cpu, handle) {
        *sample = Sample::new();
    }
    finish(cpu, 0);
}

pub fn set_sample_address(cpu: &mut Cpu) {
    let (handle, data, length) = (arg(cpu, 0), arg(cpu, 1), arg(cpu, 2));
    if let Some(sample) = sample_mut(cpu, handle) {
        sample.data = data;
        sample.length = length;
        sample.streaming = false;
    }
    finish(cpu, 0);
}

pub fn set_sample_type(cpu: &mut Cpu) {
    let (handle, format) = (arg(cpu, 0), arg(cpu, 1));
    if let Some(sample) = sample_mut(cpu, handle) {
        sample.format = format & 3;
    }
    finish(cpu, 0);
}

pub fn set_sample_playback_rate(cpu: &mut Cpu) {
    let (handle, rate) = (arg(cpu, 0), arg(cpu, 1));
    if let Some(sample) = sample_mut(cpu, handle) {
        sample.rate = rate.clamp(1, 200_000);
    }
    finish(cpu, 0);
}

pub fn set_sample_volume(cpu: &mut Cpu) {
    let (handle, volume) = (arg(cpu, 0), arg(cpu, 1));
    if let Some(sample) = sample_mut(cpu, handle) {
        sample.volume = volume.min(127);
    }
    finish(cpu, 0);
}

pub fn set_sample_pan(cpu: &mut Cpu) {
    let (handle, pan) = (arg(cpu, 0), arg(cpu, 1));
    if let Some(sample) = sample_mut(cpu, handle) {
        sample.pan = pan.min(127);
    }
    finish(cpu, 0);
}

pub fn set_sample_loop_count(cpu: &mut Cpu) {
    let (handle, loops) = (arg(cpu, 0), arg(cpu, 1));
    if let Some(sample) = sample_mut(cpu, handle) {
        sample.loops = loops;
    }
    finish(cpu, 0);
}

pub fn start_sample(cpu: &mut Cpu) {
    let handle = arg(cpu, 0);
    if let Some(sample) = sample_mut(cpu, handle) {
        sample.position = 0.0;
        sample.status = SMP_PLAYING;
    }
    finish(cpu, 0);
}

pub fn end_sample(cpu: &mut Cpu) {
    let handle = arg(cpu, 0);
    if let Some(sample) = sample_mut(cpu, handle) {
        sample.status = SMP_DONE;
    }
    finish(cpu, 0);
}

pub fn sample_status(cpu: &mut Cpu) {
    let handle = arg(cpu, 0);
    let status = sample_mut(cpu, handle).map_or(SMP_DONE, |sample| sample.status);
    finish(cpu, status);
}

/// `AIL_sample_buffer_ready`: the buffer that wants data, or -1.
pub fn sample_buffer_ready(cpu: &mut Cpu) {
    let handle = arg(cpu, 0);
    let ready = sample_mut(cpu, handle).and_then(|sample| {
        let free = |index: usize| sample.buffers[index].1 == 0;
        match sample.current {
            None => (0..2).find(|&index| free(index)),
            Some(playing) => Some(1 - playing).filter(|&other| free(other)),
        }
    });
    finish(cpu, ready.map_or(u32::MAX, |index| index as u32));
}

/// `AIL_load_sample_buffer(sample, buffer, address, length)`: queues one of the
/// two buffers, and starts playing when nothing is.
pub fn load_sample_buffer(cpu: &mut Cpu) {
    let (handle, index, data, length) = (arg(cpu, 0), arg(cpu, 1) as usize & 1, arg(cpu, 2), arg(cpu, 3));
    if let Some(sample) = sample_mut(cpu, handle) {
        sample.streaming = true;
        sample.buffers[index] = (data, length);
        if sample.current.is_none() && length > 0 {
            sample.current = Some(index);
            sample.position = 0.0;
        }
        if length > 0 {
            sample.status = SMP_PLAYING;
        }
    }
    finish(cpu, 0);
}

pub fn sample_position(cpu: &mut Cpu) {
    let handle = arg(cpu, 0);
    let position = sample_mut(cpu, handle).map_or(0, |sample| sample.position as u32);
    finish(cpu, position);
}

pub fn register_eob_callback(cpu: &mut Cpu) {
    let (handle, callback) = (arg(cpu, 0), arg(cpu, 1));
    let previous = sample_mut(cpu, handle).map_or(0, |sample| std::mem::replace(&mut sample.end_of_buffer, callback));
    finish(cpu, previous);
}

// Sequences.

pub fn allocate_sequence_handle(cpu: &mut Cpu) {
    let sequences = &mut miles(cpu).sequences;
    let index = sequences.iter().position(Option::is_none).unwrap_or_else(|| {
        sequences.push(None);
        sequences.len() - 1
    });
    sequences[index] = Some(Sequence {
        status: SEQ_DONE,
        events: Vec::new(),
        branches: Vec::new(),
        next: 0,
        waiting: 0.0,
        loops: Vec::new(),
        notes: Vec::new(),
        synth: None,
        volume: 1.0,
        fade: None,
        trigger: 0,
    });
    finish(cpu, SEQUENCE_BASE + index as u32);
}

pub fn release_sequence_handle(cpu: &mut Cpu) {
    let index = arg(cpu, 0).wrapping_sub(SEQUENCE_BASE) as usize;
    if let Some(slot) = miles(cpu).sequences.get_mut(index) {
        *slot = None;
    }
    finish(cpu, 0);
}

/// `AIL_init_sequence(sequence, xmidi, number)`: 1 when it could be read.
pub fn init_sequence(cpu: &mut Cpu) {
    let (handle, at, number) = (arg(cpu, 0), arg(cpu, 1), arg(cpu, 2));
    let size = xmidi_size(cpu, at);
    let data: Vec<u8> = (0..size).map(|offset| cpu.r8(at + offset)).collect();
    let found = find_sequence(&data, number as usize);
    trace(cpu, format_args!("init_sequence {handle:#x}: {} bytes, {:?} events", size, found.as_ref().map(|(events, _)| events.len())));
    let ok = match (found, sequence_mut(cpu, handle)) {
        (Some((events, branches)), Some(sequence)) => {
            sequence.events = events;
            sequence.branches = branches;
            sequence.next = 0;
            sequence.waiting = sequence.events.first().map_or(0.0, |event| event.wait as f64);
            sequence.loops.clear();
            sequence.notes.clear();
            sequence.status = SEQ_STOPPED;
            sequence.fade = None;
            sequence.volume = 1.0;
            if sequence.synth.is_none() {
                sequence.synth = super::midi::synthesizer();
            }
            if let Some(synth) = sequence.synth.as_mut() {
                synth.reset();
            }
            1
        }
        _ => 0,
    };
    finish(cpu, ok);
}

fn silence(sequence: &mut Sequence) {
    sequence.notes.clear();
    if let Some(synth) = sequence.synth.as_mut() {
        synth.note_off_all(false);
    }
}

pub fn start_sequence(cpu: &mut Cpu) {
    let handle = arg(cpu, 0);
    if let Some(sequence) = sequence_mut(cpu, handle) {
        silence(sequence);
        sequence.next = 0;
        sequence.waiting = sequence.events.first().map_or(0.0, |event| event.wait as f64);
        sequence.loops.clear();
        sequence.status = SEQ_PLAYING;
        if let Some(synth) = sequence.synth.as_mut() {
            synth.reset();
        }
    }
    finish(cpu, 0);
}

pub fn stop_sequence(cpu: &mut Cpu) {
    let handle = arg(cpu, 0);
    if let Some(sequence) = sequence_mut(cpu, handle) {
        if sequence.status == SEQ_PLAYING {
            sequence.status = SEQ_STOPPED;
            silence(sequence);
        }
    }
    finish(cpu, 0);
}

pub fn resume_sequence(cpu: &mut Cpu) {
    let handle = arg(cpu, 0);
    if let Some(sequence) = sequence_mut(cpu, handle) {
        if sequence.status == SEQ_STOPPED {
            sequence.status = SEQ_PLAYING;
        }
    }
    finish(cpu, 0);
}

pub fn end_sequence(cpu: &mut Cpu) {
    let handle = arg(cpu, 0);
    if let Some(sequence) = sequence_mut(cpu, handle) {
        sequence.status = SEQ_DONE;
        silence(sequence);
    }
    finish(cpu, 0);
}

pub fn sequence_status(cpu: &mut Cpu) {
    let handle = arg(cpu, 0);
    let status = sequence_mut(cpu, handle).map_or(SEQ_DONE, |sequence| sequence.status);
    finish(cpu, status);
}

/// `AIL_set_sequence_volume(sequence, volume, milliseconds)`: a fade to the volume.
pub fn set_sequence_volume(cpu: &mut Cpu) {
    let (handle, volume, milliseconds) = (arg(cpu, 0), arg(cpu, 1), arg(cpu, 2));
    if let Some(sequence) = sequence_mut(cpu, handle) {
        let target = volume.min(127) as f32 / 127.0;
        let frames = milliseconds as f32 / 1000.0 * RATE as f32;
        if frames < 1.0 {
            sequence.volume = target;
            sequence.fade = None;
        } else {
            sequence.fade = Some((target, (target - sequence.volume) / frames));
        }
    }
    finish(cpu, 0);
}

/// `AIL_branch_index(sequence, marker)`: goes on from the branch point that
/// controller 120 with this value marks.
pub fn branch_index(cpu: &mut Cpu) {
    let (handle, marker) = (arg(cpu, 0), arg(cpu, 1));
    if let Some(sequence) = sequence_mut(cpu, handle) {
        branch(sequence, marker as u8);
    }
    finish(cpu, 0);
}

fn branch(sequence: &mut Sequence, marker: u8) {
    let Some(&(_, offset)) = sequence.branches.iter().find(|(id, _)| *id == marker) else { return };
    if let Some(index) = sequence.events.iter().position(|event| event.offset >= offset) {
        silence(sequence);
        sequence.next = index;
        sequence.waiting = 0.0;
        sequence.loops.clear();
    }
}

pub fn register_trigger_callback(cpu: &mut Cpu) {
    let (handle, callback) = (arg(cpu, 0), arg(cpu, 1));
    let previous = sequence_mut(cpu, handle).map_or(0, |sequence| std::mem::replace(&mut sequence.trigger, callback));
    finish(cpu, previous);
}

// Playing.

/// Calls a cdecl callback of the game, which leaves its arguments on the stack.
fn call_back(cpu: &mut Cpu, function: u32, arguments: &[u32]) {
    let esp = cpu.esp;
    super::call_guest(cpu, function, arguments);
    cpu.esp = esp;
}

fn sample_value(cpu: &Cpu, at: u32, sixteen: bool) -> f32 {
    if sixteen {
        cpu.r16(at) as i16 as f32 / 32768.0
    } else {
        (cpu.r8(at) as f32 - 128.0) / 128.0
    }
}

/// Mixes one sample until the mix is full or a buffer ends with a callback
/// to call. Returns the frames done and the callback.
fn mix_sample(cpu: &Cpu, sample: &mut Sample, master: u32, mix: &mut [f32], peak: &mut f32) -> (usize, Option<(u32, u32)>) {
    let frames = mix.len() / 2;
    let gain = sample.volume as f32 / 127.0 * master as f32 / 127.0;
    let left_gain = gain * ((127 - sample.pan) as f32 / 63.5).min(1.0);
    let right_gain = gain * (sample.pan as f32 / 63.5).min(1.0);
    let frame_bytes = sample.frame_bytes();
    let (sixteen, stereo) = (sample.format & DIG_F_16BITS != 0, sample.format & DIG_F_STEREO != 0);
    let step = sample.rate as f64 / RATE * frame_bytes as f64;
    let mut done = 0;
    while done < frames {
        let Some((data, length)) = sample.playing_data() else { return (done, None) };
        let end = (length / frame_bytes * frame_bytes) as f64;
        if end == 0.0 && !sample.streaming {
            sample.status = SMP_DONE;
            return (frames, None);
        }
        if sample.position >= end {
            if sample.streaming {
                let finished = sample.current.unwrap_or(0);
                sample.buffers[finished].1 = 0;
                let other = 1 - finished;
                sample.position = 0.0;
                if sample.buffers[other].1 > 0 {
                    sample.current = Some(other);
                } else {
                    sample.current = None;
                    sample.status = SMP_DONE;
                }
                if sample.end_of_buffer != 0 {
                    return (done, Some((sample.end_of_buffer, finished as u32)));
                }
                if sample.current.is_none() {
                    return (frames, None);
                }
                continue;
            }
            if sample.loops == 1 {
                sample.status = SMP_DONE;
                return (frames, None);
            }
            sample.loops = sample.loops.saturating_sub(1).max(if sample.loops == 0 { 0 } else { 1 });
            sample.position -= end;
            continue;
        }
        let at = data + (sample.position as u32 / frame_bytes) * frame_bytes;
        let left = sample_value(cpu, at, sixteen);
        let right = if stereo { sample_value(cpu, at + frame_bytes / 2, sixteen) } else { left };
        mix[done * 2] += left * left_gain;
        mix[done * 2 + 1] += right * right_gain;
        *peak = peak.max((left * left_gain).abs()).max((right * right_gain).abs());
        sample.position += step;
        done += 1;
    }
    (done, None)
}

/// Plays one event of a sequence. Returns a trigger to report: channel and value.
fn play(sequence: &mut Sequence) -> Option<(u32, u32)> {
    let index = sequence.next;
    let Some(&event) = sequence.events.get(index) else {
        sequence.status = SEQ_DONE;
        return None;
    };
    sequence.next += 1;
    sequence.waiting = sequence.events.get(sequence.next).map_or(0.0, |next| next.wait as f64);
    match event.kind {
        Kind::End => {
            sequence.status = SEQ_DONE;
            silence(sequence);
        }
        Kind::Note(channel, key, velocity, duration) => {
            if let Some(synth) = sequence.synth.as_mut() {
                synth.note_on(channel as i32, key as i32, velocity as i32);
            }
            sequence.notes.push((duration as f64, channel, key));
        }
        Kind::Message(status, first, second) => {
            let channel = status & 0xf;
            if status & 0xf0 == 0xb0 && (110..=120).contains(&first) {
                // XMIDI's own controllers, which a synthesizer never sees.
                match first {
                    116 => sequence.loops.push((sequence.next, second as u32)),
                    117 if second >= 64 => {
                        if let Some(&(start, passes)) = sequence.loops.last() {
                            let again = passes == 0 || passes > 1;
                            if passes > 1 {
                                sequence.loops.last_mut().unwrap().1 -= 1;
                            }
                            if again {
                                sequence.next = start;
                                sequence.waiting = sequence.events.get(start).map_or(0.0, |next| next.wait as f64);
                            } else {
                                sequence.loops.pop();
                            }
                        }
                    }
                    119 => return Some((channel as u32, second as u32)),
                    _ => {}
                }
            } else if let Some(synth) = sequence.synth.as_mut() {
                synth.process_midi_message(channel as i32, (status & 0xf0) as i32, first as i32, second as i32);
            }
        }
    }
    None
}

/// Renders a sequence's synthesizer for `frames`, playing the events that
/// fall in them. Stops early at a trigger, returning the frames done and it.
fn render_sequence(sequence: &mut Sequence, left: &mut [f32], right: &mut [f32]) -> (usize, Option<(u32, u32)>) {
    let frames = left.len();
    let per_tick = RATE / TICKS_PER_SECOND;
    let mut done = 0;
    while done < frames {
        let mut run = frames - done;
        if sequence.status == SEQ_PLAYING {
            // The next thing due: an event, or a note's end.
            let note_end = sequence.notes.iter().map(|note| note.0).fold(f64::INFINITY, f64::min);
            let due = sequence.waiting.min(note_end);
            if due * per_tick < 0.5 {
                if note_end <= sequence.waiting {
                    let synth = sequence.synth.as_mut();
                    let mut ended = Vec::new();
                    sequence.notes.retain(|&(ticks, channel, key)| {
                        let keep = ticks > note_end + 1e-9;
                        if !keep {
                            ended.push((channel, key));
                        }
                        keep
                    });
                    if let Some(synth) = synth {
                        for (channel, key) in ended {
                            synth.note_off(channel as i32, key as i32);
                        }
                    }
                    let passed = note_end.max(0.0);
                    sequence.waiting -= passed;
                    for note in &mut sequence.notes {
                        note.0 -= passed;
                    }
                    continue;
                }
                let passed = sequence.waiting.max(0.0);
                for note in &mut sequence.notes {
                    note.0 -= passed;
                }
                if let Some(trigger) = play(sequence) {
                    return (done, Some(trigger));
                }
                continue;
            }
            run = run.min((due * per_tick).ceil() as usize).max(1);
            let ticks = run as f64 / per_tick;
            sequence.waiting -= ticks;
            for note in &mut sequence.notes {
                note.0 -= ticks;
            }
        }
        if let Some(synth) = sequence.synth.as_mut() {
            synth.render(&mut left[done..done + run], &mut right[done..done + run]);
        }
        done += run;
    }
    (frames, None)
}

/// Adds the samples and music to `mix` (interleaved stereo at 44.1 kHz), and
/// returns the loudest of each.
pub fn mix(cpu: &mut Cpu, mix: &mut [f32]) -> (f32, f32) {
    let (mut sample_peak, mut music_peak) = (0f32, 0f32);
    let frames = mix.len() / 2;
    for index in 0..cpu.host.audio.miles.samples.len() {
        let mut done = 0;
        let mut callbacks = 0;
        while done < frames && callbacks < 64 {
            callbacks += 1;
            let master = cpu.host.audio.miles.digital_volume;
            let Some(mut sample) = cpu.host.audio.miles.samples.get(index).copied().flatten() else { break };
            if sample.status != SMP_PLAYING {
                break;
            }
            let (frames_done, callback) = mix_sample(cpu, &mut sample, master, &mut mix[done * 2..], &mut sample_peak);
            done += frames_done;
            if let Some(slot) = cpu.host.audio.miles.samples.get_mut(index) {
                *slot = Some(sample);
            }
            match callback {
                Some((function, _)) => call_back(cpu, function, &[SAMPLE_BASE + index as u32]),
                None => break,
            }
        }
    }
    for index in 0..cpu.host.audio.miles.sequences.len() {
        let mut done = 0;
        let mut left = std::mem::take(&mut cpu.host.audio.miles.left);
        let mut right = std::mem::take(&mut cpu.host.audio.miles.right);
        left.clear();
        left.resize(frames, 0.0);
        right.clear();
        right.resize(frames, 0.0);
        // A trigger whose callback branches back to itself mustn't hang the game.
        let mut triggers = 0;
        while done < frames && triggers < 64 {
            triggers += 1;
            let Some(mut sequence) = cpu.host.audio.miles.sequences.get_mut(index).and_then(Option::take) else { break };
            if sequence.status != SEQ_PLAYING && sequence.notes.is_empty() && sequence.synth.is_none() {
                cpu.host.audio.miles.sequences[index] = Some(sequence);
                break;
            }
            let (frames_done, trigger) = render_sequence(&mut sequence, &mut left[done..], &mut right[done..]);
            let master = cpu.host.audio.miles.music_volume as f32 / 127.0;
            for frame in done..done + frames_done {
                if let Some((target, step)) = sequence.fade {
                    sequence.volume += step;
                    if (step >= 0.0 && sequence.volume >= target) || (step < 0.0 && sequence.volume <= target) {
                        sequence.volume = target;
                        sequence.fade = None;
                    }
                }
                // Miles scaled the channels' volume controllers, which a General
                // MIDI synthesizer hears on a square law.
                let gain = (sequence.volume * master).powi(2) * super::midi::LEVEL;
                let (l, r) = (left[frame] * gain, right[frame] * gain);
                mix[frame * 2] += l;
                mix[frame * 2 + 1] += r;
                music_peak = music_peak.max(l.abs()).max(r.abs());
            }
            done += frames_done;
            let trigger_function = sequence.trigger;
            cpu.host.audio.miles.sequences[index] = Some(sequence);
            match trigger {
                Some((channel, value)) if trigger_function != 0 => {
                    call_back(cpu, trigger_function, &[SEQUENCE_BASE + index as u32, channel, value]);
                }
                Some(_) => {}
                None => break,
            }
        }
        cpu.host.audio.miles.left = left;
        cpu.host.audio.miles.right = right;
    }
    (sample_peak, music_peak)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xmidi_events_add_up_delays_and_carry_note_lengths() {
        // A delay of 0x7f + 3, a note of 96 ticks, a program change, the end.
        let data = [0x7f, 0x03, 0x90, 60, 100, 0x60, 0xc1, 5, 0xff, 0x2f, 0x00];
        let events = parse_events(&data);
        assert_eq!(events[0], Event { wait: 130, offset: 2, kind: Kind::Note(0, 60, 100, 96) });
        assert_eq!(events[1], Event { wait: 0, offset: 6, kind: Kind::Message(0xc1, 5, 0) });
        assert_eq!(events[2].kind, Kind::End);
    }

    #[test]
    fn a_truncated_chunk_still_ends() {
        let events = parse_events(&[0x10, 0x90, 60]);
        assert_eq!(events.last().map(|event| event.kind), Some(Kind::End));
    }

    #[test]
    fn a_long_note_length_is_read_as_a_variable_number() {
        let events = parse_events(&[0x90, 60, 100, 0x81, 0x00]);
        assert_eq!(events[0].kind, Kind::Note(0, 60, 100, 128));
    }

    fn form(kind: &[u8], body: &[u8]) -> Vec<u8> {
        let mut out = b"FORM".to_vec();
        out.extend_from_slice(&((body.len() + 4) as u32).to_be_bytes());
        out.extend_from_slice(kind);
        out.extend_from_slice(body);
        out
    }

    fn chunk(id: &[u8], body: &[u8]) -> Vec<u8> {
        let mut out = id.to_vec();
        out.extend_from_slice(&(body.len() as u32).to_be_bytes());
        out.extend_from_slice(body);
        if body.len() & 1 == 1 {
            out.push(0);
        }
        out
    }

    #[test]
    fn the_second_sequence_of_a_collection_and_its_branches_are_found() {
        let first = form(b"XMID", &chunk(b"EVNT", &[0xff, 0x2f, 0]));
        let mut branches = vec![1, 0, 7, 0, 3, 0, 0, 0];
        let mut body = chunk(b"RBRN", &branches);
        body.extend(chunk(b"EVNT", &[0x05, 0xc0, 1, 0xff, 0x2f, 0]));
        let second = form(b"XMID", &body);
        let mut data = form(b"XDIR", &chunk(b"INFO", &[2, 0]));
        let mut cat = b"CAT ".to_vec();
        cat.extend_from_slice(&((4 + first.len() + second.len()) as u32).to_be_bytes());
        cat.extend_from_slice(b"XMID");
        cat.extend(first);
        cat.extend(second);
        data.extend(cat);
        let (events, found) = find_sequence(&data, 1).unwrap();
        assert_eq!(events[0], Event { wait: 5, offset: 1, kind: Kind::Message(0xc0, 1, 0) });
        assert_eq!(found, vec![(7, 3)]);
        branches.clear();
        assert!(find_sequence(&data, 2).is_none());
    }

    fn sequence(events: Vec<Event>) -> Sequence {
        let waiting = events.first().map_or(0.0, |event| event.wait as f64);
        Sequence { status: SEQ_PLAYING, events, branches: Vec::new(), next: 0, waiting, loops: Vec::new(), notes: Vec::new(), synth: None, volume: 1.0, fade: None, trigger: 0 }
    }

    #[test]
    fn a_for_loop_without_a_count_repeats_for_ever() {
        let controller = |number, value| Kind::Message(0xb0, number, value);
        let mut playing = sequence(vec![
            Event { wait: 0, offset: 0, kind: controller(116, 0) },
            Event { wait: 10, offset: 3, kind: Kind::Message(0xc0, 1, 0) },
            Event { wait: 0, offset: 5, kind: controller(117, 127) },
            Event { wait: 0, offset: 8, kind: Kind::End },
        ]);
        for _ in 0..10 {
            play(&mut playing);
        }
        assert_eq!(playing.status, SEQ_PLAYING);
        assert!(playing.next <= 2);
    }

    #[test]
    fn a_trigger_is_reported_with_its_channel_and_value() {
        let mut playing = sequence(vec![Event { wait: 0, offset: 0, kind: Kind::Message(0xb3, 119, 9) }, Event { wait: 0, offset: 3, kind: Kind::End }]);
        assert_eq!(play(&mut playing), Some((3, 9)));
    }

    #[test]
    fn music_at_120_ticks_a_second_ends_on_time() {
        let mut playing = sequence(vec![Event { wait: 120, offset: 0, kind: Kind::End }]);
        let (mut left, mut right) = (vec![0.0; 44_000], vec![0.0; 44_000]);
        render_sequence(&mut playing, &mut left, &mut right);
        assert_eq!(playing.status, SEQ_PLAYING);
        let (mut left, mut right) = (vec![0.0; 200], vec![0.0; 200]);
        render_sequence(&mut playing, &mut left, &mut right);
        assert_eq!(playing.status, SEQ_DONE);
    }
}
