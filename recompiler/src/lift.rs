//! Translates discovered code to Rust.
//!
//! Each guest function becomes one Rust function: a loop over its basic blocks,
//! because Rust has no goto. Functions that share code are translated together
//! as one body that starts at any of their addresses (see `regions`), so each
//! block is translated once. A call pushes the return address on the guest's
//! stack and calls the Rust function, and `ret` pops it, notes where it went
//! back to, and returns. A caller that isn't the one returned to returns too,
//! which unwinds the Rust frames a reset of the game's stack skipped. An
//! instruction with no translation becomes a named fault, so gaps show up when
//! that code runs, and are counted here.

use crate::discover::{branch_target, Report};
use iced_x86::{FlowControl, Instruction, Mnemonic, OpKind, Register, RflagsBits};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Write;

/// What a flat 32-bit program sees in its segment registers.
const FLAT_SEGMENT: u32 = 0x2b;
const ALL_FLAGS: u32 = RflagsBits::OF | RflagsBits::SF | RflagsBits::ZF | RflagsBits::AF | RflagsBits::CF | RflagsBits::PF | RflagsBits::DF;

#[derive(Default)]
pub struct Stats {
    pub missing: BTreeMap<String, usize>,
}

struct Block {
    addresses: Vec<u32>,
    successors: Vec<u32>,
    live_out: u32,
    live_in: u32,
}

/// `replaced` names the function `o_` instead of `f_`, because a hand-written
/// mod takes its place.
pub fn lift_function(start: u32, report: &Report, all_functions: &HashSet<u32>, replaced: bool, stats: &mut Stats) -> String {
    let text = lift_region(&[start], report, all_functions, stats);
    if replaced {
        text.replacen(&format!("pub fn f_{start:08x}("), &format!("pub fn o_{start:08x}("), 1)
    } else {
        text
    }
}

/// Groups the functions to translate into regions, which share code. A body
/// is everything its start reaches without passing another function's start,
/// so the entries that pointers lead into the middle of other code (common in
/// hand-written assembly) each had a copy of what they share. Z's DOS program
/// had each block about four times, some over 200 times. Functions whose
/// bodies overlap now make one region, translated once. Functions a mod
/// replaces stay on their own, so that a jump to one still goes to the mod.
pub fn regions(functions: &[u32], report: &Report, all_functions: &HashSet<u32>, replaced: &BTreeSet<u32>) -> Vec<Vec<u32>> {
    let mut parent: Vec<usize> = (0..functions.len()).collect();
    fn find(parent: &mut [usize], mut i: usize) -> usize {
        while parent[i] != i {
            parent[i] = parent[parent[i]];
            i = parent[i];
        }
        i
    }
    let mut owner: HashMap<u32, usize> = HashMap::new();
    for (index, &start) in functions.iter().enumerate() {
        if replaced.contains(&start) {
            continue;
        }
        let body = Function::collect(&[start], report, all_functions);
        for address in body.blocks.values().flat_map(|b| &b.addresses) {
            match owner.get(address) {
                Some(&other) => {
                    let (a, b) = (find(&mut parent, index), find(&mut parent, other));
                    if a != b {
                        parent[a.max(b)] = a.min(b);
                    }
                }
                None => {
                    owner.insert(*address, index);
                }
            }
        }
    }
    let mut groups: BTreeMap<usize, Vec<u32>> = BTreeMap::new();
    for index in 0..functions.len() {
        let root = find(&mut parent, index);
        groups.entry(root).or_default().push(functions[index]);
    }
    groups.into_values().collect()
}

/// Translates a region: one Rust function for one function, or for several,
/// one shared body `r_…` that starts where it's told, and an `f_…` per start.
pub fn lift_region(starts: &[u32], report: &Report, all_functions: &HashSet<u32>, stats: &mut Stats) -> String {
    let mut body = Function::collect(starts, report, all_functions);
    let loads = segments() && body.blocks.values().flat_map(|b| &b.addresses).filter_map(|a| report.instructions.get(a)).any(writes_ds);
    DS_LOADED.with(|cell| cell.set(loads));
    body.solve_flags();
    body.emit(stats)
}

/// Whether the game has callbacks that Windows ran on a thread of its own,
/// such as `timeSetEvent`'s. The translated game has one thread, so its loops
/// let the runtime run them. Set once, before any function is lifted.
pub static POLLS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Whether the program being translated uses segments: a DOS program reaches
/// its PSP and environment through selectors of their own. Its segment
/// registers then hold selectors, and memory accesses add their segment's
/// base. A Windows program is flat, and its code stays free of that.
pub static SEGMENTS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn segments() -> bool {
    SEGMENTS.load(std::sync::atomic::Ordering::Relaxed)
}

/// The CPU's index for a segment register: ES, CS, SS, DS, FS, GS.
fn segment_index(register: Register) -> usize {
    match register {
        Register::ES => 0,
        Register::CS => 1,
        Register::SS => 2,
        Register::DS => 3,
        Register::FS => 4,
        _ => 5,
    }
}

thread_local! {
    /// Whether the function being lifted loads DS. Only such functions add
    /// DS's base to their memory accesses: Watcom's flat model keeps DS on the
    /// flat selector everywhere else, as synd (Syndicate Plus) also assumes.
    static DS_LOADED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn ds_loaded() -> bool {
    DS_LOADED.with(|cell| cell.get())
}

/// What a function that loads DS runs before it leaves its own code: a call,
/// a jump to another function or a return. If DS isn't flat there, code that
/// assumed it is would read the wrong memory, so it stops instead. Polls
/// aren't checked: a loop may poll with DS moved, and the interrupt handlers
/// the poll runs load their own DS.
fn ds_check(at: u32) -> String {
    if ds_loaded() { format!("cpu.flat_ds({at:#x}); ") } else { String::new() }
}

fn writes_ds(i: &Instruction) -> bool {
    i.mnemonic() == Mnemonic::Lds || (i.op_count() > 0 && i.op0_kind() == OpKind::Register && i.op0_register() == Register::DS)
}

fn poll() -> &'static str {
    if POLLS.load(std::sync::atomic::Ordering::Relaxed) { "cpu.poll(); " } else { "" }
}

