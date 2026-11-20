//! winmm's MIDI streams, played by a SoundFont synthesizer.
//!
//! Games of the time turned their `.MID` files into buffers of timed events
//! themselves, and queued them on a MIDI stream with `midiStreamOut`. Windows
//! played each event at its time on the sound card's synthesizer, and called the
//! game back when a buffer was done, so it could fill and queue it again. Here
//! the events are timed against the mixer's samples and played by `rustysynth`,
//! with the free GeneralUser GS bank built into the program (see
//! `assets/GeneralUser-GS-LICENSE.txt`), so it sounds like a good wavetable
//! card of the time on every system.

use crate::cpu::Cpu;
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
use std::collections::VecDeque;
use std::sync::Arc;

static SOUND_BANK: &[u8] = include_bytes!("../../assets/GeneralUser-GS.sf2");

const RATE: i32 = 44_100;
const MOM_DONE: u32 = 0x3c9;
const MOM_POSITIONCB: u32 = 0x3ca;
const CALLBACK_TYPE: u32 = 0x7_0000;
const CALLBACK_WINDOW: u32 = 0x1_0000;
const CALLBACK_FUNCTION: u32 = 0x3_0000;
const MHDR_DONE: u32 = 1;
const MHDR_INQUEUE: u32 = 4;
const MHDR_ISSTRM: u32 = 8;
const HEADER_LENGTH: u32 = 8;
const HEADER_FLAGS: u32 = 0x10;
const HEADER_OFFSET: u32 = 0x1c;
const MEVT_F_LONG: u32 = 0x8000_0000;
const MEVT_F_CALLBACK: u32 = 0x4000_0000;
const MEVT_SHORTMSG: u32 = 0;
const MEVT_TEMPO: u32 = 1;
const MIDIPROP_SET: u32 = 0x8000_0000;
const MIDIPROP_TIMEDIV: u32 = 1;
const MIDIPROP_TEMPO: u32 = 2;
/// Quarter notes of half a second, and the division Windows starts with.
const DEFAULT_TEMPO: u32 = 500_000;
const DEFAULT_DIVISION: u32 = 96;
/// The synthesizer is quieter than a card was: this brings it near the level
/// of the game's sound effects.
pub(crate) const LEVEL: f32 = 1.5;
const MIDIERR_STILLPLAYING: u32 = 65;
const MMSYSERR_INVALHANDLE: u32 = 5;

/// The stream the game opened.
struct Stream {
    handle: u32,
    callback: u32,
    instance: u32,
    kind: u32,
    division: u32,
    tempo: u32,
    /// The buffers queued, the first playing.
    queue: VecDeque<u32>,
    /// Where the next event is in the first buffer.
    at: u32,
    /// Ticks still to wait before that event, once its delta has been read.
    waiting: Option<f64>,
    playing: bool,
}

#[derive(Default)]
pub struct Midi {
    synth: Option<Synthesizer>,
    stream: Option<Stream>,
    /// `midiOutSetVolume`'s left and right, from 0 to 1.
    volume: Option<(f32, f32)>,
    /// Calls owed to the game: its callback, or window, and the message.
    owed: Vec<(u32, u32, u32, u32, u32)>,
    left: Vec<f32>,
    right: Vec<f32>,
}

/// The built-in bank, read once and shared by every synthesizer.
fn bank() -> Option<Arc<SoundFont>> {
    static BANK: std::sync::OnceLock<Option<Arc<SoundFont>>> = std::sync::OnceLock::new();
    BANK.get_or_init(|| SoundFont::new(&mut &SOUND_BANK[..]).map(Arc::new).map_err(|error| println!("the MIDI sound bank can't be read: {error:?}")).ok()).clone()
}

pub(crate) fn synthesizer() -> Option<Synthesizer> {
    Synthesizer::new(&bank()?, &SynthesizerSettings::new(RATE)).map_err(|error| println!("the MIDI synthesizer can't start: {error:?}")).ok()
}

