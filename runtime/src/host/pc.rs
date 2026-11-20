//! The PC's hardware, for recompiled DOS programs: the timer chip, the
//! interrupt controller and the keyboard.
//!
//! A DOS game hooks the timer interrupt (8) and the keyboard interrupt (9)
//! and programs the timer's rate itself. Translated code can't be interrupted
//! in the middle, so the runtime runs those handlers at safe points, the
//! backward jumps of the game's loops (see `Cpu::poll`), as often as the
//! hardware would have: the timer at the rate the game set, the keyboard once
//! for each key that went down or up, with its scan code on port 0x60.
//!
//! The same safe points pump the window's input, the sound and the picture,
//! which a Windows program's message loop does for it.

use crate::cpu::Cpu;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// The timer chip's clock.
const PIT_HZ: f64 = 1_193_182.0;
/// The BIOS's tick count, which DOS's clock reads.
const BIOS_TICKS: u32 = 0x46c;
/// The BIOS's rate: the timer's slowest, 65536 counts.
const BIOS_PERIOD: Duration = Duration::from_nanos(54_925_400);
/// How far behind the timer may fall before it skips ahead, after a pause.
const FURTHEST_BEHIND: Duration = Duration::from_millis(250);
const TIMER_VECTOR: u8 = 8;
const KEYBOARD_VECTOR: u8 = 9;
const TIMER_IRQ: u8 = 1;
const KEYBOARD_IRQ: u8 = 2;

pub struct Pc {
    /// The timer's count for channel 0, and the low byte of one being written.
    divisor: u32,
    low_byte: Option<u8>,
    next_tick: Instant,
    next_bios_tick: Instant,
    /// The interrupt controller's mask: a set bit holds that line back.
    mask: u8,
    /// Scan codes waiting for the keyboard interrupt, and the one on port 0x60.
    /// Each is due at its time, so a scripted release can come as late as a finger's.
    keys: VecDeque<(u8, Instant)>,
    scancode: u8,
    /// A handler is running; another can't start until it ends.
    in_interrupt: bool,
}

impl Default for Pc {
    fn default() -> Pc {
        Pc {
            divisor: 65536,
            low_byte: None,
            next_tick: Instant::now(),
            next_bios_tick: Instant::now(),
            mask: 0,
            keys: VecDeque::new(),
            scancode: 0,
            in_interrupt: false,
        }
    }
}

impl Pc {
    fn period(&self) -> Duration {
        Duration::from_secs_f64(self.divisor.max(1) as f64 / PIT_HZ)
    }
}

/// A port this module answers, read: `None` for the others.
pub fn read(cpu: &mut Cpu, port: u16) -> Option<u32> {
    let pc = &mut cpu.host.pc;
    Some(match port {
        0x60 => pc.scancode as u32,
        // The keyboard controller's status: a byte waits when one does.
        0x64 => u32::from(pc.keys.front().is_some_and(|&(_, at)| at <= Instant::now())),
        0x61 => 0x20,
        0x21 => pc.mask as u32,
        0x20 | 0xa0 | 0xa1 => 0,
        // The timer's count: near enough, from the time since the last tick.
        0x40 => {
            let left = pc.next_tick.saturating_duration_since(Instant::now()).as_secs_f64() * PIT_HZ;
            (left as u32) & 0xff
        }
        _ => return None,
    })
}

/// A port this module answers, written: false for the others.
pub fn write(cpu: &mut Cpu, port: u16, value: u32) -> bool {
    let pc = &mut cpu.host.pc;
    match port {
        // The timer's control word: a new count for channel 0 follows, low
        // byte first.
        0x43 => {
            if value >> 6 == 0 {
                pc.low_byte = None;
            }
        }
        0x40 => match pc.low_byte.take() {
            None => pc.low_byte = Some(value as u8),
            Some(low) => {
                let count = (value & 0xff) << 8 | low as u32;
                pc.divisor = if count == 0 { 65536 } else { count };
                pc.next_tick = Instant::now() + pc.period();
            }
        },
        0x21 => pc.mask = value as u8,
        // End of interrupt, and the second controller: nothing to do.
        0x20 | 0xa0 | 0xa1 | 0x61 | 0x42 => {}
        _ => return false,
    }
    true
}