struct Function<'a> {
    /// Where the functions translated together start. The first names them.
    starts: BTreeSet<u32>,
    report: &'a Report,
    all_functions: &'a HashSet<u32>,
    blocks: BTreeMap<u32, Block>,
}

fn jump_successors(address: u32, instruction: &Instruction, report: &Report) -> (Vec<u32>, bool) {
    let next = address + instruction.len() as u32;
    match instruction.flow_control() {
        FlowControl::UnconditionalBranch => (vec![branch_target(&instruction)], true),
        FlowControl::ConditionalBranch => (vec![branch_target(&instruction), next], true),
        FlowControl::IndirectBranch => (report.jump_tables.get(&address).cloned().unwrap_or_default(), true),
        FlowControl::Return | FlowControl::Exception => (Vec::new(), true),
        _ => (vec![next], false),
    }
}

impl<'a> Function<'a> {
    fn internal(&self, target: u32) -> bool {
        self.starts.contains(&target) || (!self.all_functions.contains(&target) && self.report.instructions.contains_key(&target))
    }

    fn collect(starts: &[u32], report: &'a Report, all_functions: &'a HashSet<u32>) -> Self {
        let starts: BTreeSet<u32> = starts.iter().copied().collect();
        let mut leaders = starts.clone();
        let mut work: Vec<u32> = starts.iter().copied().collect();
        let mut function = Function { starts, report, all_functions, blocks: BTreeMap::new() };
        let mut seen: BTreeSet<u32> = BTreeSet::new();
        while let Some(address) = work.pop() {
            if !seen.insert(address) {
                continue;
            }
            let Some(instruction) = report.instructions.get(&address) else { continue };
            let (successors, jumps) = jump_successors(address, instruction, report);
            for target in successors {
                if function.internal(target) {
                    if jumps {
                        leaders.insert(target);
                    }
                    work.push(target);
                }
            }
        }
        for &leader in &leaders {
            let mut block = Block { addresses: Vec::new(), successors: Vec::new(), live_out: 0, live_in: 0 };
            let mut address = leader;
            loop {
                block.addresses.push(address);
                let Some(instruction) = report.instructions.get(&address) else { break };
                let (successors, jumps) = jump_successors(address, instruction, report);
                let next = address + instruction.len() as u32;
                let ends = jumps || leaders.contains(&next) || !seen.contains(&next) || !function.internal(next);
                if ends {
                    block.successors = successors.into_iter().filter(|&t| leaders.contains(&t)).collect();
                    break;
                }
                address = next;
            }
            function.blocks.insert(leader, block);
        }
        function
    }

    /// True if control can leave this function at the end of the block.
    fn leaves_function(&self, block: &Block) -> bool {
        let last = *block.addresses.last().unwrap();
        let Some(instruction) = self.report.instructions.get(&last) else { return true };
        let (targets, jumps) = jump_successors(last, instruction, self.report);
        let exits = matches!(instruction.flow_control(), FlowControl::Return | FlowControl::IndirectBranch);
        let next = last + instruction.len() as u32;
        exits || targets.iter().any(|t| !self.blocks.contains_key(t)) || (!jumps && !self.blocks.contains_key(&next))
    }

