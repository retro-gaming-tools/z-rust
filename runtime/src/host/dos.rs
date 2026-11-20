//! DOS, the BIOS and the DOS/4GW extender, for recompiled DOS programs.
//!
//! A flat 32-bit DOS program has no imports: it asks for files, memory, the
//! screen, the mouse and the clock through software interrupts, `int 21h` for
//! DOS, `int 31h` for the extender's DPMI services, `int 10h` for the video
//! BIOS and so on. Each `int` in the translated code calls `interrupt` here,
//! with the registers as the program set them.
//!
//! Memory is laid out as DOS/4GW gave it: the first megabyte is DOS's, with
//! the BIOS data at 0x400, the program's PSP and environment, and the VGA
//! memory at 0xA0000, and the program itself is loaded higher up (see `le`).
//! Its data selector is flat, based at 0. The PSP and the environment have
//! selectors of their own, as under DOS/4GW, which the program's start-up
//! code reads them through.
//!
//! Only what a game reaches is answered. Anything else stops with a named
//! error, so gaps show up as they're met, as missing imports do for Windows.

use crate::cpu::Cpu;
use std::collections::HashMap;

/// The flat selector of code, data and stack, based at 0.
pub const FLAT: u16 = 0x2b;
/// DOS/4GW's selector for the PSP, which Watcom's start-up code expects.
const PSP_SELECTOR: u16 = 0x24;
const ENVIRONMENT_SELECTOR: u16 = 0x2c;
/// Where the PSP and the environment are, in the first megabyte.
const PSP: u32 = 0x5000;
const ENVIRONMENT: u32 = 0x5200;
const ES: usize = 0;
const CS: usize = 1;
const SS: usize = 2;
const DS: usize = 3;
const FS: usize = 4;
const GS: usize = 5;

#[derive(Default)]
pub struct Dos {
    /// Selectors the program can load, and their bases.
    selectors: HashMap<u16, u32>,
    /// DOS file handles, from 5 up, and the runtime's handles for them.
    handles: HashMap<u16, u32>,
    /// Where `find first` writes what it found (DOS's disk transfer area).
    transfer_area: u32,
    /// What `find first` found and `find next` hands out.
    found: Vec<(String, bool, u64)>,
    /// Interrupt handlers the program set: selector and offset.
    pub vectors: HashMap<u8, (u16, u32)>,
    /// Where the next DOS memory block goes.
    dos_next: u32,
    /// The handler every vector has before the program sets its own: a
    /// runtime function that returns as `iretd` does (see `previous_handler`).
    previous: u32,
    /// Whether a DOS program runs, so paths follow DOS's drives and folders.
    pub(crate) dos_mode: bool,
    /// The current drive, 0 for A:.
    pub(crate) drive: u8,
    /// The CD drive, which is the disc folder: the folder above the program's.
    pub(crate) cd_drive: Option<u8>,
    /// Each drive's current folder, without drive or leading backslash.
    pub(crate) folders: HashMap<u8, String>,
}

impl Dos {
    /// A DOS path made whole: its drive, and its folder from the root, with backslashes.
    pub(crate) fn absolute(&self, guest: &str) -> (u8, String) {
        let text = guest.replace('/', "\\");
        let bytes = text.as_bytes();
        let (drive, rest) = if bytes.len() >= 2 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
            (bytes[0].to_ascii_uppercase() - b'A', &text[2..])
        } else {
            (self.drive, &text[..])
        };
        let joined = if let Some(root) = rest.strip_prefix('\\') {
            root.to_string()
        } else {
            match self.folders.get(&drive).filter(|f| !f.is_empty()) {
                Some(folder) => format!("{folder}\\{rest}"),
                None => rest.to_string(),
            }
        };
        // `.` and `..` resolved, as DOS does.
        let mut parts: Vec<&str> = Vec::new();
        for part in joined.split('\\').filter(|p| !p.is_empty() && *p != ".") {
            if part == ".." {
                parts.pop();
            } else {
                parts.push(part);
            }
        }
        (drive, parts.join("\\"))
    }
}

/// What an interrupt vector held before the program set it: DOS's or the
/// BIOS's handler, which has nothing left to do here, as the runtime does
/// their work itself. A handler that chains to it, by a far call after
/// `pushfd` or a far jump, comes back as if from `iretd`.
fn previous_handler(cpu: &mut Cpu) {
    cpu.returned_to = cpu.pop();
    cpu.pop();
    let flags = cpu.pop();
    cpu.set_flags(flags);
}

/// The vector's handler: the program's, or the previous one.
fn vector(cpu: &Cpu, number: u8) -> (u16, u32) {
    cpu.host.dos.vectors.get(&number).copied().unwrap_or((FLAT, cpu.host.dos.previous))
}

