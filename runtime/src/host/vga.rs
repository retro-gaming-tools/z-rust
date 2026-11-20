//! The VGA and a VESA BIOS, for recompiled DOS programs.
//!
//! DOS games drew straight into video memory: 320x200 in mode 13h at
//! 0xA0000, or, through a VESA BIOS (VBE 2.0), 640x480 and more in a linear
//! frame buffer. Colours came from the VGA's palette (its DAC), 6 bits per
//! component, set through ports 0x3C8 and 0x3C9 or VBE function 9.
//!
//! The runtime offers VBE 2.0 with 640x480 in 256 colours and a linear
//! frame buffer, the mode Z's VESA option uses, and mode 13h. What the
//! program draws there is shown in the window at every refresh, as a
//! monitor did (see `frame`).

use crate::cpu::Cpu;

/// Where the frame buffer seems to be, for DPMI's physical mapping, and how
/// much video memory there is.
const LFB_PHYSICAL: u32 = 0xe000_0000;
const VIDEO_MEMORY: u32 = 4 << 20;
const MODE_13H: u16 = 0x13;
const TEXT_MODE: u16 = 0x03;
/// The VESA modes offered: 640x400 and 640x480, in 256 colours.
const MODES: [(u16, u32, u32); 2] = [(0x100, 640, 400), (0x101, 640, 480)];
const VBE_OK: u32 = 0x004f;
const VBE_FAILED: u32 = 0x014f;
const LINEAR: u16 = 0x4000;
const WINDOW: u32 = 0xa0000;
const WINDOW_SIZE: u32 = 0x10000;

pub struct Vga {
    pub mode: u16,
    pub width: u32,
    pub height: u32,
    /// Where the program draws: the frame buffer's linear address.
    pub frame_buffer: u32,
    /// The linear frame buffer, once mapped.
    linear: u32,
    /// The 64 KB bank shown at 0xA0000, for banked VESA modes.
    bank: u32,
    pub palette: [[u8; 3]; 256],
    write_index: usize,
    read_index: usize,
    component: usize,
    /// Where the picture starts in video memory, for scrolling and page flips.
    display_start: u32,
    /// A copy of the page that went out of view at the last VBE flip, which
    /// the window shows (see function 0x4F07). Cleared on a mode change.
    shown_page: Option<Vec<u8>>,
    /// The sequencer's, graphics controller's and CRT controller's
    /// registers, by data port and index, as the program last set them.
    registers: std::collections::HashMap<(u16, u8), u8>,
    /// Each controller's selected index, by its index port.
    indexes: std::collections::HashMap<u16, u8>,
    /// With chain-4 off (mode X), the four 64 KB planes, one after the other.
    planes: Vec<u8>,
    /// Whether memory at 0xA0000 is the planes: `Cpu`'s memory accessors
    /// send it to `planar_read` and `planar_write` then.
    pub planar: bool,
    /// What the last read loaded from each plane, which write mode 1 stores.
    latches: std::cell::Cell<[u8; 4]>,
    /// How many palette components were written through port 0x3C9, for the log.
    dac_writes: u64,
}

/// The sequencer's and graphics controller's data ports, and their registers.
const SEQUENCER: u16 = 0x3c5;
const GRAPHICS: u16 = 0x3cf;
const MAP_MASK: u8 = 2;
const MEMORY_MODE: u8 = 4;
const CHAIN_4: u8 = 8;
const READ_MAP: u8 = 4;
const GRAPHICS_MODE: u8 = 5;
const BIT_MASK: u8 = 8;
const PLANE: usize = WINDOW_SIZE as usize;

/// Whether mode 13h's memory is unchained, as four planes (mode X).
fn unchained(vga: &Vga) -> bool {
    vga.mode == MODE_13H && vga.registers.get(&(SEQUENCER, MEMORY_MODE)).is_some_and(|mode| mode & CHAIN_4 == 0)
}

fn register(vga: &Vga, port: u16, index: u8, default: u8) -> u8 {
    vga.registers.get(&(port, index)).copied().unwrap_or(default)
}

