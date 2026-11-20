//! Reads a DOS/4GW program: an LE file, as Watcom C made them.
//!
//! An LE file is an MZ stub followed by objects (code and data), each linked
//! for its own base address, and fixups that say where pointers between them
//! are. DOS/4GW loaded the objects wherever it had room and applied the
//! fixups, and so does this: at `LOAD_BASE`, above the first megabyte, which
//! holds DOS's memory, the BIOS data and the VGA memory at 0xA0000, as on a
//! real PC. Watcom linked programs at 0x10000, right on top of all that. The
//! 32-bit pointers the fixups name are kept, as base relocations are for a PE
//! file, because they're where code pointers are. There are no imports: a DOS
//! program asks DOS and the extender for everything through interrupts.

use crate::pe::{Image, Section};

/// Where a DOS program's first object goes. It's clear of the first megabyte,
/// and of where Windows programs load (0x400000 up), so one port can carry a
/// game's DOS and Windows programs without their code addresses meeting.
pub const LOAD_BASE: u32 = 0x0100_0000;
const OBJECT_EXECUTABLE: u32 = 4;
/// Set for a 32-bit object. A 16-bit one, such as real-mode code a program
/// copies down into DOS memory, is reached through a selector of its own.
const OBJECT_32BIT: u32 = 0x2000;
/// The selector a 16-bit object gets: this plus 8 for each object number.
/// They're fixed, because the recompiler reads them out of the code too, and
/// clear of the runtime's own (`dos::FIRST_SELECTOR` and up).
pub const OBJECT_SELECTORS: u16 = 0x1000;
const SOURCE_SELECTOR16: u8 = 2;
const SOURCE_POINTER16_16: u8 = 3;
const SOURCE_OFFSET16: u8 = 5;
const SOURCE_POINTER16_32: u8 = 6;
const SOURCE_OFFSET32: u8 = 7;
const SOURCE_RELATIVE32: u8 = 8;
const SOURCE_LIST: u8 = 0x20;
const TARGET_KIND: u8 = 3;
const ADDITIVE: u8 = 4;
const TARGET_OFFSET32: u8 = 0x10;
const ADDITIVE32: u8 = 0x20;
const OBJECT16: u8 = 0x40;

fn u16_at(data: &[u8], at: usize) -> Option<u32> {
    Some(u16::from_le_bytes(data.get(at..at + 2)?.try_into().ok()?) as u32)
}

fn i16_at(data: &[u8], at: usize) -> Option<i32> {
    Some(i16::from_le_bytes(data.get(at..at + 2)?.try_into().ok()?) as i32)
}

fn u32_at(data: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(data.get(at..at + 4)?.try_into().ok()?))
}

struct Object {
    size: u32,
    base: u32,
    flags: u32,
    first_page: u32,
    pages: u32,
}

