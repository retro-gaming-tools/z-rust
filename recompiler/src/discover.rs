//! Finds the code in an image by following it from what is certainly code.
//!
//! Starts at the entry point and the exports, and adds anything that looks like
//! a pointer to code: base relocations, values stored in data sections, and
//! immediates in the code already found. Follows branches and calls, and reads
//! the jump tables behind indirect jumps.

use crate::pe::Image;
use iced_x86::{Code, Decoder, DecoderOptions, FlowControl, Instruction, Mnemonic, OpKind, Register};
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Default)]
pub struct Report {
    pub instructions: BTreeMap<u32, Instruction>,
    /// Real function entries: called, exported, or pointed at and not inside other code.
    pub functions: BTreeSet<u32>,
    /// Addresses inside functions that a pointer might reach.
    pub entries: BTreeSet<u32>,
    pub jump_tables: BTreeMap<u32, Vec<u32>>,
    pub unresolved_jumps: Vec<u32>,
    pub invalid: Vec<u32>,
    pub mnemonics: BTreeMap<Mnemonic, usize>,
    /// Functions found in code that nothing found leads to (DOS programs).
    pub unreached: usize,
}

fn decode_at(image: &Image, address: u32) -> Option<Instruction> {
    let offset = address.checked_sub(image.base)? as usize;
    let bytes = image.memory.get(offset..(offset + 16).min(image.memory.len()))?;
    let mut decoder = Decoder::with_ip(32, bytes, address as u64, DecoderOptions::NONE);
    Some(decoder.decode())
}

/// True if code at this address decodes cleanly for a few instructions.
fn plausible_code(image: &Image, mut address: u32) -> bool {
    for _ in 0..4 {
        let Some(instruction) = decode_at(image, address) else { return false };
        if instruction.code() == Code::INVALID || !image.is_code(address) {
            return false;
        }
        match instruction.flow_control() {
            FlowControl::Return | FlowControl::UnconditionalBranch | FlowControl::IndirectBranch => return true,
            _ => address += instruction.len() as u32,
        }
    }
    true
}

fn pointers_in_data(image: &Image) -> Vec<u32> {
    let mut found = Vec::new();
    for section in image.sections.iter().filter(|s| !s.executable) {
        // At every byte, not every fourth: hand-written assembly packs its
        // records, so a code pointer can sit at any offset.
        for address in section.start..section.end.saturating_sub(3) {
            if let Some(value) = image.u32_at(address) {
                if image.is_code(value) {
                    found.push(value);
                }
            }
        }
    }
    found
}

/// Reads a table behind `jmp [index*4+table]`, which the compiler lays out as
/// consecutive pointers. It ends at the first value that isn't code.
///
/// An index that never is 0 lets the compiler put the table's base before its
/// first pointer, inside the code in front of it, as MSVC's `memcpy` does with
/// `jmp [eax*4+table]` for eax from 1 to 3. So a few leading values that aren't
/// code are skipped.
fn read_jump_table(image: &Image, table: u32) -> Vec<u32> {
    let mut targets = Vec::new();
    let mut at = table;
    for _ in 0..3 {
        if image.u32_at(at).is_some_and(|value| !image.is_code(value)) {
            at += 4;
        }
    }
    while let Some(value) = image.u32_at(at) {
        if !image.is_code(value) || targets.len() >= 1024 {
            break;
        }
        targets.push(value);
        at += 4;
    }
    targets
}

struct Walk<'a> {
    image: &'a Image,
    slots: BTreeSet<u32>,
    report: Report,
    work: Vec<u32>,
    candidates: Vec<u32>,
    called: BTreeSet<u32>,
    pointed: BTreeSet<u32>,
}

