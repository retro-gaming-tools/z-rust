//! The DOS mouse driver, `int 33h`, for recompiled DOS programs.
//!
//! The player never installs a driver: the runtime is the driver, and it
//! reports the real pointer, which the window keeps in game pixels
//! (`host.pointer`, 640x480), in whatever range the program asked for.
//!
//! Only what a game reaches is answered, as in `dos`.

use crate::cpu::Cpu;

const SCREEN: (i32, i32) = (640, 480);

pub struct Mouse {
    /// The ranges the program set with functions 7 and 8, inclusive.
    x_range: (i32, i32),
    y_range: (i32, i32),
    /// Mickeys per 8 pixels, from function 0Fh.
    ratio: (i32, i32),
    /// Where the pointer was at the last motion read (function 0Bh).
    seen: Option<(u32, u32)>,
    /// The program's event handler (function 0Ch): its offset and the events
    /// it wants, and what it was last told: pointer, buttons and mickeys.
    handler: Option<(u32, u16)>,
    told: Option<((u32, u32), u32)>,
    mickeys: (i32, i32),
}

impl Default for Mouse {
    fn default() -> Mouse {
        Mouse { x_range: (0, SCREEN.0 - 1), y_range: (0, SCREEN.1 - 1), ratio: (8, 16), seen: None, handler: None, told: None, mickeys: (0, 0) }
    }
}

/// A pointer coordinate, 0 to `size` - 1, in the program's range. The last
/// pixel is the range's end, as a real driver gives when the mouse is pushed
/// against the edge: Z's DOS missions set 0-2552 (320 in eighths) and scroll
/// right only at its end, which plain scaling (2549) never reached.
fn to_range(value: u32, size: i32, (low, high): (i32, i32)) -> i32 {
    if value as i32 >= size - 1 {
        return high;
    }
    low + (value as i32 * (high - low + 1) / size).min(high - low)
}

/// The other way: a coordinate in the program's range to the pointer's.
fn from_range(value: i32, size: i32, (low, high): (i32, i32)) -> u32 {
    let span = (high - low + 1).max(1);
    ((value.clamp(low, high) - low) * size / span).clamp(0, size - 1) as u32
}

/// The part of the window the picture fills: left, top, width and height.
/// A doubled 320x200 has bars above and below, which aren't the game's.
fn area(cpu: &Cpu) -> (i32, i32, i32, i32) {
    let (left, top, width, height) = super::vga::shown_area(&cpu.host.vga);
    if width == 0 || height == 0 {
        return (0, 0, SCREEN.0, SCREEN.1);
    }
    (left as i32, top as i32, width as i32, height as i32)
}

/// The real pointer, in the ranges the program set.
fn position(cpu: &Cpu, (x, y): (u32, u32)) -> (i32, i32) {
    let (left, top, width, height) = area(cpu);
    let inside = |value: u32, start: i32, size: i32| (value as i32 - start).clamp(0, size - 1) as u32;
    let mouse = &cpu.host.mouse;
    (to_range(inside(x, left, width), width, mouse.x_range), to_range(inside(y, top, height), height, mouse.y_range))
}

/// The other way, for a program that moves the pointer.
fn pointer_at(cpu: &Cpu, x: i32, y: i32) -> (u32, u32) {
    let (left, top, width, height) = area(cpu);
    let mouse = &cpu.host.mouse;
    (from_range(x, width, mouse.x_range) + left as u32, from_range(y, height, mouse.y_range) + top as u32)
}

fn set16(register: &mut u32, value: i32) {
    *register = (*register & !0xffff) | (value as u32 & 0xffff);
}

/// Answers `int 33h`, or returns false for a function it doesn't have.
pub fn call(cpu: &mut Cpu) -> bool {
    let ax = cpu.eax & 0xffff;
    let (cx, dx) = ((cpu.ecx & 0xffff) as u16 as i16 as i32, (cpu.edx & 0xffff) as u16 as i16 as i32);
    match ax {
        // Reset: a driver is there, with two buttons. Ranges go back to the screen.
        0x0000 | 0x0021 => {
            cpu.host.mouse = Mouse::default();
            set16(&mut cpu.eax, 0xffff);
            set16(&mut cpu.ebx, 2);
        }
        // Show and hide the driver's cursor. Games draw their own.
        0x0001 | 0x0002 => {}
        // The cursor's look, graphics or text, and the area to keep it out of.
        0x0009 | 0x000a | 0x0010 => {}
        // Speed-ups, sensitivity and the display page: the real pointer
        // already moves as the player set it up.
        0x0013 | 0x001a | 0x001d => {}
        // The position and the buttons: bit 0 left, bit 1 right.
        0x0003 => {
            let buttons = cpu.host.input.mouse_buttons();
            let (x, y) = position(cpu, cpu.host.pointer);
            set16(&mut cpu.ebx, buttons as i32);
            set16(&mut cpu.ecx, x);
            set16(&mut cpu.edx, y);
        }
        // Move the pointer.
        // Z's right-button drag puts the pointer in the middle each time and
        // scrolls by how far it went. Like Windows' `SetCursorPos`, the window
        // then follows the mouse's movement while a button is held, or the
        // next real event would jump back to the cursor and the drag could
        // only go towards the top left.
        0x0004 => {
            let (x, y) = pointer_at(cpu, cx, dx);
            super::video::set_cursor(cpu, x, y);
        }
        // The horizontal and the vertical range.
        0x0007 => cpu.host.mouse.x_range = (cx.min(dx), cx.max(dx)),
        0x0008 => cpu.host.mouse.y_range = (cx.min(dx), cx.max(dx)),
        // How far it moved since the last read, in mickeys.
        0x000b => {
            let pointer = cpu.host.pointer;
            let mouse = &mut cpu.host.mouse;
            let seen = mouse.seen.replace(pointer).unwrap_or(pointer);
            let dx = (pointer.0 as i32 - seen.0 as i32) * mouse.ratio.0 / 8;
            let dy = (pointer.1 as i32 - seen.1 as i32) * mouse.ratio.1 / 8;
            set16(&mut cpu.ecx, dx);
            set16(&mut cpu.edx, dy);
        }
        // The driver's version, 8.20 in BCD, and its mouse: PS/2 (type 4), so no IRQ.
        0x0024 => {
            set16(&mut cpu.ebx, 0x0820);
            set16(&mut cpu.ecx, 0x0400);
        }
        // The event handler, at ES:EDX, called far for the events in CX.
        0x000c | 0x0014 => {
            let old = cpu.host.mouse.handler.unwrap_or((0, 0));
            let offset = cpu.edx;
            if cpu.trace {
                println!("  mouse handler at {offset:#x}, events {:#x}", cpu.ecx & 0xffff);
            }
            let mask = (cpu.ecx & 0xffff) as u16;
            cpu.host.mouse.handler = (offset != 0 && mask != 0).then_some((offset, mask));
            if ax == 0x0014 {
                // Swap: the old handler comes back in ES:EDX and CX.
                cpu.edx = old.0;
                set16(&mut cpu.ecx, old.1 as i32);
            }
        }
        // Mickeys per 8 pixels.
        0x000f => cpu.host.mouse.ratio = (cx.max(1), dx.max(1)),
        _ => return false,
    }
    true
}

