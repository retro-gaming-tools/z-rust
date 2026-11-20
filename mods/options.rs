//! The DOS program's first start, `ZED.EXE` 0x42614 linked (0x1032614 here).
//!
//! With no `options.cfg`, `ZED.EXE` runs a graphics speed profile (this
//! function) and writes the file from its result. The profile sets the detail
//! levels, but never the SVGA option (`0xff352`, byte 196 of the file), which
//! stays at the program's built-in "off". On a real PC the DOS installer wrote
//! `options.cfg` with SVGA on; a disc image has no installer, and players are
//! never meant to set up graphics themselves. So after the profile, SVGA is
//! switched on whenever the game found it can run SVGA (`0x115e6a`), as the
//! installer left it. The player can still switch it off in the menu, and
//! the saved choice is kept, since this only runs while the file is created.

use crate::code::*;
use recomp_runtime::Cpu;

/// Where `ZED.EXE`'s data sits here: linked address + 0xff0000.
const SVGA_OPTION: u32 = 0xff352 + 0xff0000;
const SVGA_POSSIBLE: u32 = 0x115e6a + 0xff0000;

pub fn f_01032614(cpu: &mut Cpu) {
    o_01032614(cpu);
    if cpu.r8(SVGA_POSSIBLE) != 0 {
        cpu.w8(SVGA_OPTION, 1);
    }
}