/// An address the program passed in a segment and an offset register.
fn at(cpu: &Cpu, segment: usize, offset: u32) -> u32 {
    cpu.bases[segment].wrapping_add(offset)
}

/// Sets or clears the carry flag, DOS's sign of an error, with the error code in AX.
fn result(cpu: &mut Cpu, outcome: Result<u32, u32>) {
    match outcome {
        Ok(value) => {
            set_ax(cpu, value);
            cpu.cf = false;
        }
        Err(code) => {
            set_ax(cpu, code);
            cpu.cf = true;
        }
    }
}

const FILE_NOT_FOUND: u32 = 2;
const PATH_NOT_FOUND: u32 = 3;
const ACCESS_DENIED: u32 = 5;
const BAD_HANDLE: u32 = 6;
const NO_MORE_FILES: u32 = 0x12;
const FIRST_HANDLE: u16 = 5;

fn new_handle(cpu: &mut Cpu, host: u32) -> u32 {
    let handle = (FIRST_HANDLE..).find(|h| !cpu.host.dos.handles.contains_key(h)).unwrap();
    cpu.host.dos.handles.insert(handle, host);
    handle as u32
}

/// `int 21h` AH=3Ch, 3Dh: creates or opens the file named at DS:EDX.
fn open(cpu: &mut Cpu, create: bool) -> Result<u32, u32> {
    const GENERIC_READ: u32 = 0x8000_0000;
    const GENERIC_WRITE: u32 = 0x4000_0000;
    const OPEN_EXISTING: u32 = 3;
    const CREATE_ALWAYS: u32 = 2;
    const INVALID: u32 = 0xffff_ffff;
    let name = cpu.string(at(cpu, DS, cpu.edx));
    let writing = create || cpu.eax & 3 != 0;
    let access = GENERIC_READ | if writing { GENERIC_WRITE } else { 0 };
    let host = super::kernel32::create_file(cpu, name, access, if create { CREATE_ALWAYS } else { OPEN_EXISTING });
    if host == INVALID {
        return Err(FILE_NOT_FOUND);
    }
    Ok(new_handle(cpu, host))
}

fn host_handle(cpu: &Cpu) -> Option<u32> {
    cpu.host.dos.handles.get(&(cpu.ebx as u16)).copied()
}

/// `int 21h` AH=42h: moves a file's position, CX:DX from where AL says.
fn seek(cpu: &mut Cpu) -> Result<u32, u32> {
    use std::io::{Seek, SeekFrom};
    let handle = host_handle(cpu).ok_or(BAD_HANDLE)?;
    let distance = ((cpu.ecx & 0xffff) << 16 | cpu.edx & 0xffff) as i32 as i64;
    let target = match cpu.eax & 0xff {
        0 => SeekFrom::Start(distance as u64),
        1 => SeekFrom::Current(distance),
        _ => SeekFrom::End(distance),
    };
    let open = cpu.host.files.get_mut(&handle).ok_or(BAD_HANDLE)?;
    let position = open.file.seek(target).map_err(|_| BAD_HANDLE)? as u32;
    cpu.edx = (cpu.edx & !0xffff) | position >> 16;
    Ok(position & 0xffff)
}

/// `int 21h` AH=4Eh, 4Fh: files matching DS:EDX, written to the transfer area.
fn find(cpu: &mut Cpu, first: bool) -> Result<u32, u32> {
    if first {
        let pattern = cpu.string(at(cpu, DS, cpu.edx)).replace('\\', "/");
        let (folder, mask) = pattern.rsplit_once('/').unwrap_or(("", &pattern));
        let directories = cpu.ecx & 0x10 != 0;
        let path = super::kernel32::host_path(cpu, if folder.is_empty() { "." } else { folder });
        let mut found: Vec<(String, bool, u64)> = std::fs::read_dir(path)
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|entry| {
                let name = entry.file_name().to_string_lossy().to_uppercase();
                let metadata = entry.metadata().ok()?;
                (matches(mask, &name) && (directories || !metadata.is_dir())).then_some((name, metadata.is_dir(), metadata.len()))
            })
            .collect();
        found.sort();
        found.reverse();
        cpu.host.dos.found = found;
    }
    let (name, directory, size) = cpu.host.dos.found.pop().ok_or(if first { FILE_NOT_FOUND } else { NO_MORE_FILES })?;
    let area = cpu.host.dos.transfer_area;
    cpu.fill(area, 43, 0);
    cpu.w8(area + 0x15, if directory { 0x10 } else { 0x20 });
    cpu.w32(area + 0x1a, size as u32);
    cpu.write_string(area + 0x1e, &name);
    Ok(0)
}