/// A read of mode X memory: it loads all four latches and gives the plane
/// the read map selects (read mode 0).
pub fn planar_read(cpu: &Cpu, at: u32) -> u8 {
    let vga = &cpu.host.vga;
    let offset = (at - WINDOW) as usize & (PLANE - 1);
    let latches = [0, 1, 2, 3].map(|plane| vga.planes[plane * PLANE + offset]);
    vga.latches.set(latches);
    latches[register(vga, GRAPHICS, READ_MAP, 0) as usize & 3]
}

/// A write to mode X memory, to the planes the map mask selects. Write mode 0
/// stores the byte, through the bit mask; write mode 1 stores the latches,
/// which is how games copy between pages four pixels at a time.
pub fn planar_write(cpu: &mut Cpu, at: u32, value: u8) {
    let vga = &mut cpu.host.vga;
    let offset = (at - WINDOW) as usize & (PLANE - 1);
    let mask = register(vga, SEQUENCER, MAP_MASK, 0x0f);
    let mode = register(vga, GRAPHICS, GRAPHICS_MODE, 0) & 3;
    let bits = register(vga, GRAPHICS, BIT_MASK, 0xff);
    let latches = vga.latches.get();
    for plane in (0..4).filter(|plane| mask & (1 << plane) != 0) {
        vga.planes[plane * PLANE + offset] = match mode {
            1 => latches[plane],
            _ => (value & bits) | (latches[plane] & !bits),
        };
    }
}

/// The video state, for screenshots in the log.
pub fn describe(cpu: &Cpu) -> String {
    let vga = &cpu.host.vga;
    let crt = |index| register(vga, 0x3d5, index, 0);
    let mut used = [false; 256];
    if !vga.planar && vga.frame_buffer != 0 {
        for byte in cpu.bytes(vga.frame_buffer, vga.width * vga.height) {
            used[*byte as usize] = true;
        }
    }
    let lit = vga.palette.iter().filter(|c| c.iter().any(|v| *v != 0)).count();
    format!(
        "{} indices on screen, {lit} palette entries lit, {} DAC writes, mode {:#x} {}x{}, planar {}, map mask {:#x}, write mode {}, start {:#x}, offset {}, max scan {:#x}",
        used.iter().filter(|u| **u).count(),
        vga.dac_writes,
        vga.mode,
        vga.width,
        vga.height,
        vga.planar,
        register(vga, SEQUENCER, MAP_MASK, 0x0f),
        register(vga, GRAPHICS, GRAPHICS_MODE, 0) & 3,
        u32::from(crt(0x0c)) << 8 | u32::from(crt(0x0d)),
        crt(0x13),
        crt(0x09)
    )
}

/// A sequencer register changed. Turning chain-4 off spreads mode 13h's
/// pixels over the planes, every fourth one in each.
fn set_sequencer(cpu: &mut Cpu, index: u8, value: u8) {
    cpu.host.vga.registers.insert((SEQUENCER, index), value);
    let now = unchained(&cpu.host.vga);
    if now && !cpu.host.vga.planar {
        let data = cpu.bytes(WINDOW, WINDOW_SIZE).to_vec();
        let mut planes = vec![0u8; 4 * PLANE];
        for (address, byte) in data.iter().enumerate() {
            planes[(address & 3) * PLANE + address / 4] = *byte;
        }
        cpu.host.vga.planes = planes;
    }
    cpu.host.vga.planar = now;
}

impl Default for Vga {
    fn default() -> Vga {
        // The VGA's default colours are close enough: grey steps.
        let mut palette = [[0u8; 3]; 256];
        for (index, colour) in palette.iter_mut().enumerate() {
            *colour = [(index >> 2) as u8; 3];
        }
        Vga { mode: TEXT_MODE, width: 0, height: 0, frame_buffer: 0, linear: 0, bank: 0, palette, write_index: 0, read_index: 0, component: 0, display_start: 0, shown_page: None, registers: std::collections::HashMap::new(), indexes: std::collections::HashMap::new(), planes: Vec::new(), planar: false, latches: std::cell::Cell::new([0; 4]), dac_writes: 0 }
    }
}

/// The registers a video BIOS call sees: the program's own, or those of a
/// real-mode call through DPMI. `es` is a real-mode segment for the latter,
/// and buffers are at `es * 16 + di` then.
pub struct Registers {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub edi: u32,
    /// Where a buffer given as ES:DI is.
    pub buffer: u32,
    /// A buffer's real-mode segment, for far pointers written into it.
    pub segment: u32,
}