pub fn discover(image: &Image) -> Report {
    let exports = image.exports.iter().map(|e| e.address).filter(|&a| image.is_code(a));
    let mut work: Vec<u32> = exports.collect();
    if image.entry != 0 {
        work.push(image.entry);
    }
    let mut candidates: Vec<u32> = image.relocated.iter().filter_map(|&at| image.u32_at(at)).filter(|&v| image.is_code(v)).collect();
    candidates.extend(pointers_in_data(image));
    // Watcom's int386 runs a computed interrupt by jumping into a table of
    // `int N; ret` stubs, so each stub is a function nothing calls directly.
    if image.is_dos() {
        work.extend(interrupt_stubs(image));
    }
    let called = work.iter().copied().collect();
    let slots = image.imports.iter().map(|i| i.slot).collect();
    let mut walk = Walk { image, slots, report: Report::default(), work, candidates, called, pointed: BTreeSet::new() };

    let mut seen: HashSet<u32> = HashSet::new();
    loop {
        loop {
            while let Some(address) = walk.work.pop() {
                walk.follow(address);
            }
            let Some(candidate) = walk.candidates.pop() else { break };
            if !seen.insert(candidate) {
                continue;
            }
            if walk.report.instructions.contains_key(&candidate) {
                walk.pointed.insert(candidate);
            } else if plausible_code(image, candidate) {
                walk.pointed.insert(candidate);
                walk.work.push(candidate);
            }
        }
        // Hand-written assembly also keeps tables of code pointers between its
        // routines, inside the code section. Whatever no instruction covers
        // may be such a table, and what it points at may hold more.
        let found: Vec<u32> = pointers_between_code(image, &walk.report).into_iter().filter(|value| !seen.contains(value)).collect();
        if !found.is_empty() {
            walk.candidates.extend(found);
            continue;
        }
        // Last, and once only, a DOS program's code that nothing found leads
        // to: Watcom links whole libraries, so most of what's left is
        // functions nothing calls directly. Translated too, a call through a
        // pointer discovery missed still finds code. After that, what's left
        // is data.
        let gaps = if image.is_dos() && walk.report.unreached == 0 { unreached_functions(image, &walk.report) } else { Vec::new() };
        if gaps.is_empty() {
            break;
        }
        walk.report.unreached += gaps.len();
        walk.called.extend(&gaps);
        walk.work.extend(gaps);
    }
    walk.finish()
}

/// The `int N; ret` stubs in runs of at least 16 in a row: three bytes apart,
/// or four with a `nop` after each, as Watcom's int386 and the DOS/4GW
/// interface code lay them out.
fn interrupt_stubs(image: &Image) -> Vec<u32> {
    let mut found = Vec::new();
    for section in image.sections.iter().filter(|s| s.executable) {
        let bytes = &image.memory[(section.start - image.base) as usize..(section.end - image.base) as usize];
        let stub = |at: usize, stride: usize| at + stride <= bytes.len() && bytes[at] == 0xcd && bytes[at + 2] == 0xc3 && (stride == 3 || bytes[at + 3] == 0x90);
        let mut at = 0;
        while at < bytes.len() {
            let mut taken = false;
            for stride in [4, 3] {
                let mut end = at;
                while stub(end, stride) {
                    end += stride;
                }
                if (end - at) / stride >= 16 {
                    found.extend((at..end).step_by(stride).map(|offset| section.start + offset as u32));
                    at = end;
                    taken = true;
                    break;
                }
            }
            if !taken {
                at += 1;
            }
        }
    }
    found
}

/// Watcom's padding between functions: `lea` and `mov` that change nothing,
/// `nop`, `int3` and zeros. Returns its length at the start of `bytes`.
fn padding_len(bytes: &[u8]) -> usize {
    const SIX: [[u8; 6]; 4] = [[0x8d, 0x80, 0, 0, 0, 0], [0x8d, 0x92, 0, 0, 0, 0], [0x8d, 0x89, 0, 0, 0, 0], [0x8d, 0x9b, 0, 0, 0, 0]];
    const FOUR: [[u8; 4]; 2] = [[0x8d, 0x54, 0x22, 0], [0x8d, 0x44, 0x20, 0]];
    const THREE: [[u8; 3]; 6] = [[0x8d, 0x40, 0], [0x8d, 0x52, 0], [0x8d, 0x49, 0], [0x8d, 0x5b, 0], [0x8d, 0x76, 0], [0x8d, 0x7f, 0]];
    const TWO: [[u8; 2]; 7] = [[0x8b, 0xc0], [0x8b, 0xd2], [0x8b, 0xdb], [0x8b, 0xc9], [0x8b, 0xff], [0x8b, 0xf6], [0x89, 0xc0]];
    if SIX.iter().any(|p| bytes.starts_with(p)) {
        6
    } else if FOUR.iter().any(|p| bytes.starts_with(p)) {
        4
    } else if THREE.iter().any(|p| bytes.starts_with(p)) {
        3
    } else if TWO.iter().any(|p| bytes.starts_with(p)) {
        2
    } else if matches!(bytes.first(), Some(0x90 | 0xcc | 0x00)) {
        1
    } else {
        0
    }
}