    /// Finds which flags each block's successors read, so instructions only
    /// compute flags something uses. Hand-written assembly returns results in
    /// flags, with `call` then `je`, so every flag is live when a function ends
    /// and nothing assumes a call changes them.
    fn solve_flags(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            let leaders: Vec<u32> = self.blocks.keys().copied().rev().collect();
            for leader in leaders {
                let block = &self.blocks[&leader];
                let exits = if self.leaves_function(block) { ALL_FLAGS } else { 0 };
                let out = block.successors.iter().fold(exits, |live, s| live | self.blocks[s].live_in);
                let mut live = out;
                for address in block.addresses.iter().rev() {
                    let Some(instruction) = self.report.instructions.get(address) else { continue };
                    live = live_before(instruction, live);
                }
                let block = self.blocks.get_mut(&leader).unwrap();
                if block.live_out != out || block.live_in != live {
                    block.live_out = out;
                    block.live_in = live;
                    changed = true;
                }
            }
        }
    }

    /// A jump from the instruction at `from`. A game with timer callbacks gets
    /// a poll on every jump backwards, so a loop that only waits for a counter
    /// the callback changes sees it change.
    fn jump(&self, from: u32, target: u32) -> String {
        if self.blocks.contains_key(&target) {
            let poll = if target <= from { poll() } else { "" };
            format!("{poll}pc = {target:#x}; continue;")
        } else if self.all_functions.contains(&target) {
            format!("{}f_{target:08x}(cpu); return;", ds_check(from))
        } else {
            format!("cpu.call({target:#x}); return;")
        }
    }

    /// True if the straight code from a function's start to the `ret` at
    /// `address` leaves more on the stack than it found: the `ret` then goes
    /// to a value the function pushed, not back to its caller. Anything else,
    /// such as another block, a call or other changes to `esp`, keeps it an
    /// ordinary return. (`setjmp` pops its return address and pushes it back,
    /// so a push alone doesn't decide it.)
    fn pushed_before(&self, address: u32) -> bool {
        use Mnemonic as M;
        let writes_esp = |i: &Instruction| i.op_count() > 0 && i.op0_kind() == OpKind::Register && i.op0_register().full_register32() == Register::ESP;
        let mut at = address;
        let mut net: i32 = 0;
        for _ in 0..64 {
            let Some((&before, i)) = self.report.instructions.range(at.saturating_sub(15)..at).rev().find(|(&a, i)| a + i.len() as u32 == at) else {
                return false;
            };
            match i.mnemonic() {
                M::Push => net += 1,
                M::Pop => net -= 1,
                M::Pushad | M::Popad | M::Pushfd | M::Popfd | M::Pushf | M::Popf | M::Leave | M::Enter => return false,
                _ if i.flow_control() == FlowControl::Next && !writes_esp(i) => {}
                _ => return false,
            }
            at = before;
            if self.starts.contains(&at) {
                return net > 0;
            }
            if self.blocks.contains_key(&at) {
                return false;
            }
        }
        false
    }

    fn emit(&self, stats: &mut Stats) -> String {
        let first = *self.starts.first().unwrap();
        let mut text = if self.starts.len() == 1 {
            format!("pub fn f_{first:08x}(cpu: &mut Cpu) {{\n if cpu.profiling {{ cpu.hit({first:#x}); }}\n let mut pc: u32 = {first:#x};\n loop {{ match pc {{\n")
        } else {
            let mut text = String::new();
            for start in &self.starts {
                let _ = writeln!(text, "pub fn f_{start:08x}(cpu: &mut Cpu) {{ if cpu.profiling {{ cpu.hit({start:#x}); }} r_{first:08x}(cpu, {start:#x}); }}");
            }
            let _ = write!(text, "fn r_{first:08x}(cpu: &mut Cpu, start: u32) {{\n let mut pc: u32 = start;\n loop {{ match pc {{\n");
            text
        };
        for (&leader, block) in &self.blocks {
            let _ = writeln!(text, " {leader:#x} => {{");
            let mut live_after = vec![0; block.addresses.len()];
            let mut live = block.live_out;
            for (index, address) in block.addresses.iter().enumerate().rev() {
                live_after[index] = live;
                if let Some(instruction) = self.report.instructions.get(address) {
                    live = live_before(instruction, live);
                }
            }
            let mut ended = false;
            for (index, address) in block.addresses.iter().enumerate() {
                let Some(instruction) = self.report.instructions.get(address) else {
                    let _ = writeln!(text, "  cpu.call({address:#x}); return;");
                    ended = true;
                    break;
                };
                let line = self.instruction(*address, instruction, live_after[index], stats);
                let _ = writeln!(text, "  {line}");
                ended = matches!(instruction.flow_control(), FlowControl::UnconditionalBranch | FlowControl::IndirectBranch | FlowControl::Return | FlowControl::Exception);
            }
            if !ended {
                let last = *block.addresses.last().unwrap();
                let next = last + self.report.instructions[&last].len() as u32;
                let _ = writeln!(text, "  {}", self.jump(last, next));
            }
            text.push_str(" }\n");
        }
        text.push_str(" _ => { cpu.call(pc); return; }\n } }\n}\n");
        text
    }

    fn instruction(&self, address: u32, i: &Instruction, live: u32, stats: &mut Stats) -> String {
        let next = address + i.len() as u32;
        match i.flow_control() {
            FlowControl::UnconditionalBranch => return self.jump(address, branch_target(&i)),
            FlowControl::ConditionalBranch => {
                let name = format!("{:?}", i.mnemonic());
                let condition = match i.mnemonic() {
                    Mnemonic::Jecxz => "cpu.ecx == 0".to_string(),
                    // With an address-size prefix the test is on CX alone.
                    Mnemonic::Jcxz => "cpu.ecx & 0xffff == 0".to_string(),
                    Mnemonic::Loop => "{ cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 }".to_string(),
                    Mnemonic::Loope => "{ cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf }".to_string(),
                    Mnemonic::Loopne => "{ cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf }".to_string(),
                    _ => condition(&name[1..]),
                };
                return format!("if {condition} {{ {} }}", self.jump(address, branch_target(&i)));
            }
            // After a call, a return to somewhere else than the next
            // instruction means the game reset its stack to a frame further
            // up, as a `longjmp` does: this frame is gone too, so it returns
            // at once, and the frame that made that call goes on.
            FlowControl::Call => {
                let target = branch_target(&i);
                let unwind = format!("if cpu.returned_to != {next:#x} {{ return; }}");
                let mut check = ds_check(address);
                // A far call: the code selector goes on the stack first, for `retf`.
                if i.op0_kind() == OpKind::FarBranch32 {
                    check.push_str("cpu.push(cpu.segments[1] as u32); ");
                }
                if self.all_functions.contains(&target) {
                    return format!("{check}cpu.push({next:#x}); f_{target:08x}(cpu); {unwind}");
                }
                return format!("{check}cpu.push({next:#x}); cpu.call({target:#x}); {unwind}");
            }
            // A far call through a selector and offset in memory: the code
            // selector goes on the stack too, and `retf` takes it off.
            FlowControl::IndirectCall if i.op_kind(0) == OpKind::Memory && i.memory_size() == iced_x86::MemorySize::SegPtr32 => {
                return format!(
                    "let t = {}; {}cpu.push(cpu.segments[1] as u32); cpu.push({next:#x}); cpu.call(t); if cpu.returned_to != {next:#x} {{ return; }}",
                    read(i, 0, 4),
                    ds_check(address)
                )
            }
            FlowControl::IndirectCall => {
                return format!("let t = {}; {}cpu.push({next:#x}); cpu.call(t); if cpu.returned_to != {next:#x} {{ return; }}", read(i, 0, 4), ds_check(address))
            }
            FlowControl::IndirectBranch => return format!("{}pc = {}; continue;", poll(), read(i, 0, 4)),
            FlowControl::Return => {
                // The end of an interrupt handler, which the runtime called
                // with the flags, a selector and a return address pushed.
                if i.mnemonic() == Mnemonic::Iretd {
                    return format!("{}cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;", ds_check(address));
                }
                // A far return: the offset, then the code selector.
                if i.mnemonic() == Mnemonic::Retf {
                    let pops = if i.op_count() == 1 { i.immediate16() as u32 } else { 0 };
                    return format!("cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add({pops}); return;");
                }
                if i.mnemonic() != Mnemonic::Ret {
                    return missing(address, i, stats);
                }
                // `push target; ret` is a jump, which Watcom's int386 uses
                // to run one of its `int N; ret` stubs. That stub's own `ret`
                // then returns from this function.
                if i.op_count() == 0 && self.pushed_before(address) {
                    return format!("{}pc = cpu.pop(); continue;", poll());
                }
                let pops = if i.op_count() == 1 { i.immediate16() as u32 } else { 0 };
                return format!("{}cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add({});  return;", ds_check(address), 4 + pops);
            }
            _ => {}
        }
        match translate(i, live) {
            Some(text) => format!("{{ {text} }}"),
            None => missing(address, i, stats),
        }
    }
}

