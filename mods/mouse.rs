//! The game's pointer, `WINZ.EXE` 0x402a10.
//!
//! The game reads how far the mouse moved from DirectInput and adds that to a
//! pointer position of its own, which it keeps in quarter pixels and limits to
//! the screen. Such a pointer starts wherever the game put it and drifts away
//! from the real one: in a window, or with a touch screen or a controller, a
//! click then lands somewhere else than where the player pointed.
//!
//! This lets the original read the mouse, for the buttons, and then puts the
//! game's pointer where the real one is.
//!
//! The game also moves its pointer itself (0x402db0, its `int 33h` function
//! 4): a right press puts it in the middle, and while the button is held the
//! view scrolls by how far it moved from there. Snapping it back to the real
//! pointer then looked like a drag, so every right click scrolled and never
//! counted as a click (deselect). When the game has moved its pointer, the
//! real pointer goes there instead, as Windows' `SetCursorPos` would.

use crate::code::*;
use recomp_runtime::Cpu;
use std::sync::Mutex;

/// Where this mod last put the game's pointer, in the game's units.
static PUT: Mutex<Option<(i32, i32)>> = Mutex::new(None);

const X: u32 = 0x5eca00;
const Y: u32 = 0x5eca04;
const X_LIMITS: (u32, u32) = (0x5eca08, 0x5eca0c);
const Y_LIMITS: (u32, u32) = (0x5eca10, 0x5eca14);
/// How far the pointer moved in the last read, which the game keeps too.
const MOVED_X: u32 = 0x5eca1c;
const MOVED_Y: u32 = 0x5eca20;
const UNITS_PER_PIXEL: i32 = 4;

pub fn f_00402a10(cpu: &mut Cpu) {
    let before = (cpu.r32(X) as i32, cpu.r32(Y) as i32);
    let put = *PUT.lock().unwrap();
    if put.is_some_and(|put| put != before) {
        let to_pixel = |value: i32| (value.max(0) / UNITS_PER_PIXEL) as u32;
        recomp_runtime::host::video::set_cursor(cpu, to_pixel(before.0), to_pixel(before.1));
    }
    o_00402a10(cpu);
    if cpu.eax == 0 {
        return; // There is no mouse device yet.
    }
    let (pointer_x, pointer_y) = cpu.host.pointer;
    let limit = |cpu: &Cpu, value: u32, (low, high): (u32, u32)| (value as i32 * UNITS_PER_PIXEL).clamp(cpu.r32(low) as i32, cpu.r32(high) as i32);
    let x = limit(cpu, pointer_x, X_LIMITS);
    let y = limit(cpu, pointer_y, Y_LIMITS);
    cpu.w32(X, x as u32);
    cpu.w32(Y, y as u32);
    *PUT.lock().unwrap() = Some((x, y));
    cpu.w32(MOVED_X, (x - before.0) as u32);
    cpu.w32(MOVED_Y, (y - before.1) as u32);
}