impl Stream {
    fn samples_per_tick(&self) -> f64 {
        self.tempo as f64 / 1_000_000.0 / self.division.max(1) as f64 * RATE as f64
    }

    /// Owes the game a callback with this message about a buffer.
    fn owe(&self, owed: &mut Vec<(u32, u32, u32, u32, u32)>, message: u32, header: u32) {
        owed.push((self.kind, self.callback, self.handle, message, header));
        let _ = self.instance;
    }

    /// Marks every queued buffer done, as stopping or resetting does.
    fn return_all(&mut self, cpu: &mut Cpu, owed: &mut Vec<(u32, u32, u32, u32, u32)>) {
        while let Some(header) = self.queue.pop_front() {
            let flags = cpu.r32(header + HEADER_FLAGS);
            cpu.w32(header + HEADER_FLAGS, (flags | MHDR_DONE) & !MHDR_INQUEUE);
            self.owe(owed, MOM_DONE, header);
        }
        self.at = 0;
        self.waiting = None;
    }
}

/// Plays one event of the first buffer, and moves past it.
fn play_event(cpu: &mut Cpu, stream: &mut Stream, synth: &mut Option<Synthesizer>, owed: &mut Vec<(u32, u32, u32, u32, u32)>, header: u32) {
    let data = cpu.r32(header);
    let event = cpu.r32(data + stream.at + 8);
    let size = if event & MEVT_F_LONG != 0 { 12 + ((event & 0xff_ffff) + 3 & !3) } else { 12 };
    match event >> 24 & 0x3f {
        MEVT_SHORTMSG if event & MEVT_F_LONG == 0 => {
            if let Some(synth) = synth.as_mut() {
                let status = (event & 0xff) as i32;
                synth.process_midi_message(status & 0xf, status & 0xf0, (event >> 8 & 0x7f) as i32, (event >> 16 & 0x7f) as i32);
            }
        }
        MEVT_TEMPO if event & MEVT_F_LONG == 0 => stream.tempo = (event & 0xff_ffff).max(1),
        // System exclusive messages, comments and versions change nothing here.
        _ => {}
    }
    if event & MEVT_F_CALLBACK != 0 {
        cpu.w32(header + HEADER_OFFSET, stream.at);
        stream.owe(owed, MOM_POSITIONCB, header);
    }
    stream.at += size;
    stream.waiting = None;
}

/// Adds `mix` (interleaved stereo) worth of music, playing the events that
/// fall in it, and returns its loudest sample.
pub fn mix(cpu: &mut Cpu, mix: &mut [f32]) -> f32 {
    let mut midi = std::mem::take(&mut cpu.host.audio.midi);
    let frames = mix.len() / 2;
    let mut peak = 0f32;
    if let Some(mut stream) = midi.stream.take() {
        midi.left.resize(frames, 0.0);
        midi.right.resize(frames, 0.0);
        let mut done = 0;
        while done < frames {
            let mut run = frames - done;
            if stream.playing {
                if let Some(&header) = stream.queue.front() {
                    let data = cpu.r32(header);
                    if stream.at + 12 > cpu.r32(header + HEADER_LENGTH) {
                        // The buffer is played out: the game gets it back.
                        let flags = cpu.r32(header + HEADER_FLAGS);
                        cpu.w32(header + HEADER_FLAGS, (flags | MHDR_DONE) & !MHDR_INQUEUE);
                        stream.owe(&mut midi.owed, MOM_DONE, header);
                        stream.queue.pop_front();
                        stream.at = 0;
                        stream.waiting = None;
                        continue;
                    }
                    let ticks = *stream.waiting.get_or_insert(cpu.r32(data + stream.at) as f64);
                    let samples = ticks * stream.samples_per_tick();
                    if samples < 0.5 {
                        play_event(cpu, &mut stream, &mut midi.synth, &mut midi.owed, header);
                        continue;
                    }
                    run = run.min(samples.ceil() as usize);
                    stream.waiting = Some((ticks - run as f64 / stream.samples_per_tick()).max(0.0));
                }
            }
            if let Some(synth) = midi.synth.as_mut() {
                synth.render(&mut midi.left[done..done + run], &mut midi.right[done..done + run]);
            }
            done += run;
        }
        if midi.synth.is_some() {
            let (left_gain, right_gain) = midi.volume.unwrap_or((1.0, 1.0));
            for (index, output) in mix.chunks_exact_mut(2).enumerate() {
                let (left, right) = (midi.left[index] * LEVEL * left_gain, midi.right[index] * LEVEL * right_gain);
                output[0] += left;
                output[1] += right;
                peak = peak.max(left.abs()).max(right.abs());
            }
        }
        midi.stream = Some(stream);
    }
    cpu.host.audio.midi = midi;
    peak
}