fn mode_size(mode: u16) -> Option<(u32, u32)> {
    MODES.iter().find(|(number, ..)| *number == mode & 0x1ff).map(|&(_, w, h)| (w, h))
}

/// Maps the linear frame buffer, as DPMI 0x0800 does: guest memory for it,
/// once. Other physical addresses (the first megabyte) are where they are.
pub fn map_physical(cpu: &mut Cpu, physical: u32) -> u32 {
    if !(LFB_PHYSICAL..LFB_PHYSICAL + VIDEO_MEMORY).contains(&physical) {
        return physical;
    }
    if cpu.host.vga.linear == 0 {
        let memory = cpu.allocate(VIDEO_MEMORY);
        cpu.fill(memory, VIDEO_MEMORY, 0);
        cpu.host.vga.linear = memory;
    }
    cpu.host.vga.linear + (physical - LFB_PHYSICAL)
}

fn set_mode(cpu: &mut Cpu, mode: u16) -> bool {
    let vga = &mut cpu.host.vga;
    vga.planar = false;
    vga.shown_page = None;
    if mode & 0xff == MODE_13H && mode < 0x100 {
        vga.mode = MODE_13H;
        (vga.width, vga.height, vga.frame_buffer) = (320, 200, WINDOW);
        vga.display_start = 0;
        // Mode 13h's registers: chain-4 on, all planes written, start 0.
        vga.registers.clear();
        vga.registers.insert((0x3c5, 2), 0x0f);
        vga.registers.insert((0x3c5, 4), 0x0e);
        vga.planes.clear();
        vga.planar = false;
        cpu.fill(WINDOW, WINDOW_SIZE, 0);
    } else if mode & 0x7f == TEXT_MODE && mode < 0x100 {
        vga.mode = TEXT_MODE;
        (vga.width, vga.height) = (0, 0);
    } else if let Some((width, height)) = mode_size(mode) {
        vga.mode = mode & 0x1ff;
        (vga.width, vga.height) = (width, height);
        vga.display_start = 0;
        vga.bank = 0;
        let linear = map_physical(cpu, LFB_PHYSICAL);
        cpu.host.vga.frame_buffer = linear;
        if mode & 0x8000 == 0 {
            cpu.fill(linear, VIDEO_MEMORY, 0);
        }
    } else {
        return false;
    }
    true
}