pub fn parse(file: &[u8]) -> Option<Image> {
    if file.get(..2)? != b"MZ" {
        return None;
    }
    let le = u32_at(file, 0x3c)? as usize;
    if file.get(le..le + 2)? != b"LE" {
        return None;
    }
    let page_size = u32_at(file, le + 0x28)?;
    let last_page = u32_at(file, le + 0x2c)?;
    let total_pages = u32_at(file, le + 0x14)?;
    let object_table = le + u32_at(file, le + 0x40)? as usize;
    let page_map = le + u32_at(file, le + 0x48)? as usize;
    let fixup_pages = le + u32_at(file, le + 0x68)? as usize;
    let fixup_records = le + u32_at(file, le + 0x6c)? as usize;
    let pages_at = u32_at(file, le + 0x80)? as usize;
    let mut objects: Vec<Object> = (0..u32_at(file, le + 0x44)? as usize)
        .map(|index| {
            let entry = object_table + index * 24;
            Some(Object {
                size: u32_at(file, entry)?,
                base: u32_at(file, entry + 4)?,
                flags: u32_at(file, entry + 8)?,
                first_page: u32_at(file, entry + 12)?,
                pages: u32_at(file, entry + 16)?,
            })
        })
        .collect::<Option<_>>()?;
    // All objects move by the same amount, keeping their places to each other.
    let delta = LOAD_BASE.wrapping_sub(objects.iter().map(|o| o.base).min()?);
    objects.iter_mut().for_each(|object| object.base = object.base.wrapping_add(delta));
    let base = LOAD_BASE;
    let end = objects.iter().map(|o| o.base + o.size.max(o.pages * page_size)).max()?;
    let mut memory = vec![0u8; (end - base) as usize];

    // Pages, through the page map: its entries are 1-based page numbers in
    // the file, in the high three bytes.
    for object in &objects {
        for index in 0..object.pages {
            let page = object.first_page + index;
            let entry = page_map + (page as usize - 1) * 4;
            let number = (*file.get(entry)? as usize) << 16 | (*file.get(entry + 1)? as usize) << 8 | *file.get(entry + 2)? as usize;
            if number == 0 {
                continue;
            }
            let start = pages_at + (number - 1) * page_size as usize;
            let size = if page == total_pages { last_page } else { page_size } as usize;
            let chunk = file.get(start..(start + size).min(file.len()))?;
            let at = (object.base + index * page_size - base) as usize;
            memory[at..at + chunk.len()].copy_from_slice(chunk);
        }
    }

    // Fixups, for the objects' new places.
    let mut relocated = Vec::new();
    for object in &objects {
        for index in 0..object.pages {
            let page = (object.first_page + index - 1) as usize;
            let mut at = fixup_records + u32_at(file, fixup_pages + page * 4)? as usize;
            let stop = fixup_records + u32_at(file, fixup_pages + (page + 1) * 4)? as usize;
            let page_address = object.base + index * page_size;
            while at < stop {
                let source = *file.get(at)?;
                let flags = *file.get(at + 1)?;
                at += 2;
                let kind = source & 0xf;
                let mut offsets = Vec::new();
                let list = if source & SOURCE_LIST != 0 {
                    at += 1;
                    Some(*file.get(at - 1)?)
                } else {
                    offsets.push(i16_at(file, at)?);
                    at += 2;
                    None
                };
                if flags & TARGET_KIND != 0 {
                    // Imports and entry points: Watcom's DOS/4GW programs have none.
                    return None;
                }
                let target_object = if flags & OBJECT16 != 0 {
                    at += 2;
                    u16_at(file, at - 2)?
                } else {
                    at += 1;
                    *file.get(at - 1)? as u32
                };
                let mut target = 0;
                if kind != SOURCE_SELECTOR16 {
                    target = if flags & TARGET_OFFSET32 != 0 {
                        at += 4;
                        u32_at(file, at - 4)?
                    } else {
                        at += 2;
                        u16_at(file, at - 2)?
                    };
                }
                if flags & ADDITIVE != 0 {
                    target += if flags & ADDITIVE32 != 0 {
                        at += 4;
                        u32_at(file, at - 4)?
                    } else {
                        at += 2;
                        u16_at(file, at - 2)?
                    };
                }
                if let Some(count) = list {
                    for _ in 0..count {
                        offsets.push(i16_at(file, at)?);
                        at += 2;
                    }
                }
                let object = objects.get(target_object as usize - 1)?;
                let target = object.base.wrapping_add(target);
                // A 32-bit object's selector is the flat one, which 0 stands for.
                let selector = if object.flags & OBJECT_32BIT != 0 { 0 } else { OBJECT_SELECTORS + target_object as u16 * 8 };
                for offset in offsets {
                    let place = page_address.wrapping_add(offset as u32);
                    let Some(slot) = place.checked_sub(base).map(|o| o as usize).filter(|o| o + 4 <= memory.len()) else { continue };
                    match kind {
                        SOURCE_OFFSET32 | SOURCE_POINTER16_32 => {
                            memory[slot..slot + 4].copy_from_slice(&target.to_le_bytes());
                            relocated.push(place);
                        }
                        SOURCE_RELATIVE32 => {
                            let relative = target.wrapping_sub(place).wrapping_sub(4);
                            memory[slot..slot + 4].copy_from_slice(&relative.to_le_bytes());
                        }
                        SOURCE_OFFSET16 => memory[slot..slot + 2].copy_from_slice(&(target as u16).to_le_bytes()),
                        SOURCE_SELECTOR16 => memory[slot..slot + 2].copy_from_slice(&selector.to_le_bytes()),
                        // A 16-bit offset, then the selector.
                        SOURCE_POINTER16_16 if object.flags & OBJECT_32BIT == 0 => {
                            memory[slot..slot + 2].copy_from_slice(&(target.wrapping_sub(object.base) as u16).to_le_bytes());
                            memory[slot + 2..slot + 4].copy_from_slice(&selector.to_le_bytes());
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    relocated.sort_unstable();
    relocated.dedup();

    let object_base = |number: u32| objects.get(number as usize - 1).map(|o| o.base);
    let entry = object_base(u32_at(file, le + 0x18)?)? + u32_at(file, le + 0x1c)?;
    let stack = object_base(u32_at(file, le + 0x20)?)? + u32_at(file, le + 0x24)?;
    let sections = objects
        .iter()
        .map(|o| Section { start: o.base, end: o.base + o.size, executable: o.flags & OBJECT_EXECUTABLE != 0 })
        .collect();
    let selectors = objects
        .iter()
        .enumerate()
        .filter(|(_, object)| object.flags & OBJECT_32BIT == 0)
        .map(|(index, object)| (OBJECT_SELECTORS + (index as u16 + 1) * 8, object.base))
        .collect();
    Some(Image { base, entry, is_dll: false, memory, sections, exports: Vec::new(), imports: Vec::new(), relocated, stack, selectors })
}
