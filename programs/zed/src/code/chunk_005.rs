#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_0102b51c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102b51c); }
 let mut pc: u32 = 0x102b51c;
 loop { match pc {
 0x102b51c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4188u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.ecx = v; }
  pc = 0x102b530; continue;
 }
 0x102b530 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x270u32; cpu.edx = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x27f0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102b530; continue; }
  pc = 0x102b5ba; continue;
 }
 0x102b5ba => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x2800u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102b5c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102b5c3); }
 let mut pc: u32 = 0x102b5c3;
 loop { match pc {
 0x102b5c3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4188u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.eax = v; }
  pc = 0x102b5da; continue;
 }
 0x102b5da => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x270u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x102b5fb; continue;
 }
 0x102b5fb => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102b5fb; continue; }
  pc = 0x102b603; continue;
 }
 0x102b603 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102b5da; continue; }
  pc = 0x102b60b; continue;
 }
 0x102b60b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x280u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102b618(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102b618); }
 let mut pc: u32 = 0x102b618;
 loop { match pc {
 0x102b618 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e4188u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.eax = v; }
  pc = 0x102b638; continue;
 }
 0x102b638 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x270u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x102b659; continue;
 }
 0x102b659 => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102b659; continue; }
  pc = 0x102b661; continue;
 }
 0x102b661 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102b638; continue; }
  pc = 0x102b669; continue;
 }
 0x102b669 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x280u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102b676(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102b676); }
 let mut pc: u32 = 0x102b676;
 loop { match pc {
 0x102b676 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4188u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x274u32; cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x27f4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x102b72c; continue;
 }
 0x102b72c => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x270u32; cpu.edx = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x27f0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102b72c; continue; }
  pc = 0x102b7b6; continue;
 }
 0x102b7b6 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x27cu32; cpu.edx = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x2800u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102b83c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102b83c); }
 let mut pc: u32 = 0x102b83c;
 loop { match pc {
 0x102b83c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4188u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x274u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x102b86e; continue;
 }
 0x102b86e => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102b86e; continue; }
  pc = 0x102b878; continue;
 }
 0x102b878 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x102b882; continue;
 }
 0x102b882 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x270u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x102b8a3; continue;
 }
 0x102b8a3 => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102b8a3; continue; }
  pc = 0x102b8ab; continue;
 }
 0x102b8ab => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102b882; continue; }
  pc = 0x102b8b3; continue;
 }
 0x102b8b3 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x27cu32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  pc = 0x102b8d2; continue;
 }
 0x102b8d2 => {
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102b8d2; continue; }
  pc = 0x102b8da; continue;
 }
 0x102b8da => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x280u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102b8e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102b8e7); }
 let mut pc: u32 = 0x102b8e7;
 loop { match pc {
 0x102b8e7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e4188u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x274u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x102b922; continue;
 }
 0x102b922 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102b922; continue; }
  pc = 0x102b92c; continue;
 }
 0x102b92c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x102b936; continue;
 }
 0x102b936 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x270u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x102b957; continue;
 }
 0x102b957 => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102b957; continue; }
  pc = 0x102b95f; continue;
 }
 0x102b95f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102b936; continue; }
  pc = 0x102b967; continue;
 }
 0x102b967 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x27cu32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  pc = 0x102b986; continue;
 }
 0x102b986 => {
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102b986; continue; }
  pc = 0x102b98e; continue;
 }
 0x102b98e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x280u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102b99b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102b99b); }
 let mut pc: u32 = 0x102b99b;
 loop { match pc {
 0x102b99b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4188u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x278u32; cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x27f8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x102ba41; continue;
 }
 0x102ba41 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x270u32; cpu.edx = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x27f0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102ba41; continue; }
  pc = 0x102bacb; continue;
 }
 0x102bacb => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x278u32; cpu.edx = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x2800u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102bb61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102bb61); }
 let mut pc: u32 = 0x102bb61;
 loop { match pc {
 0x102bb61 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4188u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x278u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x102bb93; continue;
 }
 0x102bb93 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102bb93; continue; }
  pc = 0x102bb9c; continue;
 }
 0x102bb9c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x102bba6; continue;
 }
 0x102bba6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x270u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x102bbc7; continue;
 }
 0x102bbc7 => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102bbc7; continue; }
  pc = 0x102bbcf; continue;
 }
 0x102bbcf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102bba6; continue; }
  pc = 0x102bbd7; continue;
 }
 0x102bbd7 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x278u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  pc = 0x102bbf6; continue;
 }
 0x102bbf6 => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102bbf6; continue; }
  pc = 0x102bbff; continue;
 }
 0x102bbff => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x280u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102bc0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102bc0c); }
 let mut pc: u32 = 0x102bc0c;
 loop { match pc {
 0x102bc0c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e4188u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x278u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x102bc47; continue;
 }
 0x102bc47 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102bc47; continue; }
  pc = 0x102bc50; continue;
 }
 0x102bc50 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x102bc5a; continue;
 }
 0x102bc5a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x270u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x102bc7b; continue;
 }
 0x102bc7b => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102bc7b; continue; }
  pc = 0x102bc83; continue;
 }
 0x102bc83 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102bc5a; continue; }
  pc = 0x102bc8b; continue;
 }
 0x102bc8b => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x278u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  pc = 0x102bcaa; continue;
 }
 0x102bcaa => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102bcaa; continue; }
  pc = 0x102bcb3; continue;
 }
 0x102bcb3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x280u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102bcc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102bcc0); }
 let mut pc: u32 = 0x102bcc0;
 loop { match pc {
 0x102bcc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4188u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x27cu32; cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x27fcu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x102bd56; continue;
 }
 0x102bd56 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x270u32; cpu.edx = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x27f0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102bd56; continue; }
  pc = 0x102bde0; continue;
 }
 0x102bde0 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x274u32; cpu.edx = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x2800u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102be86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102be86); }
 let mut pc: u32 = 0x102be86;
 loop { match pc {
 0x102be86 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4188u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x27cu32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x102beb8; continue;
 }
 0x102beb8 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102beb8; continue; }
  pc = 0x102bec0; continue;
 }
 0x102bec0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x102beca; continue;
 }
 0x102beca => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x270u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x102beeb; continue;
 }
 0x102beeb => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102beeb; continue; }
  pc = 0x102bef3; continue;
 }
 0x102bef3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102beca; continue; }
  pc = 0x102befb; continue;
 }
 0x102befb => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x274u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  pc = 0x102bf1a; continue;
 }
 0x102bf1a => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102bf1a; continue; }
  pc = 0x102bf24; continue;
 }
 0x102bf24 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x280u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102bf31(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102bf31); }
 let mut pc: u32 = 0x102bf31;
 loop { match pc {
 0x102bf31 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e4188u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x27cu32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x102bf6c; continue;
 }
 0x102bf6c => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102bf6c; continue; }
  pc = 0x102bf74; continue;
 }
 0x102bf74 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x102bf7e; continue;
 }
 0x102bf7e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x270u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x102bf9f; continue;
 }
 0x102bf9f => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102bf9f; continue; }
  pc = 0x102bfa7; continue;
 }
 0x102bfa7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102bf7e; continue; }
  pc = 0x102bfaf; continue;
 }
 0x102bfaf => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = 0x274u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  pc = 0x102bfce; continue;
 }
 0x102bfce => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102bfce; continue; }
  pc = 0x102bfd8; continue;
 }
 0x102bfd8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x280u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102bfe5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102bfe5); }
 let mut pc: u32 = 0x102bfe5;
 loop { match pc {
 0x102bfe5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e4188u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.ecx = v; }
  pc = 0x102bffb; continue;
 }
 0x102bffb => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x4u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102c02e; continue; }
  pc = 0x102c01f; continue;
 }
 0x102c01f => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102c02d); f_01027bb1(cpu); if cpu.returned_to != 0x102c02d { return; }
  { cpu.popad(); }
  pc = 0x102c02e; continue;
 }
 0x102c02e => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102bffb; continue; }
  pc = 0x102c030; continue;
 }
 0x102c030 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x280u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c038(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102c038); }
 let mut pc: u32 = 0x102c038;
 loop { match pc {
 0x102c038 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e4188u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x102c06d; continue;
 }
 0x102c06d => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x4u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102c09d; continue; }
  pc = 0x102c08e; continue;
 }
 0x102c08e => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102c09c); f_01027bb1(cpu); if cpu.returned_to != 0x102c09c { return; }
  { cpu.popad(); }
  pc = 0x102c09d; continue;
 }
 0x102c09d => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102c06d; continue; }
  pc = 0x102c0a2; continue;
 }
 0x102c0a2 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x4u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102c0d2; continue; }
  pc = 0x102c0c3; continue;
 }
 0x102c0c3 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102c0d1); f_01027bb1(cpu); if cpu.returned_to != 0x102c0d1 { return; }
  { cpu.popad(); }
  pc = 0x102c0d2; continue;
 }
 0x102c0d2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x280u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c0da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102c0da); }
 let mut pc: u32 = 0x102c0da;
 loop { match pc {
 0x102c0da => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e4188u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x102c10e; continue;
 }
 0x102c10e => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x4u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102c13f; continue; }
  pc = 0x102c130; continue;
 }
 0x102c130 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102c13e); f_01027bb1(cpu); if cpu.returned_to != 0x102c13e { return; }
  { cpu.popad(); }
  pc = 0x102c13f; continue;
 }
 0x102c13f => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102c10e; continue; }
  pc = 0x102c143; continue;
 }
 0x102c143 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x4u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102c174; continue; }
  pc = 0x102c165; continue;
 }
 0x102c165 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102c173); f_01027bb1(cpu); if cpu.returned_to != 0x102c173 { return; }
  { cpu.popad(); }
  pc = 0x102c174; continue;
 }
 0x102c174 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x280u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c17c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102c17c); }
 let mut pc: u32 = 0x102c17c;
 loop { match pc {
 0x102c17c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e4188u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1121361u32, v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x1121359u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x102c1af; continue;
 }
 0x102c1af => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x4u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102c1e1; continue; }
  pc = 0x102c1d2; continue;
 }
 0x102c1d2 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102c1e0); f_01027bb1(cpu); if cpu.returned_to != 0x102c1e0 { return; }
  { cpu.popad(); }
  pc = 0x102c1e1; continue;
 }
 0x102c1e1 => {
  { cpu.movs(4, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102c1af; continue; }
  pc = 0x102c1e4; continue;
 }
 0x102c1e4 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 7u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1121361u32)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x4u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102c216; continue; }
  pc = 0x102c207; continue;
 }
 0x102c207 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102c215); f_01027bb1(cpu); if cpu.returned_to != 0x102c215 { return; }
  { cpu.popad(); }
  pc = 0x102c216; continue;
 }
 0x102c216 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x280u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c21e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102c21e); }
 let mut pc: u32 = 0x102c21e;
 loop { match pc {
 0x102c21e => {
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x1122455u32); cpu.push(0x102c225); cpu.call(t); if cpu.returned_to != 0x102c225 { return; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x80u32); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102c21e; continue; }
  pc = 0x102c22e; continue;
 }
 0x102c22e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c257(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102c257); }
 let mut pc: u32 = 0x102c257;
 loop { match pc {
 0x102c257 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1121365u32); cpu.eax = v; }
  cpu.push(0x102c267); f_01027bb1(cpu); if cpu.returned_to != 0x102c267 { return; }
  { let v: u32 = 0x10e6910u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x10e3f60u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x10e3f64u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0xa0000u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x1121349u32)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x112134du32); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x280u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x10e3f60u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  if cpu.zf { pc = 0x102c2b7; continue; }
  pc = 0x102c2ae; continue;
 }
 0x102c2ae => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.cf { pc = 0x102c2e1; continue; }
  pc = 0x102c2b3; continue;
 }
 0x102c2b3 => {
  if cpu.zf { pc = 0x102c30b; continue; }
  pc = 0x102c2b5; continue;
 }
 0x102c2b5 => {
  pc = 0x102c335; continue;
 }
 0x102c2b7 => {
  { let v: u32 = 0x102b51cu32; cpu.w32(0x1122455u32, v); }
  { let v: u32 = 0x102b5c3u32; cpu.w32(0x1122459u32, v); }
  { let v: u32 = 0x102b618u32; cpu.w32(0x112245du32, v); }
  { let v: u32 = 0x102bfe5u32; cpu.w32(0x1122461u32, v); }
  pc = 0x102c35d; continue;
 }
 0x102c2e1 => {
  { let v: u32 = 0x102b676u32; cpu.w32(0x1122455u32, v); }
  { let v: u32 = 0x102b83cu32; cpu.w32(0x1122459u32, v); }
  { let v: u32 = 0x102b8e7u32; cpu.w32(0x112245du32, v); }
  { let v: u32 = 0x102c038u32; cpu.w32(0x1122461u32, v); }
  pc = 0x102c35d; continue;
 }
 0x102c30b => {
  { let v: u32 = 0x102b99bu32; cpu.w32(0x1122455u32, v); }
  { let v: u32 = 0x102bb61u32; cpu.w32(0x1122459u32, v); }
  { let v: u32 = 0x102bc0cu32; cpu.w32(0x112245du32, v); }
  { let v: u32 = 0x102c0dau32; cpu.w32(0x1122461u32, v); }
  pc = 0x102c35d; continue;
 }
 0x102c335 => {
  { let v: u32 = 0x102bcc0u32; cpu.w32(0x1122455u32, v); }
  { let v: u32 = 0x102be86u32; cpu.w32(0x1122459u32, v); }
  { let v: u32 = 0x102bf31u32; cpu.w32(0x112245du32, v); }
  { let v: u32 = 0x102c17cu32; cpu.w32(0x1122461u32, v); }
  pc = 0x102c35d; continue;
 }
 0x102c35d => {
  { let v: u32 = cpu.r32(0x112134du32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1121355u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x10e3f64u32), 0xfu8); }
  if !cpu.zf { pc = 0x102c38c; continue; }
  pc = 0x102c376; continue;
 }
 0x102c376 => {
  let t = cpu.r32(0x1122455u32); cpu.push(0x102c37c); cpu.call(t); if cpu.returned_to != 0x102c37c { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x80u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(0x10u32); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_sub(0x10u32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102c3ac; continue;
 }
 0x102c38c => {
  { let v: u16 = 0x10u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_sub(cpu.eax); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x112245du32); cpu.push(0x102c3a6); cpu.call(t); if cpu.returned_to != 0x102c3a6 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x80u32); cpu.ebx = v; }
  pc = 0x102c3ac; continue;
 }
 0x102c3ac => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x10u32); }
  if cpu.cf { pc = 0x102c407; continue; }
  pc = 0x102c3b2; continue;
 }
 0x102c3b2 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x1121911u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.sf { pc = 0x102c3d6; continue; }
  pc = 0x102c3c0; continue;
 }
 0x102c3c0 => {
  let t = cpu.r32(0x1122455u32); cpu.push(0x102c3c6); cpu.call(t); if cpu.returned_to != 0x102c3c6 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x80u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(0x10u32); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_sub(0x10u32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x102c3ac; continue;
 }
 0x102c3d6 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x102c3e2; continue; }
  pc = 0x102c3da; continue;
 }
 0x102c3da => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1122459u32); cpu.push(0x102c3e1); cpu.call(t); if cpu.returned_to != 0x102c3e1 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x102c3e2; continue;
 }
 0x102c3e2 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x1122461u32); cpu.push(0x102c3e9); cpu.call(t); if cpu.returned_to != 0x102c3e9 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0xfu32); cpu.eax = v; }
  if cpu.zf { pc = 0x102c3f7; continue; }
  pc = 0x102c3f1; continue;
 }
 0x102c3f1 => {
  let t = cpu.r32(0x112245du32); cpu.push(0x102c3f7); cpu.call(t); if cpu.returned_to != 0x102c3f7 { return; }
  pc = 0x102c3f7; continue;
 }
 0x102c3f7 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x80u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(0x10u32); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_sub(0x10u32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x102c3ac; continue;
 }
 0x102c407 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x102c414; continue; }
  pc = 0x102c40e; continue;
 }
 0x102c40e => {
  let t = cpu.r32(0x1122459u32); cpu.push(0x102c414); cpu.call(t); if cpu.returned_to != 0x102c414 { return; }
  pc = 0x102c414; continue;
 }
 0x102c414 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c416(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102c416); }
 let mut pc: u32 = 0x102c416;
 loop { match pc {
 0x102c416 => {
  { let v: u32 = cpu.r32(0x1121365u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1121369u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1121365u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1121369u32, v); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x102c434); f_01028d53(cpu); if cpu.returned_to != 0x102c434 { return; }
  { let v: u32 = 0x3dau32; cpu.edx = v; }
  pc = 0x102c439; continue;
 }
 0x102c439 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { cpu.poll(); pc = 0x102c439; continue; }
  pc = 0x102c43e; continue;
 }
 0x102c43e => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x102c43e; continue; }
  pc = 0x102c443; continue;
 }
 0x102c443 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c444(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102c444); }
 let mut pc: u32 = 0x102c444;
 loop { match pc {
 0x102c444 => {
  { let v: u32 = cpu.r32(0x1121365u32); cpu.eax = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  pc = 0x102c44e; continue;
 }
 0x102c44e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x102c455); f_01027bb1(cpu); if cpu.returned_to != 0x102c455 { return; }
  { let v: u32 = 0xa0000u32; cpu.edi = v; }
  { let v: u32 = 0x4000u32; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102c44e; continue; }
  pc = 0x102c468; continue;
 }
 0x102c468 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c469(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102c469); }
 let mut pc: u32 = 0x102c469;
 loop { match pc {
 0x102c469 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1121369u32, v); }
  { cpu.sub8(cpu.r8(0x1121348u32), 0x0u8); }
  if !cpu.zf { pc = 0x102c488; continue; }
  pc = 0x102c47c; continue;
 }
 0x102c47c => {
  { let v: u32 = 0x5u32; cpu.w32(0x1121365u32, v); }
  pc = 0x102c492; continue;
 }
 0x102c488 => {
  { let v: u32 = 0x4u32; cpu.w32(0x1121365u32, v); }
  pc = 0x102c492; continue;
 }
 0x102c492 => {
  cpu.push(0x102c497); f_0102c444(cpu); if cpu.returned_to != 0x102c497 { return; }
  cpu.push(0x102c49c); f_0102c416(cpu); if cpu.returned_to != 0x102c49c { return; }
  cpu.push(0x102c4a1); f_0102c444(cpu); if cpu.returned_to != 0x102c4a1 { return; }
  cpu.push(0x102c4a6); f_0102c416(cpu); if cpu.returned_to != 0x102c4a6 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c4a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102c4a7); }
 let mut pc: u32 = 0x102c4a7;
 loop { match pc {
 0x102c4a7 => {
  { let v: u32 = 0x40u32; cpu.w32(0x1121349u32, v); }
  { let v: u32 = 0x200u32; cpu.w32(0x1121351u32, v); }
  { let v: u32 = 0x20u32; cpu.w32(0x1121359u32, v); }
  { let v: u32 = 0x200u32; cpu.w32(0x10e4104u32, v); }
  { cpu.sub8(cpu.r8(0x1121348u32), 0x1u8); }
  if !cpu.zf { pc = 0x102c502; continue; }
  pc = 0x102c4d8; continue;
 }
 0x102c4d8 => {
  { let v: u32 = 0x28u32; cpu.w32(0x112134du32, v); }
  { let v: u32 = 0x140u32; cpu.w32(0x1121355u32, v); }
  { let v: u32 = 0x14u32; cpu.w32(0x112135du32, v); }
  { let v: u32 = 0x140u32; cpu.w32(0x10e4108u32, v); }
  pc = 0x102c52a; continue;
 }
 0x102c502 => {
  { let v: u32 = 0x30u32; cpu.w32(0x112134du32, v); }
  { let v: u32 = 0x180u32; cpu.w32(0x1121355u32, v); }
  { let v: u32 = 0x18u32; cpu.w32(0x112135du32, v); }
  { let v: u32 = 0x180u32; cpu.w32(0x10e4108u32, v); }
  pc = 0x102c52a; continue;
 }
 0x102c52a => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e40fcu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4100u32, v); }
  cpu.push(0x102c543); f_0102c469(cpu); if cpu.returned_to != 0x102c543 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c544(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102c544); }
 let mut pc: u32 = 0x102c544;
 loop { match pc {
 0x102c544 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1121349u32, v); }
  { let v: u32 = 0x280u32; cpu.w32(0x1121351u32, v); }
  { let v: u32 = 0x28u32; cpu.w32(0x1121359u32, v); }
  { let v: u32 = 0x280u32; cpu.w32(0x10e4104u32, v); }
  { cpu.sub8(cpu.r8(0x1121348u32), 0x1u8); }
  if !cpu.zf { pc = 0x102c59f; continue; }
  pc = 0x102c575; continue;
 }
 0x102c575 => {
  { let v: u32 = 0x0u32; cpu.w32(0x112134du32, v); }
  { let v: u32 = 0x190u32; cpu.w32(0x1121355u32, v); }
  { let v: u32 = 0x19u32; cpu.w32(0x112135du32, v); }
  { let v: u32 = 0x190u32; cpu.w32(0x10e4108u32, v); }
  pc = 0x102c5c7; continue;
 }
 0x102c59f => {
  { let v: u32 = 0x0u32; cpu.w32(0x112134du32, v); }
  { let v: u32 = 0x1e0u32; cpu.w32(0x1121355u32, v); }
  { let v: u32 = 0x1eu32; cpu.w32(0x112135du32, v); }
  { let v: u32 = 0x1e0u32; cpu.w32(0x10e4108u32, v); }
  pc = 0x102c5c7; continue;
 }
 0x102c5c7 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e40fcu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4100u32, v); }
  cpu.push(0x102c5e0); f_0102c469(cpu); if cpu.returned_to != 0x102c5e0 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c5e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102c5e1); }
 let mut pc: u32 = 0x102c5e1;
 loop { match pc {
 0x102c5e1 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x10e3e53u32) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11cccf0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x102c636; continue; }
  pc = 0x102c5f2; continue;
 }
 0x102c5f2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11cccf0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.lods(1, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.lods(1, false); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x3u32); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0xfffffffcu32; cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  pc = 0x102c616; continue;
 }
 0x102c616 => {
  { let v: u32 = 0x11d6898u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x102c61e; continue;
 }
 0x102c61e => {
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x3u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102c61e; continue; }
  pc = 0x102c624; continue;
 }
 0x102c624 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x4u32); }
  if !cpu.zf { cpu.poll(); pc = 0x102c616; continue; }
  pc = 0x102c62a; continue;
 }
 0x102c62a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11d6898u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x102c636; continue;
 }
 0x102c636 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c638(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c638); } r_0102c638(cpu, 0x102c638); }
pub fn f_0102c683(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c683); } r_0102c638(cpu, 0x102c683); }
fn r_0102c638(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102c638 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x10e3e53u32) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11cccf0u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x102c691; continue; }
  pc = 0x102c649; continue;
 }
 0x102c649 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11cccf0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.lods(1, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.lods(1, false); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x3u32); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0xfffffffcu32; cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0x11d6898u32; cpu.edi = v; }
  pc = 0x102c672; continue;
 }
 0x102c672 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x102c678; continue;
 }
 0x102c678 => {
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x3u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102c678; continue; }
  pc = 0x102c67e; continue;
 }
 0x102c67e => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x4u32); }
  pc = 0x102c683; continue;
 }
 0x102c683 => {
  if !cpu.zf { cpu.poll(); pc = 0x102c672; continue; }
  pc = 0x102c685; continue;
 }
 0x102c685 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11d6898u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x102c691; continue;
 }
 0x102c691 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c693(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c693); } r_0102c693(cpu, 0x102c693); }
pub fn f_0102c6ac(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c6ac); } r_0102c693(cpu, 0x102c6ac); }
pub fn f_0102c6bc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c6bc); } r_0102c693(cpu, 0x102c6bc); }
pub fn f_0102c6cc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c6cc); } r_0102c693(cpu, 0x102c6cc); }
pub fn f_0102c6f9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c6f9); } r_0102c693(cpu, 0x102c6f9); }
pub fn f_0102c719(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c719); } r_0102c693(cpu, 0x102c719); }
fn r_0102c693(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102c693 => {
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.edx >> 8) as u8), 2u32); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x280u32); cpu.ebx = v; }
  pc = 0x102c6a0; continue;
 }
 0x102c6a0 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.poll(); pc = cpu.r32(0x102c6acu32.wrapping_add(cpu.eax.wrapping_mul(2))); continue;
 }
 0x102c6ac => {
  { let v: u32 = 0xcc0102c6u32; cpu.esp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.cf = true; }
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.sbb32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x102c6bc; continue;
 }
 0x102c6bc => {
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102c6a0; continue; }
  pc = 0x102c6c7; continue;
 }
 0x102c6c7 => {
  pc = 0x102c753; continue;
 }
 0x102c6cc => {
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.ecx = v; }
  pc = 0x102c6cf; continue;
 }
 0x102c6cf => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c6d6; continue; }
  pc = 0x102c6d4; continue;
 }
 0x102c6d4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c6d6; continue;
 }
 0x102c6d6 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c6de; continue; }
  pc = 0x102c6dc; continue;
 }
 0x102c6dc => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c6de; continue;
 }
 0x102c6de => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c6e6; continue; }
  pc = 0x102c6e4; continue;
 }
 0x102c6e4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c6e6; continue;
 }
 0x102c6e6 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c6ee; continue; }
  pc = 0x102c6ec; continue;
 }
 0x102c6ec => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c6ee; continue;
 }
 0x102c6ee => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102c6cf; continue; }
  pc = 0x102c6f1; continue;
 }
 0x102c6f1 => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102c6a0; continue; }
  pc = 0x102c6f7; continue;
 }
 0x102c6f7 => {
  pc = 0x102c753; continue;
 }
 0x102c6f9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x10c0008u32; cpu.ebx = v; }
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.ecx = v; }
  pc = 0x102c702; continue;
 }
 0x102c702 => {
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102c702; continue; }
  pc = 0x102c710; continue;
 }
 0x102c710 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102c6a0; continue; }
  pc = 0x102c717; continue;
 }
 0x102c717 => {
  pc = 0x102c753; continue;
 }
 0x102c719 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x10c0008u32; cpu.ebx = v; }
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.ecx = v; }
  pc = 0x102c722; continue;
 }
 0x102c722 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c72a; continue; }
  pc = 0x102c727; continue;
 }
 0x102c727 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c72a; continue;
 }
 0x102c72a => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c733; continue; }
  pc = 0x102c730; continue;
 }
 0x102c730 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c733; continue;
 }
 0x102c733 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c73c; continue; }
  pc = 0x102c739; continue;
 }
 0x102c739 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c73c; continue;
 }
 0x102c73c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c745; continue; }
  pc = 0x102c742; continue;
 }
 0x102c742 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c745; continue;
 }
 0x102c745 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102c722; continue; }
  pc = 0x102c748; continue;
 }
 0x102c748 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102c6a0; continue; }
  pc = 0x102c753; continue;
 }
 0x102c753 => {
  { cpu.sub32(cpu.edi, 0xb0000u32); }
  if cpu.cf { pc = 0x102c770; continue; }
  pc = 0x102c75b; continue;
 }
 0x102c75b => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102c769); f_01027bb1(cpu); if cpu.returned_to != 0x102c769 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.sub32(cpu.edi, 0x10000u32); cpu.edi = v; }
  pc = 0x102c770; continue;
 }
 0x102c770 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c771(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c771); } r_0102c771(cpu, 0x102c771); }
pub fn f_0102c797(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c797); } r_0102c771(cpu, 0x102c797); }
pub fn f_0102c7a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c7a4); } r_0102c771(cpu, 0x102c7a4); }
pub fn f_0102c7ec(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c7ec); } r_0102c771(cpu, 0x102c7ec); }
pub fn f_0102c810(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c810); } r_0102c771(cpu, 0x102c810); }
fn r_0102c771(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102c771 => {
  { let v: u32 = 0x278u32; cpu.ecx = v; }
  { let v: u32 = 0x10c0008u32; cpu.ebx = v; }
  pc = 0x102c77b; continue;
 }
 0x102c77b => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.poll(); pc = cpu.r32(0x102c787u32.wrapping_add(cpu.eax.wrapping_mul(2))); continue;
 }
 0x102c797 => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ecx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102c77b; continue; }
  pc = 0x102c79f; continue;
 }
 0x102c79f => {
  pc = 0x102c862; continue;
 }
 0x102c7a4 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c7ab; continue; }
  pc = 0x102c7a9; continue;
 }
 0x102c7a9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c7ab; continue;
 }
 0x102c7ab => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c7b3; continue; }
  pc = 0x102c7b1; continue;
 }
 0x102c7b1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c7b3; continue;
 }
 0x102c7b3 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c7bb; continue; }
  pc = 0x102c7b9; continue;
 }
 0x102c7b9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c7bb; continue;
 }
 0x102c7bb => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c7c3; continue; }
  pc = 0x102c7c1; continue;
 }
 0x102c7c1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c7c3; continue;
 }
 0x102c7c3 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c7cb; continue; }
  pc = 0x102c7c9; continue;
 }
 0x102c7c9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c7cb; continue;
 }
 0x102c7cb => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c7d3; continue; }
  pc = 0x102c7d1; continue;
 }
 0x102c7d1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c7d3; continue;
 }
 0x102c7d3 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c7db; continue; }
  pc = 0x102c7d9; continue;
 }
 0x102c7d9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c7db; continue;
 }
 0x102c7db => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c7e3; continue; }
  pc = 0x102c7e1; continue;
 }
 0x102c7e1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c7e3; continue;
 }
 0x102c7e3 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ecx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102c77b; continue; }
  pc = 0x102c7ea; continue;
 }
 0x102c7ea => {
  pc = 0x102c862; continue;
 }
 0x102c7ec => {
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ecx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102c77b; continue; }
  pc = 0x102c80e; continue;
 }
 0x102c80e => {
  pc = 0x102c862; continue;
 }
 0x102c810 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c818; continue; }
  pc = 0x102c815; continue;
 }
 0x102c815 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c818; continue;
 }
 0x102c818 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c821; continue; }
  pc = 0x102c81e; continue;
 }
 0x102c81e => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c821; continue;
 }
 0x102c821 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c82a; continue; }
  pc = 0x102c827; continue;
 }
 0x102c827 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c82a; continue;
 }
 0x102c82a => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c833; continue; }
  pc = 0x102c830; continue;
 }
 0x102c830 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c833; continue;
 }
 0x102c833 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c83c; continue; }
  pc = 0x102c839; continue;
 }
 0x102c839 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c83c; continue;
 }
 0x102c83c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c845; continue; }
  pc = 0x102c842; continue;
 }
 0x102c842 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c845; continue;
 }
 0x102c845 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c84e; continue; }
  pc = 0x102c84b; continue;
 }
 0x102c84b => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c84e; continue;
 }
 0x102c84e => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c857; continue; }
  pc = 0x102c854; continue;
 }
 0x102c854 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c857; continue;
 }
 0x102c857 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ecx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102c77b; continue; }
  pc = 0x102c862; continue;
 }
 0x102c862 => {
  { cpu.sub32(cpu.edi, 0xb0000u32); }
  if cpu.cf { pc = 0x102c87f; continue; }
  pc = 0x102c86a; continue;
 }
 0x102c86a => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102c878); f_01027bb1(cpu); if cpu.returned_to != 0x102c878 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.sub32(cpu.edi, 0x10000u32); cpu.edi = v; }
  pc = 0x102c87f; continue;
 }
 0x102c87f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c787(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102c787); }
 let mut pc: u32 = 0x102c787;
 loop { match pc {
 0x102c787 => {
  { let a = cpu.edi; let b = cpu.eax; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = 0x2c7a401u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.esp.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = 0x2c81001u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.add32(cpu.r32(0xfef903a5u32.wrapping_add(cpu.ebp)), cpu.esp); cpu.w32(0xfef903a5u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(56437); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102c880(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c880); } r_0102c880(cpu, 0x102c880); }
pub fn f_0102c896(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c896); } r_0102c880(cpu, 0x102c896); }
pub fn f_0102c8a6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c8a6); } r_0102c880(cpu, 0x102c8a6); }
pub fn f_0102c8b5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c8b5); } r_0102c880(cpu, 0x102c8b5); }
pub fn f_0102c944(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c944); } r_0102c880(cpu, 0x102c944); }
pub fn f_0102c983(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102c983); } r_0102c880(cpu, 0x102c983); }
fn r_0102c880(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102c880 => {
  { let v: u32 = 0x270u32; cpu.ecx = v; }
  { let v: u32 = 0x10c0008u32; cpu.ebx = v; }
  pc = 0x102c88a; continue;
 }
 0x102c88a => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.poll(); pc = cpu.r32(0x102c896u32.wrapping_add(cpu.eax.wrapping_mul(2))); continue;
 }
 0x102c896 => {
  { cpu.cmps(1, 0); }
  cpu.unimplemented(0x102c897, "Enter");
  cpu.unimplemented(0x102c89b, "Enter");
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx | 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x3a5a5a5u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.esp); cpu.w32(0x3a5a5a5u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102c88a; continue; }
  pc = 0x102c8b0; continue;
 }
 0x102c8a6 => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ecx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102c88a; continue; }
  pc = 0x102c8b0; continue;
 }
 0x102c8b0 => {
  pc = 0x102ca1d; continue;
 }
 0x102c8b5 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c8bc; continue; }
  pc = 0x102c8ba; continue;
 }
 0x102c8ba => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c8bc; continue;
 }
 0x102c8bc => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c8c4; continue; }
  pc = 0x102c8c2; continue;
 }
 0x102c8c2 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c8c4; continue;
 }
 0x102c8c4 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c8cc; continue; }
  pc = 0x102c8ca; continue;
 }
 0x102c8ca => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c8cc; continue;
 }
 0x102c8cc => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c8d4; continue; }
  pc = 0x102c8d2; continue;
 }
 0x102c8d2 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c8d4; continue;
 }
 0x102c8d4 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c8dc; continue; }
  pc = 0x102c8da; continue;
 }
 0x102c8da => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c8dc; continue;
 }
 0x102c8dc => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c8e4; continue; }
  pc = 0x102c8e2; continue;
 }
 0x102c8e2 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c8e4; continue;
 }
 0x102c8e4 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c8ec; continue; }
  pc = 0x102c8ea; continue;
 }
 0x102c8ea => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c8ec; continue;
 }
 0x102c8ec => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c8f4; continue; }
  pc = 0x102c8f2; continue;
 }
 0x102c8f2 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c8f4; continue;
 }
 0x102c8f4 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c8fc; continue; }
  pc = 0x102c8fa; continue;
 }
 0x102c8fa => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c8fc; continue;
 }
 0x102c8fc => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c904; continue; }
  pc = 0x102c902; continue;
 }
 0x102c902 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c904; continue;
 }
 0x102c904 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c90c; continue; }
  pc = 0x102c90a; continue;
 }
 0x102c90a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c90c; continue;
 }
 0x102c90c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c914; continue; }
  pc = 0x102c912; continue;
 }
 0x102c912 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c914; continue;
 }
 0x102c914 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c91c; continue; }
  pc = 0x102c91a; continue;
 }
 0x102c91a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c91c; continue;
 }
 0x102c91c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c924; continue; }
  pc = 0x102c922; continue;
 }
 0x102c922 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c924; continue;
 }
 0x102c924 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c92c; continue; }
  pc = 0x102c92a; continue;
 }
 0x102c92a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c92c; continue;
 }
 0x102c92c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c934; continue; }
  pc = 0x102c932; continue;
 }
 0x102c932 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c934; continue;
 }
 0x102c934 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ecx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102c88a; continue; }
  pc = 0x102c93f; continue;
 }
 0x102c93f => {
  pc = 0x102ca1d; continue;
 }
 0x102c944 => {
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ecx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102c88a; continue; }
  pc = 0x102c97e; continue;
 }
 0x102c97e => {
  pc = 0x102ca1d; continue;
 }
 0x102c983 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c98b; continue; }
  pc = 0x102c988; continue;
 }
 0x102c988 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c98b; continue;
 }
 0x102c98b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c994; continue; }
  pc = 0x102c991; continue;
 }
 0x102c991 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c994; continue;
 }
 0x102c994 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c99d; continue; }
  pc = 0x102c99a; continue;
 }
 0x102c99a => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c99d; continue;
 }
 0x102c99d => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c9a6; continue; }
  pc = 0x102c9a3; continue;
 }
 0x102c9a3 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c9a6; continue;
 }
 0x102c9a6 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c9af; continue; }
  pc = 0x102c9ac; continue;
 }
 0x102c9ac => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c9af; continue;
 }
 0x102c9af => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c9b8; continue; }
  pc = 0x102c9b5; continue;
 }
 0x102c9b5 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c9b8; continue;
 }
 0x102c9b8 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c9c1; continue; }
  pc = 0x102c9be; continue;
 }
 0x102c9be => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c9c1; continue;
 }
 0x102c9c1 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c9ca; continue; }
  pc = 0x102c9c7; continue;
 }
 0x102c9c7 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c9ca; continue;
 }
 0x102c9ca => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c9d3; continue; }
  pc = 0x102c9d0; continue;
 }
 0x102c9d0 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c9d3; continue;
 }
 0x102c9d3 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c9dc; continue; }
  pc = 0x102c9d9; continue;
 }
 0x102c9d9 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c9dc; continue;
 }
 0x102c9dc => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c9e5; continue; }
  pc = 0x102c9e2; continue;
 }
 0x102c9e2 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c9e5; continue;
 }
 0x102c9e5 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c9ee; continue; }
  pc = 0x102c9eb; continue;
 }
 0x102c9eb => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c9ee; continue;
 }
 0x102c9ee => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102c9f7; continue; }
  pc = 0x102c9f4; continue;
 }
 0x102c9f4 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102c9f7; continue;
 }
 0x102c9f7 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ca00; continue; }
  pc = 0x102c9fd; continue;
 }
 0x102c9fd => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ca00; continue;
 }
 0x102ca00 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ca09; continue; }
  pc = 0x102ca06; continue;
 }
 0x102ca06 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ca09; continue;
 }
 0x102ca09 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ca12; continue; }
  pc = 0x102ca0f; continue;
 }
 0x102ca0f => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ca12; continue;
 }
 0x102ca12 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ecx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102c88a; continue; }
  pc = 0x102ca1d; continue;
 }
 0x102ca1d => {
  { cpu.sub32(cpu.edi, 0xb0000u32); }
  if cpu.cf { pc = 0x102ca3a; continue; }
  pc = 0x102ca25; continue;
 }
 0x102ca25 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102ca33); f_01027bb1(cpu); if cpu.returned_to != 0x102ca33 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.sub32(cpu.edi, 0x10000u32); cpu.edi = v; }
  pc = 0x102ca3a; continue;
 }
 0x102ca3a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102ca3b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102ca3b); } r_0102ca3b(cpu, 0x102ca3b); }
pub fn f_0102ca61(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102ca61); } r_0102ca3b(cpu, 0x102ca61); }
pub fn f_0102ca74(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102ca74); } r_0102ca3b(cpu, 0x102ca74); }
pub fn f_0102cb83(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102cb83); } r_0102ca3b(cpu, 0x102cb83); }
pub fn f_0102cbf2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102cbf2); } r_0102ca3b(cpu, 0x102cbf2); }
fn r_0102ca3b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102ca3b => {
  { let v: u32 = 0x260u32; cpu.ecx = v; }
  { let v: u32 = 0x10c0008u32; cpu.ebx = v; }
  pc = 0x102ca45; continue;
 }
 0x102ca45 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.poll(); pc = cpu.r32(0x102ca51u32.wrapping_add(cpu.eax.wrapping_mul(2))); continue;
 }
 0x102ca61 => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ecx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102ca45; continue; }
  pc = 0x102ca6f; continue;
 }
 0x102ca6f => {
  pc = 0x102cd1c; continue;
 }
 0x102ca74 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ca7b; continue; }
  pc = 0x102ca79; continue;
 }
 0x102ca79 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ca7b; continue;
 }
 0x102ca7b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ca83; continue; }
  pc = 0x102ca81; continue;
 }
 0x102ca81 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ca83; continue;
 }
 0x102ca83 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ca8b; continue; }
  pc = 0x102ca89; continue;
 }
 0x102ca89 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ca8b; continue;
 }
 0x102ca8b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ca93; continue; }
  pc = 0x102ca91; continue;
 }
 0x102ca91 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ca93; continue;
 }
 0x102ca93 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ca9b; continue; }
  pc = 0x102ca99; continue;
 }
 0x102ca99 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ca9b; continue;
 }
 0x102ca9b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102caa3; continue; }
  pc = 0x102caa1; continue;
 }
 0x102caa1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102caa3; continue;
 }
 0x102caa3 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102caab; continue; }
  pc = 0x102caa9; continue;
 }
 0x102caa9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102caab; continue;
 }
 0x102caab => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cab3; continue; }
  pc = 0x102cab1; continue;
 }
 0x102cab1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cab3; continue;
 }
 0x102cab3 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cabb; continue; }
  pc = 0x102cab9; continue;
 }
 0x102cab9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cabb; continue;
 }
 0x102cabb => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cac3; continue; }
  pc = 0x102cac1; continue;
 }
 0x102cac1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cac3; continue;
 }
 0x102cac3 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cacb; continue; }
  pc = 0x102cac9; continue;
 }
 0x102cac9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cacb; continue;
 }
 0x102cacb => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cad3; continue; }
  pc = 0x102cad1; continue;
 }
 0x102cad1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cad3; continue;
 }
 0x102cad3 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cadb; continue; }
  pc = 0x102cad9; continue;
 }
 0x102cad9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cadb; continue;
 }
 0x102cadb => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cae3; continue; }
  pc = 0x102cae1; continue;
 }
 0x102cae1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cae3; continue;
 }
 0x102cae3 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102caeb; continue; }
  pc = 0x102cae9; continue;
 }
 0x102cae9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102caeb; continue;
 }
 0x102caeb => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102caf3; continue; }
  pc = 0x102caf1; continue;
 }
 0x102caf1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102caf3; continue;
 }
 0x102caf3 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cafb; continue; }
  pc = 0x102caf9; continue;
 }
 0x102caf9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cafb; continue;
 }
 0x102cafb => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cb03; continue; }
  pc = 0x102cb01; continue;
 }
 0x102cb01 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cb03; continue;
 }
 0x102cb03 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cb0b; continue; }
  pc = 0x102cb09; continue;
 }
 0x102cb09 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cb0b; continue;
 }
 0x102cb0b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cb13; continue; }
  pc = 0x102cb11; continue;
 }
 0x102cb11 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cb13; continue;
 }
 0x102cb13 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cb1b; continue; }
  pc = 0x102cb19; continue;
 }
 0x102cb19 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cb1b; continue;
 }
 0x102cb1b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cb23; continue; }
  pc = 0x102cb21; continue;
 }
 0x102cb21 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cb23; continue;
 }
 0x102cb23 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cb2b; continue; }
  pc = 0x102cb29; continue;
 }
 0x102cb29 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cb2b; continue;
 }
 0x102cb2b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cb33; continue; }
  pc = 0x102cb31; continue;
 }
 0x102cb31 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cb33; continue;
 }
 0x102cb33 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cb3b; continue; }
  pc = 0x102cb39; continue;
 }
 0x102cb39 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cb3b; continue;
 }
 0x102cb3b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cb43; continue; }
  pc = 0x102cb41; continue;
 }
 0x102cb41 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cb43; continue;
 }
 0x102cb43 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cb4b; continue; }
  pc = 0x102cb49; continue;
 }
 0x102cb49 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cb4b; continue;
 }
 0x102cb4b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cb53; continue; }
  pc = 0x102cb51; continue;
 }
 0x102cb51 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cb53; continue;
 }
 0x102cb53 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cb5b; continue; }
  pc = 0x102cb59; continue;
 }
 0x102cb59 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cb5b; continue;
 }
 0x102cb5b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cb63; continue; }
  pc = 0x102cb61; continue;
 }
 0x102cb61 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cb63; continue;
 }
 0x102cb63 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cb6b; continue; }
  pc = 0x102cb69; continue;
 }
 0x102cb69 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cb6b; continue;
 }
 0x102cb6b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cb73; continue; }
  pc = 0x102cb71; continue;
 }
 0x102cb71 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cb73; continue;
 }
 0x102cb73 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ecx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102ca45; continue; }
  pc = 0x102cb7e; continue;
 }
 0x102cb7e => {
  pc = 0x102cd1c; continue;
 }
 0x102cb83 => {
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ecx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102ca45; continue; }
  pc = 0x102cbed; continue;
 }
 0x102cbed => {
  pc = 0x102cd1c; continue;
 }
 0x102cbf2 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cbfa; continue; }
  pc = 0x102cbf7; continue;
 }
 0x102cbf7 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cbfa; continue;
 }
 0x102cbfa => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc03; continue; }
  pc = 0x102cc00; continue;
 }
 0x102cc00 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc03; continue;
 }
 0x102cc03 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc0c; continue; }
  pc = 0x102cc09; continue;
 }
 0x102cc09 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc0c; continue;
 }
 0x102cc0c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc15; continue; }
  pc = 0x102cc12; continue;
 }
 0x102cc12 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc15; continue;
 }
 0x102cc15 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc1e; continue; }
  pc = 0x102cc1b; continue;
 }
 0x102cc1b => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc1e; continue;
 }
 0x102cc1e => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc27; continue; }
  pc = 0x102cc24; continue;
 }
 0x102cc24 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc27; continue;
 }
 0x102cc27 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc30; continue; }
  pc = 0x102cc2d; continue;
 }
 0x102cc2d => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc30; continue;
 }
 0x102cc30 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc39; continue; }
  pc = 0x102cc36; continue;
 }
 0x102cc36 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc39; continue;
 }
 0x102cc39 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc42; continue; }
  pc = 0x102cc3f; continue;
 }
 0x102cc3f => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc42; continue;
 }
 0x102cc42 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc4b; continue; }
  pc = 0x102cc48; continue;
 }
 0x102cc48 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc4b; continue;
 }
 0x102cc4b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc54; continue; }
  pc = 0x102cc51; continue;
 }
 0x102cc51 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc54; continue;
 }
 0x102cc54 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc5d; continue; }
  pc = 0x102cc5a; continue;
 }
 0x102cc5a => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc5d; continue;
 }
 0x102cc5d => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc66; continue; }
  pc = 0x102cc63; continue;
 }
 0x102cc63 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc66; continue;
 }
 0x102cc66 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc6f; continue; }
  pc = 0x102cc6c; continue;
 }
 0x102cc6c => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc6f; continue;
 }
 0x102cc6f => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc78; continue; }
  pc = 0x102cc75; continue;
 }
 0x102cc75 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc78; continue;
 }
 0x102cc78 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc81; continue; }
  pc = 0x102cc7e; continue;
 }
 0x102cc7e => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc81; continue;
 }
 0x102cc81 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc8a; continue; }
  pc = 0x102cc87; continue;
 }
 0x102cc87 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc8a; continue;
 }
 0x102cc8a => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc93; continue; }
  pc = 0x102cc90; continue;
 }
 0x102cc90 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc93; continue;
 }
 0x102cc93 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cc9c; continue; }
  pc = 0x102cc99; continue;
 }
 0x102cc99 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cc9c; continue;
 }
 0x102cc9c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cca5; continue; }
  pc = 0x102cca2; continue;
 }
 0x102cca2 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cca5; continue;
 }
 0x102cca5 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ccae; continue; }
  pc = 0x102ccab; continue;
 }
 0x102ccab => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ccae; continue;
 }
 0x102ccae => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ccb7; continue; }
  pc = 0x102ccb4; continue;
 }
 0x102ccb4 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ccb7; continue;
 }
 0x102ccb7 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ccc0; continue; }
  pc = 0x102ccbd; continue;
 }
 0x102ccbd => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ccc0; continue;
 }
 0x102ccc0 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ccc9; continue; }
  pc = 0x102ccc6; continue;
 }
 0x102ccc6 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ccc9; continue;
 }
 0x102ccc9 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ccd2; continue; }
  pc = 0x102cccf; continue;
 }
 0x102cccf => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ccd2; continue;
 }
 0x102ccd2 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ccdb; continue; }
  pc = 0x102ccd8; continue;
 }
 0x102ccd8 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ccdb; continue;
 }
 0x102ccdb => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cce4; continue; }
  pc = 0x102cce1; continue;
 }
 0x102cce1 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cce4; continue;
 }
 0x102cce4 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cced; continue; }
  pc = 0x102ccea; continue;
 }
 0x102ccea => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cced; continue;
 }
 0x102cced => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ccf6; continue; }
  pc = 0x102ccf3; continue;
 }
 0x102ccf3 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ccf6; continue;
 }
 0x102ccf6 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ccff; continue; }
  pc = 0x102ccfc; continue;
 }
 0x102ccfc => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ccff; continue;
 }
 0x102ccff => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cd08; continue; }
  pc = 0x102cd05; continue;
 }
 0x102cd05 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cd08; continue;
 }
 0x102cd08 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cd11; continue; }
  pc = 0x102cd0e; continue;
 }
 0x102cd0e => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cd11; continue;
 }
 0x102cd11 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ecx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102ca45; continue; }
  pc = 0x102cd1c; continue;
 }
 0x102cd1c => {
  { cpu.sub32(cpu.edi, 0xb0000u32); }
  if cpu.cf { pc = 0x102cd39; continue; }
  pc = 0x102cd24; continue;
 }
 0x102cd24 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102cd32); f_01027bb1(cpu); if cpu.returned_to != 0x102cd32 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.sub32(cpu.edi, 0x10000u32); cpu.edi = v; }
  pc = 0x102cd39; continue;
 }
 0x102cd39 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102ca51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102ca51); }
 let mut pc: u32 = 0x102ca51;
 loop { match pc {
 0x102ca51 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(258); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102cd3a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102cd3a); } r_0102cd3a(cpu, 0x102cd3a); }
pub fn f_0102cd51(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102cd51); } r_0102cd3a(cpu, 0x102cd51); }
pub fn f_0102cd61(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102cd61); } r_0102cd3a(cpu, 0x102cd61); }
pub fn f_0102cd71(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102cd71); } r_0102cd3a(cpu, 0x102cd71); }
pub fn f_0102cd89(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102cd89); } r_0102cd3a(cpu, 0x102cd89); }
pub fn f_0102cda3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102cda3); } r_0102cd3a(cpu, 0x102cda3); }
fn r_0102cd3a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102cd3a => {
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x280u32); cpu.ebx = v; }
  pc = 0x102cd44; continue;
 }
 0x102cd44 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.poll(); pc = cpu.r32(0x102cd51u32.wrapping_add(cpu.eax.wrapping_mul(2))); continue;
 }
 0x102cd51 => {
  { cpu.popad(); }
  { cpu.interrupt(0x2); }
  { let v: u32 = cpu.r32(0xffffffcdu32.wrapping_add(cpu.ecx)).wrapping_add(cpu.esi); cpu.w32(0xffffffcdu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xf0102cdu32, v); }
  { let v: u8 = 0xceu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102cd44; continue; }
  pc = 0x102cd6f; continue;
 }
 0x102cd61 => {
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102cd44; continue; }
  pc = 0x102cd6f; continue;
 }
 0x102cd6f => {
  pc = 0x102cdc1; continue;
 }
 0x102cd71 => {
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.ecx = v; }
  pc = 0x102cd74; continue;
 }
 0x102cd74 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cd7b; continue; }
  pc = 0x102cd79; continue;
 }
 0x102cd79 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cd7b; continue;
 }
 0x102cd7b => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102cd74; continue; }
  pc = 0x102cd7e; continue;
 }
 0x102cd7e => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102cd44; continue; }
  pc = 0x102cd87; continue;
 }
 0x102cd87 => {
  pc = 0x102cdc1; continue;
 }
 0x102cd89 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x10c0008u32; cpu.ebx = v; }
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.ecx = v; }
  pc = 0x102cd92; continue;
 }
 0x102cd92 => {
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102cd92; continue; }
  pc = 0x102cd97; continue;
 }
 0x102cd97 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102cd44; continue; }
  pc = 0x102cda1; continue;
 }
 0x102cda1 => {
  pc = 0x102cdc1; continue;
 }
 0x102cda3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x10c0008u32; cpu.ebx = v; }
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.ecx = v; }
  pc = 0x102cdac; continue;
 }
 0x102cdac => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cdb4; continue; }
  pc = 0x102cdb1; continue;
 }
 0x102cdb1 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cdb4; continue;
 }
 0x102cdb4 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102cdac; continue; }
  pc = 0x102cdb7; continue;
 }
 0x102cdb7 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102cd44; continue; }
  pc = 0x102cdc1; continue;
 }
 0x102cdc1 => {
  { cpu.sub32(cpu.edi, 0xb0000u32); }
  if cpu.cf { pc = 0x102cdde; continue; }
  pc = 0x102cdc9; continue;
 }
 0x102cdc9 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102cdd7); f_01027bb1(cpu); if cpu.returned_to != 0x102cdd7 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.sub32(cpu.edi, 0x10000u32); cpu.edi = v; }
  pc = 0x102cdde; continue;
 }
 0x102cdde => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102cddf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102cddf); }
 let mut pc: u32 = 0x102cddf;
 loop { match pc {
 0x102cddf => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x280u32); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.poll(); pc = cpu.r32(0x102cdf6u32.wrapping_add(cpu.eax.wrapping_mul(2))); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102cdf6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102cdf6); } r_0102cdf6(cpu, 0x102cdf6); }
pub fn f_0102ce06(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102ce06); } r_0102cdf6(cpu, 0x102ce06); }
pub fn f_0102ce23(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102ce23); } r_0102cdf6(cpu, 0x102ce23); }
pub fn f_0102ce44(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102ce44); } r_0102cdf6(cpu, 0x102ce44); }
pub fn f_0102ce69(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102ce69); } r_0102cdf6(cpu, 0x102ce69); }
fn r_0102cdf6(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102cdf6 => {
  { cpu.push(cpu.segments[0] as u32); }
  cpu.unimplemented(0x102cdf7, "Into");
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & cpu.esi; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  cpu.unimplemented(0x102cdff, "Into");
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.esi, 0xb60f0102u32); cpu.ecx = v; }
  cpu.unimplemented(0x102ce08, "Into");
  pc = 0x102ce09; continue;
 }
 0x102ce06 => {
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.ecx = v; }
  pc = 0x102ce09; continue;
 }
 0x102ce09 => {
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.inc16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x102ce1f; continue; }
  pc = 0x102ce10; continue;
 }
 0x102ce10 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102ce1e); f_01027bb1(cpu); if cpu.returned_to != 0x102ce1e { return; }
  { cpu.popad(); }
  pc = 0x102ce1f; continue;
 }
 0x102ce1f => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102ce09; continue; }
  pc = 0x102ce21; continue;
 }
 0x102ce21 => {
  pc = 0x102ce90; continue;
 }
 0x102ce23 => {
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.ecx = v; }
  pc = 0x102ce26; continue;
 }
 0x102ce26 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ce2d; continue; }
  pc = 0x102ce2b; continue;
 }
 0x102ce2b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ce2d; continue;
 }
 0x102ce2d => {
  { let v: u16 = cpu.inc16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x102ce40; continue; }
  pc = 0x102ce31; continue;
 }
 0x102ce31 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102ce3f); f_01027bb1(cpu); if cpu.returned_to != 0x102ce3f { return; }
  { cpu.popad(); }
  pc = 0x102ce40; continue;
 }
 0x102ce40 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102ce26; continue; }
  pc = 0x102ce42; continue;
 }
 0x102ce42 => {
  pc = 0x102ce90; continue;
 }
 0x102ce44 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x10c0008u32; cpu.ebx = v; }
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.ecx = v; }
  pc = 0x102ce4d; continue;
 }
 0x102ce4d => {
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.inc16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x102ce64; continue; }
  pc = 0x102ce55; continue;
 }
 0x102ce55 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102ce63); f_01027bb1(cpu); if cpu.returned_to != 0x102ce63 { return; }
  { cpu.popad(); }
  pc = 0x102ce64; continue;
 }
 0x102ce64 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102ce4d; continue; }
  pc = 0x102ce66; continue;
 }
 0x102ce66 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x102ce90; continue;
 }
 0x102ce69 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x10c0008u32; cpu.ebx = v; }
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.ecx = v; }
  pc = 0x102ce72; continue;
 }
 0x102ce72 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102ce7a; continue; }
  pc = 0x102ce77; continue;
 }
 0x102ce77 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102ce7a; continue;
 }
 0x102ce7a => {
  { let v: u16 = cpu.inc16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x102ce8d; continue; }
  pc = 0x102ce7e; continue;
 }
 0x102ce7e => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102ce8c); f_01027bb1(cpu); if cpu.returned_to != 0x102ce8c { return; }
  { cpu.popad(); }
  pc = 0x102ce8d; continue;
 }
 0x102ce8d => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102ce72; continue; }
  pc = 0x102ce8f; continue;
 }
 0x102ce8f => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x102ce90; continue;
 }
 0x102ce90 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xb0000u32); }
  if cpu.cf { pc = 0x102ceaf; continue; }
  pc = 0x102ce9a; continue;
 }
 0x102ce9a => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102cea8); f_01027bb1(cpu); if cpu.returned_to != 0x102cea8 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.edi.wrapping_sub(0x10000u32); cpu.edi = v; }
  pc = 0x102ceaf; continue;
 }
 0x102ceaf => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102ceb3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102ceb3); }
 let mut pc: u32 = 0x102ceb3;
 loop { match pc {
 0x102ceb3 => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x280u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x102cec4; continue;
 }
 0x102cec4 => {
  { let v: u32 = ((cpu.edx >> 8) as u8) as u32; cpu.ecx = v; }
  pc = 0x102cec7; continue;
 }
 0x102cec7 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cece; continue; }
  pc = 0x102cecc; continue;
 }
 0x102cecc => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cece; continue;
 }
 0x102cece => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102cec7; continue; }
  pc = 0x102ced1; continue;
 }
 0x102ced1 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102cec4; continue; }
  pc = 0x102ced9; continue;
 }
 0x102ced9 => {
  { cpu.sub32(cpu.edi, 0xb0000u32); }
  if cpu.cf { pc = 0x102cef6; continue; }
  pc = 0x102cee1; continue;
 }
 0x102cee1 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102ceef); f_01027bb1(cpu); if cpu.returned_to != 0x102ceef { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.sub32(cpu.edi, 0x10000u32); cpu.edi = v; }
  pc = 0x102cef6; continue;
 }
 0x102cef6 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102cef8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102cef8); }
 let mut pc: u32 = 0x102cef8;
 loop { match pc {
 0x102cef8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = (cpu.ebx as u8) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x280u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x102cf07; continue;
 }
 0x102cf07 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102cf0e; continue; }
  pc = 0x102cf0c; continue;
 }
 0x102cf0c => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102cf0e; continue;
 }
 0x102cf0e => {
  { let v: u16 = cpu.inc16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x102cf21; continue; }
  pc = 0x102cf12; continue;
 }
 0x102cf12 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102cf20); f_01027bb1(cpu); if cpu.returned_to != 0x102cf20 { return; }
  { cpu.popad(); }
  pc = 0x102cf21; continue;
 }
 0x102cf21 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102cf07; continue; }
  pc = 0x102cf23; continue;
 }
 0x102cf23 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xb0000u32); }
  if cpu.cf { pc = 0x102cf42; continue; }
  pc = 0x102cf2d; continue;
 }
 0x102cf2d => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102cf3b); f_01027bb1(cpu); if cpu.returned_to != 0x102cf3b { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.edi.wrapping_sub(0x10000u32); cpu.edi = v; }
  pc = 0x102cf42; continue;
 }
 0x102cf42 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102cf46(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102cf46); } r_0102cf46(cpu, 0x102cf46); }
pub fn f_0102cf4a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102cf4a); } r_0102cf46(cpu, 0x102cf4a); }
pub fn f_0102cf4e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102cf4e); } r_0102cf46(cpu, 0x102cf4e); }
pub fn f_0102cf52(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102cf52); } r_0102cf46(cpu, 0x102cf52); }
pub fn f_0102cf56(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102cf56); } r_0102cf46(cpu, 0x102cf56); }
fn r_0102cf46(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102cf46 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x102cf4a; continue;
 }
 0x102cf4a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x102cf4e; continue;
 }
 0x102cf4e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x102cf52; continue;
 }
 0x102cf52 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x102cf56; continue;
 }
 0x102cf56 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  f_0102cf5a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102cf5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102cf5a); }
 let mut pc: u32 = 0x102cf5a;
 loop { match pc {
 0x102cf5a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x102cf6c); f_010338ed(cpu); if cpu.returned_to != 0x102cf6c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x102cf46u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x102cf4au32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x102cf56u32, v); }
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x118cd66u32)); }
  if !cpu.cf { pc = 0x102d413; continue; }
  pc = 0x102cf91; continue;
 }
 0x102cf91 => {
  { let v: u32 = cpu.r32(0x118cd68u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.lods(1, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x102cf52u32, v); }
  { cpu.lods(1, false); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x102cf4eu32, v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x102cf52u32)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x3u32); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0xfffffffcu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x102cf4au32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x102d413; continue; }
  pc = 0x102cfc8; continue;
 }
 0x102cfc8 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x102cf52u32)); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102d413; continue; }
  pc = 0x102cfd4; continue;
 }
 0x102cfd4 => {
  { let v: u32 = cpu.r32(0x102cf46u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x102d413; continue; }
  pc = 0x102cfe5; continue;
 }
 0x102cfe5 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x102cf4eu32)); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102d413; continue; }
  pc = 0x102cff1; continue;
 }
 0x102cff1 => {
  { let v: u32 = cpu.r32(0x102cf4au32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x10e4100u32)); cpu.eax = v; }
  if !cpu.sf { pc = 0x102d020; continue; }
  pc = 0x102cffe; continue;
 }
 0x102cffe => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x102cf52u32), cpu.eax); cpu.w32(0x102cf52u32, v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102d413; continue; }
  pc = 0x102d00c; continue;
 }
 0x102d00c => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.r32(0x102cf4eu32); cpu.mul32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e4100u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x102cf4au32, v); }
  pc = 0x102d020; continue;
 }
 0x102d020 => {
  { let v: u32 = cpu.r32(0x102cf4au32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x102cf52u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4108u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102d04f; continue; }
  pc = 0x102d033; continue;
 }
 0x102d033 => {
  { let v: u32 = cpu.r32(0x10e4108u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x102cf4au32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x102cf52u32, v); }
  if cpu.sf { pc = 0x102d413; continue; }
  pc = 0x102d049; continue;
 }
 0x102d049 => {
  if cpu.zf { pc = 0x102d413; continue; }
  pc = 0x102d04f; continue;
 }
 0x102d04f => {
  { let v: u32 = cpu.r32(0x102cf46u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e40fcu32)); }
  if cpu.sf { pc = 0x102d29c; continue; }
  pc = 0x102d060; continue;
 }
 0x102d060 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x102cf4eu32)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x10e4104u32)); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x102d2b7; continue; }
  pc = 0x102d072; continue;
 }
 0x102d072 => {
  { cpu.sub8(cpu.r8(0x10ef324u32), 0x1u8); }
  if cpu.zf { pc = 0x102d2c2; continue; }
  pc = 0x102d07f; continue;
 }
 0x102d07f => {
  { let v: u32 = cpu.r32(0x1121349u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x102cf46u32).wrapping_add(cpu.eax); cpu.w32(0x102cf46u32, v); }
  { let v: u32 = cpu.r32(0x112134du32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x102cf4au32).wrapping_add(cpu.eax); cpu.w32(0x102cf4au32, v); }
  { let v: u32 = cpu.r32(0x102cf4au32); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, 0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x102cf46u32)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1121365u32)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x102d0b8); f_01027bb1(cpu); if cpu.returned_to != 0x102d0b8 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0xa0000u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x102cf4eu32), 0x8u32); }
  if cpu.zf { pc = 0x102d158; continue; }
  pc = 0x102d0d2; continue;
 }
 0x102d0d2 => {
  { cpu.sub32(cpu.r32(0x102cf4eu32), 0x10u32); }
  if cpu.zf { pc = 0x102d1c4; continue; }
  pc = 0x102d0df; continue;
 }
 0x102d0df => {
  { cpu.sub32(cpu.r32(0x102cf4eu32), 0x20u32); }
  if cpu.zf { pc = 0x102d230; continue; }
  pc = 0x102d0ec; continue;
 }
 0x102d0ec => {
  { let v: u32 = cpu.r32(0x102cf4au32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x1121af1u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x102d128; continue; }
  pc = 0x102d0fc; continue;
 }
 0x102d0fc => {
  { let v: u32 = cpu.r32(0x102cf52u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x102d148; continue; }
  pc = 0x102d106; continue;
 }
 0x102d106 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x102d114); f_0102c693(cpu); if cpu.returned_to != 0x102d114 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x102cf4au32).wrapping_add(cpu.eax); cpu.w32(0x102cf4au32, v); }
  { let v: u32 = cpu.sub32(cpu.r32(0x102cf52u32), cpu.eax); cpu.w32(0x102cf52u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d0ec; continue; }
  pc = 0x102d123; continue;
 }
 0x102d123 => {
  pc = 0x102d413; continue;
 }
 0x102d128 => {
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  cpu.push(0x102d135); f_0102cddf(cpu); if cpu.returned_to != 0x102d135 { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x102cf4au32)); cpu.w32(0x102cf4au32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x102cf52u32)); cpu.w32(0x102cf52u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d0ec; continue; }
  pc = 0x102d143; continue;
 }
 0x102d143 => {
  pc = 0x102d413; continue;
 }
 0x102d148 => {
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  cpu.push(0x102d153); f_0102c693(cpu); if cpu.returned_to != 0x102d153 { return; }
  pc = 0x102d413; continue;
 }
 0x102d158 => {
  { let v: u32 = cpu.r32(0x102cf4au32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x1121af1u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x102d194; continue; }
  pc = 0x102d168; continue;
 }
 0x102d168 => {
  { let v: u32 = cpu.r32(0x102cf52u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x102d1b4; continue; }
  pc = 0x102d172; continue;
 }
 0x102d172 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x102d180); f_0102c771(cpu); if cpu.returned_to != 0x102d180 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x102cf4au32).wrapping_add(cpu.eax); cpu.w32(0x102cf4au32, v); }
  { let v: u32 = cpu.sub32(cpu.r32(0x102cf52u32), cpu.eax); cpu.w32(0x102cf52u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d158; continue; }
  pc = 0x102d18f; continue;
 }
 0x102d18f => {
  pc = 0x102d413; continue;
 }
 0x102d194 => {
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  cpu.push(0x102d1a1); f_0102cddf(cpu); if cpu.returned_to != 0x102d1a1 { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x102cf4au32)); cpu.w32(0x102cf4au32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x102cf52u32)); cpu.w32(0x102cf52u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d158; continue; }
  pc = 0x102d1af; continue;
 }
 0x102d1af => {
  pc = 0x102d413; continue;
 }
 0x102d1b4 => {
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  cpu.push(0x102d1bf); f_0102c771(cpu); if cpu.returned_to != 0x102d1bf { return; }
  pc = 0x102d413; continue;
 }
 0x102d1c4 => {
  { let v: u32 = cpu.r32(0x102cf4au32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x1121af1u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x102d200; continue; }
  pc = 0x102d1d4; continue;
 }
 0x102d1d4 => {
  { let v: u32 = cpu.r32(0x102cf52u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x102d220; continue; }
  pc = 0x102d1de; continue;
 }
 0x102d1de => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x102d1ec); f_0102c880(cpu); if cpu.returned_to != 0x102d1ec { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x102cf4au32).wrapping_add(cpu.eax); cpu.w32(0x102cf4au32, v); }
  { let v: u32 = cpu.sub32(cpu.r32(0x102cf52u32), cpu.eax); cpu.w32(0x102cf52u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d1c4; continue; }
  pc = 0x102d1fb; continue;
 }
 0x102d1fb => {
  pc = 0x102d413; continue;
 }
 0x102d200 => {
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  cpu.push(0x102d20d); f_0102cddf(cpu); if cpu.returned_to != 0x102d20d { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x102cf4au32)); cpu.w32(0x102cf4au32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x102cf52u32)); cpu.w32(0x102cf52u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d1c4; continue; }
  pc = 0x102d21b; continue;
 }
 0x102d21b => {
  pc = 0x102d413; continue;
 }
 0x102d220 => {
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  cpu.push(0x102d22b); f_0102c880(cpu); if cpu.returned_to != 0x102d22b { return; }
  pc = 0x102d413; continue;
 }
 0x102d230 => {
  { let v: u32 = cpu.r32(0x102cf4au32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x1121af1u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x102d26c; continue; }
  pc = 0x102d240; continue;
 }
 0x102d240 => {
  { let v: u32 = cpu.r32(0x102cf52u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x102d28c; continue; }
  pc = 0x102d24a; continue;
 }
 0x102d24a => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x102d258); f_0102ca3b(cpu); if cpu.returned_to != 0x102d258 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x102cf4au32).wrapping_add(cpu.eax); cpu.w32(0x102cf4au32, v); }
  { let v: u32 = cpu.sub32(cpu.r32(0x102cf52u32), cpu.eax); cpu.w32(0x102cf52u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d230; continue; }
  pc = 0x102d267; continue;
 }
 0x102d267 => {
  pc = 0x102d413; continue;
 }
 0x102d26c => {
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  cpu.push(0x102d279); f_0102cddf(cpu); if cpu.returned_to != 0x102d279 { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x102cf4au32)); cpu.w32(0x102cf4au32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x102cf52u32)); cpu.w32(0x102cf52u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d230; continue; }
  pc = 0x102d287; continue;
 }
 0x102d287 => {
  pc = 0x102d413; continue;
 }
 0x102d28c => {
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  cpu.push(0x102d297); f_0102ca3b(cpu); if cpu.returned_to != 0x102d297 { return; }
  pc = 0x102d413; continue;
 }
 0x102d29c => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x102cf56u32, v); }
  { let v: u32 = cpu.r32(0x102cf4eu32).wrapping_sub(cpu.eax); cpu.w32(0x102cf4eu32, v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x102cf46u32, v); }
  pc = 0x102d2c2; continue;
 }
 0x102d2b7 => {
  { let v: u32 = cpu.eax; cpu.w32(0x102cf56u32, v); }
  { let v: u32 = cpu.r32(0x102cf4eu32).wrapping_sub(cpu.eax); cpu.w32(0x102cf4eu32, v); }
  pc = 0x102d2c2; continue;
 }
 0x102d2c2 => {
  { let v: u32 = cpu.r32(0x1121349u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x102cf46u32).wrapping_add(cpu.eax); cpu.w32(0x102cf46u32, v); }
  { let v: u32 = cpu.r32(0x112134du32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x102cf4au32).wrapping_add(cpu.eax); cpu.w32(0x102cf4au32, v); }
  { let v: u32 = cpu.r32(0x102cf4au32); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, 0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x102cf46u32)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1121365u32)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x102d2fb); f_01027bb1(cpu); if cpu.returned_to != 0x102d2fb { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0xa0000u32); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x10ef324u32), 0x1u8); }
  if cpu.zf { pc = 0x102d38d; continue; }
  pc = 0x102d311; continue;
 }
 0x102d311 => {
  { let v: u32 = cpu.r32(0x102cf4au32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x1121af1u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x102d353; continue; }
  pc = 0x102d321; continue;
 }
 0x102d321 => {
  { let v: u32 = cpu.r32(0x102cf52u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x102d377; continue; }
  pc = 0x102d32b; continue;
 }
 0x102d32b => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x102cf56u32); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x102d33f); f_0102cd3a(cpu); if cpu.returned_to != 0x102d33f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x102cf4au32).wrapping_add(cpu.eax); cpu.w32(0x102cf4au32, v); }
  { let v: u32 = cpu.sub32(cpu.r32(0x102cf52u32), cpu.eax); cpu.w32(0x102cf52u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d311; continue; }
  pc = 0x102d34e; continue;
 }
 0x102d34e => {
  pc = 0x102d413; continue;
 }
 0x102d353 => {
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x102cf56u32); cpu.ecx = v; }
  cpu.push(0x102d364); f_0102cddf(cpu); if cpu.returned_to != 0x102d364 { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x102cf4au32)); cpu.w32(0x102cf4au32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x102cf52u32)); cpu.w32(0x102cf52u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d311; continue; }
  pc = 0x102d372; continue;
 }
 0x102d372 => {
  pc = 0x102d413; continue;
 }
 0x102d377 => {
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x102cf56u32); cpu.ecx = v; }
  cpu.push(0x102d388); f_0102cd3a(cpu); if cpu.returned_to != 0x102d388 { return; }
  pc = 0x102d413; continue;
 }
 0x102d38d => {
  { let v: u32 = cpu.r32(0x102cf4au32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x1121af1u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x102d3d3; continue; }
  pc = 0x102d39d; continue;
 }
 0x102d39d => {
  { let v: u32 = cpu.r32(0x102cf52u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x102d3fb; continue; }
  pc = 0x102d3a7; continue;
 }
 0x102d3a7 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x102cf56u32); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x102d3bb); f_0102ceb3(cpu); if cpu.returned_to != 0x102d3bb { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x102cf4au32).wrapping_add(cpu.eax); cpu.w32(0x102cf4au32, v); }
  { let v: u32 = cpu.sub32(cpu.r32(0x102cf52u32), cpu.eax); cpu.w32(0x102cf52u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d38d; continue; }
  pc = 0x102d3ca; continue;
 }
 0x102d3ca => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef324u32, v); }
  pc = 0x102d413; continue;
 }
 0x102d3d3 => {
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x102cf56u32); cpu.ecx = v; }
  cpu.push(0x102d3e4); f_0102cef8(cpu); if cpu.returned_to != 0x102d3e4 { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x102cf4au32)); cpu.w32(0x102cf4au32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x102cf52u32)); cpu.w32(0x102cf52u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d38d; continue; }
  pc = 0x102d3f2; continue;
 }
 0x102d3f2 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef324u32, v); }
  pc = 0x102d413; continue;
 }
 0x102d3fb => {
  { let v: u32 = cpu.r32(0x102cf4eu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x102cf56u32); cpu.ecx = v; }
  cpu.push(0x102d40c); f_0102ceb3(cpu); if cpu.returned_to != 0x102d40c { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef324u32, v); }
  pc = 0x102d413; continue;
 }
 0x102d413 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d415(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102d415); }
 let mut pc: u32 = 0x102d415;
 loop { match pc {
 0x102d415 => {
  f_0102cf5a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d41a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102d41a); }
 let mut pc: u32 = 0x102d41a;
 loop { match pc {
 0x102d41a => {
  f_0102cf5a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d41f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102d41f); }
 let mut pc: u32 = 0x102d41f;
 loop { match pc {
 0x102d41f => {
  f_0102cf5a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d424(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102d424); }
 let mut pc: u32 = 0x102d424;
 loop { match pc {
 0x102d424 => {
  { let v: u8 = cpu.r8(0x10ef324u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef324u32, v); }
  cpu.push(0x102d437); f_0102cf5a(cpu); if cpu.returned_to != 0x102d437 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10ef324u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d43f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102d43f); }
 let mut pc: u32 = 0x102d43f;
 loop { match pc {
 0x102d43f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.push(0x102d447); f_0102d449(cpu); if cpu.returned_to != 0x102d447 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d449(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102d449); }
 let mut pc: u32 = 0x102d449;
 loop { match pc {
 0x102d449 => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102d465; continue; }
  pc = 0x102d464; continue;
 }
 0x102d464 => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x102d465; continue;
 }
 0x102d465 => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102d46b; continue; }
  pc = 0x102d469; continue;
 }
 0x102d469 => {
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x102d46b; continue;
 }
 0x102d46b => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let a = cpu.edx; let b = cpu.ebp; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebp = v; } }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x102d473; continue;
 }
 0x102d473 => {
  cpu.push(0x102d478); f_0102d673(cpu); if cpu.returned_to != 0x102d478 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102d473; continue; }
  pc = 0x102d47c; continue;
 }
 0x102d47c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d47d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102d47d); }
 let mut pc: u32 = 0x102d47d;
 loop { match pc {
 0x102d47d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d488(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102d488); }
 let mut pc: u32 = 0x102d488;
 loop { match pc {
 0x102d488 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = (cpu.edx as u16) as i16 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x102d4a3; continue; }
  pc = 0x102d498; continue;
 }
 0x102d498 => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if !cpu.zf { pc = 0x102d4a3; continue; }
  pc = 0x102d49c; continue;
 }
 0x102d49c => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  f_0102d5d9(cpu); return;
 }
 0x102d4a3 => {
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102d4aa; continue; }
  pc = 0x102d4a7; continue;
 }
 0x102d4a7 => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x102d4aa; continue;
 }
 0x102d4aa => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x280u32; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.ebx); cpu.edx = v; }
  if !cpu.sf { pc = 0x102d4b9; continue; }
  pc = 0x102d4b5; continue;
 }
 0x102d4b5 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x102d4b9; continue;
 }
 0x102d4b9 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x112134du32)); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, 0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x1121349u32)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1121365u32)); cpu.eax = v; }
  { cpu.pushad(); }
  cpu.push(0x102d4e1); f_01027bb1(cpu); if cpu.returned_to != 0x102d4e1 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.edi & 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0xa0000u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if cpu.cf { pc = 0x102d565; continue; }
  pc = 0x102d4f3; continue;
 }
 0x102d4f3 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 1u32); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.sf { pc = 0x102d531; continue; }
  pc = 0x102d4ff; continue;
 }
 0x102d4ff => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.inc16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x102d514; continue; }
  pc = 0x102d505; continue;
 }
 0x102d505 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102d513); f_01027bb1(cpu); if cpu.returned_to != 0x102d513 { return; }
  { cpu.popad(); }
  pc = 0x102d514; continue;
 }
 0x102d514 => {
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  if !cpu.sf { pc = 0x102d52e; continue; }
  pc = 0x102d518; continue;
 }
 0x102d518 => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { let v: u16 = cpu.add16((cpu.edi as u16), (cpu.esi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102d52e; continue; }
  pc = 0x102d51f; continue;
 }
 0x102d51f => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102d52d); f_01027bb1(cpu); if cpu.returned_to != 0x102d52d { return; }
  { cpu.popad(); }
  pc = 0x102d52e; continue;
 }
 0x102d52e => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102d4ff; continue; }
  pc = 0x102d530; continue;
 }
 0x102d530 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x102d531 => {
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x102d533; continue;
 }
 0x102d533 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.inc16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x102d548; continue; }
  pc = 0x102d539; continue;
 }
 0x102d539 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102d547); f_01027bb1(cpu); if cpu.returned_to != 0x102d547 { return; }
  { cpu.popad(); }
  pc = 0x102d548; continue;
 }
 0x102d548 => {
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  if !cpu.sf { pc = 0x102d562; continue; }
  pc = 0x102d54c; continue;
 }
 0x102d54c => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.esi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102d562; continue; }
  pc = 0x102d553; continue;
 }
 0x102d553 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102d561); f_01027bb1(cpu); if cpu.returned_to != 0x102d561 { return; }
  { cpu.popad(); }
  pc = 0x102d562; continue;
 }
 0x102d562 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102d533; continue; }
  pc = 0x102d564; continue;
 }
 0x102d564 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x102d565 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 1u32); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.sf { pc = 0x102d5a5; continue; }
  pc = 0x102d573; continue;
 }
 0x102d573 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.add16((cpu.edi as u16), (cpu.esi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102d589; continue; }
  pc = 0x102d57a; continue;
 }
 0x102d57a => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102d588); f_01027bb1(cpu); if cpu.returned_to != 0x102d588 { return; }
  { cpu.popad(); }
  pc = 0x102d589; continue;
 }
 0x102d589 => {
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.ebx); cpu.ebp = v; }
  if !cpu.sf { pc = 0x102d5a2; continue; }
  pc = 0x102d58d; continue;
 }
 0x102d58d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u16 = cpu.inc16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x102d5a2; continue; }
  pc = 0x102d593; continue;
 }
 0x102d593 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102d5a1); f_01027bb1(cpu); if cpu.returned_to != 0x102d5a1 { return; }
  { cpu.popad(); }
  pc = 0x102d5a2; continue;
 }
 0x102d5a2 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102d573; continue; }
  pc = 0x102d5a4; continue;
 }
 0x102d5a4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x102d5a5 => {
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x102d5a7; continue;
 }
 0x102d5a7 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.esi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102d5bd; continue; }
  pc = 0x102d5ae; continue;
 }
 0x102d5ae => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102d5bc); f_01027bb1(cpu); if cpu.returned_to != 0x102d5bc { return; }
  { cpu.popad(); }
  pc = 0x102d5bd; continue;
 }
 0x102d5bd => {
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.ebx); cpu.ebp = v; }
  if !cpu.sf { pc = 0x102d5d6; continue; }
  pc = 0x102d5c1; continue;
 }
 0x102d5c1 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u16 = cpu.inc16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x102d5d6; continue; }
  pc = 0x102d5c7; continue;
 }
 0x102d5c7 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102d5d5); f_01027bb1(cpu); if cpu.returned_to != 0x102d5d5 { return; }
  { cpu.popad(); }
  pc = 0x102d5d6; continue;
 }
 0x102d5d6 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102d5a7; continue; }
  pc = 0x102d5d8; continue;
 }
 0x102d5d8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d5d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102d5d9); }
 let mut pc: u32 = 0x102d5d9;
 loop { match pc {
 0x102d5d9 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = (cpu.edx as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1121349u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x112134du32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1121cd1u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.edi = v; }
  { let v: u16 = cpu.add16((cpu.edi as u16), (cpu.eax as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x1121371u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.r32(0x1121365u32)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x102d60c); f_01027bb1(cpu); if cpu.returned_to != 0x102d60c { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d611(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102d611); }
 let mut pc: u32 = 0x102d611;
 loop { match pc {
 0x102d611 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1121349u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x112134du32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1121cd1u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.edi = v; }
  { let v: u16 = cpu.add16((cpu.edi as u16), (cpu.eax as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x1121371u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.r32(0x1121365u32)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x102d641); f_01027bb1(cpu); if cpu.returned_to != 0x102d641 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d646(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102d646); }
 let mut pc: u32 = 0x102d646;
 loop { match pc {
 0x102d646 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = (cpu.edx as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x102d656); f_0102d5d9(cpu); if cpu.returned_to != 0x102d656 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x102d660); f_0102d5d9(cpu); if cpu.returned_to != 0x102d660 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x102d66a); f_0102d5d9(cpu); if cpu.returned_to != 0x102d66a { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102d672); f_0102d5d9(cpu); if cpu.returned_to != 0x102d672 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d673(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102d673); }
 let mut pc: u32 = 0x102d673;
 loop { match pc {
 0x102d673 => {
  { cpu.pushad(); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = (cpu.edx as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x102d70b; continue; }
  pc = 0x102d688; continue;
 }
 0x102d688 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x10e4108u32)); }
  if !cpu.cf { pc = 0x102d70b; continue; }
  pc = 0x102d690; continue;
 }
 0x102d690 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.sf { pc = 0x102d69a; continue; }
  pc = 0x102d694; continue;
 }
 0x102d694 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102d70b; continue; }
  pc = 0x102d698; continue;
 }
 0x102d698 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x102d69a; continue;
 }
 0x102d69a => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0x10e4104u32)); cpu.ebp = v; }
  if (cpu.sf != cpu.of) { pc = 0x102d6aa; continue; }
  pc = 0x102d6a6; continue;
 }
 0x102d6a6 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebp); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102d70b; continue; }
  pc = 0x102d6aa; continue;
 }
 0x102d6aa => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1121349u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x112134du32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1121cd1u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.edi = v; }
  { let v: u16 = cpu.add16((cpu.edi as u16), (cpu.eax as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x1121371u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.r32(0x1121365u32)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x102d6d5); f_01027bb1(cpu); if cpu.returned_to != 0x102d6d5 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0x1121551u32.wrapping_add(cpu.ebx.wrapping_mul(2))), 0x0u16); }
  if !cpu.zf { pc = 0x102d6f2; continue; }
  pc = 0x102d6e3; continue;
 }
 0x102d6e3 => {
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(2, true); }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { cpu.stos(1, true); }
  pc = 0x102d70b; continue;
 }
 0x102d6f2 => {
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x102d6f4; continue;
 }
 0x102d6f4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.inc16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x102d709; continue; }
  pc = 0x102d6fa; continue;
 }
 0x102d6fa => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102d708); f_01027bb1(cpu); if cpu.returned_to != 0x102d708 { return; }
  { cpu.popad(); }
  pc = 0x102d709; continue;
 }
 0x102d709 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102d6f4; continue; }
  pc = 0x102d70b; continue;
 }
 0x102d70b => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d70d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102d70d); }
 let mut pc: u32 = 0x102d70d;
 loop { match pc {
 0x102d70d => {
  { cpu.pushad(); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = (cpu.edx as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x102d789; continue; }
  pc = 0x102d71e; continue;
 }
 0x102d71e => {
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4104u32)); }
  if !cpu.cf { pc = 0x102d789; continue; }
  pc = 0x102d726; continue;
 }
 0x102d726 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.sf { pc = 0x102d730; continue; }
  pc = 0x102d72a; continue;
 }
 0x102d72a => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102d789; continue; }
  pc = 0x102d72e; continue;
 }
 0x102d72e => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x102d730; continue;
 }
 0x102d730 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0x10e4108u32)); cpu.ebp = v; }
  if (cpu.sf != cpu.of) { pc = 0x102d741; continue; }
  pc = 0x102d73c; continue;
 }
 0x102d73c => {
  { let v: u16 = cpu.sub16((cpu.ecx as u16), (cpu.ebp as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102d789; continue; }
  pc = 0x102d741; continue;
 }
 0x102d741 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1121349u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x112134du32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1121cd1u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.edi = v; }
  { let v: u16 = cpu.add16((cpu.edi as u16), (cpu.eax as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x1121371u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.r32(0x1121365u32)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x102d76b); f_01027bb1(cpu); if cpu.returned_to != 0x102d76b { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x102d76f; continue;
 }
 0x102d76f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x280u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102d787; continue; }
  pc = 0x102d778; continue;
 }
 0x102d778 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102d786); f_01027bb1(cpu); if cpu.returned_to != 0x102d786 { return; }
  { cpu.popad(); }
  pc = 0x102d787; continue;
 }
 0x102d787 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102d76f; continue; }
  pc = 0x102d789; continue;
 }
 0x102d789 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d78d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102d78d); }
 let mut pc: u32 = 0x102d78d;
 loop { match pc {
 0x102d78d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x118cd68u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.lods(1, false); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.cf { pc = 0x102d856; continue; }
  pc = 0x102d7a9; continue;
 }
 0x102d7a9 => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(1, false); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.cf { pc = 0x102d856; continue; }
  pc = 0x102d7bc; continue;
 }
 0x102d7bc => {
  { let v: u32 = 0x11d6898u32; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x3u32); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0xfffffffcu32; cpu.esi = v; }
  { let v: u32 = 0xfcu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  pc = 0x102d7ef; continue;
 }
 0x102d7ef => {
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x4u8); }
  if !cpu.zf { pc = 0x102d809; continue; }
  pc = 0x102d7f4; continue;
 }
 0x102d7f4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d7ef; continue; }
  pc = 0x102d807; continue;
 }
 0x102d807 => {
  pc = 0x102d831; continue;
 }
 0x102d809 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x10c0008u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x102d815; continue;
 }
 0x102d815 => {
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { cpu.lods(1, false); }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102d815; continue; }
  pc = 0x102d823; continue;
 }
 0x102d823 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d7ef; continue; }
  pc = 0x102d831; continue;
 }
 0x102d831 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  if !cpu.sf { pc = 0x102d83b; continue; }
  pc = 0x102d839; continue;
 }
 0x102d839 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x102d83b; continue;
 }
 0x102d83b => {
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  pc = 0x102d83e; continue;
 }
 0x102d83e => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102d83e; continue; }
  pc = 0x102d849; continue;
 }
 0x102d849 => {
  { let v: u16 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x102d856 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d85a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102d85a); }
 let mut pc: u32 = 0x102d85a;
 loop { match pc {
 0x102d85a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3fffu32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1122480u32); cpu.edi = v; }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x280u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if cpu.cf { pc = 0x102d8c6; continue; }
  pc = 0x102d86d; continue;
 }
 0x102d86d => {
  { let v: u16 = (cpu.edi as u16).wrapping_sub(0x280u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1122470u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1122474u32); cpu.edx = v; }
  pc = 0x102d885; continue;
 }
 0x102d885 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & cpu.ebp; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x11d6898u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102d8a7; continue; }
  pc = 0x102d895; continue;
 }
 0x102d895 => {
  { cpu.stos(1, false); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1122478u32)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x112247cu32)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102d885; continue; }
  pc = 0x102d8a5; continue;
 }
 0x102d8a5 => {
  pc = 0x102d8b7; continue;
 }
 0x102d8a7 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1122478u32)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x112247cu32)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102d885; continue; }
  pc = 0x102d8b7; continue;
 }
 0x102d8b7 => {
  { let v: u32 = cpu.add32(cpu.r32(0x1122480u32), 0x280u32); cpu.w32(0x1122480u32, v); }
  pc = 0x102d949; continue;
 }
 0x102d8c6 => {
  { let v: u16 = (cpu.edi as u16).wrapping_sub(0x280u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edi as u16), (cpu.eax as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102d8df; continue; }
  pc = 0x102d8d0; continue;
 }
 0x102d8d0 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102d8de); f_01027bb1(cpu); if cpu.returned_to != 0x102d8de { return; }
  { cpu.popad(); }
  pc = 0x102d8df; continue;
 }
 0x102d8df => {
  { let v: u32 = 0x27fu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1122470u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1122474u32); cpu.edx = v; }
  pc = 0x102d8fa; continue;
 }
 0x102d8fa => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & cpu.ebp; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x11d6898u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102d90c; continue; }
  pc = 0x102d90a; continue;
 }
 0x102d90a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102d90c; continue;
 }
 0x102d90c => {
  { let v: u16 = cpu.inc16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x102d91f; continue; }
  pc = 0x102d910; continue;
 }
 0x102d910 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102d91e); f_01027bb1(cpu); if cpu.returned_to != 0x102d91e { return; }
  { cpu.popad(); }
  pc = 0x102d91f; continue;
 }
 0x102d91f => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1122478u32)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x112247cu32)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102d8fa; continue; }
  pc = 0x102d92e; continue;
 }
 0x102d92e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.edi as u16), (cpu.eax as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102d943; continue; }
  pc = 0x102d934; continue;
 }
 0x102d934 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102d942); f_01027bb1(cpu); if cpu.returned_to != 0x102d942 { return; }
  { cpu.popad(); }
  pc = 0x102d943; continue;
 }
 0x102d943 => {
  { let v: u32 = cpu.edi; cpu.w32(0x1122480u32, v); }
  pc = 0x102d949; continue;
 }
 0x102d949 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102d94b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102d94b); } r_0102d94b(cpu, 0x102d94b); }
pub fn f_0102d9e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102d9e8); } r_0102d94b(cpu, 0x102d9e8); }
pub fn f_0102d9f0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102d9f0); } r_0102d94b(cpu, 0x102d9f0); }
pub fn f_0102d9f6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102d9f6); } r_0102d94b(cpu, 0x102d9f6); }
pub fn f_0102da20(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102da20); } r_0102d94b(cpu, 0x102da20); }
pub fn f_0102da28(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102da28); } r_0102d94b(cpu, 0x102da28); }
pub fn f_0102da30(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102da30); } r_0102d94b(cpu, 0x102da30); }
pub fn f_0102da70(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102da70); } r_0102d94b(cpu, 0x102da70); }
fn r_0102d94b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102d94b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x118cd68u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.lods(1, false); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.cf { pc = 0x102da6c; continue; }
  pc = 0x102d967; continue;
 }
 0x102d967 => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(1, false); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.cf { pc = 0x102da6c; continue; }
  pc = 0x102d97a; continue;
 }
 0x102d97a => {
  { let v = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.mul32(v); }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x102d9e8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x102da20u32, v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x102d9f0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x102da28u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x102d9f6u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x102da30u32, v); }
  pc = 0x102d9a6; continue;
 }
 0x102d9a6 => {
  { let v: u32 = 0x11d6898u32; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x3u32); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0xfffffffcu32; cpu.esi = v; }
  { let v: u32 = 0xfcu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  pc = 0x102d9d9; continue;
 }
 0x102d9d9 => {
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x4u8); }
  if !cpu.zf { pc = 0x102da0e; continue; }
  pc = 0x102d9de; continue;
 }
 0x102d9de => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x102d9e4; continue;
 }
 0x102d9e4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x12345678u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x12345678u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x12345678u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102d9e4; continue; }
  pc = 0x102d9ff; continue;
 }
 0x102d9e8 => {
  if cpu.sf { pc = 0x102da40; continue; }
  pc = 0x102d9ea; continue;
 }
 0x102d9ea => {
  { let v: u8 = (cpu.eax as u8) ^ 0x12u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x12345678u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x12345678u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102d9e4; continue; }
  pc = 0x102d9ff; continue;
 }
 0x102d9f0 => {
  if cpu.sf { pc = 0x102da48; continue; }
  pc = 0x102d9f2; continue;
 }
 0x102d9f2 => {
  { let v: u8 = (cpu.eax as u8) ^ 0x12u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x12345678u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102d9e4; continue; }
  pc = 0x102d9ff; continue;
 }
 0x102d9f6 => {
  if cpu.sf { pc = 0x102da4e; continue; }
  pc = 0x102d9f8; continue;
 }
 0x102d9f8 => {
  { let v: u8 = (cpu.eax as u8) ^ 0x12u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102d9e4; continue; }
  pc = 0x102d9ff; continue;
 }
 0x102d9ff => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d9d9; continue; }
  pc = 0x102da0c; continue;
 }
 0x102da0c => {
  pc = 0x102da47; continue;
 }
 0x102da0e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x10c0008u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x102da1a; continue;
 }
 0x102da1a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x12345678u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x12345678u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x12345678u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102da1a; continue; }
  pc = 0x102da39; continue;
 }
 0x102da20 => {
  if cpu.sf { pc = 0x102da78; continue; }
  pc = 0x102da22; continue;
 }
 0x102da22 => {
  { let v: u8 = (cpu.eax as u8) ^ 0x12u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x12345678u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x12345678u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102da1a; continue; }
  pc = 0x102da39; continue;
 }
 0x102da28 => {
  if cpu.sf { pc = 0x102da80; continue; }
  pc = 0x102da2a; continue;
 }
 0x102da2a => {
  { let v: u8 = (cpu.eax as u8) ^ 0x12u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x12345678u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102da1a; continue; }
  pc = 0x102da39; continue;
 }
 0x102da30 => {
  if cpu.sf { pc = 0x102da88; continue; }
  pc = 0x102da32; continue;
 }
 0x102da32 => {
  { let v: u8 = (cpu.eax as u8) ^ 0x12u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102da1a; continue; }
  pc = 0x102da39; continue;
 }
 0x102da39 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  pc = 0x102da40; continue;
 }
 0x102da40 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x102d9d9; continue; }
  pc = 0x102da47; continue;
 }
 0x102da47 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  if !cpu.sf { pc = 0x102da51; continue; }
  pc = 0x102da4f; continue;
 }
 0x102da48 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.df = false; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  if !cpu.sf { pc = 0x102da51; continue; }
  pc = 0x102da4f; continue;
 }
 0x102da4e => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.r32(0xca8b02c3u32.wrapping_add(cpu.ebx)) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102da54; continue; }
  pc = 0x102da5f; continue;
 }
 0x102da4f => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x102da51; continue;
 }
 0x102da51 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  pc = 0x102da54; continue;
 }
 0x102da54 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102da54; continue; }
  pc = 0x102da5f; continue;
 }
 0x102da5f => {
  { let v: u16 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x102da6c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x102da70 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3fffu32; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1122484u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1122480u32); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1122470u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1122474u32); cpu.edx = v; }
  pc = 0x102daa8; continue;
 }
 0x102da78 => {
  { let v: u32 = 0x2b003c4u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1122484u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1122480u32); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1122470u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1122474u32); cpu.edx = v; }
  pc = 0x102daa8; continue;
 }
 0x102da80 => {
  { let v: u8 = (cpu.eax as u8) & 0x12u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffefu32.wrapping_add(cpu.esi)).wrapping_add(cpu.esp); cpu.w32(0xffffffefu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1122480u32); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1122470u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1122474u32); cpu.edx = v; }
  pc = 0x102daa8; continue;
 }
 0x102da88 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) & 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1122470u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1122474u32); cpu.edx = v; }
  pc = 0x102daa8; continue;
 }
 0x102daa8 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & cpu.ebp; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x11d6898u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x102daba; continue; }
  pc = 0x102dab8; continue;
 }
 0x102dab8 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x102daba; continue;
 }
 0x102daba => {
  { let v: u32 = cpu.edi.wrapping_add(0x52u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1122478u32)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x112247cu32)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102daa8; continue; }
  pc = 0x102dacc; continue;
 }
 0x102dacc => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102dace(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102dace); }
 let mut pc: u32 = 0x102dace;
 loop { match pc {
 0x102dace => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff8u32); cpu.esp = v; }
  { let v: u32 = 0x10ea910u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e4098u32); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x10e690cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0x10e690eu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102daf7; continue;
 }
 0x102daf7 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x102dafb; continue;
 }
 0x102dafb => {
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.lods(1, false); }
  { let v: u16 = (cpu.eax as u16) & 0x7f7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x7eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0x7f7fu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x18u8); }
  if cpu.zf { pc = 0x102db62; continue; }
  pc = 0x102db10; continue;
 }
 0x102db10 => {
  { cpu.sub8((cpu.eax as u8), 0x19u8); }
  if cpu.zf { pc = 0x102db62; continue; }
  pc = 0x102db14; continue;
 }
 0x102db14 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x18u8); }
  if cpu.zf { pc = 0x102db58; continue; }
  pc = 0x102db19; continue;
 }
 0x102db19 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x19u8); }
  if cpu.zf { pc = 0x102db58; continue; }
  pc = 0x102db1e; continue;
 }
 0x102db1e => {
  { cpu.sub8((cpu.ebx as u8), 0x18u8); }
  if cpu.zf { pc = 0x102db60; continue; }
  pc = 0x102db23; continue;
 }
 0x102db23 => {
  { cpu.sub8((cpu.ebx as u8), 0x19u8); }
  if cpu.zf { pc = 0x102db60; continue; }
  pc = 0x102db28; continue;
 }
 0x102db28 => {
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x18u8); }
  if cpu.zf { pc = 0x102db5c; continue; }
  pc = 0x102db2d; continue;
 }
 0x102db2d => {
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x19u8); }
  if cpu.zf { pc = 0x102db5c; continue; }
  pc = 0x102db32; continue;
 }
 0x102db32 => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if cpu.zf { pc = 0x102db62; continue; }
  pc = 0x102db36; continue;
 }
 0x102db36 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xbu8); }
  if cpu.zf { pc = 0x102db58; continue; }
  pc = 0x102db3b; continue;
 }
 0x102db3b => {
  { cpu.sub8((cpu.ebx as u8), 0xbu8); }
  if cpu.zf { pc = 0x102db60; continue; }
  pc = 0x102db40; continue;
 }
 0x102db40 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x102db62; continue; }
  pc = 0x102db44; continue;
 }
 0x102db44 => {
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x102db62; continue; }
  pc = 0x102db48; continue;
 }
 0x102db48 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x102db62; continue; }
  pc = 0x102db4c; continue;
 }
 0x102db4c => {
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x102db58; continue; }
  pc = 0x102db50; continue;
 }
 0x102db50 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x102db58; continue; }
  pc = 0x102db54; continue;
 }
 0x102db54 => {
  { cpu.sub8((cpu.ebx as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x102db60; continue; }
  pc = 0x102db58; continue;
 }
 0x102db58 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x102db62; continue;
 }
 0x102db5c => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x102db62; continue;
 }
 0x102db60 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x102db62; continue;
 }
 0x102db62 => {
  { let v: u32 = 0x10f7363u32; cpu.ebx = v; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102dafb; continue; }
  pc = 0x102db6c; continue;
 }
 0x102db6c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x100u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x102daf7; continue; }
  pc = 0x102db7c; continue;
 }
 0x102db7c => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102db7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102db7e); }
 let mut pc: u32 = 0x102db7e;
 loop { match pc {
 0x102db7e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.eax = v; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1121349u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x112134du32)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, 0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1121365u32)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x102dbac); f_01027bb1(cpu); if cpu.returned_to != 0x102dbac { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0xa0000u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e4098u32); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10e690cu32) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 3u32); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x10e690eu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  pc = 0x102dbd2; continue;
 }
 0x102dbd2 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u16 = cpu.add16((cpu.edx as u16), 0x280u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if cpu.cf { pc = 0x102dbed; continue; }
  pc = 0x102dbdd; continue;
 }
 0x102dbdd => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x280u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102dbd2; continue; }
  pc = 0x102dbeb; continue;
 }
 0x102dbeb => {
  pc = 0x102dc2a; continue;
 }
 0x102dbed => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x4u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102dc05; continue; }
  pc = 0x102dbf6; continue;
 }
 0x102dbf6 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102dc04); f_01027bb1(cpu); if cpu.returned_to != 0x102dc04 { return; }
  { cpu.popad(); }
  pc = 0x102dc05; continue;
 }
 0x102dc05 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102dbed; continue; }
  pc = 0x102dc07; continue;
 }
 0x102dc07 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = 0x280u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.edi as u16), (cpu.eax as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x102dc27; continue; }
  pc = 0x102dc18; continue;
 }
 0x102dc18 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x11d6652u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x102dc26); f_01027bb1(cpu); if cpu.returned_to != 0x102dc26 { return; }
  { cpu.popad(); }
  pc = 0x102dc27; continue;
 }
 0x102dc27 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102dbd2; continue; }
  pc = 0x102dc2a; continue;
 }
 0x102dc2a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102dc2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102dc2c); }
 let mut pc: u32 = 0x102dc2c;
 loop { match pc {
 0x102dc2c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x1121348u32), 0x1u8); }
  if !cpu.zf { pc = 0x102dc51; continue; }
  pc = 0x102dc45; continue;
 }
 0x102dc45 => {
  { let v: u32 = 0x3e800u32; cpu.w32(0x112136du32, v); }
  pc = 0x102dc5b; continue;
 }
 0x102dc51 => {
  { let v: u32 = 0x4b000u32; cpu.w32(0x112136du32, v); }
  pc = 0x102dc5b; continue;
 }
 0x102dc5b => {
  { let v: u32 = 0xa0000u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102dc62; continue;
 }
 0x102dc62 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1121369u32)); cpu.eax = v; }
  cpu.push(0x102dc70); f_01027bb1(cpu); if cpu.returned_to != 0x102dc70 { return; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1121365u32)); cpu.eax = v; }
  cpu.push(0x102dc8b); f_01027bb1(cpu); if cpu.returned_to != 0x102dc8b { return; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u16 = cpu.add16(cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x400u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  if !cpu.cf { pc = 0x102dca3; continue; }
  pc = 0x102dca0; continue;
 }
 0x102dca0 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102dca3; continue;
 }
 0x102dca3 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x112136du32), 0x400u32); cpu.w32(0x112136du32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x102dc62; continue; }
  pc = 0x102dcaf; continue;
 }
 0x102dcaf => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102dcb1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102dcb1); }
 let mut pc: u32 = 0x102dcb1;
 loop { match pc {
 0x102dcb1 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x102dcdb; continue; }
  pc = 0x102dcd0; continue;
 }
 0x102dcd0 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4108u32)); }
  if (cpu.sf != cpu.of) { pc = 0x102dcdd; continue; }
  pc = 0x102dcdb; continue;
 }
 0x102dcdb => {
  pc = 0x102dce8; continue;
 }
 0x102dcdd => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4100u32)); }
  if (cpu.sf == cpu.of) { pc = 0x102dced; continue; }
  pc = 0x102dce8; continue;
 }
 0x102dce8 => {
  pc = 0x102dd74; continue;
 }
 0x102dced => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x102dd1e; continue; }
  pc = 0x102dd15; continue;
 }
 0x102dd15 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(0x80u32); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102dd22; continue;
 }
 0x102dd1e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(0xffffff81u32); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102dd22; continue;
 }
 0x102dd22 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x102dd28; continue;
 }
 0x102dd28 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102dd3a; continue; }
  pc = 0x102dd30; continue;
 }
 0x102dd30 => {
  pc = 0x102dd74; continue;
 }
 0x102dd32 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x102dd28; continue;
 }
 0x102dd3a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4100u32)); }
  if (cpu.sf != cpu.of) { pc = 0x102dd50; continue; }
  pc = 0x102dd45; continue;
 }
 0x102dd45 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4108u32)); }
  if (cpu.sf != cpu.of) { pc = 0x102dd52; continue; }
  pc = 0x102dd50; continue;
 }
 0x102dd50 => {
  pc = 0x102dd6c; continue;
 }
 0x102dd52 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11daca8u32.wrapping_add(cpu.eax), v); }
  pc = 0x102dd6c; continue;
 }
 0x102dd6c => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x102dd32; continue;
 }
 0x102dd74 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102dd7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102dd7c); }
 let mut pc: u32 = 0x102dd7c;
 loop { match pc {
 0x102dd7c => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xa0u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff80u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff84u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x80u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102e30c; continue; }
  pc = 0x102dda6; continue;
 }
 0x102dda6 => {
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x102ddae); f_0102d78d(cpu); if cpu.returned_to != 0x102ddae { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)) & 0xffffu32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x102ddcf; continue; }
  pc = 0x102ddc7; continue;
 }
 0x102ddc7 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x102dde2; continue; }
  pc = 0x102ddcf; continue;
 }
 0x102ddcf => {
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x102dddd); f_0102cf5a(cpu); if cpu.returned_to != 0x102dddd { return; }
  pc = 0x102e30c; continue;
 }
 0x102dde2 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.neg32(cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.neg32(cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)) & 0xffu32; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1122488u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1122488u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1122488u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1122488u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff64u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff74u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff68u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff78u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff6cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 6u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x102e039; continue; }
  pc = 0x102e02e; continue;
 }
 0x102e02e => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e40fcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x102e03e; continue; }
  pc = 0x102e039; continue;
 }
 0x102e039 => {
  pc = 0x102e30c; continue;
 }
 0x102e03e => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x102e054; continue; }
  pc = 0x102e049; continue;
 }
 0x102e049 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4100u32)); }
  if (cpu.sf == cpu.of) { pc = 0x102e059; continue; }
  pc = 0x102e054; continue;
 }
 0x102e054 => {
  pc = 0x102e30c; continue;
 }
 0x102e059 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4100u32)); }
  if (cpu.sf == cpu.of) { pc = 0x102e06c; continue; }
  pc = 0x102e064; continue;
 }
 0x102e064 => {
  { let v: u32 = cpu.r32(0x10e4100u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e06c; continue;
 }
 0x102e06c => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4108u32)); }
  if (cpu.sf != cpu.of) { pc = 0x102e080; continue; }
  pc = 0x102e077; continue;
 }
 0x102e077 => {
  { let v: u32 = cpu.r32(0x10e4108u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e080; continue;
 }
 0x102e080 => {
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x102e0bb); f_0102dcb1(cpu); if cpu.returned_to != 0x102e0bb { return; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x102e0f6); f_0102dcb1(cpu); if cpu.returned_to != 0x102e0f6 { return; }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x102e134); f_0102dcb1(cpu); if cpu.returned_to != 0x102e134 { return; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x102e172); f_0102dcb1(cpu); if cpu.returned_to != 0x102e172 { return; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x80u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1122478u32, v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x112247cu32, v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e40fcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x102e1a3; continue; }
  pc = 0x102e19b; continue;
 }
 0x102e19b => {
  { let v: u32 = cpu.r32(0x10e40fcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e1a3; continue;
 }
 0x102e1a3 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4104u32)); }
  if (cpu.sf != cpu.of) { pc = 0x102e1b7; continue; }
  pc = 0x102e1ae; continue;
 }
 0x102e1ae => {
  { let v: u32 = cpu.r32(0x10e4104u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e1b7; continue;
 }
 0x102e1b7 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)), 0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1121365u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1121349u32)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x112134du32), 0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1122480u32, v); }
  { let v: u32 = cpu.r32(0x1122480u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x102e1ed); f_01027bb1(cpu); if cpu.returned_to != 0x102e1ed { return; }
  { let v: u32 = cpu.r32(0x1122480u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xa0000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1122480u32, v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e207; continue;
 }
 0x102e207 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102e21c; continue; }
  pc = 0x102e20f; continue;
 }
 0x102e20f => {
  pc = 0x102e30c; continue;
 }
 0x102e214 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x102e207; continue;
 }
 0x102e21c => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11daca8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11dacacu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e40fcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x102e250; continue; }
  pc = 0x102e245; continue;
 }
 0x102e245 => {
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4104u32)); }
  if (cpu.sf != cpu.of) { pc = 0x102e252; continue; }
  pc = 0x102e250; continue;
 }
 0x102e250 => {
  pc = 0x102e25a; continue;
 }
 0x102e252 => {
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x102e25f; continue; }
  pc = 0x102e25a; continue;
 }
 0x102e25a => {
  pc = 0x102e2da; continue;
 }
 0x102e25f => {
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e40fcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x102e272; continue; }
  pc = 0x102e26a; continue;
 }
 0x102e26a => {
  { let v: u32 = cpu.r32(0x10e40fcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e272; continue;
 }
 0x102e272 => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4104u32)); }
  if (cpu.sf != cpu.of) { pc = 0x102e286; continue; }
  pc = 0x102e27d; continue;
 }
 0x102e27d => {
  { let v: u32 = cpu.r32(0x10e4104u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e286; continue;
 }
 0x102e286 => {
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1122470u32, v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1122474u32, v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x102e2d8); f_0102d85a(cpu); if cpu.returned_to != 0x102e2d8 { return; }
  pc = 0x102e307; continue;
 }
 0x102e2da => {
  { let v: u32 = cpu.r32(0x1122480u32).wrapping_add(0x280u32); cpu.w32(0x1122480u32, v); }
  { cpu.sub32(cpu.r32(0x1122480u32), 0xb0000u32); }
  if cpu.cf { pc = 0x102e307; continue; }
  pc = 0x102e2f0; continue;
 }
 0x102e2f0 => {
  { let v: u16 = cpu.r16(0x11d6652u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x102e2fd); f_01027bb1(cpu); if cpu.returned_to != 0x102e2fd { return; }
  { let v: u32 = cpu.r32(0x1122480u32).wrapping_sub(0x10000u32); cpu.w32(0x1122480u32, v); }
  pc = 0x102e307; continue;
 }
 0x102e307 => {
  cpu.poll(); pc = 0x102e214; continue;
 }
 0x102e30c => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102e311(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102e311); }
 let mut pc: u32 = 0x102e311;
 loop { match pc {
 0x102e311 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x102e33b; continue; }
  pc = 0x102e330; continue;
 }
 0x102e330 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4104u32)); }
  if (cpu.sf != cpu.of) { pc = 0x102e33d; continue; }
  pc = 0x102e33b; continue;
 }
 0x102e33b => {
  pc = 0x102e348; continue;
 }
 0x102e33d => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e40fcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x102e34d; continue; }
  pc = 0x102e348; continue;
 }
 0x102e348 => {
  pc = 0x102e3d4; continue;
 }
 0x102e34d => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x102e37e; continue; }
  pc = 0x102e375; continue;
 }
 0x102e375 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(0x80u32); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e382; continue;
 }
 0x102e37e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(0xffffff81u32); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e382; continue;
 }
 0x102e382 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e388; continue;
 }
 0x102e388 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102e39a; continue; }
  pc = 0x102e390; continue;
 }
 0x102e390 => {
  pc = 0x102e3d4; continue;
 }
 0x102e392 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x102e388; continue;
 }
 0x102e39a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e40fcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x102e3b0; continue; }
  pc = 0x102e3a5; continue;
 }
 0x102e3a5 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4104u32)); }
  if (cpu.sf != cpu.of) { pc = 0x102e3b2; continue; }
  pc = 0x102e3b0; continue;
 }
 0x102e3b0 => {
  pc = 0x102e3cc; continue;
 }
 0x102e3b2 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11daca8u32.wrapping_add(cpu.eax), v); }
  pc = 0x102e3cc; continue;
 }
 0x102e3cc => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x102e392; continue;
 }
 0x102e3d4 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102e3dc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102e3dc); } r_0102e3dc(cpu, 0x102e3dc); }
pub fn f_0102e905(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102e905); } r_0102e3dc(cpu, 0x102e905); }
fn r_0102e3dc(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102e3dc => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xa0u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff80u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff84u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x80u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102e935; continue; }
  pc = 0x102e406; continue;
 }
 0x102e406 => {
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x102e40e); f_0102d94b(cpu); if cpu.returned_to != 0x102e40e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)) & 0xffffu32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x102e42f; continue; }
  pc = 0x102e427; continue;
 }
 0x102e427 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x102e434; continue; }
  pc = 0x102e42f; continue;
 }
 0x102e42f => {
  pc = 0x102e935; continue;
 }
 0x102e434 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.neg32(cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.neg32(cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)) & 0xffu32; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1122488u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1122488u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1122488u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1122488u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff64u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff74u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff68u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff78u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff6cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 6u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x102e68b; continue; }
  pc = 0x102e680; continue;
 }
 0x102e680 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e40fcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x102e690; continue; }
  pc = 0x102e68b; continue;
 }
 0x102e68b => {
  pc = 0x102e935; continue;
 }
 0x102e690 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x102e6a6; continue; }
  pc = 0x102e69b; continue;
 }
 0x102e69b => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4100u32)); }
  if (cpu.sf == cpu.of) { pc = 0x102e6ab; continue; }
  pc = 0x102e6a6; continue;
 }
 0x102e6a6 => {
  pc = 0x102e935; continue;
 }
 0x102e6ab => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4100u32)); }
  if (cpu.sf == cpu.of) { pc = 0x102e6be; continue; }
  pc = 0x102e6b6; continue;
 }
 0x102e6b6 => {
  { let v: u32 = cpu.r32(0x10e4100u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e6be; continue;
 }
 0x102e6be => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4108u32)); }
  if (cpu.sf != cpu.of) { pc = 0x102e6d2; continue; }
  pc = 0x102e6c9; continue;
 }
 0x102e6c9 => {
  { let v: u32 = cpu.r32(0x10e4108u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e6d2; continue;
 }
 0x102e6d2 => {
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x102e70d); f_0102e311(cpu); if cpu.returned_to != 0x102e70d { return; }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x102e748); f_0102e311(cpu); if cpu.returned_to != 0x102e748 { return; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x102e786); f_0102e311(cpu); if cpu.returned_to != 0x102e786 { return; }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v = cpu.r32(0xffffff70u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffff70u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff60u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x102e7c4); f_0102e311(cpu); if cpu.returned_to != 0x102e7c4 { return; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x80u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1122478u32, v); }
  { let v: u32 = cpu.neg32(cpu.r32(0x1122478u32)); cpu.w32(0x1122478u32, v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x112247cu32, v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e40fcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x102e7fb; continue; }
  pc = 0x102e7f3; continue;
 }
 0x102e7f3 => {
  { let v: u32 = cpu.r32(0x10e40fcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e7fb; continue;
 }
 0x102e7fb => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4104u32)); }
  if (cpu.sf != cpu.of) { pc = 0x102e80f; continue; }
  pc = 0x102e806; continue;
 }
 0x102e806 => {
  { let v: u32 = cpu.r32(0x10e4104u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e80f; continue;
 }
 0x102e80f => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10e4114u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1122480u32, v); }
  { let v: u8 = cpu.r8(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1122484u32, v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e843; continue;
 }
 0x102e843 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102e858; continue; }
  pc = 0x102e84b; continue;
 }
 0x102e84b => {
  pc = 0x102e935; continue;
 }
 0x102e850 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x102e843; continue;
 }
 0x102e858 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11daca8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11dacacu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4100u32)); }
  if (cpu.sf != cpu.of) { pc = 0x102e88c; continue; }
  pc = 0x102e881; continue;
 }
 0x102e881 => {
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4108u32)); }
  if (cpu.sf != cpu.of) { pc = 0x102e88e; continue; }
  pc = 0x102e88c; continue;
 }
 0x102e88c => {
  pc = 0x102e896; continue;
 }
 0x102e88e => {
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x102e89b; continue; }
  pc = 0x102e896; continue;
 }
 0x102e896 => {
  pc = 0x102e914; continue;
 }
 0x102e89b => {
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4100u32)); }
  if (cpu.sf == cpu.of) { pc = 0x102e8ae; continue; }
  pc = 0x102e8a6; continue;
 }
 0x102e8a6 => {
  { let v: u32 = cpu.r32(0x10e4100u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e8ae; continue;
 }
 0x102e8ae => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4108u32)); }
  if (cpu.sf != cpu.of) { pc = 0x102e8c2; continue; }
  pc = 0x102e8b9; continue;
 }
 0x102e8b9 => {
  { let v: u32 = cpu.r32(0x10e4108u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102e8c2; continue;
 }
 0x102e8c2 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1122470u32, v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1122474u32, v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x102e914); f_0102da70(cpu); if cpu.returned_to != 0x102e914 { return; }
  pc = 0x102e914; continue;
 }
 0x102e905 => {
  if cpu.zf { pc = 0x102e92b; continue; }
  pc = 0x102e907; continue;
 }
 0x102e907 => {
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x102e914); f_0102da70(cpu); if cpu.returned_to != 0x102e914 { return; }
  pc = 0x102e914; continue;
 }
 0x102e914 => {
  { let v: u8 = cpu.shl8(cpu.r8(0x1122484u32), 1u32); cpu.w8(0x1122484u32, v); }
  { cpu.sub8(cpu.r8(0x1122484u32), 0x10u8); }
  if !cpu.zf { pc = 0x102e930; continue; }
  pc = 0x102e923; continue;
 }
 0x102e923 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1122480u32)); cpu.w32(0x1122480u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1122484u32, v); }
  pc = 0x102e930; continue;
 }
 0x102e92b => {
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))), ((cpu.eax >> 8) as u8)); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x102e930; continue;
 }
 0x102e930 => {
  cpu.poll(); pc = 0x102e850; continue;
 }
 0x102e935 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102e93c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102e93c); }
 let mut pc: u32 = 0x102e93c;
 loop { match pc {
 0x102e93c => {
  { let v: u32 = 0x0u32; cpu.w32(0x1105774u32, v); }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x102e950; continue;
 }
 0x102e950 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x102e95d; continue; }
  pc = 0x102e957; continue;
 }
 0x102e957 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1105774u32)); cpu.w32(0x1105774u32, v); }
  pc = 0x102e95d; continue;
 }
 0x102e95d => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102e950; continue; }
  pc = 0x102e963; continue;
 }
 0x102e963 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102e964(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102e964); }
 let mut pc: u32 = 0x102e964;
 loop { match pc {
 0x102e964 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x110eab8u32), 0x88u32); }
  if (cpu.sf == cpu.of) { pc = 0x102e9d5; continue; }
  pc = 0x102e971; continue;
 }
 0x102e971 => {
  { let v: u32 = cpu.r32(0x10e3f7cu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10fcc0eu32, v); }
  { let v: u32 = 0xddu32; cpu.w32(0x110eab8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x110eac0u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x102e9b3; continue; }
  pc = 0x102e992; continue;
 }
 0x102e992 => {
  { let v: u32 = 0x110edaau32; cpu.w32(0x110eabcu32, v); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x102e9d2; continue; }
  pc = 0x102e9a5; continue;
 }
 0x102e9a5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x102e9b0); f_010363b2(cpu); if cpu.returned_to != 0x102e9b0 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x102e9d2; continue;
 }
 0x102e9b3 => {
  { let v: u32 = 0x110edaeu32; cpu.w32(0x110eabcu32, v); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x102e9d2; continue; }
  pc = 0x102e9c6; continue;
 }
 0x102e9c6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x102e9d1); f_010363b2(cpu); if cpu.returned_to != 0x102e9d1 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x102e9d2; continue;
 }
 0x102e9d2 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x102e9d5 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102e9d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102e9d8); }
 let mut pc: u32 = 0x102e9d8;
 loop { match pc {
 0x102e9d8 => {
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x102e9e2; continue;
 }
 0x102e9e2 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x102e9ee; continue; }
  pc = 0x102e9e9; continue;
 }
 0x102e9e9 => {
  cpu.push(0x102e9ee); f_0102e9f5(cpu); if cpu.returned_to != 0x102e9ee { return; }
  pc = 0x102e9ee; continue;
 }
 0x102e9ee => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102e9e2; continue; }
  pc = 0x102e9f4; continue;
 }
 0x102e9f4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102e9f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102e9f5); }
 let mut pc: u32 = 0x102e9f5;
 loop { match pc {
 0x102e9f5 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x102ea85; continue; }
  pc = 0x102ea02; continue;
 }
 0x102ea02 => {
  { cpu.sub8(cpu.r8(0x4fu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x102ea85; continue; }
  pc = 0x102ea08; continue;
 }
 0x102ea08 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4fu32.wrapping_add(cpu.ebp))); cpu.w8(0x4fu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x4fu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x102ea85; continue; }
  pc = 0x102ea11; continue;
 }
 0x102ea11 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x64u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = 0xau8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10fcde6u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4fu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x64u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.r8(0x10fce4au32.wrapping_add(cpu.edx)); cpu.mul8(v); }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)).wrapping_sub((cpu.eax as u16)); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x102ea67; continue; }
  pc = 0x102ea5f; continue;
 }
 0x102ea5f => {
  if !cpu.sf { pc = 0x102ea85; continue; }
  pc = 0x102ea61; continue;
 }
 0x102ea61 => {
  { let v: u16 = 0x0u16; cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x102ea67; continue;
 }
 0x102ea67 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x102ea6d); f_0102ea87(cpu); if cpu.returned_to != 0x102ea6d { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x110eab4u32), cpu.ebp); }
  if !cpu.zf { pc = 0x102ea85; continue; }
  pc = 0x102ea76; continue;
 }
 0x102ea76 => {
  { let v: u32 = 0x0u32; cpu.w32(0x110eab4u32, v); }
  cpu.push(0x102ea85); f_010863ff(cpu); if cpu.returned_to != 0x102ea85 { return; }
  pc = 0x102ea85; continue;
 }
 0x102ea85 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102ea87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102ea87); }
 let mut pc: u32 = 0x102ea87;
 loop { match pc {
 0x102ea87 => {
  { let v: u8 = 0x1u8; cpu.w8(0x110d5d2u32, v); }
  { cpu.sub32(cpu.r32(0x110eab4u32), cpu.ebp); }
  if !cpu.zf { pc = 0x102eaa0; continue; }
  pc = 0x102ea96; continue;
 }
 0x102ea96 => {
  { let v: u32 = 0x0u32; cpu.w32(0x110eab4u32, v); }
  pc = 0x102eaa0; continue;
 }
 0x102eaa0 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x102eab2; continue; }
  pc = 0x102eaa6; continue;
 }
 0x102eaa6 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x102eab2 => {
  { let v: u32 = 0x4au32; cpu.eax = v; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x10c6120u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u32 = cpu.r32(0x1122a08u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x102eaed; continue; }
  pc = 0x102eae9; continue;
 }
 0x102eae9 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x102eaed; continue;
 }
 0x102eaed => {
  cpu.push(0x102eaf2); f_0102f786(cpu); if cpu.returned_to != 0x102eaf2 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r16(0x10e3e97u32) as u32; cpu.ecx = v; }
  let t = cpu.r32(0x10302d1u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.push(0x102eb01); cpu.call(t); if cpu.returned_to != 0x102eb01 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x102eb0a); f_010302f5(cpu); if cpu.returned_to != 0x102eb0a { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4bu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x102eb18); f_0102f8be(cpu); if cpu.returned_to != 0x102eb18 { return; }
  { let v: u32 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x102eb2f; continue; }
  pc = 0x102eb22; continue;
 }
 0x102eb22 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  cpu.push(0x102eb2e); f_010504cc(cpu); if cpu.returned_to != 0x102eb2e { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x102eb2f; continue;
 }
 0x102eb2f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x10u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, 0x10u32); cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11bd38bu32, v); }
  cpu.push(0x102eb67); f_010538dc(cpu); if cpu.returned_to != 0x102eb67 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102eb68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102eb68); }
 let mut pc: u32 = 0x102eb68;
 loop { match pc {
 0x102eb68 => {
  { let v: u32 = cpu.r32(0x10e3f80u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x384u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e3f80u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102efca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102efca); }
 let mut pc: u32 = 0x102efca;
 loop { match pc {
 0x102efca => {
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x102efd4; continue;
 }
 0x102efd4 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x102f02e; continue; }
  pc = 0x102efdb; continue;
 }
 0x102efdb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x10c6120u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x102effc; continue; }
  pc = 0x102eff2; continue;
 }
 0x102eff2 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x102f016; continue; }
  pc = 0x102eff8; continue;
 }
 0x102eff8 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x102f02e; continue;
 }
 0x102effc => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x609u32; cpu.ecx = v; }
  cpu.push(0x102f012); f_0102f035(cpu); if cpu.returned_to != 0x102f012 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x102f02e; continue;
 }
 0x102f016 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x608u32; cpu.ecx = v; }
  cpu.push(0x102f02c); f_0102f035(cpu); if cpu.returned_to != 0x102f02c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x102f02e; continue;
 }
 0x102f02e => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102efd4; continue; }
  pc = 0x102f034; continue;
 }
 0x102f034 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102f035(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102f035); }
 let mut pc: u32 = 0x102f035;
 loop { match pc {
 0x102f035 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.sf != cpu.of) { pc = 0x102f06d; continue; }
  pc = 0x102f042; continue;
 }
 0x102f042 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if (cpu.sf != cpu.of) { pc = 0x102f06d; continue; }
  pc = 0x102f04f; continue;
 }
 0x102f04f => {
  { let v: u16 = cpu.r16(0x10e4104u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x102f06d; continue; }
  pc = 0x102f05b; continue;
 }
 0x102f05b => {
  { let v: u16 = cpu.r16(0x10e4108u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x102f06d; continue; }
  pc = 0x102f067; continue;
 }
 0x102f067 => {
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x102f06d); cpu.call(t); if cpu.returned_to != 0x102f06d { return; }
  pc = 0x102f06d; continue;
 }
 0x102f06d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102f06e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102f06e); }
 let mut pc: u32 = 0x102f06e;
 loop { match pc {
 0x102f06e => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1105e5au32), 0x4u8); }
  if cpu.zf { pc = 0x102f0a2; continue; }
  pc = 0x102f07c; continue;
 }
 0x102f07c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1105e5au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1105e55u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 3u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x102f098; continue; }
  pc = 0x102f091; continue;
 }
 0x102f091 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  if cpu.zf { pc = 0x102f0a2; continue; }
  pc = 0x102f098; continue;
 }
 0x102f098 => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102f0a2; continue; }
  pc = 0x102f09d; continue;
 }
 0x102f09d => {
  { let v: u32 = 0xau32; cpu.eax = v; }
  pc = 0x102f0a2; continue;
 }
 0x102f0a2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102f0a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102f0a3); }
 let mut pc: u32 = 0x102f0a3;
 loop { match pc {
 0x102f0a3 => {
  cpu.push(0x102f0a8); f_01046bb0(cpu); if cpu.returned_to != 0x102f0a8 { return; }
  cpu.push(0x102f0ad); f_01031d3a(cpu); if cpu.returned_to != 0x102f0ad { return; }
  cpu.push(0x102f0b2); f_0103259d(cpu); if cpu.returned_to != 0x102f0b2 { return; }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x102f0bc; continue;
 }
 0x102f0bc => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x102f5a8; continue; }
  pc = 0x102f0c7; continue;
 }
 0x102f0c7 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x102f1fc; continue; }
  pc = 0x102f0d3; continue;
 }
 0x102f0d3 => {
  { cpu.sub32(cpu.r32(0x45u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if cpu.zf { pc = 0x102f1fc; continue; }
  pc = 0x102f0e0; continue;
 }
 0x102f0e0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x1105e5au32), 0x4u8); }
  if cpu.zf { pc = 0x102f139; continue; }
  pc = 0x102f0f4; continue;
 }
 0x102f0f4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1105e5au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x102f114; continue; }
  pc = 0x102f104; continue;
 }
 0x102f104 => {
  { cpu.sub8(cpu.r8(0x1105e55u32.wrapping_add(cpu.eax)), 0xcu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102f139; continue; }
  pc = 0x102f10d; continue;
 }
 0x102f10d => {
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  pc = 0x102f122; continue;
 }
 0x102f114 => {
  { cpu.sub8(cpu.r8(0x1105e55u32.wrapping_add(cpu.eax)), 0x19u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102f139; continue; }
  pc = 0x102f11d; continue;
 }
 0x102f11d => {
  { let v: u32 = 0x19u32; cpu.ecx = v; }
  pc = 0x102f122; continue;
 }
 0x102f122 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1105e55u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4bu32); }
  if (cpu.sf != cpu.of) { pc = 0x102f136; continue; }
  pc = 0x102f131; continue;
 }
 0x102f131 => {
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  pc = 0x102f136; continue;
 }
 0x102f136 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x64u32); cpu.ebx = v; }
  pc = 0x102f139; continue;
 }
 0x102f139 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u16 = 0x1eu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x45u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x102f15d; continue; }
  pc = 0x102f153; continue;
 }
 0x102f153 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x102f15c); f_01015682(cpu); if cpu.returned_to != 0x102f15c { return; }
  { cpu.popad(); }
  pc = 0x102f15d; continue;
 }
 0x102f15d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x1105e5au32), 0x4u8); }
  if cpu.zf { pc = 0x102f1b2; continue; }
  pc = 0x102f16d; continue;
 }
 0x102f16d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1105e5au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x102f18d; continue; }
  pc = 0x102f17d; continue;
 }
 0x102f17d => {
  { cpu.sub8(cpu.r8(0x1105e55u32.wrapping_add(cpu.eax)), 0xcu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102f1b2; continue; }
  pc = 0x102f186; continue;
 }
 0x102f186 => {
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  pc = 0x102f19b; continue;
 }
 0x102f18d => {
  { cpu.sub8(cpu.r8(0x1105e55u32.wrapping_add(cpu.eax)), 0x19u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102f1b2; continue; }
  pc = 0x102f196; continue;
 }
 0x102f196 => {
  { let v: u32 = 0x19u32; cpu.ecx = v; }
  pc = 0x102f19b; continue;
 }
 0x102f19b => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1105e55u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4bu32); }
  if (cpu.sf != cpu.of) { pc = 0x102f1af; continue; }
  pc = 0x102f1aa; continue;
 }
 0x102f1aa => {
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  pc = 0x102f1af; continue;
 }
 0x102f1af => {
  { let v: u32 = cpu.ebx.wrapping_add(0x64u32); cpu.ebx = v; }
  pc = 0x102f1b2; continue;
 }
 0x102f1b2 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x45u32.wrapping_add(cpu.ebp)).wrapping_sub(cpu.ebx); cpu.w32(0x45u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x45u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.sf { pc = 0x102f1fc; continue; }
  pc = 0x102f1bd; continue;
 }
 0x102f1bd => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x102f1c6); f_0102f715(cpu); if cpu.returned_to != 0x102f1c6 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x49u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if !cpu.zf { pc = 0x102f1d9; continue; }
  pc = 0x102f1d2; continue;
 }
 0x102f1d2 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x102f1f7; continue;
 }
 0x102f1d9 => {
  { let v: u16 = 0x11u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x64u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x102f1f7; continue;
 }
 0x102f1f7 => {
  { let v: u32 = cpu.edx; cpu.w32(0x45u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x102f1fc; continue;
 }
 0x102f1fc => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1105e5au32, v); }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x102f310; continue; }
  pc = 0x102f214; continue;
 }
 0x102f214 => {
  { cpu.sub32(cpu.r32(0x11b5420u32), 0x0u32); }
  if !cpu.zf { pc = 0x102f310; continue; }
  pc = 0x102f221; continue;
 }
 0x102f221 => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x102f310; continue; }
  pc = 0x102f22f; continue;
 }
 0x102f22f => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x102f2d4; continue; }
  pc = 0x102f239; continue;
 }
 0x102f239 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x102f310; continue; }
  pc = 0x102f246; continue;
 }
 0x102f246 => {
  { let v: u32 = 0x10e3d2bu32; cpu.esi = v; }
  { let v: u32 = 0x10e3d2du32; cpu.edi = v; }
  { let v: u32 = 0x115f5b8u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x1105e5au32), 0x1u8); }
  if cpu.zf { pc = 0x102f2a8; continue; }
  pc = 0x102f25e; continue;
 }
 0x102f25e => {
  { let v: u32 = 0x10e3d2fu32; cpu.esi = v; }
  { let v: u32 = 0x10e3d31u32; cpu.edi = v; }
  { let v: u32 = 0x115f93cu32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x1105e5au32), 0x0u8); }
  if cpu.zf { pc = 0x102f2a8; continue; }
  pc = 0x102f276; continue;
 }
 0x102f276 => {
  { let v: u32 = 0x10e3d33u32; cpu.esi = v; }
  { let v: u32 = 0x10e3d35u32; cpu.edi = v; }
  { let v: u32 = 0x115fcc0u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x1105e5au32), 0x2u8); }
  if cpu.zf { pc = 0x102f2a8; continue; }
  pc = 0x102f28e; continue;
 }
 0x102f28e => {
  { let v: u32 = 0x10e3d37u32; cpu.esi = v; }
  { let v: u32 = 0x10e3d39u32; cpu.edi = v; }
  { let v: u32 = 0x1160044u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x1105e5au32), 0x3u8); }
  if cpu.zf { pc = 0x102f2a8; continue; }
  pc = 0x102f2a6; continue;
 }
 0x102f2a6 => {
  pc = 0x102f310; continue;
 }
 0x102f2a8 => {
  cpu.push(0x102f2ad); f_0102f06e(cpu); if cpu.returned_to != 0x102f2ad { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), (cpu.ecx as u16)); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { pc = 0x102f2cc; continue; }
  pc = 0x102f2ba; continue;
 }
 0x102f2ba => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  cpu.push(0x102f2ca); f_01072cb4(cpu); if cpu.returned_to != 0x102f2ca { return; }
  pc = 0x102f310; continue;
 }
 0x102f2cc => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x102f310; continue;
 }
 0x102f2d4 => {
  { cpu.sub32(cpu.r32(0x45u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if cpu.zf { pc = 0x102f310; continue; }
  pc = 0x102f2dd; continue;
 }
 0x102f2dd => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x102f310; continue; }
  pc = 0x102f2e6; continue;
 }
 0x102f2e6 => {
  cpu.push(0x102f2eb); f_0102f06e(cpu); if cpu.returned_to != 0x102f2eb { return; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u16)); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102f2fd; continue; }
  pc = 0x102f2f9; continue;
 }
 0x102f2f9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x102f2fd; continue;
 }
 0x102f2fd => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102f310); f_01072cb4(cpu); if cpu.returned_to != 0x102f310 { return; }
  pc = 0x102f310; continue;
 }
 0x102f310 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x102f358; continue; }
  pc = 0x102f319; continue;
 }
 0x102f319 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x102f358; continue; }
  pc = 0x102f322; continue;
 }
 0x102f322 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x102f33d; continue; }
  pc = 0x102f32a; continue;
 }
 0x102f32a => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1122890u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1122890u32).wrapping_add(0x4u32); cpu.w32(0x1122890u32, v); }
  pc = 0x102f358; continue;
 }
 0x102f33d => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1cu32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102f358; continue; }
  pc = 0x102f347; continue;
 }
 0x102f347 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x112288cu32); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x112288cu32).wrapping_add(0x4u32); cpu.w32(0x112288cu32, v); }
  pc = 0x102f358; continue;
 }
 0x102f358 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x34u32.wrapping_add(cpu.ebp))); cpu.w16(0x34u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x102f367; continue; }
  pc = 0x102f361; continue;
 }
 0x102f361 => {
  { let v: u16 = 0x0u16; cpu.w16(0x34u32.wrapping_add(cpu.ebp), v); }
  pc = 0x102f367; continue;
 }
 0x102f367 => {
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x10c6120u32.wrapping_add(cpu.eax); cpu.esi = v; }
  let t = cpu.r32(0x3au32.wrapping_add(cpu.esi)); cpu.push(0x102f379); cpu.call(t); if cpu.returned_to != 0x102f379 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x11cc2b7u32), 0x0u8); }
  if !cpu.zf { pc = 0x102f3a1; continue; }
  pc = 0x102f384; continue;
 }
 0x102f384 => {
  { cpu.sub8(cpu.r8(0x1105e54u32), 0x0u8); }
  if cpu.zf { pc = 0x102f4e2; continue; }
  pc = 0x102f391; continue;
 }
 0x102f391 => {
  { cpu.sub32(cpu.r32(0x45u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if cpu.zf { pc = 0x102f4e2; continue; }
  pc = 0x102f39e; continue;
 }
 0x102f39e => {
  { cpu.pushad(); }
  pc = 0x102f3db; continue;
 }
 0x102f3a1 => {
  { cpu.sub16(cpu.r16(0x49u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x102f4e2; continue; }
  pc = 0x102f3ad; continue;
 }
 0x102f3ad => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x49u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x1105e5au32, v); }
  { let v: u16 = 0x11u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x64u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x102f3f0; continue;
 }
 0x102f3db => {
  { let v: u32 = cpu.r32(0x45u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.sf { pc = 0x102f4e1; continue; }
  pc = 0x102f3e7; continue;
 }
 0x102f3e7 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x102f4e1; continue; }
  pc = 0x102f3f0; continue;
 }
 0x102f3f0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x1105e5au32), 0x4u8); }
  if cpu.zf { pc = 0x102f445; continue; }
  pc = 0x102f400; continue;
 }
 0x102f400 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1105e5au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x102f420; continue; }
  pc = 0x102f410; continue;
 }
 0x102f410 => {
  { cpu.sub8(cpu.r8(0x1105e55u32.wrapping_add(cpu.eax)), 0xcu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102f445; continue; }
  pc = 0x102f419; continue;
 }
 0x102f419 => {
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  pc = 0x102f42e; continue;
 }
 0x102f420 => {
  { cpu.sub8(cpu.r8(0x1105e55u32.wrapping_add(cpu.eax)), 0x19u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x102f445; continue; }
  pc = 0x102f429; continue;
 }
 0x102f429 => {
  { let v: u32 = 0x19u32; cpu.ecx = v; }
  pc = 0x102f42e; continue;
 }
 0x102f42e => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1105e55u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4bu32); }
  if (cpu.sf != cpu.of) { pc = 0x102f442; continue; }
  pc = 0x102f43d; continue;
 }
 0x102f43d => {
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  pc = 0x102f442; continue;
 }
 0x102f442 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x64u32); cpu.ebx = v; }
  pc = 0x102f445; continue;
 }
 0x102f445 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x28u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x102f4b1; continue; }
  pc = 0x102f489; continue;
 }
 0x102f489 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if cpu.sf { pc = 0x102f4b1; continue; }
  pc = 0x102f48f; continue;
 }
 0x102f48f => {
  { cpu.sub16((cpu.eax as u16), 0x100u16); }
  if (cpu.sf == cpu.of) { pc = 0x102f4b1; continue; }
  pc = 0x102f495; continue;
 }
 0x102f495 => {
  { cpu.sub16((cpu.ebx as u16), 0xaau16); }
  if (cpu.sf == cpu.of) { pc = 0x102f4b1; continue; }
  pc = 0x102f49c; continue;
 }
 0x102f49c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0xffu32; cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x102f4ae); f_01059c7e(cpu); if cpu.returned_to != 0x102f4ae { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x102f4b1; continue;
 }
 0x102f4b1 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0x7fu32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0xffu32; cpu.ebp = v; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x102f4e1; continue; }
  pc = 0x102f4c9; continue;
 }
 0x102f4c9 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if cpu.sf { pc = 0x102f4e1; continue; }
  pc = 0x102f4cf; continue;
 }
 0x102f4cf => {
  { cpu.sub16((cpu.eax as u16), 0x100u16); }
  if (cpu.sf == cpu.of) { pc = 0x102f4e1; continue; }
  pc = 0x102f4d5; continue;
 }
 0x102f4d5 => {
  { cpu.sub16((cpu.ebx as u16), 0xaau16); }
  if (cpu.sf == cpu.of) { pc = 0x102f4e1; continue; }
  pc = 0x102f4dc; continue;
 }
 0x102f4dc => {
  cpu.push(0x102f4e1); f_01059c7e(cpu); if cpu.returned_to != 0x102f4e1 { return; }
  pc = 0x102f4e1; continue;
 }
 0x102f4e1 => {
  { cpu.popad(); }
  pc = 0x102f4e2; continue;
 }
 0x102f4e2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x10ef376u32), 0x0u8); }
  if cpu.zf { pc = 0x102f5a6; continue; }
  pc = 0x102f4f1; continue;
 }
 0x102f4f1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x102f4ff); f_01059d5a(cpu); if cpu.returned_to != 0x102f4ff { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x102f5a6; continue; }
  pc = 0x102f511; continue;
 }
 0x102f511 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x102f5a6; continue; }
  pc = 0x102f522; continue;
 }
 0x102f522 => {
  { let v: u16 = cpu.r16(0x10e4104u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x30u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x102f5a6; continue; }
  pc = 0x102f532; continue;
 }
 0x102f532 => {
  { let v: u16 = cpu.r16(0x10e4108u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x10u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.ecx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x102f5a6; continue; }
  pc = 0x102f542; continue;
 }
 0x102f542 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x2fu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0xfu16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.ebp = v; }
  let t = cpu.r32(0x10e4318u32); cpu.push(0x102f55a); cpu.call(t); if cpu.returned_to != 0x102f55a { return; }
  { cpu.popad(); }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x10ef397u32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  cpu.push(0x102f56c); f_01034211(cpu); if cpu.returned_to != 0x102f56c { return; }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x102f576); f_01031c81(cpu); if cpu.returned_to != 0x102f576 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x102f58e; continue; }
  pc = 0x102f57c; continue;
 }
 0x102f57c => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x102f58e; continue;
 }
 0x102f58e => {
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x102f599); f_01059d5a(cpu); if cpu.returned_to != 0x102f599 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  cpu.push(0x102f5a6); f_01034211(cpu); if cpu.returned_to != 0x102f5a6 { return; }
  pc = 0x102f5a6; continue;
 }
 0x102f5a6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x102f5a8; continue;
 }
 0x102f5a8 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102f0bc; continue; }
  pc = 0x102f5b2; continue;
 }
 0x102f5b2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102f5b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102f5b3); }
 let mut pc: u32 = 0x102f5b3;
 loop { match pc {
 0x102f5b3 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x102f610; continue; }
  pc = 0x102f5bc; continue;
 }
 0x102f5bc => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x3e8u32; cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11230c2u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1122fc7u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xffffu16); }
  if cpu.zf { pc = 0x102f610; continue; }
  pc = 0x102f5ec; continue;
 }
 0x102f5ec => {
  { let v: u16 = cpu.r16(0x1122fc5u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x102f610); cpu.call(t); if cpu.returned_to != 0x102f610 { return; }
  pc = 0x102f610; continue;
 }
 0x102f610 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102f612(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102f612); }
 let mut pc: u32 = 0x102f612;
 loop { match pc {
 0x102f612 => {
  { let v: u32 = 0x11603c8u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e3f80u32, v); }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x102f622; continue;
 }
 0x102f622 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x102f628); f_0107281d(cpu); if cpu.returned_to != 0x102f628 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102f622; continue; }
  pc = 0x102f62b; continue;
 }
 0x102f62b => {
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x102f635; continue;
 }
 0x102f635 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x102f709; continue; }
  pc = 0x102f640; continue;
 }
 0x102f640 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x10c6120u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x38u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x33u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x34u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) & 0xe0u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x37u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x49u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if !cpu.zf { pc = 0x102f6d1; continue; }
  pc = 0x102f6ca; continue;
 }
 0x102f6ca => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x102f6ef; continue;
 }
 0x102f6d1 => {
  { let v: u16 = 0x11u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x64u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x102f6ef; continue;
 }
 0x102f6ef => {
  { let v: u32 = cpu.edx; cpu.w32(0x45u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x102f6f8); f_0102eb68(cpu); if cpu.returned_to != 0x102f6f8 { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x16u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  let t = cpu.r32(0x36u32.wrapping_add(cpu.esi)); cpu.push(0x102f702); cpu.call(t); if cpu.returned_to != 0x102f702 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x102f709); f_0102f745(cpu); if cpu.returned_to != 0x102f709 { return; }
  pc = 0x102f709; continue;
 }
 0x102f709 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102f635; continue; }
  pc = 0x102f714; continue;
 }
 0x102f714 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102f715(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102f715); }
 let mut pc: u32 = 0x102f715;
 loop { match pc {
 0x102f715 => {
  { cpu.pushad(); }
  cpu.push(0x102f71b); f_01015954(cpu); if cpu.returned_to != 0x102f71b { return; }
  if cpu.cf { pc = 0x102f743; continue; }
  pc = 0x102f71d; continue;
 }
 0x102f71d => {
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x2eu32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xau32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x102f743; continue;
 }
 0x102f743 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102f745(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102f745); }
 let mut pc: u32 = 0x102f745;
 loop { match pc {
 0x102f745 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x102f753); f_0108c1c5(cpu); if cpu.returned_to != 0x102f753 { return; }
  if cpu.cf { pc = 0x102f758; continue; }
  pc = 0x102f755; continue;
 }
 0x102f755 => {
  { let v: u32 = cpu.esi; cpu.w32(0x1eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x102f758; continue;
 }
 0x102f758 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102f75b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102f75b); }
 let mut pc: u32 = 0x102f75b;
 loop { match pc {
 0x102f75b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x4au32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x10c6120u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x11228f8u32; cpu.esi = v; }
  cpu.push(0x102f785); f_0102f786(cpu); if cpu.returned_to != 0x102f785 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102f786(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102f786); }
 let mut pc: u32 = 0x102f786;
 loop { match pc {
 0x102f786 => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.esi, 0x11228f8u32); }
  if cpu.zf { pc = 0x102f7a3; continue; }
  pc = 0x102f78f; continue;
 }
 0x102f78f => {
  { cpu.sub32(cpu.esi, 0x1122a2cu32); }
  if cpu.zf { pc = 0x102f7a3; continue; }
  pc = 0x102f797; continue;
 }
 0x102f797 => {
  { cpu.sub32(cpu.esi, 0x11228e4u32); }
  if cpu.zf { pc = 0x102f7a3; continue; }
  pc = 0x102f79f; continue;
 }
 0x102f79f => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x102f7a3; continue;
 }
 0x102f7a3 => {
  cpu.push(0x102f7a8); f_0104f36d(cpu); if cpu.returned_to != 0x102f7a8 { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4000u32); cpu.edi = v; }
  pc = 0x102f7b1; continue;
 }
 0x102f7b1 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x102f7b4; continue;
 }
 0x102f7b4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102f7b4; continue; }
  pc = 0x102f7bf; continue;
 }
 0x102f7bf => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x102f7b1; continue; }
  pc = 0x102f7ca; continue;
 }
 0x102f7ca => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102f7cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102f7cb); }
 let mut pc: u32 = 0x102f7cb;
 loop { match pc {
 0x102f7cb => {
  { let v = cpu.r32(0x112ad0cu32); cpu.push(v); }
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x112c96bu32), 0x0u8); }
  if !cpu.zf { pc = 0x102f7e3; continue; }
  pc = 0x102f7db; continue;
 }
 0x102f7db => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x102f7e3; continue;
 }
 0x102f7e3 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112ad0cu32, v); }
  cpu.push(0x102f7f0); f_01018e6e(cpu); if cpu.returned_to != 0x102f7f0 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if cpu.cf { pc = 0x102f7fb; continue; }
  pc = 0x102f7f3; continue;
 }
 0x102f7f3 => {
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.w32(0x112ad0cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x102f7fb => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e413cu32, v); }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { cpu.poll(); pc = 0x102f7f3; continue; }
  pc = 0x102f80f; continue;
 }
 0x102f80f => {
  { cpu.sub8(cpu.r8(0x4fu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x102f848; continue; }
  pc = 0x102f815; continue;
 }
 0x102f815 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x64u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = 0xau8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10fcde6u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4fu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x102f848; continue;
 }
 0x102f848 => {
  { let v: u32 = (cpu.edx as u8) as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x102f855; continue; }
  pc = 0x102f853; continue;
 }
 0x102f853 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x102f855; continue;
 }
 0x102f855 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x102f7f3; continue; }
  pc = 0x102f85d; continue;
 }
 0x102f85d => {
  { cpu.sub8(cpu.r8(0x112c967u32), 0x0u8); }
  if !cpu.zf { pc = 0x102f884; continue; }
  pc = 0x102f866; continue;
 }
 0x102f866 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  cpu.push(0x102f881); f_010362a4(cpu); if cpu.returned_to != 0x102f881 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x102f884; continue;
 }
 0x102f884 => {
  { cpu.pushad(); }
  cpu.push(0x102f88a); f_0102ea87(cpu); if cpu.returned_to != 0x102f88a { return; }
  { cpu.popad(); }
  cpu.poll(); pc = 0x102f7f3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102f890(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102f890); }
 let mut pc: u32 = 0x102f890;
 loop { match pc {
 0x102f890 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  cpu.push(0x102f8b1); f_0105af34(cpu); if cpu.returned_to != 0x102f8b1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x1122ff1u32), 0xau32); cpu.w32(0x1122ff1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102f8be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102f8be); }
 let mut pc: u32 = 0x102f8be;
 loop { match pc {
 0x102f8be => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x102f8c7; continue;
 }
 0x102f8c7 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x102f8dd; continue; }
  pc = 0x102f8ce; continue;
 }
 0x102f8ce => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.edi)), 0xfffeu16); }
  if !cpu.zf { pc = 0x102f8d8; continue; }
  pc = 0x102f8d6; continue;
 }
 0x102f8d6 => {
  pc = 0x102f8e0; continue;
 }
 0x102f8d8 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x12u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x102f8c7; continue; }
  pc = 0x102f8dd; continue;
 }
 0x102f8dd => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x102f8e0 => {
  { let v: u32 = cpu.edi; cpu.w32(0x16u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102f8e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102f8e6); }
 let mut pc: u32 = 0x102f8e6;
 loop { match pc {
 0x102f8e6 => {
  { let v: u32 = cpu.r8(0x10ef33au32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u32 = 0x11af104u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xau16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1122ff5u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u8 = 0xau8; cpu.w8(0x1122ff9u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1122ff1u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3du16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb44u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102f96f); f_0102f890(cpu); if cpu.returned_to != 0x102f96f { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3du16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb45u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102f999); f_0102f890(cpu); if cpu.returned_to != 0x102f999 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x5u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb59u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102f9c3); f_0102f890(cpu); if cpu.returned_to != 0x102f9c3 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb5bu16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102f9ed); f_0102f890(cpu); if cpu.returned_to != 0x102f9ed { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb5eu16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fa17); f_0102f890(cpu); if cpu.returned_to != 0x102fa17 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fa41); f_0102f890(cpu); if cpu.returned_to != 0x102fa41 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fa6b); f_0102f890(cpu); if cpu.returned_to != 0x102fa6b { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fa95); f_0102f890(cpu); if cpu.returned_to != 0x102fa95 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fabf); f_0102f890(cpu); if cpu.returned_to != 0x102fabf { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fae9); f_0102f890(cpu); if cpu.returned_to != 0x102fae9 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102faf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102faf0); }
 let mut pc: u32 = 0x102faf0;
 loop { match pc {
 0x102faf0 => {
  { let v: u32 = cpu.r8(0x10ef33au32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u32 = 0x11ac904u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xcu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1122ff5u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u8 = 0xcu8; cpu.w8(0x1122ff9u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1122ff1u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x847u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fb79); f_0102f890(cpu); if cpu.returned_to != 0x102fb79 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x83eu16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fba3); f_0102f890(cpu); if cpu.returned_to != 0x102fba3 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xdu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x3fu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x843u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fbcd); f_0102f890(cpu); if cpu.returned_to != 0x102fbcd { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x46u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x845u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fbf7); f_0102f890(cpu); if cpu.returned_to != 0x102fbf7 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x16u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x37u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x841u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fc21); f_0102f890(cpu); if cpu.returned_to != 0x102fc21 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xfu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x833u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fc4b); f_0102f890(cpu); if cpu.returned_to != 0x102fc4b { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fc75); f_0102f890(cpu); if cpu.returned_to != 0x102fc75 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fc9f); f_0102f890(cpu); if cpu.returned_to != 0x102fc9f { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fcc9); f_0102f890(cpu); if cpu.returned_to != 0x102fcc9 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fcf3); f_0102f890(cpu); if cpu.returned_to != 0x102fcf3 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fd1d); f_0102f890(cpu); if cpu.returned_to != 0x102fd1d { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fd47); f_0102f890(cpu); if cpu.returned_to != 0x102fd47 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102fd4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102fd4e); }
 let mut pc: u32 = 0x102fd4e;
 loop { match pc {
 0x102fd4e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102fd4f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x102fd4f); }
 let mut pc: u32 = 0x102fd4f;
 loop { match pc {
 0x102fd4f => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x69cu16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x14u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x14u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x102fd89); f_0102f890(cpu); if cpu.returned_to != 0x102fd89 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1eu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6a8u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fd9b); f_0102f890(cpu); if cpu.returned_to != 0x102fd9b { return; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x1eu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6b4u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fdad); f_0102f890(cpu); if cpu.returned_to != 0x102fdad { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x19u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6c0u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fdc3); f_0102f890(cpu); if cpu.returned_to != 0x102fdc3 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0102fdc4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102fdc4); } r_0102fdc4(cpu, 0x102fdc4); }
pub fn f_0102ff00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102ff00); } r_0102fdc4(cpu, 0x102ff00); }
pub fn f_0102ff02(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102ff02); } r_0102fdc4(cpu, 0x102ff02); }
pub fn f_0102ffff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x102ffff); } r_0102fdc4(cpu, 0x102ffff); }
pub fn f_01030001(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1030001); } r_0102fdc4(cpu, 0x1030001); }
pub fn f_01030002(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1030002); } r_0102fdc4(cpu, 0x1030002); }
pub fn f_01030003(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1030003); } r_0102fdc4(cpu, 0x1030003); }
fn r_0102fdc4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x102fdc4 => {
  { let v: u32 = cpu.r8(0x10ef33au32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u32 = 0x11aa104u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x17u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1122ff5u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u8 = 0x17u8; cpu.w8(0x1122ff9u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1122ff1u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xdu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x1du16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb3bu16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fe4d); f_0102f890(cpu); if cpu.returned_to != 0x102fe4d { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x26u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x37u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb51u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fe77); f_0102f890(cpu); if cpu.returned_to != 0x102fe77 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x2fu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x849u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fea1); f_0102f890(cpu); if cpu.returned_to != 0x102fea1 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x82bu16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fecb); f_0102f890(cpu); if cpu.returned_to != 0x102fecb { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x83bu16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fef5); f_0102f890(cpu); if cpu.returned_to != 0x102fef5 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x102ff02; continue;
 }
 0x102ff00 => {
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.add32(cpu.r32(0xffffff83u32.wrapping_add(cpu.esi)), cpu.esp); cpu.w32(0xffffff83u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.rcr8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 102u32); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x54u32); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x84bu16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102ff1f); f_0102f890(cpu); if cpu.returned_to != 0x102ff1f { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff28); f_0102f890(cpu); if cpu.returned_to != 0x102ff28 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff31); f_0102f890(cpu); if cpu.returned_to != 0x102ff31 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff3a); f_0102f890(cpu); if cpu.returned_to != 0x102ff3a { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff43); f_0102f890(cpu); if cpu.returned_to != 0x102ff43 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff4c); f_0102f890(cpu); if cpu.returned_to != 0x102ff4c { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x22u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x1eu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb57u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102ff76); f_0102f890(cpu); if cpu.returned_to != 0x102ff76 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff7f); f_0102f890(cpu); if cpu.returned_to != 0x102ff7f { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff88); f_0102f890(cpu); if cpu.returned_to != 0x102ff88 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff91); f_0102f890(cpu); if cpu.returned_to != 0x102ff91 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff9a); f_0102f890(cpu); if cpu.returned_to != 0x102ff9a { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ffa3); f_0102f890(cpu); if cpu.returned_to != 0x102ffa3 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102ffcd); f_0102f890(cpu); if cpu.returned_to != 0x102ffcd { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fff7); f_0102f890(cpu); if cpu.returned_to != 0x102fff7 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030021); f_0102f890(cpu); if cpu.returned_to != 0x1030021 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x103004b); f_0102f890(cpu); if cpu.returned_to != 0x103004b { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x18u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x38u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030075); f_0102f890(cpu); if cpu.returned_to != 0x1030075 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x103009f); f_0102f890(cpu); if cpu.returned_to != 0x103009f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x102ff02 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x84bu16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102ff1f); f_0102f890(cpu); if cpu.returned_to != 0x102ff1f { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff28); f_0102f890(cpu); if cpu.returned_to != 0x102ff28 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff31); f_0102f890(cpu); if cpu.returned_to != 0x102ff31 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff3a); f_0102f890(cpu); if cpu.returned_to != 0x102ff3a { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff43); f_0102f890(cpu); if cpu.returned_to != 0x102ff43 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff4c); f_0102f890(cpu); if cpu.returned_to != 0x102ff4c { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x22u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x1eu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb57u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102ff76); f_0102f890(cpu); if cpu.returned_to != 0x102ff76 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff7f); f_0102f890(cpu); if cpu.returned_to != 0x102ff7f { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff88); f_0102f890(cpu); if cpu.returned_to != 0x102ff88 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff91); f_0102f890(cpu); if cpu.returned_to != 0x102ff91 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ff9a); f_0102f890(cpu); if cpu.returned_to != 0x102ff9a { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x102ffa3); f_0102f890(cpu); if cpu.returned_to != 0x102ffa3 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102ffcd); f_0102f890(cpu); if cpu.returned_to != 0x102ffcd { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x102fff7); f_0102f890(cpu); if cpu.returned_to != 0x102fff7 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030021); f_0102f890(cpu); if cpu.returned_to != 0x1030021 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x103004b); f_0102f890(cpu); if cpu.returned_to != 0x103004b { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x18u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x38u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030075); f_0102f890(cpu); if cpu.returned_to != 0x1030075 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x103009f); f_0102f890(cpu); if cpu.returned_to != 0x103009f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x102ffff => {
  { let v: u32 = cpu.sbb32(cpu.eax, 0x10e3e95u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030021); f_0102f890(cpu); if cpu.returned_to != 0x1030021 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x103004b); f_0102f890(cpu); if cpu.returned_to != 0x103004b { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x18u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x38u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030075); f_0102f890(cpu); if cpu.returned_to != 0x1030075 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x103009f); f_0102f890(cpu); if cpu.returned_to != 0x103009f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1030001 => {
  { cpu.push(cpu.segments[1] as u32); }
  pc = 0x1030003; continue;
 }
 0x1030002 => {
  { cpu.push(cpu.segments[1] as u32); }
  pc = 0x1030003; continue;
 }
 0x1030003 => {
  { let v: u32 = cpu.r32(0xffffff83u32.wrapping_add(cpu.esi)).wrapping_add(cpu.esp); cpu.w32(0xffffff83u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.rol8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 102u32); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030021); f_0102f890(cpu); if cpu.returned_to != 0x1030021 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x103004b); f_0102f890(cpu); if cpu.returned_to != 0x103004b { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x18u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x38u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030075); f_0102f890(cpu); if cpu.returned_to != 0x1030075 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x103009f); f_0102f890(cpu); if cpu.returned_to != 0x103009f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010300a6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10300a6); } r_010300a6(cpu, 0x10300a6); }
pub fn f_010300d0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10300d0); } r_010300a6(cpu, 0x10300d0); }
fn r_010300a6(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10300a6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x69cu16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x14u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x14u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10300e0); f_0102f890(cpu); if cpu.returned_to != 0x10300e0 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1eu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6a8u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10300f2); f_0102f890(cpu); if cpu.returned_to != 0x10300f2 { return; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x1eu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6b4u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030104); f_0102f890(cpu); if cpu.returned_to != 0x1030104 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x19u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6c0u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x103011a); f_0102f890(cpu); if cpu.returned_to != 0x103011a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10300d0 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x14u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x14u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10300e0); f_0102f890(cpu); if cpu.returned_to != 0x10300e0 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1eu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6a8u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10300f2); f_0102f890(cpu); if cpu.returned_to != 0x10300f2 { return; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x1eu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6b4u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030104); f_0102f890(cpu); if cpu.returned_to != 0x1030104 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x19u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6c0u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x103011a); f_0102f890(cpu); if cpu.returned_to != 0x103011a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103011b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103011b); } r_0103011b(cpu, 0x103011b); }
pub fn f_01030158(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1030158); } r_0103011b(cpu, 0x1030158); }
pub fn f_01030200(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1030200); } r_0103011b(cpu, 0x1030200); }
pub fn f_01030201(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1030201); } r_0103011b(cpu, 0x1030201); }
pub fn f_01030202(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1030202); } r_0103011b(cpu, 0x1030202); }
pub fn f_01030203(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1030203); } r_0103011b(cpu, 0x1030203); }
fn r_0103011b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x103011b => {
  { let v: u32 = cpu.r8(0x10ef33au32) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u32 = 0x11b1904u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x8u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1122ff5u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u8 = 0x8u8; cpu.w8(0x1122ff9u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1122ff1u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb3bu16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10301a4); f_0102f890(cpu); if cpu.returned_to != 0x10301a4 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x12u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x5d0u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10301ce); f_0102f890(cpu); if cpu.returned_to != 0x10301ce { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x29u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb4bu16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10301f8); f_0102f890(cpu); if cpu.returned_to != 0x10301f8 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x16u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb62u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030222); f_0102f890(cpu); if cpu.returned_to != 0x1030222 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x103024c); f_0102f890(cpu); if cpu.returned_to != 0x103024c { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030276); f_0102f890(cpu); if cpu.returned_to != 0x1030276 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10302a0); f_0102f890(cpu); if cpu.returned_to != 0x10302a0 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10302ca); f_0102f890(cpu); if cpu.returned_to != 0x10302ca { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1030158 => {
  { let v: u32 = cpu.r32(0x89662e7du32.wrapping_add(cpu.ecx)).wrapping_add(cpu.ecx); cpu.w32(0x89662e7du32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x8u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1122ff5u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u8 = 0x8u8; cpu.w8(0x1122ff9u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1122ff1u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb3bu16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10301a4); f_0102f890(cpu); if cpu.returned_to != 0x10301a4 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x12u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x5d0u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10301ce); f_0102f890(cpu); if cpu.returned_to != 0x10301ce { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x29u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb4bu16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10301f8); f_0102f890(cpu); if cpu.returned_to != 0x10301f8 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x16u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb62u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030222); f_0102f890(cpu); if cpu.returned_to != 0x1030222 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x103024c); f_0102f890(cpu); if cpu.returned_to != 0x103024c { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030276); f_0102f890(cpu); if cpu.returned_to != 0x1030276 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10302a0); f_0102f890(cpu); if cpu.returned_to != 0x10302a0 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10302ca); f_0102f890(cpu); if cpu.returned_to != 0x10302ca { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1030200 => {
  { let v: u32 = cpu.sbb32(cpu.eax, 0x10e3e95u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x16u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb62u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030222); f_0102f890(cpu); if cpu.returned_to != 0x1030222 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x103024c); f_0102f890(cpu); if cpu.returned_to != 0x103024c { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030276); f_0102f890(cpu); if cpu.returned_to != 0x1030276 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10302a0); f_0102f890(cpu); if cpu.returned_to != 0x10302a0 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10302ca); f_0102f890(cpu); if cpu.returned_to != 0x10302ca { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1030201 => {
  { let a = cpu.ebp; let b = cpu.eax; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x1030202; continue;
 }
 0x1030202 => {
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.add32(cpu.r32(0xffffff83u32.wrapping_add(cpu.esi)), cpu.esp); cpu.w32(0xffffff83u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.rcl8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 102u32); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x16u32); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb62u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030222); f_0102f890(cpu); if cpu.returned_to != 0x1030222 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x103024c); f_0102f890(cpu); if cpu.returned_to != 0x103024c { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030276); f_0102f890(cpu); if cpu.returned_to != 0x1030276 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10302a0); f_0102f890(cpu); if cpu.returned_to != 0x10302a0 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10302ca); f_0102f890(cpu); if cpu.returned_to != 0x10302ca { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1030203 => {
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.add32(cpu.r32(0xffffff83u32.wrapping_add(cpu.esi)), cpu.esp); cpu.w32(0xffffff83u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.rcl8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 102u32); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x16u32); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0xb62u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030222); f_0102f890(cpu); if cpu.returned_to != 0x1030222 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x103024c); f_0102f890(cpu); if cpu.returned_to != 0x103024c { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1030276); f_0102f890(cpu); if cpu.returned_to != 0x1030276 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4c6u16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10302a0); f_0102f890(cpu); if cpu.returned_to != 0x10302a0 { return; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = 0x4bau16; cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x10302ca); f_0102f890(cpu); if cpu.returned_to != 0x10302ca { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef33au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010302f5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10302f5); } r_010302f5(cpu, 0x10302f5); }
pub fn f_01030300(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1030300); } r_010302f5(cpu, 0x1030300); }
pub fn f_01030303(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1030303); } r_010302f5(cpu, 0x1030303); }
fn r_010302f5(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10302f5 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  cpu.push(0x10302fd); f_0101bdfe(cpu); if cpu.returned_to != 0x10302fd { return; }
  if cpu.cf { pc = 0x103039d; continue; }
  pc = 0x1030303; continue;
 }
 0x1030300 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xffffff8bu32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffff8bu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0xffffff89u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffff89u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1eu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xe2u8; cpu.w8(0x26u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x1d9u16; cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1fu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x27u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x23u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  pc = 0x1030348; continue;
 }
 0x1030303 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1eu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xe2u8; cpu.w8(0x26u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x1d9u16; cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1fu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x27u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x23u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  pc = 0x1030348; continue;
 }
 0x1030348 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x103034e); f_0101bdfe(cpu); if cpu.returned_to != 0x103034e { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x103039d; continue; }
  pc = 0x1030351; continue;
 }
 0x1030351 => {
  cpu.push(0x1030356); f_0105f178(cpu); if cpu.returned_to != 0x1030356 { return; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = 0x1fu32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1030365); f_0105f178(cpu); if cpu.returned_to != 0x1030365 { return; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xe1u8; cpu.w8(0x26u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x18bu16; cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x197u16; cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1fu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x27u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x23u32.wrapping_add(cpu.edi), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1030348; continue; }
  pc = 0x103039d; continue;
 }
 0x103039d => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103039f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103039f); }
 let mut pc: u32 = 0x103039f;
 loop { match pc {
 0x103039f => {
  { let v: u8 = 0x8u8; cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x5u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x11228e4u32; cpu.esi = v; }
  cpu.push(0x10303c8); f_0102f786(cpu); if cpu.returned_to != 0x10303c8 { return; }
  cpu.push(0x10303cd); f_0105f215(cpu); if cpu.returned_to != 0x10303cd { return; }
  { let v: u16 = (cpu.eax as u16) & 0x33u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x88u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1122d00u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x10303fd); f_01072a54(cpu); if cpu.returned_to != 0x10303fd { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010303fe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10303fe); } r_010303fe(cpu, 0x10303fe); }
pub fn f_01030500(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1030500); } r_010303fe(cpu, 0x1030500); }
pub fn f_01030503(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1030503); } r_010303fe(cpu, 0x1030503); }
pub fn f_01030507(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1030507); } r_010303fe(cpu, 0x1030507); }
fn r_010303fe(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10303fe => {
  { let v: u8 = 0x0u8; cpu.w8(0x11b4c78u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32.wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32.wrapping_add(cpu.bases[3]))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1122f8cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1122f8eu32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x1030428); cpu.push(0x103042d); f_01030880(cpu); if cpu.returned_to != 0x103042d { return; }
  { let v: u8 = 0x8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x1030443; continue; }
  pc = 0x1030438; continue;
 }
 0x1030438 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x1030443; continue; }
  pc = 0x1030441; continue;
 }
 0x1030441 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1030443; continue;
 }
 0x1030443 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x3bu32.wrapping_add(cpu.ebp))); cpu.w8(0x3bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32.wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32.wrapping_add(cpu.bases[3]))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32.wrapping_add(cpu.bases[3]))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x103050a; continue; }
  pc = 0x1030469; continue;
 }
 0x1030469 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32.wrapping_add(cpu.bases[3]))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x103050a; continue; }
  pc = 0x1030476; continue;
 }
 0x1030476 => {
  { cpu.sub16((cpu.eax as u16), 0xffb0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x103050a; continue; }
  pc = 0x1030480; continue;
 }
 0x1030480 => {
  { cpu.sub16((cpu.ebx as u16), 0xffc0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x103050a; continue; }
  pc = 0x103048a; continue;
 }
 0x103048a => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1030714; continue; }
  pc = 0x1030496; continue;
 }
 0x1030496 => {
  { let v: u32 = 0x1122894u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10f78bcu32.wrapping_add(cpu.bases[3])) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub8((cpu.edx as u8), 0x8u8); }
  if cpu.zf { pc = 0x10304b2; continue; }
  pc = 0x10304ad; continue;
 }
 0x10304ad => {
  cpu.flat_ds(0x10304ad); cpu.push(0x10304b2); f_01030af5(cpu); if cpu.returned_to != 0x10304b2 { return; }
  pc = 0x10304b2; continue;
 }
 0x10304b2 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x10304b8); cpu.push(0x10304be); cpu.call(t); if cpu.returned_to != 0x10304be { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x10304c2); cpu.push(0x10304c7); f_010308ab(cpu); if cpu.returned_to != 0x10304c7 { return; }
  cpu.flat_ds(0x10304c7); cpu.push(0x10304cc); f_01030927(cpu); if cpu.returned_to != 0x10304cc { return; }
  cpu.flat_ds(0x10304cc); cpu.push(0x10304d1); f_010309aa(cpu); if cpu.returned_to != 0x10304d1 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if (cpu.sf != cpu.of) { pc = 0x10304e3; continue; }
  pc = 0x10304df; continue;
 }
 0x10304df => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1030505; continue;
 }
 0x10304e3 => {
  { let v: u8 = cpu.r8(0x10ee950u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.flat_ds(0x10304e9); cpu.push(0x10304f0); cpu.call(t); if cpu.returned_to != 0x10304f0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11230cbu32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x10304fe); cpu.push(0x1030504); cpu.call(t); if cpu.returned_to != 0x1030504 { return; }
  { cpu.popad(); }
  pc = 0x1030505; continue;
 }
 0x1030500 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.push(cpu.segments[1] as u32); }
  pc = 0x1030503; continue;
 }
 0x1030503 => {
  { let v: u32 = cpu.add32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), cpu.esp); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  if !cpu.zf { pc = 0x103050a; continue; }
  pc = 0x1030508; continue;
 }
 0x1030505 => {
  cpu.flat_ds(0x1030505); cpu.push(0x103050a); f_0103077f(cpu); if cpu.returned_to != 0x103050a { return; }
  pc = 0x103050a; continue;
 }
 0x1030507 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffff81u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff81u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  if (cpu.sf == cpu.of) { pc = 0x103052a; continue; }
  pc = 0x103050e; continue;
 }
 0x1030508 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x103050a; continue;
 }
 0x103050a => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1030767; continue; }
  pc = 0x1030516; continue;
 }
 0x103050e => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  if cpu.zf { pc = 0x1030767; continue; }
  pc = 0x1030516; continue;
 }
 0x1030516 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x1030524); cpu.push(0x1030529); f_01072b15(cpu); if cpu.returned_to != 0x1030529 { return; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x4u8); }
  if !cpu.cf { pc = 0x1030713; continue; }
  pc = 0x1030536; continue;
 }
 0x103052a => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x4u8); }
  if !cpu.cf { pc = 0x1030713; continue; }
  pc = 0x1030536; continue;
 }
 0x1030536 => {
  { cpu.sub8(cpu.r8(0x11cc2b7u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x1030713; continue; }
  pc = 0x1030543; continue;
 }
 0x1030543 => {
  { cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x1030713; continue; }
  pc = 0x1030550; continue;
 }
 0x1030550 => {
  { let v: u16 = 0x1au16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x29u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x1030558); cpu.push(0x103055d); f_01030ba5(cpu); if cpu.returned_to != 0x103055d { return; }
  cpu.flat_ds(0x103055d); cpu.push(0x1030562); f_010307cc(cpu); if cpu.returned_to != 0x1030562 { return; }
  cpu.flat_ds(0x1030562); cpu.push(0x1030567); f_01030a43(cpu); if cpu.returned_to != 0x1030567 { return; }
  cpu.flat_ds(0x1030567); cpu.push(0x103056c); f_01031c81(cpu); if cpu.returned_to != 0x103056c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1030713; continue; }
  pc = 0x1030579; continue;
 }
 0x1030579 => {
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebp)), 0x11u8); }
  if (cpu.sf != cpu.of) { pc = 0x1030620; continue; }
  pc = 0x1030583; continue;
 }
 0x1030583 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3bu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if !cpu.zf { pc = 0x1030594; continue; }
  pc = 0x1030592; continue;
 }
 0x1030592 => {
  pc = 0x10305c4; continue;
 }
 0x1030594 => {
  { let v: u32 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x3u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x10305be; continue; }
  pc = 0x10305b2; continue;
 }
 0x10305b2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11b4c78u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1030629; continue;
 }
 0x10305be => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x4u8); }
  if !cpu.zf { pc = 0x10305d9; continue; }
  pc = 0x10305c4; continue;
 }
 0x10305c4 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1030713; continue;
 }
 0x10305d9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x103061d; continue; }
  pc = 0x10305e6; continue;
 }
 0x10305e6 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.flat_ds(0x10305f2); cpu.push(0x10305f7); f_01032188(cpu); if cpu.returned_to != 0x10305f7 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if cpu.zf { pc = 0x103061d; continue; }
  pc = 0x10305fe; continue;
 }
 0x10305fe => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x1u16); }
  if (cpu.sf == cpu.of) { pc = 0x1030619; continue; }
  pc = 0x1030615; continue;
 }
 0x1030615 => {
  { let v: u16 = 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1030619; continue;
 }
 0x1030619 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u16)); cpu.w16(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x103061d; continue;
 }
 0x103061d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1030620; continue;
 }
 0x1030620 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1030713; continue; }
  pc = 0x1030629; continue;
 }
 0x1030629 => {
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x11b4c78u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x103065a; continue; }
  pc = 0x1030652; continue;
 }
 0x1030652 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x103065a; continue;
 }
 0x103065a => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  cpu.flat_ds(0x103066b); cpu.push(0x1030670); f_01047379(cpu); if cpu.returned_to != 0x1030670 { return; }
  cpu.flat_ds(0x1030670); cpu.push(0x1030675); f_0106a326(cpu); if cpu.returned_to != 0x1030675 { return; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x42u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x1124e88u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x4u16; cpu.w16(0x10e3e13u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xabbu32; cpu.eax = v; }
  cpu.flat_ds(0x10306b4); cpu.push(0x10306b9); f_01059dbe(cpu); if cpu.returned_to != 0x10306b9 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.flat_ds(0x10306ba); cpu.push(0x10306bf); f_01036a75(cpu); if cpu.returned_to != 0x10306bf { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x1124e88u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x66u8; cpu.w8(0xa5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x10306f5; continue; }
  pc = 0x10306e2; continue;
 }
 0x10306e2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) | 0x20u8; cpu.w8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10306f5; continue;
 }
 0x10306f5 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x103070c; continue; }
  pc = 0x10306fe; continue;
 }
 0x10306fe => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32.wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x1030711; continue; }
  pc = 0x103070c; continue;
 }
 0x103070c => {
  cpu.flat_ds(0x103070c); cpu.push(0x1030711); f_010363a5(cpu); if cpu.returned_to != 0x1030711 { return; }
  pc = 0x1030711; continue;
 }
 0x1030711 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1030713; continue;
 }
 0x1030713 => {
  cpu.flat_ds(0x1030713); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1030714 => {
  { cpu.pushad(); }
  { let v: u32 = 0x1122894u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10f78bcu32.wrapping_add(cpu.bases[3])) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x1030726); cpu.push(0x103072c); cpu.call(t); if cpu.returned_to != 0x103072c { return; }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if (cpu.sf != cpu.of) { pc = 0x103073f; continue; }
  pc = 0x103073b; continue;
 }
 0x103073b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1030767; continue;
 }
 0x103073f => {
  { let v: u8 = cpu.r8(0x10ee950u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.flat_ds(0x1030745); cpu.push(0x103074c); cpu.call(t); if cpu.returned_to != 0x103074c { return; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.sf == cpu.of) { pc = 0x1030767; continue; }
  pc = 0x1030753; continue;
 }
 0x1030753 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11230cbu32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x1030760); cpu.push(0x1030766); cpu.call(t); if cpu.returned_to != 0x1030766 { return; }
  { cpu.popad(); }
  pc = 0x1030767; continue;
 }
 0x1030767 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = 0x96u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xfffeu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x1030779); cpu.push(0x103077e); f_01072b15(cpu); if cpu.returned_to != 0x103077e { return; }
  cpu.flat_ds(0x103077e); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103077f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103077f); }
 let mut pc: u32 = 0x103077f;
 loop { match pc {
 0x103077f => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x41u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x10307ca; continue; }
  pc = 0x1030786; continue;
 }
 0x1030786 => {
  { cpu.sub8(cpu.r8(0x41u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x10307b8; continue; }
  pc = 0x103078c; continue;
 }
 0x103078c => {
  { let v: u32 = cpu.r16(0x1122f8cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1122f8eu32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x31u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(cpu.r8(0x41u32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x762u32); cpu.ecx = v; }
  { cpu.pushad(); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10307b7); cpu.call(t); if cpu.returned_to != 0x10307b7 { return; }
  { cpu.popad(); }
  pc = 0x10307b8; continue;
 }
 0x10307b8 => {
  { cpu.sub8(cpu.r8(0x41u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x10307ca; continue; }
  pc = 0x10307be; continue;
 }
 0x10307be => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { pc = 0x10307ca; continue; }
  pc = 0x10307c7; continue;
 }
 0x10307c7 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x41u32.wrapping_add(cpu.ebp))); cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10307ca; continue;
 }
 0x10307ca => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010307cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10307cc); }
 let mut pc: u32 = 0x10307cc;
 loop { match pc {
 0x10307cc => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { pc = 0x1030821; continue; }
  pc = 0x10307d6; continue;
 }
 0x10307d6 => {
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xfu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x10307e8; continue; }
  pc = 0x10307dd; continue;
 }
 0x10307dd => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) | (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1030821; continue;
 }
 0x10307e8 => {
  { let v: u32 = cpu.r16(0x1122f8cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1122f8eu32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x1u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1030811); f_0101c251(cpu); if cpu.returned_to != 0x1030811 { return; }
  cpu.push(0x1030816); f_0105f215(cpu); if cpu.returned_to != 0x1030816 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) & 0xf0u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) | (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1030821; continue;
 }
 0x1030821 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { pc = 0x103087e; continue; }
  pc = 0x103082a; continue;
 }
 0x103082a => {
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xf0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1030842; continue; }
  pc = 0x1030831; continue;
 }
 0x1030831 => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) & 0xfu8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x103087e; continue;
 }
 0x1030842 => {
  { let v: u32 = cpu.r16(0x1122f8cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1122f8eu32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x38u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x7u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103086b); f_0101c251(cpu); if cpu.returned_to != 0x103086b { return; }
  cpu.push(0x1030870); f_0105f215(cpu); if cpu.returned_to != 0x1030870 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) & 0xfu8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x103087e; continue;
 }
 0x103087e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01030880(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1030880); } r_01030880(cpu, 0x1030880); }
pub fn f_010309aa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10309aa); } r_01030880(cpu, 0x10309aa); }
pub fn f_01030e28(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1030e28); } r_01030880(cpu, 0x1030e28); }
fn r_01030880(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1030880 => {
  { cpu.pushad(); }
  cpu.push(0x1030886); f_0103258d(cpu); if cpu.returned_to != 0x1030886 { return; }
  if !cpu.cf { pc = 0x1030a41; continue; }
  pc = 0x103088c; continue;
 }
 0x103088c => {
  { cpu.sub8(cpu.r8(0x41u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if !cpu.zf { pc = 0x1030a41; continue; }
  pc = 0x1030896; continue;
 }
 0x1030896 => {
  { let v: u8 = 0x4u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  cpu.push(0x10308a8); f_0102f75b(cpu); if cpu.returned_to != 0x10308a8 { return; }
  { cpu.popad(); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10309aa => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x1122f8cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1122f8eu32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xb5eu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1030a41; continue; }
  pc = 0x10309d3; continue;
 }
 0x10309d3 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1030a41; continue; }
  pc = 0x10309dc; continue;
 }
 0x10309dc => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1030a41; continue; }
  pc = 0x10309e2; continue;
 }
 0x10309e2 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1030a41; continue; }
  pc = 0x10309e8; continue;
 }
 0x10309e8 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10309ef); cpu.call(t); if cpu.returned_to != 0x10309ef { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10309f9); f_0103258d(cpu); if cpu.returned_to != 0x10309f9 { return; }
  if !cpu.cf { pc = 0x1030a41; continue; }
  pc = 0x10309fb; continue;
 }
 0x10309fb => {
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x1030eb4; continue; }
  pc = 0x1030a24; continue;
 }
 0x1030a24 => {
  { let v: u8 = 0x1u8; cpu.w8(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x41u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if !cpu.zf { pc = 0x1030a41; continue; }
  pc = 0x1030a2e; continue;
 }
 0x1030a2e => {
  { let v: u8 = 0x4u8; cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  cpu.push(0x1030a40); f_0102f75b(cpu); if cpu.returned_to != 0x1030a40 { return; }
  { cpu.popad(); }
  pc = 0x1030a41; continue;
 }
 0x1030a41 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1030e28 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x1122f8cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1122f8eu32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xb38u32; cpu.ecx = v; }
  { let v: u16 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)) as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.edx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1030eb4; continue; }
  pc = 0x1030e55; continue;
 }
 0x1030e55 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1030eb4; continue; }
  pc = 0x1030e5e; continue;
 }
 0x1030e5e => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1030eb4; continue; }
  pc = 0x1030e64; continue;
 }
 0x1030e64 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1030eb4; continue; }
  pc = 0x1030e6a; continue;
 }
 0x1030e6a => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1030e71); cpu.call(t); if cpu.returned_to != 0x1030e71 { return; }
  { cpu.popad(); }
  cpu.push(0x1030e77); f_0103258d(cpu); if cpu.returned_to != 0x1030e77 { return; }
  if !cpu.cf { pc = 0x1030eb4; continue; }
  pc = 0x1030e79; continue;
 }
 0x1030e79 => {
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x1030eb4; continue; }
  pc = 0x1030e9e; continue;
 }
 0x1030e9e => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x1030eb4; continue; }
  pc = 0x1030ea7; continue;
 }
 0x1030ea7 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x12u32.wrapping_add(cpu.ebp))); cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.ebp)), 0xcu8); }
  if !cpu.zf { pc = 0x1030eb4; continue; }
  pc = 0x1030eb0; continue;
 }
 0x1030eb0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1030eb4; continue;
 }
 0x1030eb4 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010308ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10308ab); }
 let mut pc: u32 = 0x10308ab;
 loop { match pc {
 0x10308ab => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x1122f8cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1122f8eu32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x12u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x5u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xb59u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1030925; continue; }
  pc = 0x10308d4; continue;
 }
 0x10308d4 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1030925; continue; }
  pc = 0x10308dd; continue;
 }
 0x10308dd => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1030925; continue; }
  pc = 0x10308e3; continue;
 }
 0x10308e3 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1030925; continue; }
  pc = 0x10308e9; continue;
 }
 0x10308e9 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10308f0); cpu.call(t); if cpu.returned_to != 0x10308f0 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10308fa); f_0103258d(cpu); if cpu.returned_to != 0x10308fa { return; }
  if !cpu.cf { pc = 0x1030925; continue; }
  pc = 0x10308fc; continue;
 }
 0x10308fc => {
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x1030925; continue; }
  pc = 0x1030921; continue;
 }
 0x1030921 => {
  { let v: u8 = 0x1u8; cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1030925; continue;
 }
 0x1030925 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01030927(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1030927); }
 let mut pc: u32 = 0x1030927;
 loop { match pc {
 0x1030927 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x1122f8cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1122f8eu32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x13u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xb5bu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10309a8; continue; }
  pc = 0x1030950; continue;
 }
 0x1030950 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10309a8; continue; }
  pc = 0x1030959; continue;
 }
 0x1030959 => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10309a8; continue; }
  pc = 0x103095f; continue;
 }
 0x103095f => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10309a8; continue; }
  pc = 0x1030965; continue;
 }
 0x1030965 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103096c); cpu.call(t); if cpu.returned_to != 0x103096c { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x10309a4; continue; }
  pc = 0x1030992; continue;
 }
 0x1030992 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x10309a8; continue; }
  pc = 0x103099b; continue;
 }
 0x103099b => {
  { let v: u8 = cpu.inc8(cpu.r8(0x13u32.wrapping_add(cpu.ebp))); cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x10309a8; continue; }
  pc = 0x10309a4; continue;
 }
 0x10309a4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10309a8; continue;
 }
 0x10309a8 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01030a43(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1030a43); }
 let mut pc: u32 = 0x1030a43;
 loop { match pc {
 0x1030a43 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x1122f8cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1122f8eu32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x15u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xb60u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1030a99; continue; }
  pc = 0x1030a6c; continue;
 }
 0x1030a6c => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1030a99; continue; }
  pc = 0x1030a75; continue;
 }
 0x1030a75 => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1030a99; continue; }
  pc = 0x1030a7b; continue;
 }
 0x1030a7b => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1030a99; continue; }
  pc = 0x1030a81; continue;
 }
 0x1030a81 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1030a88); cpu.call(t); if cpu.returned_to != 0x1030a88 { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x1030a99; continue; }
  pc = 0x1030a92; continue;
 }
 0x1030a92 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x15u32.wrapping_add(cpu.ebp))); cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1030a99; continue;
 }
 0x1030a99 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01030a9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1030a9b); }
 let mut pc: u32 = 0x1030a9b;
 loop { match pc {
 0x1030a9b => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x11b4c6cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1030ac8; continue; }
  pc = 0x1030aa8; continue;
 }
 0x1030aa8 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e55f8u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf == cpu.of) { pc = 0x1030ac8; continue; }
  pc = 0x1030ab8; continue;
 }
 0x1030ab8 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e68bau32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x1030ad1; continue; }
  pc = 0x1030ac8; continue;
 }
 0x1030ac8 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b4c6cu32, v); }
  pc = 0x1030af4; continue;
 }
 0x1030ad1 => {
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edi = v; }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x64u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = 0x9cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.add32(cpu.r32(0x10e4010u32), 0x8u32); cpu.w32(0x10e4010u32, v); }
  pc = 0x1030af4; continue;
 }
 0x1030af4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01030af5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1030af5); }
 let mut pc: u32 = 0x1030af5;
 loop { match pc {
 0x1030af5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01030af6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1030af6); }
 let mut pc: u32 = 0x1030af6;
 loop { match pc {
 0x1030af6 => {
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x11b4c6cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1030b1d; continue; }
  pc = 0x1030b05; continue;
 }
 0x1030b05 => {
  { cpu.sub32(cpu.edi, 0x10e55f8u32); }
  if (cpu.sf == cpu.of) { pc = 0x1030b1d; continue; }
  pc = 0x1030b0d; continue;
 }
 0x1030b0d => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e68bau32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x1030b26; continue; }
  pc = 0x1030b1d; continue;
 }
 0x1030b1d => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b4c6cu32, v); }
  pc = 0x1030b4d; continue;
 }
 0x1030b26 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x65u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = 0x9bu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.add32(cpu.r32(0x10e4010u32), 0x8u32); cpu.w32(0x10e4010u32, v); }
  pc = 0x1030b4d; continue;
 }
 0x1030b4d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01030b4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1030b4e); }
 let mut pc: u32 = 0x1030b4e;
 loop { match pc {
 0x1030b4e => {
  { let v: u8 = 0x8u8; cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x112290cu32; cpu.esi = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1030b7f); f_0102f786(cpu); if cpu.returned_to != 0x1030b7f { return; }
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1122c0eu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x1030ba4); f_01072a54(cpu); if cpu.returned_to != 0x1030ba4 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01030ba5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1030ba5); }
 let mut pc: u32 = 0x1030ba5;
 loop { match pc {
 0x1030ba5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1030bac); f_01031c81(cpu); if cpu.returned_to != 0x1030bac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1030bbf; continue; }
  pc = 0x1030bb0; continue;
 }
 0x1030bb0 => {
  cpu.push(0x1030bb5); f_01032188(cpu); if cpu.returned_to != 0x1030bb5 { return; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1030bbe); f_010321ae(cpu); if cpu.returned_to != 0x1030bbe { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1030bbf => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = 0x1194208u32; cpu.esi = v; }
  { let v: u8 = 0x2au8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3au8; cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x2au8; cpu.w8(0x3u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1030c00); f_0104b988(cpu); if cpu.returned_to != 0x1030c00 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01030c02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1030c02); }
 let mut pc: u32 = 0x1030c02;
 loop { match pc {
 0x1030c02 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1030c2c; continue; }
  pc = 0x1030c10; continue;
 }
 0x1030c10 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10f570cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10f570du32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10f570eu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10f570fu32, v); }
  pc = 0x1030c2c; continue;
 }
 0x1030c2c => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x1030c6a; continue; }
  pc = 0x1030c36; continue;
 }
 0x1030c36 => {
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1030c66; continue; }
  pc = 0x1030c40; continue;
 }
 0x1030c40 => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x1030c50; continue; }
  pc = 0x1030c46; continue;
 }
 0x1030c46 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x32u8; cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1030c6a; continue;
 }
 0x1030c50 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1030c6a; continue; }
  pc = 0x1030c56; continue;
 }
 0x1030c56 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x15u32.wrapping_add(cpu.ebp))); cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1030c6a; continue; }
  pc = 0x1030c5f; continue;
 }
 0x1030c5f => {
  cpu.push(0x1030c64); f_01036371(cpu); if cpu.returned_to != 0x1030c64 { return; }
  pc = 0x1030c6a; continue;
 }
 0x1030c66 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1030c6a; continue;
 }
 0x1030c6a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1030d07; continue; }
  pc = 0x1030c89; continue;
 }
 0x1030c89 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1030d07; continue; }
  pc = 0x1030c92; continue;
 }
 0x1030c92 => {
  { cpu.sub16((cpu.eax as u16), 0xffc0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1030d07; continue; }
  pc = 0x1030c98; continue;
 }
 0x1030c98 => {
  { cpu.sub16((cpu.ebx as u16), 0xffd0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1030d07; continue; }
  pc = 0x1030c9e; continue;
 }
 0x1030c9e => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1030d23; continue; }
  pc = 0x1030ca6; continue;
 }
 0x1030ca6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1122f8cu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1122f8eu32, v); }
  cpu.push(0x1030cb8); f_01030fd2(cpu); if cpu.returned_to != 0x1030cb8 { return; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x1030cf0; continue; }
  pc = 0x1030cc5; continue;
 }
 0x1030cc5 => {
  { let v: u8 = cpu.r8(0x10ee950u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x1030cd2); cpu.call(t); if cpu.returned_to != 0x1030cd2 { return; }
  { let v: u16 = cpu.r16(0x1122f8cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1122f8eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11230c9u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1030cf0); cpu.call(t); if cpu.returned_to != 0x1030cf0 { return; }
  pc = 0x1030cf0; continue;
 }
 0x1030cf0 => {
  { cpu.popad(); }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x1030cf8); f_01030eb6(cpu); if cpu.returned_to != 0x1030cf8 { return; }
  cpu.push(0x1030cfd); f_01030d54(cpu); if cpu.returned_to != 0x1030cfd { return; }
  cpu.push(0x1030d02); f_01030dd0(cpu); if cpu.returned_to != 0x1030d02 { return; }
  cpu.push(0x1030d07); f_01030e28(cpu); if cpu.returned_to != 0x1030d07 { return; }
  pc = 0x1030d07; continue;
 }
 0x1030d07 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1030d3c; continue; }
  pc = 0x1030d0f; continue;
 }
 0x1030d0f => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1030d22); f_01072b15(cpu); if cpu.returned_to != 0x1030d22 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1030d23 => {
  { cpu.pushad(); }
  { let v: u32 = 0x11228d0u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10f78bcu32) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1030d3b); cpu.call(t); if cpu.returned_to != 0x1030d3b { return; }
  { cpu.popad(); }
  pc = 0x1030d3c; continue;
 }
 0x1030d3c => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = 0x96u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xfffeu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1030d53); f_01072b15(cpu); if cpu.returned_to != 0x1030d53 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01030d54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1030d54); }
 let mut pc: u32 = 0x1030d54;
 loop { match pc {
 0x1030d54 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x1122f8cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1122f8eu32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x29u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xb4bu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1030dce; continue; }
  pc = 0x1030d7d; continue;
 }
 0x1030d7d => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1030dce; continue; }
  pc = 0x1030d86; continue;
 }
 0x1030d86 => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1030dce; continue; }
  pc = 0x1030d8c; continue;
 }
 0x1030d8c => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1030dce; continue; }
  pc = 0x1030d92; continue;
 }
 0x1030d92 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1030d99); cpu.call(t); if cpu.returned_to != 0x1030d99 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1030da3); f_0103258d(cpu); if cpu.returned_to != 0x1030da3 { return; }
  if !cpu.cf { pc = 0x1030dce; continue; }
  pc = 0x1030da5; continue;
 }
 0x1030da5 => {
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x1030dce; continue; }
  pc = 0x1030dca; continue;
 }
 0x1030dca => {
  { let v: u8 = 0x1u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1030dce; continue;
 }
 0x1030dce => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01030dd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1030dd0); }
 let mut pc: u32 = 0x1030dd0;
 loop { match pc {
 0x1030dd0 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x1122f8cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1122f8eu32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x16u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x13u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xb62u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1030e26; continue; }
  pc = 0x1030df9; continue;
 }
 0x1030df9 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1030e26; continue; }
  pc = 0x1030e02; continue;
 }
 0x1030e02 => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1030e26; continue; }
  pc = 0x1030e08; continue;
 }
 0x1030e08 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1030e26; continue; }
  pc = 0x1030e0e; continue;
 }
 0x1030e0e => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1030e15); cpu.call(t); if cpu.returned_to != 0x1030e15 { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x1030e26; continue; }
  pc = 0x1030e1f; continue;
 }
 0x1030e1f => {
  { let v: u8 = cpu.inc8(cpu.r8(0x13u32.wrapping_add(cpu.ebp))); cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x13u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1030e26; continue;
 }
 0x1030e26 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01030eb6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1030eb6); }
 let mut pc: u32 = 0x1030eb6;
 loop { match pc {
 0x1030eb6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_add(0x10u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_sub(0x12u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x1122fbdu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.eax as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x11b4c6cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1030f07; continue; }
  pc = 0x1030ee7; continue;
 }
 0x1030ee7 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e55f8u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf == cpu.of) { pc = 0x1030f07; continue; }
  pc = 0x1030ef7; continue;
 }
 0x1030ef7 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e68bau32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x1030f10; continue; }
  pc = 0x1030f07; continue;
 }
 0x1030f07 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b4c6cu32, v); }
  pc = 0x1030f35; continue;
 }
 0x1030f10 => {
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edi = v; }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x32u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x5d0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xceu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.add32(cpu.r32(0x10e4010u32), 0x8u32); cpu.w32(0x10e4010u32, v); }
  pc = 0x1030f35; continue;
 }
 0x1030f35 => {
  cpu.push(0x1030f3a); f_0103258d(cpu); if cpu.returned_to != 0x1030f3a { return; }
  if !cpu.cf { pc = 0x1030fce; continue; }
  pc = 0x1030f40; continue;
 }
 0x1030f40 => {
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x1030fce; continue; }
  pc = 0x1030f65; continue;
 }
 0x1030f65 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x1030fce; continue; }
  pc = 0x1030f6e; continue;
 }
 0x1030f6e => {
  { let v: u8 = cpu.inc8(cpu.r8(0xau32.wrapping_add(cpu.ebp))); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)) & 0x7u8; cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xau32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1030fce; continue; }
  pc = 0x1030f7b; continue;
 }
 0x1030f7b => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91du32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1030fce; continue; }
  pc = 0x1030f89; continue;
 }
 0x1030f89 => {
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1030fce; continue; }
  pc = 0x1030f92; continue;
 }
 0x1030f92 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1030fa1); f_01072d00(cpu); if cpu.returned_to != 0x1030fa1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1030fc3; continue; }
  pc = 0x1030fb9; continue;
 }
 0x1030fb9 => {
  cpu.push(0x1030fbe); f_01012780(cpu); if cpu.returned_to != 0x1030fbe { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1030fc3; continue;
 }
 0x1030fc3 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1030fce); f_010356fe(cpu); if cpu.returned_to != 0x1030fce { return; }
  pc = 0x1030fce; continue;
 }
 0x1030fce => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01030fd2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1030fd2); }
 let mut pc: u32 = 0x1030fd2;
 loop { match pc {
 0x1030fd2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x11228d0u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10f78bcu32) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1030fed); cpu.call(t); if cpu.returned_to != 0x1030fed { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01030ff3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1030ff3); }
 let mut pc: u32 = 0x1030ff3;
 loop { match pc {
 0x1030ff3 => {
  { let v: u8 = 0x8u8; cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1031007); f_0105f178(cpu); if cpu.returned_to != 0x1031007 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1122918u32; cpu.esi = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1031028); f_0102f786(cpu); if cpu.returned_to != 0x1031028 { return; }
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1122a74u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x103104d); f_01072a54(cpu); if cpu.returned_to != 0x103104d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103104e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103104e); }
 let mut pc: u32 = 0x103104e;
 loop { match pc {
 0x103104e => {
  { let v: u8 = 0x8u8; cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1031062); f_0105f178(cpu); if cpu.returned_to != 0x1031062 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x103106c); f_01031cd1(cpu); if cpu.returned_to != 0x103106c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103106f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103106f); } r_0103106f(cpu, 0x103106f); }
pub fn f_01031200(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1031200); } r_0103106f(cpu, 0x1031200); }
fn r_0103106f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x103106f => {
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e93u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1123000u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1031173; continue; }
  pc = 0x10310a0; continue;
 }
 0x10310a0 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1031173; continue; }
  pc = 0x10310ad; continue;
 }
 0x10310ad => {
  { cpu.sub16((cpu.eax as u16), 0xffc0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1031173; continue; }
  pc = 0x10310b7; continue;
 }
 0x10310b7 => {
  { cpu.sub16((cpu.ebx as u16), 0xff90u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1031173; continue; }
  pc = 0x10310c1; continue;
 }
 0x10310c1 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1031343; continue; }
  pc = 0x10310cd; continue;
 }
 0x10310cd => {
  { let v: u8 = 0x1u8; cpu.w8(0x1123000u32, v); }
  { let v: u32 = 0x11228a8u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10f78bcu32) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1122f8cu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1122f8eu32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10310f9); cpu.call(t); if cpu.returned_to != 0x10310f9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x103113a; continue; }
  pc = 0x1031107; continue;
 }
 0x1031107 => {
  { let v: u8 = cpu.r8(0x10ee950u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x1031114); cpu.call(t); if cpu.returned_to != 0x1031114 { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1122f8cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1122f8eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11230c5u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1031138); cpu.call(t); if cpu.returned_to != 0x1031138 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x103113a; continue;
 }
 0x103113a => {
  cpu.push(0x103113f); f_010319aa(cpu); if cpu.returned_to != 0x103113f { return; }
  cpu.push(0x1031144); f_0102f5b3(cpu); if cpu.returned_to != 0x1031144 { return; }
  cpu.push(0x1031149); f_01031957(cpu); if cpu.returned_to != 0x1031149 { return; }
  cpu.push(0x103114e); f_0103180e(cpu); if cpu.returned_to != 0x103114e { return; }
  cpu.push(0x1031153); f_01031764(cpu); if cpu.returned_to != 0x1031153 { return; }
  cpu.push(0x1031158); f_010318fd(cpu); if cpu.returned_to != 0x1031158 { return; }
  cpu.push(0x103115d); f_01031898(cpu); if cpu.returned_to != 0x103115d { return; }
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x103116b; continue; }
  pc = 0x1031163; continue;
 }
 0x1031163 => {
  cpu.push(0x1031168); f_0105f178(cpu); if cpu.returned_to != 0x1031168 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x103116b; continue;
 }
 0x103116b => {
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1031173); f_01031a1f(cpu); if cpu.returned_to != 0x1031173 { return; }
  pc = 0x1031173; continue;
 }
 0x1031173 => {
  { cpu.sub8(cpu.r8(0x11cc2b7u32), 0x0u8); }
  if !cpu.zf { pc = 0x103132f; continue; }
  pc = 0x1031180; continue;
 }
 0x1031180 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x103139b; continue; }
  pc = 0x103118c; continue;
 }
 0x103118c => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x4u8); }
  if !cpu.cf { pc = 0x103132f; continue; }
  pc = 0x1031199; continue;
 }
 0x1031199 => {
  { cpu.sub16(cpu.r16(0x34u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x10311a9; continue; }
  pc = 0x10311a1; continue;
 }
 0x10311a1 => {
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x103120a; continue;
 }
 0x10311a9 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x103120a; continue; }
  pc = 0x10311af; continue;
 }
 0x10311af => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x10311be; continue; }
  pc = 0x10311b8; continue;
 }
 0x10311b8 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x10311c6; continue; }
  pc = 0x10311be; continue;
 }
 0x10311be => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x10311dd; continue; }
  pc = 0x10311c4; continue;
 }
 0x10311c4 => {
  pc = 0x10311f7; continue;
 }
 0x10311c6 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1031200; continue; }
  pc = 0x10311cc; continue;
 }
 0x10311cc => {
  { let v: u8 = 0x0u8; cpu.w8(0x11eb7d8u32, v); }
  cpu.push(0x10311d8); f_0108c3f7(cpu); if cpu.returned_to != 0x10311d8 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10311f7; continue;
 }
 0x10311dd => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1031200; continue; }
  pc = 0x10311e3; continue;
 }
 0x10311e3 => {
  { let v: u16 = cpu.r16(0x10e3d79u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10311ee); f_01016ace(cpu); if cpu.returned_to != 0x10311ee { return; }
  if cpu.cf { pc = 0x103132f; continue; }
  pc = 0x10311f4; continue;
 }
 0x10311f4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10311f7; continue;
 }
 0x10311f7 => {
  cpu.push(0x10311fc); f_01031710(cpu); if cpu.returned_to != 0x10311fc { return; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1031200; continue;
 }
 0x1031200 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x103132f; continue; }
  pc = 0x103120a; continue;
 }
 0x103120a => {
  { cpu.sub16(cpu.r16(0x34u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1031235; continue; }
  pc = 0x1031212; continue;
 }
 0x1031212 => {
  { cpu.and16(cpu.r16(0x10e3d07u32), 0x2u16); }
  if cpu.zf { pc = 0x103132f; continue; }
  pc = 0x1031221; continue;
 }
 0x1031221 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = 0x23u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3au16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1031230); f_010321ae(cpu); if cpu.returned_to != 0x1031230 { return; }
  pc = 0x103132f; continue;
 }
 0x1031235 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x1123000u32), 0x0u8); }
  if cpu.zf { pc = 0x1031249; continue; }
  pc = 0x1031242; continue;
 }
 0x1031242 => {
  { let v: u8 = 0x1u8; cpu.w8(0x110d739u32, v); }
  pc = 0x1031249; continue;
 }
 0x1031249 => {
  { let v: u16 = 0x23u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3au16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1031256); f_01030ba5(cpu); if cpu.returned_to != 0x1031256 { return; }
  cpu.push(0x103125b); f_01031ca9(cpu); if cpu.returned_to != 0x103125b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x103132f; continue; }
  pc = 0x1031268; continue;
 }
 0x1031268 => {
  { let v: u32 = 0x1bu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.dec16(cpu.r16(0xau32.wrapping_add(cpu.ebp))); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0xau32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x103127f; continue; }
  pc = 0x103127b; continue;
 }
 0x103127b => {
  { let v: u16 = cpu.neg16(cpu.r16(0xau32.wrapping_add(cpu.ebp))); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x103127f; continue;
 }
 0x103127f => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r32(0x10e4138u32); cpu.push(v); }
  cpu.push(0x103128d); f_0101776f(cpu); if cpu.returned_to != 0x103128d { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4138u32, v); }
  if cpu.cf { pc = 0x103132f; continue; }
  pc = 0x1031299; continue;
 }
 0x1031299 => {
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  if (cpu.sf == cpu.of) { pc = 0x103132f; continue; }
  pc = 0x10312a7; continue;
 }
 0x10312a7 => {
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10312b3); f_01031710(cpu); if cpu.returned_to != 0x10312b3 { return; }
  { let v: u16 = 0x44u16; cpu.w16(0x34u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.pushad(); }
  cpu.push(0x10312c9); f_01031aa6(cpu); if cpu.returned_to != 0x10312c9 { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x11eb7d8u32), 0x0u8); }
  if !cpu.zf { pc = 0x10313ad; continue; }
  pc = 0x10312d7; continue;
 }
 0x10312d7 => {
  { let v: u32 = 0x0u32; cpu.w32(0x2au32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x10312fa; continue; }
  pc = 0x10312e7; continue;
 }
 0x10312e7 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x10312fa; continue; }
  pc = 0x10312f0; continue;
 }
 0x10312f0 => {
  cpu.push(0x10312f5); f_0108c3f7(cpu); if cpu.returned_to != 0x10312f5 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x103132f; continue;
 }
 0x10312fa => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1031302); f_01016ace(cpu); if cpu.returned_to != 0x1031302 { return; }
  if cpu.cf { pc = 0x103131e; continue; }
  pc = 0x1031304; continue;
 }
 0x1031304 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103131e; continue; }
  pc = 0x103130b; continue;
 }
 0x103130b => {
  { let v = cpu.r32(0x10e4138u32); cpu.push(v); }
  cpu.push(0x1031316); f_01017741(cpu); if cpu.returned_to != 0x1031316 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4138u32, v); }
  if !cpu.cf { pc = 0x103132f; continue; }
  pc = 0x103131e; continue;
 }
 0x103131e => {
  { let v: u8 = 0xffu8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x103132f; continue;
 }
 0x103132f => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1031342); f_01072b15(cpu); if cpu.returned_to != 0x1031342 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1031343 => {
  { cpu.pushad(); }
  { let v: u32 = 0x11228a8u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10f78bcu32) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103135b); cpu.call(t); if cpu.returned_to != 0x103135b { return; }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if (cpu.sf != cpu.of) { pc = 0x103136e; continue; }
  pc = 0x103136a; continue;
 }
 0x103136a => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x103139b; continue;
 }
 0x103136e => {
  { let v: u8 = cpu.r8(0x10ee950u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x103137b); cpu.call(t); if cpu.returned_to != 0x103137b { return; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.sf == cpu.of) { pc = 0x103139b; continue; }
  pc = 0x1031382; continue;
 }
 0x1031382 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11230c5u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103139a); cpu.call(t); if cpu.returned_to != 0x103139a { return; }
  { cpu.popad(); }
  pc = 0x103139b; continue;
 }
 0x103139b => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = 0xfffeu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10313a7); f_01072b15(cpu); if cpu.returned_to != 0x10313a7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10313ad => {
  { let v: u32 = cpu.r32(0x2au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x116e72cu32).wrapping_add(cpu.eax); cpu.w32(0x116e72cu32, v); }
  cpu.push(0x10313bb); f_0108c3f7(cpu); if cpu.returned_to != 0x10313bb { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11eb7d8u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x103132f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010313ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10313ca); }
 let mut pc: u32 = 0x10313ca;
 loop { match pc {
 0x10313ca => {
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e93u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x11dbe00u32, v); }
  { cpu.sub8(cpu.r8(0x11cc2b7u32), 0x0u8); }
  if !cpu.zf { pc = 0x103168c; continue; }
  pc = 0x10313f2; continue;
 }
 0x10313f2 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x103169b; continue; }
  pc = 0x10313fe; continue;
 }
 0x10313fe => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x4u8); }
  { let v: u32 = 0x2u32; cpu.w32(0x11dbe00u32, v); }
  if !cpu.cf { pc = 0x103168c; continue; }
  pc = 0x1031415; continue;
 }
 0x1031415 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1031423); f_010027ce(cpu); if cpu.returned_to != 0x1031423 { return; }
  { cpu.sub8(cpu.r8(0x10fd43du32.wrapping_add(cpu.ecx)), 0x0u8); }
  { cpu.popad(); }
  { let v: u32 = 0x3u32; cpu.w32(0x11dbe00u32, v); }
  if cpu.cf { pc = 0x103168c; continue; }
  pc = 0x103143b; continue;
 }
 0x103143b => {
  if cpu.zf { pc = 0x103168c; continue; }
  pc = 0x1031441; continue;
 }
 0x1031441 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1031448); f_01031cd1(cpu); if cpu.returned_to != 0x1031448 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x34u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x103145a; continue; }
  pc = 0x1031452; continue;
 }
 0x1031452 => {
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x10314c8; continue;
 }
 0x103145a => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x10314c8; continue; }
  pc = 0x1031460; continue;
 }
 0x1031460 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x103146f; continue; }
  pc = 0x1031469; continue;
 }
 0x1031469 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x1031477; continue; }
  pc = 0x103146f; continue;
 }
 0x103146f => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1031487; continue; }
  pc = 0x1031475; continue;
 }
 0x1031475 => {
  pc = 0x10314ab; continue;
 }
 0x1031477 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x10314b4; continue; }
  pc = 0x103147d; continue;
 }
 0x103147d => {
  cpu.push(0x1031482); f_0108c451(cpu); if cpu.returned_to != 0x1031482 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10314ab; continue;
 }
 0x1031487 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x10314b4; continue; }
  pc = 0x103148d; continue;
 }
 0x103148d => {
  { let v: u16 = cpu.r16(0x10e3d79u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1031498); f_01016ace(cpu); if cpu.returned_to != 0x1031498 { return; }
  { let v: u32 = 0x4u32; cpu.w32(0x11dbe00u32, v); }
  if cpu.cf { pc = 0x103168c; continue; }
  pc = 0x10314a8; continue;
 }
 0x10314a8 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10314ab; continue;
 }
 0x10314ab => {
  cpu.push(0x10314b0); f_01031710(cpu); if cpu.returned_to != 0x10314b0 { return; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10314b4; continue;
 }
 0x10314b4 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  { let v: u32 = 0x5u32; cpu.w32(0x11dbe00u32, v); }
  if cpu.zf { pc = 0x103168c; continue; }
  pc = 0x10314c8; continue;
 }
 0x10314c8 => {
  { cpu.sub16(cpu.r16(0x34u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x103151e; continue; }
  pc = 0x10314d0; continue;
 }
 0x10314d0 => {
  { cpu.and16(cpu.r16(0x10e3d07u32), 0x2u16); }
  { let v: u32 = 0x6u32; cpu.w32(0x11dbe00u32, v); }
  if cpu.zf { pc = 0x103168c; continue; }
  pc = 0x10314e9; continue;
 }
 0x10314e9 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = 0x4au16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4au16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  { let v: u32 = 0x7u32; cpu.w32(0x11dbe00u32, v); }
  if !cpu.zf { pc = 0x103168c; continue; }
  pc = 0x103150a; continue;
 }
 0x103150a => {
  cpu.push(0x103150f); f_010321ae(cpu); if cpu.returned_to != 0x103150f { return; }
  { let v: u32 = 0x8u32; cpu.w32(0x11dbe00u32, v); }
  pc = 0x103168c; continue;
 }
 0x103151e => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x4au16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4au16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1031538; continue; }
  pc = 0x1031533; continue;
 }
 0x1031533 => {
  cpu.push(0x1031538); f_01030ba5(cpu); if cpu.returned_to != 0x1031538 { return; }
  pc = 0x1031538; continue;
 }
 0x1031538 => {
  cpu.push(0x103153d); f_01031ca9(cpu); if cpu.returned_to != 0x103153d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x9u32; cpu.w32(0x11dbe00u32, v); }
  if cpu.zf { pc = 0x103168c; continue; }
  pc = 0x1031554; continue;
 }
 0x1031554 => {
  { let v: u32 = 0x1bu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.dec16(cpu.r16(0xau32.wrapping_add(cpu.ebp))); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0xau32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x103156b; continue; }
  pc = 0x1031567; continue;
 }
 0x1031567 => {
  { let v: u16 = cpu.neg16(cpu.r16(0xau32.wrapping_add(cpu.ebp))); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x103156b; continue;
 }
 0x103156b => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r32(0x10e4138u32); cpu.push(v); }
  cpu.push(0x1031579); f_0101776f(cpu); if cpu.returned_to != 0x1031579 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4138u32, v); }
  { let v: u32 = 0xau32; cpu.w32(0x11dbe00u32, v); }
  if cpu.cf { pc = 0x103168c; continue; }
  pc = 0x103158f; continue;
 }
 0x103158f => {
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xbu32; cpu.w32(0x11dbe00u32, v); }
  if (cpu.sf == cpu.of) { pc = 0x103168c; continue; }
  pc = 0x10315a7; continue;
 }
 0x10315a7 => {
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10315b3); f_01031710(cpu); if cpu.returned_to != 0x10315b3 { return; }
  { let v: u16 = 0x44u16; cpu.w16(0x34u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.pushad(); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x10315eb; continue; }
  pc = 0x10315c9; continue;
 }
 0x10315c9 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x3u8); }
  if cpu.zf { pc = 0x10315eb; continue; }
  pc = 0x10315ce; continue;
 }
 0x10315ce => {
  { let v: u16 = 0x48u16; cpu.w16(0x1122f94u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x1122f96u32, v); }
  { let v: u16 = 0xffd0u16; cpu.w16(0x1122f98u32, v); }
  pc = 0x1031606; continue;
 }
 0x10315eb => {
  { let v: u16 = 0x47u16; cpu.w16(0x1122f94u32, v); }
  { let v: u16 = 0x68u16; cpu.w16(0x1122f96u32, v); }
  { let v: u16 = 0x70u16; cpu.w16(0x1122f98u32, v); }
  pc = 0x1031606; continue;
 }
 0x1031606 => {
  cpu.push(0x103160b); f_01031aa6(cpu); if cpu.returned_to != 0x103160b { return; }
  { let v: u16 = 0x20u16; cpu.w16(0x1122f94u32, v); }
  { let v: u16 = 0x32u16; cpu.w16(0x1122f96u32, v); }
  { let v: u16 = 0x30u16; cpu.w16(0x1122f98u32, v); }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x103164d; continue; }
  pc = 0x1031630; continue;
 }
 0x1031630 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x103164d; continue; }
  pc = 0x1031639; continue;
 }
 0x1031639 => {
  cpu.push(0x103163e); f_0108c451(cpu); if cpu.returned_to != 0x103163e { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xcu32; cpu.w32(0x11dbe00u32, v); }
  pc = 0x103168c; continue;
 }
 0x103164d => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1031655); f_01016ace(cpu); if cpu.returned_to != 0x1031655 { return; }
  if cpu.cf { pc = 0x103167b; continue; }
  pc = 0x1031657; continue;
 }
 0x1031657 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103167b; continue; }
  pc = 0x103165e; continue;
 }
 0x103165e => {
  { let v = cpu.r32(0x10e4138u32); cpu.push(v); }
  cpu.push(0x1031669); f_01017741(cpu); if cpu.returned_to != 0x1031669 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4138u32, v); }
  { let v: u32 = 0xdu32; cpu.w32(0x11dbe00u32, v); }
  if !cpu.cf { pc = 0x103168c; continue; }
  pc = 0x103167b; continue;
 }
 0x103167b => {
  { let v: u8 = 0xffu8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  pc = 0x103168c; continue;
 }
 0x103168c => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103169b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010316a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10316a1); }
 let mut pc: u32 = 0x10316a1;
 loop { match pc {
 0x10316a1 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x10316b0; continue; }
  pc = 0x10316a9; continue;
 }
 0x10316a9 => {
  { let v: u32 = 0x11111111u32; cpu.eax = v; }
  pc = 0x10316ca; continue;
 }
 0x10316b0 => {
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10316c9); f_01087028(cpu); if cpu.returned_to != 0x10316c9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x10316ca; continue;
 }
 0x10316ca => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xddu32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x10e3d81u32); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x2au32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  if !cpu.zf { pc = 0x10316ea; continue; }
  pc = 0x10316e5; continue;
 }
 0x10316e5 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10316ea; continue;
 }
 0x10316ea => {
  { let v: u32 = cpu.eax; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010316f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10316f5); }
 let mut pc: u32 = 0x10316f5;
 loop { match pc {
 0x10316f5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x370u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x10e3d81u32); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01031701(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1031701); }
 let mut pc: u32 = 0x1031701;
 loop { match pc {
 0x1031701 => {
  { cpu.sub32(cpu.r32(0x4589010eu32), 0x45c76622u32); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffffc3u32.wrapping_add(cpu.eax)), (cpu.ebx as u8)); cpu.w8(0xffffffc3u32.wrapping_add(cpu.eax), v); }
  f_01031710(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01031710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1031710); }
 let mut pc: u32 = 0x1031710;
 loop { match pc {
 0x1031710 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x103171f; continue; }
  pc = 0x1031718; continue;
 }
 0x1031718 => {
  { let v: u32 = 0x11111111u32; cpu.eax = v; }
  pc = 0x1031739; continue;
 }
 0x103171f => {
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1031738); f_01087028(cpu); if cpu.returned_to != 0x1031738 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1031739; continue;
 }
 0x1031739 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xddu32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x10e3d81u32); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x2au32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  if !cpu.zf { pc = 0x1031759; continue; }
  pc = 0x1031754; continue;
 }
 0x1031754 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1031759; continue;
 }
 0x1031759 => {
  { let v: u32 = cpu.eax; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01031764(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1031764); }
 let mut pc: u32 = 0x1031764;
 loop { match pc {
 0x1031764 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x103176f; continue; }
  pc = 0x103176b; continue;
 }
 0x103176b => {
  { let v: u8 = 0x1u8; cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  pc = 0x103176f; continue;
 }
 0x103176f => {
  { let v: u32 = cpu.r16(0x1122f8cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1122f8eu32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x22u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x13u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  pc = 0x103178e; continue;
 }
 0x103178e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0xb57u32); cpu.ecx = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10317c0; continue; }
  pc = 0x10317a3; continue;
 }
 0x10317a3 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10317c0; continue; }
  pc = 0x10317ac; continue;
 }
 0x10317ac => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10317c0; continue; }
  pc = 0x10317b2; continue;
 }
 0x10317b2 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10317c0; continue; }
  pc = 0x10317b8; continue;
 }
 0x10317b8 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10317bf); cpu.call(t); if cpu.returned_to != 0x10317bf { return; }
  { cpu.popad(); }
  pc = 0x10317c0; continue;
 }
 0x10317c0 => {
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103178e; continue; }
  pc = 0x10317ca; continue;
 }
 0x10317ca => {
  cpu.push(0x10317cf); f_0103258d(cpu); if cpu.returned_to != 0x10317cf { return; }
  if !cpu.cf { pc = 0x103180c; continue; }
  pc = 0x10317d1; continue;
 }
 0x10317d1 => {
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x103180c; continue; }
  pc = 0x10317f6; continue;
 }
 0x10317f6 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x103180c; continue; }
  pc = 0x10317ff; continue;
 }
 0x10317ff => {
  { let v: u8 = cpu.shl8(cpu.r8(0x13u32.wrapping_add(cpu.ebp)), 1u32); cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x103180c; continue; }
  pc = 0x1031808; continue;
 }
 0x1031808 => {
  { let v: u8 = 0x1u8; cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  pc = 0x103180c; continue;
 }
 0x103180c => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103180e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103180e); }
 let mut pc: u32 = 0x103180e;
 loop { match pc {
 0x103180e => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x1122f8cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1122f8eu32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xdu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xdu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xb38u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1031896; continue; }
  pc = 0x1031837; continue;
 }
 0x1031837 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1031896; continue; }
  pc = 0x1031840; continue;
 }
 0x1031840 => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1031896; continue; }
  pc = 0x1031846; continue;
 }
 0x1031846 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1031896; continue; }
  pc = 0x103184c; continue;
 }
 0x103184c => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1031853); cpu.call(t); if cpu.returned_to != 0x1031853 { return; }
  { cpu.popad(); }
  cpu.push(0x1031859); f_0103258d(cpu); if cpu.returned_to != 0x1031859 { return; }
  if !cpu.cf { pc = 0x1031896; continue; }
  pc = 0x103185b; continue;
 }
 0x103185b => {
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x1031896; continue; }
  pc = 0x1031880; continue;
 }
 0x1031880 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x1031896; continue; }
  pc = 0x1031889; continue;
 }
 0x1031889 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11u32.wrapping_add(cpu.ebp))); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0xcu8); }
  if !cpu.zf { pc = 0x1031896; continue; }
  pc = 0x1031892; continue;
 }
 0x1031892 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1031896; continue;
 }
 0x1031896 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01031898(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1031898); }
 let mut pc: u32 = 0x1031898;
 loop { match pc {
 0x1031898 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x1122f8cu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1122f8eu32) as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x26u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x27u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xb51u32); cpu.ecx = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10318fb; continue; }
  pc = 0x10318c2; continue;
 }
 0x10318c2 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10318fb; continue; }
  pc = 0x10318cb; continue;
 }
 0x10318cb => {
  { cpu.sub16((cpu.eax as u16), 0xfff0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10318fb; continue; }
  pc = 0x10318d1; continue;
 }
 0x10318d1 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10318fb; continue; }
  pc = 0x10318d7; continue;
 }
 0x10318d7 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10318de); cpu.call(t); if cpu.returned_to != 0x10318de { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0xau32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x10318fb; continue; }
  pc = 0x10318e5; continue;
 }
 0x10318e5 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x10318fb; continue; }
  pc = 0x10318ee; continue;
 }
 0x10318ee => {
  { let v: u8 = cpu.inc8(cpu.r8(0x12u32.wrapping_add(cpu.ebp))); cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if !cpu.zf { pc = 0x10318fb; continue; }
  pc = 0x10318f7; continue;
 }
 0x10318f7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10318fb; continue;
 }
 0x10318fb => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010318fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10318fd); }
 let mut pc: u32 = 0x10318fd;
 loop { match pc {
 0x10318fd => {
  cpu.push(0x1031902); f_0103258d(cpu); if cpu.returned_to != 0x1031902 { return; }
  if !cpu.cf { pc = 0x1031956; continue; }
  pc = 0x1031904; continue;
 }
 0x1031904 => {
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x1031956; continue; }
  pc = 0x1031929; continue;
 }
 0x1031929 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x1fu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x84au32; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1031954); cpu.call(t); if cpu.returned_to != 0x1031954 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1031956; continue;
 }
 0x1031956 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01031957(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1031957); }
 let mut pc: u32 = 0x1031957;
 loop { match pc {
 0x1031957 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xfffeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x82bu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1031986); cpu.call(t); if cpu.returned_to != 0x1031986 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0xau32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x10319a9; continue; }
  pc = 0x103198e; continue;
 }
 0x103198e => {
  { cpu.sub8(cpu.r8(0xau32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10319a9; continue; }
  pc = 0x1031994; continue;
 }
 0x1031994 => {
  cpu.push(0x1031999); f_010176bf(cpu); if cpu.returned_to != 0x1031999 { return; }
  if cpu.cf { pc = 0x10319a9; continue; }
  pc = 0x103199b; continue;
 }
 0x103199b => {
  { cpu.pushad(); }
  cpu.push(0x10319a1); f_010319e9(cpu); if cpu.returned_to != 0x10319a1 { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.inc8(cpu.r8(0x14u32.wrapping_add(cpu.ebp))); cpu.w8(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x14u32.wrapping_add(cpu.ebp)), 0x7u8); cpu.w8(0x14u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10319a9; continue;
 }
 0x10319a9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010319aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10319aa); }
 let mut pc: u32 = 0x10319aa;
 loop { match pc {
 0x10319aa => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)) & 0xfu8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x8u8); }
  { let v: u8 = (cpu.zf) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x83bu32); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10319e6); cpu.call(t); if cpu.returned_to != 0x10319e6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010319e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10319e9); }
 let mut pc: u32 = 0x10319e9;
 loop { match pc {
 0x10319e9 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1031a1e; continue; }
  pc = 0x10319f1; continue;
 }
 0x10319f1 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1031a1b; continue; }
  pc = 0x10319f7; continue;
 }
 0x10319f7 => {
  cpu.push(0x10319fc); f_0105f178(cpu); if cpu.returned_to != 0x10319fc { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x23u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0xfff6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1031a19); f_0101c251(cpu); if cpu.returned_to != 0x1031a19 { return; }
  pc = 0x1031a1e; continue;
 }
 0x1031a1b => {
  { let v: u8 = cpu.dec8(cpu.r8(0x15u32.wrapping_add(cpu.ebp))); cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1031a1e; continue;
 }
 0x1031a1e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01031a1f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1031a1f); }
 let mut pc: u32 = 0x1031a1f;
 loop { match pc {
 0x1031a1f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  cpu.push(0x1031a2a); f_0103258d(cpu); if cpu.returned_to != 0x1031a2a { return; }
  if !cpu.cf { pc = 0x1031aa5; continue; }
  pc = 0x1031a2c; continue;
 }
 0x1031a2c => {
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x1031aa5; continue; }
  pc = 0x1031a51; continue;
 }
 0x1031a51 => {
  { cpu.pushad(); }
  { let v: u16 = 0x1au16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x44u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1031a73; continue;
 }
 0x1031a73 => {
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x84bu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.adc16((cpu.ecx as u16), 0x0u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.ebx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.edx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1031a96); cpu.call(t); if cpu.returned_to != 0x1031a96 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), 0x3u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1031a73; continue; }
  pc = 0x1031aa4; continue;
 }
 0x1031aa4 => {
  { cpu.popad(); }
  pc = 0x1031aa5; continue;
 }
 0x1031aa5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01031aa6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1031aa6); }
 let mut pc: u32 = 0x1031aa6;
 loop { match pc {
 0x1031aa6 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1031c7b; continue; }
  pc = 0x1031ab3; continue;
 }
 0x1031ab3 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11eb7d8u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x52u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4650u32); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1122faau32.wrapping_add(cpu.eax)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x1031b15; continue; }
  pc = 0x1031b01; continue;
 }
 0x1031b01 => {
  cpu.push(0x1031b06); f_010324eb(cpu); if cpu.returned_to != 0x1031b06 { return; }
  if !cpu.cf { pc = 0x1031c7b; continue; }
  pc = 0x1031b0c; continue;
 }
 0x1031b0c => {
  { let v: u8 = 0x1u8; cpu.w8(0x11eb7d8u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1031b15 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd43du32.wrapping_add(cpu.ecx)), 0xffu8); }
  if !cpu.zf { pc = 0x1031c7b; continue; }
  pc = 0x1031b2f; continue;
 }
 0x1031b2f => {
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x1122f94u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3f43u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x1122f96u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3f45u32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x10ef20au32, v); }
  { let v: u32 = ((cpu.eax >> 8) as u8) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x78u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x10ef209u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef37bu32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1031b78); f_0103fb02(cpu); if cpu.returned_to != 0x1031b78 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef37bu32, v); }
  if cpu.cf { pc = 0x1031c7b; continue; }
  pc = 0x1031b86; continue;
 }
 0x1031b86 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x10fcc4du32, v); }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1031ba8; continue; }
  pc = 0x1031ba1; continue;
 }
 0x1031ba1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fcc4du32, v); }
  pc = 0x1031ba8; continue;
 }
 0x1031ba8 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e3f7cu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x13u32); cpu.ebx = v; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  pc = 0x1031bb9; continue;
 }
 0x1031bb9 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x1031c6f; continue; }
  pc = 0x1031bc7; continue;
 }
 0x1031bc7 => {
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x10fcc4du32), 0x0u8); }
  if !cpu.zf { pc = 0x1031be9; continue; }
  pc = 0x1031bd8; continue;
 }
 0x1031bd8 => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.eax = v; }
  cpu.push(0x1031be2); f_0102e964(cpu); if cpu.returned_to != 0x1031be2 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10fcc4du32, v); }
  pc = 0x1031be9; continue;
 }
 0x1031be9 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_sub(0x54u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u16)); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u16)); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u16)); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u16)); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3f43u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3f45u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x1122f98u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x54u32); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x1124e88u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u32 = 0xabbu32; cpu.eax = v; }
  cpu.push(0x1031c58); f_01059dbe(cpu); if cpu.returned_to != 0x1031c58 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1031c5e); f_01036a75(cpu); if cpu.returned_to != 0x1031c5e { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x1124e88u32, v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x1031c6f; continue;
 }
 0x1031c6f => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1031bb9; continue; }
  pc = 0x1031c78; continue;
 }
 0x1031c78 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1031c7b => {
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01031c81(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1031c81); }
 let mut pc: u32 = 0x1031c81;
 loop { match pc {
 0x1031c81 => {
  { let v: u32 = 0x64u32; cpu.eax = v; }
  { let v = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.mul16(v); }
  { let v = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.div16(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1123001u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u8 = 0x64u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01031ca9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1031ca9); }
 let mut pc: u32 = 0x1031ca9;
 loop { match pc {
 0x1031ca9 => {
  { let v: u32 = 0x64u32; cpu.eax = v; }
  { let v = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.mul16(v); }
  { let v = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.div16(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1123001u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u8 = 0x64u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01031cd1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1031cd1); }
 let mut pc: u32 = 0x1031cd1;
 loop { match pc {
 0x1031cd1 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x1031cfb; continue; }
  pc = 0x1031cdc; continue;
 }
 0x1031cdc => {
  { cpu.sub8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x1031d10; continue; }
  pc = 0x1031ce1; continue;
 }
 0x1031ce1 => {
  { cpu.sub8((cpu.ecx as u8), 0x3u8); }
  if cpu.zf { pc = 0x1031d25; continue; }
  pc = 0x1031ce6; continue;
 }
 0x1031ce6 => {
  { let v: u16 = cpu.r16(0x10e3d2bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3d2du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1031cfb => {
  { let v: u16 = cpu.r16(0x10e3d2fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3d31u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1031d10 => {
  { let v: u16 = cpu.r16(0x10e3d33u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3d35u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1031d25 => {
  { let v: u16 = cpu.r16(0x10e3d37u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3d39u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01031d3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1031d3a); }
 let mut pc: u32 = 0x1031d3a;
 loop { match pc {
 0x1031d3a => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1031d41); f_01031d6b(cpu); if cpu.returned_to != 0x1031d41 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1123001u32, v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1031d4d); f_01031d6b(cpu); if cpu.returned_to != 0x1031d4d { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1123002u32, v); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1031d59); f_01031d6b(cpu); if cpu.returned_to != 0x1031d59 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1123003u32, v); }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1031d65); f_01031d6b(cpu); if cpu.returned_to != 0x1031d65 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1123004u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01031d6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1031d6b); }
 let mut pc: u32 = 0x1031d6b;
 loop { match pc {
 0x1031d6b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u8 = 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x1031d76; continue;
 }
 0x1031d76 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if (cpu.sf != cpu.of) { pc = 0x1031d86; continue; }
  pc = 0x1031d7d; continue;
 }
 0x1031d7d => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x8u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x1031d86; continue; }
  pc = 0x1031d84; continue;
 }
 0x1031d84 => {
  { let v: u8 = cpu.inc8(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1031d86; continue;
 }
 0x1031d86 => {
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1031d76; continue; }
  pc = 0x1031d8d; continue;
 }
 0x1031d8d => {
  { let v: u32 = 0x64u32; cpu.eax = v; }
  { let v = ((cpu.ebx >> 8) as u8); cpu.mul8(v); }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01031d9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1031d9e); }
 let mut pc: u32 = 0x1031d9e;
 loop { match pc {
 0x1031d9e => {
  { let v: u8 = 0x8u8; cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x112292cu32; cpu.esi = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1031dc7); f_0102f786(cpu); if cpu.returned_to != 0x1031dc7 { return; }
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1122e2eu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x1031dec); f_01072a54(cpu); if cpu.returned_to != 0x1031dec { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01031ded(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1031ded); }
 let mut pc: u32 = 0x1031ded;
 loop { match pc {
 0x1031ded => {
  { let v: u8 = 0x0u8; cpu.w8(0x1123000u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1031f2c; continue; }
  pc = 0x1031e17; continue;
 }
 0x1031e17 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1031f2c; continue; }
  pc = 0x1031e24; continue;
 }
 0x1031e24 => {
  { cpu.sub16((cpu.eax as u16), 0xffc0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1031f2c; continue; }
  pc = 0x1031e2e; continue;
 }
 0x1031e2e => {
  { cpu.sub16((cpu.ebx as u16), 0xff90u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1031f2c; continue; }
  pc = 0x1031e38; continue;
 }
 0x1031e38 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1032106; continue; }
  pc = 0x1031e44; continue;
 }
 0x1031e44 => {
  { let v: u8 = 0x1u8; cpu.w8(0x1123000u32, v); }
  { let v: u32 = 0x11228bcu32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10f78bcu32) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1122f8cu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1122f8eu32, v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1031e6f); cpu.call(t); if cpu.returned_to != 0x1031e6f { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x1031eac; continue; }
  pc = 0x1031e7d; continue;
 }
 0x1031e7d => {
  { let v: u8 = cpu.r8(0x10ee950u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x1031e8a); cpu.call(t); if cpu.returned_to != 0x1031e8a { return; }
  { let v: u16 = cpu.r16(0x1122f8cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1122f8eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11230c7u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1031eac); cpu.call(t); if cpu.returned_to != 0x1031eac { return; }
  pc = 0x1031eac; continue;
 }
 0x1031eac => {
  { cpu.popad(); }
  cpu.push(0x1031eb2); f_0102f5b3(cpu); if cpu.returned_to != 0x1031eb2 { return; }
  cpu.push(0x1031eb7); f_01032222(cpu); if cpu.returned_to != 0x1031eb7 { return; }
  cpu.push(0x1031ebc); f_01032261(cpu); if cpu.returned_to != 0x1031ebc { return; }
  cpu.push(0x1031ec1); f_01032305(cpu); if cpu.returned_to != 0x1031ec1 { return; }
  cpu.push(0x1031ec6); f_0103258d(cpu); if cpu.returned_to != 0x1031ec6 { return; }
  if !cpu.cf { pc = 0x1031f2c; continue; }
  pc = 0x1031ec8; continue;
 }
 0x1031ec8 => {
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x1031ef5; continue; }
  pc = 0x1031eed; continue;
 }
 0x1031eed => {
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x101u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1031ef5; continue;
 }
 0x1031ef5 => {
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x5u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x1031f2c; continue; }
  pc = 0x1031f1a; continue;
 }
 0x1031f1a => {
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1031f23); f_010322d4(cpu); if cpu.returned_to != 0x1031f23 { return; }
  { let v: u16 = 0x101u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1031f2c); f_010322d4(cpu); if cpu.returned_to != 0x1031f2c { return; }
  pc = 0x1031f2c; continue;
 }
 0x1031f2c => {
  { cpu.sub8(cpu.r8(0x11cc2b7u32), 0x0u8); }
  if !cpu.zf { pc = 0x10320f2; continue; }
  pc = 0x1031f39; continue;
 }
 0x1031f39 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x103215e; continue; }
  pc = 0x1031f45; continue;
 }
 0x1031f45 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x4u8); }
  if !cpu.cf { pc = 0x10320f2; continue; }
  pc = 0x1031f52; continue;
 }
 0x1031f52 => {
  { cpu.sub16(cpu.r16(0x34u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1031f62; continue; }
  pc = 0x1031f5a; continue;
 }
 0x1031f5a => {
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x1031fc3; continue;
 }
 0x1031f62 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1031fc3; continue; }
  pc = 0x1031f68; continue;
 }
 0x1031f68 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1031f77; continue; }
  pc = 0x1031f71; continue;
 }
 0x1031f71 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x1031f7f; continue; }
  pc = 0x1031f77; continue;
 }
 0x1031f77 => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1031f96; continue; }
  pc = 0x1031f7d; continue;
 }
 0x1031f7d => {
  pc = 0x1031fb0; continue;
 }
 0x1031f7f => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1031fb9; continue; }
  pc = 0x1031f85; continue;
 }
 0x1031f85 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11eb7d8u32, v); }
  cpu.push(0x1031f91); f_0108c3a6(cpu); if cpu.returned_to != 0x1031f91 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1031fb0; continue;
 }
 0x1031f96 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1031fb9; continue; }
  pc = 0x1031f9c; continue;
 }
 0x1031f9c => {
  { let v: u16 = cpu.r16(0x10e3d7bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1031fa7); f_01016ace(cpu); if cpu.returned_to != 0x1031fa7 { return; }
  if cpu.cf { pc = 0x10320f2; continue; }
  pc = 0x1031fad; continue;
 }
 0x1031fad => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1031fb0; continue;
 }
 0x1031fb0 => {
  cpu.push(0x1031fb5); f_010316a1(cpu); if cpu.returned_to != 0x1031fb5 { return; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1031fb9; continue;
 }
 0x1031fb9 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10320f2; continue; }
  pc = 0x1031fc3; continue;
 }
 0x1031fc3 => {
  { cpu.sub16(cpu.r16(0x34u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1031fee; continue; }
  pc = 0x1031fcb; continue;
 }
 0x1031fcb => {
  { cpu.and16(cpu.r16(0x10e3d07u32), 0x2u16); }
  if cpu.zf { pc = 0x10320f2; continue; }
  pc = 0x1031fda; continue;
 }
 0x1031fda => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = 0x20u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x39u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1031fe9); f_010321ae(cpu); if cpu.returned_to != 0x1031fe9 { return; }
  pc = 0x10320f2; continue;
 }
 0x1031fee => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x1123000u32), 0x0u8); }
  if cpu.zf { pc = 0x1032002; continue; }
  pc = 0x1031ffb; continue;
 }
 0x1031ffb => {
  { let v: u8 = 0x2u8; cpu.w8(0x110d739u32, v); }
  pc = 0x1032002; continue;
 }
 0x1032002 => {
  { let v: u16 = 0x20u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x39u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103200f); f_01030ba5(cpu); if cpu.returned_to != 0x103200f { return; }
  cpu.push(0x1032014); f_010176bf(cpu); if cpu.returned_to != 0x1032014 { return; }
  if cpu.cf { pc = 0x103201b; continue; }
  pc = 0x1032016; continue;
 }
 0x1032016 => {
  cpu.push(0x103201b); f_010321f4(cpu); if cpu.returned_to != 0x103201b { return; }
  pc = 0x103201b; continue;
 }
 0x103201b => {
  cpu.push(0x1032020); f_01031ca9(cpu); if cpu.returned_to != 0x1032020 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10320f2; continue; }
  pc = 0x103202d; continue;
 }
 0x103202d => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x103203d; continue; }
  pc = 0x1032036; continue;
 }
 0x1032036 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x12u32.wrapping_add(cpu.ebp))); cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)) & 0x7u8; cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  pc = 0x103203d; continue;
 }
 0x103203d => {
  { let v: u32 = 0x1bu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.dec16(cpu.r16(0xau32.wrapping_add(cpu.ebp))); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0xau32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x1032054; continue; }
  pc = 0x1032050; continue;
 }
 0x1032050 => {
  { let v: u16 = cpu.neg16(cpu.r16(0xau32.wrapping_add(cpu.ebp))); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x1032054; continue;
 }
 0x1032054 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r32(0x10e4138u32); cpu.push(v); }
  cpu.push(0x1032062); f_0101776f(cpu); if cpu.returned_to != 0x1032062 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4138u32, v); }
  if cpu.cf { pc = 0x10320f2; continue; }
  pc = 0x103206e; continue;
 }
 0x103206e => {
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  if (cpu.sf == cpu.of) { pc = 0x10320f2; continue; }
  pc = 0x1032078; continue;
 }
 0x1032078 => {
  cpu.push(0x103207d); f_010316a1(cpu); if cpu.returned_to != 0x103207d { return; }
  { let v: u16 = 0x44u16; cpu.w16(0x34u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.pushad(); }
  cpu.push(0x1032093); f_0103236f(cpu); if cpu.returned_to != 0x1032093 { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x11eb7d8u32), 0x0u8); }
  if !cpu.zf { pc = 0x103216b; continue; }
  pc = 0x10320a1; continue;
 }
 0x10320a1 => {
  { let v: u32 = 0x0u32; cpu.w32(0x2au32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x10320c4; continue; }
  pc = 0x10320b1; continue;
 }
 0x10320b1 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x10320c4; continue; }
  pc = 0x10320ba; continue;
 }
 0x10320ba => {
  cpu.push(0x10320bf); f_0108c3a6(cpu); if cpu.returned_to != 0x10320bf { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10320f2; continue;
 }
 0x10320c4 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x10320e1; continue; }
  pc = 0x10320ce; continue;
 }
 0x10320ce => {
  { let v = cpu.r32(0x10e4138u32); cpu.push(v); }
  cpu.push(0x10320d9); f_01017741(cpu); if cpu.returned_to != 0x10320d9 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4138u32, v); }
  if !cpu.cf { pc = 0x10320f2; continue; }
  pc = 0x10320e1; continue;
 }
 0x10320e1 => {
  { let v: u8 = 0xffu8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x10320f2; continue;
 }
 0x10320f2 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1032105); f_01072b15(cpu); if cpu.returned_to != 0x1032105 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1032106 => {
  { cpu.pushad(); }
  { let v: u32 = 0x11228bcu32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10f78bcu32) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103211e); cpu.call(t); if cpu.returned_to != 0x103211e { return; }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if (cpu.sf != cpu.of) { pc = 0x1032131; continue; }
  pc = 0x103212d; continue;
 }
 0x103212d => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x103215e; continue;
 }
 0x1032131 => {
  { let v: u8 = cpu.r8(0x10ee950u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x103213e); cpu.call(t); if cpu.returned_to != 0x103213e { return; }
  { cpu.sub32(cpu.ebx, 0x4u32); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.sf == cpu.of) { pc = 0x103215e; continue; }
  pc = 0x1032145; continue;
 }
 0x1032145 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x11230c7u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103215d); cpu.call(t); if cpu.returned_to != 0x103215d { return; }
  { cpu.popad(); }
  pc = 0x103215e; continue;
 }
 0x103215e => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = 0xfffeu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x103216a); f_01072b15(cpu); if cpu.returned_to != 0x103216a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103216b => {
  { let v: u32 = cpu.r32(0x2au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x116e72cu32).wrapping_add(cpu.eax); cpu.w32(0x116e72cu32, v); }
  cpu.push(0x1032179); f_0108c3a6(cpu); if cpu.returned_to != 0x1032179 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11eb7d8u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10320f2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032188(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032188); }
 let mut pc: u32 = 0x1032188;
 loop { match pc {
 0x1032188 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1032191); f_01031c81(cpu); if cpu.returned_to != 0x1032191 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10321a7; continue; }
  pc = 0x1032197; continue;
 }
 0x1032197 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  pc = 0x10321a7; continue;
 }
 0x10321a7 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010321ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10321ae); }
 let mut pc: u32 = 0x10321ae;
 loop { match pc {
 0x10321ae => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = 0x3cu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = ((cpu.eax >> 8) as u8) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x112d78fu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x112d793u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  cpu.push(0x10321f2); f_01017083(cpu); if cpu.returned_to != 0x10321f2 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010321f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10321f4); }
 let mut pc: u32 = 0x10321f4;
 loop { match pc {
 0x10321f4 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x103221e; continue; }
  pc = 0x10321fa; continue;
 }
 0x10321fa => {
  cpu.push(0x10321ff); f_0105f178(cpu); if cpu.returned_to != 0x10321ff { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x22u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0xfff3u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x103221c); f_0101c251(cpu); if cpu.returned_to != 0x103221c { return; }
  pc = 0x1032221; continue;
 }
 0x103221e => {
  { let v: u8 = cpu.dec8(cpu.r8(0x13u32.wrapping_add(cpu.ebp))); cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1032221; continue;
 }
 0x1032221 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032222(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032222); }
 let mut pc: u32 = 0x1032222;
 loop { match pc {
 0x1032222 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)) & 0xfu8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 2u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x847u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103225e); cpu.call(t); if cpu.returned_to != 0x103225e { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032261(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032261); }
 let mut pc: u32 = 0x1032261;
 loop { match pc {
 0x1032261 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x11u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x83du16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1032291); cpu.call(t); if cpu.returned_to != 0x1032291 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x1032298); f_0103258d(cpu); if cpu.returned_to != 0x1032298 { return; }
  if !cpu.cf { pc = 0x10322a1; continue; }
  pc = 0x103229a; continue;
 }
 0x103229a => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11u32.wrapping_add(cpu.ebp))); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x3u8); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10322a1; continue;
 }
 0x10322a1 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010322a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10322a3); }
 let mut pc: u32 = 0x10322a3;
 loop { match pc {
 0x10322a3 => {
  { cpu.pushad(); }
  { let v: u16 = 0x845u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.eax >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x36u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10322d2); cpu.call(t); if cpu.returned_to != 0x10322d2 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010322d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10322d4); }
 let mut pc: u32 = 0x10322d4;
 loop { match pc {
 0x10322d4 => {
  { cpu.pushad(); }
  { let v: u16 = 0x843u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.eax >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1du8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xdu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2fu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1032303); cpu.call(t); if cpu.returned_to != 0x1032303 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032305(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032305); }
 let mut pc: u32 = 0x1032305;
 loop { match pc {
 0x1032305 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x17u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x27u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebp)), 0x0u16); }
  { let v: u8 = (cpu.sf) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x841u32); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103233b); cpu.call(t); if cpu.returned_to != 0x103233b { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xffffu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x833u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103236c); cpu.call(t); if cpu.returned_to != 0x103236c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103236f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103236f); }
 let mut pc: u32 = 0x103236f;
 loop { match pc {
 0x103236f => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x10324ea; continue; }
  pc = 0x103237c; continue;
 }
 0x103237c => {
  { let v: u8 = 0x0u8; cpu.w8(0x11eb7d8u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x52u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4650u32); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1122faau32.wrapping_add(cpu.eax)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x10323de; continue; }
  pc = 0x10323ca; continue;
 }
 0x10323ca => {
  cpu.push(0x10323cf); f_010324eb(cpu); if cpu.returned_to != 0x10323cf { return; }
  if !cpu.cf { pc = 0x10324ea; continue; }
  pc = 0x10323d5; continue;
 }
 0x10323d5 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11eb7d8u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10323de => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd43du32.wrapping_add(cpu.ecx)), 0xffu8); }
  if !cpu.zf { pc = 0x10324ea; continue; }
  pc = 0x10323f2; continue;
 }
 0x10323f2 => {
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x17u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3f43u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3f45u32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x10ef20au32, v); }
  { let v: u32 = ((cpu.eax >> 8) as u8) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x78u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x10ef209u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef37bu32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1032435); f_0103fb02(cpu); if cpu.returned_to != 0x1032435 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef37bu32, v); }
  if cpu.cf { pc = 0x10324ea; continue; }
  pc = 0x1032443; continue;
 }
 0x1032443 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1032473; continue; }
  pc = 0x1032457; continue;
 }
 0x1032457 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e3f7cu32); cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let a = cpu.ebp; let b = cpu.eax; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.eax = v; } }
  cpu.push(0x1032472); f_0102e964(cpu); if cpu.returned_to != 0x1032472 { return; }
  { cpu.popad(); }
  pc = 0x1032473; continue;
 }
 0x1032473 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e3f7cu32); cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10e3f43u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3f45u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x38u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x1124e88u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u32 = 0xabbu32; cpu.eax = v; }
  cpu.push(0x10324d3); f_01059dbe(cpu); if cpu.returned_to != 0x10324d3 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10324d9); f_01036a75(cpu); if cpu.returned_to != 0x10324d9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x1124e88u32, v); }
  { let v: u8 = cpu.or8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x10u8); cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  pc = 0x10324ea; continue;
 }
 0x10324ea => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010324eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10324eb); }
 let mut pc: u32 = 0x10324eb;
 loop { match pc {
 0x10324eb => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x103251c; continue; }
  pc = 0x10324f4; continue;
 }
 0x10324f4 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x103251c; continue; }
  pc = 0x10324fd; continue;
 }
 0x10324fd => {
  { let a = cpu.r8(0x10ef37du32); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x10ef37du32, v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x110eac4u32); cpu.push(v); }
  cpu.push(0x103250f); f_0108c4ab(cpu); if cpu.returned_to != 0x103250f { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x110eac4u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef37du32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103251c => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x1032572; continue; }
  pc = 0x103252d; continue;
 }
 0x103252d => {
  { cpu.sub32(cpu.r32(0x110eab8u32), 0x88u32); }
  if (cpu.sf == cpu.of) { pc = 0x1032572; continue; }
  pc = 0x1032539; continue;
 }
 0x1032539 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10fcc0eu32, v); }
  { let v: u32 = 0xddu32; cpu.w32(0x110eab8u32, v); }
  { let v: u32 = 0x110edb3u32; cpu.w32(0x110eabcu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x110eac0u32, v); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1032572; continue; }
  pc = 0x1032566; continue;
 }
 0x1032566 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.push(0x1032571); f_010363b2(cpu); if cpu.returned_to != 0x1032571 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1032572; continue;
 }
 0x1032572 => {
  cpu.push(0x1032577); f_01032579(cpu); if cpu.returned_to != 0x1032577 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032579(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032579); }
 let mut pc: u32 = 0x1032579;
 loop { match pc {
 0x1032579 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4bu32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103258d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103258d); }
 let mut pc: u32 = 0x103258d;
 loop { match pc {
 0x103258d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x3u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x103259a; continue; }
  pc = 0x1032597; continue;
 }
 0x1032597 => {
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103259a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103259d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103259d); }
 let mut pc: u32 = 0x103259d;
 loop { match pc {
 0x103259d => {
  { let v: u32 = 0x11dbce8u32; cpu.esi = v; }
  { let v: u32 = 0x11dbd74u32; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1122890u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x112288cu32, v); }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x10325b8; continue;
 }
 0x10325b8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10325b8; continue; }
  pc = 0x10325cc; continue;
 }
 0x10325cc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010325cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10325cd); }
 let mut pc: u32 = 0x10325cd;
 loop { match pc {
 0x10325cd => {
  { let v: u32 = 0x0u32; cpu.w32(0x10ee944u32, v); }
  { let v: u32 = 0x116c0d8u32; cpu.esi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x10325e1; continue;
 }
 0x10325e1 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103260b; continue; }
  pc = 0x10325e8; continue;
 }
 0x10325e8 => {
  { let v: u16 = (cpu.eax as u16) ^ (cpu.eax as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x103260b; continue; }
  pc = 0x10325f5; continue;
 }
 0x10325f5 => {
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x103260b; continue; }
  pc = 0x10325ff; continue;
 }
 0x10325ff => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x103260b; continue; }
  pc = 0x1032605; continue;
 }
 0x1032605 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10ee944u32.wrapping_add(cpu.eax))); cpu.w8(0x10ee944u32.wrapping_add(cpu.eax), v); }
  pc = 0x103260b; continue;
 }
 0x103260b => {
  { let v: u32 = cpu.add32(cpu.esi, 0x50u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10325e1; continue; }
  pc = 0x1032610; continue;
 }
 0x1032610 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032611(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032611); }
 let mut pc: u32 = 0x1032611;
 loop { match pc {
 0x1032611 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032612(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032612); }
 let mut pc: u32 = 0x1032612;
 loop { match pc {
 0x1032612 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_01032614(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032614); }
 let mut pc: u32 = 0x1032614;
 loop { match pc {
 0x1032614 => {
  { let v: u32 = cpu.r32(0x10e3d63u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10e438cu32; cpu.esi = v; }
  cpu.push(0x1032624); f_0100ac15(cpu); if cpu.returned_to != 0x1032624 { return; }
  let t = cpu.r32(0x10e434cu32); cpu.push(0x103262a); cpu.call(t); if cpu.returned_to != 0x103262a { return; }
  { let v: u8 = 0x6u8; cpu.w8(0x10e3d63u32, v); }
  cpu.push(0x1032636); f_0103ec02(cpu); if cpu.returned_to != 0x1032636 { return; }
  cpu.push(0x103263b); f_0103ec28(cpu); if cpu.returned_to != 0x103263b { return; }
  cpu.push(0x1032640); f_0103e93b(cpu); if cpu.returned_to != 0x1032640 { return; }
  let t = cpu.r32(0x10e4334u32); cpu.push(0x1032646); cpu.call(t); if cpu.returned_to != 0x1032646 { return; }
  { let v: u32 = cpu.r32(0x11ceb58u32); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0xb66u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e4308u32); cpu.push(0x1032661); cpu.call(t); if cpu.returned_to != 0x1032661 { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.push(0x103266b); f_01032ac8(cpu); if cpu.returned_to != 0x103266b { return; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x1032675); f_01032b00(cpu); if cpu.returned_to != 0x1032675 { return; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x103267f); f_01032b4c(cpu); if cpu.returned_to != 0x103267f { return; }
  { cpu.sub8(cpu.r8(0x1105e6au32), 0x0u8); }
  if cpu.zf { pc = 0x10326b8; continue; }
  pc = 0x1032688; continue;
 }
 0x1032688 => {
  { let v: u32 = 0x10e442cu32; cpu.esi = v; }
  cpu.push(0x1032692); f_0100ac15(cpu); if cpu.returned_to != 0x1032692 { return; }
  let t = cpu.r32(0x10e434cu32); cpu.push(0x1032698); cpu.call(t); if cpu.returned_to != 0x1032698 { return; }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.push(0x10326a2); f_01032ac8(cpu); if cpu.returned_to != 0x10326a2 { return; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x10326ac); f_01032b00(cpu); if cpu.returned_to != 0x10326ac { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x10326b6); f_01032b4c(cpu); if cpu.returned_to != 0x10326b6 { return; }
  pc = 0x10326b8; continue;
 }
 0x10326b8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1129914u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11cea28u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1129916u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11cea2au32, v); }
  cpu.push(0x10326d8); f_01032991(cpu); if cpu.returned_to != 0x10326d8 { return; }
  cpu.push(0x10326dd); f_01032a1c(cpu); if cpu.returned_to != 0x10326dd { return; }
  cpu.push(0x10326e2); f_01072682(cpu); if cpu.returned_to != 0x10326e2 { return; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x11cea1cu32; cpu.edi = v; }
  { cpu.stos(2, true); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3d63u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010327e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10327e1); }
 let mut pc: u32 = 0x10327e1;
 loop { match pc {
 0x10327e1 => {
  cpu.push(0x10327e6); f_01001552(cpu); if cpu.returned_to != 0x10327e6 { return; }
  { let v: u32 = 0x11bd8ffu32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { cpu.pushad(); }
  cpu.push(0x10327f6); f_010019bc(cpu); if cpu.returned_to != 0x10327f6 { return; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  { cpu.popad(); }
  if !cpu.zf { pc = 0x1032818; continue; }
  pc = 0x10327fd; continue;
 }
 0x10327fd => {
  { let v: u32 = 0x11bd8ffu32; cpu.eax = v; }
  { let v: u32 = 0x11bd917u32; cpu.ebx = v; }
  { let v: u32 = 0x11bd92du32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1032818); f_01001c47(cpu); if cpu.returned_to != 0x1032818 { return; }
  pc = 0x1032818; continue;
 }
 0x1032818 => {
  { let v: u32 = 0x11bd8ffu32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { cpu.pushad(); }
  cpu.push(0x1032828); f_010018d7(cpu); if cpu.returned_to != 0x1032828 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d8fu32, v); }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x10e3d8fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.sf { pc = 0x1032990; continue; }
  pc = 0x103283e; continue;
 }
 0x103283e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d8fu32, v); }
  cpu.push(0x1032849); f_0105e77c(cpu); if cpu.returned_to != 0x1032849 { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x11bd8ffu32; cpu.edx = v; }
  { let v: u32 = 0x11bd9adu32; cpu.ebx = v; }
  { let v: u32 = 0x8000u32; cpu.ecx = v; }
  cpu.push(0x1032862); f_0105e70b(cpu); if cpu.returned_to != 0x1032862 { return; }
  { let v: u32 = 0x11bd936u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11cea1cu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1032875); f_01032aa7(cpu); if cpu.returned_to != 0x1032875 { return; }
  { let v: u32 = 0x11bd943u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11cea1eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1032888); f_01032aa7(cpu); if cpu.returned_to != 0x1032888 { return; }
  { let v: u32 = 0x11bd950u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11cea20u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x103289b); f_01032aa7(cpu); if cpu.returned_to != 0x103289b { return; }
  { let v: u32 = 0x11bd95du32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11cea22u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x10328ae); f_01032aa7(cpu); if cpu.returned_to != 0x10328ae { return; }
  { let v: u32 = 0x11bd96au32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11cea24u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x10328c1); f_01032aa7(cpu); if cpu.returned_to != 0x10328c1 { return; }
  { let v: u32 = 0x11bd977u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11cea26u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x10328d4); f_01032aa7(cpu); if cpu.returned_to != 0x10328d4 { return; }
  cpu.push(0x10328d9); f_01032991(cpu); if cpu.returned_to != 0x10328d9 { return; }
  { let v: u32 = 0x11bd97eu32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x11c59c3u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x10328eb); f_01032abe(cpu); if cpu.returned_to != 0x10328eb { return; }
  { let v: u32 = 0x11c59adu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x1129914u32), 0x3u8); }
  if cpu.zf { pc = 0x103291a; continue; }
  pc = 0x10328f9; continue;
 }
 0x10328f9 => {
  { let v: u32 = 0x11c59b1u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x1129914u32), 0x4u8); }
  if cpu.zf { pc = 0x103291a; continue; }
  pc = 0x1032907; continue;
 }
 0x1032907 => {
  { let v: u32 = 0x11c59b5u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x1129914u32), 0x5u8); }
  if cpu.zf { pc = 0x103291a; continue; }
  pc = 0x1032915; continue;
 }
 0x1032915 => {
  { let v: u32 = 0x11c59b9u32; cpu.ebx = v; }
  pc = 0x103291a; continue;
 }
 0x103291a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x11bd987u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x11bd98fu32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x1129916u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x103293a); f_0107a118(cpu); if cpu.returned_to != 0x103293a { return; }
  { let v: u32 = cpu.r32(0x11ceb58u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x0u32); }
  if cpu.zf { pc = 0x103295c; continue; }
  pc = 0x1032945; continue;
 }
 0x1032945 => {
  { let v: u32 = 0x11bd999u32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x103294f; continue;
 }
 0x103294f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x103295c; continue; }
  pc = 0x1032955; continue;
 }
 0x1032955 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103294f; continue; }
  pc = 0x103295c; continue;
 }
 0x103295c => {
  { let v: u32 = 0x11bd9adu32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10e3ff0u32)); cpu.edi = v; }
  { let v: u32 = 0x11bd92eu32; cpu.esi = v; }
  pc = 0x103296c; continue;
 }
 0x103296c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x103296c; continue; }
  pc = 0x1032976; continue;
 }
 0x1032976 => {
  { let v: u32 = 0x11bd8ffu32; cpu.eax = v; }
  { let v: u32 = 0x11bd9adu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1032990); f_01001c47(cpu); if cpu.returned_to != 0x1032990 { return; }
  pc = 0x1032990; continue;
 }
 0x1032990 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032991(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032991); }
 let mut pc: u32 = 0x1032991;
 loop { match pc {
 0x1032991 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11cea1cu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), cpu.r16(0x1129916u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x10329a5; continue; }
  pc = 0x10329a3; continue;
 }
 0x10329a3 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10329a5; continue;
 }
 0x10329a5 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x11cea1eu32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x11cea20u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x11c59c1u32, v); }
  { cpu.sub8(cpu.r8(0x1129914u32), 0x5u8); }
  if (cpu.sf == cpu.of) { pc = 0x10329f0; continue; }
  pc = 0x10329c3; continue;
 }
 0x10329c3 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c59c3u32, v); }
  { cpu.sub16(cpu.r16(0x11c59c1u32), 0x50u16); }
  if (cpu.sf == cpu.of) { pc = 0x1032a1b; continue; }
  pc = 0x10329d5; continue;
 }
 0x10329d5 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c59c3u32, v); }
  { cpu.sub16(cpu.r16(0x11c59c1u32), 0x28u16); }
  if (cpu.sf == cpu.of) { pc = 0x1032a1b; continue; }
  pc = 0x10329e7; continue;
 }
 0x10329e7 => {
  { let v: u8 = 0x2u8; cpu.w8(0x11c59c3u32, v); }
  pc = 0x1032a1b; continue;
 }
 0x10329f0 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c59c3u32, v); }
  { cpu.sub16(cpu.r16(0x11c59c1u32), 0xc8u16); }
  if (cpu.sf == cpu.of) { pc = 0x1032a1b; continue; }
  pc = 0x1032a02; continue;
 }
 0x1032a02 => {
  { let v: u8 = 0x2u8; cpu.w8(0x11c59c3u32, v); }
  { cpu.sub16(cpu.r16(0x11c59c1u32), 0x64u16); }
  if (cpu.sf == cpu.of) { pc = 0x1032a1b; continue; }
  pc = 0x1032a14; continue;
 }
 0x1032a14 => {
  { let v: u8 = 0x3u8; cpu.w8(0x11c59c3u32, v); }
  pc = 0x1032a1b; continue;
 }
 0x1032a1b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032a1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032a1c); }
 let mut pc: u32 = 0x1032a1c;
 loop { match pc {
 0x1032a1c => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x11cea22u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), cpu.r16(0x1129916u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1032a30; continue; }
  pc = 0x1032a2e; continue;
 }
 0x1032a2e => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1032a30; continue;
 }
 0x1032a30 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x11cea24u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x11cea26u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x11c59c1u32, v); }
  { cpu.sub8(cpu.r8(0x1129914u32), 0x5u8); }
  if (cpu.sf == cpu.of) { pc = 0x1032a7b; continue; }
  pc = 0x1032a4e; continue;
 }
 0x1032a4e => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b23u32, v); }
  { cpu.sub16(cpu.r16(0x11c59c1u32), 0x50u16); }
  if (cpu.sf == cpu.of) { pc = 0x1032aa6; continue; }
  pc = 0x1032a60; continue;
 }
 0x1032a60 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b23u32, v); }
  { cpu.sub16(cpu.r16(0x11c59c1u32), 0x28u16); }
  if (cpu.sf == cpu.of) { pc = 0x1032aa6; continue; }
  pc = 0x1032a72; continue;
 }
 0x1032a72 => {
  { let v: u8 = 0x2u8; cpu.w8(0x11c5b23u32, v); }
  pc = 0x1032aa6; continue;
 }
 0x1032a7b => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b23u32, v); }
  { cpu.sub16(cpu.r16(0x11c59c1u32), 0xc8u16); }
  if (cpu.sf == cpu.of) { pc = 0x1032aa6; continue; }
  pc = 0x1032a8d; continue;
 }
 0x1032a8d => {
  { let v: u8 = 0x2u8; cpu.w8(0x11c5b23u32, v); }
  { cpu.sub16(cpu.r16(0x11c59c1u32), 0x64u16); }
  if (cpu.sf == cpu.of) { pc = 0x1032aa6; continue; }
  pc = 0x1032a9f; continue;
 }
 0x1032a9f => {
  { let v: u8 = 0x3u8; cpu.w8(0x11c5b23u32, v); }
  pc = 0x1032aa6; continue;
 }
 0x1032aa6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032aa7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032aa7); }
 let mut pc: u32 = 0x1032aa7;
 loop { match pc {
 0x1032aa7 => {
  { cpu.sub32(cpu.edx, 0x0u32); }
  if cpu.zf { pc = 0x1032ab7; continue; }
  pc = 0x1032aac; continue;
 }
 0x1032aac => {
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1032ab5); f_0107a118(cpu); if cpu.returned_to != 0x1032ab5 { return; }
  pc = 0x1032abd; continue;
 }
 0x1032ab7 => {
  { let v: u32 = 0x2d2d2d2du32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1032abd; continue;
 }
 0x1032abd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032abe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032abe); }
 let mut pc: u32 = 0x1032abe;
 loop { match pc {
 0x1032abe => {
  { let v: u16 = 0x1u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1032ac7); f_0107a118(cpu); if cpu.returned_to != 0x1032ac7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032ac8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032ac8); }
 let mut pc: u32 = 0x1032ac8;
 loop { match pc {
 0x1032ac8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3f60u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3f64u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e3f13u32, v); }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x1032ae6; continue;
 }
 0x1032ae6 => {
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x10e4314u32); cpu.push(0x1032aed); cpu.call(t); if cpu.returned_to != 0x1032aed { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1032ae6; continue; }
  pc = 0x1032af1; continue;
 }
 0x1032af1 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e3f13u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x11cea1cu32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032b00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032b00); }
 let mut pc: u32 = 0x1032b00;
 loop { match pc {
 0x1032b00 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e3f13u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10c052cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4130u32, v); }
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x1032b23; continue;
 }
 0x1032b23 => {
  { cpu.pushad(); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0xb66u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1032b39); cpu.call(t); if cpu.returned_to != 0x1032b39 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1032b23; continue; }
  pc = 0x1032b3d; continue;
 }
 0x1032b3d => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e3f13u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x11cea1cu32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032b4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032b4c); }
 let mut pc: u32 = 0x1032b4c;
 loop { match pc {
 0x1032b4c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e3f13u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10c052cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4130u32, v); }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x1032b6f; continue;
 }
 0x1032b6f => {
  { cpu.pushad(); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0xb66u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e4308u32); cpu.push(0x1032b85); cpu.call(t); if cpu.returned_to != 0x1032b85 { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.inc16(cpu.r16(0x10c052cu32)); cpu.w16(0x10c052cu32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x10e4130u32)); cpu.w32(0x10e4130u32, v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1032b6f; continue; }
  pc = 0x1032b96; continue;
 }
 0x1032b96 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e3f13u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x11cea1cu32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032ba5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032ba5); }
 let mut pc: u32 = 0x1032ba5;
 loop { match pc {
 0x1032ba5 => {
  cpu.push(cpu.segments[1] as u32); cpu.push(0x1032bac); f_010aea80(cpu); if cpu.returned_to != 0x1032bac { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1129914u32, v); }
  cpu.push(cpu.segments[1] as u32); cpu.push(0x1032bb8); f_010aeaf8(cpu); if cpu.returned_to != 0x1032bb8 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1129916u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032bbf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032bbf); }
 let mut pc: u32 = 0x1032bbf;
 loop { match pc {
 0x1032bbf => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1032bc9; continue; }
  pc = 0x1032bc8; continue;
 }
 0x1032bc8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1032bc9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11ca47cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11ca47du32, v); }
  cpu.push(0x1032bdc); f_01032cb7(cpu); if cpu.returned_to != 0x1032bdc { return; }
  { let v: u32 = cpu.r32(0x11c6271u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11c6275u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11c6279u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11c627du32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11c6281u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11c6285u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11ca47cu32, v); }
  { let v: u32 = cpu.r32(0x11c6289u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6271u32, v); }
  { let v: u32 = cpu.r32(0x11c628du32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6275u32, v); }
  { let v: u32 = cpu.r32(0x11c6291u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6279u32, v); }
  { let v: u32 = cpu.r32(0x11c6295u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c627du32, v); }
  { let v: u32 = cpu.r32(0x11c6299u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6281u32, v); }
  { let v: u32 = cpu.r32(0x11c629du32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6285u32, v); }
  cpu.push(0x1032c48); f_01032cb7(cpu); if cpu.returned_to != 0x1032c48 { return; }
  { let v: u32 = cpu.r32(0x11c6271u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6289u32, v); }
  { let v: u32 = cpu.r32(0x11c6275u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c628du32, v); }
  { let v: u32 = cpu.r32(0x11c6279u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6291u32, v); }
  { let v: u32 = cpu.r32(0x11c627du32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6295u32, v); }
  { let v: u32 = cpu.r32(0x11c6281u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6299u32, v); }
  { let v: u32 = cpu.r32(0x11c6285u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c629du32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6285u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6281u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c627du32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6279u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6275u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6271u32, v); }
  { cpu.sub8(cpu.r8(0x11ca47du32), 0x0u8); }
  if cpu.zf { pc = 0x1032cb6; continue; }
  pc = 0x1032cb1; continue;
 }
 0x1032cb1 => {
  cpu.push(0x1032cb6); f_0103313f(cpu); if cpu.returned_to != 0x1032cb6 { return; }
  pc = 0x1032cb6; continue;
 }
 0x1032cb6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032cb7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032cb7); }
 let mut pc: u32 = 0x1032cb7;
 loop { match pc {
 0x1032cb7 => {
  { let v: u32 = cpu.r32(0x11c6271u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x11c6275u32)); cpu.eax = v; }
  if !cpu.zf { pc = 0x1032cc5; continue; }
  pc = 0x1032cc4; continue;
 }
 0x1032cc4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1032cc5 => {
  { let v: u32 = cpu.r32(0x11c627du32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x11c6285u32)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11c6279u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x1032df5; continue; }
  pc = 0x1032cf6; continue;
 }
 0x1032cf6 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11c6271u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c6275u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x1032d1a); f_0106b882(cpu); if cpu.returned_to != 0x1032d1a { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.push(0x1032d21); f_01057d72(cpu); if cpu.returned_to != 0x1032d21 { return; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x1032d94; continue; }
  pc = 0x1032d2d; continue;
 }
 0x1032d2d => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x1032d94; continue; }
  pc = 0x1032d33; continue;
 }
 0x1032d33 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0xcu16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1032d94; continue; }
  pc = 0x1032d3f; continue;
 }
 0x1032d3f => {
  { cpu.sub16((cpu.edx as u16), 0xcu16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1032d94; continue; }
  pc = 0x1032d45; continue;
 }
 0x1032d45 => {
  { let v: u32 = cpu.r32(0x11c6271u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c6275u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11ca47cu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x189u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0xcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.pushad(); }
  cpu.push(0x1032d78); f_0106b882(cpu); if cpu.returned_to != 0x1032d78 { return; }
  { cpu.popad(); }
  { cpu.sub32(cpu.r32(0x11c627du32), 0x3u32); }
  if cpu.zf { pc = 0x1032d94; continue; }
  pc = 0x1032d82; continue;
 }
 0x1032d82 => {
  { cpu.and8(cpu.r8(0x10ee968u32), 0x3u8); }
  if cpu.zf { pc = 0x1032d94; continue; }
  pc = 0x1032d8b; continue;
 }
 0x1032d8b => {
  { let v: u8 = 0x1u8; cpu.w8(0x11ca47du32, v); }
  pc = 0x1032df4; continue;
 }
 0x1032d94 => {
  { cpu.sub32(cpu.r32(0x11c627du32), 0x3u32); }
  if cpu.zf { pc = 0x1032dc2; continue; }
  pc = 0x1032d9d; continue;
 }
 0x1032d9d => {
  { cpu.sub32(cpu.r32(0x11c627du32), 0x0u32); }
  if cpu.zf { pc = 0x1032dc2; continue; }
  pc = 0x1032da6; continue;
 }
 0x1032da6 => {
  { let v: u32 = cpu.r32(0x11c6271u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c6275u32); cpu.ebx = v; }
  cpu.push(0x1032db6); f_01032e77(cpu); if cpu.returned_to != 0x1032db6 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1032dc2; continue; }
  pc = 0x1032dbb; continue;
 }
 0x1032dbb => {
  { cpu.sub32(cpu.eax, 0x8d00u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x1032d8b; continue; }
  pc = 0x1032dc2; continue;
 }
 0x1032dc2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x11c6281u32)); cpu.w32(0x11c6281u32, v); }
  if !cpu.zf { pc = 0x1032df4; continue; }
  pc = 0x1032dcc; continue;
 }
 0x1032dcc => {
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x11c627du32), 0x4u32); }
  if (cpu.sf != cpu.of) { pc = 0x1032de8; continue; }
  pc = 0x1032ddb; continue;
 }
 0x1032ddb => {
  cpu.push(0x1032de0); f_0105f215(cpu); if cpu.returned_to != 0x1032de0 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x1032de8; continue;
 }
 0x1032de8 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x11c6281u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11c6279u32)); cpu.w32(0x11c6279u32, v); }
  pc = 0x1032df4; continue;
 }
 0x1032df4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1032df5 => {
  { cpu.sub32(cpu.r32(0x11c627du32), 0x3u32); }
  if cpu.zf { pc = 0x1032e5e; continue; }
  pc = 0x1032dfe; continue;
 }
 0x1032dfe => {
  { cpu.sub32(cpu.r32(0x11c627du32), 0x0u32); }
  if cpu.zf { pc = 0x1032e17; continue; }
  pc = 0x1032e07; continue;
 }
 0x1032e07 => {
  { cpu.sub32(cpu.r32(0x11c627du32), 0x1u32); }
  if !cpu.zf { pc = 0x1032e1e; continue; }
  pc = 0x1032e10; continue;
 }
 0x1032e10 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x1032e2d; continue;
 }
 0x1032e17 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x1032e2d; continue;
 }
 0x1032e1e => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  cpu.push(0x1032e2d); f_0105f2ec(cpu); if cpu.returned_to != 0x1032e2d { return; }
  pc = 0x1032e2d; continue;
 }
 0x1032e2d => {
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c627du32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11c6279u32, v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x11c6285u32)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11c6281u32, v); }
  cpu.poll(); pc = 0x1032cc5; continue;
 }
 0x1032e5e => {
  { let v: u32 = 0x0u32; cpu.w32(0x11c6271u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11c6275u32, v); }
  cpu.poll(); pc = 0x1032df4; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032e77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032e77); }
 let mut pc: u32 = 0x1032e77;
 loop { match pc {
 0x1032e77 => {
  { let v: u32 = cpu.eax; cpu.w32(0x110d6f7u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x110d6fbu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11cc27eu32, v); }
  { let v: u32 = 0x1152e54u32; cpu.esi = v; }
  { let v: u32 = 0x11ca47eu32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1032e98; continue;
 }
 0x1032e98 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1032f01; continue; }
  pc = 0x1032e9f; continue;
 }
 0x1032e9f => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x1032f01; continue; }
  pc = 0x1032ea5; continue;
 }
 0x1032ea5 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x1032ec5; continue; }
  pc = 0x1032eab; continue;
 }
 0x1032eab => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1032ec5; continue; }
  pc = 0x1032eb1; continue;
 }
 0x1032eb1 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.zf { pc = 0x1032ec5; continue; }
  pc = 0x1032eb7; continue;
 }
 0x1032eb7 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0xfu8); }
  if cpu.zf { pc = 0x1032ec5; continue; }
  pc = 0x1032ebd; continue;
 }
 0x1032ebd => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x1032ec5; continue; }
  pc = 0x1032ec3; continue;
 }
 0x1032ec3 => {
  pc = 0x1032f01; continue;
 }
 0x1032ec5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6f7u32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x110d6fbu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x11cc27eu32)); cpu.w8(0x11cc27eu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1032f01; continue;
 }
 0x1032f01 => {
  { let v: u32 = cpu.esi.wrapping_add(0xb8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1032e98; continue; }
  pc = 0x1032f0a; continue;
 }
 0x1032f0a => {
  { cpu.sub8(cpu.r8(0x11cc27eu32), 0x1u8); }
  if cpu.zf { pc = 0x1032f56; continue; }
  pc = 0x1032f13; continue;
 }
 0x1032f13 => {
  { cpu.sub8(cpu.r8(0x11cc27eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1032f23; continue; }
  pc = 0x1032f1c; continue;
 }
 0x1032f1c => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x1032f5e; continue;
 }
 0x1032f23 => {
  { let v: u8 = cpu.r8(0x11cc27eu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1032f29; continue;
 }
 0x1032f29 => {
  { let v: u32 = 0x11ca47eu32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x11cc27eu32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1032f36; continue;
 }
 0x1032f36 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x1032f4b; continue; }
  pc = 0x1032f3e; continue;
 }
 0x1032f3e => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1032f4b; continue;
 }
 0x1032f4b => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x1032f36; continue; }
  pc = 0x1032f52; continue;
 }
 0x1032f52 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1032f29; continue; }
  pc = 0x1032f56; continue;
 }
 0x1032f56 => {
  { let v: u32 = 0x11ca47eu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  pc = 0x1032f5e; continue;
 }
 0x1032f5e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032f5f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032f5f); }
 let mut pc: u32 = 0x1032f5f;
 loop { match pc {
 0x1032f5f => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1032f69; continue; }
  pc = 0x1032f68; continue;
 }
 0x1032f68 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1032f69 => {
  { cpu.sub32(cpu.r32(0x11cc283u32), 0x0u32); }
  if cpu.zf { pc = 0x1032f79; continue; }
  pc = 0x1032f72; continue;
 }
 0x1032f72 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11cc283u32)); cpu.w32(0x11cc283u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1032f79 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1105e56u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xfu8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1032fbb; continue; }
  pc = 0x1032f84; continue;
 }
 0x1032f84 => {
  { let v: u8 = cpu.r8(0x1105e55u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x50u8); }
  if (cpu.sf != cpu.of) { pc = 0x1032fbb; continue; }
  pc = 0x1032f8d; continue;
 }
 0x1032f8d => {
  { cpu.sub32(cpu.r32(0x11b5424u32), 0x0u32); }
  if !cpu.zf { pc = 0x1032fbb; continue; }
  pc = 0x1032f96; continue;
 }
 0x1032f96 => {
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x9c4u32; cpu.ecx = v; }
  cpu.push(0x1032fa5); f_0105f2ec(cpu); if cpu.returned_to != 0x1032fa5 { return; }
  { cpu.sub16((cpu.eax as u16), 0x2au16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1032fbb; continue; }
  pc = 0x1032fab; continue;
 }
 0x1032fab => {
  cpu.push(0x1032fb0); f_01032fbc(cpu); if cpu.returned_to != 0x1032fb0 { return; }
  { let v: u32 = 0x2710u32; cpu.w32(0x11cc283u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1032fbb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01032fbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1032fbc); }
 let mut pc: u32 = 0x1032fbc;
 loop { match pc {
 0x1032fbc => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1032fc6; continue; }
  pc = 0x1032fc5; continue;
 }
 0x1032fc5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1032fc6 => {
  { cpu.sub32(cpu.r32(0x11c6271u32), 0x0u32); }
  if !cpu.zf { pc = 0x10330fa; continue; }
  pc = 0x1032fd3; continue;
 }
 0x1032fd3 => {
  { cpu.sub32(cpu.r32(0x11c6275u32), 0x0u32); }
  if !cpu.zf { pc = 0x10330fa; continue; }
  pc = 0x1032fe0; continue;
 }
 0x1032fe0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1032ff4); f_0108c1c5(cpu); if cpu.returned_to != 0x1032ff4 { return; }
  if cpu.cf { pc = 0x10330fa; continue; }
  pc = 0x1032ffa; continue;
 }
 0x1032ffa => {
  cpu.push(0x1032fff); f_0105f215(cpu); if cpu.returned_to != 0x1032fff { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf == cpu.of) { pc = 0x1033022; continue; }
  pc = 0x1033007; continue;
 }
 0x1033007 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1033064; continue;
 }
 0x1033022 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x103302e); f_0105f2ec(cpu); if cpu.returned_to != 0x103302e { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x103303c); f_0105f2ec(cpu); if cpu.returned_to != 0x103303c { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1033057); f_0104f36d(cpu); if cpu.returned_to != 0x1033057 { return; }
  cpu.push(0x103305c); f_010330fb(cpu); if cpu.returned_to != 0x103305c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x10330fa; continue; }
  pc = 0x1033064; continue;
 }
 0x1033064 => {
  { let v: u32 = cpu.eax; cpu.w32(0x11c6271u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11c6275u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11c6279u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x11c627du32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x11c6281u32, v); }
  { let v: u32 = 0x11c62cdu32; cpu.w32(0x11c6285u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11c6289u32, v); }
  { let v: u32 = cpu.r32(0x11c6289u32).wrapping_add(0x10u32); cpu.w32(0x11c6289u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11c628du32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11c6291u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x11c6295u32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x11c6299u32, v); }
  { let v: u32 = 0x11c62a1u32; cpu.w32(0x11c629du32, v); }
  { cpu.sub8(cpu.r8(0x1124ef0u32), 0x2u8); }
  if !cpu.zf { pc = 0x10330fa; continue; }
  pc = 0x10330da; continue;
 }
 0x10330da => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10330e0); f_0105f215(cpu); if cpu.returned_to != 0x10330e0 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if !cpu.zf { pc = 0x10330f9; continue; }
  pc = 0x10330e6; continue;
 }
 0x10330e6 => {
  cpu.push(0x10330eb); f_0105f215(cpu); if cpu.returned_to != 0x10330eb { return; }
  { let v: u8 = (cpu.eax as u8) & 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x10330f9; continue; }
  pc = 0x10330f1; continue;
 }
 0x10330f1 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x10330f7); f_010759b2(cpu); if cpu.returned_to != 0x10330f7 { return; }
  pc = 0x10330fa; continue;
 }
 0x10330f9 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10330fa; continue;
 }
 0x10330fa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010330fb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10330fb); } r_010330fb(cpu, 0x10330fb); }
pub fn f_01033114(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1033114); } r_010330fb(cpu, 0x1033114); }
fn r_010330fb(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10330fb => {
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x103310b; continue; }
  pc = 0x1033107; continue;
 }
 0x1033107 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if !cpu.zf { pc = 0x103313d; continue; }
  pc = 0x103310b; continue;
 }
 0x103310b => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x103311b; continue; }
  pc = 0x1033117; continue;
 }
 0x1033114 => {
  { let v: u8 = cpu.add8(cpu.r8(0x3cu32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))), ((cpu.edx >> 8) as u8)); cpu.w8(0x3cu32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  if !cpu.zf { pc = 0x103313d; continue; }
  pc = 0x103311b; continue;
 }
 0x1033117 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if !cpu.zf { pc = 0x103313d; continue; }
  pc = 0x103311b; continue;
 }
 0x103311b => {
  { let v: u8 = cpu.r8(0x4080u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x103312b; continue; }
  pc = 0x1033127; continue;
 }
 0x1033127 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if !cpu.zf { pc = 0x103313d; continue; }
  pc = 0x103312b; continue;
 }
 0x103312b => {
  { let v: u8 = cpu.r8(0x4081u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x103313b; continue; }
  pc = 0x1033137; continue;
 }
 0x1033137 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if !cpu.zf { pc = 0x103313d; continue; }
  pc = 0x103313b; continue;
 }
 0x103313b => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103313d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103313f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103313f); }
 let mut pc: u32 = 0x103313f;
 loop { match pc {
 0x103313f => {
  { let v: u32 = 0x0u32; cpu.w32(0x11c6279u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11c627du32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x11c6281u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11c6291u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11c6295u32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x11c6299u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103317c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103317c); }
 let mut pc: u32 = 0x103317c;
 loop { match pc {
 0x103317c => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1033186; continue; }
  pc = 0x1033185; continue;
 }
 0x1033185 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033186 => {
  { cpu.sub8(cpu.r8(0x110587bu32), 0x0u8); }
  if cpu.zf { pc = 0x1033190; continue; }
  pc = 0x103318f; continue;
 }
 0x103318f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033190 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11b5194u32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10331b3; continue; }
  pc = 0x10331a4; continue;
 }
 0x10331a4 => {
  { let v: u32 = cpu.r32(0x10e412cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x190u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11b5194u32, v); }
  pc = 0x10331b3; continue;
 }
 0x10331b3 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3d7fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x11b4c14u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10331d3; continue; }
  pc = 0x10331cc; continue;
 }
 0x10331cc => {
  { let v: u32 = cpu.r32(0x11b4c40u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  pc = 0x10331d3; continue;
 }
 0x10331d3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x10e3d7fu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11b4c83u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10331f3; continue; }
  pc = 0x10331ed; continue;
 }
 0x10331ed => {
  { let v: u8 = cpu.r8(0x11b4c88u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10331f3; continue;
 }
 0x10331f3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e4144u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11b5180u32, v); }
  pc = 0x10331ff; continue;
 }
 0x10331ff => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xbdu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xc7u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1033230; continue; }
  pc = 0x1033218; continue;
 }
 0x1033218 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x17au16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x18eu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x11b5194u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x11b5194u32)); cpu.edx = v; }
  pc = 0x1033230; continue;
 }
 0x1033230 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x10e4144u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  cpu.push(0x1033253); f_01018ffd(cpu); if cpu.returned_to != 0x1033253 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x1033265; continue; }
  pc = 0x1033258; continue;
 }
 0x1033258 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11b5180u32)); cpu.w8(0x11b5180u32, v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10331ff; continue; }
  pc = 0x1033264; continue;
 }
 0x1033264 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033265 => {
  { let v: u8 = cpu.r8(0x11b5180u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91du32), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x103327a; continue; }
  pc = 0x1033273; continue;
 }
 0x1033273 => {
  cpu.push(0x1033278); f_0103341b(cpu); if cpu.returned_to != 0x1033278 { return; }
  cpu.poll(); pc = 0x1033264; continue;
 }
 0x103327a => {
  cpu.push(0x103327f); f_0103cdf9(cpu); if cpu.returned_to != 0x103327f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033280); }
 let mut pc: u32 = 0x1033280;
 loop { match pc {
 0x1033280 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x103328a; continue; }
  pc = 0x1033289; continue;
 }
 0x1033289 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103328a => {
  { cpu.sub8(cpu.r8(0x110587bu32), 0x0u8); }
  if cpu.zf { pc = 0x1033294; continue; }
  pc = 0x1033293; continue;
 }
 0x1033293 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033294 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11b5194u32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10332b7; continue; }
  pc = 0x10332a8; continue;
 }
 0x10332a8 => {
  { let v: u32 = cpu.r32(0x10e412cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x190u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11b5194u32, v); }
  pc = 0x10332b7; continue;
 }
 0x10332b7 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.r32(0x10e4108u32); cpu.push(v); }
  { let v: u16 = 0xc6u16; cpu.w16(0x10e4108u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3d7fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x11b4c14u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10332f6; continue; }
  pc = 0x10332df; continue;
 }
 0x10332df => {
  { let v: u32 = cpu.r32(0x10e412cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x4u32); cpu.edi = v; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x10e4108u32, v); }
  { let v: u32 = cpu.r32(0x11b4c40u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  pc = 0x10332f6; continue;
 }
 0x10332f6 => {
  { let v: u32 = 0x1105e83u32; cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x10e3d7fu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11b4c79u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x103331b; continue; }
  pc = 0x1033315; continue;
 }
 0x1033315 => {
  { let v: u8 = cpu.r8(0x11b4c7eu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103331b; continue;
 }
 0x103331b => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e4144u32, v); }
  pc = 0x1033320; continue;
 }
 0x1033320 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xbu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xbdu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x103334a; continue; }
  pc = 0x1033335; continue;
 }
 0x1033335 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b1cu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0xbu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x11b5194u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1u32); cpu.ebx = v; }
  pc = 0x103334a; continue;
 }
 0x103334a => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x13b4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e4144u32); cpu.edx = v; }
  cpu.push(0x103335d); f_0104ead3(cpu); if cpu.returned_to != 0x103335d { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b1cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1033320; continue; }
  pc = 0x103336e; continue;
 }
 0x103336e => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4108u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033375(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033375); }
 let mut pc: u32 = 0x1033375;
 loop { match pc {
 0x1033375 => {
  { cpu.pushad(); }
  cpu.push(0x103337b); f_010334c4(cpu); if cpu.returned_to != 0x103337b { return; }
  if !cpu.cf { pc = 0x103338a; continue; }
  pc = 0x103337d; continue;
 }
 0x103337d => {
  { cpu.pushad(); }
  cpu.push(0x1033383); f_010333e6(cpu); if cpu.returned_to != 0x1033383 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.edi = v; }
  pc = 0x103338a; continue;
 }
 0x103338a => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(0xcu32); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xaau32; cpu.w32(0x11b5177u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x11b4734u32); cpu.eax = v; }
  { let v: u32 = 0x49u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11b517bu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x10333cc; continue;
 }
 0x10333cc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10333e4; continue; }
  pc = 0x10333d7; continue;
 }
 0x10333d7 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x3bu32); }
  if !cpu.zf { cpu.poll(); pc = 0x10333cc; continue; }
  pc = 0x10333dd; continue;
 }
 0x10333dd => {
  { let v: u8 = 0x0u8; cpu.w8(0xcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  pc = 0x10333e4; continue;
 }
 0x10333e4 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010333e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10333e6); }
 let mut pc: u32 = 0x10333e6;
 loop { match pc {
 0x10333e6 => {
  { cpu.pushad(); }
  { let v: u32 = 0x11b4734u32; cpu.edi = v; }
  { let v: u32 = 0x11b477du32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x10333f6; continue;
 }
 0x10333f6 => {
  { let v: u32 = 0x49u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)).wrapping_sub(0x10u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x49u32); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.movs(1, true); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10333f6; continue; }
  pc = 0x103340e; continue;
 }
 0x103340e => {
  { let v: u32 = cpu.edi; cpu.w32(0x10e40c0u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103341b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103341b); }
 let mut pc: u32 = 0x103341b;
 loop { match pc {
 0x103341b => {
  { cpu.sub32(cpu.r32(0x11b5177u32), 0x0u32); }
  if cpu.zf { pc = 0x1033466; continue; }
  pc = 0x1033424; continue;
 }
 0x1033424 => {
  cpu.push(0x1033429); f_010334e7(cpu); if cpu.returned_to != 0x1033429 { return; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x1033470; continue; }
  pc = 0x103342e; continue;
 }
 0x103342e => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x1033466; continue; }
  pc = 0x1033433; continue;
 }
 0x1033433 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x11b517bu32)); cpu.w32(0x11b517bu32, v); }
  { let v: u32 = cpu.r32(0x11b517bu32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if !cpu.sf { pc = 0x1033451; continue; }
  pc = 0x1033446; continue;
 }
 0x1033446 => {
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11b517bu32, v); }
  pc = 0x1033451; continue;
 }
 0x1033451 => {
  { let v: u32 = 0x49u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11b4734u32); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0xffffu16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x1033433; continue; }
  pc = 0x1033466; continue;
 }
 0x1033466 => {
  { let v: u32 = 0xaau32; cpu.w32(0x11b5177u32, v); }
  pc = 0x1033470; continue;
 }
 0x1033470 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033471(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033471); }
 let mut pc: u32 = 0x1033471;
 loop { match pc {
 0x1033471 => {
  { cpu.sub32(cpu.r32(0x11b5177u32), 0x0u32); }
  if cpu.zf { pc = 0x10334b9; continue; }
  pc = 0x103347a; continue;
 }
 0x103347a => {
  cpu.push(0x103347f); f_010334e7(cpu); if cpu.returned_to != 0x103347f { return; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x10334c3; continue; }
  pc = 0x1033484; continue;
 }
 0x1033484 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x10334b9; continue; }
  pc = 0x1033489; continue;
 }
 0x1033489 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11b517bu32)); cpu.w32(0x11b517bu32, v); }
  { let v: u32 = cpu.r32(0x11b517bu32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x6u32); }
  if !cpu.zf { pc = 0x10334a4; continue; }
  pc = 0x103349c; continue;
 }
 0x103349c => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11b517bu32, v); }
  pc = 0x10334a4; continue;
 }
 0x10334a4 => {
  { let v: u32 = 0x49u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11b4734u32); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0xffffu16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x1033489; continue; }
  pc = 0x10334b9; continue;
 }
 0x10334b9 => {
  { let v: u32 = 0xaau32; cpu.w32(0x11b5177u32, v); }
  pc = 0x10334c3; continue;
 }
 0x10334c3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010334c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10334c4); }
 let mut pc: u32 = 0x10334c4;
 loop { match pc {
 0x10334c4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x11b4734u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  pc = 0x10334d1; continue;
 }
 0x10334d1 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x10334e4; continue; }
  pc = 0x10334d8; continue;
 }
 0x10334d8 => {
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x49u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10334d1; continue; }
  pc = 0x10334e1; continue;
 }
 0x10334e1 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10334e4 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010334e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10334e7); }
 let mut pc: u32 = 0x10334e7;
 loop { match pc {
 0x10334e7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x11b4734u32; cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  pc = 0x10334f3; continue;
 }
 0x10334f3 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x10334fb; continue; }
  pc = 0x10334fa; continue;
 }
 0x10334fa => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10334fb; continue;
 }
 0x10334fb => {
  { let v: u32 = cpu.add32(cpu.edi, 0x49u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10334f3; continue; }
  pc = 0x1033501; continue;
 }
 0x1033501 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033502(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033502); }
 let mut pc: u32 = 0x1033502;
 loop { match pc {
 0x1033502 => {
  { let v: u32 = 0x11b4734u32; cpu.edi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  pc = 0x103350c; continue;
 }
 0x103350c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x49u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103350c; continue; }
  pc = 0x1033517; continue;
 }
 0x1033517 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033518(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033518); }
 let mut pc: u32 = 0x1033518;
 loop { match pc {
 0x1033518 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1033531; continue; }
  pc = 0x1033521; continue;
 }
 0x1033521 => {
  { cpu.sub8(cpu.r8(0x11c5bb6u32), 0xffu8); }
  if cpu.zf { pc = 0x1033531; continue; }
  pc = 0x103352a; continue;
 }
 0x103352a => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x1308u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1033537; continue;
 }
 0x1033531 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x12d8u32); cpu.ecx = v; }
  pc = 0x1033537; continue;
 }
 0x1033537 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103353f); cpu.call(t); if cpu.returned_to != 0x103353f { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x103357d; continue; }
  pc = 0x103354b; continue;
 }
 0x103354b => {
  { let v: u32 = cpu.eax.wrapping_add(0x16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x5u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c5bb6u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x103357d; continue; }
  pc = 0x103355e; continue;
 }
 0x103355e => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.push(0x1033566); cpu.call(t); if cpu.returned_to != 0x1033566 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x1105e83u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x135fu32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.push(0x103357d); f_0104bd14(cpu); if cpu.returned_to != 0x103357d { return; }
  pc = 0x103357d; continue;
 }
 0x103357d => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103357f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103357f); }
 let mut pc: u32 = 0x103357f;
 loop { match pc {
 0x103357f => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1033589; continue; }
  pc = 0x1033588; continue;
 }
 0x1033588 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033589 => {
  { cpu.sub32(cpu.r32(0x11b5177u32), 0x0u32); }
  if !cpu.zf { pc = 0x1033593; continue; }
  pc = 0x1033592; continue;
 }
 0x1033592 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033593 => {
  cpu.push(0x1033598); f_010334e7(cpu); if cpu.returned_to != 0x1033598 { return; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x103359e; continue; }
  pc = 0x103359d; continue;
 }
 0x103359d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103359e => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x11b48eau32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11b48eeu32, v); }
  { let v: u32 = 0x40u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10335d8; continue; }
  pc = 0x10335ce; continue;
 }
 0x10335ce => {
  { let v: u32 = 0xc0u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  pc = 0x10335d8; continue;
 }
 0x10335d8 => {
  { let v: u32 = 0xa4u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xeu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11b48eau32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11b48eeu32); cpu.edi = v; }
  cpu.push(0x10335f2); f_01018ffd(cpu); if cpu.returned_to != 0x10335f2 { return; }
  if !cpu.cf { pc = 0x103361d; continue; }
  pc = 0x10335f4; continue;
 }
 0x10335f4 => {
  { let v: u32 = 0x0u32; cpu.w32(0x11b5177u32, v); }
  { let v: u32 = cpu.r32(0x11b517bu32); cpu.ebx = v; }
  { let v: u32 = 0x49u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x11b4734u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x103361a); f_0103cdf9(cpu); if cpu.returned_to != 0x103361a { return; }
  { cpu.cf = true; }
  pc = 0x103366c; continue;
 }
 0x103361d => {
  { let v: u32 = 0xe4u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0xf4u32; cpu.ecx = v; }
  { let v: u32 = 0x4eu32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x103364e; continue; }
  pc = 0x103363a; continue;
 }
 0x103363a => {
  { let v: u32 = 0x164u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x174u32; cpu.ecx = v; }
  { let v: u32 = 0x8eu32; cpu.edx = v; }
  pc = 0x103364e; continue;
 }
 0x103364e => {
  { let v: u32 = cpu.r32(0x11b48eau32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11b48eeu32); cpu.edi = v; }
  cpu.push(0x103365f); f_01018ffd(cpu); if cpu.returned_to != 0x103365f { return; }
  if !cpu.cf { pc = 0x103366c; continue; }
  pc = 0x1033661; continue;
 }
 0x1033661 => {
  { let v: u32 = 0x0u32; cpu.w32(0x11b5177u32, v); }
  { cpu.cf = true; }
  pc = 0x103366c; continue;
 }
 0x103366c => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103366e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103366e); }
 let mut pc: u32 = 0x103366e;
 loop { match pc {
 0x103366e => {
  { let v = cpu.r32(0x10e4108u32); cpu.push(v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x10e4108u32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1033691; continue; }
  pc = 0x1033686; continue;
 }
 0x1033686 => {
  { let v: u32 = cpu.r32(0x10e412cu32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e4108u32, v); }
  pc = 0x1033691; continue;
 }
 0x1033691 => {
  { let v: u32 = 0x11b4734u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x16du32); cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x38u32; cpu.eax = v; }
  { let v: u32 = 0xbau32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10336be; continue; }
  pc = 0x10336b1; continue;
 }
 0x10336b1 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x11b5194u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x5u32); cpu.ebx = v; }
  pc = 0x10336be; continue;
 }
 0x10336be => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x10336f5; continue; }
  pc = 0x10336d0; continue;
 }
 0x10336d0 => {
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10336e0; continue; }
  pc = 0x10336d9; continue;
 }
 0x10336d9 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b1cu32, v); }
  pc = 0x10336e0; continue;
 }
 0x10336e0 => {
  { cpu.sub8(cpu.r8(0x110587bu32), 0x0u8); }
  if cpu.zf { pc = 0x10336ee; continue; }
  pc = 0x10336e9; continue;
 }
 0x10336e9 => {
  cpu.push(0x10336ee); f_01033755(cpu); if cpu.returned_to != 0x10336ee { return; }
  pc = 0x10336ee; continue;
 }
 0x10336ee => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b1cu32, v); }
  pc = 0x10336f5; continue;
 }
 0x10336f5 => {
  { cpu.sub32(cpu.r32(0x11b5177u32), 0x0u32); }
  if cpu.zf { pc = 0x103374e; continue; }
  pc = 0x10336fe; continue;
 }
 0x10336fe => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11b5177u32)); cpu.w32(0x11b5177u32, v); }
  { let v: u32 = cpu.r32(0x11b517bu32); cpu.ebx = v; }
  { let v: u32 = 0x49u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11b4734u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1033735; continue; }
  pc = 0x103372b; continue;
 }
 0x103372b => {
  { let v: u32 = 0xc0u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  pc = 0x1033735; continue;
 }
 0x1033735 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x103374e; continue; }
  pc = 0x1033749; continue;
 }
 0x1033749 => {
  cpu.push(0x103374e); f_01033755(cpu); if cpu.returned_to != 0x103374e { return; }
  pc = 0x103374e; continue;
 }
 0x103374e => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4108u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033755(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033755); }
 let mut pc: u32 = 0x1033755;
 loop { match pc {
 0x1033755 => {
  { let v = cpu.r32(0x10e4108u32); cpu.push(v); }
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if cpu.zf { pc = 0x103378f; continue; }
  pc = 0x1033760; continue;
 }
 0x1033760 => {
  { cpu.pushad(); }
  { let v: u32 = 0x17a6u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103376c); cpu.call(t); if cpu.returned_to != 0x103376c { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x68u32); cpu.eax = v; }
  { let v: u32 = 0x17a7u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103377c); cpu.call(t); if cpu.returned_to != 0x103377c { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0xa4u32); cpu.eax = v; }
  { let v: u32 = 0x17a8u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103378e); cpu.call(t); if cpu.returned_to != 0x103378e { return; }
  { cpu.popad(); }
  pc = 0x103378f; continue;
 }
 0x103378f => {
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x10337de; continue; }
  pc = 0x1033794; continue;
 }
 0x1033794 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.push(0x103379c); cpu.call(t); if cpu.returned_to != 0x103379c { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x1105e83u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x3u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e4108u32, v); }
  { let v: u16 = cpu.r16(0x10e4108u32).wrapping_add(0xcu16); cpu.w16(0x10e4108u32, v); }
  { cpu.sub8(cpu.r8(0x11c5b1cu32), 0x0u8); }
  if cpu.zf { pc = 0x10337ce; continue; }
  pc = 0x10337c3; continue;
 }
 0x10337c3 => {
  { let v: u16 = cpu.r16(0x10e4108u32).wrapping_add(0x4u16); cpu.w16(0x10e4108u32, v); }
  { let v: u32 = cpu.ebx.wrapping_sub(0x3u32); cpu.ebx = v; }
  pc = 0x10337ce; continue;
 }
 0x10337ce => {
  { let v: u32 = 0x135fu32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.push(0x10337dd); f_0104bd14(cpu); if cpu.returned_to != 0x10337dd { return; }
  { cpu.popad(); }
  pc = 0x10337de; continue;
 }
 0x10337de => {
  { cpu.sub8(cpu.r8(0x11c5b1cu32), 0x0u8); }
  if !cpu.zf { pc = 0x103381a; continue; }
  pc = 0x10337e7; continue;
 }
 0x10337e7 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x23u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x3u32); cpu.ebx = v; }
  { let v: u32 = 0x128cu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10337f9); cpu.call(t); if cpu.returned_to != 0x10337f9 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x27u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x3u32); cpu.ebx = v; }
  { let v: u32 = 0x1272u32; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b1cu32, v); }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.push(0x1033817); f_0104bd14(cpu); if cpu.returned_to != 0x1033817 { return; }
  { cpu.popad(); }
  pc = 0x103383f; continue;
 }
 0x103381a => {
  { let v: u32 = cpu.eax.wrapping_add(0x46u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x1b25u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x103382a); cpu.call(t); if cpu.returned_to != 0x103382a { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = 0x1272u32; cpu.ecx = v; }
  { let v: u32 = 0x100u32; cpu.edx = v; }
  cpu.push(0x103383e); f_0104bd14(cpu); if cpu.returned_to != 0x103383e { return; }
  { cpu.popad(); }
  pc = 0x103383f; continue;
 }
 0x103383f => {
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if !cpu.zf { pc = 0x1033892; continue; }
  pc = 0x1033844; continue;
 }
 0x1033844 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e3f78u32); cpu.edx = v; }
  { let v: u32 = cpu.and32(cpu.edx, 0x3u32); cpu.edx = v; }
  if !cpu.zf { pc = 0x1033891; continue; }
  pc = 0x1033850; continue;
 }
 0x1033850 => {
  { let v: u32 = cpu.eax.wrapping_add(0x27u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x103385e; continue; }
  pc = 0x1033858; continue;
 }
 0x1033858 => {
  { let v: u16 = cpu.r16(0x1128de1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x103385e; continue;
 }
 0x103385e => {
  { let v: u32 = cpu.ebx.wrapping_add(0x3u32); cpu.ebx = v; }
  { let v: u32 = 0x18c8u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11c5b1cu32), 0x0u8); }
  if cpu.zf { pc = 0x103388b; continue; }
  pc = 0x103386f; continue;
 }
 0x103386f => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x1033877; continue; }
  pc = 0x1033874; continue;
 }
 0x1033874 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x20u32); cpu.eax = v; }
  pc = 0x1033877; continue;
 }
 0x1033877 => {
  { let v: u32 = 0x18c8u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1033885); cpu.call(t); if cpu.returned_to != 0x1033885 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x8u32); cpu.ebx = v; }
  pc = 0x103388b; continue;
 }
 0x103388b => {
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1033891); cpu.call(t); if cpu.returned_to != 0x1033891 { return; }
  pc = 0x1033891; continue;
 }
 0x1033891 => {
  { cpu.popad(); }
  pc = 0x1033892; continue;
 }
 0x1033892 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4108u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103389f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103389f); }
 let mut pc: u32 = 0x103389f;
 loop { match pc {
 0x103389f => {
  { let v: u16 = cpu.r16(0x10e3ce1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef315u32, v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x10ef315u32); cpu.mul8(v); }
  { let v: u8 = cpu.r8(0x1152eb2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x2cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef314u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010338d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10338d0); }
 let mut pc: u32 = 0x10338d0;
 loop { match pc {
 0x10338d0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10ee948u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10ee948u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x10338e8); cpu.call(t); if cpu.returned_to != 0x10338e8 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033a19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033a19); }
 let mut pc: u32 = 0x1033a19;
 loop { match pc {
 0x1033a19 => {
  cpu.push(0x1033a1e); f_01072435(cpu); if cpu.returned_to != 0x1033a1e { return; }
  { let v: u32 = cpu.r32(0x11310d4u32); cpu.edx = v; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  cpu.push(0x1033a2d); f_0105eaf1(cpu); if cpu.returned_to != 0x1033a2d { return; }
  { cpu.interrupts_off = true; }
  { let v: u32 = cpu.r32(0x10e4034u32); cpu.esp = v; }
  { cpu.interrupts_off = false; }
  cpu.push(0x1033a3a); f_01054675(cpu); if cpu.returned_to != 0x1033a3a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033a3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033a3b); }
 let mut pc: u32 = 0x1033a3b;
 loop { match pc {
 0x1033a3b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x10e3d8fu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x42u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033a4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033a4b); }
 let mut pc: u32 = 0x1033a4b;
 loop { match pc {
 0x1033a4b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3d93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d8fu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033a5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033a5a); }
 let mut pc: u32 = 0x1033a5a;
 loop { match pc {
 0x1033a5a => {
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = cpu.r16(0x10e3db1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d8fu32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033a6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033a6b); }
 let mut pc: u32 = 0x1033a6b;
 loop { match pc {
 0x1033a6b => {
  { let v: u32 = 0x10f7b7eu32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x1u8); }
  if cpu.zf { pc = 0x1033ab1; continue; }
  pc = 0x1033a79; continue;
 }
 0x1033a79 => {
  { let v: u32 = 0x10f7b8au32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x2u8); }
  if cpu.zf { pc = 0x1033ab1; continue; }
  pc = 0x1033a87; continue;
 }
 0x1033a87 => {
  { let v: u32 = 0x10f7b97u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x3u8); }
  if cpu.zf { pc = 0x1033ab1; continue; }
  pc = 0x1033a95; continue;
 }
 0x1033a95 => {
  { let v: u32 = 0x10f7ba4u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x4u8); }
  if cpu.zf { pc = 0x1033ab1; continue; }
  pc = 0x1033aa3; continue;
 }
 0x1033aa3 => {
  { let v: u32 = 0x10f7bb1u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x5u8); }
  if cpu.zf { pc = 0x1033ab1; continue; }
  pc = 0x1033ab1; continue;
 }
 0x1033ab1 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0x8u32; cpu.edx = v; }
  { cpu.pushad(); }
  cpu.push(0x1033abe); f_010018d7(cpu); if cpu.returned_to != 0x1033abe { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d8fu32, v); }
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x11b542du32, v); }
  { let v: u16 = cpu.r16(0x10e3d8fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.sf { f_0105f11c(cpu); return; }
  pc = 0x1033adb; continue;
 }
 0x1033adb => {
  { let v: u8 = 0x0u8; cpu.w8(0x11b542du32, v); }
  { let v: u16 = cpu.r16(0x10e3d8fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3db1u32, v); }
  cpu.push(0x1033afc); f_01033a4b(cpu); if cpu.returned_to != 0x1033afc { return; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x1033b05); f_01033a3b(cpu); if cpu.returned_to != 0x1033b05 { return; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0x10e3db9u32; cpu.edx = v; }
  cpu.push(0x1033b14); f_0105e746(cpu); if cpu.returned_to != 0x1033b14 { return; }
  { let v: u16 = cpu.r16(0x10e3db9u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x118cd66u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033b21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033b21); }
 let mut pc: u32 = 0x1033b21;
 loop { match pc {
 0x1033b21 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r16(0x10e3d93u32) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1033b33; continue; }
  pc = 0x1033b31; continue;
 }
 0x1033b31 => {
  { cpu.interrupt(0x21); }
  pc = 0x1033b33; continue;
 }
 0x1033b33 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r16(0x10e3db1u32) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1033b45; continue; }
  pc = 0x1033b43; continue;
 }
 0x1033b43 => {
  { cpu.interrupt(0x21); }
  pc = 0x1033b45; continue;
 }
 0x1033b45 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033b46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033b46); }
 let mut pc: u32 = 0x1033b46;
 loop { match pc {
 0x1033b46 => {
  { cpu.sub8(cpu.r8(0x10fd360u32), 0x1u8); }
  if cpu.zf { pc = 0x1033b6f; continue; }
  pc = 0x1033b4f; continue;
 }
 0x1033b4f => {
  cpu.push(0x1033b54); f_01071c54(cpu); if cpu.returned_to != 0x1033b54 { return; }
  { let v: u32 = 0x118cd68u32; cpu.edi = v; }
  { let v: u32 = 0x1d25u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x10e4014u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e400cu32, v); }
  pc = 0x1033b6f; continue;
 }
 0x1033b6f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033b70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033b70); }
 let mut pc: u32 = 0x1033b70;
 loop { match pc {
 0x1033b70 => {
  { cpu.pushad(); }
  { let v: u32 = 0x11cc9c6u32; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  cpu.push(0x1033b84); f_0105ce4d(cpu); if cpu.returned_to != 0x1033b84 { return; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.sf { pc = 0x1033be7; continue; }
  pc = 0x1033b89; continue;
 }
 0x1033b89 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = 0x10f7bbeu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x1u8); }
  if cpu.zf { pc = 0x1033bd1; continue; }
  pc = 0x1033b99; continue;
 }
 0x1033b99 => {
  { let v: u32 = 0x10f7bcau32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x2u8); }
  if cpu.zf { pc = 0x1033bd1; continue; }
  pc = 0x1033ba7; continue;
 }
 0x1033ba7 => {
  { let v: u32 = 0x10f7bd7u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x3u8); }
  if cpu.zf { pc = 0x1033bd1; continue; }
  pc = 0x1033bb5; continue;
 }
 0x1033bb5 => {
  { let v: u32 = 0x10f7be4u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x4u8); }
  if cpu.zf { pc = 0x1033bd1; continue; }
  pc = 0x1033bc3; continue;
 }
 0x1033bc3 => {
  { let v: u32 = 0x10f7bf1u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x5u8); }
  if cpu.zf { pc = 0x1033bd1; continue; }
  pc = 0x1033bd1; continue;
 }
 0x1033bd1 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  cpu.push(0x1033bdf); f_0105ce4d(cpu); if cpu.returned_to != 0x1033bdf { return; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.sf { pc = 0x1033be7; continue; }
  pc = 0x1033be4; continue;
 }
 0x1033be4 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033be7 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033bea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033bea); }
 let mut pc: u32 = 0x1033bea;
 loop { match pc {
 0x1033bea => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10e4054u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x112304bu32, v); }
  { cpu.sub8(cpu.r8(0x10ef31eu32), 0x0u8); }
  if cpu.zf { pc = 0x1033c1f; continue; }
  pc = 0x1033c01; continue;
 }
 0x1033c01 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e4054u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112304bu32); cpu.edx = v; }
  cpu.push(0x1033c12); f_010019bc(cpu); if cpu.returned_to != 0x1033c12 { return; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x1033c1f; continue; }
  pc = 0x1033c19; continue;
 }
 0x1033c19 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x1033cd5; continue;
 }
 0x1033c1f => {
  { cpu.pushad(); }
  { let v: u32 = 0x11cc9c6u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  cpu.push(0x1033c33); f_0105ce4d(cpu); if cpu.returned_to != 0x1033c33 { return; }
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  { let v: u8 = 0x3u8; cpu.w8(0x11b542du32, v); }
  if cpu.zf { pc = 0x1033c54; continue; }
  pc = 0x1033c3f; continue;
 }
 0x1033c3f => {
  { let v: u8 = 0x0u8; cpu.w8(0x11b542du32, v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.sf { pc = 0x1033c5c; continue; }
  pc = 0x1033c4a; continue;
 }
 0x1033c4a => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e3ff0u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033c54 => {
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  f_0105f11c(cpu); return;
 }
 0x1033c5c => {
  { cpu.popad(); }
  { let v: u32 = 0x10f7bbeu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x1u8); }
  if cpu.zf { pc = 0x1033ca3; continue; }
  pc = 0x1033c6b; continue;
 }
 0x1033c6b => {
  { let v: u32 = 0x10f7bcau32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x2u8); }
  if cpu.zf { pc = 0x1033ca3; continue; }
  pc = 0x1033c79; continue;
 }
 0x1033c79 => {
  { let v: u32 = 0x10f7bd7u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x3u8); }
  if cpu.zf { pc = 0x1033ca3; continue; }
  pc = 0x1033c87; continue;
 }
 0x1033c87 => {
  { let v: u32 = 0x10f7be4u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x4u8); }
  if cpu.zf { pc = 0x1033ca3; continue; }
  pc = 0x1033c95; continue;
 }
 0x1033c95 => {
  { let v: u32 = 0x10f7bf1u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x5u8); }
  if cpu.zf { pc = 0x1033ca3; continue; }
  pc = 0x1033ca3; continue;
 }
 0x1033ca3 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  cpu.push(0x1033cb1); f_0105ce4d(cpu); if cpu.returned_to != 0x1033cb1 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  { let v: u8 = 0x3u8; cpu.w8(0x11b542du32, v); }
  if cpu.zf { f_0105f11c(cpu); return; }
  pc = 0x1033cc2; continue;
 }
 0x1033cc2 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11b542du32, v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.sf { pc = 0x1033cd5; continue; }
  pc = 0x1033ccd; continue;
 }
 0x1033ccd => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e3ff0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033cd5 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e4054u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112304bu32); cpu.edx = v; }
  cpu.push(0x1033ce7); f_010019bc(cpu); if cpu.returned_to != 0x1033ce7 { return; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x1033d1d; continue; }
  pc = 0x1033cee; continue;
 }
 0x1033cee => {
  { let v: u32 = cpu.r32(0x10e4054u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x112304bu32); cpu.eax = v; }
  cpu.push(0x1033cfe); f_0105e70b(cpu); if cpu.returned_to != 0x1033cfe { return; }
  if !cpu.cf { pc = 0x1033d1c; continue; }
  pc = 0x1033d00; continue;
 }
 0x1033d00 => {
  { cpu.sub8(cpu.r8(0x10ef366u32), 0x0u8); }
  { let v: u8 = 0x5u8; cpu.w8(0x11b542du32, v); }
  if cpu.zf { f_0105f11c(cpu); return; }
  pc = 0x1033d14; continue;
 }
 0x1033d14 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11b542du32, v); }
  { cpu.cf = true; }
  pc = 0x1033d1c; continue;
 }
 0x1033d1c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033d1d => {
  { cpu.sub8(cpu.r8(0x10ef366u32), 0x0u8); }
  { let v: u8 = 0x6u8; cpu.w8(0x11b542du32, v); }
  if cpu.zf { f_0105f11c(cpu); return; }
  pc = 0x1033d31; continue;
 }
 0x1033d31 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11b542du32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033d3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033d3a); }
 let mut pc: u32 = 0x1033d3a;
 loop { match pc {
 0x1033d3a => {
  { let v = cpu.r16(0x10e3ce1u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3debu32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1033dfe; continue; }
  pc = 0x1033d55; continue;
 }
 0x1033d55 => {
  { cpu.sub32(cpu.r32(0x10e4174u32), 0x0u32); }
  if cpu.zf { pc = 0x1033d80; continue; }
  pc = 0x1033d5e; continue;
 }
 0x1033d5e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4174u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ce1u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3debu32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1033d80; continue;
 }
 0x1033d80 => {
  { cpu.pushad(); }
  cpu.push(0x1033d86); f_0103389f(cpu); if cpu.returned_to != 0x1033d86 { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ef314u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if (cpu.sf != cpu.of) { pc = 0x1033d93; continue; }
  pc = 0x1033d91; continue;
 }
 0x1033d91 => {
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1033d93; continue;
 }
 0x1033d93 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1119dc5u32, v); }
  { cpu.popad(); }
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x1033dfe; continue; }
  pc = 0x1033da3; continue;
 }
 0x1033da3 => {
  { cpu.pushad(); }
  cpu.push(0x1033da9); f_010576b2(cpu); if cpu.returned_to != 0x1033da9 { return; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4174u32, v); }
  { let v: u32 = cpu.r8(0x10f7dc7u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1119dc9u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x1033dcb; continue; }
  pc = 0x1033dc1; continue;
 }
 0x1033dc1 => {
  { let v: u32 = 0x2u32; cpu.w32(0x1119dc9u32, v); }
  pc = 0x1033dcb; continue;
 }
 0x1033dcb => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1033df3; continue; }
  pc = 0x1033dd1; continue;
 }
 0x1033dd1 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e416cu32, v); }
  { cpu.popad(); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3debu32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3ce1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033df3 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e416cu32, v); }
  { cpu.popad(); }
  pc = 0x1033dfe; continue;
 }
 0x1033dfe => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3debu32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3ce1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033e0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033e0d); }
 let mut pc: u32 = 0x1033e0d;
 loop { match pc {
 0x1033e0d => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1033e6a; continue; }
  pc = 0x1033e16; continue;
 }
 0x1033e16 => {
  { let v: u32 = cpu.r32(0x10e416cu32); cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x1033e6a; continue; }
  pc = 0x1033e20; continue;
 }
 0x1033e20 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = 0xeu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1033e6a; continue; }
  pc = 0x1033e35; continue;
 }
 0x1033e35 => {
  cpu.push(0x1033e3a); f_0101d6df(cpu); if cpu.returned_to != 0x1033e3a { return; }
  cpu.push(0x1033e3f); f_010341b5(cpu); if cpu.returned_to != 0x1033e3f { return; }
  if cpu.cf { pc = 0x1033e69; continue; }
  pc = 0x1033e41; continue;
 }
 0x1033e41 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1033e69; continue; }
  pc = 0x1033e47; continue;
 }
 0x1033e47 => {
  { cpu.sub16(cpu.r16(0x10fcc02u32), 0x0u16); }
  if !cpu.zf { pc = 0x1033e69; continue; }
  pc = 0x1033e51; continue;
 }
 0x1033e51 => {
  cpu.push(0x1033e56); f_0105f178(cpu); if cpu.returned_to != 0x1033e56 { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x1fu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1033e69; continue; }
  pc = 0x1033e5a; continue;
 }
 0x1033e5a => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x63u32; cpu.ebx = v; }
  cpu.push(0x1033e69); f_01033e70(cpu); if cpu.returned_to != 0x1033e69 { return; }
  pc = 0x1033e69; continue;
 }
 0x1033e69 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033e6a => {
  cpu.push(0x1033e6f); f_01034269(cpu); if cpu.returned_to != 0x1033e6f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033e70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033e70); }
 let mut pc: u32 = 0x1033e70;
 loop { match pc {
 0x1033e70 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1033eb5; continue; }
  pc = 0x1033e7a; continue;
 }
 0x1033e7a => {
  { cpu.sub32(cpu.r32(0x1119d8cu32), 0x0u32); }
  if cpu.zf { pc = 0x1033eb7; continue; }
  pc = 0x1033e83; continue;
 }
 0x1033e83 => {
  { cpu.sub32(cpu.r32(0x1119d94u32), 0x0u32); }
  if !cpu.zf { pc = 0x1033ebf; continue; }
  pc = 0x1033e8c; continue;
 }
 0x1033e8c => {
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x1033eb5; continue; }
  pc = 0x1033e91; continue;
 }
 0x1033e91 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e4164u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4168u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1033ea8); f_01033d3a(cpu); if cpu.returned_to != 0x1033ea8 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e41bcu32, v); }
  cpu.push(0x1033eb5); f_0101cebe(cpu); if cpu.returned_to != 0x1033eb5 { return; }
  pc = 0x1033eb5; continue;
 }
 0x1033eb5 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033eb7 => {
  { cpu.pushad(); }
  cpu.push(0x1033ebd); f_0101cebe(cpu); if cpu.returned_to != 0x1033ebd { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033ebf => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x1033efa; continue; }
  pc = 0x1033ec4; continue;
 }
 0x1033ec4 => {
  { cpu.sub8(cpu.r8(0x1123048u32), 0x0u8); }
  if !cpu.zf { pc = 0x1033efa; continue; }
  pc = 0x1033ecd; continue;
 }
 0x1033ecd => {
  { cpu.sub32(cpu.r32(0x1119d94u32), 0x4u32); }
  if !cpu.zf { pc = 0x1033efa; continue; }
  pc = 0x1033ed6; continue;
 }
 0x1033ed6 => {
  { let v: u32 = cpu.r32(0x10e41bcu32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xff00u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x400u32); }
  if cpu.zf { pc = 0x1033f86; continue; }
  pc = 0x1033eee; continue;
 }
 0x1033eee => {
  { cpu.sub32(cpu.edx, 0x500u32); }
  if cpu.zf { pc = 0x1033f86; continue; }
  pc = 0x1033efa; continue;
 }
 0x1033efa => {
  { cpu.sub32(cpu.r32(0x10e4164u32), 0x0u32); }
  if cpu.zf { pc = 0x1033f0c; continue; }
  pc = 0x1033f03; continue;
 }
 0x1033f03 => {
  { cpu.sub32(cpu.r32(0x10e4164u32), 0x2u32); }
  if cpu.zf { pc = 0x1033f0c; continue; }
  pc = 0x1033f0c; continue;
 }
 0x1033f0c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1123048u32, v); }
  cpu.push(0x1033f1a); f_01033d3a(cpu); if cpu.returned_to != 0x1033f1a { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e41bcu32, v); }
  cpu.push(0x1033f29); f_0101cebe(cpu); if cpu.returned_to != 0x1033f29 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { cpu.poll(); pc = 0x1033eb5; continue; }
  pc = 0x1033f34; continue;
 }
 0x1033f34 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xff00u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x100u32); }
  if cpu.zf { pc = 0x1033f88; continue; }
  pc = 0x1033f44; continue;
 }
 0x1033f44 => {
  { cpu.sub32(cpu.edx, 0x400u32); }
  if cpu.zf { pc = 0x1033f88; continue; }
  pc = 0x1033f4c; continue;
 }
 0x1033f4c => {
  { cpu.sub32(cpu.edx, 0x500u32); }
  if cpu.zf { pc = 0x1033f88; continue; }
  pc = 0x1033f54; continue;
 }
 0x1033f54 => {
  { cpu.sub32(cpu.edx, 0xa00u32); }
  if cpu.zf { pc = 0x1033f88; continue; }
  pc = 0x1033f5c; continue;
 }
 0x1033f5c => {
  { cpu.sub32(cpu.edx, 0xb00u32); }
  if cpu.zf { pc = 0x1033f88; continue; }
  pc = 0x1033f64; continue;
 }
 0x1033f64 => {
  pc = 0x1033f88; continue;
 }
 0x1033f86 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033f88 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e4164u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4168u32, v); }
  cpu.poll(); pc = 0x1033f86; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01033f9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1033f9a); }
 let mut pc: u32 = 0x1033f9a;
 loop { match pc {
 0x1033f9a => {
  { cpu.sub32(cpu.r32(0x10e4164u32), 0x0u32); }
  if cpu.zf { pc = 0x1033fe0; continue; }
  pc = 0x1033fa3; continue;
 }
 0x1033fa3 => {
  { cpu.sub32(cpu.r32(0x1119d94u32), 0x3u32); }
  if !cpu.zf { pc = 0x1033fdf; continue; }
  pc = 0x1033fac; continue;
 }
 0x1033fac => {
  { cpu.sub32(cpu.r32(0x10e4164u32), 0x2u32); }
  if !cpu.zf { pc = 0x1033fbf; continue; }
  pc = 0x1033fb5; continue;
 }
 0x1033fb5 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e4174u32, v); }
  pc = 0x1033fbf; continue;
 }
 0x1033fbf => {
  { let v: u32 = cpu.r32(0x10e4164u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e4168u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e41bcu32, v); }
  cpu.push(0x1033fd5); f_0101cebe(cpu); if cpu.returned_to != 0x1033fd5 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4164u32, v); }
  pc = 0x1033fdf; continue;
 }
 0x1033fdf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1033fe0 => {
  { cpu.sub32(cpu.r32(0x1119d94u32), 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x1033fdf; continue; }
  pc = 0x1033fe9; continue;
 }
 0x1033fe9 => {
  { cpu.sub32(cpu.r32(0x1119d94u32), 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x1033fdf; continue; }
  pc = 0x1033ff2; continue;
 }
 0x1033ff2 => {
  cpu.push(0x1033ff7); f_010341b5(cpu); if cpu.returned_to != 0x1033ff7 { return; }
  if cpu.cf { pc = 0x10340d9; continue; }
  pc = 0x1033ffd; continue;
 }
 0x1033ffd => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x10340d9; continue; }
  pc = 0x1034007; continue;
 }
 0x1034007 => {
  { cpu.sub16(cpu.r16(0x10fcc02u32), 0x0u16); }
  if !cpu.zf { cpu.poll(); pc = 0x1033fdf; continue; }
  pc = 0x1034011; continue;
 }
 0x1034011 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x1034033; continue; }
  pc = 0x1034017; continue;
 }
 0x1034017 => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x1034048; continue; }
  pc = 0x1034021; continue;
 }
 0x1034021 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1034027); f_010576b2(cpu); if cpu.returned_to != 0x1034027 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if cpu.zf { pc = 0x1034033; continue; }
  pc = 0x103402c; continue;
 }
 0x103402c => {
  {  }
  if !cpu.zf { pc = 0x10340d9; continue; }
  pc = 0x1034033; continue;
 }
 0x1034033 => {
  cpu.push(0x1034038); f_010341b5(cpu); if cpu.returned_to != 0x1034038 { return; }
  if cpu.cf { pc = 0x10340d9; continue; }
  pc = 0x103403e; continue;
 }
 0x103403e => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x10340d9; continue; }
  pc = 0x1034048; continue;
 }
 0x1034048 => {
  { cpu.sub32(cpu.r32(0x1119d94u32), 0x0u32); }
  if cpu.zf { pc = 0x103408a; continue; }
  pc = 0x1034051; continue;
 }
 0x1034051 => {
  { cpu.sub32(cpu.r32(0x1119d94u32), 0x2u32); }
  if cpu.zf { pc = 0x103408a; continue; }
  pc = 0x103405a; continue;
 }
 0x103405a => {
  { cpu.sub32(cpu.r32(0x1119d8cu32), 0x0u32); }
  if cpu.zf { pc = 0x1034092; continue; }
  pc = 0x1034063; continue;
 }
 0x1034063 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1034092; continue; }
  pc = 0x1034069; continue;
 }
 0x1034069 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1034092; continue; }
  pc = 0x103406f; continue;
 }
 0x103406f => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x103407b; continue; }
  pc = 0x1034075; continue;
 }
 0x1034075 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x103408a; continue; }
  pc = 0x103407b; continue;
 }
 0x103407b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x103408a; continue; }
  pc = 0x1034084; continue;
 }
 0x1034084 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1034092; continue; }
  pc = 0x103408a; continue;
 }
 0x103408a => {
  { let v: u8 = 0x33u8; cpu.w8(0x1123049u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034092 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x1123049u32)); cpu.w8(0x1123049u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x1033fdf; continue; }
  pc = 0x103409e; continue;
 }
 0x103409e => {
  { let v: u8 = 0x0u8; cpu.w8(0x1123049u32, v); }
  cpu.push(0x10340aa); f_0105f178(cpu); if cpu.returned_to != 0x10340aa { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xfu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1033fdf; continue; }
  pc = 0x10340b2; continue;
 }
 0x10340b2 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fu32; cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x1123008u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x1033fdf; continue; }
  pc = 0x10340cc; continue;
 }
 0x10340cc => {
  cpu.push(0x10340d1); f_01033e70(cpu); if cpu.returned_to != 0x10340d1 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x1123048u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10340d9 => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x103411c; continue; }
  pc = 0x10340e3; continue;
 }
 0x10340e3 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e416cu32, v); }
  cpu.push(0x10340f2); f_01034269(cpu); if cpu.returned_to != 0x10340f2 { return; }
  cpu.push(0x10340f7); f_0105f178(cpu); if cpu.returned_to != 0x10340f7 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4174u32, v); }
  { let v: u32 = cpu.r8(0x1123008u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  cpu.push(0x103411b); f_01033e70(cpu); if cpu.returned_to != 0x103411b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103411c => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e416cu32, v); }
  cpu.push(0x103412b); f_01034269(cpu); if cpu.returned_to != 0x103412b { return; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4164u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4168u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e41bcu32, v); }
  cpu.push(0x1034148); f_0101cebe(cpu); if cpu.returned_to != 0x1034148 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4174u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034153(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034153); }
 let mut pc: u32 = 0x1034153;
 loop { match pc {
 0x1034153 => {
  { cpu.sub32(cpu.r32(0x110eab4u32), 0x0u32); }
  if cpu.zf { pc = 0x10341b4; continue; }
  pc = 0x103415c; continue;
 }
 0x103415c => {
  cpu.push(0x1034161); f_0101cc10(cpu); if cpu.returned_to != 0x1034161 { return; }
  { cpu.sub32(cpu.r32(0x1119d94u32), 0x0u32); }
  if cpu.zf { pc = 0x10341af; continue; }
  pc = 0x103416a; continue;
 }
 0x103416a => {
  { cpu.sub32(cpu.r32(0x1119d94u32), 0x2u32); }
  if cpu.zf { pc = 0x10341af; continue; }
  pc = 0x1034173; continue;
 }
 0x1034173 => {
  { cpu.sub32(cpu.r32(0x1119d94u32), 0x4u32); }
  if cpu.zf { pc = 0x10341af; continue; }
  pc = 0x103417c; continue;
 }
 0x103417c => {
  { let v: u8 = cpu.dec8(cpu.r8(0x112304au32)); cpu.w8(0x112304au32, v); }
  if !cpu.sf { pc = 0x10341af; continue; }
  pc = 0x1034184; continue;
 }
 0x1034184 => {
  { let v: u8 = 0x0u8; cpu.w8(0x112304au32, v); }
  cpu.push(0x1034190); f_0105f178(cpu); if cpu.returned_to != 0x1034190 { return; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xfu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x10341af; continue; }
  pc = 0x1034194; continue;
 }
 0x1034194 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fu32; cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x1123008u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x10341af; continue; }
  pc = 0x10341aa; continue;
 }
 0x10341aa => {
  cpu.push(0x10341af); f_0101cebe(cpu); if cpu.returned_to != 0x10341af { return; }
  pc = 0x10341af; continue;
 }
 0x10341af => {
  cpu.push(0x10341b4); f_0101cbf7(cpu); if cpu.returned_to != 0x10341b4 { return; }
  pc = 0x10341b4; continue;
 }
 0x10341b4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010341b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10341b5); }
 let mut pc: u32 = 0x10341b5;
 loop { match pc {
 0x10341b5 => {
  { let v: u32 = cpu.r32(0x10e4174u32); cpu.ebp = v; }
  { let v: u32 = cpu.or32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if cpu.zf { pc = 0x10341f5; continue; }
  pc = 0x10341bf; continue;
 }
 0x10341bf => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x10341f5; continue; }
  pc = 0x10341c5; continue;
 }
 0x10341c5 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x10341f3; continue; }
  pc = 0x10341cb; continue;
 }
 0x10341cb => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x10341f5; continue; }
  pc = 0x10341df; continue;
 }
 0x10341df => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x10341f5; continue; }
  pc = 0x10341e5; continue;
 }
 0x10341e5 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  pc = 0x10341f3; continue;
 }
 0x10341f3 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10341f5 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e4174u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034201(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034201); }
 let mut pc: u32 = 0x1034201;
 loop { match pc {
 0x1034201 => {
  { let v: u32 = cpu.r32(0x10e40a8u32); cpu.edi = v; }
  { let v: u32 = 0x3300u32; cpu.ecx = v; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034211(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034211); }
 let mut pc: u32 = 0x1034211;
 loop { match pc {
 0x1034211 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  let t = cpu.r32(0x10e436cu32); cpu.push(0x1034225); cpu.call(t); if cpu.returned_to != 0x1034225 { return; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10c0208u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.edi = v; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  pc = 0x1034249; continue;
 }
 0x1034249 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1034262; continue; }
  pc = 0x1034250; continue;
 }
 0x1034250 => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x1034262; continue; }
  pc = 0x1034254; continue;
 }
 0x1034254 => {
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x1034264; continue; }
  pc = 0x1034258; continue;
 }
 0x1034258 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x1034262; continue; }
  pc = 0x103425c; continue;
 }
 0x103425c => {
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.zf { cpu.poll(); pc = 0x1034249; continue; }
  pc = 0x1034260; continue;
 }
 0x1034260 => {
  cpu.poll(); pc = 0x1034249; continue;
 }
 0x1034262 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034264 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  cpu.poll(); pc = 0x1034249; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034269(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034269); }
 let mut pc: u32 = 0x1034269;
 loop { match pc {
 0x1034269 => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1119dafu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1119daeu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1119db0u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1119db1u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1119db2u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034285(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034285); }
 let mut pc: u32 = 0x1034285;
 loop { match pc {
 0x1034285 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034288(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034288); }
 let mut pc: u32 = 0x1034288;
 loop { match pc {
 0x1034288 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11230c4u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x10342a5; continue; }
  pc = 0x1034297; continue;
 }
 0x1034297 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x7fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x12u8); }
  if !cpu.zf { pc = 0x10342d1; continue; }
  pc = 0x10342a5; continue;
 }
 0x10342a5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { cpu.poll(); pc = 0x10342a5; continue; }
  pc = 0x10342b3; continue;
 }
 0x10342b3 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x80u32); cpu.edi = v; }
  { let v: u8 = 0x12u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10342bb; continue;
 }
 0x10342bb => {
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if !cpu.zf { pc = 0x10342d1; continue; }
  pc = 0x10342bf; continue;
 }
 0x10342bf => {
  { let v: u32 = cpu.edi.wrapping_sub(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x11230c4u32)); cpu.w8(0x11230c4u32, v); }
  cpu.poll(); pc = 0x10342bb; continue;
 }
 0x10342d1 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4000u32); cpu.eax = v; }
  { let v: u32 = 0x1388u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e40a0u32); cpu.edi = v; }
  pc = 0x10342e5; continue;
 }
 0x10342e5 => {
  { cpu.scas(4, 0); }
  if cpu.zf { pc = 0x10342ee; continue; }
  pc = 0x10342e8; continue;
 }
 0x10342e8 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10342e5; continue; }
  pc = 0x10342ed; continue;
 }
 0x10342ed => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10342ee => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x4u32); cpu.edi = v; }
  { let v: u8 = cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.edi)), (cpu.edx as u8)); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  if !cpu.sf { pc = 0x103436e; continue; }
  pc = 0x10342f8; continue;
 }
 0x10342f8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x10e6910u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xff80u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x8au8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = 0x116aa58u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  cpu.push(0x103433c); f_01034cfe(cpu); if cpu.returned_to != 0x103433c { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x103434b); f_010346fa(cpu); if cpu.returned_to != 0x103434b { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.push(0x1034352); f_01034371(cpu); if cpu.returned_to != 0x1034352 { return; }
  if cpu.cf { pc = 0x103435b; continue; }
  pc = 0x1034354; continue;
 }
 0x1034354 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x103435a); f_010346fa(cpu); if cpu.returned_to != 0x103435a { return; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x103435b; continue;
 }
 0x103435b => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef34au32, v); }
  cpu.push(0x1034367); f_010347b2(cpu); if cpu.returned_to != 0x1034367 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x103436d); f_01034505(cpu); if cpu.returned_to != 0x103436d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103436e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034371(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034371); }
 let mut pc: u32 = 0x1034371;
 loop { match pc {
 0x1034371 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10f5764u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x103441f; continue; }
  pc = 0x1034388; continue;
 }
 0x1034388 => {
  { let v: u32 = 0x4001u32.wrapping_add(cpu.edi); cpu.eax = v; }
  cpu.push(0x1034393); f_01008d26(cpu); if cpu.returned_to != 0x1034393 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11dbf64u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x11dbf66u32, v); }
  { let v: u32 = 0x116aa58u32; cpu.esi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x10343b2; continue;
 }
 0x10343b2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.r32(0x11dbf64u32); cpu.push(v); }
  { cpu.sub32(cpu.esi, cpu.r32(0x11dbf68u32)); }
  if cpu.zf { pc = 0x1034422; continue; }
  pc = 0x10343c2; continue;
 }
 0x10343c2 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x11dbf64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x1034422; continue; }
  pc = 0x10343d2; continue;
 }
 0x10343d2 => {
  { cpu.sub16((cpu.eax as u16), 0x200u16); }
  if (cpu.sf == cpu.of) { pc = 0x1034422; continue; }
  pc = 0x10343d8; continue;
 }
 0x10343d8 => {
  { let v: u16 = cpu.r16(0x11dbf66u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x1034422; continue; }
  pc = 0x10343e5; continue;
 }
 0x10343e5 => {
  { cpu.sub16((cpu.ebx as u16), 0x200u16); }
  if (cpu.sf == cpu.of) { pc = 0x1034422; continue; }
  pc = 0x10343ec; continue;
 }
 0x10343ec => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  cpu.push(0x1034409); f_01034cbc(cpu); if cpu.returned_to != 0x1034409 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x1034417; continue; }
  pc = 0x103440d; continue;
 }
 0x103440d => {
  { let v: u16 = cpu.r16(0x11dbf66u32).wrapping_sub(0x10u16); cpu.w16(0x11dbf66u32, v); }
  pc = 0x1034422; continue;
 }
 0x1034417 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x11dbf64u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x103441f; continue;
 }
 0x103441f => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034422 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x11dbf64u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8au32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10343b2; continue; }
  pc = 0x1034437; continue;
 }
 0x1034437 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103443a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103443a); }
 let mut pc: u32 = 0x103443a;
 loop { match pc {
 0x103443a => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi.wrapping_sub(0x10ea910u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffff80u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x5cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11230b0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11230b4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x10344aa; continue; }
  pc = 0x1034493; continue;
 }
 0x1034493 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x10344af; continue; }
  pc = 0x103449c; continue;
 }
 0x103449c => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x10344b4; continue; }
  pc = 0x10344a5; continue;
 }
 0x10344a5 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x10344b7; continue;
 }
 0x10344aa => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  pc = 0x10344b7; continue;
 }
 0x10344af => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x10344b7; continue;
 }
 0x10344b4 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x10344b7; continue;
 }
 0x10344b7 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10344ba; continue;
 }
 0x10344ba => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10344c1); f_0105f215(cpu); if cpu.returned_to != 0x10344c1 { return; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x10344cf; continue; }
  pc = 0x10344c7; continue;
 }
 0x10344c7 => {
  { let v: u16 = cpu.r16(0x112308au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10344d5; continue;
 }
 0x10344cf => {
  { let v: u16 = cpu.r16(0x112308cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10344d5; continue;
 }
 0x10344d5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ebfu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x11230b0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11230b4u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x10344ff); f_0105aa11(cpu); if cpu.returned_to != 0x10344ff { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10344ba; continue; }
  pc = 0x1034503; continue;
 }
 0x1034503 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034505(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034505); }
 let mut pc: u32 = 0x1034505;
 loop { match pc {
 0x1034505 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.edi.wrapping_sub(0x10ea910u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffff80u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11230b0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11230b4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  cpu.push(0x1034541); f_0105f215(cpu); if cpu.returned_to != 0x1034541 { return; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x103454f; continue; }
  pc = 0x1034547; continue;
 }
 0x1034547 => {
  { let v: u16 = cpu.r16(0x1123092u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1034555; continue;
 }
 0x103454f => {
  { let v: u16 = cpu.r16(0x1123094u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1034555; continue;
 }
 0x1034555 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0xbu8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x105adb4u32; cpu.w32(0x11230b8u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x103458f; continue; }
  pc = 0x1034578; continue;
 }
 0x1034578 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x1034594; continue; }
  pc = 0x1034581; continue;
 }
 0x1034581 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x1034599; continue; }
  pc = 0x103458a; continue;
 }
 0x103458a => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  pc = 0x103459c; continue;
 }
 0x103458f => {
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  pc = 0x103459c; continue;
 }
 0x1034594 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  pc = 0x103459c; continue;
 }
 0x1034599 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  pc = 0x103459c; continue;
 }
 0x103459c => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11230b0u32).wrapping_sub(0x8u32); cpu.w32(0x11230b0u32, v); }
  { let v: u32 = cpu.r32(0x11230b4u32).wrapping_sub(0x8u32); cpu.w32(0x11230b4u32, v); }
  pc = 0x10345ad; continue;
 }
 0x10345ad => {
  cpu.push(0x10345b2); f_0103469c(cpu); if cpu.returned_to != 0x10345b2 { return; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10345ad; continue; }
  pc = 0x10345b4; continue;
 }
 0x10345b4 => {
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x105aa11u32; cpu.w32(0x11230b8u32, v); }
  { let v: u32 = cpu.r32(0x11230b0u32).wrapping_add(0x4u32); cpu.w32(0x11230b0u32, v); }
  { let v: u32 = cpu.r32(0x11230b4u32).wrapping_add(0x4u32); cpu.w32(0x11230b4u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x10345fd; continue; }
  pc = 0x10345e6; continue;
 }
 0x10345e6 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x1034602; continue; }
  pc = 0x10345ef; continue;
 }
 0x10345ef => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x1034607; continue; }
  pc = 0x10345f8; continue;
 }
 0x10345f8 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x103460a; continue;
 }
 0x10345fd => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  pc = 0x103460a; continue;
 }
 0x1034602 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x103460a; continue;
 }
 0x1034607 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x103460a; continue;
 }
 0x103460a => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x103460d; continue;
 }
 0x103460d => {
  cpu.push(0x1034612); f_0105f215(cpu); if cpu.returned_to != 0x1034612 { return; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x1034620; continue; }
  pc = 0x1034618; continue;
 }
 0x1034618 => {
  { let v: u16 = cpu.r16(0x112308eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1034626; continue;
 }
 0x1034620 => {
  { let v: u16 = cpu.r16(0x1123090u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1034626; continue;
 }
 0x1034626 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1034631); f_0103469c(cpu); if cpu.returned_to != 0x1034631 { return; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103460d; continue; }
  pc = 0x1034633; continue;
 }
 0x1034633 => {
  { let v: u32 = cpu.r32(0x11230b0u32).wrapping_add(0x2u32); cpu.w32(0x11230b0u32, v); }
  { let v: u32 = cpu.r32(0x11230b4u32).wrapping_add(0x2u32); cpu.w32(0x11230b4u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x1034664; continue; }
  pc = 0x103464d; continue;
 }
 0x103464d => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x1034669; continue; }
  pc = 0x1034656; continue;
 }
 0x1034656 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x103466e; continue; }
  pc = 0x103465f; continue;
 }
 0x103465f => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x1034671; continue;
 }
 0x1034664 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  pc = 0x1034671; continue;
 }
 0x1034669 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x1034671; continue;
 }
 0x103466e => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x1034671; continue;
 }
 0x1034671 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1034674; continue;
 }
 0x1034674 => {
  cpu.push(0x1034679); f_0105f215(cpu); if cpu.returned_to != 0x1034679 { return; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x1034687; continue; }
  pc = 0x103467f; continue;
 }
 0x103467f => {
  { let v: u16 = cpu.r16(0x112308au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x103468d; continue;
 }
 0x1034687 => {
  { let v: u16 = cpu.r16(0x112308cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x103468d; continue;
 }
 0x103468d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ebfu32, v); }
  cpu.push(0x1034698); f_0103469c(cpu); if cpu.returned_to != 0x1034698 { return; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1034674; continue; }
  pc = 0x103469a; continue;
 }
 0x103469a => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103469c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103469c); }
 let mut pc: u32 = 0x103469c;
 loop { match pc {
 0x103469c => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r8(0x11230c4u32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x10346b8); f_0105f215(cpu); if cpu.returned_to != 0x10346b8 { return; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x10346c9); f_0105f215(cpu); if cpu.returned_to != 0x10346c9 { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11230b0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x11230b4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  let t = cpu.r32(0x11230b8u32); cpu.push(0x10346f8); cpu.call(t); if cpu.returned_to != 0x10346f8 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010346fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10346fa); }
 let mut pc: u32 = 0x10346fa;
 loop { match pc {
 0x10346fa => {
  cpu.push(0x10346ff); f_0105f215(cpu); if cpu.returned_to != 0x10346ff { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x11233fau32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x16u8; cpu.w8(0x4000u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.edi)) | ((cpu.eax >> 8) as u8); cpu.w8(0x4000u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x80u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10346fa; continue; }
  pc = 0x1034725; continue;
 }
 0x1034725 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034726(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034726); }
 let mut pc: u32 = 0x1034726;
 loop { match pc {
 0x1034726 => {
  cpu.push(0x103472b); f_0105f215(cpu); if cpu.returned_to != 0x103472b { return; }
  { let v: u32 = cpu.eax & 0x307u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.r16(0x10f782au32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x16u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x4000u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x80u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { f_010346fa(cpu); return; }
  pc = 0x1034751; continue;
 }
 0x1034751 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034752(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034752); }
 let mut pc: u32 = 0x1034752;
 loop { match pc {
 0x1034752 => {
  cpu.push(0x1034757); f_01034781(cpu); if cpu.returned_to != 0x1034757 { return; }
  { let v: u32 = 0x116aa58u32; cpu.ebp = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef34au32, v); }
  pc = 0x1034768; continue;
 }
 0x1034768 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x103476f); f_010347b2(cpu); if cpu.returned_to != 0x103476f { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x8au32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1034768; continue; }
  pc = 0x1034779; continue;
 }
 0x1034779 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef34au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034781(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034781); }
 let mut pc: u32 = 0x1034781;
 loop { match pc {
 0x1034781 => {
  { let v: u32 = 0x116aa58u32; cpu.ebp = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x103478d; continue;
 }
 0x103478d => {
  { let v: u32 = cpu.ebx; cpu.w32(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  pc = 0x103479a; continue;
 }
 0x103479a => {
  cpu.push(0x103479f); f_01034cbc(cpu); if cpu.returned_to != 0x103479f { return; }
  if cpu.zf { pc = 0x10347a4; continue; }
  pc = 0x10347a1; continue;
 }
 0x10347a1 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  pc = 0x10347a4; continue;
 }
 0x10347a4 => {
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103479a; continue; }
  pc = 0x10347a8; continue;
 }
 0x10347a8 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x8au32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103478d; continue; }
  pc = 0x10347b1; continue;
 }
 0x10347b1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010347b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10347b2); }
 let mut pc: u32 = 0x10347b2;
 loop { match pc {
 0x10347b2 => {
  { let v: u8 = 0x5u8; cpu.w8(0x10fcbf9u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11dbf68u32, v); }
  cpu.push(0x10347c4); f_010348bb(cpu); if cpu.returned_to != 0x10347c4 { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 3u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10e6910u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e93u32).wrapping_add(0x200u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.r16(0x10e3e95u32).wrapping_add(0x200u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  pc = 0x1034814; continue;
 }
 0x1034814 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  cpu.push(0x103481d); f_01034cbc(cpu); if cpu.returned_to != 0x103481d { return; }
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = cpu.ecx; cpu.push(v); }
  if cpu.zf { pc = 0x1034858; continue; }
  pc = 0x1034822; continue;
 }
 0x1034822 => {
  cpu.push(0x1034827); f_0103487e(cpu); if cpu.returned_to != 0x1034827 { return; }
  cpu.push(0x103482c); f_01034b54(cpu); if cpu.returned_to != 0x103482c { return; }
  cpu.push(0x1034831); f_01034907(cpu); if cpu.returned_to != 0x1034831 { return; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x14u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(0x12u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x103483b; continue;
 }
 0x103483b => {
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if (cpu.sf != cpu.of) { pc = 0x1034857; continue; }
  pc = 0x1034846; continue;
 }
 0x1034846 => {
  if !cpu.zf { pc = 0x103484b; continue; }
  pc = 0x1034848; continue;
 }
 0x1034848 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x14u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x103484b; continue;
 }
 0x103484b => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x4000u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x103483b; continue;
 }
 0x1034857 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x1034858; continue;
 }
 0x1034858 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e93u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1034814; continue; }
  pc = 0x1034869; continue;
 }
 0x1034869 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x200u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x60u32); cpu.edi = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3e95u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1034814; continue; }
  pc = 0x103487d; continue;
 }
 0x103487d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103487e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103487e); }
 let mut pc: u32 = 0x103487e;
 loop { match pc {
 0x103487e => {
  { cpu.sub8(cpu.r8(0x10ef34au32), 0x0u8); }
  if cpu.zf { pc = 0x10348ba; continue; }
  pc = 0x1034887; continue;
 }
 0x1034887 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e40a4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e40a0u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10ef34bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10348aa); f_0105f215(cpu); if cpu.returned_to != 0x10348aa { return; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x10e40a4u32), 0x8u32); cpu.w32(0x10e40a4u32, v); }
  pc = 0x10348ba; continue;
 }
 0x10348ba => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010348bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10348bb); }
 let mut pc: u32 = 0x10348bb;
 loop { match pc {
 0x10348bb => {
  { cpu.sub8(cpu.r8(0x10ef34au32), 0x0u8); }
  if cpu.zf { pc = 0x10348fe; continue; }
  pc = 0x10348c4; continue;
 }
 0x10348c4 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x116aa58u32); cpu.eax = v; }
  { let v: u8 = 0x8au8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef34bu32, v); }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x90u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebp, 0x116b496u32); }
  if !cpu.zf { pc = 0x10348ea; continue; }
  pc = 0x10348e5; continue;
 }
 0x10348e5 => {
  { let v: u32 = 0x9c38u32; cpu.ecx = v; }
  pc = 0x10348ea; continue;
 }
 0x10348ea => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0x6u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  if cpu.zf { pc = 0x10348fe; continue; }
  pc = 0x10348ef; continue;
 }
 0x10348ef => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x10e40a0u32)); cpu.edi = v; }
  { cpu.stos(2, true); }
  pc = 0x10348fe; continue;
 }
 0x10348fe => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e40a4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034907(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034907); }
 let mut pc: u32 = 0x1034907;
 loop { match pc {
 0x1034907 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xcu16); }
  if cpu.zf { pc = 0x1034952; continue; }
  pc = 0x103491c; continue;
 }
 0x103491c => {
  { cpu.sub16((cpu.edx as u16), 0xeu16); }
  if cpu.zf { pc = 0x1034952; continue; }
  pc = 0x1034922; continue;
 }
 0x1034922 => {
  { cpu.sub16((cpu.edx as u16), 0x10u16); }
  if cpu.zf { pc = 0x1034952; continue; }
  pc = 0x1034928; continue;
 }
 0x1034928 => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x12u8); }
  if !cpu.zf { pc = 0x1034952; continue; }
  pc = 0x1034936; continue;
 }
 0x1034936 => {
  { let v: u16 = cpu.r16(0x10c79f9u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1034942); f_01034a1a(cpu); if cpu.returned_to != 0x1034942 { return; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x4001u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.edi)) | 0x12u8; cpu.w8(0x4001u32.wrapping_add(cpu.edi), v); }
  pc = 0x1034952; continue;
 }
 0x1034952 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x80u32); cpu.edi = v; }
  cpu.push(0x103495d); f_01035211(cpu); if cpu.returned_to != 0x103495d { return; }
  if cpu.cf { pc = 0x1034a0e; continue; }
  pc = 0x1034963; continue;
 }
 0x1034963 => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if (cpu.sf != cpu.of) { pc = 0x1034a15; continue; }
  pc = 0x103496c; continue;
 }
 0x103496c => {
  if !cpu.zf { pc = 0x10349a9; continue; }
  pc = 0x103496e; continue;
 }
 0x103496e => {
  { let v: u16 = cpu.r16(0x10c79fdu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x4001u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0xffffff81u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1034a15; continue; }
  pc = 0x1034989; continue;
 }
 0x1034989 => {
  { cpu.sub8(cpu.r8(0x3f81u32.wrapping_add(cpu.edi)), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x1034a15; continue; }
  pc = 0x1034995; continue;
 }
 0x1034995 => {
  { let v: u16 = cpu.r16(0x10c79fbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffff81u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), 0x0u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x3f81u32.wrapping_add(cpu.edi), v); }
  pc = 0x1034a15; continue;
 }
 0x10349a9 => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x12u8); }
  if !cpu.zf { pc = 0x10349e1; continue; }
  pc = 0x10349b7; continue;
 }
 0x10349b7 => {
  { let v: u8 = cpu.r8(0xffffff81u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x3f81u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10c7a2bu32)); }
  if cpu.zf { pc = 0x10349d5; continue; }
  pc = 0x10349cc; continue;
 }
 0x10349cc => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10c7a2du32)); }
  if !cpu.zf { pc = 0x10349e1; continue; }
  pc = 0x10349d5; continue;
 }
 0x10349d5 => {
  { let v: u16 = cpu.r16(0x10c7a2du32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x12u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1034a05; continue;
 }
 0x10349e1 => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.r16(0x10c79fbu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x12u8); }
  if !cpu.zf { pc = 0x1034a05; continue; }
  pc = 0x10349f6; continue;
 }
 0x10349f6 => {
  { let v: u16 = cpu.r16(0x10c79f9u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1034a02); f_01034a1a(cpu); if cpu.returned_to != 0x1034a02 { return; }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x12u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1034a05; continue;
 }
 0x1034a05 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x4001u32.wrapping_add(cpu.edi), v); }
  pc = 0x1034a0e; continue;
 }
 0x1034a0e => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x1034952; continue;
 }
 0x1034a15 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034a1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034a1a); }
 let mut pc: u32 = 0x1034a1a;
 loop { match pc {
 0x1034a1a => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a2bu32)); }
  if cpu.zf { pc = 0x1034b48; continue; }
  pc = 0x1034a33; continue;
 }
 0x1034a33 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a2du32)); }
  if cpu.zf { pc = 0x1034b50; continue; }
  pc = 0x1034a40; continue;
 }
 0x1034a40 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10c79dfu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x14u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if cpu.zf { pc = 0x1034b40; continue; }
  pc = 0x1034a56; continue;
 }
 0x1034a56 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10c79dfu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x28u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if cpu.zf { pc = 0x1034b48; continue; }
  pc = 0x1034a6c; continue;
 }
 0x1034a6c => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c79ffu32)); }
  if cpu.zf { pc = 0x1034b30; continue; }
  pc = 0x1034a79; continue;
 }
 0x1034a79 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a01u32)); }
  if cpu.zf { pc = 0x1034b30; continue; }
  pc = 0x1034a86; continue;
 }
 0x1034a86 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a03u32)); }
  if cpu.zf { pc = 0x1034b30; continue; }
  pc = 0x1034a93; continue;
 }
 0x1034a93 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a05u32)); }
  if cpu.zf { pc = 0x1034b30; continue; }
  pc = 0x1034aa0; continue;
 }
 0x1034aa0 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a17u32)); }
  if cpu.zf { pc = 0x1034b38; continue; }
  pc = 0x1034aad; continue;
 }
 0x1034aad => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a0bu32)); }
  if cpu.zf { pc = 0x1034b40; continue; }
  pc = 0x1034aba; continue;
 }
 0x1034aba => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a1du32)); }
  if cpu.zf { pc = 0x1034b48; continue; }
  pc = 0x1034ac7; continue;
 }
 0x1034ac7 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a07u32)); }
  if cpu.zf { pc = 0x1034b10; continue; }
  pc = 0x1034ad0; continue;
 }
 0x1034ad0 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a19u32)); }
  if cpu.zf { pc = 0x1034b18; continue; }
  pc = 0x1034ad9; continue;
 }
 0x1034ad9 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a0du32)); }
  if cpu.zf { pc = 0x1034b40; continue; }
  pc = 0x1034ae2; continue;
 }
 0x1034ae2 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a1fu32)); }
  if cpu.zf { pc = 0x1034b48; continue; }
  pc = 0x1034aeb; continue;
 }
 0x1034aeb => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a09u32)); }
  if cpu.zf { pc = 0x1034b20; continue; }
  pc = 0x1034af4; continue;
 }
 0x1034af4 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a1bu32)); }
  if cpu.zf { pc = 0x1034b28; continue; }
  pc = 0x1034afd; continue;
 }
 0x1034afd => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a0fu32)); }
  if cpu.zf { pc = 0x1034b40; continue; }
  pc = 0x1034b06; continue;
 }
 0x1034b06 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c7a21u32)); }
  if cpu.zf { pc = 0x1034b48; continue; }
  pc = 0x1034b0f; continue;
 }
 0x1034b0f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034b10 => {
  { let v: u16 = cpu.r16(0x10c7a23u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034b18 => {
  { let v: u16 = cpu.r16(0x10c7a27u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034b20 => {
  { let v: u16 = cpu.r16(0x10c7a25u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034b28 => {
  { let v: u16 = cpu.r16(0x10c7a29u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034b30 => {
  { let v: u16 = cpu.r16(0x10c79f7u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034b38 => {
  { let v: u16 = cpu.r16(0x10c79f9u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034b40 => {
  { let v: u16 = cpu.r16(0x10c7a2bu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034b48 => {
  { let v: u16 = cpu.r16(0x10c7a2du32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034b50 => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034b54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034b54); }
 let mut pc: u32 = 0x1034b54;
 loop { match pc {
 0x1034b54 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = 0xfu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x20u16); }
  if (cpu.sf != cpu.of) { pc = 0x1034b6e; continue; }
  pc = 0x1034b60; continue;
 }
 0x1034b60 => {
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1034b69); f_01034cbc(cpu); if cpu.returned_to != 0x1034b69 { return; }
  if cpu.zf { pc = 0x1034b6e; continue; }
  pc = 0x1034b6b; continue;
 }
 0x1034b6b => {
  { let v: u8 = (cpu.ebx as u8) & 0xeu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1034b6e; continue;
 }
 0x1034b6e => {
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x1fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x1fu16); }
  if cpu.zf { pc = 0x1034b8a; continue; }
  pc = 0x1034b7b; continue;
 }
 0x1034b7b => {
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1034b85); f_01034cbc(cpu); if cpu.returned_to != 0x1034b85 { return; }
  if cpu.zf { pc = 0x1034b8a; continue; }
  pc = 0x1034b87; continue;
 }
 0x1034b87 => {
  { let v: u8 = (cpu.ebx as u8) & 0xdu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1034b8a; continue;
 }
 0x1034b8a => {
  { cpu.sub16((cpu.ecx as u16), 0x3e0u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1034ba2; continue; }
  pc = 0x1034b91; continue;
 }
 0x1034b91 => {
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1034b9d); f_01034cbc(cpu); if cpu.returned_to != 0x1034b9d { return; }
  if cpu.zf { pc = 0x1034ba2; continue; }
  pc = 0x1034b9f; continue;
 }
 0x1034b9f => {
  { let v: u8 = (cpu.ebx as u8) & 0xbu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1034ba2; continue;
 }
 0x1034ba2 => {
  { cpu.and16((cpu.ecx as u16), 0x1fu16); }
  if cpu.zf { pc = 0x1034bb8; continue; }
  pc = 0x1034ba9; continue;
 }
 0x1034ba9 => {
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1034bb3); f_01034cbc(cpu); if cpu.returned_to != 0x1034bb3 { return; }
  if cpu.zf { pc = 0x1034bb8; continue; }
  pc = 0x1034bb5; continue;
 }
 0x1034bb5 => {
  { let v: u8 = (cpu.ebx as u8) & 0x7u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1034bb8; continue;
 }
 0x1034bb8 => {
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u8 = 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x10e40a0u32); cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1034bfe; continue; }
  pc = 0x1034bc9; continue;
 }
 0x1034bc9 => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1388u32; cpu.ecx = v; }
  pc = 0x1034bd2; continue;
 }
 0x1034bd2 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)))); }
  if cpu.zf { pc = 0x1034bdc; continue; }
  pc = 0x1034bd7; continue;
 }
 0x1034bd7 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1034bd2; continue; }
  pc = 0x1034bdc; continue;
 }
 0x1034bdc => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10c7a3bu32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(0x0u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x4000u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034bfe => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1388u32; cpu.ecx = v; }
  pc = 0x1034c08; continue;
 }
 0x1034c08 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)))); }
  if cpu.zf { pc = 0x1034c12; continue; }
  pc = 0x1034c0d; continue;
 }
 0x1034c0d => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1034c08; continue; }
  pc = 0x1034c12; continue;
 }
 0x1034c12 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x6u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ebx as u8), 3u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x10c7a5bu32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.ecx as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(0x0u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x4000u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0x80u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x10c79f1u32)); }
  if !cpu.zf { pc = 0x1034c5d; continue; }
  pc = 0x1034c4e; continue;
 }
 0x1034c4e => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), cpu.r8(0x10c79f2u32)); }
  if !cpu.zf { pc = 0x1034c5d; continue; }
  pc = 0x1034c56; continue;
 }
 0x1034c56 => {
  { let v: u16 = cpu.r16(0x10c79dfu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1034c5d; continue;
 }
 0x1034c5d => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x10c79d5u32)); }
  if !cpu.zf { pc = 0x1034c74; continue; }
  pc = 0x1034c65; continue;
 }
 0x1034c65 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), cpu.r8(0x10c79d6u32)); }
  if !cpu.zf { pc = 0x1034c74; continue; }
  pc = 0x1034c6d; continue;
 }
 0x1034c6d => {
  { let v: u16 = cpu.r16(0x10c79c3u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1034c74; continue;
 }
 0x1034c74 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x10c79d7u32)); }
  if !cpu.zf { pc = 0x1034c8b; continue; }
  pc = 0x1034c7c; continue;
 }
 0x1034c7c => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), cpu.r8(0x10c79d8u32)); }
  if !cpu.zf { pc = 0x1034c8b; continue; }
  pc = 0x1034c84; continue;
 }
 0x1034c84 => {
  { let v: u16 = cpu.r16(0x10c79c5u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1034c8b; continue;
 }
 0x1034c8b => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x10c79cdu32)); }
  if cpu.zf { pc = 0x1034ca4; continue; }
  pc = 0x1034c93; continue;
 }
 0x1034c93 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x10c79d3u32)); }
  if cpu.zf { pc = 0x1034cac; continue; }
  pc = 0x1034c9b; continue;
 }
 0x1034c9b => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x10c79d9u32)); }
  if cpu.zf { pc = 0x1034cb4; continue; }
  pc = 0x1034ca3; continue;
 }
 0x1034ca3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034ca4 => {
  { let v: u16 = cpu.r16(0x10c79bbu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034cac => {
  { let v: u16 = cpu.r16(0x10c79c1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1034cb4 => {
  { let v: u16 = cpu.r16(0x10c79c7u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034cbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034cbc); }
 let mut pc: u32 = 0x1034cbc;
 loop { match pc {
 0x1034cbc => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 3u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0xau32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))), (cpu.eax as u8)); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034cdd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034cdd); }
 let mut pc: u32 = 0x1034cdd;
 loop { match pc {
 0x1034cdd => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 3u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u8 = cpu.or8(cpu.r8(0xau32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))), (cpu.eax as u8)); cpu.w8(0xau32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034cfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034cfe); }
 let mut pc: u32 = 0x1034cfe;
 loop { match pc {
 0x1034cfe => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 3u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u8 = !(cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8(cpu.r8(0xau32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))), (cpu.eax as u8)); cpu.w8(0xau32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034d22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034d22); }
 let mut pc: u32 = 0x1034d22;
 loop { match pc {
 0x1034d22 => {
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11238d4u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x10c797du32; cpu.edi = v; }
  { let v: u32 = 0xbeu32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x11dbe24u32; cpu.esi = v; }
  { let v: u32 = 0x10c7a3bu32; cpu.edi = v; }
  { let v: u8 = 0x10u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1034d53; continue;
 }
 0x1034d53 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10c797du32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1034d53; continue; }
  pc = 0x1034d69; continue;
 }
 0x1034d69 => {
  { let v: u32 = 0x11dbe64u32; cpu.esi = v; }
  { let v: u32 = 0x10c7a5bu32; cpu.edi = v; }
  { let v: u8 = 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1034d75; continue;
 }
 0x1034d75 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10c797du32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1034d75; continue; }
  pc = 0x1034d8b; continue;
 }
 0x1034d8b => {
  { let v: u32 = 0x11dbe04u32; cpu.esi = v; }
  { let v: u32 = 0x11233fau32; cpu.edi = v; }
  { let v: u8 = 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1034d97; continue;
 }
 0x1034d97 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10c797du32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1034d97; continue; }
  pc = 0x1034dad; continue;
 }
 0x1034dad => {
  cpu.push(0x1034db2); f_01034db3(cpu); if cpu.returned_to != 0x1034db2 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01034db3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1034db3); }
 let mut pc: u32 = 0x1034db3;
 loop { match pc {
 0x1034db3 => {
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11230cdu32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x10f78bcu32, v); }
  { let v: u32 = cpu.r32(0x10f787au32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x10f782au32; cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = 0x10f785au32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x11238e8u32); cpu.esi = v; }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x10f7825u32, v); }
  { let v: u32 = 0x10f784au32; cpu.esi = v; }
  { let v: u32 = 0x10f7852u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x11232bau32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1123092u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1123094u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112308eu32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1123090u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112308au32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112308cu32, v); }
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 5u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x112330au32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1123074u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1123078u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1123076u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112307au32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1123070u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112307eu32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1123072u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1123082u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112307cu32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1123080u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1123088u32, v); }
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 3u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x112cae9u32); cpu.esi = v; }
  { let v: u32 = 0x112cae1u32; cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x11233aau32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112be28u32, v); }
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x11233b4u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112be2au32, v); }
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x112325bu32); cpu.esi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10f7dc7u32, v); }
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x11233beu32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1123086u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1123084u32, v); }
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1123274u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1123096u32, v); }
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1123260u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1123098u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112309au32, v); }
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x11233f0u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11230c0u32, v); }
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x11233d2u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11230c2u32, v); }
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x112327eu32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11b4c93u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11b4c95u32, v); }
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 5u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x112cb75u32); cpu.esi = v; }
  { let v: u32 = 0x112cb55u32; cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 3u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1123292u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11230c5u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11230c7u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11230c9u32, v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11230cbu32, v); }
  { let v: u8 = 0xc5u8; cpu.w8(0x10f74e2u32, v); }
  { let v: u8 = 0xc5u8; cpu.w8(0x10f74e8u32, v); }
  { let v: u8 = 0x5fu8; cpu.w8(0x10f7525u32, v); }
  { let v: u8 = 0x5fu8; cpu.w8(0x10f752bu32, v); }
  { let v: u8 = 0xeau8; cpu.w8(0x10f7568u32, v); }
  { let v: u8 = 0xeau8; cpu.w8(0x10f756eu32, v); }
  { let v: u8 = 0xf9u8; cpu.w8(0x10f75abu32, v); }
  { let v: u8 = 0xf9u8; cpu.w8(0x10f75b1u32, v); }
  { let v: u8 = 0xd4u8; cpu.w8(0x10f75eeu32, v); }
  { let v: u8 = 0xd4u8; cpu.w8(0x10f75f4u32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1035095; continue; }
  pc = 0x103504f; continue;
 }
 0x103504f => {
  { let v: u8 = 0x8cu8; cpu.w8(0x10f74e2u32, v); }
  { let v: u8 = 0x8cu8; cpu.w8(0x10f74e8u32, v); }
  { let v: u8 = 0xfeu8; cpu.w8(0x10f7525u32, v); }
  { let v: u8 = 0xfeu8; cpu.w8(0x10f752bu32, v); }
  { let v: u8 = 0xdfu8; cpu.w8(0x10f7568u32, v); }
  { let v: u8 = 0xdfu8; cpu.w8(0x10f756eu32, v); }
  { let v: u8 = 0x54u8; cpu.w8(0x10f75abu32, v); }
  { let v: u8 = 0x54u8; cpu.w8(0x10f75b1u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10f75eeu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10f75f4u32, v); }
  pc = 0x1035095; continue;
 }
 0x1035095 => {
  { let v: u32 = cpu.r32(0x11230cdu32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11233dcu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x10f7363u32; cpu.edi = v; }
  { let v: u32 = 0x43u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010350b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10350b1); }
 let mut pc: u32 = 0x10350b1;
 loop { match pc {
 0x10350b1 => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x10e690cu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x2u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x11230a0u32, v); }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x11230a8u32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x11230a2u32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x11230aau32, v); }
  { let v: u16 = cpu.r16(0x10e690eu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x2u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x112309eu32, v); }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x11230a6u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x112309cu32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x11230a4u32, v); }
  { let v: u16 = 0x3u16; cpu.w16(0x11230acu32, v); }
  { let v: u16 = 0x4u16; cpu.w16(0x11230aeu32, v); }
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1035126; continue;
 }
 0x1035126 => {
  { let v: u16 = cpu.r16(0x10e690cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x103512e; continue;
 }
 0x103512e => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x112309cu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10351dc; continue; }
  pc = 0x103513b; continue;
 }
 0x103513b => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x112309eu32)); }
  if (cpu.sf == cpu.of) { pc = 0x10351dc; continue; }
  pc = 0x1035148; continue;
 }
 0x1035148 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x11230a2u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10351dc; continue; }
  pc = 0x1035155; continue;
 }
 0x1035155 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x11230a0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x10351dc; continue; }
  pc = 0x103515e; continue;
 }
 0x103515e => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x11230a4u32)); }
  if cpu.zf { pc = 0x103519a; continue; }
  pc = 0x1035167; continue;
 }
 0x1035167 => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x11230a6u32)); }
  if cpu.zf { pc = 0x10351a5; continue; }
  pc = 0x1035170; continue;
 }
 0x1035170 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x11230aau32)); }
  if cpu.zf { pc = 0x10351b0; continue; }
  pc = 0x1035179; continue;
 }
 0x1035179 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x11230a8u32)); }
  if cpu.zf { pc = 0x10351bb; continue; }
  pc = 0x1035182; continue;
 }
 0x1035182 => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x11230acu32)); }
  if cpu.zf { pc = 0x10351c6; continue; }
  pc = 0x103518b; continue;
 }
 0x103518b => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x11230aeu32)); }
  if cpu.zf { pc = 0x10351d1; continue; }
  pc = 0x1035194; continue;
 }
 0x1035194 => {
  { let v: u16 = 0x78u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10351e5; continue;
 }
 0x103519a => {
  { let v: u16 = cpu.r16(0x10c79c1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x13u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10351e5; continue;
 }
 0x10351a5 => {
  { let v: u16 = cpu.r16(0x10c7983u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x13u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10351e5; continue;
 }
 0x10351b0 => {
  { let v: u16 = cpu.r16(0x10c798fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x13u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10351e5; continue;
 }
 0x10351bb => {
  { let v: u16 = cpu.r16(0x10c79b5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x13u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10351e5; continue;
 }
 0x10351c6 => {
  { let v: u16 = cpu.r16(0x10c7a05u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x13u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10351e5; continue;
 }
 0x10351d1 => {
  { let v: u16 = cpu.r16(0x10c7a17u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x13u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10351e5; continue;
 }
 0x10351dc => {
  { let v: u16 = cpu.r16(0x10c7995u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x13u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10351e5; continue;
 }
 0x10351e5 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e6910u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x10ea910u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103512e; continue; }
  pc = 0x10351fa; continue;
 }
 0x10351fa => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x10e690eu32)); }
  if !cpu.zf { cpu.poll(); pc = 0x1035126; continue; }
  pc = 0x1035210; continue;
 }
 0x1035210 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01035211(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1035211); }
 let mut pc: u32 = 0x1035211;
 loop { match pc {
 0x1035211 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10f5764u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x10352c3; continue; }
  pc = 0x1035228; continue;
 }
 0x1035228 => {
  { let v: u32 = 0x4001u32.wrapping_add(cpu.edi); cpu.eax = v; }
  cpu.push(0x1035233); f_01008d26(cpu); if cpu.returned_to != 0x1035233 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11dbf64u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x11dbf66u32, v); }
  { let v: u32 = 0x116aa58u32; cpu.esi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x1035252; continue;
 }
 0x1035252 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.r32(0x11dbf64u32); cpu.push(v); }
  { cpu.sub32(cpu.esi, cpu.r32(0x11dbf68u32)); }
  if cpu.zf { pc = 0x10352c6; continue; }
  pc = 0x1035262; continue;
 }
 0x1035262 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1035267; continue;
 }
 0x1035267 => {
  { let v: u16 = cpu.r16(0x11dbf64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x10352c6; continue; }
  pc = 0x1035272; continue;
 }
 0x1035272 => {
  { cpu.sub16((cpu.eax as u16), 0x200u16); }
  if (cpu.sf == cpu.of) { pc = 0x10352c6; continue; }
  pc = 0x1035278; continue;
 }
 0x1035278 => {
  { let v: u16 = cpu.r16(0x11dbf66u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x10352c6; continue; }
  pc = 0x1035285; continue;
 }
 0x1035285 => {
  { cpu.sub16((cpu.ebx as u16), 0x200u16); }
  if (cpu.sf == cpu.of) { pc = 0x10352c6; continue; }
  pc = 0x103528c; continue;
 }
 0x103528c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  cpu.push(0x10352a9); f_01034cbc(cpu); if cpu.returned_to != 0x10352a9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x10352bb; continue; }
  pc = 0x10352ad; continue;
 }
 0x10352ad => {
  { let v: u16 = cpu.r16(0x11dbf66u32).wrapping_sub(0x10u16); cpu.w16(0x11dbf66u32, v); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1035267; continue; }
  pc = 0x10352b9; continue;
 }
 0x10352b9 => {
  pc = 0x10352c6; continue;
 }
 0x10352bb => {
  { let v: u32 = cpu.pop(); cpu.w32(0x11dbf64u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x10352c3; continue;
 }
 0x10352c3 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10352c6 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x11dbf64u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8au32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1035252; continue; }
  pc = 0x10352db; continue;
 }
 0x10352db => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010352e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10352e0); }
 let mut pc: u32 = 0x10352e0;
 loop { match pc {
 0x10352e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10fcc17u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10fcc1bu32, v); }
  pc = 0x1035301; continue;
 }
 0x1035301 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1035319; continue; }
  pc = 0x1035308; continue;
 }
 0x1035308 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10fcc1bu32)); cpu.w32(0x10fcc1bu32, v); }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1035319; continue; }
  pc = 0x1035313; continue;
 }
 0x1035313 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10fcc17u32)); cpu.w32(0x10fcc17u32, v); }
  pc = 0x1035319; continue;
 }
 0x1035319 => {
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1035301; continue; }
  pc = 0x103531e; continue;
 }
 0x103531e => {
  { let v: u32 = cpu.r32(0x10fcc17u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x103533a; continue; }
  pc = 0x1035327; continue;
 }
 0x1035327 => {
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.r32(0x10fcc1bu32); cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  pc = 0x103533a; continue;
 }
 0x103533a => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103533e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103533e); }
 let mut pc: u32 = 0x103533e;
 loop { match pc {
 0x103533e => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x103539b; continue; }
  pc = 0x1035347; continue;
 }
 0x1035347 => {
  { let v: u32 = cpu.r32(0x10c6a5cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xb4u32); }
  if (cpu.sf != cpu.of) { pc = 0x103539b; continue; }
  pc = 0x1035355; continue;
 }
 0x1035355 => {
  { let v: u32 = cpu.r32(0x10fcc1fu32); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x103539b; continue; }
  pc = 0x103535e; continue;
 }
 0x103535e => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1035379; continue; }
  pc = 0x1035367; continue;
 }
 0x1035367 => {
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1035371); f_010352e0(cpu); if cpu.returned_to != 0x1035371 { return; }
  { cpu.sub16((cpu.eax as u16), 0xcu16); }
  if (cpu.sf == cpu.of) { pc = 0x103539b; continue; }
  pc = 0x1035377; continue;
 }
 0x1035377 => {
  pc = 0x1035389; continue;
 }
 0x1035379 => {
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1035383); f_010352e0(cpu); if cpu.returned_to != 0x1035383 { return; }
  { cpu.sub16((cpu.eax as u16), 0x19u16); }
  if (cpu.sf == cpu.of) { pc = 0x103539b; continue; }
  pc = 0x1035389; continue;
 }
 0x1035389 => {
  cpu.push(0x103538e); f_010356ae(cpu); if cpu.returned_to != 0x103538e { return; }
  { let v: u32 = cpu.r32(0x10c6a5cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0xau32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fcc1fu32, v); }
  pc = 0x103539b; continue;
 }
 0x103539b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103539c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103539c); }
 let mut pc: u32 = 0x103539c;
 loop { match pc {
 0x103539c => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x103542f; continue; }
  pc = 0x10353a9; continue;
 }
 0x10353a9 => {
  { let v: u32 = cpu.r32(0x10c6a5cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xb4u32); }
  if (cpu.sf != cpu.of) { pc = 0x103542f; continue; }
  pc = 0x10353bb; continue;
 }
 0x10353bb => {
  { let v: u32 = cpu.r32(0x10fcc23u32); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x103542f; continue; }
  pc = 0x10353c8; continue;
 }
 0x10353c8 => {
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10353d2); f_010352e0(cpu); if cpu.returned_to != 0x10353d2 { return; }
  { cpu.sub16((cpu.eax as u16), 0x4bu16); }
  if (cpu.sf != cpu.of) { pc = 0x103542f; continue; }
  pc = 0x10353d8; continue;
 }
 0x10353d8 => {
  { let v: u32 = cpu.r32(0x10c6a5cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fcc23u32, v); }
  { let v: u32 = cpu.r32(0x10fcbf1u32); cpu.ebp = v; }
  pc = 0x10353eb; continue;
 }
 0x10353eb => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1035403; continue; }
  pc = 0x10353f1; continue;
 }
 0x10353f1 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x1035403; continue; }
  pc = 0x10353f8; continue;
 }
 0x10353f8 => {
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee91du32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x103541c; continue; }
  pc = 0x1035403; continue;
 }
 0x1035403 => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x10353eb; continue; }
  pc = 0x1035411; continue;
 }
 0x1035411 => {
  { let v: u32 = 0x1152e54u32; cpu.w32(0x10fcbf1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103541c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10fcbf1u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x103542f); f_0103653f(cpu); if cpu.returned_to != 0x103542f { return; }
  pc = 0x103542f; continue;
 }
 0x103542f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01035430(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1035430); }
 let mut pc: u32 = 0x1035430;
 loop { match pc {
 0x1035430 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1035468; continue; }
  pc = 0x1035439; continue;
 }
 0x1035439 => {
  { let v: u32 = cpu.r32(0x10c6a5cu32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x10fcc4cu32), 0x0u8); }
  if cpu.zf { pc = 0x1035468; continue; }
  pc = 0x1035448; continue;
 }
 0x1035448 => {
  { let v: u32 = cpu.r32(0x10fcc48u32); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x1035468; continue; }
  pc = 0x1035451; continue;
 }
 0x1035451 => {
  { let v: u32 = cpu.r32(0x10c6a5cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0xau32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fcc48u32, v); }
  cpu.push(0x1035463); f_01035688(cpu); if cpu.returned_to != 0x1035463 { return; }
  cpu.push(0x1035468); f_01002564(cpu); if cpu.returned_to != 0x1035468 { return; }
  pc = 0x1035468; continue;
 }
 0x1035468 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01035469(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1035469); }
 let mut pc: u32 = 0x1035469;
 loop { match pc {
 0x1035469 => {
  { let v: u32 = cpu.r32(0x10c6a5cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0xau32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fcc48u32, v); }
  cpu.push(0x103547b); f_01035688(cpu); if cpu.returned_to != 0x103547b { return; }
  cpu.push(0x1035480); f_01002564(cpu); if cpu.returned_to != 0x1035480 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01035481(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1035481); }
 let mut pc: u32 = 0x1035481;
 loop { match pc {
 0x1035481 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10c6a5cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x3cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fcc23u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01035491(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1035491); }
 let mut pc: u32 = 0x1035491;
 loop { match pc {
 0x1035491 => {
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  pc = 0x1035496; continue;
 }
 0x1035496 => {
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x10e4350u32); cpu.push(0x103549d); cpu.call(t); if cpu.returned_to != 0x103549d { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1035496; continue; }
  pc = 0x10354a1; continue;
 }
 0x10354a1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010354a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10354a2); }
 let mut pc: u32 = 0x10354a2;
 loop { match pc {
 0x10354a2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  cpu.push(0x10354ae); f_0101340e(cpu); if cpu.returned_to != 0x10354ae { return; }
  cpu.push(0x10354b3); f_01035491(cpu); if cpu.returned_to != 0x10354b3 { return; }
  cpu.push(0x10354b8); f_010132ed(cpu); if cpu.returned_to != 0x10354b8 { return; }
  cpu.push(0x10354bd); f_010129b5(cpu); if cpu.returned_to != 0x10354bd { return; }
  cpu.push(0x10354c2); f_0101302f(cpu); if cpu.returned_to != 0x10354c2 { return; }
  cpu.push(0x10354c7); f_0101223e(cpu); if cpu.returned_to != 0x10354c7 { return; }
  cpu.push(0x10354cc); f_010357ab(cpu); if cpu.returned_to != 0x10354cc { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x110d76au32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x10354dd); f_010357cf(cpu); if cpu.returned_to != 0x10354dd { return; }
  { cpu.sub8(cpu.r8(0x1124469u32), 0x0u8); }
  if cpu.zf { pc = 0x10354f2; continue; }
  pc = 0x10354e6; continue;
 }
 0x10354e6 => {
  cpu.push(0x10354eb); f_0103587a(cpu); if cpu.returned_to != 0x10354eb { return; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  cpu.push(0x10354f2); f_0103589b(cpu); if cpu.returned_to != 0x10354f2 { return; }
  pc = 0x10354f2; continue;
 }
 0x10354f2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010354f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10354f3); }
 let mut pc: u32 = 0x10354f3;
 loop { match pc {
 0x10354f3 => {
  { cpu.sub32(cpu.r32(0x10fcc27u32), 0x0u32); }
  if !cpu.zf { pc = 0x103551c; continue; }
  pc = 0x10354fc; continue;
 }
 0x10354fc => {
  { let v: u8 = 0xau8; cpu.w8(0x11cccdfu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fcc2fu32, v); }
  { let v: u32 = 0x1152e54u32; cpu.w32(0x10fcc2bu32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x10fcc27u32, v); }
  pc = 0x103551c; continue;
 }
 0x103551c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103551d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103551d); }
 let mut pc: u32 = 0x103551d;
 loop { match pc {
 0x103551d => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0xa9u32); }
  if (cpu.sf == cpu.of) { pc = 0x10355a4; continue; }
  pc = 0x103552d; continue;
 }
 0x103552d => {
  { cpu.sub32(cpu.r32(0x10fcc27u32), 0x0u32); }
  if cpu.zf { pc = 0x10355a4; continue; }
  pc = 0x103553a; continue;
 }
 0x103553a => {
  if cpu.sf { pc = 0x10355a4; continue; }
  pc = 0x1035540; continue;
 }
 0x1035540 => {
  { cpu.sub8(cpu.r8(0x11cccdfu32), 0x0u8); }
  if cpu.zf { pc = 0x10355a4; continue; }
  pc = 0x1035549; continue;
 }
 0x1035549 => {
  if cpu.sf { pc = 0x10355a4; continue; }
  pc = 0x103554b; continue;
 }
 0x103554b => {
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x10355a4; continue; }
  pc = 0x1035554; continue;
 }
 0x1035554 => {
  { let v: u32 = cpu.r32(0x10fcc2bu32); cpu.ebp = v; }
  pc = 0x103555a; continue;
 }
 0x103555a => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1035572; continue; }
  pc = 0x1035560; continue;
 }
 0x1035560 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x1035572; continue; }
  pc = 0x1035567; continue;
 }
 0x1035567 => {
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10fcc2fu32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x103558b; continue; }
  pc = 0x1035572; continue;
 }
 0x1035572 => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x103555a; continue; }
  pc = 0x1035580; continue;
 }
 0x1035580 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10fcc27u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103558b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10fcc2bu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x103559e); f_01036596(cpu); if cpu.returned_to != 0x103559e { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x11cccdfu32)); cpu.w8(0x11cccdfu32, v); }
  pc = 0x10355a4; continue;
 }
 0x10355a4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010355a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10355a5); }
 let mut pc: u32 = 0x10355a5;
 loop { match pc {
 0x10355a5 => {
  { cpu.sub32(cpu.r32(0x10fcc30u32), 0x0u32); }
  if !cpu.zf { pc = 0x10355ce; continue; }
  pc = 0x10355ae; continue;
 }
 0x10355ae => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fcc38u32, v); }
  { let v: u32 = 0x1152e54u32; cpu.w32(0x10fcc34u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x10fcc30u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11cccdeu32, v); }
  pc = 0x10355ce; continue;
 }
 0x10355ce => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010355cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10355cf); }
 let mut pc: u32 = 0x10355cf;
 loop { match pc {
 0x10355cf => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0xa9u32); }
  if (cpu.sf == cpu.of) { pc = 0x1035687; continue; }
  pc = 0x10355df; continue;
 }
 0x10355df => {
  { cpu.sub32(cpu.r32(0x10fcc30u32), 0x0u32); }
  if cpu.zf { pc = 0x1035687; continue; }
  pc = 0x10355ec; continue;
 }
 0x10355ec => {
  if cpu.sf { pc = 0x1035687; continue; }
  pc = 0x10355f2; continue;
 }
 0x10355f2 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x11cccdeu32)); cpu.w8(0x11cccdeu32, v); }
  if !cpu.sf { pc = 0x1035687; continue; }
  pc = 0x10355fe; continue;
 }
 0x10355fe => {
  { let v: u8 = 0x11u8; cpu.w8(0x11cccdeu32, v); }
  { let v: u32 = cpu.r32(0x11d02bdu32); cpu.eax = v; }
  cpu.push(0x103560f); f_010034b2(cpu); if cpu.returned_to != 0x103560f { return; }
  { cpu.sub32(cpu.r32(0x10fcc02u32), 0x0u32); }
  if !cpu.zf { pc = 0x1035687; continue; }
  pc = 0x103561c; continue;
 }
 0x103561c => {
  { let v: u32 = cpu.r32(0x10fcc34u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1035643; continue; }
  pc = 0x1035628; continue;
 }
 0x1035628 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x1035643; continue; }
  pc = 0x103562f; continue;
 }
 0x103562f => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1035643; continue; }
  pc = 0x1035638; continue;
 }
 0x1035638 => {
  cpu.push(0x103563d); f_010504cc(cpu); if cpu.returned_to != 0x103563d { return; }
  { let v: u32 = cpu.r32(0x10fcc34u32); cpu.ebp = v; }
  pc = 0x1035643; continue;
 }
 0x1035643 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x103565b; continue; }
  pc = 0x1035649; continue;
 }
 0x1035649 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x103565b; continue; }
  pc = 0x1035650; continue;
 }
 0x1035650 => {
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10fcc38u32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1035674; continue; }
  pc = 0x103565b; continue;
 }
 0x103565b => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x1035643; continue; }
  pc = 0x1035669; continue;
 }
 0x1035669 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10fcc30u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1035674 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10fcc34u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x1035687); f_01036604(cpu); if cpu.returned_to != 0x1035687 { return; }
  pc = 0x1035687; continue;
 }
 0x1035687 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01035688(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1035688); }
 let mut pc: u32 = 0x1035688;
 loop { match pc {
 0x1035688 => {
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x10356ac; continue; }
  pc = 0x1035692; continue;
 }
 0x1035692 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x10356ac; continue; }
  pc = 0x10356a7; continue;
 }
 0x10356a7 => {
  cpu.push(0x10356ac); f_010363b2(cpu); if cpu.returned_to != 0x10356ac { return; }
  pc = 0x10356ac; continue;
 }
 0x10356ac => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010356ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10356ae); }
 let mut pc: u32 = 0x10356ae;
 loop { match pc {
 0x10356ae => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x10356fa; continue; }
  pc = 0x10356ba; continue;
 }
 0x10356ba => {
  { let v: u32 = 0x3ffu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10fd2bbu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10356cb); f_01036086(cpu); if cpu.returned_to != 0x10356cb { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10fd2bbu32, v); }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10356dc; continue; }
  pc = 0x10356d7; continue;
 }
 0x10356d7 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x10356dc; continue;
 }
 0x10356dc => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x11057a7u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x10356fa; continue; }
  pc = 0x10356f5; continue;
 }
 0x10356f5 => {
  cpu.push(0x10356fa); f_010363b2(cpu); if cpu.returned_to != 0x10356fa { return; }
  pc = 0x10356fa; continue;
 }
 0x10356fa => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010356fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10356fe); }
 let mut pc: u32 = 0x10356fe;
 loop { match pc {
 0x10356fe => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x103572c; continue; }
  pc = 0x1035702; continue;
 }
 0x1035702 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x112c96eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x103571a; continue; }
  pc = 0x1035716; continue;
 }
 0x1035716 => {
  { let v: u16 = 0x0u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x103571a; continue;
 }
 0x103571a => {
  { cpu.sub16((cpu.edx as u16), 0x100u16); }
  if (cpu.sf != cpu.of) { pc = 0x1035725; continue; }
  pc = 0x1035721; continue;
 }
 0x1035721 => {
  { let v: u16 = 0xffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1035725; continue;
 }
 0x1035725 => {
  cpu.push(0x103572a); f_01012bd0(cpu); if cpu.returned_to != 0x103572a { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x103572c; continue;
 }
 0x103572c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103572d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103572d); }
 let mut pc: u32 = 0x103572d;
 loop { match pc {
 0x103572d => {
  { cpu.sub8(cpu.r8(0x1124468u32), 0x0u8); }
  if !cpu.zf { pc = 0x103578c; continue; }
  pc = 0x1035736; continue;
 }
 0x1035736 => {
  { let v: u32 = cpu.r32(0x10e4160u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10fd2c4u32); cpu.edx = v; }
  cpu.push(0x1035746); f_01011c8e(cpu); if cpu.returned_to != 0x1035746 { return; }
  { let v: u32 = 0x2b11u32; cpu.eax = v; }
  { let v: u32 = 0x46u32; cpu.edx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x105e84cu32; cpu.ebx = v; }
  cpu.push(0x103575d); f_0101345d(cpu); if cpu.returned_to != 0x103575d { return; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.sf { pc = 0x1035794; continue; }
  pc = 0x1035762; continue;
 }
 0x1035762 => {
  { let v: u8 = 0x1u8; cpu.w8(0x1124468u32, v); }
  cpu.push(0x103576e); f_0103589b(cpu); if cpu.returned_to != 0x103576e { return; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x1035778); f_01012a8c(cpu); if cpu.returned_to != 0x1035778 { return; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x103578c; continue; }
  pc = 0x1035781; continue;
 }
 0x1035781 => {
  { let v: u16 = 0x2u16; cpu.w16(0x10e3efdu32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103578c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1035794 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103579b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103579b); }
 let mut pc: u32 = 0x103579b;
 loop { match pc {
 0x103579b => {
  cpu.push(0x10357a0); f_0101223e(cpu); if cpu.returned_to != 0x10357a0 { return; }
  { let v: u32 = 0x11b5188u32; cpu.eax = v; }
  cpu.push(0x10357aa); f_0101244e(cpu); if cpu.returned_to != 0x10357aa { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010357ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10357ab); }
 let mut pc: u32 = 0x10357ab;
 loop { match pc {
 0x10357ab => {
  cpu.push(0x10357b0); f_0101223e(cpu); if cpu.returned_to != 0x10357b0 { return; }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112457cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.push(0x10357ce); f_0101244e(cpu); if cpu.returned_to != 0x10357ce { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010357cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10357cf); }
 let mut pc: u32 = 0x10357cf;
 loop { match pc {
 0x10357cf => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x10357d7); f_0101329b(cpu); if cpu.returned_to != 0x10357d7 { return; }
  cpu.push(0x10357dc); f_010358c0(cpu); if cpu.returned_to != 0x10357dc { return; }
  { cpu.popad(); }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x103584b; continue; }
  pc = 0x10357e6; continue;
 }
 0x10357e6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3d63u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11245b8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x112456fu32; cpu.edi = v; }
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x112456eu32; cpu.edx = v; }
  cpu.push(0x103581e); f_01013067(cpu); if cpu.returned_to != 0x103581e { return; }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x11245efu32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x112446du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  cpu.push(0x1035845); f_010133d9(cpu); if cpu.returned_to != 0x1035845 { return; }
  cpu.push(0x103584a); f_0103589b(cpu); if cpu.returned_to != 0x103584a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103584b => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1035855; continue; }
  pc = 0x1035850; continue;
 }
 0x1035850 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1035855; continue;
 }
 0x1035855 => {
  { let v: u32 = cpu.r32(0x11245f4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x112456fu32; cpu.edi = v; }
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x112456eu32; cpu.edx = v; }
  cpu.push(0x1035874); f_01013067(cpu); if cpu.returned_to != 0x1035874 { return; }
  cpu.push(0x1035879); f_0103589b(cpu); if cpu.returned_to != 0x1035879 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103587a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103587a); }
 let mut pc: u32 = 0x103587a;
 loop { match pc {
 0x103587a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x1035881); f_01013224(cpu); if cpu.returned_to != 0x1035881 { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  cpu.push(0x1035895); f_010131a8(cpu); if cpu.returned_to != 0x1035895 { return; }
  cpu.push(0x103589a); f_0103589b(cpu); if cpu.returned_to != 0x103589a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