/// True if `start..end` decodes cleanly as compiled code, without
/// instructions a compiler doesn't emit, and ends in a return or a jump
/// followed by nothing but padding.
fn looks_like_function(image: &Image, start: u32, end: u32) -> bool {
    use Mnemonic as M;
    let bytes = &image.memory[(start - image.base) as usize..(end - image.base) as usize];
    let mut decoder = Decoder::with_ip(32, bytes, start as u64, DecoderOptions::NONE);
    while decoder.can_decode() {
        let i = decoder.decode();
        if i.code() == Code::INVALID
            || matches!(i.mnemonic(), M::Arpl | M::Bound | M::Into | M::Aaa | M::Aas | M::Daa | M::Das | M::Salc | M::Hlt | M::Insb | M::Insd | M::Outsb | M::Outsd | M::Les | M::Lds)
        {
            return false;
        }
        if matches!(i.flow_control(), FlowControl::Return | FlowControl::UnconditionalBranch | FlowControl::IndirectBranch) {
            // Data would end it off an instruction boundary, or before a
            // return or jump.
            let mut at = (i.next_ip() as u32 - start) as usize;
            while at < bytes.len() {
                let n = padding_len(&bytes[at..]);
                if n == 0 {
                    break;
                }
                at += n;
            }
            if at >= bytes.len() {
                return true;
            }
        }
    }
    false
}

/// The starts of stretches of code that no instruction found covers and that
/// look like whole functions, once the padding before them is skipped. A
/// stretch that starts with a code pointer is a table, not code.
fn unreached_functions(image: &Image, report: &Report) -> Vec<u32> {
    let mut found = Vec::new();
    for section in image.sections.iter().filter(|s| s.executable) {
        let mut covered = vec![false; (section.end - section.start) as usize];
        for (&address, instruction) in report.instructions.range(section.start..section.end) {
            let start = (address - section.start) as usize;
            let end = (start + instruction.len()).min(covered.len());
            covered[start..end].fill(true);
        }
        let base = (section.start - image.base) as usize;
        let mut at = 0;
        while at < covered.len() {
            if covered[at] {
                at += 1;
                continue;
            }
            let mut end = at;
            while end < covered.len() && !covered[end] {
                end += 1;
            }
            let mut start = at;
            while start < end {
                let n = padding_len(&image.memory[base + start..base + end]);
                if n == 0 || start + n > end {
                    break;
                }
                start += n;
            }
            let address = section.start + start as u32;
            if end - start >= 8 && !image.u32_at(address).is_some_and(|v| image.is_code(v)) && looks_like_function(image, address, section.start + end as u32) {
                found.push(address);
            }
            at = end;
        }
    }
    found
}

/// Values that point into code, read from the bytes of the code sections that
/// no instruction found so far covers.
fn pointers_between_code(image: &Image, report: &Report) -> Vec<u32> {
    let mut found = Vec::new();
    for section in image.sections.iter().filter(|s| s.executable) {
        let mut covered = vec![false; (section.end - section.start) as usize];
        for (&address, instruction) in report.instructions.range(section.start..section.end) {
            let start = (address - section.start) as usize;
            let end = (start + instruction.len()).min(covered.len());
            covered[start..end].fill(true);
        }
        for address in section.start..section.end.saturating_sub(3) {
            let at = (address - section.start) as usize;
            if covered[at..at + 4].iter().any(|&byte| byte) {
                continue;
            }
            if let Some(value) = image.u32_at(address) {
                if image.is_code(value) {
                    found.push(value);
                }
            }
        }
    }
    found
}