/// Events for the handler: the pointer moved, then a press and a release
/// for each of the left and the right button.
const MOVED: u16 = 1;
const PRESSED: [u16; 2] = [2, 8];
const RELEASED: [u16; 2] = [4, 16];

/// What changed since the handler was last told, as its event bits.
fn events(before: Option<((u32, u32), u32)>, pointer: (u32, u32), buttons: u32) -> u16 {
    let Some((was, held)) = before else { return MOVED };
    let mut events = if was != pointer { MOVED } else { 0 };
    for button in 0..2 {
        let (then, now) = (held >> button & 1, buttons >> button & 1);
        if now > then {
            events |= PRESSED[button];
        } else if now < then {
            events |= RELEASED[button];
        }
    }
    events
}

/// Calls the program's handler when the mouse did something it asked for:
/// at the program's safe points, from `pc::service`. A click that came and
/// went between two calls is told as a press, and its release the next time.
pub fn service(cpu: &mut Cpu) {
    let Some((offset, mask)) = cpu.host.mouse.handler else { return };
    let pointer = cpu.host.pointer;
    let buttons = cpu.host.input.mouse_buttons();
    let mouse = &mut cpu.host.mouse;
    let happened = events(mouse.told, pointer, buttons);
    if happened == 0 {
        return;
    }
    if let Some(((x, y), _)) = mouse.told {
        mouse.mickeys.0 += (pointer.0 as i32 - x as i32) * mouse.ratio.0 / 8;
        mouse.mickeys.1 += (pointer.1 as i32 - y as i32) * mouse.ratio.1 / 8;
    }
    mouse.told = Some((pointer, buttons));
    if happened & mask == 0 {
        return;
    }
    let mickeys = mouse.mickeys;
    let (x, y) = position(cpu, pointer);
    let saved = super::timers::Registers::of(cpu);
    cpu.eax = (happened & mask) as u32;
    cpu.ebx = buttons;
    cpu.ecx = x as u32 & 0xffff;
    cpu.edx = y as u32 & 0xffff;
    cpu.esi = mickeys.0 as u32 & 0xffff;
    cpu.edi = mickeys.1 as u32 & 0xffff;
    cpu.df = false;
    // A far call: the code selector, then the return address, for its `retf`.
    cpu.push(super::dos::FLAT as u32);
    cpu.push(0);
    // The driver calls it from its interrupt, with interrupts off.
    cpu.interrupts_off = true;
    cpu.call(offset);
    saved.restore(cpu);
    cpu.interrupts_off = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges_map_both_ways() {
        assert_eq!(to_range(0, 640, (0, 639)), 0);
        assert_eq!(to_range(639, 640, (0, 639)), 639);
        assert_eq!(to_range(320, 640, (0, 319)), 160);
        assert_eq!(to_range(479, 480, (0, 199)), 199);
        assert_eq!(to_range(639, 640, (0, 0x9f8)), 0x9f8);
        assert_eq!(to_range(399, 400, (0, 0x638)), 0x638);
        assert_eq!(from_range(160, 640, (0, 319)), 320);
        assert_eq!(from_range(1000, 640, (0, 319)), 638);
        assert_eq!(from_range(-5, 480, (10, 20)), 0);
    }

    #[test]
    fn events_name_what_changed() {
        assert_eq!(events(None, (0, 0), 0), MOVED);
        assert_eq!(events(Some(((1, 1), 0)), (1, 1), 0), 0);
        assert_eq!(events(Some(((1, 1), 0)), (2, 1), 1), MOVED | PRESSED[0]);
        assert_eq!(events(Some(((1, 1), 3)), (1, 1), 1), RELEASED[1]);
    }
}