/// Queues a key for the keyboard interrupt, from its DirectInput code: the
/// scan code, with 0x80 for keys that send 0xE0 first. A release sets the
/// top bit of the code.
pub fn key(cpu: &mut Cpu, code: usize, down: bool) {
    key_at(cpu, code, down, Instant::now());
}

/// How long a scripted key is held. Games that look at which keys are down
/// once a frame miss a press released a millisecond later, as Z's video
/// player does; a real key is down for 50 ms or more.
pub const SCRIPT_HOLD: Duration = Duration::from_millis(120);

/// `key`, delivered no earlier than `at`. Keys stay in order, so a later one
/// waits behind it.
pub fn key_at(cpu: &mut Cpu, code: usize, down: bool, at: Instant) {
    const EXTENDED: usize = 0x80;
    if code & EXTENDED != 0 {
        cpu.host.pc.keys.push_back((0xe0, at));
    }
    let release = if down { 0 } else { 0x80 };
    cpu.host.pc.keys.push_back(((code & 0x7f) as u8 | release, at));
}

/// Calls the program's handler for an interrupt, as the processor would:
/// the flags, the code selector and a return address on its stack, and the
/// handler's `iretd` takes them off. Registers come back as they were.
fn call_handler(cpu: &mut Cpu, vector: u8) -> bool {
    let Some(&(_, offset)) = cpu.host.dos.vectors.get(&vector) else { return false };
    if offset == 0 {
        return false;
    }
    let saved = super::timers::Registers::of(cpu);
    let flags = cpu.flags();
    cpu.push(flags);
    cpu.push(super::dos::FLAT as u32);
    cpu.push(0);
    cpu.df = false;
    // An interrupt gate turns interrupts off; `iretd` restores them.
    cpu.interrupts_off = true;
    cpu.call(offset);
    saved.restore(cpu);
    cpu.interrupts_off = false;
    true
}

/// Runs the hardware's part: called at the program's safe points, about once
/// a millisecond. Interrupts don't nest, as the processor holds them back
/// while one is handled.
pub fn service(cpu: &mut Cpu) {
    if cpu.host.pc.in_interrupt {
        return;
    }
    cpu.host.pc.in_interrupt = true;
    super::video::pump(cpu);
    super::audio::pump(cpu);
    // A DOS program has no message loop, so the script's steps run here.
    super::script::advance(cpu);
    // With interrupts off (`cli`), the hardware's interrupts wait for `sti`.
    if cpu.interrupts_off {
        cpu.host.pc.in_interrupt = false;
        return;
    }
    super::mouse::service(cpu);
    let now = Instant::now();

    // The BIOS clock, which DOS's time and some games read.
    while cpu.host.pc.next_bios_tick <= now {
        cpu.host.pc.next_bios_tick += BIOS_PERIOD;
        let ticks = cpu.r32(BIOS_TICKS).wrapping_add(1);
        cpu.w32(BIOS_TICKS, ticks);
    }

    // The keyboard: one scan code per interrupt.
    if cpu.host.pc.mask & KEYBOARD_IRQ == 0 {
        let due = cpu.host.pc.keys.front().is_some_and(|&(_, at)| at <= now);
        if let Some((code, _)) = if due { cpu.host.pc.keys.pop_front() } else { None } {
            cpu.host.pc.scancode = code;
            let called = call_handler(cpu, KEYBOARD_VECTOR);
            if cpu.trace {
                println!("keyboard: scan code {code:02x}, handler {}", if called { "called" } else { "missing" });            }
        }
    }

    // The timer, as often as its count says.
    if cpu.host.pc.mask & TIMER_IRQ == 0 && cpu.host.dos.vectors.contains_key(&TIMER_VECTOR) {
        let period = cpu.host.pc.period();
        if now.saturating_duration_since(cpu.host.pc.next_tick) > FURTHEST_BEHIND {
            cpu.host.pc.next_tick = now;
        }
        while cpu.host.pc.next_tick <= Instant::now() {
            cpu.host.pc.next_tick += period;
            call_handler(cpu, TIMER_VECTOR);
        }
    }
    cpu.host.pc.in_interrupt = false;
}