/// Makes the calls the game is owed, after mixing, when the stream is back in
/// place for any call the game makes in return.
pub fn deliver(cpu: &mut Cpu) {
    let owed = std::mem::take(&mut cpu.host.audio.midi.owed);
    let instance = cpu.host.audio.midi.stream.as_ref().map_or(0, |stream| stream.instance);
    for (kind, callback, handle, message, header) in owed {
        match kind {
            CALLBACK_FUNCTION if callback != 0 => {
                super::call_guest(cpu, callback, &[handle, message, instance, header, 0]);
            }
            CALLBACK_WINDOW if callback != 0 => cpu.host.messages.push_back((message, handle, header)),
            _ => {}
        }
    }
}

fn stream_mut(cpu: &mut Cpu, handle: u32) -> Option<&mut Stream> {
    cpu.host.audio.midi.stream.as_mut().filter(|stream| stream.handle == handle)
}

pub fn open(cpu: &mut Cpu, out: u32, callback: u32, instance: u32, flags: u32) -> u32 {
    if cpu.host.audio.midi.stream.is_some() {
        const MMSYSERR_ALLOCATED: u32 = 4;
        return MMSYSERR_ALLOCATED;
    }
    if cpu.host.audio.midi.synth.is_none() {
        cpu.host.audio.midi.synth = synthesizer();
    }
    let handle = cpu.host.handle();
    cpu.w32(out, handle);
    cpu.host.audio.midi.stream = Some(Stream {
        handle,
        callback,
        instance,
        kind: flags & CALLBACK_TYPE,
        division: DEFAULT_DIVISION,
        tempo: DEFAULT_TEMPO,
        queue: VecDeque::new(),
        at: 0,
        waiting: None,
        playing: false,
    });
    0
}

pub fn close(cpu: &mut Cpu, handle: u32) -> u32 {
    let Some(stream) = stream_mut(cpu, handle) else { return MMSYSERR_INVALHANDLE };
    if !stream.queue.is_empty() {
        return MIDIERR_STILLPLAYING;
    }
    cpu.host.audio.midi.stream = None;
    if let Some(synth) = cpu.host.audio.midi.synth.as_mut() {
        synth.reset();
    }
    0
}

pub fn property(cpu: &mut Cpu, handle: u32, data: u32, flags: u32) -> u32 {
    let value = cpu.r32(data + 4);
    let Some(stream) = stream_mut(cpu, handle) else { return MMSYSERR_INVALHANDLE };
    let field = if flags & MIDIPROP_TIMEDIV != 0 {
        &mut stream.division
    } else if flags & MIDIPROP_TEMPO != 0 {
        &mut stream.tempo
    } else {
        return 0;
    };
    if flags & MIDIPROP_SET != 0 {
        *field = value.max(1);
    } else {
        let value = *field;
        cpu.w32(data + 4, value);
    }
    0
}

pub fn out(cpu: &mut Cpu, handle: u32, header: u32) -> u32 {
    let flags = cpu.r32(header + HEADER_FLAGS);
    cpu.w32(header + HEADER_FLAGS, (flags | MHDR_INQUEUE | MHDR_ISSTRM) & !MHDR_DONE);
    let Some(stream) = stream_mut(cpu, handle) else { return MMSYSERR_INVALHANDLE };
    stream.queue.push_back(header);
    0
}