/// `int 10h`, with the registers in `r`. Returns false for a function the
/// runtime doesn't have.
pub fn bios(cpu: &mut Cpu, r: &mut Registers) -> bool {
    let (ah, ax) = ((r.eax >> 8) & 0xff, r.eax & 0xffff);
    let set_ax = |r: &mut Registers, value: u32| r.eax = (r.eax & !0xffff) | value;
    match (ah, ax) {
        (0x00, _) => {
            set_mode(cpu, (r.eax & 0xff) as u16);
        }
        // The current mode, and 80 columns.
        (0x0f, _) => r.eax = (r.eax & !0xffff) | 80 << 8 | (cpu.host.vga.mode & 0xff) as u32,
        // The cursor and text: nothing to show in graphics.
        (0x01 | 0x02 | 0x03 | 0x06 | 0x09 | 0x0e, _) => {}
        // Palette registers: block of DAC colours from ES:DX.
        (0x10, _) if r.eax & 0xff == 0x12 => {
            let (first, count) = ((r.ebx & 0xffff) as usize, (r.ecx & 0xffff) as usize);
            for index in 0..count.min(256 - first.min(256)) {
                let at = r.buffer + index as u32 * 3;
                cpu.host.vga.palette[first + index] = [cpu.r8(at) & 0x3f, cpu.r8(at + 1) & 0x3f, cpu.r8(at + 2) & 0x3f];
            }
        }
        (0x10, _) => {}
        // Font information: 16 lines a character, 25 rows. The font's address
        // in ES:BP isn't given: Z's hardware check only reads the sizes.
        (0x11, 0x1130) => {
            r.ecx = (r.ecx & !0xffff) | 16;
            r.edx = (r.edx & !0xff) | 24;
        }
        // EGA information (BL 10h): a colour card with 256 KB, no feature bits.
        (0x12, _) if r.ebx & 0xff == 0x10 => {
            r.ebx = (r.ebx & !0xffff) | 0x0003;
            r.ecx &= !0xffff;
        }
        // The display combination: VGA with a colour monitor, and no second card.
        (0x1a, 0x1a00) => {
            set_ax(r, 0x001a);
            r.ebx = (r.ebx & !0xffff) | 0x0008;
        }
        // VBE: the controller.
        (_, 0x4f00) => {
            let at = r.buffer;
            cpu.fill(at, 512, 0);
            cpu.bytes_mut(at, 4).copy_from_slice(b"VESA");
            cpu.w16(at + 4, 0x0200);
            // The OEM name and the mode list are inside the block, as far
            // pointers to it.
            let far = |offset: u32| (r.segment << 16) | ((r.buffer - r.segment * 16) + offset);
            cpu.write_string(at + 0x100, "Z-Rust VBE");
            cpu.w32(at + 6, far(0x100));
            cpu.w32(at + 0x0a, 0);
            cpu.w32(at + 0x0e, far(0x120));
            for (index, (mode, ..)) in MODES.iter().enumerate() {
                cpu.w16(at + 0x120 + index as u32 * 2, *mode);
            }
            cpu.w16(at + 0x120 + MODES.len() as u32 * 2, 0xffff);
            cpu.w16(at + 0x12, (VIDEO_MEMORY >> 16) as u16);
            set_ax(r, VBE_OK);
        }
        // VBE: a mode.
        (_, 0x4f01) => {
            let Some((width, height)) = mode_size((r.ecx & 0xffff) as u16) else {
                set_ax(r, VBE_FAILED);
                return true;
            };
            let at = r.buffer;
            cpu.fill(at, 256, 0);
            // Supported, colour, graphics, linear frame buffer available.
            cpu.w16(at, 0x009b);
            cpu.w8(at + 2, 7);
            cpu.w16(at + 4, 64);
            cpu.w16(at + 6, 64);
            cpu.w16(at + 8, (WINDOW >> 4) as u16);
            cpu.w16(at + 0x10, width as u16);
            cpu.w16(at + 0x12, width as u16);
            cpu.w16(at + 0x14, height as u16);
            cpu.w8(at + 0x16, 8);
            cpu.w8(at + 0x17, 16);
            cpu.w8(at + 0x18, 1);
            cpu.w8(at + 0x19, 8);
            cpu.w8(at + 0x1a, 1);
            cpu.w8(at + 0x1b, 4);
            cpu.w8(at + 0x1d, (VIDEO_MEMORY / (width * height)).saturating_sub(1) as u8);
            cpu.w8(at + 0x1e, 1);
            cpu.w32(at + 0x28, LFB_PHYSICAL);
            set_ax(r, VBE_OK);
        }
        (_, 0x4f02) => {
            let mode = (r.ebx & 0xffff) as u16;
            let ok = set_mode(cpu, mode & !LINEAR) || set_mode(cpu, mode);
            set_ax(r, if ok { VBE_OK } else { VBE_FAILED });
        }
        (_, 0x4f03) => {
            r.ebx = (r.ebx & !0xffff) | cpu.host.vga.mode as u32;
            set_ax(r, VBE_OK);
        }
        // VBE: the bank shown in the window at 0xA0000. Banked drawing goes
        // to the window, and is copied into the frame buffer on each switch.
        (_, 0x4f05) => {
            if r.ebx & 0xff00 == 0 {
                flush_bank(cpu);
                cpu.host.vga.bank = r.edx & 0xffff;
                let source = cpu.host.vga.frame_buffer + cpu.host.vga.bank * WINDOW_SIZE;
                let data = cpu.bytes(source, WINDOW_SIZE).to_vec();
                cpu.bytes_mut(WINDOW, WINDOW_SIZE).copy_from_slice(&data);
            } else {
                r.edx = (r.edx & !0xffff) | cpu.host.vga.bank;
            }
            set_ax(r, VBE_OK);
        }
        // VBE: bytes a scan line, and where the picture starts.
        (_, 0x4f06) => {
            let width = cpu.host.vga.width;
            r.ebx = (r.ebx & !0xffff) | width;
            r.ecx = (r.ecx & !0xffff) | width;
            r.edx = (r.edx & !0xffff) | (VIDEO_MEMORY / width.max(1)).min(0xffff);
            set_ax(r, VBE_OK);
        }
        (_, 0x4f07) => {
            if r.ebx & 0xff == 0 || r.ebx & 0xff == 0x80 {
                // Z draws a frame on the hidden page, shows it, and then puts
                // its pointer on the page shown. The page going out of view
                // is therefore the one that's complete, pointer and all: the
                // window shows a copy of it, a frame late.
                let vga = &cpu.host.vga;
                if vga.frame_buffer != 0 && vga.width != 0 {
                    let leaving = cpu.bytes(vga.frame_buffer + vga.display_start, vga.width * vga.height).to_vec();
                    cpu.host.vga.shown_page = Some(leaving);
                }
                cpu.host.vga.display_start = (r.edx & 0xffff) * cpu.host.vga.width + (r.ecx & 0xffff);
                page_flipped(cpu);
            } else {
                let width = cpu.host.vga.width.max(1);
                r.ecx = (r.ecx & !0xffff) | cpu.host.vga.display_start % width;
                r.edx = (r.edx & !0xffff) | cpu.host.vga.display_start / width;
            }
            set_ax(r, VBE_OK);
        }
        // VBE: the DAC stays 6 bits wide.
        (_, 0x4f08) => {
            r.ebx = (r.ebx & !0xff00) | 6 << 8;
            set_ax(r, VBE_OK);
        }
        // VBE: palette entries from ES:DI, blue, green, red, padding.
        (_, 0x4f09) => {
            if r.ebx & 0xff == 0 || r.ebx & 0xff == 0x80 {
                let (count, first) = ((r.ecx & 0xffff) as usize, (r.edx & 0xffff) as usize);
                for index in 0..count.min(256 - first.min(256)) {
                    let at = r.buffer + index as u32 * 4;
                    cpu.host.vga.palette[first + index] = [cpu.r8(at + 2) & 0x3f, cpu.r8(at + 1) & 0x3f, cpu.r8(at) & 0x3f];
                }
            }
            set_ax(r, VBE_OK);
        }
        // VBE: no protected-mode interface; the program calls the BIOS.
        (_, 0x4f0a) => set_ax(r, VBE_FAILED),
        _ => return false,
    }
    true
}