/// DOS's wildcards: `*` up to the dot or the end, `?` any one character.
fn matches(mask: &str, name: &str) -> bool {
    let split = |text: &str| -> (String, String) {
        let (base, extension) = text.split_once('.').unwrap_or((text, ""));
        (base.to_uppercase(), extension.to_uppercase())
    };
    let part = |mask: &str, text: &str| -> bool {
        let (mask, text): (Vec<char>, Vec<char>) = (mask.chars().collect(), text.chars().collect());
        let mut index = 0;
        for (position, &m) in mask.iter().enumerate() {
            match m {
                '*' => return true,
                '?' => index += 1,
                c if text.get(index) == Some(&c) => index += 1,
                _ => {
                    let _ = position;
                    return false;
                }
            }
        }
        index >= text.len()
    };
    let ((mask_base, mask_extension), (base, extension)) = (split(mask), split(name));
    part(&mask_base, &base) && part(if mask.contains('.') { &mask_extension } else { "*" }, &extension)
}

/// Where DOS memory blocks start, in the first megabyte, and where it ends:
/// below the VGA memory.
const DOS_MEMORY: u32 = 0x10000;
const DOS_MEMORY_END: u32 = 0xa0000;
const FIRST_SELECTOR: u16 = 0x80;
const SELECTOR_STEP: u16 = 8;

fn new_selector(cpu: &mut Cpu, base: u32) -> u16 {
    let selector = (0..).map(|n| FIRST_SELECTOR + n * SELECTOR_STEP).find(|s| !cpu.host.dos.selectors.contains_key(s)).unwrap();
    cpu.host.dos.selectors.insert(selector, base);
    selector
}

/// `int 31h`: DPMI, as DOS/4GW gives it.
fn dpmi_call(cpu: &mut Cpu, ax: u32) -> bool {
    let (bx, cx, dx) = (cpu.ebx & 0xffff, cpu.ecx & 0xffff, cpu.edx & 0xffff);
    let split = |value: u32| (value >> 16, value & 0xffff);
    match ax {
        // Descriptors: allocate CX of them, free one, the step between them,
        // set a base, a limit or rights, alias one.
        0x0000 => {
            let first = new_selector(cpu, 0);
            for _ in 1..cx.max(1) {
                new_selector(cpu, 0);
            }
            set_ax(cpu, first as u32);
        }
        0x0001 => {
            cpu.host.dos.selectors.remove(&(bx as u16));
        }
        0x0003 => set_ax(cpu, SELECTOR_STEP as u32),
        0x0007 => {
            cpu.host.dos.selectors.insert(bx as u16, cx << 16 | dx);
        }
        0x0008 | 0x0009 => {}
        0x000a => {
            let base = selector_base(cpu, bx as u16);
            let alias = new_selector(cpu, base);
            set_ax(cpu, alias as u32);
        }
        // DOS memory: BX paragraphs, at a real-mode segment in AX with a
        // selector in DX. It's never given back: games ask for little.
        0x0100 => {
            let size = bx << 4;
            let start = cpu.host.dos.dos_next.max(DOS_MEMORY);
            if start + size > DOS_MEMORY_END {
                set_ax(cpu, 8);
                cpu.ebx = (cpu.ebx & !0xffff) | (DOS_MEMORY_END - start) >> 4;
                cpu.cf = true;
                return true;
            }
            cpu.host.dos.dos_next = (start + size + 15) & !15;
            cpu.fill(start, size, 0);
            let selector = new_selector(cpu, start);
            set_ax(cpu, start >> 4);
            cpu.edx = (cpu.edx & !0xffff) | selector as u32;
        }
        0x0101 | 0x0102 => {}
        // Real-mode interrupt vectors, and the program's own handlers for
        // processor exceptions and interrupts.
        0x0200 => {
            cpu.ecx &= !0xffff;
            cpu.edx &= !0xffff;
        }
        0x0201 | 0x0202 | 0x0203 => {}
        0x0204 => {
            let (selector, offset) = vector(cpu, bx as u8);
            cpu.ecx = (cpu.ecx & !0xffff) | selector as u32;
            cpu.edx = offset;
        }
        0x0205 => {
            cpu.host.dos.vectors.insert(bx as u8, (cx as u16, cpu.edx));
        }
        // A real-mode interrupt with the registers at ES:EDI.
        0x0300 => real_mode_interrupt(cpu, bx as u8, at(cpu, ES, cpu.edi)),
        // DPMI 0.9 on a 486, with the interrupt controllers where DOS has them.
        0x0400 => {
            set_ax(cpu, 0x005a);
            cpu.ebx = (cpu.ebx & !0xffff) | 0x0005;
            cpu.ecx = (cpu.ecx & !0xff) | 4;
            cpu.edx = (cpu.edx & !0xffff) | 0x0870;
        }
        // Free memory: plenty.
        0x0500 => {
            let info = at(cpu, ES, cpu.edi);
            const PLENTY: u32 = 64 << 20;
            const PAGE: u32 = 4096;
            cpu.fill(info, 0x30, 0xff);
            cpu.w32(info, PLENTY);
            cpu.w32(info + 4, PLENTY / PAGE);
            cpu.w32(info + 8, PLENTY / PAGE);
            cpu.w32(info + 0x14, PLENTY / PAGE);
        }
        // Memory blocks, BX:CX bytes, at the linear address in BX:CX, with
        // the same as the handle in SI:DI.
        0x0501 => {
            let size = bx << 16 | cx;
            let block = cpu.allocate(size);
            cpu.fill(block, size, 0);
            let (high, low) = split(block);
            cpu.ebx = (cpu.ebx & !0xffff) | high;
            cpu.ecx = (cpu.ecx & !0xffff) | low;
            cpu.esi = (cpu.esi & !0xffff) | high;
            cpu.edi = (cpu.edi & !0xffff) | low;
        }
        0x0502 => {}
        0x0503 => {
            let (size, old) = (bx << 16 | cx, (cpu.esi & 0xffff) << 16 | cpu.edi & 0xffff);
            let block = cpu.allocate(size);
            let keep = cpu.r32(old.wrapping_sub(12)).min(size);
            let data = cpu.bytes(old, keep).to_vec();
            cpu.bytes_mut(block, keep).copy_from_slice(&data);
            let (high, low) = split(block);
            cpu.ebx = (cpu.ebx & !0xffff) | high;
            cpu.ecx = (cpu.ecx & !0xffff) | low;
            cpu.esi = (cpu.esi & !0xffff) | high;
            cpu.edi = (cpu.edi & !0xffff) | low;
        }
        // Physical memory: the VESA frame buffer, BX:CX, to a linear address.
        0x0800 => {
            let linear = super::vga::map_physical(cpu, bx << 16 | cx);
            let (high, low) = split(linear);
            cpu.ebx = (cpu.ebx & !0xffff) | high;
            cpu.ecx = (cpu.ecx & !0xffff) | low;
        }
        0x0801 => {}
        // Locking memory, and the virtual interrupt flag: nothing to do.
        0x0600 | 0x0601 | 0x0602 | 0x0603 => {}
        0x0900 | 0x0901 => set_al(cpu, 1),
        0x0902 => set_al(cpu, 1),
        _ => return false,
    }
    cpu.cf = false;
    true
}