fn live_before(instruction: &Instruction, live: u32) -> u32 {
    let kills = match instruction.flow_control() {
        // A callee may set flags or leave them alone, so a call kills nothing.
        FlowControl::Call | FlowControl::IndirectCall => 0,
        _ => instruction.rflags_modified(),
    };
    (live & !kills) | instruction.rflags_read()
}

fn missing(address: u32, i: &Instruction, stats: &mut Stats) -> String {
    let name = format!("{:?}", i.mnemonic());
    *stats.missing.entry(name.clone()).or_default() += 1;
    format!("cpu.unimplemented({address:#x}, \"{name}\");")
}

fn condition(name: &str) -> String {
    match name {
        "e" => "cpu.zf",
        "ne" => "!cpu.zf",
        "a" => "(!cpu.cf && !cpu.zf)",
        "ae" => "!cpu.cf",
        "b" => "cpu.cf",
        "be" => "(cpu.cf || cpu.zf)",
        "g" => "(!cpu.zf && cpu.sf == cpu.of)",
        "ge" => "(cpu.sf == cpu.of)",
        "l" => "(cpu.sf != cpu.of)",
        "le" => "(cpu.zf || cpu.sf != cpu.of)",
        "s" => "cpu.sf",
        "ns" => "!cpu.sf",
        "o" => "cpu.of",
        "no" => "!cpu.of",
        "p" => "cpu.pf()",
        "np" => "!cpu.pf()",
        other => panic!("unknown condition {other}"),
    }
    .to_string()
}

fn register32(register: Register) -> String {
    format!("cpu.{}", format!("{:?}", register.full_register32()).to_lowercase())
}

fn address(i: &Instruction) -> String {
    let mut text = format!("{:#x}u32", i.memory_displacement32());
    if i.memory_base() != Register::None {
        let _ = write!(text, ".wrapping_add({})", register32(i.memory_base()));
    }
    if i.memory_index() != Register::None {
        let _ = write!(text, ".wrapping_add({}.wrapping_mul({}))", register32(i.memory_index()), i.memory_index_scale());
    }
    if segments() {
        // The stack is flat. Every other segment adds its base.
        match i.memory_segment() {
            Register::SS | Register::CS | Register::None => {}
            Register::DS if !ds_loaded() => {}
            segment => {
                let _ = write!(text, ".wrapping_add(cpu.bases[{}])", segment_index(segment));
            }
        }
    } else if i.memory_segment() == Register::FS {
        text.push_str(".wrapping_add(cpu.fs_base)");
    }
    text
}

fn size_of(i: &Instruction, operand: u32) -> u32 {
    match i.op_kind(operand) {
        OpKind::Register => i.op_register(operand).size() as u32,
        OpKind::Memory => i.memory_size().size() as u32,
        _ => 4,
    }
}

fn high_byte(register: Register) -> bool {
    matches!(register, Register::AH | Register::CH | Register::DH | Register::BH)
}

/// An operand's value, as an unsigned integer of `size` bytes.
fn read(i: &Instruction, operand: u32, size: u32) -> String {
    match i.op_kind(operand) {
        OpKind::Register if i.op_register(operand).is_segment_register() && segments() => {
            format!("(cpu.segments[{}] as u{})", segment_index(i.op_register(operand)), size * 8)
        }
        OpKind::Register if i.op_register(operand).is_segment_register() => format!("{FLAT_SEGMENT:#x}u{}", size * 8),
        OpKind::Register => {
            let register = i.op_register(operand);
            let full = register32(register);
            match register.size() {
                4 => full,
                2 => format!("({full} as u16)"),
                _ if high_byte(register) => format!("(({full} >> 8) as u8)"),
                _ => format!("({full} as u8)"),
            }
        }
        OpKind::Memory => format!("cpu.r{}({})", size * 8, address(i)),
        _ => {
            let value = i.immediate(operand);
            match size {
                1 => format!("{:#x}u8", value as u8),
                2 => format!("{:#x}u16", value as u16),
                _ => format!("{:#x}u32", value as u32),
            }
        }
    }
}

/// Stores an expression of the operand's size.
fn write_to(i: &Instruction, operand: u32, value: &str) -> String {
    match i.op_kind(operand) {
        OpKind::Register if i.op_register(operand).is_segment_register() && segments() => {
            format!("let v = {value}; cpu.set_segment({}, v as u16);", segment_index(i.op_register(operand)))
        }
        // Memory is flat, so loading a segment register changes nothing.
        OpKind::Register if i.op_register(operand).is_segment_register() => format!("let _ = {value};"),
        OpKind::Register => {
            let register = i.op_register(operand);
            let full = register32(register);
            match register.size() {
                4 => format!("let v: u32 = {value}; {full} = v;"),
                2 => format!("let v: u16 = {value}; {full} = ({full} & 0xffff0000) | v as u32;"),
                _ if high_byte(register) => format!("let v: u8 = {value}; {full} = ({full} & 0xffff00ff) | ((v as u32) << 8);"),
                _ => format!("let v: u8 = {value}; {full} = ({full} & 0xffffff00) | v as u32;"),
            }
        }
        _ => {
            let bits = size_of(i, operand) * 8;
            format!("let v: u{bits} = {value}; cpu.w{bits}({}, v);", address(i))
        }
    }
}