/// Two 16-bit colours mixed, `share` 256ths of the way from `a` to `b`.
fn blend(a: u16, b: u16, share: u32) -> u16 {
    if share == 0 || a == b {
        return a;
    }
    let mix = |shift: u32, mask: u32| {
        let (from, to) = ((a as u32 >> shift) & mask, (b as u32 >> shift) & mask);
        ((from * (256 - share) + to * share + 128) / 256) << shift
    };
    (mix(11, 31) | mix(5, 63) | mix(0, 31)) as u16
}

/// Mode X's page on screen: pixel x is in plane x % 4, at x / 4 in its row,
/// from the CRT controller's start address, which games flip pages with.
fn unchained_page(vga: &Vga) -> Vec<u8> {
    let crt = |index, default| u32::from(vga.registers.get(&(0x3d5, index)).copied().unwrap_or(default));
    let start = crt(START_HIGH, 0) << 8 | crt(START_LOW, 0);
    let row_bytes = crt(0x13, 40) * 2;
    (0..vga.width * vga.height)
        .map(|i| {
            let (x, y) = (i % vga.width, i / vga.width);
            let offset = (start + y * row_bytes + x / 4) & (WINDOW_SIZE - 1);
            vga.planes[(x & 3) as usize * WINDOW_SIZE as usize + offset as usize]
        })
        .collect()
}

/// Mode X is about to show another page: keep the one leaving, as for VBE.
fn keep_leaving_page(cpu: &mut Cpu) {
    let vga = &cpu.host.vga;
    if unchained(vga) && !vga.planes.is_empty() && vga.width != 0 {
        cpu.host.vga.shown_page = Some(unchained_page(&cpu.host.vga));
    }
}

/// How long after a flip the game counts as flipping: longer than Z's
/// slowest frames (18 a second in missions).
const STILL_FLIPPING: std::time::Duration = std::time::Duration::from_millis(150);

/// The CRT controller's start address, high byte, written first of the two.
const START_HIGH: u8 = 0x0c;