/// A real-mode interrupt, with registers as DPMI's 0x32-byte structure at
/// `registers`: EDI, ESI, EBP, -, EBX, EDX, ECX, EAX, flags, ES, DS, ...
/// Buffers are at a real-mode segment and offset.
fn real_mode_interrupt(cpu: &mut Cpu, number: u8, registers: u32) {
    const EDI: u32 = 0;
    const EBX: u32 = 0x10;
    const EDX: u32 = 0x14;
    const ECX: u32 = 0x18;
    const EAX: u32 = 0x1c;
    const FLAGS: u32 = 0x20;
    const SEGMENT_ES: u32 = 0x22;
    const CARRY: u16 = 1;
    let segment = cpu.r16(registers + SEGMENT_ES) as u32;
    let edi = cpu.r32(registers + EDI);
    let mut r = super::vga::Registers {
        eax: cpu.r32(registers + EAX),
        ebx: cpu.r32(registers + EBX),
        ecx: cpu.r32(registers + ECX),
        edx: cpu.r32(registers + EDX),
        edi,
        buffer: segment * 16 + (edi & 0xffff),
        segment,
    };
    const ESI: u32 = 0x04;
    const SEGMENT_DS: u32 = 0x24;
    if cpu.trace {
        println!("  real-mode int {number:02x}h ax={:04x} bx={:04x} cx={:04x} dx={:04x} es:di={segment:04x}:{:04x}", r.eax & 0xffff, r.ebx & 0xffff, r.ecx & 0xffff, r.edx & 0xffff, edi & 0xffff);
    }
    let handled = match (number, r.eax & 0xffff) {
        (0x10, _) => super::vga::bios(cpu, &mut r),
        // DOS's table of lead bytes for double-byte characters: empty, two
        // zero bytes in the PSP.
        (0x21, 0x6300) => {
            const EMPTY: u32 = 0xfe;
            cpu.w16(PSP + EMPTY, 0);
            cpu.w16(registers + SEGMENT_DS, (PSP >> 4) as u16);
            cpu.w32(registers + ESI, EMPTY);
            r.eax &= !0xff;
            true
        }
        _ => false,
    };
    if !handled {
        panic!("the runtime doesn't have real-mode int {number:02x}h with ax={:04x} bx={:04x} cx={:04x} dx={:04x} yet", r.eax & 0xffff, r.ebx & 0xffff, r.ecx & 0xffff, r.edx & 0xffff);
    }
    cpu.w32(registers + EAX, r.eax);
    cpu.w32(registers + EBX, r.ebx);
    cpu.w32(registers + ECX, r.ecx);
    cpu.w32(registers + EDX, r.edx);
    cpu.w32(registers + EDI, r.edi);
    let flags = cpu.r16(registers + FLAGS) & !CARRY;
    cpu.w16(registers + FLAGS, flags);
}