fn translate(i: &Instruction, live: u32) -> Option<String> {
    let flags_used = live & i.rflags_modified() != 0;
    let size = if i.op_count() > 0 { size_of(i, 0) } else { 4 };
    let bits = size * 8;
    let binary = |helper: &str, plain: &str, store: bool| -> String {
        let (a, b) = (read(i, 0, size), read(i, 1, size));
        let value = if flags_used || plain.is_empty() {
            format!("cpu.{helper}{bits}({a}, {b})")
        } else {
            format!("{a}.{plain}({b})")
        };
        if store { write_to(i, 0, &value) } else { format!("{value};") }
    };
    let bitwise = |helper: &str, operator: &str| -> String {
        let (a, b) = (read(i, 0, size), read(i, 1, size));
        let value = if flags_used { format!("cpu.{helper}{bits}({a}, {b})") } else { format!("{a} {operator} {b}") };
        write_to(i, 0, &value)
    };
    let shift = |helper: &str| -> String {
        let count = match i.op_kind(1) {
            OpKind::Register => "(cpu.ecx & 0xff)".to_string(),
            _ => format!("{}u32", i.immediate(1) as u8),
        };
        write_to(i, 0, &format!("cpu.{helper}{bits}({}, {count})", read(i, 0, size)))
    };
    let unary = |helper: &str| write_to(i, 0, &format!("cpu.{helper}{bits}({})", read(i, 0, size)));
    let name = format!("{:?}", i.mnemonic());
    if name.starts_with("Set") {
        return Some(write_to(i, 0, &format!("({}) as u8", condition(&name[3..]))));
    }
    if name.starts_with("Cmov") {
        return Some(format!("if {} {{ {} }}", condition(&name[4..]), write_to(i, 0, &read(i, 1, size))));
    }
    if let Some(text) = fpu(i) {
        return Some(text);
    }
    // Hardware ports, which code brought over from DOS still reads and writes.
    // The port is DX or a byte in the instruction, and the runtime answers.
    let port = |operand: u32| match i.op_kind(operand) {
        OpKind::Register => "(cpu.edx as u16)".to_string(),
        _ => format!("{:#x}u16", i.immediate(operand) as u16),
    };
    match i.mnemonic() {
        Mnemonic::In => return Some(write_to(i, 0, &format!("cpu.port_in({}, {size}) as u{bits}", port(1)))),
        Mnemonic::Out => {
            let size = size_of(i, 1);
            return Some(format!("cpu.port_out({}, {size}, {} as u32);", port(0), read(i, 1, size)));
        }
        // Control and debug registers: start-up code of a DOS extender reads
        // CR0 to check for protected mode and the FPU. Reads give what a
        // 386 in protected mode with an FPU has, and writes change nothing.
        Mnemonic::Mov if (0..2).any(|op| i.op_kind(op) == OpKind::Register && (i.op_register(op).is_cr() || i.op_register(op).is_dr() || i.op_register(op).is_tr())) => {
            let special = i.op_register(0).is_cr() || i.op_register(0).is_dr() || i.op_register(0).is_tr();
            const PROTECTED_WITH_FPU: u32 = 0x11;
            return Some(if special { String::new() } else { write_to(i, 0, &format!("{PROTECTED_WITH_FPU:#x}u32")) });
        }
        // A DOS program asks DOS, the BIOS and its extender for everything
        // through software interrupts, which the runtime answers.
        Mnemonic::Int => return Some(format!("cpu.interrupt({:#x});", i.immediate8())),
        // Interrupts can't arrive in the middle of translated code: the
        // runtime runs a game's interrupt handlers at safe points, and only
        // while interrupts are on.
        Mnemonic::Cli => return Some("cpu.interrupts_off = true;".to_string()),
        Mnemonic::Sti => return Some("cpu.interrupts_off = false;".to_string()),
        _ => {}
    }
    use Mnemonic as M;
    Some(match i.mnemonic() {
        M::Mov => write_to(i, 0, &read(i, 1, size)),
        M::Movzx => {
            let source = read(i, 1, size_of(i, 1));
            write_to(i, 0, &format!("{source} as u{bits}"))
        }
        M::Movsx => {
            let source_bits = size_of(i, 1) * 8;
            write_to(i, 0, &format!("{} as i{source_bits} as i{bits} as u{bits}", read(i, 1, size_of(i, 1))))
        }
        M::Lea => {
            let mut text = format!("{:#x}u32", i.memory_displacement32());
            if i.memory_base() != Register::None {
                let _ = write!(text, ".wrapping_add({})", register32(i.memory_base()));
            }
            if i.memory_index() != Register::None {
                let _ = write!(text, ".wrapping_add({}.wrapping_mul({}))", register32(i.memory_index()), i.memory_index_scale());
            }
            write_to(i, 0, &if size == 2 { format!("{text} as u16") } else { text })
        }
        M::Xchg => format!(
            "let a = {}; let b = {}; {{ {} }} {{ {} }}",
            read(i, 0, size), read(i, 1, size), write_to(i, 0, "b"), write_to(i, 1, "a")
        ),
        M::Push if format!("{:?}", i.code()).starts_with("Pushw") || format!("{:?}", i.code()).contains("16") => {
            format!("let v = {}; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v);", read(i, 0, 2))
        }
        M::Pop if format!("{:?}", i.code()).starts_with("Popw") || format!("{:?}", i.code()).contains("16") => {
            format!("let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); {}", write_to(i, 0, "value"))
        }
        // A far pointer: the offset into the register, the selector after it
        // into the segment register.
        M::Les | M::Lds | M::Lfs | M::Lgs | M::Lss if size_of(i, 0) == 4 => {
            let segment = match i.mnemonic() {
                M::Les => 0,
                M::Lss => 2,
                M::Lds => 3,
                M::Lfs => 4,
                _ => 5,
            };
            let at = address(i);
            format!("let at = {at}; let selector = cpu.r16(at.wrapping_add(4)); {} cpu.set_segment({segment}, selector);", write_to(i, 0, "cpu.r32(at)"))
        }
        M::Push if i.op_kind(0) == OpKind::Register && i.op0_register().is_segment_register() && segments() => {
            format!("cpu.push(cpu.segments[{}] as u32);", segment_index(i.op0_register()))
        }
        M::Pop if i.op_kind(0) == OpKind::Register && i.op0_register().is_segment_register() && segments() => {
            format!("let v = cpu.pop(); cpu.set_segment({}, v as u16);", segment_index(i.op0_register()))
        }
        M::Push if i.op_kind(0) == OpKind::Register && i.op0_register().is_segment_register() => format!("cpu.push({FLAT_SEGMENT:#x});"),
        M::Pop if i.op_kind(0) == OpKind::Register && i.op0_register().is_segment_register() => "cpu.pop();".to_string(),
        M::Push if size == 4 || i.op_kind(0) != OpKind::Memory && i.op_kind(0) != OpKind::Register => format!("let v = {}; cpu.push(v);", read(i, 0, 4)),
        M::Pop if size == 4 => write_to(i, 0, "cpu.pop()"),
        M::Xlatb => "let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32;".to_string(),
        M::Sal => shift("shl"),
        M::Pushad => "cpu.pushad();".to_string(),
        M::Popad => "cpu.popad();".to_string(),
        M::Pushfd => "let v = cpu.flags(); cpu.push(v);".to_string(),
        M::Popfd => "let v = cpu.pop(); cpu.set_flags(v);".to_string(),
        M::Add => binary("add", "wrapping_add", true),
        M::Sub => binary("sub", "wrapping_sub", true),
        M::Adc => binary("adc", "", true),
        M::Sbb => binary("sbb", "", true),
        M::And => bitwise("and", "&"),
        M::Or => bitwise("or", "|"),
        M::Xor => bitwise("xor", "^"),
        M::Cmp => format!("cpu.sub{bits}({}, {});", read(i, 0, size), read(i, 1, size)),
        M::Test => format!("cpu.and{bits}({}, {});", read(i, 0, size), read(i, 1, size)),
        M::Inc => unary("inc"),
        M::Dec => unary("dec"),
        M::Neg => unary("neg"),
        M::Not => write_to(i, 0, &format!("!{}", read(i, 0, size))),
        M::Shl => shift("shl"),
        M::Shr => shift("shr"),
        M::Sar => shift("sar"),
        M::Rol => shift("rol"),
        M::Ror => shift("ror"),
        M::Rcl => shift("rcl"),
        M::Rcr => shift("rcr"),
        M::Shld | M::Shrd if size == 4 => {
            let count = match i.op_kind(2) {
                OpKind::Register => "(cpu.ecx & 0xff)".to_string(),
                _ => format!("{}u32", i.immediate(2) as u8),
            };
            let helper = if i.mnemonic() == M::Shld { "shld32" } else { "shrd32" };
            write_to(i, 0, &format!("cpu.{helper}({}, {}, {count})", read(i, 0, 4), read(i, 1, 4)))
        }
        M::Shld | M::Shrd if size == 2 => {
            let count = match i.op_kind(2) {
                OpKind::Register => "(cpu.ecx & 31)".to_string(),
                _ => format!("{}u32", i.immediate(2) as u8 & 31),
            };
            // Both halves side by side in 32 bits, so one shift moves the bits across.
            let (wide, result, carry) = if i.mnemonic() == M::Shld {
                ("((a << 16) | b) << n", "(wide >> 16) as u16", "(a >> (16 - n)) & 1 != 0")
            } else {
                ("((b << 16) | a) >> n", "wide as u16", "(a >> (n - 1)) & 1 != 0")
            };
            format!(
                "let a = {} as u32; let b = {} as u32; let n = {count}; if n != 0 && n <= 16 {{ let wide: u32 = {wide}; let r = {result}; cpu.cf = {carry}; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; {} }}",
                read(i, 0, 2),
                read(i, 1, 2),
                write_to(i, 0, "r")
            )
        }
        M::Pushf => "let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v);".to_string(),
        M::Popf => "let v = cpu.r16(cpu.esp) as u32; cpu.esp = cpu.esp.wrapping_add(2); let h = cpu.eflags_high; cpu.set_flags(v); cpu.eflags_high = h;".to_string(),
        M::Mul => format!("let v = {}; cpu.mul{bits}(v);", read(i, 0, size)),
        M::Div => format!("let v = {}; cpu.div{bits}(v);", read(i, 0, size)),
        M::Idiv => format!("let v = {}; cpu.idiv{bits}(v);", read(i, 0, size)),
        M::Imul if i.op_count() == 1 => format!("let v = {}; cpu.imul{bits}(v);", read(i, 0, size)),
        M::Imul if size >= 2 => {
            let (a, b) = if i.op_count() == 2 { (read(i, 0, size), read(i, 1, size)) } else { (read(i, 1, size), read(i, 2, size)) };
            write_to(i, 0, &format!("cpu.imul2_{bits}({a}, {b})"))
        }
        M::Cdq => "cpu.edx = ((cpu.eax as i32) >> 31) as u32;".to_string(),
        M::Cwde => "cpu.eax = cpu.eax as u16 as i16 as i32 as u32;".to_string(),
        M::Cbw => "let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v);".to_string(),
        M::Cwd => "cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32);".to_string(),
        M::Leave => "cpu.esp = cpu.ebp; cpu.ebp = cpu.pop();".to_string(),
        // `enter size, 0`: a frame without the display of outer frames, which
        // is all compilers emit.
        M::Enter if i.immediate8_2nd() == 0 && i.code() == iced_x86::Code::Enterd_imm16_imm8 => {
            format!("cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub({});", i.immediate16())
        }
        M::Nop | M::Fnop | M::Wait => String::new(),
        M::Bswap => write_to(i, 0, &format!("{}.swap_bytes()", read(i, 0, 4))),
        // Bit scans: the index of the lowest (bsf) or highest (bsr) set bit.
        // A zero source sets ZF and leaves the destination as it was, as
        // Intel's processors do.
        M::Bsf | M::Bsr if size == 2 || size == 4 => {
            let ty = if size == 2 { "u16" } else { "u32" };
            let index = if i.mnemonic() == M::Bsf { "s.trailing_zeros()".to_string() } else { format!("{} - s.leading_zeros()", bits - 1) };
            format!(
                "let s = {} as {ty}; cpu.zf = s == 0; if s != 0 {{ {} }}",
                read(i, 1, size),
                write_to(i, 0, &format!("({index}) as {ty}"))
            )
        }
        M::Bt | M::Bts | M::Btr | M::Btc if i.op_kind(0) == OpKind::Register || i.op_kind(1) != OpKind::Register => {
            let (a, b) = (read(i, 0, size), read(i, 1, 1));
            let index = if i.op_kind(1) == OpKind::Register { format!("({} as u32 & {})", read(i, 1, size), bits - 1) } else { format!("({b} as u32 & {})", bits - 1) };
            let test = format!("let a = {a}; let index = {index}; cpu.cf = (a >> index) & 1 != 0;");
            match i.mnemonic() {
                M::Bt => test,
                M::Bts => format!("{test} {}", write_to(i, 0, &format!("a | (1 << index)"))),
                M::Btr => format!("{test} {}", write_to(i, 0, &format!("a & !(1 << index)"))),
                _ => format!("{test} {}", write_to(i, 0, &format!("a ^ (1 << index)"))),
            }
        }
        M::Bt | M::Bts | M::Btr | M::Btc if size == 4 => {
            // With a register index, the bit can be outside the addressed word.
            let store = match i.mnemonic() {
                M::Bt => "",
                M::Bts => "cpu.w32(at, a | (1 << bit));",
                M::Btr => "cpu.w32(at, a & !(1 << bit));",
                _ => "cpu.w32(at, a ^ (1 << bit));",
            };
            format!(
                "let index = {}; let at = ({}).wrapping_add((((index as i32) >> 5) as u32).wrapping_mul(4)); let bit = index & 31; let a = cpu.r32(at); cpu.cf = (a >> bit) & 1 != 0; {store}",
                read(i, 1, 4),
                address(i)
            )
        }
        M::Sahf => "let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i;".to_string(),
        M::Lahf => "let v = cpu.flags() & 0xff; cpu.eax = (cpu.eax & 0xffff00ff) | (v << 8);".to_string(),
        M::Cld => "cpu.df = false;".to_string(),
        M::Std => "cpu.df = true;".to_string(),
        M::Clc => "cpu.cf = false;".to_string(),
        M::Stc => "cpu.cf = true;".to_string(),
        M::Cmc => "cpu.cf = !cpu.cf;".to_string(),
        M::Cpuid => "cpu.cpuid();".to_string(),
        // On movs, stos and lods, `repne` repeats just as `rep` does: the
        // processor ignores the flags for them. Watcom's memcpy uses `repnz movs`.
        M::Movsb => format!("cpu.movs(1, {});", repeats(i)),
        M::Movsw => format!("cpu.movs(2, {});", repeats(i)),
        M::Movsd if i.op0_kind() != OpKind::Register => format!("cpu.movs(4, {});", repeats(i)),
        M::Stosb => format!("cpu.stos(1, {});", repeats(i)),
        M::Stosw => format!("cpu.stos(2, {});", repeats(i)),
        M::Stosd => format!("cpu.stos(4, {});", repeats(i)),
        M::Lodsb => format!("cpu.lods(1, {});", repeats(i)),
        M::Lodsw => format!("cpu.lods(2, {});", repeats(i)),
        M::Lodsd => format!("cpu.lods(4, {});", repeats(i)),
        M::Scasb => format!("cpu.scas(1, {});", repeat(i)),
        M::Scasw => format!("cpu.scas(2, {});", repeat(i)),
        M::Scasd => format!("cpu.scas(4, {});", repeat(i)),
        M::Cmpsb => format!("cpu.cmps(1, {});", repeat(i)),
        M::Cmpsw => format!("cpu.cmps(2, {});", repeat(i)),
        M::Cmpsd if i.op0_kind() != OpKind::Register => format!("cpu.cmps(4, {});", repeat(i)),
        _ => return None,
    })
}