impl Walk<'_> {
    /// Case labels and addresses inside functions also look like pointers, so a
    /// pointed-at address only counts as an entry if nothing jumps or falls into it.
    fn finish(mut self) -> Report {
        let mut inside: HashSet<u32> = HashSet::new();
        for (&address, instruction) in &self.report.instructions {
            let next = address + instruction.len() as u32;
            match instruction.flow_control() {
                FlowControl::UnconditionalBranch => {
                    inside.insert(branch_target(&instruction));
                }
                FlowControl::ConditionalBranch => {
                    inside.insert(branch_target(&instruction));
                    inside.insert(next);
                }
                FlowControl::Next | FlowControl::Call | FlowControl::IndirectCall | FlowControl::Interrupt => {
                    inside.insert(next);
                }
                _ => {}
            }
        }
        let cases: HashSet<u32> = self.report.jump_tables.values().flatten().copied().collect();
        inside.extend(&cases);
        let pointed = self.pointed.iter().copied().filter(|address| !inside.contains(address));
        let functions: BTreeSet<u32> = self.called.iter().copied().chain(pointed).collect();
        // Anything else pointed at may still be reached through a pointer, so it
        // gets an entry for indirect calls, without splitting the code around it.
        // A jump table in a data section can run on into tables of function
        // pointers behind it, as hand-written assembly keeps them, so only the
        // cases of tables inside the code are certain to be mere labels.
        let labels: HashSet<u32> = self
            .report
            .jump_tables
            .iter()
            .filter(|(jump, _)| self.report.instructions.get(jump).is_some_and(|i| self.image.is_code(i.memory_displacement32())))
            .flat_map(|(_, targets)| targets.iter().copied())
            .collect();
        self.report.entries = self.pointed.iter().copied().filter(|a| !functions.contains(a) && !labels.contains(a)).collect();
        self.report.functions = functions;
        self.report
    }

    fn is_import(&self, instruction: &Instruction) -> bool {
        instruction.memory_base() == Register::None
            && instruction.memory_index() == Register::None
            && self.slots.contains(&instruction.memory_displacement32())
    }

    fn follow(&mut self, start: u32) {
        let image = self.image;
        let mut address = start;
        loop {
            if self.report.instructions.contains_key(&address) || !image.is_code(address) {
                return;
            }
            let Some(instruction) = decode_at(image, address) else { return };
            if instruction.code() == Code::INVALID {
                self.report.invalid.push(address);
                return;
            }
            self.report.instructions.insert(address, instruction);
            *self.report.mnemonics.entry(instruction.mnemonic()).or_default() += 1;
            for operand in 0..instruction.op_count() {
                if instruction.op_kind(operand) == OpKind::Immediate32 && image.is_code(instruction.immediate32()) {
                    self.candidates.push(instruction.immediate32());
                }
            }
            let next = address + instruction.len() as u32;
            match instruction.flow_control() {
                FlowControl::Next | FlowControl::Interrupt | FlowControl::IndirectCall | FlowControl::Exception | FlowControl::XbeginXabortXend => address = next,
                FlowControl::ConditionalBranch => {
                    self.work.push(branch_target(&instruction));
                    address = next;
                }
                FlowControl::UnconditionalBranch => address = branch_target(&instruction),
                FlowControl::Call => {
                    let target = branch_target(&instruction);
                    if image.is_code(target) {
                        self.called.insert(target);
                        self.work.push(target);
                    }
                    address = next;
                }
                FlowControl::IndirectBranch => {
                    if !self.is_import(&instruction) {
                        self.jump_table(address, &instruction);
                    }
                    return;
                }
                _ => return,
            }
        }
    }

    fn jump_table(&mut self, address: u32, instruction: &Instruction) {
        let targets = if instruction.op0_kind() == OpKind::Memory && instruction.memory_index_scale() == 4 {
            read_jump_table(self.image, instruction.memory_displacement32())
        } else {
            Vec::new()
        };
        if targets.is_empty() {
            self.report.unresolved_jumps.push(address);
        } else {
            self.work.extend(&targets);
            self.report.jump_tables.insert(address, targets);
        }
    }
}

/// Where a direct jump or call goes. A far one (`call ptr16:32`, which DOS
/// programs use) has its offset in the instruction, already relocated like
/// any other address. Its selector is the program's code, and is ignored.
pub fn branch_target(instruction: &Instruction) -> u32 {
    match instruction.op0_kind() {
        OpKind::FarBranch32 => instruction.far_branch32(),
        OpKind::FarBranch16 => instruction.far_branch16() as u32,
        _ => instruction.near_branch_target() as u32,
    }
}