/// `int 10h` from protected mode: buffers are at ES:EDI.
fn video_bios(cpu: &mut Cpu) -> bool {
    let mut r = super::vga::Registers {
        eax: cpu.eax,
        ebx: cpu.ebx,
        ecx: cpu.ecx,
        edx: cpu.edx,
        edi: cpu.edi,
        buffer: at(cpu, ES, cpu.edi),
        segment: 0,
    };
    // Function 10h/12h takes its colours at ES:DX.
    if cpu.eax & 0xffff == 0x1012 {
        r.buffer = at(cpu, ES, cpu.edx);
    }
    if !super::vga::bios(cpu, &mut r) {
        return false;
    }
    (cpu.eax, cpu.ebx, cpu.ecx, cpu.edx, cpu.edi) = (r.eax, r.ebx, r.ecx, r.edx, r.edi);
    true
}

/// `int 21h`: DOS.
fn dos_call(cpu: &mut Cpu, ah: u32) -> bool {
    use std::io::Write;
    match ah {
        // Starting another program. The game runs SciTech's UVCONFIG.EXE
        // when `univbe.drv` is missing, to probe the graphics card. The
        // runtime offers VBE 2.0 itself, so there's nothing to run: DOS's
        // answer for a missing program, error 2.
        0x4b => {
            let name = cpu.string(at(cpu, DS, cpu.edx));
            println!("  not running {name}");
            cpu.eax = (cpu.eax & !0xffff) | 2;
            cpu.cf = true;
        }
        // Whether a key waits at DOS's console: none. Keys reach the game as
        // scan codes through its keyboard handler (`host::pc`), not here.
        0x0b => set_al(cpu, 0),
        // The current drive.
        0x19 => set_al(cpu, u32::from(cpu.host.dos.drive)),
        // Selecting a drive: there are 26.
        0x0e => {
            let drive = cpu.edx as u8;
            if drive < 26 {
                cpu.host.dos.drive = drive;
            }
            set_al(cpu, 26);
        }
        // A drive's free space (DL: 0 the current drive, 1 A:). Z asks it of
        // its CD drive before a mission, as a disc check: a CD, as MSCDEX
        // reports it, has 2048-byte sectors, one to a cluster, and nothing
        // free; with space free it takes the disc for a copy and quits with
        // "COMMANDER ZOD SHOOTS PIRATES AT DAWN!". Other drives get the most
        // DOS 7 reports: 64 sectors of 512 bytes in 65535 clusters, all free.
        0x36 => {
            let asked = cpu.edx as u8;
            let drive = if asked == 0 { cpu.host.dos.drive } else { asked - 1 };
            let (per_cluster, free, sector) = if Some(drive) == cpu.host.dos.cd_drive { (1, 0, 2048) } else { (64, 0xffff, 512) };
            set_ax(cpu, per_cluster);
            cpu.ebx = (cpu.ebx & !0xffff) | free;
            cpu.ecx = (cpu.ecx & !0xffff) | sector;
            cpu.edx = (cpu.edx & !0xffff) | 0xffff;
        }
        // The disk transfer area, for finding files.
        0x1a => cpu.host.dos.transfer_area = at(cpu, DS, cpu.edx),
        0x2f => {
            let area = cpu.host.dos.transfer_area;
            cpu.ebx = area.wrapping_sub(cpu.bases[ES]);
        }
        // Interrupt vectors the program sets and reads: a selector and offset.
        0x25 => {
            let (number, selector, offset) = (cpu.eax as u8, cpu.segments[DS], cpu.edx);
            cpu.host.dos.vectors.insert(number, (selector, offset));
        }
        0x35 => {
            let (selector, offset) = vector(cpu, cpu.eax as u8);
            cpu.set_segment(ES, selector);
            cpu.ebx = offset;
        }
        // Date and time.
        0x2a | 0x2c => {
            let seconds = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
            let (days, rest) = (seconds / 86400, seconds % 86400);
            if ah == 0x2c {
                cpu.ecx = (cpu.ecx & !0xffff) | ((rest / 3600) as u32) << 8 | ((rest / 60) % 60) as u32;
                cpu.edx = (cpu.edx & !0xffff) | ((rest % 60) as u32) << 8;
            } else {
                // Days since 1970 to a date, by the civil calendar.
                let z = days as i64 + 719_468;
                let era = z.div_euclid(146_097);
                let day = z - era * 146_097;
                let year_of_era = (day - day / 1460 + day / 36524 - day / 146_096) / 365;
                let day_of_year = day - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
                let month_part = (5 * day_of_year + 2) / 153;
                let date = day_of_year - (153 * month_part + 2) / 5 + 1;
                let month = if month_part < 10 { month_part + 3 } else { month_part - 9 };
                let year = year_of_era + era * 400 + i64::from(month <= 2);
                cpu.ecx = (cpu.ecx & !0xffff) | year as u32;
                cpu.edx = (cpu.edx & !0xffff) | (month as u32) << 8 | date as u32;
                set_al(cpu, ((days + 4) % 7) as u32);
            }
        }
        0x3c | 0x3d => {
            let outcome = open(cpu, ah == 0x3c);
            result(cpu, outcome);
        }
        0x3e => {
            let outcome = match cpu.host.dos.handles.remove(&(cpu.ebx as u16)) {
                Some(host) => {
                    cpu.host.files.remove(&host);
                    Ok(cpu.eax)
                }
                None if (cpu.ebx as u16) < FIRST_HANDLE => Ok(cpu.eax),
                None => Err(BAD_HANDLE),
            };
            result(cpu, outcome);
        }
        0x3f => {
            let outcome = match host_handle(cpu) {
                Some(host) => {
                    let target = at(cpu, DS, cpu.edx);
                    let count = super::kernel32::read_file(cpu, host, target, cpu.ecx);
                    cpu.eax = count;
                    cpu.cf = false;
                    return true;
                }
                // Standard input: nothing typed.
                None if (cpu.ebx as u16) < FIRST_HANDLE => Ok(0),
                None => Err(BAD_HANDLE),
            };
            result(cpu, outcome);
        }
        0x40 => {
            let data = cpu.bytes(at(cpu, DS, cpu.edx), cpu.ecx).to_vec();
            let outcome = match host_handle(cpu) {
                Some(host) => match cpu.host.files.get_mut(&host) {
                    Some(open) => open.file.write_all(&data).map(|_| data.len() as u32).map_err(|_| BAD_HANDLE),
                    None => Err(BAD_HANDLE),
                },
                // Standard output and error go to the log.
                None if (cpu.ebx as u16) < FIRST_HANDLE => {
                    print!("{}", String::from_utf8_lossy(&data));
                    Ok(data.len() as u32)
                }
                None => Err(BAD_HANDLE),
            };
            if let Ok(count) = outcome {
                cpu.eax = count;
                cpu.cf = false;
            } else {
                result(cpu, outcome);
            }
        }
        0x41 => {
            let name = cpu.string(at(cpu, DS, cpu.edx));
            let path = super::kernel32::host_path(cpu, &name);
            let outcome = std::fs::remove_file(path).map(|_| 0).map_err(|_| FILE_NOT_FOUND);
            result(cpu, outcome);
        }
        0x42 => {
            let outcome = seek(cpu);
            result(cpu, outcome);
        }
        // A file's attributes: whether it's there.
        0x43 => {
            let name = cpu.string(at(cpu, DS, cpu.edx));
            let path = super::kernel32::host_path(cpu, &name);
            let outcome = match std::fs::metadata(path) {
                Ok(metadata) => {
                    cpu.ecx = (cpu.ecx & !0xffff) | if metadata.is_dir() { 0x10 } else { 0x20 };
                    Ok(cpu.eax)
                }
                Err(_) => Err(FILE_NOT_FOUND),
            };
            result(cpu, outcome);
        }
        // Device information: the standard handles are the console, the
        // others files on drive C:.
        0x44 if cpu.eax & 0xff == 0 => {
            let handle = cpu.ebx as u16;
            let info = if handle < FIRST_HANDLE { 0x80d3 } else if cpu.host.dos.handles.contains_key(&handle) { 0x0002 } else { 0 };
            cpu.edx = (cpu.edx & !0xffff) | info;
            result(cpu, if info == 0 { Err(BAD_HANDLE) } else { Ok(cpu.eax) });
        }
        // A drive's current folder, DL 0 for the current drive, 1 for A:.
        0x47 => {
            let target = at(cpu, DS, cpu.esi);
            let dl = cpu.edx as u8;
            let drive = if dl == 0 { cpu.host.dos.drive } else { dl - 1 };
            let folder = cpu.host.dos.folders.get(&drive).cloned().unwrap_or_default().to_uppercase();
            for (i, byte) in folder.bytes().take(63).enumerate() {
                cpu.w8(target + i as u32, byte);
            }
            cpu.w8(target + folder.len().min(63) as u32, 0);
            result(cpu, Ok(cpu.eax));
        }
        // Create or remove a folder. DOS answers 5 (access denied) when
        // the folder already exists or can't be made.
        0x39 | 0x3a => {
            let name = cpu.string(at(cpu, DS, cpu.edx));
            let path = super::kernel32::host_path(cpu, &name);
            let done = if ah == 0x39 { std::fs::create_dir(&path) } else { std::fs::remove_dir(&path) };
            let outcome = match done {
                Ok(()) => Ok(cpu.eax),
                Err(_) if !path.parent().map_or(true, |parent| parent.is_dir()) => Err(PATH_NOT_FOUND),
                Err(_) => Err(ACCESS_DENIED),
            };
            result(cpu, outcome);
        }
        0x3b => {
            let name = cpu.string(at(cpu, DS, cpu.edx));
            println!("  changing folder to {name} ({})", super::kernel32::host_path(cpu, &name).display());
            let outcome = if super::kernel32::host_path(cpu, &name).is_dir() {
                let (drive, folder) = cpu.host.dos.absolute(&name);
                cpu.host.dos.folders.insert(drive, folder);
                Ok(cpu.eax)
            } else {
                Err(PATH_NOT_FOUND)
            };
            result(cpu, outcome);
        }
        0x4e | 0x4f => {
            let outcome = find(cpu, ah == 0x4e);
            result(cpu, outcome);
        }
        // A text ended by `$`, to standard output: the log.
        0x09 => {
            let start = at(cpu, DS, cpu.edx);
            let text: String = (0..4096u32).map(|i| cpu.r8(start + i)).take_while(|&byte| byte != b'$').map(|byte| byte as char).collect();
            print!("{text}");
            cpu.eax = (cpu.eax & !0xff) | b'$' as u32;
        }
        0x4c => {
            println!("the game exited with code {}", cpu.eax & 0xff);
            std::process::exit((cpu.eax & 0xff) as i32);
        }
        _ => return false,
    }
    true
}