/// The CRT controller's start address, low byte: written last when a game
/// flips its page, after the high byte.
const START_LOW: u8 = 0x0d;

/// The game showed another page. The window shows a DOS program's picture at
/// its flips, as for a DirectDraw Flip, so it never catches a page halfway
/// drawn, such as the pointer taken off and not yet put back.
fn page_flipped(cpu: &mut Cpu) {
    cpu.host.draw.frames += 1;
    cpu.host.draw.last_flip = std::time::Instant::now();
}

/// Banked drawing: what was drawn in the window goes back to its bank.
fn flush_bank(cpu: &mut Cpu) {
    let vga = &cpu.host.vga;
    if vga.mode < 0x100 || vga.frame_buffer == 0 {
        return;
    }
    let target = vga.frame_buffer + vga.bank * WINDOW_SIZE;
    let data = cpu.bytes(WINDOW, WINDOW_SIZE).to_vec();
    cpu.bytes_mut(target, WINDOW_SIZE).copy_from_slice(&data);
}

/// The DAC's ports: 0x3C7 picks a colour to read, 0x3C8 one to write, and
/// 0x3C9 reads or writes its red, green and blue in turn.
pub fn read(cpu: &mut Cpu, port: u16) -> Option<u32> {
    let vga = &mut cpu.host.vga;
    Some(match port {
        0x3c9 => {
            let value = vga.palette[vga.read_index & 0xff][vga.component];
            vga.component += 1;
            if vga.component == 3 {
                vga.component = 0;
                vga.read_index += 1;
            }
            value as u32
        }
        0x3c8 => vga.write_index as u32 & 0xff,
        // Index ports and their data ports: the value last written.
        0x3c4 | 0x3ce | 0x3d4 => u32::from(vga.indexes.get(&port).copied().unwrap_or(0)),
        0x3c5 | 0x3cf | 0x3d5 => {
            let index = vga.indexes.get(&(port - 1)).copied().unwrap_or(0);
            u32::from(vga.registers.get(&(port, index)).copied().unwrap_or(0))
        }
        _ => return None,
    })
}

/// The window's frame, which every mode is shown in.
const WIDTH: u32 = super::GAME_WIDTH as u32;
const HEIGHT: u32 = super::GAME_HEIGHT as u32;

/// Where the picture sits in the 640x480 frame: left, top, width and height.
/// A VGA monitor showed every mode over its whole 4:3 screen, so 640x400 and
/// 320x200 are stretched to fill the frame, as there. The mouse driver uses
/// this to put the game's pointer where the real one is.
pub fn shown_area(vga: &Vga) -> (u32, u32, u32, u32) {
    if vga.width == 0 || vga.height == 0 {
        return (0, 0, 0, 0);
    }
    (0, 0, WIDTH, HEIGHT)
}

/// `size` is in bytes: an `out dx,ax` writes two ports at once.
pub fn write(cpu: &mut Cpu, port: u16, size: u32, value: u32) -> bool {
    let vga = &mut cpu.host.vga;
    match port {
        0x3c7 => {
            vga.read_index = (value & 0xff) as usize;
            vga.component = 0;
        }
        0x3c8 => {
            vga.write_index = (value & 0xff) as usize;
            vga.component = 0;
        }
        0x3c9 => {
            vga.dac_writes += 1;
            vga.palette[vga.write_index & 0xff][vga.component] = (value & 0x3f) as u8;
            vga.component += 1;
            if vga.component == 3 {
                vga.component = 0;
                vga.write_index += 1;
            }
        }
        // The sequencer's, graphics and CRT controllers' registers, which a
        // game sets for its mode. They're kept, to be read back, and change
        // nothing else here. A 16-bit write sets the index and the value, also
        // when the value is 0: Z sets its page's start address with
        // `out dx,ax`, and the page at 0x0010 has a high byte of 0.
        0x3c4 | 0x3ce | 0x3d4 => {
            vga.indexes.insert(port, value as u8);
            if size >= 2 {
                if port == 0x3c4 {
                    set_sequencer(cpu, value as u8, (value >> 8) as u8);
                } else {
                    if port == 0x3d4 && value as u8 == START_HIGH {
                        keep_leaving_page(cpu);
                    }
                    cpu.host.vga.registers.insert((port + 1, value as u8), (value >> 8) as u8);
                    if port == 0x3d4 && value as u8 == START_LOW {
                        page_flipped(cpu);
                    }
                }
            }
        }
        0x3c5 | 0x3cf | 0x3d5 => {
            let index = vga.indexes.get(&(port - 1)).copied().unwrap_or(0);
            if port == SEQUENCER {
                set_sequencer(cpu, index, value as u8);
            } else {
                if port == 0x3d5 && index == START_HIGH {
                    keep_leaving_page(cpu);
                }
                cpu.host.vga.registers.insert((port, index), value as u8);
                if port == 0x3d5 && index == START_LOW {
                    page_flipped(cpu);
                }
            }
        }
        // The attribute controller and the rest.
        0x3c0..=0x3cf => {}
        _ => return false,
    }
    true
}

