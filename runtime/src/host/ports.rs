//! Hardware ports, for code that was brought over from DOS and still talks to
//! the PC's hardware with `in` and `out`.
//!
//! Only what a game is seen to use is here. A port nobody answers reads as all
//! ones, as an empty bus does, and is named once in the log.

use crate::cpu::Cpu;
use std::time::Duration;

/// The VGA's status register. Bit 3 is set while the picture's beam travels
/// back to the top, and games wait for it to time their frames.
const VGA_STATUS: u16 = 0x3da;
const VERTICAL_RETRACE: u32 = 8;
const DISPLAY_ENABLE: u32 = 1;
/// A VGA drew 640x480 sixty times a second, and the way back took about a
/// twentieth of that.
const REFRESH: Duration = Duration::from_micros(16_667);
const RETRACE: Duration = Duration::from_micros(800);
/// One line at 31.5 kHz, and the part of it that's blank.
const LINE: Duration = Duration::from_nanos(31_778);
const HORIZONTAL_BLANK: Duration = Duration::from_nanos(6_000);
/// The longest one read sleeps. A game waiting for the retrace reads the port
/// in a loop, and would otherwise keep a processor busy doing so.
const LONGEST_SLEEP: Duration = Duration::from_millis(1);

fn log_once(port: u16, action: &str) {
    static SEEN: std::sync::Mutex<Vec<(u16, bool)>> = std::sync::Mutex::new(Vec::new());
    let key = (port, action == "read");
    let mut seen = SEEN.lock().unwrap();
    if !seen.contains(&key) {
        seen.push(key);
        println!("port {port:#x} was {action}, and the runtime has nothing there");
    }
}

pub fn read(cpu: &mut Cpu, port: u16, size: u32) -> u32 {
    if let Some(value) = super::pc::read(cpu, port).or_else(|| super::vga::read(cpu, port)) {
        return value;
    }
    match port {
        VGA_STATUS => {
            let period = REFRESH.as_nanos() as u64;
            let phase = Duration::from_nanos(cpu.host.started.elapsed().as_nanos() as u64 % period);
            let in_retrace = phase >= REFRESH - RETRACE;
            if in_retrace {
                return VERTICAL_RETRACE | DISPLAY_ENABLE;
            }
            // Bit 0 is also set in each line's horizontal blank, which some
            // games wait for between palette writes (Z's fades, 0x8265b).
            let line = Duration::from_nanos(phase.as_nanos() as u64 % LINE.as_nanos() as u64);
            if line >= LINE - HORIZONTAL_BLANK {
                return DISPLAY_ENABLE;
            }
            // Sleep towards the next blank, so a waiting loop costs little.
            super::sleep(cpu, (LINE - HORIZONTAL_BLANK - line).min(LONGEST_SLEEP));
            0
        }
        _ => {
            log_once(port, "read");
            u32::MAX >> (32 - size * 8)
        }
    }
}

pub fn write(cpu: &mut Cpu, port: u16, size: u32, value: u32) {
    if super::pc::write(cpu, port, value) || super::vga::write(cpu, port, size, value) {
        return;
    }
    log_once(port, "written");
}