/// A selector's base, for `Cpu::set_segment`. Unknown selectors are taken as
/// flat, and logged.
pub fn selector_base(cpu: &mut Cpu, selector: u16) -> u32 {
    if selector == FLAT || selector == 0 {
        return 0;
    }
    match cpu.host.dos.selectors.get(&selector) {
        Some(&base) => base,
        None => {
            println!("unknown selector {selector:#x}, taken as flat");
            cpu.host.dos.selectors.insert(selector, 0);
            0
        }
    }
}

/// Sets up what DOS and DOS/4GW give a program before its first instruction:
/// the PSP with the command line, the environment, the selectors and the
/// registers. `program` is the path DOS would name in the environment.
pub fn start(cpu: &mut Cpu, image: &crate::pe::Image, program: &str, arguments: &str) {
    let stack = image.stack;
    cpu.host.dos.selectors.insert(PSP_SELECTOR, PSP);
    cpu.host.dos.selectors.insert(ENVIRONMENT_SELECTOR, ENVIRONMENT);
    // The program's 16-bit objects, whose selectors the loader wrote into its code.
    for &(selector, base) in &image.selectors {
        cpu.host.dos.selectors.insert(selector, base);
    }
    cpu.host.dos.dos_mode = true;
    cpu.host.dos.drive = 2;
    // The BIOS data for a VGA in colour text mode: the mode, 80 columns, and
    // the CRT controller's port, which games add 6 to for the status port.
    cpu.w8(0x449, 3);
    cpu.w16(0x44a, 80);
    cpu.w16(0x463, 0x3d4);
    cpu.w8(0x484, 24);
    // The CD drive is the one the command line names, as `Z.BAT`'s `/CD:E`.
    cpu.host.dos.cd_drive = arguments
        .split_whitespace()
        .find_map(|word| word.to_ascii_uppercase().strip_prefix("/CD:").and_then(|rest| rest.bytes().next()))
        .filter(u8::is_ascii_uppercase)
        .map(|letter| letter - b'A');

    // The PSP: `int 20h` at its start, the environment's selector at 0x2C,
    // and the command line's length and text at 0x80.
    cpu.fill(PSP, 0x100, 0);
    cpu.w16(PSP, 0x20cd);
    cpu.w16(PSP + 0x2c, ENVIRONMENT_SELECTOR);
    let tail: String = format!(" {arguments}").chars().take(126).collect();
    let tail = if arguments.is_empty() { String::new() } else { tail };
    cpu.w8(PSP + 0x80, tail.len() as u8);
    cpu.write_string(PSP + 0x81, &tail);
    cpu.w8(PSP + 0x81 + tail.len() as u32, 0x0d);

    // The environment: strings, an empty one, a count of 1, then the
    // program's own path.
    let mut at = ENVIRONMENT;
    for variable in ["COMSPEC=C:\\COMMAND.COM", "PATH=C:\\", "DOS4G=QUIET"] {
        cpu.write_string(at, variable);
        at += variable.len() as u32 + 1;
    }
    cpu.w8(at, 0);
    cpu.w16(at + 1, 1);
    cpu.write_string(at + 3, program);

    for register in [CS, SS, DS, FS] {
        cpu.set_segment(register, FLAT);
    }
    cpu.set_segment(ES, PSP_SELECTOR);
    cpu.set_segment(GS, 0);
    cpu.esp = stack;
    // DOS's default transfer area is in the PSP.
    cpu.host.dos.transfer_area = PSP + 0x80;
    cpu.imports.push(crate::cpu::Host { name: "dos!previous_handler".to_string(), function: Some(previous_handler) });
    cpu.host.dos.previous = crate::cpu::IMPORT_BASE + cpu.imports.len() as u32 - 1;
    // The PC's hardware is serviced at the program's safe points.
    cpu.host.timers.dos = true;
    cpu.host.timers.publish();
    // The screen: a DOS program has it from the start.
    if cpu.host.video.is_none() {
        cpu.host.video = super::video::Video::open(cpu.host.title, cpu.host.icon);
    }
}

