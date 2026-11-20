//! SciTech UniVBE's `UV_install`, `ZED.EXE` 0x9d630 linked (0x108d630 here).
//!
//! The DOS program links UVBELib, which loads `univbe.drv`, SciTech's driver
//! for the video card, and runs `UVCONFIG.EXE` to write it when it's missing.
//! Z calls it as `UV_install(path, ignoreVBE = 1, 1)`, so it always wants that
//! driver, even when the card has VBE 2.0. No copy of Z has the driver, and it
//! could only be written by probing real hardware.
//!
//! The runtime answers `int 10h` with VBE 2.0 and a linear frame buffer
//! (`host/vga.rs`), which is what the driver would have provided. So this
//! reports success, code 0, the case Z prints as "UVBELib successfully
//! installed the driver", and loads nothing.

use recomp_runtime::Cpu;

/// cdecl: the caller removes the three arguments. The caller checks
/// `returned_to` against its own return address, as after every call.
pub fn f_0108d630(cpu: &mut Cpu) {
    cpu.eax = 0;
    cpu.returned_to = cpu.pop();
}