/// The picture for the window, as 640x480 16-bit pixels, from what the
/// program drew and the palette: `None` in text mode. 320x200 is doubled,
/// and a smaller picture is centred.
pub fn frame(cpu: &mut Cpu) -> Option<Vec<u16>> {
    if cpu.host.vga.mode == TEXT_MODE || cpu.host.vga.width == 0 {
        return None;
    }
    flush_bank(cpu);
    // A tweaked mode X sets its own size through the CRT controller.
    if unchained(&cpu.host.vga) {
        let vga = &mut cpu.host.vga;
        let crt = |index: u8, default: u8| u32::from(vga.registers.get(&(0x3d5, index)).copied().unwrap_or(default));
        let overflow = crt(0x07, 0x1f);
        let display_end = (crt(0x12, 0x8f) | (overflow >> 1 & 1) << 8 | (overflow >> 6 & 1) << 9) + 1;
        let scan = crt(0x09, 0x41);
        let lines = display_end / ((scan & 0x1f) + 1) / if scan & 0x80 != 0 { 2 } else { 1 };
        let width = (crt(0x01, 0x4f) + 1) * 4;
        (vga.width, vga.height) = (width.clamp(8, 640), lines.clamp(1, 480));
    }
    let vga = &cpu.host.vga;
    let table: Vec<u16> = vga.palette.iter().map(|c| ((c[0] as u16) >> 1) << 11 | (c[1] as u16) << 5 | (c[2] as u16) >> 1).collect();
    let (left, top, shown_width, shown_height) = shown_area(vga);
    // Mode X: pixel x is in plane x % 4, at x / 4 in its row, from the CRT
    // controller's start address, which games flip pages with.
    let unchained_pixels: Vec<u8>;
    // The kept page only while the game flips: screens drawn without
    // flipping, such as the logos and loading pictures, show as they are.
    // Keeping the last flip's page instead showed it for seconds, often blank.
    let flipping = cpu.host.draw.last_flip.elapsed() < STILL_FLIPPING;
    let kept = vga.shown_page.as_ref().filter(|page| flipping && page.len() == (vga.width * vga.height) as usize);
    let pixels = if let Some(page) = kept {
        &page[..]
    } else if unchained(vga) && !vga.planes.is_empty() {
        unchained_pixels = unchained_page(vga);
        &unchained_pixels[..]
    } else {
        cpu.bytes(vga.frame_buffer + vga.display_start, vga.width * vga.height)
    };
    let mut picture = vec![0u16; (WIDTH * HEIGHT) as usize];
    // 400 or 200 lines become 480 as a monitor's beam drew them: each line of
    // the window takes its share of the one or two lines it falls on, so no
    // line is doubled and letters keep even strokes.
    let row_of = |y: u32| &pixels[(y.min(vga.height - 1) * vga.width) as usize..][..vga.width as usize];
    for y in 0..shown_height {
        // The source line under this line's middle, in 256ths.
        let middle = ((2 * y + 1) * vga.height * 256 / (2 * shown_height)).saturating_sub(128);
        let (first, share) = (middle / 256, middle % 256);
        let (above, below) = (row_of(first), row_of(first + 1));
        let target = &mut picture[((top + y) * WIDTH + left) as usize..][..shown_width as usize];
        for (x, pixel) in target.iter_mut().enumerate() {
            let source = x * vga.width as usize / shown_width as usize;
            *pixel = blend(table[above[source] as usize], table[below[source] as usize], share);
        }
    }
    Some(picture)
}