fn set_al(cpu: &mut Cpu, value: u32) {
    cpu.eax = (cpu.eax & !0xff) | (value & 0xff);
}

fn set_ax(cpu: &mut Cpu, value: u32) {
    cpu.eax = (cpu.eax & !0xffff) | (value & 0xffff);
}

/// `int N` from the translated code.
pub fn interrupt(cpu: &mut Cpu, number: u8) {
    let ah = (cpu.eax >> 8) & 0xff;
    let ax = cpu.eax & 0xffff;
    if cpu.trace {
        println!("int {number:02x}h ax={ax:04x} bx={:04x} cx={:04x} edx={:08x}", cpu.ebx & 0xffff, cpu.ecx & 0xffff, cpu.edx);
    }
    match (number, ah, ax) {
        // DOS's version: 7.10, and no other extender's mark in the high half.
        (0x21, 0x30, _) => {
            cpu.eax = 0x0a07;
            cpu.ebx &= !0xffff;
        }
        // Resizing the program's memory block: there's room.
        (0x21, 0x4a, _) => cpu.cf = false,
        // The PSP: DOS/4GW gives its selector in protected mode, not a segment.
        (0x21, 0x51 | 0x62, _) => cpu.ebx = (cpu.ebx & !0xffff) | PSP_SELECTOR as u32,
        // DOS/4GW's own check: it's there.
        (0x21, _, 0xff00) => set_al(cpu, 0xff),
        // Is Windows running in enhanced mode? No: AL 0, plain DOS.
        (0x2f, _, 0x1600) => set_al(cpu, 0),
        // DPMI: a selector's base, in CX:DX.
        (0x31, _, 0x0006) => {
            let base = selector_base(cpu, cpu.ebx as u16);
            cpu.ecx = (cpu.ecx & !0xffff) | base >> 16;
            cpu.edx = (cpu.edx & !0xffff) | base & 0xffff;
            cpu.cf = false;
        }
        (0x21, _, _) if dos_call(cpu, ah) => {}
        (0x31, _, _) if dpmi_call(cpu, ax) => {}
        (0x10, _, _) if video_bios(cpu) => {}
        (0x33, _, _) if super::mouse::call(cpu) => {}
        // The BIOS keyboard: the shift keys, from the BIOS data at 0x417.
        // No keys wait in its buffer: they reach the program through its own
        // keyboard interrupt (`pc::key`).
        (0x16, 0x02 | 0x12, _) => {
            let shifts = cpu.r8(0x417) as u32;
            cpu.eax = (cpu.eax & !0xff) | shifts;
        }
        (0x16, 0x01 | 0x11, _) => cpu.zf = true,
        _ => panic!("the runtime doesn't have int {number:02x}h with ah={ah:02x} (ax={ax:04x}) yet"),
    }
}