/// Whether a movs, stos or lods repeats: with either prefix.
fn repeats(i: &Instruction) -> bool {
    i.has_rep_prefix() || i.has_repne_prefix()
}

fn repeat(i: &Instruction) -> u32 {
    if i.has_repe_prefix() {
        1
    } else if i.has_repne_prefix() {
        2
    } else {
        0
    }
}

fn st_index(register: Register) -> usize {
    register as usize - Register::ST0 as usize
}

/// The value of a floating-point memory operand, as f64.
fn float_memory(i: &Instruction) -> String {
    let at = address(i);
    match i.memory_size().size() {
        4 => format!("cpu.rf32({at})"),
        8 => format!("cpu.rf64({at})"),
        _ => format!("cpu.rf80({at})"),
    }
}

fn integer_memory(i: &Instruction) -> String {
    let at = address(i);
    match i.memory_size().size() {
        2 => format!("(cpu.r16({at}) as i16 as f64)"),
        4 => format!("(cpu.r32({at}) as i32 as f64)"),
        _ => format!("(cpu.r64({at}) as i64 as f64)"),
    }
}

fn fpu(i: &Instruction) -> Option<String> {
    use Mnemonic as M;
    let arithmetic = |d: usize, x: &str, order: &str| -> String {
        let value = match order {
            "+" | "*" | "-" | "/" => format!("cpu.st({d}) {order} {x}"),
            "r-" => format!("{x} - cpu.st({d})"),
            _ => format!("{x} / cpu.st({d})"),
        };
        format!("let v = {value}; cpu.set_st({d}, v);")
    };
    let binary = |order: &str, pop: bool| -> String {
        if i.op_kind(0) == OpKind::Memory {
            return arithmetic(0, &float_memory(i), order);
        }
        let (d, s) = (st_index(i.op0_register()), st_index(i.op1_register()));
        let text = arithmetic(d, &format!("cpu.st({s})"), order);
        if pop { format!("{text} cpu.fpop();") } else { text }
    };
    let integer = |order: &str| arithmetic(0, &integer_memory(i), order);
    let compare = |pops: u32| -> String {
        let other = match i.op_kind(0) {
            // fcompp and fucompp name no operands, and always use ST(1).
            _ if i.op_count() == 0 => "cpu.st(1)".to_string(),
            OpKind::Memory => float_memory(i),
            OpKind::Register if i.op_count() == 2 => format!("cpu.st({})", st_index(i.op1_register())),
            OpKind::Register => format!("cpu.st({})", st_index(i.op0_register())),
            _ => "cpu.st(1)".to_string(),
        };
        format!("let a = cpu.st(0); let b = {other}; cpu.fcom(a, b); {}", "cpu.fpop(); ".repeat(pops as usize))
    };
    let comparei = |pop: bool| -> String {
        let text = format!("let a = cpu.st(0); let b = cpu.st({}); cpu.fcomi(a, b);", st_index(i.op1_register()));
        if pop { format!("{text} cpu.fpop();") } else { text }
    };
    let store = |pop: bool| -> String {
        let text = match i.op_kind(0) {
            OpKind::Memory => {
                let helper = match i.memory_size().size() { 4 => "wf32", 8 => "wf64", _ => "wf80" };
                format!("let v = cpu.st(0); cpu.{helper}({}, v);", address(i))
            }
            _ => format!("let v = cpu.st(0); cpu.set_st({}, v);", st_index(i.op0_register())),
        };
        if pop { format!("{text} cpu.fpop();") } else { text }
    };
    let store_integer = |pop: bool| -> String {
        let at = address(i);
        let text = match i.memory_size().size() {
            2 => format!("let v = cpu.fi16(cpu.st(0)); cpu.w16({at}, v);"),
            4 => format!("let v = cpu.fi32(cpu.st(0)); cpu.w32({at}, v);"),
            _ => format!("let v = cpu.fi64(cpu.st(0)); cpu.w64({at}, v);"),
        };
        if pop { format!("{text} cpu.fpop();") } else { text }
    };
    let constant = |value: &str| format!("cpu.fpush({value});");
    let change = |value: &str| format!("let v = {value}; cpu.set_st(0, v);");
    Some(match i.mnemonic() {
        M::Fld => match i.op_kind(0) {
            OpKind::Memory => format!("let v = {}; cpu.fpush(v);", float_memory(i)),
            _ => format!("let v = cpu.st({}); cpu.fpush(v);", st_index(i.op0_register())),
        },
        M::Fild => format!("let v = {}; cpu.fpush(v);", integer_memory(i)),
        M::Fst => store(false),
        M::Fstp => store(true),
        M::Fist => store_integer(false),
        M::Fistp => store_integer(true),
        M::Fadd => binary("+", false),
        M::Faddp => binary("+", true),
        M::Fmul => binary("*", false),
        M::Fmulp => binary("*", true),
        M::Fsub => binary("-", false),
        M::Fsubp => binary("-", true),
        M::Fsubr => binary("r-", false),
        M::Fsubrp => binary("r-", true),
        M::Fdiv => binary("/", false),
        M::Fdivp => binary("/", true),
        M::Fdivr => binary("r/", false),
        M::Fdivrp => binary("r/", true),
        M::Fiadd => integer("+"),
        M::Fimul => integer("*"),
        M::Fisub => integer("-"),
        M::Fisubr => integer("r-"),
        M::Fidiv => integer("/"),
        M::Fidivr => integer("r/"),
        M::Fcom | M::Fucom => compare(0),
        M::Fcomp | M::Fucomp => compare(1),
        M::Fcompp | M::Fucompp => compare(2),
        M::Ficom => format!("let a = cpu.st(0); let b = {}; cpu.fcom(a, b);", integer_memory(i)),
        M::Ficomp => format!("let a = cpu.st(0); let b = {}; cpu.fcom(a, b); cpu.fpop();", integer_memory(i)),
        M::Fcomi | M::Fucomi => comparei(false),
        M::Fcomip | M::Fucomip => comparei(true),
        M::Ftst => "let a = cpu.st(0); cpu.fcom(a, 0.0);".to_string(),
        M::Fxam => "cpu.fxam();".to_string(),
        M::Fxch => {
            let index = if i.op_count() == 2 { st_index(i.op1_register()) } else { 1 };
            format!("let a = cpu.st(0); let b = cpu.st({index}); cpu.set_st(0, b); cpu.set_st({index}, a);")
        }
        M::Fchs => change("-cpu.st(0)"),
        M::Fabs => change("cpu.st(0).abs()"),
        M::Fsqrt => change("cpu.st(0).sqrt()"),
        // C2 says an angle was out of range, and the C runtime checks it.
        M::Fsin => format!("cpu.c2 = false; {}", change("cpu.st(0).sin()")),
        M::Fcos => format!("cpu.c2 = false; {}", change("cpu.st(0).cos()")),
        M::Frndint => change("cpu.round(cpu.st(0))"),
        M::Fsincos => "cpu.c2 = false; let a = cpu.st(0); cpu.set_st(0, a.sin()); cpu.fpush(a.cos());".to_string(),
        M::Fptan => "cpu.c2 = false; let a = cpu.st(0); cpu.set_st(0, a.tan()); cpu.fpush(1.0);".to_string(),
        M::Fpatan => "let v = cpu.st(1).atan2(cpu.st(0)); cpu.fpop(); cpu.set_st(0, v);".to_string(),
        M::F2xm1 => change("2f64.powf(cpu.st(0)) - 1.0"),
        M::Fyl2x => "let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v);".to_string(),
        M::Fscale => "cpu.fscale();".to_string(),
        M::Fprem | M::Fprem1 => "cpu.fprem();".to_string(),
        M::Fld1 => constant("1.0"),
        M::Fldz => constant("0.0"),
        M::Fldpi => constant("std::f64::consts::PI"),
        M::Fldln2 => constant("std::f64::consts::LN_2"),
        M::Fldlg2 => constant("std::f64::consts::LOG10_2"),
        M::Fldl2e => constant("std::f64::consts::LOG2_E"),
        M::Fldl2t => constant("std::f64::consts::LOG2_10"),
        M::Fnstsw => match i.op_kind(0) {
            OpKind::Memory => format!("let v = cpu.fstatus(); cpu.w16({}, v);", address(i)),
            _ => "let v = cpu.fstatus(); cpu.set_ax(v);".to_string(),
        },
        M::Fnstcw => format!("let v = cpu.fpu_control; cpu.w16({}, v);", address(i)),
        M::Fldcw => format!("cpu.fpu_control = cpu.r16({});", address(i)),
        M::Fnclex | M::Ffree => String::new(),
        M::Fninit => "cpu.top = 0; cpu.fpu_control = 0x37f;".to_string(),
        _ => return None,
    })
}