pub fn set_playing(cpu: &mut Cpu, handle: u32, playing: bool) -> u32 {
    let Some(stream) = stream_mut(cpu, handle) else { return MMSYSERR_INVALHANDLE };
    stream.playing = playing;
    if !playing {
        if let Some(synth) = cpu.host.audio.midi.synth.as_mut() {
            synth.note_off_all(false);
        }
    }
    0
}

/// `midiStreamStop` and `midiOutReset`: silence, and every buffer back to the
/// game, which Windows reports at once.
pub fn stop(cpu: &mut Cpu, handle: u32, reset: bool) -> u32 {
    let Some(mut stream) = cpu.host.audio.midi.stream.take() else { return MMSYSERR_INVALHANDLE };
    if stream.handle != handle {
        cpu.host.audio.midi.stream = Some(stream);
        return MMSYSERR_INVALHANDLE;
    }
    let mut owed = std::mem::take(&mut cpu.host.audio.midi.owed);
    stream.return_all(cpu, &mut owed);
    stream.playing = false;
    cpu.host.audio.midi.owed = owed;
    cpu.host.audio.midi.stream = Some(stream);
    if let Some(synth) = cpu.host.audio.midi.synth.as_mut() {
        synth.note_off_all(true);
        if reset {
            synth.reset_all_controllers();
        }
    }
    deliver(cpu);
    0
}

pub fn set_volume(cpu: &mut Cpu, volume: u32) -> u32 {
    let (left, right) = (volume & 0xffff, volume >> 16);
    cpu.host.audio.midi.volume = Some((left as f32 / 65535.0, right as f32 / 65535.0));
    0
}

pub fn volume(cpu: &mut Cpu, at: u32) -> u32 {
    let (left, right) = cpu.host.audio.midi.volume.unwrap_or((1.0, 1.0));
    let value = (left * 65535.0) as u32 | ((right * 65535.0) as u32) << 16;
    cpu.w32(at, value);
    0
}

/// `midiOutGetDevCapsA`: one synthesizer with volume and streams, so games
/// that check show their music volume control.
pub fn capabilities(cpu: &mut Cpu, at: u32, size: u32) -> u32 {
    const MOD_SWSYNTH: u16 = 7;
    const MIDICAPS_VOLUME: u32 = 1;
    const MIDICAPS_LRVOLUME: u32 = 2;
    const MIDICAPS_STREAM: u32 = 8;
    const VOICES: u16 = 64;
    let mut caps = [0u8; 0x34];
    let name = b"SoundFont Synthesizer";
    caps[8..8 + name.len()].copy_from_slice(name);
    caps[0x28..0x2a].copy_from_slice(&MOD_SWSYNTH.to_le_bytes());
    caps[0x2a..0x2c].copy_from_slice(&VOICES.to_le_bytes());
    caps[0x2c..0x2e].copy_from_slice(&VOICES.to_le_bytes());
    caps[0x2e..0x30].copy_from_slice(&0xffffu16.to_le_bytes());
    caps[0x30..0x34].copy_from_slice(&(MIDICAPS_VOLUME | MIDICAPS_LRVOLUME | MIDICAPS_STREAM).to_le_bytes());
    let size = size.min(caps.len() as u32);
    cpu.bytes_mut(at, size).copy_from_slice(&caps[..size as usize]);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_quarter_note_at_the_default_tempo_lasts_half_a_second() {
        let stream = Stream { handle: 1, callback: 0, instance: 0, kind: 0, division: 96, tempo: DEFAULT_TEMPO, queue: VecDeque::new(), at: 0, waiting: None, playing: true };
        let half_second = stream.samples_per_tick() * 96.0;
        assert!((half_second - RATE as f64 / 2.0).abs() < 1e-6);
    }

    #[test]
    fn the_built_in_bank_loads() {
        assert!(synthesizer().is_some());
    }
}
