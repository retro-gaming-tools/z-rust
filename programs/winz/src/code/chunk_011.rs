#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_00477d03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x477d03); }
 let mut pc: u32 = 0x477d03;
 loop { match pc {
 0x477d03 => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x477d0d; continue;
 }
 0x477d0d => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x477dd7; continue; }
  pc = 0x477d17; continue;
 }
 0x477d17 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x13u32.wrapping_add(cpu.esi))); cpu.w8(0x13u32.wrapping_add(cpu.esi), v); }
  if !cpu.sf { pc = 0x477dd7; continue; }
  pc = 0x477d20; continue;
 }
 0x477d20 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x13u32.wrapping_add(cpu.esi))); cpu.w8(0x13u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x477dd7; continue; }
  pc = 0x477d2d; continue;
 }
 0x477d2d => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x477dd7; continue; }
  pc = 0x477d37; continue;
 }
 0x477d37 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.eax)), 0x3u8); }
  if !cpu.zf { pc = 0x477dd7; continue; }
  pc = 0x477d41; continue;
 }
 0x477d41 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x477d4a); f_00478676(cpu); if cpu.returned_to != 0x477d4a { return; }
  if cpu.cf { pc = 0x477dd7; continue; }
  pc = 0x477d50; continue;
 }
 0x477d50 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x477dd7; continue; }
  pc = 0x477d59; continue;
 }
 0x477d59 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x477dd7; continue; }
  pc = 0x477d60; continue;
 }
 0x477d60 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d39d0u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5d39d2u32, v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x4000u32); }
  if !cpu.cf { pc = 0x477dd7; continue; }
  pc = 0x477db5; continue;
 }
 0x477db5 => {
  { let v: u16 = cpu.r16(0x5d39d0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d39d2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x477dd1); f_00489637(cpu); if cpu.returned_to != 0x477dd1 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x64u8; cpu.w8(0x13u32.wrapping_add(cpu.esi), v); }
  pc = 0x477dd7; continue;
 }
 0x477dd7 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x477d0d; continue; }
  pc = 0x477de1; continue;
 }
 0x477de1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00477de2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x477de2); }
 let mut pc: u32 = 0x477de2;
 loop { match pc {
 0x477de2 => {
  { cpu.sub8(cpu.r8(0x4d67dau32), 0x12u8); }
  if cpu.zf { pc = 0x477e9d; continue; }
  pc = 0x477def; continue;
 }
 0x477def => {
  { let v: u32 = 0x559754u32; cpu.ebp = v; }
  { let v: u8 = 0xc8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x477df6; continue;
 }
 0x477df6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x477e92; continue; }
  pc = 0x477e04; continue;
 }
 0x477e04 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x477e92; continue; }
  pc = 0x477e10; continue;
 }
 0x477e10 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4b839cu32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xc8u32); }
  if cpu.zf { pc = 0x477e73; continue; }
  pc = 0x477e1e; continue;
 }
 0x477e1e => {
  { cpu.sub32(cpu.ebx, 0x271u32); }
  if cpu.zf { pc = 0x477e73; continue; }
  pc = 0x477e26; continue;
 }
 0x477e26 => {
  { cpu.sub32(cpu.ebx, 0x2d5u32); }
  if cpu.zf { pc = 0x477e73; continue; }
  pc = 0x477e2e; continue;
 }
 0x477e2e => {
  { cpu.sub32(cpu.ebx, 0x307u32); }
  if cpu.zf { pc = 0x477e73; continue; }
  pc = 0x477e36; continue;
 }
 0x477e36 => {
  { cpu.sub32(cpu.ebx, 0x20du32); }
  if cpu.zf { pc = 0x477e73; continue; }
  pc = 0x477e3e; continue;
 }
 0x477e3e => {
  { cpu.sub32(cpu.ebx, 0x226u32); }
  if cpu.zf { pc = 0x477e73; continue; }
  pc = 0x477e46; continue;
 }
 0x477e46 => {
  { cpu.sub32(cpu.ebx, 0x23fu32); }
  if cpu.zf { pc = 0x477e73; continue; }
  pc = 0x477e4e; continue;
 }
 0x477e4e => {
  { cpu.sub32(cpu.ebx, 0x258u32); }
  if cpu.zf { pc = 0x477e73; continue; }
  pc = 0x477e56; continue;
 }
 0x477e56 => {
  { cpu.sub32(cpu.ebx, 0x339u32); }
  if cpu.zf { pc = 0x477e73; continue; }
  pc = 0x477e5e; continue;
 }
 0x477e5e => {
  { cpu.sub32(cpu.ebx, 0x36bu32); }
  if cpu.zf { pc = 0x477e73; continue; }
  pc = 0x477e66; continue;
 }
 0x477e66 => {
  { cpu.sub32(cpu.ebx, 0x39du32); }
  if cpu.zf { pc = 0x477e73; continue; }
  pc = 0x477e6e; continue;
 }
 0x477e6e => {
  { cpu.sub32(cpu.ebx, 0x7du32); }
  if !cpu.zf { pc = 0x477e92; continue; }
  pc = 0x477e73; continue;
 }
 0x477e73 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x477e81); f_0047a2d9(cpu); if cpu.returned_to != 0x477e81 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x477e92; continue; }
  pc = 0x477e89; continue;
 }
 0x477e89 => {
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.zf { pc = 0x477e92; continue; }
  pc = 0x477e8d; continue;
 }
 0x477e8d => {
  { let v: u8 = cpu.r8(0x1au32.wrapping_add(cpu.ebp)) & 0xfeu8; cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  pc = 0x477e92; continue;
 }
 0x477e92 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x28u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x477df6; continue; }
  pc = 0x477e9d; continue;
 }
 0x477e9d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00477e9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x477e9e); }
 let mut pc: u32 = 0x477e9e;
 loop { match pc {
 0x477e9e => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x477ef1; continue; }
  pc = 0x477ea5; continue;
 }
 0x477ea5 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.zf { pc = 0x477ec5; continue; }
  pc = 0x477eab; continue;
 }
 0x477eab => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x477ec5; continue; }
  pc = 0x477eb1; continue;
 }
 0x477eb1 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x477ec5; continue; }
  pc = 0x477eb7; continue;
 }
 0x477eb7 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x477ed9; continue; }
  pc = 0x477ebd; continue;
 }
 0x477ebd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.eax)), 0x2u8); }
  if !cpu.zf { pc = 0x477ef4; continue; }
  pc = 0x477ec5; continue;
 }
 0x477ec5 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x477ed9; continue; }
  pc = 0x477ecd; continue;
 }
 0x477ecd => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x477ed9; continue; }
  pc = 0x477ed2; continue;
 }
 0x477ed2 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x477ed8); f_00454b5f(cpu); if cpu.returned_to != 0x477ed8 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x477ed9; continue;
 }
 0x477ed9 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x477ee0); f_00478676(cpu); if cpu.returned_to != 0x477ee0 { return; }
  if cpu.cf { pc = 0x477ef1; continue; }
  pc = 0x477ee2; continue;
 }
 0x477ee2 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5d3a04u32); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x477ef1); f_00479a1a(cpu); if cpu.returned_to != 0x477ef1 { return; }
  pc = 0x477ef1; continue;
 }
 0x477ef1 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x477ef4 => {
  { let v: u8 = cpu.or8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); cpu.w8(0x10u32.wrapping_add(cpu.esi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00477efa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x477efa); }
 let mut pc: u32 = 0x477efa;
 loop { match pc {
 0x477efa => {
  cpu.push(0x477eff); f_00477fd9(cpu); if cpu.returned_to != 0x477eff { return; }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x477f09; continue;
 }
 0x477f09 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x522514u32); cpu.eax = v; }
  if cpu.zf { pc = 0x477f1d; continue; }
  pc = 0x477f12; continue;
 }
 0x477f12 => {
  { let v: u32 = 0x50u32; cpu.edx = v; }
  { let v = (cpu.edx as u8); cpu.div8(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x477f74; continue; }
  pc = 0x477f1d; continue;
 }
 0x477f1d => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.sf { pc = 0x477f74; continue; }
  pc = 0x477f26; continue;
 }
 0x477f26 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x477f74; continue; }
  pc = 0x477f2d; continue;
 }
 0x477f2d => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.edx)), 0x1u8); }
  if !cpu.zf { pc = 0x477f74; continue; }
  pc = 0x477f37; continue;
 }
 0x477f37 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x477f3e); f_00445c78(cpu); if cpu.returned_to != 0x477f3e { return; }
  { cpu.sub8((cpu.eax as u8), 0x1au8); }
  if cpu.cf { pc = 0x477f44; continue; }
  pc = 0x477f42; continue;
 }
 0x477f42 => {
  { let v: u8 = 0x19u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x477f44; continue;
 }
 0x477f44 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x50u32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4d6be4u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bdcu32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x477f72); f_00477f7b(cpu); if cpu.returned_to != 0x477f72 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x477f74; continue;
 }
 0x477f74 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x1eu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x477f09; continue; }
  pc = 0x477f7a; continue;
 }
 0x477f7a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00477f7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x477f7b); }
 let mut pc: u32 = 0x477f7b;
 loop { match pc {
 0x477f7b => {
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x477f85; continue;
 }
 0x477f85 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.sf { pc = 0x477fd2; continue; }
  pc = 0x477f8c; continue;
 }
 0x477f8c => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x477fce; continue; }
  pc = 0x477f92; continue;
 }
 0x477f92 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6bd8u32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6bdcu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x4d6be4u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x477fd2; continue; }
  pc = 0x477fce; continue;
 }
 0x477fce => {
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.esi)) | 0x1u8; cpu.w8(0x1du32.wrapping_add(cpu.esi), v); }
  pc = 0x477fd2; continue;
 }
 0x477fd2 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x477f85; continue; }
  pc = 0x477fd8; continue;
 }
 0x477fd8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00477fd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x477fd9); }
 let mut pc: u32 = 0x477fd9;
 loop { match pc {
 0x477fd9 => {
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x477fe3; continue;
 }
 0x477fe3 => {
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.esi)) & 0xfeu8; cpu.w8(0x1du32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x477fe3; continue; }
  pc = 0x477fed; continue;
 }
 0x477fed => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004780a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4780a6); }
 let mut pc: u32 = 0x4780a6;
 loop { match pc {
 0x4780a6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004780a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4780a7); }
 let mut pc: u32 = 0x4780a7;
 loop { match pc {
 0x4780a7 => {
  { let v: u32 = 0x52bbf0u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x524b80u32; cpu.ebp = v; }
  pc = 0x4780b3; continue;
 }
 0x4780b3 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x478101; continue; }
  pc = 0x4780ba; continue;
 }
 0x4780ba => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x478101; continue; }
  pc = 0x4780c0; continue;
 }
 0x4780c0 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x478101; continue; }
  pc = 0x4780c6; continue;
 }
 0x4780c6 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x478101; continue; }
  pc = 0x4780cd; continue;
 }
 0x4780cd => {
  cpu.push(0x4780d2); f_00479411(cpu); if cpu.returned_to != 0x4780d2 { return; }
  if cpu.cf { pc = 0x478101; continue; }
  pc = 0x4780d4; continue;
 }
 0x4780d4 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x478101; continue; }
  pc = 0x4780dc; continue;
 }
 0x4780dc => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.edi)), 0x0u16); }
  if !cpu.zf { pc = 0x478101; continue; }
  pc = 0x4780e4; continue;
 }
 0x4780e4 => {
  { cpu.pushad(); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.push(0x478100); f_00479a1a(cpu); if cpu.returned_to != 0x478100 { return; }
  { cpu.popad(); }
  pc = 0x478101; continue;
 }
 0x478101 => {
  { let v: u32 = cpu.esi.wrapping_add(0x48u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x16u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4780b3; continue; }
  pc = 0x47810b; continue;
 }
 0x47810b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047810c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47810c); }
 let mut pc: u32 = 0x47810c;
 loop { match pc {
 0x47810c => {
  { let v: u32 = 0x52bbf0u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x524b80u32; cpu.ebp = v; }
  pc = 0x478118; continue;
 }
 0x478118 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4782e7; continue; }
  pc = 0x478125; continue;
 }
 0x478125 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x4782e7; continue; }
  pc = 0x47812f; continue;
 }
 0x47812f => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4782e7; continue; }
  pc = 0x478139; continue;
 }
 0x478139 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x47814b; continue; }
  pc = 0x478140; continue;
 }
 0x478140 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x9u8); }
  if !cpu.zf { pc = 0x4782e7; continue; }
  pc = 0x47814b; continue;
 }
 0x47814b => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x478152); f_00478676(cpu); if cpu.returned_to != 0x478152 { return; }
  if cpu.cf { pc = 0x4782e7; continue; }
  pc = 0x478158; continue;
 }
 0x478158 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.eax)), 0x2084u32); }
  if !cpu.zf { pc = 0x478166; continue; }
  pc = 0x478161; continue;
 }
 0x478161 => {
  pc = 0x4782e7; continue;
 }
 0x478166 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bf4u32, v); }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.eax)), 0x40u8); }
  if cpu.zf { pc = 0x478190; continue; }
  pc = 0x478174; continue;
 }
 0x478174 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x478184); f_0047a2d9(cpu); if cpu.returned_to != 0x478184 { return; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.zf { pc = 0x4781a0; continue; }
  pc = 0x47818b; continue;
 }
 0x47818b => {
  pc = 0x478226; continue;
 }
 0x478190 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x478226; continue; }
  pc = 0x4781a0; continue;
 }
 0x4781a0 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x4781d1; continue; }
  pc = 0x4781ac; continue;
 }
 0x4781ac => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.eax)), 0x40u8); }
  if cpu.zf { pc = 0x4781c6; continue; }
  pc = 0x4781b5; continue;
 }
 0x4781b5 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.eax)) & 0xfbu8; cpu.w8(0xfu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  pc = 0x4781d1; continue;
 }
 0x4781c6 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.eax)) & 0xfbu8; cpu.w8(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x37u32.wrapping_add(cpu.eax), v); }
  pc = 0x4781d1; continue;
 }
 0x4781d1 => {
  { cpu.pushad(); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xc8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  cpu.push(0x47821b); f_00413337(cpu); if cpu.returned_to != 0x47821b { return; }
  { cpu.popad(); }
  { let v: u8 = 0x8u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4782e7; continue;
 }
 0x478226 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x4782c4; continue; }
  pc = 0x478236; continue;
 }
 0x478236 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x9u8); }
  if !cpu.zf { pc = 0x4782e7; continue; }
  pc = 0x478241; continue;
 }
 0x478241 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.eax)), 0x40u8); }
  if cpu.zf { pc = 0x478282; continue; }
  pc = 0x47824a; continue;
 }
 0x47824a => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x47825c; continue; }
  pc = 0x478253; continue;
 }
 0x478253 => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x478267; continue; }
  pc = 0x47825c; continue;
 }
 0x47825c => {
  { cpu.sub8(cpu.r8(0x83u32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x478267; continue; }
  pc = 0x478265; continue;
 }
 0x478265 => {
  pc = 0x478269; continue;
 }
 0x478267 => {
  pc = 0x4782e7; continue;
 }
 0x478269 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.eax)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x4781a0; continue; }
  pc = 0x478276; continue;
 }
 0x478276 => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.eax)), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x4781a0; continue; }
  pc = 0x478280; continue;
 }
 0x478280 => {
  pc = 0x4782e7; continue;
 }
 0x478282 => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x478294; continue; }
  pc = 0x47828b; continue;
 }
 0x47828b => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x47829f; continue; }
  pc = 0x478294; continue;
 }
 0x478294 => {
  { cpu.sub8(cpu.r8(0x83u32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x47829f; continue; }
  pc = 0x47829d; continue;
 }
 0x47829d => {
  pc = 0x4782a1; continue;
 }
 0x47829f => {
  pc = 0x4782e7; continue;
 }
 0x4782a1 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.eax)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x4781a0; continue; }
  pc = 0x4782ae; continue;
 }
 0x4782ae => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.eax)), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x4781a0; continue; }
  pc = 0x4782b8; continue;
 }
 0x4782b8 => {
  { cpu.sub8(cpu.r8(0x1cu32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x4781a0; continue; }
  pc = 0x4782c2; continue;
 }
 0x4782c2 => {
  pc = 0x4782e7; continue;
 }
 0x4782c4 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.edi)), 0x0u16); }
  if cpu.zf { pc = 0x4782d2; continue; }
  pc = 0x4782d0; continue;
 }
 0x4782d0 => {
  if !cpu.sf { pc = 0x4782d8; continue; }
  pc = 0x4782d2; continue;
 }
 0x4782d2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.poll(); pc = 0x4781a0; continue;
 }
 0x4782d8 => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1cu32.wrapping_add(cpu.edi))); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x4781a0; continue; }
  pc = 0x4782e7; continue;
 }
 0x4782e7 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x48u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x16u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x478118; continue; }
  pc = 0x4782f7; continue;
 }
 0x4782f7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004782f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4782f8); }
 let mut pc: u32 = 0x4782f8;
 loop { match pc {
 0x4782f8 => {
  { cpu.pushad(); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x47831f; continue; }
  pc = 0x47830c; continue;
 }
 0x47830c => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax & 0x303u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x47831f; continue; }
  pc = 0x47831c; continue;
 }
 0x47831c => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47831f => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00478322(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x478322); } r_00478322(cpu, 0x478322); }
pub fn f_00478478(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x478478); } r_00478322(cpu, 0x478478); }
fn r_00478322(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x478322 => {
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x47832c; continue;
 }
 0x47832c => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x47833e; continue; }
  pc = 0x478332; continue;
 }
 0x478332 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x478345; continue; }
  pc = 0x478338; continue;
 }
 0x478338 => {
  { cpu.sub8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x20u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x478345; continue; }
  pc = 0x47833e; continue;
 }
 0x47833e => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47832c; continue; }
  pc = 0x478343; continue;
 }
 0x478343 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x478345 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { cpu.poll(); pc = 0x478343; continue; }
  pc = 0x47834d; continue;
 }
 0x47834d => {
  { let v: u32 = 0x509290u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x478357; continue;
 }
 0x478357 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x478384; continue; }
  pc = 0x478360; continue;
 }
 0x478360 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x478384; continue; }
  pc = 0x478367; continue;
 }
 0x478367 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x478377); f_0047a2d9(cpu); if cpu.returned_to != 0x478377 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x478384; continue; }
  pc = 0x478380; continue;
 }
 0x478380 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x47838e; continue;
 }
 0x478384 => {
  { let v: u32 = cpu.add32(cpu.edi, 0xb8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x478357; continue; }
  pc = 0x47838c; continue;
 }
 0x47838c => {
  cpu.poll(); pc = 0x478343; continue;
 }
 0x47838e => {
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x478398; continue;
 }
 0x478398 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4784af; continue; }
  pc = 0x4783a5; continue;
 }
 0x4783a5 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if !cpu.zf { pc = 0x4784af; continue; }
  pc = 0x4783b3; continue;
 }
 0x4783b3 => {
  cpu.push(0x4783b8); f_00478632(cpu); if cpu.returned_to != 0x4783b8 { return; }
  if cpu.cf { pc = 0x4784af; continue; }
  pc = 0x4783be; continue;
 }
 0x4783be => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x10u8); }
  if !cpu.zf { pc = 0x4784af; continue; }
  pc = 0x4783c8; continue;
 }
 0x4783c8 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x4784af; continue; }
  pc = 0x4783d3; continue;
 }
 0x4783d3 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x4784af; continue; }
  pc = 0x4783de; continue;
 }
 0x4783de => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x4784af; continue; }
  pc = 0x4783e9; continue;
 }
 0x4783e9 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4784af; continue; }
  pc = 0x4783f4; continue;
 }
 0x4783f4 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x4784af; continue; }
  pc = 0x4783ff; continue;
 }
 0x4783ff => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x47842f; continue; }
  pc = 0x478406; continue;
 }
 0x478406 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x478414; continue; }
  pc = 0x47840d; continue;
 }
 0x47840d => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xfbu8); }
  if !cpu.zf { pc = 0x478489; continue; }
  pc = 0x478414; continue;
 }
 0x478414 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x47841b); f_00451959(cpu); if cpu.returned_to != 0x47841b { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x4784af; continue; }
  pc = 0x478425; continue;
 }
 0x478425 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x4784af; continue; }
  pc = 0x47842d; continue;
 }
 0x47842d => {
  pc = 0x478489; continue;
 }
 0x47842f => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x478489; continue; }
  pc = 0x478436; continue;
 }
 0x478436 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x478444; continue; }
  pc = 0x47843f; continue;
 }
 0x47843f => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x478449; continue; }
  pc = 0x478444; continue;
 }
 0x478444 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x478449; continue;
 }
 0x478449 => {
  { cpu.sub16(cpu.r16(0xeu32.wrapping_add(cpu.ebx)), 0xfb2eu16); }
  if cpu.zf { pc = 0x478468; continue; }
  pc = 0x478451; continue;
 }
 0x478451 => {
  { cpu.sub16(cpu.r16(0xeu32.wrapping_add(cpu.ebx)), 0xef1fu16); }
  if !cpu.zf { pc = 0x478489; continue; }
  pc = 0x478459; continue;
 }
 0x478459 => {
  { let v: u32 = cpu.r16(0x10u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.zf { pc = 0x478489; continue; }
  pc = 0x478466; continue;
 }
 0x478466 => {
  pc = 0x4784af; continue;
 }
 0x478468 => {
  { let v: u32 = cpu.r16(0x10u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x478489; continue; }
  pc = 0x478478; continue;
 }
 0x478478 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x4784af; continue; }
  pc = 0x478489; continue;
 }
 0x478489 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v = cpu.r32(0x5b02e4u32); cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b02e4u32, v); }
  cpu.push(0x4784a3); f_0047965e(cpu); if cpu.returned_to != 0x4784a3 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x5b02e4u32, v); }
  { let v: u32 = cpu.r32(0x5b02e4u32).wrapping_add(cpu.eax); cpu.w32(0x5b02e4u32, v); }
  pc = 0x4784af; continue;
 }
 0x4784af => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x478398; continue; }
  pc = 0x4784be; continue;
 }
 0x4784be => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004784c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4784c0); }
 let mut pc: u32 = 0x4784c0;
 loop { match pc {
 0x4784c0 => {
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x4784ca; continue;
 }
 0x4784ca => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x4784d6; continue; }
  pc = 0x4784d0; continue;
 }
 0x4784d0 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { pc = 0x4784dd; continue; }
  pc = 0x4784d6; continue;
 }
 0x4784d6 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4784ca; continue; }
  pc = 0x4784db; continue;
 }
 0x4784db => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4784dd => {
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd4u32, v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x4784fe; continue; }
  pc = 0x4784fa; continue;
 }
 0x4784fa => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x478500; continue;
 }
 0x4784fe => {
  cpu.poll(); pc = 0x4784d6; continue;
 }
 0x478500 => {
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x47850a; continue;
 }
 0x47850a => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.sf == cpu.of) { pc = 0x478623; continue; }
  pc = 0x478515; continue;
 }
 0x478515 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if !cpu.zf { pc = 0x478623; continue; }
  pc = 0x478523; continue;
 }
 0x478523 => {
  cpu.push(0x478528); f_00478632(cpu); if cpu.returned_to != 0x478528 { return; }
  if cpu.cf { pc = 0x478623; continue; }
  pc = 0x47852e; continue;
 }
 0x47852e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.eax))); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.zf { pc = 0x478623; continue; }
  pc = 0x47853e; continue;
 }
 0x47853e => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x478623; continue; }
  pc = 0x478549; continue;
 }
 0x478549 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x478623; continue; }
  pc = 0x478554; continue;
 }
 0x478554 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x478623; continue; }
  pc = 0x47855f; continue;
 }
 0x47855f => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x478623; continue; }
  pc = 0x47856a; continue;
 }
 0x47856a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x478623; continue; }
  pc = 0x478575; continue;
 }
 0x478575 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x478623; continue; }
  pc = 0x478580; continue;
 }
 0x478580 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x478623; continue; }
  pc = 0x47858b; continue;
 }
 0x47858b => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4785b3; continue; }
  pc = 0x478592; continue;
 }
 0x478592 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x4785a0; continue; }
  pc = 0x478599; continue;
 }
 0x478599 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xfbu8); }
  if !cpu.zf { pc = 0x47860d; continue; }
  pc = 0x4785a0; continue;
 }
 0x4785a0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4785a7); f_00451959(cpu); if cpu.returned_to != 0x4785a7 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x478623; continue; }
  pc = 0x4785ad; continue;
 }
 0x4785ad => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x478623; continue; }
  pc = 0x4785b1; continue;
 }
 0x4785b1 => {
  pc = 0x47860d; continue;
 }
 0x4785b3 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x47860d; continue; }
  pc = 0x4785ba; continue;
 }
 0x4785ba => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x4785c8; continue; }
  pc = 0x4785c3; continue;
 }
 0x4785c3 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x4785cd; continue; }
  pc = 0x4785c8; continue;
 }
 0x4785c8 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x4785cd; continue;
 }
 0x4785cd => {
  { cpu.sub16(cpu.r16(0xeu32.wrapping_add(cpu.ebx)), 0xfb2eu16); }
  if cpu.zf { pc = 0x4785ec; continue; }
  pc = 0x4785d5; continue;
 }
 0x4785d5 => {
  { cpu.sub16(cpu.r16(0xeu32.wrapping_add(cpu.ebx)), 0xef1fu16); }
  if !cpu.zf { pc = 0x47860d; continue; }
  pc = 0x4785dd; continue;
 }
 0x4785dd => {
  { let v: u32 = cpu.r16(0x10u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.zf { pc = 0x47860d; continue; }
  pc = 0x4785ea; continue;
 }
 0x4785ea => {
  pc = 0x478623; continue;
 }
 0x4785ec => {
  { let v: u32 = cpu.r16(0x10u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x47860d; continue; }
  pc = 0x4785fc; continue;
 }
 0x4785fc => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x478623; continue; }
  pc = 0x47860d; continue;
 }
 0x47860d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x478617); f_0047965e(cpu); if cpu.returned_to != 0x478617 { return; }
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x15u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x478623 => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47850a; continue; }
  pc = 0x478630; continue;
 }
 0x478630 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00478632(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x478632); }
 let mut pc: u32 = 0x478632;
 loop { match pc {
 0x478632 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x47866f; continue; }
  pc = 0x47863b; continue;
 }
 0x47863b => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x47866f; continue; }
  pc = 0x478642; continue;
 }
 0x478642 => {
  { let v: u32 = 0x16u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x524b80u32); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x15u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if !cpu.zf { pc = 0x47866f; continue; }
  pc = 0x478662; continue;
 }
 0x478662 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x47866f; continue; }
  pc = 0x478669; continue;
 }
 0x478669 => {
  { let v: u8 = 0x1u8; cpu.w8(0x15u32.wrapping_add(cpu.eax), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47866f => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00478676(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x478676); }
 let mut pc: u32 = 0x478676;
 loop { match pc {
 0x478676 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x524b80u32); cpu.eax = v; }
  { let v: u8 = 0x16u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = 0x48u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x4786ab; continue; }
  pc = 0x478695; continue;
 }
 0x478695 => {
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x4786ab; continue; }
  pc = 0x4786a8; continue;
 }
 0x4786a8 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4786ab => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004786ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4786ae); }
 let mut pc: u32 = 0x4786ae;
 loop { match pc {
 0x4786ae => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x524b80u32); cpu.eax = v; }
  { let v: u8 = 0x16u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = 0x48u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004786c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4786c5); }
 let mut pc: u32 = 0x4786c5;
 loop { match pc {
 0x4786c5 => {
  { cpu.sub8(cpu.r8(0x4e1fb7u32), 0x0u8); }
  if !cpu.zf { pc = 0x478700; continue; }
  pc = 0x4786ce; continue;
 }
 0x4786ce => {
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x4786d8; continue;
 }
 0x4786d8 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x4786fb; continue; }
  pc = 0x4786e0; continue;
 }
 0x4786e0 => {
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x4786fb; continue; }
  pc = 0x4786e8; continue;
 }
 0x4786e8 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x4786fb; continue; }
  pc = 0x4786f0; continue;
 }
 0x4786f0 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x4786fb; continue; }
  pc = 0x4786f6; continue;
 }
 0x4786f6 => {
  cpu.push(0x4786fb); f_00478701(cpu); if cpu.returned_to != 0x4786fb { return; }
  pc = 0x4786fb; continue;
 }
 0x4786fb => {
  { let v: u32 = 0x1eu32.wrapping_add(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4786d8; continue; }
  pc = 0x478700; continue;
 }
 0x478700 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00478701(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x478701); }
 let mut pc: u32 = 0x478701;
 loop { match pc {
 0x478701 => {
  { cpu.pushad(); }
  { let v: u32 = 0x524b80u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x47870e; continue;
 }
 0x47870e => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4787bc; continue; }
  pc = 0x478719; continue;
 }
 0x478719 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x4787bc; continue; }
  pc = 0x478724; continue;
 }
 0x478724 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x4787bc; continue; }
  pc = 0x47872f; continue;
 }
 0x47872f => {
  { cpu.sub32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x4787bc; continue; }
  pc = 0x478739; continue;
 }
 0x478739 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x4787c8; continue; }
  pc = 0x478744; continue;
 }
 0x478744 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x4787c8; continue; }
  pc = 0x47874b; continue;
 }
 0x47874b => {
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.edi)), 0xffu8); }
  if cpu.zf { pc = 0x4787bb; continue; }
  pc = 0x47875f; continue;
 }
 0x47875f => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x4787bb; continue; }
  pc = 0x478773; continue;
 }
 0x478773 => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = cpu.r8(0x12u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x13u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4787b1); f_00413337(cpu); if cpu.returned_to != 0x4787b1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x2u8); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4787bb => {
  { cpu.popad(); }
  pc = 0x4787bc; continue;
 }
 0x4787bc => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x16u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47870e; continue; }
  pc = 0x4787c8; continue;
 }
 0x4787c8 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004787ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4787ca); }
 let mut pc: u32 = 0x4787ca;
 loop { match pc {
 0x4787ca => {
  cpu.push(0x4787cf); f_004788f3(cpu); if cpu.returned_to != 0x4787cf { return; }
  cpu.push(0x4787d4); f_00478997(cpu); if cpu.returned_to != 0x4787d4 { return; }
  { cpu.sub32(cpu.r32(0x5d3a22u32), 0x0u32); }
  if cpu.zf { pc = 0x4788f2; continue; }
  pc = 0x4787e1; continue;
 }
 0x4787e1 => {
  { cpu.sub32(cpu.r32(0x5d3a1eu32), 0x0u32); }
  if cpu.zf { pc = 0x4788f2; continue; }
  pc = 0x4787ee; continue;
 }
 0x4787ee => {
  { let v: u32 = 0x5d3b26u32; cpu.ebx = v; }
  pc = 0x4787f3; continue;
 }
 0x4787f3 => {
  { cpu.sub32(cpu.r32(0x5d3a22u32), 0x0u32); }
  if cpu.zf { pc = 0x4788f2; continue; }
  pc = 0x478800; continue;
 }
 0x478800 => {
  if cpu.sf { pc = 0x4788f2; continue; }
  pc = 0x478806; continue;
 }
 0x478806 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d693au32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5d3a16u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5d3a1au32, v); }
  { let v: u32 = 0x5d3c26u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5d3a1eu32); cpu.ecx = v; }
  pc = 0x47883b; continue;
 }
 0x47883b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x4788b5; continue; }
  pc = 0x478847; continue;
 }
 0x478847 => {
  { cpu.sub32(cpu.esi, 0x0u32); }
  if cpu.zf { pc = 0x4788b5; continue; }
  pc = 0x47884c; continue;
 }
 0x47884c => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x4b8b48u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x4d6938u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d693au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x478892); f_004162eb(cpu); if cpu.returned_to != 0x478892 { return; }
  { cpu.sub32(cpu.eax, 0x10000u32); }
  if (cpu.sf == cpu.of) { pc = 0x4788b5; continue; }
  pc = 0x478899; continue;
 }
 0x478899 => {
  { cpu.sub32(cpu.r32(0x5d3a1au32), 0xffffffffu32); }
  if cpu.zf { pc = 0x4788aa; continue; }
  pc = 0x4788a2; continue;
 }
 0x4788a2 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5d3a1au32)); }
  if (cpu.sf == cpu.of) { pc = 0x4788b5; continue; }
  pc = 0x4788aa; continue;
 }
 0x4788aa => {
  { let v: u32 = cpu.eax; cpu.w32(0x5d3a1au32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5d3a16u32, v); }
  pc = 0x4788b5; continue;
 }
 0x4788b5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47883b; continue; }
  pc = 0x4788c4; continue;
 }
 0x4788c4 => {
  { cpu.sub32(cpu.r32(0x5d3a16u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x4788f2; continue; }
  pc = 0x4788cd; continue;
 }
 0x4788cd => {
  { let v: u32 = cpu.r32(0x5d3a16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x4788e4; continue; }
  pc = 0x4788da; continue;
 }
 0x4788da => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x4788e4; continue; }
  pc = 0x4788df; continue;
 }
 0x4788df => {
  cpu.push(0x4788e4); f_004789f6(cpu); if cpu.returned_to != 0x4788e4 { return; }
  pc = 0x4788e4; continue;
 }
 0x4788e4 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x5d3a22u32)); cpu.w32(0x5d3a22u32, v); }
  cpu.poll(); pc = 0x4787f3; continue;
 }
 0x4788f2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004788f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4788f3); }
 let mut pc: u32 = 0x4788f3;
 loop { match pc {
 0x4788f3 => {
  { cpu.pushad(); }
  { let v: u32 = 0x5d3b26u32; cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x0u32; cpu.w32(0x5d3a22u32, v); }
  { let v: u32 = 0x5d3b26u32; cpu.esi = v; }
  { let v: u32 = 0x524b80u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x478920; continue;
 }
 0x478920 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x47898d; continue; }
  pc = 0x478927; continue;
 }
 0x478927 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x47898d; continue; }
  pc = 0x47892e; continue;
 }
 0x47892e => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x47898d; continue; }
  pc = 0x478935; continue;
 }
 0x478935 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x47898d; continue; }
  pc = 0x47893c; continue;
 }
 0x47893c => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x478943); f_00478676(cpu); if cpu.returned_to != 0x478943 { return; }
  if cpu.cf { pc = 0x47898d; continue; }
  pc = 0x478945; continue;
 }
 0x478945 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x47898d; continue; }
  pc = 0x47894b; continue;
 }
 0x47894b => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x47898d; continue; }
  pc = 0x478951; continue;
 }
 0x478951 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x47895e; continue; }
  pc = 0x478959; continue;
 }
 0x478959 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x478963; continue; }
  pc = 0x47895e; continue;
 }
 0x47895e => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x478963; continue;
 }
 0x478963 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.sf == cpu.of) { pc = 0x47898d; continue; }
  pc = 0x478972; continue;
 }
 0x478972 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x5d3a22u32)); cpu.w32(0x5d3a22u32, v); }
  pc = 0x47898d; continue;
 }
 0x47898d => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x16u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x478920; continue; }
  pc = 0x478995; continue;
 }
 0x478995 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00478997(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x478997); }
 let mut pc: u32 = 0x478997;
 loop { match pc {
 0x478997 => {
  { let v: u32 = 0x5d3c26u32; cpu.esi = v; }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5d3a1eu32, v); }
  pc = 0x4789b0; continue;
 }
 0x4789b0 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4789ef; continue; }
  pc = 0x4789b8; continue;
 }
 0x4789b8 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x4789ef; continue; }
  pc = 0x4789bf; continue;
 }
 0x4789bf => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x4789ef; continue; }
  pc = 0x4789c8; continue;
 }
 0x4789c8 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x4789ef; continue; }
  pc = 0x4789d2; continue;
 }
 0x4789d2 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x5d3a1eu32)); cpu.w32(0x5d3a1eu32, v); }
  pc = 0x4789ef; continue;
 }
 0x4789ef => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4789b0; continue; }
  pc = 0x4789f5; continue;
 }
 0x4789f5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004789f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4789f6); }
 let mut pc: u32 = 0x4789f6;
 loop { match pc {
 0x4789f6 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x4789fe); f_00478676(cpu); if cpu.returned_to != 0x4789fe { return; }
  if cpu.cf { pc = 0x478a61; continue; }
  pc = 0x478a00; continue;
 }
 0x478a00 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x4b8b48u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x34u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x35u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  cpu.push(0x478a54); f_00413337(cpu); if cpu.returned_to != 0x478a54 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0xbu8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x478a61 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00478a64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x478a64); }
 let mut pc: u32 = 0x478a64;
 loop { match pc {
 0x478a64 => {
  { let v: u32 = 0x52bbf0u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x524b80u32; cpu.ebp = v; }
  pc = 0x478a70; continue;
 }
 0x478a70 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x478af0; continue; }
  pc = 0x478a79; continue;
 }
 0x478a79 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x478af0; continue; }
  pc = 0x478a7f; continue;
 }
 0x478a7f => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x478af0; continue; }
  pc = 0x478a85; continue;
 }
 0x478a85 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x478af0; continue; }
  pc = 0x478a8c; continue;
 }
 0x478a8c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x478af0; continue; }
  pc = 0x478a96; continue;
 }
 0x478a96 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x478a9d); f_00478676(cpu); if cpu.returned_to != 0x478a9d { return; }
  if cpu.cf { pc = 0x478af0; continue; }
  pc = 0x478a9f; continue;
 }
 0x478a9f => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x24u8); }
  if cpu.zf { pc = 0x478aad; continue; }
  pc = 0x478aa5; continue;
 }
 0x478aa5 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x26u8); }
  if cpu.zf { pc = 0x478aad; continue; }
  pc = 0x478aab; continue;
 }
 0x478aab => {
  pc = 0x478af0; continue;
 }
 0x478aad => {
  { let v: u8 = 0x8u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xc8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { cpu.pushad(); }
  cpu.push(0x478aef); f_00413337(cpu); if cpu.returned_to != 0x478aef { return; }
  { cpu.popad(); }
  pc = 0x478af0; continue;
 }
 0x478af0 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x48u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x16u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x478a70; continue; }
  pc = 0x478b00; continue;
 }
 0x478b00 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00478b03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x478b03); }
 let mut pc: u32 = 0x478b03;
 loop { match pc {
 0x478b03 => {
  cpu.push(0x478b08); f_00478df6(cpu); if cpu.returned_to != 0x478b08 { return; }
  { let v: u32 = 0x5d3aa6u32; cpu.ebx = v; }
  pc = 0x478b0d; continue;
 }
 0x478b0d => {
  { cpu.sub8(cpu.r8(0x5d3a14u32), 0x0u8); }
  if cpu.zf { pc = 0x478c00; continue; }
  pc = 0x478b1a; continue;
 }
 0x478b1a => {
  if cpu.sf { pc = 0x478c00; continue; }
  pc = 0x478b20; continue;
 }
 0x478b20 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d693au32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5d3a16u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5d3a1au32, v); }
  { let v: u32 = 0x5a4794u32; cpu.edi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x478b54; continue;
 }
 0x478b54 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x478bc8; continue; }
  pc = 0x478b60; continue;
 }
 0x478b60 => {
  { cpu.sub8(cpu.r8(0x37u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x478bc8; continue; }
  pc = 0x478b66; continue;
 }
 0x478b66 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x4b8b48u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x4d6938u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d693au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x478bac); f_004162eb(cpu); if cpu.returned_to != 0x478bac { return; }
  { cpu.sub32(cpu.r32(0x5d3a1au32), 0xffffffffu32); }
  if cpu.zf { pc = 0x478bbd; continue; }
  pc = 0x478bb5; continue;
 }
 0x478bb5 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5d3a1au32)); }
  if (cpu.sf == cpu.of) { pc = 0x478bc8; continue; }
  pc = 0x478bbd; continue;
 }
 0x478bbd => {
  { let v: u32 = cpu.eax; cpu.w32(0x5d3a1au32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5d3a16u32, v); }
  pc = 0x478bc8; continue;
 }
 0x478bc8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x478b54; continue; }
  pc = 0x478bd2; continue;
 }
 0x478bd2 => {
  { cpu.sub32(cpu.r32(0x5d3a16u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x478c00; continue; }
  pc = 0x478bdb; continue;
 }
 0x478bdb => {
  { let v: u32 = cpu.r32(0x5d3a16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x478bf2; continue; }
  pc = 0x478be8; continue;
 }
 0x478be8 => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x478bf2; continue; }
  pc = 0x478bed; continue;
 }
 0x478bed => {
  cpu.push(0x478bf2); f_00478e6f(cpu); if cpu.returned_to != 0x478bf2 { return; }
  pc = 0x478bf2; continue;
 }
 0x478bf2 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5d3a14u32)); cpu.w8(0x5d3a14u32, v); }
  cpu.poll(); pc = 0x478b0d; continue;
 }
 0x478c00 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00478c01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x478c01); }
 let mut pc: u32 = 0x478c01;
 loop { match pc {
 0x478c01 => {
  cpu.push(0x478c06); f_00478df6(cpu); if cpu.returned_to != 0x478c06 { return; }
  { let v: u32 = 0x5d3aa6u32; cpu.ebx = v; }
  pc = 0x478c0b; continue;
 }
 0x478c0b => {
  { cpu.sub8(cpu.r8(0x5d3a14u32), 0x0u8); }
  if cpu.zf { pc = 0x478df5; continue; }
  pc = 0x478c18; continue;
 }
 0x478c18 => {
  if cpu.sf { pc = 0x478df5; continue; }
  pc = 0x478c1e; continue;
 }
 0x478c1e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d693au32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5d3a16u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5d3a1au32, v); }
  { let v: u32 = 0x5ce9d8u32; cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x478c52; continue;
 }
 0x478c52 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x478d52; continue; }
  pc = 0x478c62; continue;
 }
 0x478c62 => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), 0x8u8); }
  if !cpu.zf { pc = 0x478d52; continue; }
  pc = 0x478c6c; continue;
 }
 0x478c6c => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x478cec; continue; }
  pc = 0x478c72; continue;
 }
 0x478c72 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x478ca0; continue; }
  pc = 0x478c9c; continue;
 }
 0x478c9c => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x478ca0; continue;
 }
 0x478ca0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x4d6938u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d693au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x478cb9); f_004162eb(cpu); if cpu.returned_to != 0x478cb9 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d3d2eu32, v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x11u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x478cde); f_004162eb(cpu); if cpu.returned_to != 0x478cde { return; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5d3d2eu32)); }
  if (cpu.sf == cpu.of) { pc = 0x478ce9; continue; }
  pc = 0x478ce6; continue;
 }
 0x478ce6 => {
  { cpu.popad(); }
  pc = 0x478d52; continue;
 }
 0x478ce9 => {
  { cpu.popad(); }
  pc = 0x478cf2; continue;
 }
 0x478cec => {
  { cpu.sub8(cpu.r8(0x19u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x478d52; continue; }
  pc = 0x478cf2; continue;
 }
 0x478cf2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x478d1f; continue; }
  pc = 0x478d1b; continue;
 }
 0x478d1b => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x478d1f; continue;
 }
 0x478d1f => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x4d6938u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d693au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x478d36); f_004162eb(cpu); if cpu.returned_to != 0x478d36 { return; }
  { cpu.sub32(cpu.r32(0x5d3a1au32), 0xffffffffu32); }
  if cpu.zf { pc = 0x478d47; continue; }
  pc = 0x478d3f; continue;
 }
 0x478d3f => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5d3a1au32)); }
  if (cpu.sf == cpu.of) { pc = 0x478d52; continue; }
  pc = 0x478d47; continue;
 }
 0x478d47 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5d3a1au32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5d3a16u32, v); }
  pc = 0x478d52; continue;
 }
 0x478d52 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x478c52; continue; }
  pc = 0x478d61; continue;
 }
 0x478d61 => {
  { cpu.sub32(cpu.r32(0x5d3a16u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x478df5; continue; }
  pc = 0x478d6e; continue;
 }
 0x478d6e => {
  { let v: u32 = cpu.r32(0x5d3a16u32); cpu.esi = v; }
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x478dd6; continue; }
  pc = 0x478d7a; continue;
 }
 0x478d7a => {
  { cpu.pushad(); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x11u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x478d90); f_00478632(cpu); if cpu.returned_to != 0x478d90 { return; }
  { let v: u8 = 0x8u8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x478d99); f_004786ae(cpu); if cpu.returned_to != 0x478d99 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xc8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  cpu.push(0x478dd5); f_00413337(cpu); if cpu.returned_to != 0x478dd5 { return; }
  { cpu.popad(); }
  pc = 0x478dd6; continue;
 }
 0x478dd6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x478de7; continue; }
  pc = 0x478ddd; continue;
 }
 0x478ddd => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x478de7; continue; }
  pc = 0x478de2; continue;
 }
 0x478de2 => {
  cpu.push(0x478de7); f_00478f3e(cpu); if cpu.returned_to != 0x478de7 { return; }
  pc = 0x478de7; continue;
 }
 0x478de7 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x5d3a14u32)); cpu.w8(0x5d3a14u32, v); }
  cpu.poll(); pc = 0x478c0b; continue;
 }
 0x478df5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00478df6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x478df6); }
 let mut pc: u32 = 0x478df6;
 loop { match pc {
 0x478df6 => {
  { cpu.pushad(); }
  { let v: u32 = 0x5d3aa6u32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u8 = 0x0u8; cpu.w8(0x5d3a14u32, v); }
  { let v: u32 = 0x5d3aa6u32; cpu.esi = v; }
  { let v: u32 = 0x524b80u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x478e20; continue;
 }
 0x478e20 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x478e65; continue; }
  pc = 0x478e27; continue;
 }
 0x478e27 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x478e65; continue; }
  pc = 0x478e2e; continue;
 }
 0x478e2e => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x9u8); }
  if cpu.zf { pc = 0x478e65; continue; }
  pc = 0x478e35; continue;
 }
 0x478e35 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x478e3c); f_00478676(cpu); if cpu.returned_to != 0x478e3c { return; }
  if cpu.cf { pc = 0x478e65; continue; }
  pc = 0x478e3e; continue;
 }
 0x478e3e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x24u8); }
  if !cpu.zf { pc = 0x478e65; continue; }
  pc = 0x478e44; continue;
 }
 0x478e44 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x478e65; continue; }
  pc = 0x478e4a; continue;
 }
 0x478e4a => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x5d3a14u32)); cpu.w8(0x5d3a14u32, v); }
  pc = 0x478e65; continue;
 }
 0x478e65 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x16u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x478e20; continue; }
  pc = 0x478e6d; continue;
 }
 0x478e6d => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00478e6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x478e6f); }
 let mut pc: u32 = 0x478e6f;
 loop { match pc {
 0x478e6f => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x478e77); f_00478676(cpu); if cpu.returned_to != 0x478e77 { return; }
  if cpu.cf { pc = 0x478f3b; continue; }
  pc = 0x478e7d; continue;
 }
 0x478e7d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x1cu32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x478e9d; continue; }
  pc = 0x478e98; continue;
 }
 0x478e98 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x478ea2; continue; }
  pc = 0x478e9d; continue;
 }
 0x478e9d => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x478ea2; continue;
 }
 0x478ea2 => {
  { let v: u32 = cpu.r8(0x6u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x478eae; continue; }
  pc = 0x478eac; continue;
 }
 0x478eac => {
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x478eae; continue;
 }
 0x478eae => {
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x84u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)) | 0x4u8; cpu.w8(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x37u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x4b8b48u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x34u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x35u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  cpu.push(0x478f2e); f_00413337(cpu); if cpu.returned_to != 0x478f2e { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x9u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x478f3b => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00478f3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x478f3e); }
 let mut pc: u32 = 0x478f3e;
 loop { match pc {
 0x478f3e => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x478f46); f_00478676(cpu); if cpu.returned_to != 0x478f46 { return; }
  if cpu.cf { pc = 0x479045; continue; }
  pc = 0x478f4c; continue;
 }
 0x478f4c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.esi)) | 0x4u8; cpu.w8(0xfu32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x478fb0; continue; }
  pc = 0x478fa4; continue;
 }
 0x478fa4 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x478fb0; continue;
 }
 0x478fb0 => {
  { cpu.pushad(); }
  cpu.push(0x478fb6); f_00420989(cpu); if cpu.returned_to != 0x478fb6 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  if cpu.zf { pc = 0x47900e; continue; }
  pc = 0x478fc2; continue;
 }
 0x478fc2 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x478fcd); f_00420989(cpu); if cpu.returned_to != 0x478fcd { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  if cpu.zf { pc = 0x47900e; continue; }
  pc = 0x478fd9; continue;
 }
 0x478fd9 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x478fe4); f_00420989(cpu); if cpu.returned_to != 0x478fe4 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  if cpu.zf { pc = 0x47900e; continue; }
  pc = 0x478ff0; continue;
 }
 0x478ff0 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x478fff); f_00420989(cpu); if cpu.returned_to != 0x478fff { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  if cpu.zf { pc = 0x47900e; continue; }
  pc = 0x47900b; continue;
 }
 0x47900b => {
  { cpu.popad(); }
  pc = 0x479010; continue;
 }
 0x47900e => {
  { cpu.popad(); }
  {  }
  pc = 0x479010; continue;
 }
 0x479010 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  cpu.push(0x479038); f_00413337(cpu); if cpu.returned_to != 0x479038 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x9u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x479045 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479048(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x479048); }
 let mut pc: u32 = 0x479048;
 loop { match pc {
 0x479048 => {
  { let v: u32 = 0x524b80u32; cpu.edi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x52bbf0u32; cpu.ebx = v; }
  pc = 0x479054; continue;
 }
 0x479054 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x4790a5; continue; }
  pc = 0x47905a; continue;
 }
 0x47905a => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x4790a5; continue; }
  pc = 0x479060; continue;
 }
 0x479060 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x479079; continue; }
  pc = 0x479074; continue;
 }
 0x479074 => {
  cpu.push(0x479079); f_004790b0(cpu); if cpu.returned_to != 0x479079 { return; }
  pc = 0x479079; continue;
 }
 0x479079 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { pc = 0x4790a5; continue; }
  pc = 0x47907f; continue;
 }
 0x47907f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x4790a5; continue; }
  pc = 0x479086; continue;
 }
 0x479086 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x4790a5; continue; }
  pc = 0x47908c; continue;
 }
 0x47908c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x479096); f_004793ab(cpu); if cpu.returned_to != 0x479096 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x4790a5; continue; }
  pc = 0x47909d; continue;
 }
 0x47909d => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x4790a5; continue; }
  pc = 0x4790a1; continue;
 }
 0x4790a1 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) & 0xfdu8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x4790a5; continue;
 }
 0x4790a5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x16u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x48u32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x479054; continue; }
  pc = 0x4790af; continue;
 }
 0x4790af => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004790b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4790b0); }
 let mut pc: u32 = 0x4790b0;
 loop { match pc {
 0x4790b0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x4790c8; continue; }
  pc = 0x4790b7; continue;
 }
 0x4790b7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4790c1); f_004793ab(cpu); if cpu.returned_to != 0x4790c1 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x4790c9; continue; }
  pc = 0x4790c8; continue;
 }
 0x4790c8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4790c9 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x4790d4; continue; }
  pc = 0x4790d0; continue;
 }
 0x4790d0 => {
  { let v: u8 = cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0xfbu8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x4790d4; continue;
 }
 0x4790d4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004790d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4790d5); }
 let mut pc: u32 = 0x4790d5;
 loop { match pc {
 0x4790d5 => {
  { let v: u32 = 0x5242ccu32; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x4790e1; continue;
 }
 0x4790e1 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.edi)), 0x0u16); }
  if cpu.zf { pc = 0x4790ed; continue; }
  pc = 0x4790e9; continue;
 }
 0x4790e9 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x16u32.wrapping_add(cpu.edi))); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x4790ed; continue;
 }
 0x4790ed => {
  { let v: u32 = cpu.add32(cpu.edi, 0x1eu32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4790e1; continue; }
  pc = 0x4790f2; continue;
 }
 0x4790f2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479237(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x479237); }
 let mut pc: u32 = 0x479237;
 loop { match pc {
 0x479237 => {
  { let v: u32 = cpu.r32(0x4d6a0cu32); cpu.ebp = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x479242; continue;
 }
 0x479242 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x47937a; continue; }
  pc = 0x47924d; continue;
 }
 0x47924d => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.sf == cpu.of) { pc = 0x47937a; continue; }
  pc = 0x479258; continue;
 }
 0x479258 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x47937a; continue; }
  pc = 0x479263; continue;
 }
 0x479263 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x47937a; continue; }
  pc = 0x47926e; continue;
 }
 0x47926e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x47937a; continue; }
  pc = 0x479279; continue;
 }
 0x479279 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x47937a; continue; }
  pc = 0x479284; continue;
 }
 0x479284 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x47937a; continue; }
  pc = 0x47928f; continue;
 }
 0x47928f => {
  cpu.push(0x479294); f_00478632(cpu); if cpu.returned_to != 0x479294 { return; }
  if cpu.cf { pc = 0x47937a; continue; }
  pc = 0x47929a; continue;
 }
 0x47929a => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x10u8); }
  if !cpu.zf { pc = 0x47937a; continue; }
  pc = 0x4792a4; continue;
 }
 0x4792a4 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4792b4); f_0047a2d9(cpu); if cpu.returned_to != 0x4792b4 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x47937a; continue; }
  pc = 0x4792bb; continue;
 }
 0x4792bb => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x8u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x47937a; continue; }
  pc = 0x4792ca; continue;
 }
 0x4792ca => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4792f5; continue; }
  pc = 0x4792d3; continue;
 }
 0x4792d3 => {
  { let v: u32 = 0x16u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x524b80u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x2u8); }
  if !cpu.zf { pc = 0x47937a; continue; }
  pc = 0x4792eb; continue;
 }
 0x4792eb => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x8u8); }
  if cpu.zf { pc = 0x47937a; continue; }
  pc = 0x4792f5; continue;
 }
 0x4792f5 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x47931f; continue; }
  pc = 0x4792fd; continue;
 }
 0x4792fd => {
  { let v: u32 = cpu.r8(0x524b6bu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x9u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0xcu32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0xau32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x47937a; continue; }
  pc = 0x47931f; continue;
 }
 0x47931f => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x479334); f_0047a2d9(cpu); if cpu.returned_to != 0x479334 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x47933c); f_004794fb(cpu); if cpu.returned_to != 0x47933c { return; }
  cpu.push(0x479341); f_004793ab(cpu); if cpu.returned_to != 0x479341 { return; }
  if cpu.cf { pc = 0x479347; continue; }
  pc = 0x479343; continue;
 }
 0x479343 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x479357; continue; }
  pc = 0x479347; continue;
 }
 0x479347 => {
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd4u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x479357); f_00479a1a(cpu); if cpu.returned_to != 0x479357 { return; }
  pc = 0x479357; continue;
 }
 0x479357 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x3bu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x47937a; continue; }
  pc = 0x479371; continue;
 }
 0x479371 => {
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  pc = 0x47937a; continue;
 }
 0x47937a => {
  { let v: u32 = 0xb8u32.wrapping_add(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x479242; continue; }
  pc = 0x479387; continue;
 }
 0x479387 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6a0cu32, v); }
  { let v: u32 = cpu.sub32(cpu.r32(0x4d6a10u32), 0x14u32); cpu.w32(0x4d6a10u32, v); }
  if !cpu.zf { pc = 0x4793aa; continue; }
  pc = 0x479396; continue;
 }
 0x479396 => {
  { let v: u32 = 0xf0u32; cpu.w32(0x4d6a10u32, v); }
  { let v: u32 = 0x509290u32; cpu.w32(0x4d6a0cu32, v); }
  pc = 0x4793aa; continue;
 }
 0x4793aa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004793ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4793ab); }
 let mut pc: u32 = 0x4793ab;
 loop { match pc {
 0x4793ab => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x479409; continue; }
  pc = 0x4793b8; continue;
 }
 0x4793b8 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4793d6; continue; }
  pc = 0x4793cd; continue;
 }
 0x4793cd => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x479409; continue; }
  pc = 0x4793d4; continue;
 }
 0x4793d4 => {
  pc = 0x4793e3; continue;
 }
 0x4793d6 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x479409; continue; }
  pc = 0x4793dd; continue;
 }
 0x4793dd => {
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x479409; continue; }
  pc = 0x4793e3; continue;
 }
 0x4793e3 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d39c6u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5d39c8u32, v); }
  cpu.push(0x4793fd); f_0047a2d9(cpu); if cpu.returned_to != 0x4793fd { return; }
  if cpu.cf { pc = 0x479409; continue; }
  pc = 0x4793ff; continue;
 }
 0x4793ff => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x479409 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479411(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x479411); }
 let mut pc: u32 = 0x479411;
 loop { match pc {
 0x479411 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4794f3; continue; }
  pc = 0x479421; continue;
 }
 0x479421 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x47949e; continue; }
  pc = 0x479434; continue;
 }
 0x479434 => {
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x4794f3; continue; }
  pc = 0x47943e; continue;
 }
 0x47943e => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.esi)), 0xd80u16); }
  if cpu.zf { pc = 0x479487; continue; }
  pc = 0x479446; continue;
 }
 0x479446 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x479470; continue; }
  pc = 0x47944f; continue;
 }
 0x47944f => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x4794f3; continue; }
  pc = 0x479470; continue;
 }
 0x479470 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x47948f; continue;
 }
 0x479487 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47948f; continue;
 }
 0x47948f => {
  cpu.push(0x479494); f_0047a2d9(cpu); if cpu.returned_to != 0x479494 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47949e => {
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x4794f3; continue; }
  pc = 0x4794a4; continue;
 }
 0x4794a4 => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.esi)), 0xfb2eu16); }
  if cpu.zf { pc = 0x4794be; continue; }
  pc = 0x4794ac; continue;
 }
 0x4794ac => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.esi)), 0xef1fu16); }
  if cpu.zf { pc = 0x4794eb; continue; }
  pc = 0x4794b4; continue;
 }
 0x4794b4 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x47948f; continue;
 }
 0x4794be => {
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x52bbf0u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x4794f3; continue; }
  pc = 0x4794ce; continue;
 }
 0x4794ce => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4794f3; continue; }
  pc = 0x4794d4; continue;
 }
 0x4794d4 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  pc = 0x4794dc; continue;
 }
 0x4794dc => {
  { let v: u32 = cpu.add32(cpu.eax, 0x509290u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x47948f; continue;
 }
 0x4794eb => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x4794dc; continue;
 }
 0x4794f3 => {
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004794fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4794fb); }
 let mut pc: u32 = 0x4794fb;
 loop { match pc {
 0x4794fb => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd4u32, v); }
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  pc = 0x479527; continue;
 }
 0x479527 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x8u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x479572; continue; }
  pc = 0x47952c; continue;
 }
 0x47952c => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r8(0x12u32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6934u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u16 = cpu.r8(0x13u32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6936u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.cf { pc = 0x479572; continue; }
  pc = 0x479566; continue;
 }
 0x479566 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x4d6bd4u32)); }
  if cpu.zf { pc = 0x479572; continue; }
  pc = 0x47956e; continue;
 }
 0x47956e => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x479572; continue;
 }
 0x479572 => {
  { let v: u32 = 0x1eu32.wrapping_add(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x479527; continue; }
  pc = 0x479577; continue;
 }
 0x479577 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047957f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47957f); }
 let mut pc: u32 = 0x47957f;
 loop { match pc {
 0x47957f => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x4d69d8u32), 0x0u16); }
  if cpu.zf { pc = 0x47965c; continue; }
  pc = 0x47958e; continue;
 }
 0x47958e => {
  { let a = cpu.esi; let b = cpu.ebp; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.ebp = v; } }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x47965c; continue; }
  pc = 0x47959b; continue;
 }
 0x47959b => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4795b1); f_0047a2d9(cpu); if cpu.returned_to != 0x4795b1 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd4u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  pc = 0x4795c9; continue;
 }
 0x4795c9 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.sf == cpu.of) { pc = 0x47964f; continue; }
  pc = 0x4795d0; continue;
 }
 0x4795d0 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x47964f; continue; }
  pc = 0x4795d7; continue;
 }
 0x4795d7 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.ebx as u8), ((cpu.ebx >> 8) as u8)); }
  if !cpu.zf { pc = 0x47964f; continue; }
  pc = 0x4795e2; continue;
 }
 0x4795e2 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x47964f; continue; }
  pc = 0x4795e9; continue;
 }
 0x4795e9 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x4d6bf4u32)); }
  if cpu.zf { pc = 0x47964f; continue; }
  pc = 0x4795f1; continue;
 }
 0x4795f1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x479604); f_0047a2d9(cpu); if cpu.returned_to != 0x479604 { return; }
  { cpu.sub32(cpu.esi, cpu.r32(0x4d6bd4u32)); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.zf { pc = 0x47964f; continue; }
  pc = 0x479610; continue;
 }
 0x479610 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x47964f; continue; }
  pc = 0x479617; continue;
 }
 0x479617 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x47964f; continue; }
  pc = 0x47961e; continue;
 }
 0x47961e => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x47964f; continue; }
  pc = 0x479625; continue;
 }
 0x479625 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x47964f; continue; }
  pc = 0x47962c; continue;
 }
 0x47962c => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x47964f; continue; }
  pc = 0x479633; continue;
 }
 0x479633 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x47964f; continue; }
  pc = 0x47963a; continue;
 }
 0x47963a => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x47964f; continue; }
  pc = 0x479641; continue;
 }
 0x479641 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x47964f; continue; }
  pc = 0x479648; continue;
 }
 0x479648 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x47964f); f_0047965e(cpu); if cpu.returned_to != 0x47964f { return; }
  pc = 0x47964f; continue;
 }
 0x47964f => {
  { let v: u32 = 0xb8u32.wrapping_add(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4795c9; continue; }
  pc = 0x47965c; continue;
 }
 0x47965c => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047965e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47965e); }
 let mut pc: u32 = 0x47965e;
 loop { match pc {
 0x47965e => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4796bd; continue; }
  pc = 0x47966b; continue;
 }
 0x47966b => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4796bd; continue; }
  pc = 0x479672; continue;
 }
 0x479672 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x479684; continue; }
  pc = 0x479678; continue;
 }
 0x479678 => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x479691; continue;
 }
 0x479684 => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x479691; continue;
 }
 0x479691 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b2u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4796b6); f_00413337(cpu); if cpu.returned_to != 0x4796b6 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4796ba => {
  {  }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4796bd => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { cpu.poll(); pc = 0x4796ba; continue; }
  pc = 0x4796c4; continue;
 }
 0x4796c4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b2u32, v); }
  { let v: u16 = 0xfb2eu16; cpu.w16(0x4d68aeu32, v); }
  cpu.push(0x4796d8); f_0044234d(cpu); if cpu.returned_to != 0x4796d8 { return; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b0u32, v); }
  cpu.push(0x4796f0); f_00413337(cpu); if cpu.returned_to != 0x4796f0 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004796f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4796f2); }
 let mut pc: u32 = 0x4796f2;
 loop { match pc {
 0x4796f2 => {
  { cpu.sub32(cpu.r32(0x5d31bcu32), 0x0u32); }
  if cpu.zf { pc = 0x479745; continue; }
  pc = 0x4796fb; continue;
 }
 0x4796fb => {
  { let v: u32 = 0x5d3438u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5d31bcu32); cpu.ecx = v; }
  pc = 0x479706; continue;
 }
 0x479706 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x479722); f_0047a2d9(cpu); if cpu.returned_to != 0x479722 { return; }
  if cpu.cf { pc = 0x479742; continue; }
  pc = 0x479724; continue;
 }
 0x479724 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x8u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x479742; continue; }
  pc = 0x47972f; continue;
 }
 0x47972f => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0xcu32.wrapping_add(cpu.esi))); }
  if cpu.cf { pc = 0x479742; continue; }
  pc = 0x479739; continue;
 }
 0x479739 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x479740); f_00479746(cpu); if cpu.returned_to != 0x479740 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x479742; continue;
 }
 0x479742 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x479706; continue; }
  pc = 0x479745; continue;
 }
 0x479745 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479746(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x479746); }
 let mut pc: u32 = 0x479746;
 loop { match pc {
 0x479746 => {
  cpu.push(0x47974b); f_00479c13(cpu); if cpu.returned_to != 0x47974b { return; }
  { let v: u32 = cpu.r32(0x5d39d4u32); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x479798; continue; }
  pc = 0x479755; continue;
 }
 0x479755 => {
  { let v: u32 = 0x5d31c8u32; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0xau32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x479767; continue;
 }
 0x479767 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x479788; continue; }
  pc = 0x479774; continue;
 }
 0x479774 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0xau32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x479788; continue; }
  pc = 0x47977d; continue;
 }
 0x47977d => {
  cpu.push(0x479782); f_0047991c(cpu); if cpu.returned_to != 0x479782 { return; }
  if cpu.cf { pc = 0x479788; continue; }
  pc = 0x479784; continue;
 }
 0x479784 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x479788; continue;
 }
 0x479788 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x479767; continue; }
  pc = 0x47978d; continue;
 }
 0x47978d => {
  { cpu.sub32(cpu.edx, cpu.esi); }
  if cpu.zf { pc = 0x479799; continue; }
  pc = 0x479791; continue;
 }
 0x479791 => {
  cpu.push(0x479796); f_00479823(cpu); if cpu.returned_to != 0x479796 { return; }
  if cpu.cf { pc = 0x479799; continue; }
  pc = 0x479798; continue;
 }
 0x479798 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x479799 => {
  cpu.push(0x47979e); f_004797a6(cpu); if cpu.returned_to != 0x47979e { return; }
  if !cpu.cf { cpu.poll(); pc = 0x479798; continue; }
  pc = 0x4797a0; continue;
 }
 0x4797a0 => {
  cpu.push(0x4797a5); f_00479864(cpu); if cpu.returned_to != 0x4797a5 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004797a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4797a6); }
 let mut pc: u32 = 0x4797a6;
 loop { match pc {
 0x4797a6 => {
  { cpu.pushad(); }
  { let v: u32 = 0x524b80u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0xau32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x4797c1; continue;
 }
 0x4797c1 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x47980c; continue; }
  pc = 0x4797c8; continue;
 }
 0x4797c8 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x47980c; continue; }
  pc = 0x4797ce; continue;
 }
 0x4797ce => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if cpu.zf { pc = 0x47980c; continue; }
  pc = 0x4797d8; continue;
 }
 0x4797d8 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x47980c; continue; }
  pc = 0x4797df; continue;
 }
 0x4797df => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x47980c; continue; }
  pc = 0x4797e6; continue;
 }
 0x4797e6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x9u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47980c; continue; }
  pc = 0x4797f2; continue;
 }
 0x4797f2 => {
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0xau32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x47980c; continue; }
  pc = 0x4797fc; continue;
 }
 0x4797fc => {
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47980c; continue; }
  pc = 0x479801; continue;
 }
 0x479801 => {
  cpu.push(0x479806); f_0047991c(cpu); if cpu.returned_to != 0x479806 { return; }
  if cpu.cf { pc = 0x47980c; continue; }
  pc = 0x479808; continue;
 }
 0x479808 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x47980c; continue;
 }
 0x47980c => {
  { let v: u32 = cpu.edi.wrapping_add(0x16u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4797c1; continue; }
  pc = 0x479811; continue;
 }
 0x479811 => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x479820; continue; }
  pc = 0x479816; continue;
 }
 0x479816 => {
  cpu.push(0x47981b); f_00479823(cpu); if cpu.returned_to != 0x47981b { return; }
  if cpu.cf { pc = 0x479820; continue; }
  pc = 0x47981d; continue;
 }
 0x47981d => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x479820 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479823(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x479823); }
 let mut pc: u32 = 0x479823;
 loop { match pc {
 0x479823 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x47983f; continue; }
  pc = 0x47983c; continue;
 }
 0x47983c => {
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  pc = 0x47983f; continue;
 }
 0x47983f => {
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x479861; continue; }
  pc = 0x479847; continue;
 }
 0x479847 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x47985e); f_00479a1a(cpu); if cpu.returned_to != 0x47985e { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x479861 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479864(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x479864); }
 let mut pc: u32 = 0x479864;
 loop { match pc {
 0x479864 => {
  { let v: u32 = 0x524b80u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x479870; continue;
 }
 0x479870 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x479886; continue; }
  pc = 0x479876; continue;
 }
 0x479876 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0x8u8); }
  if cpu.zf { pc = 0x479886; continue; }
  pc = 0x47987c; continue;
 }
 0x47987c => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0x10u8); }
  if !cpu.zf { pc = 0x479886; continue; }
  pc = 0x479882; continue;
 }
 0x479882 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if cpu.zf { pc = 0x47988e; continue; }
  pc = 0x479886; continue;
 }
 0x479886 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edi, 0x16u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x479870; continue; }
  pc = 0x47988d; continue;
 }
 0x47988d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47988e => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x479886; continue; }
  pc = 0x4798a0; continue;
 }
 0x4798a0 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0xffffffffu32); }
  if cpu.zf { pc = 0x4798d7; continue; }
  pc = 0x4798a9; continue;
 }
 0x4798a9 => {
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x4798d7; continue; }
  pc = 0x4798ae; continue;
 }
 0x4798ae => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x4798d4; continue; }
  pc = 0x4798ca; continue;
 }
 0x4798ca => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x4798d4); f_00479a1a(cpu); if cpu.returned_to != 0x4798d4 { return; }
  pc = 0x4798d4; continue;
 }
 0x4798d4 => {
  { cpu.popad(); }
  cpu.poll(); pc = 0x479886; continue;
 }
 0x4798d7 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.edx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x479886; continue; }
  pc = 0x4798ee; continue;
 }
 0x4798ee => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x479901); f_0047a2d9(cpu); if cpu.returned_to != 0x479901 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x479909); f_004794fb(cpu); if cpu.returned_to != 0x479909 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x479916); f_00479a1a(cpu); if cpu.returned_to != 0x479916 { return; }
  { cpu.popad(); }
  cpu.poll(); pc = 0x479886; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047991c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47991c); }
 let mut pc: u32 = 0x47991c;
 loop { match pc {
 0x47991c => {
  { cpu.pushad(); }
  { let v: u32 = 0x524b80u32; cpu.edx = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5d39d8u32, v); }
  pc = 0x479937; continue;
 }
 0x479937 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x479950; continue; }
  pc = 0x47993d; continue;
 }
 0x47993d => {
  { cpu.sub32(cpu.edi, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x479950; continue; }
  pc = 0x479941; continue;
 }
 0x479941 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x47994a); f_0047997c(cpu); if cpu.returned_to != 0x47994a { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if !cpu.cf { pc = 0x479965; continue; }
  pc = 0x479950; continue;
 }
 0x479950 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x16u32); cpu.edx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x479937; continue; }
  pc = 0x479957; continue;
 }
 0x479957 => {
  { cpu.popad(); }
  { cpu.sub32(cpu.r32(0x5d39d8u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x479963; continue; }
  pc = 0x479961; continue;
 }
 0x479961 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x479963 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x479965 => {
  { let v: u32 = cpu.r32(0x5d31b8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5d39d8u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x479950; continue; }
  pc = 0x479972; continue;
 }
 0x479972 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5d39d8u32, v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  cpu.poll(); pc = 0x479950; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047997c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47997c); }
 let mut pc: u32 = 0x47997c;
 loop { match pc {
 0x47997c => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x52bc03u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5d31b8u32, v); }
  pc = 0x479998; continue;
 }
 0x479998 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4799c4; continue; }
  pc = 0x47999d; continue;
 }
 0x47999d => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x4799c4; continue; }
  pc = 0x4799b1; continue;
 }
 0x4799b1 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4799ba); f_00485452(cpu); if cpu.returned_to != 0x4799ba { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5d31b8u32).wrapping_add(cpu.eax); cpu.w32(0x5d31b8u32, v); }
  pc = 0x4799c4; continue;
 }
 0x4799c4 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x479998; continue; }
  pc = 0x4799c7; continue;
 }
 0x4799c7 => {
  { cpu.sub32(cpu.r32(0x5d31b8u32), 0x0u32); }
  if cpu.zf { pc = 0x479a18; continue; }
  pc = 0x4799d0; continue;
 }
 0x4799d0 => {
  { cpu.sub32(cpu.r32(0x5d31b8u32), 0xffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4799e6; continue; }
  pc = 0x4799dc; continue;
 }
 0x4799dc => {
  { let v: u32 = 0xffffu32; cpu.w32(0x5d31b8u32, v); }
  pc = 0x4799e6; continue;
 }
 0x4799e6 => {
  { let v: u32 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x5d31b8u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x479a18; continue; }
  pc = 0x4799fb; continue;
 }
 0x4799fb => {
  { let v: u32 = cpu.r8(0x524b6bu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x9u32.wrapping_add(cpu.edi)) as u32; cpu.ecx = v; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x479a18; continue; }
  pc = 0x479a11; continue;
 }
 0x479a11 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x479a18; continue; }
  pc = 0x479a16; continue;
 }
 0x479a16 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x479a18 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479a1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x479a1a); }
 let mut pc: u32 = 0x479a1a;
 loop { match pc {
 0x479a1a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x479ab0; continue; }
  pc = 0x479a25; continue;
 }
 0x479a25 => {
  cpu.push(0x479a2a); f_00454b24(cpu); if cpu.returned_to != 0x479a2a { return; }
  if cpu.cf { pc = 0x479ab1; continue; }
  pc = 0x479a30; continue;
 }
 0x479a30 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = cpu.r8(0x12u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x13u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x479a6d; continue; }
  pc = 0x479a57; continue;
 }
 0x479a57 => {
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xdcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x479a6d; continue;
 }
 0x479a6d => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x479a88; continue; }
  pc = 0x479a73; continue;
 }
 0x479a73 => {
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x479a88; continue;
 }
 0x479a88 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  cpu.push(0x479a9a); f_00479ae5(cpu); if cpu.returned_to != 0x479a9a { return; }
  if cpu.cf { pc = 0x479ab0; continue; }
  pc = 0x479a9c; continue;
 }
 0x479a9c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { cpu.pushad(); }
  cpu.push(0x479aaf); f_00413337(cpu); if cpu.returned_to != 0x479aaf { return; }
  { cpu.popad(); }
  pc = 0x479ab0; continue;
 }
 0x479ab0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x479ab1 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d68b2u32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b0u32, v); }
  { cpu.pushad(); }
  cpu.push(0x479ae3); f_00413337(cpu); if cpu.returned_to != 0x479ae3 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479ae5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x479ae5); }
 let mut pc: u32 = 0x479ae5;
 loop { match pc {
 0x479ae5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x479aec); f_0041c743(cpu); if cpu.returned_to != 0x479aec { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x479af6; continue;
 }
 0x479af6 => {
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d39a6u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5d39a8u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x479b18); f_0041a689(cpu); if cpu.returned_to != 0x479b18 { return; }
  if cpu.cf { pc = 0x479b21; continue; }
  pc = 0x479b1a; continue;
 }
 0x479b1a => {
  cpu.push(0x479b1f); f_00479b2f(cpu); if cpu.returned_to != 0x479b1f { return; }
  if !cpu.cf { pc = 0x479b2b; continue; }
  pc = 0x479b21; continue;
 }
 0x479b21 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.and32(cpu.esi, 0x7u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x479af6; continue; }
  pc = 0x479b27; continue;
 }
 0x479b27 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x479b2b => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479b2f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x479b2f); }
 let mut pc: u32 = 0x479b2f;
 loop { match pc {
 0x479b2f => {
  { cpu.pushad(); }
  { let v: u32 = 0x52bbf0u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x524b80u32; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  pc = 0x479b4a; continue;
 }
 0x479b4a => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x479b98; continue; }
  pc = 0x479b51; continue;
 }
 0x479b51 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x479b98; continue; }
  pc = 0x479b57; continue;
 }
 0x479b57 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  cpu.push(0x479b6d); f_004793ab(cpu); if cpu.returned_to != 0x479b6d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x479b98; continue; }
  pc = 0x479b72; continue;
 }
 0x479b72 => {
  { let v: u16 = cpu.r16(0x5d39c6u32) & 0xfff0u16; cpu.w16(0x5d39c6u32, v); }
  { let v: u16 = cpu.r16(0x5d39c8u32) & 0xfff0u16; cpu.w16(0x5d39c8u32, v); }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x5d39c6u32)); }
  if !cpu.zf { pc = 0x479b98; continue; }
  pc = 0x479b8b; continue;
 }
 0x479b8b => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x5d39c8u32)); }
  if !cpu.zf { pc = 0x479b98; continue; }
  pc = 0x479b94; continue;
 }
 0x479b94 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x479b98 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x16u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x479b4a; continue; }
  pc = 0x479ba1; continue;
 }
 0x479ba1 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479ba4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x479ba4); }
 let mut pc: u32 = 0x479ba4;
 loop { match pc {
 0x479ba4 => {
  { cpu.pushad(); }
  { let v: u32 = 0x524b80u32; cpu.edi = v; }
  { let v: u32 = 0x52bbf0u32; cpu.edx = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x479bb4; continue;
 }
 0x479bb4 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x479c09; continue; }
  pc = 0x479bba; continue;
 }
 0x479bba => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) & 0xf7u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.edx)), 0xffu8); }
  if cpu.zf { pc = 0x479c04; continue; }
  pc = 0x479bcc; continue;
 }
 0x479bcc => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.edx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x479c04; continue; }
  pc = 0x479be2; continue;
 }
 0x479be2 => {
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x479c04; continue; }
  pc = 0x479be8; continue;
 }
 0x479be8 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x479bf3; continue; }
  pc = 0x479bef; continue;
 }
 0x479bef => {
  { let v: u8 = cpu.or8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0x8u8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x479bf3; continue;
 }
 0x479bf3 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x479c02); f_0047a2d9(cpu); if cpu.returned_to != 0x479c02 { return; }
  if cpu.cf { pc = 0x479c06; continue; }
  pc = 0x479c04; continue;
 }
 0x479c04 => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x479c06; continue;
 }
 0x479c06 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x479c09; continue;
 }
 0x479c09 => {
  { let v: u32 = cpu.edx.wrapping_add(0x48u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x16u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x479bb4; continue; }
  pc = 0x479c11; continue;
 }
 0x479c11 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479c13(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x479c13); } r_00479c13(cpu, 0x479c13); }
pub fn f_00479c4e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x479c4e); } r_00479c13(cpu, 0x479c4e); }
pub fn f_00479c4f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x479c4f); } r_00479c13(cpu, 0x479c4f); }
pub fn f_00479c50(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x479c50); } r_00479c13(cpu, 0x479c50); }
pub fn f_00479c51(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x479c51); } r_00479c13(cpu, 0x479c51); }
pub fn f_00479c52(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x479c52); } r_00479c13(cpu, 0x479c52); }
pub fn f_00479c53(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x479c53); } r_00479c13(cpu, 0x479c53); }
pub fn f_00479c54(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x479c54); } r_00479c13(cpu, 0x479c54); }
pub fn f_00479c55(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x479c55); } r_00479c13(cpu, 0x479c55); }
pub fn f_00479c56(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x479c56); } r_00479c13(cpu, 0x479c56); }
pub fn f_00479c58(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x479c58); } r_00479c13(cpu, 0x479c58); }
fn r_00479c13(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x479c13 => {
  { cpu.pushad(); }
  { let v: u32 = 0x0u32; cpu.w32(0x5d39d4u32, v); }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x479c99; continue; }
  pc = 0x479c23; continue;
 }
 0x479c23 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x479c99; continue; }
  pc = 0x479c2a; continue;
 }
 0x479c2a => {
  { let v: u32 = 0x5d31c8u32; cpu.edi = v; }
  { let v: u8 = 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5242ccu32; cpu.ebp = v; }
  pc = 0x479c36; continue;
 }
 0x479c36 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x479c92; continue; }
  pc = 0x479c3f; continue;
 }
 0x479c3f => {
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if cpu.zf { pc = 0x479c92; continue; }
  pc = 0x479c43; continue;
 }
 0x479c43 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x479cee; continue; }
  pc = 0x479c51; continue;
 }
 0x479c4e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x479c50; continue;
 }
 0x479c4f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x479c51; continue;
 }
 0x479c50 => {
  { let v: u8 = cpu.r8(0xffffff8bu32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffff8bu32.wrapping_add(cpu.esi), v); }
  pc = 0x479c53; continue;
 }
 0x479c51 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x479c55; continue;
 }
 0x479c52 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  pc = 0x479c55; continue;
 }
 0x479c53 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x479c54; continue;
 }
 0x479c54 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x66u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x479c56; continue;
 }
 0x479c55 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x479cee; continue; }
  pc = 0x479c60; continue;
 }
 0x479c56 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x479cee; continue; }
  pc = 0x479c60; continue;
 }
 0x479c58 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xfu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x66000000u32.wrapping_add(cpu.esi)), (cpu.ecx as u8)); }
  { cpu.sub32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x479cd6; continue; }
  pc = 0x479c67; continue;
 }
 0x479c60 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x479cd6; continue; }
  pc = 0x479c67; continue;
 }
 0x479c67 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x479cd6; continue; }
  pc = 0x479c6e; continue;
 }
 0x479c6e => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x479cbf; continue; }
  pc = 0x479c79; continue;
 }
 0x479c79 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x6u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x479cbf; continue; }
  pc = 0x479c84; continue;
 }
 0x479c84 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x479ca8; continue; }
  pc = 0x479c8b; continue;
 }
 0x479c8b => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x6u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x479ca8; continue; }
  pc = 0x479c92; continue;
 }
 0x479c92 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x1eu32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x479c36; continue; }
  pc = 0x479c99; continue;
 }
 0x479c99 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x479c9b => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x5d39d4u32)); cpu.w32(0x5d39d4u32, v); }
  cpu.poll(); pc = 0x479c92; continue;
 }
 0x479ca8 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x479c92; continue; }
  pc = 0x479cb3; continue;
 }
 0x479cb3 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x479c92; continue; }
  pc = 0x479cbd; continue;
 }
 0x479cbd => {
  cpu.poll(); pc = 0x479c9b; continue;
 }
 0x479cbf => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x479c9b; continue; }
  pc = 0x479cc9; continue;
 }
 0x479cc9 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x479c9b; continue; }
  pc = 0x479cd4; continue;
 }
 0x479cd4 => {
  cpu.poll(); pc = 0x479c92; continue;
 }
 0x479cd6 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x479c92; continue; }
  pc = 0x479ce1; continue;
 }
 0x479ce1 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x6u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x479c92; continue; }
  pc = 0x479cec; continue;
 }
 0x479cec => {
  cpu.poll(); pc = 0x479c9b; continue;
 }
 0x479cee => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x6u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x479c9b; continue; }
  pc = 0x479cf9; continue;
 }
 0x479cf9 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x479c9b; continue; }
  pc = 0x479d04; continue;
 }
 0x479d04 => {
  cpu.poll(); pc = 0x479c92; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479d06(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x479d06); }
 let mut pc: u32 = 0x479d06;
 loop { match pc {
 0x479d06 => {
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x5d627cu32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v = cpu.r16(0x58c83cu32); cpu.mul16(v); }
  { let v: u32 = 0x2710u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x58c884u32), 0x2u8); }
  if !cpu.zf { pc = 0x479d3e; continue; }
  pc = 0x479d35; continue;
 }
 0x479d35 => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x479d4a; continue; }
  pc = 0x479d3a; continue;
 }
 0x479d3a => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x479d4a; continue;
 }
 0x479d3e => {
  { cpu.sub8(cpu.r8(0x58c884u32), 0x0u8); }
  if !cpu.zf { pc = 0x479d4a; continue; }
  pc = 0x479d47; continue;
 }
 0x479d47 => {
  { let v: u16 = cpu.add16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x479d4a; continue;
 }
 0x479d4a => {
  { let v: u32 = cpu.edx; cpu.w32(0x5d39e0u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5d3d36u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479e22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x479e22); }
 let mut pc: u32 = 0x479e22;
 loop { match pc {
 0x479e22 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5b02e4u32, v); }
  cpu.push(0x479e31); f_004852ec(cpu); if cpu.returned_to != 0x479e31 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5d3d3eu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524b6cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67f0u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524b6du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67f2u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x524b6fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67f4u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = 0x12u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x2u16); }
  if cpu.zf { pc = 0x479e7b; continue; }
  pc = 0x479e6d; continue;
 }
 0x479e6d => {
  { let v: u8 = 0x16u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { pc = 0x479e7b; continue; }
  pc = 0x479e79; continue;
 }
 0x479e79 => {
  { let v: u8 = 0x17u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x479e7b; continue;
 }
 0x479e7b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x524b6eu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d680eu32, v); }
  cpu.push(0x479e8b); f_0044ed43(cpu); if cpu.returned_to != 0x479e8b { return; }
  cpu.push(0x479e90); f_00479f0a(cpu); if cpu.returned_to != 0x479e90 { return; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x479ec8; continue; }
  pc = 0x479e99; continue;
 }
 0x479e99 => {
  cpu.push(0x479e9e); f_00479d06(cpu); if cpu.returned_to != 0x479e9e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b99f6u32, v); }
  cpu.push(0x479eaa); f_00479fe8(cpu); if cpu.returned_to != 0x479eaa { return; }
  cpu.push(0x479eaf); f_00479fd2(cpu); if cpu.returned_to != 0x479eaf { return; }
  cpu.push(0x479eb4); f_0047a02a(cpu); if cpu.returned_to != 0x479eb4 { return; }
  cpu.push(0x479eb9); f_00477b65(cpu); if cpu.returned_to != 0x479eb9 { return; }
  cpu.push(0x479ebe); f_00479ec9(cpu); if cpu.returned_to != 0x479ebe { return; }
  cpu.push(0x479ec3); f_00485770(cpu); if cpu.returned_to != 0x479ec3 { return; }
  cpu.push(0x479ec8); f_00485841(cpu); if cpu.returned_to != 0x479ec8 { return; }
  pc = 0x479ec8; continue;
 }
 0x479ec8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479ec9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x479ec9); }
 let mut pc: u32 = 0x479ec9;
 loop { match pc {
 0x479ec9 => {
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x479ed3; continue;
 }
 0x479ed3 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x479f03; continue; }
  pc = 0x479eda; continue;
 }
 0x479eda => {
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.esi)) & 0xbfu8; cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0xfu8); }
  if !cpu.zf { pc = 0x479f03; continue; }
  pc = 0x479ee4; continue;
 }
 0x479ee4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  cpu.push(0x479efb); f_0047a31a(cpu); if cpu.returned_to != 0x479efb { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x479f03; continue; }
  pc = 0x479eff; continue;
 }
 0x479eff => {
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.esi)) | 0x40u8; cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x479f03; continue;
 }
 0x479f03 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x479ed3; continue; }
  pc = 0x479f09; continue;
 }
 0x479f09 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479f0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x479f0a); }
 let mut pc: u32 = 0x479f0a;
 loop { match pc {
 0x479f0a => {
  { let v: u32 = 0x5242ccu32; cpu.edi = v; }
  { let v: u8 = 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x479f11; continue;
 }
 0x479f11 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.edi)), 0xe0u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x479f2d); f_0047a2d9(cpu); if cpu.returned_to != 0x479f2d { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.cf { pc = 0x479f3f; continue; }
  pc = 0x479f31; continue;
 }
 0x479f31 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x479f3f; continue; }
  pc = 0x479f35; continue;
 }
 0x479f35 => {
  { let v: u8 = cpu.or8(cpu.r8(0x18u32.wrapping_add(cpu.edi)), 0x1u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5d3a04u32, v); }
  pc = 0x479f3f; continue;
 }
 0x479f3f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x479f53); f_0047a2d9(cpu); if cpu.returned_to != 0x479f53 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.cf { pc = 0x479f65; continue; }
  pc = 0x479f57; continue;
 }
 0x479f57 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x479f65; continue; }
  pc = 0x479f5b; continue;
 }
 0x479f5b => {
  { let v: u8 = cpu.or8(cpu.r8(0x18u32.wrapping_add(cpu.edi)), 0x2u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5d3a08u32, v); }
  pc = 0x479f65; continue;
 }
 0x479f65 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x479f79); f_0047a2d9(cpu); if cpu.returned_to != 0x479f79 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.cf { pc = 0x479f8b; continue; }
  pc = 0x479f7d; continue;
 }
 0x479f7d => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x479f8b; continue; }
  pc = 0x479f81; continue;
 }
 0x479f81 => {
  { let v: u8 = cpu.or8(cpu.r8(0x18u32.wrapping_add(cpu.edi)), 0x4u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5d3a08u32, v); }
  pc = 0x479f8b; continue;
 }
 0x479f8b => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x479f9f); f_0047a2d9(cpu); if cpu.returned_to != 0x479f9f { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.cf { pc = 0x479fb1; continue; }
  pc = 0x479fa3; continue;
 }
 0x479fa3 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x479fb1; continue; }
  pc = 0x479fa7; continue;
 }
 0x479fa7 => {
  { let v: u8 = cpu.or8(cpu.r8(0x18u32.wrapping_add(cpu.edi)), 0x8u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5d3a08u32, v); }
  pc = 0x479fb1; continue;
 }
 0x479fb1 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x479fbc); f_0047a0de(cpu); if cpu.returned_to != 0x479fbc { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if !cpu.cf { pc = 0x479fc4; continue; }
  pc = 0x479fc0; continue;
 }
 0x479fc0 => {
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.edi)) | 0x10u8; cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  pc = 0x479fc4; continue;
 }
 0x479fc4 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x1eu32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x479f11; continue; }
  pc = 0x479fd1; continue;
 }
 0x479fd1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479fd2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x479fd2); }
 let mut pc: u32 = 0x479fd2;
 loop { match pc {
 0x479fd2 => {
  { let v: u32 = 0x5242ccu32; cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x479fde; continue;
 }
 0x479fde => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1eu32.wrapping_add(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x479fde; continue; }
  pc = 0x479fe7; continue;
 }
 0x479fe7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00479fe8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x479fe8); } r_00479fe8(cpu, 0x479fe8); }
pub fn f_0047a000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47a000); } r_00479fe8(cpu, 0x47a000); }
fn r_00479fe8(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x479fe8 => {
  { let v: u32 = 0x4e1640u32; cpu.edi = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x524a5eu32; cpu.esi = v; }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  pc = 0x47a000; continue;
 }
 0x47a000 => {
  { let v: u32 = 0x4e1640u32; cpu.edi = v; }
  pc = 0x47a005; continue;
 }
 0x47a005 => {
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x47a00f; continue; }
  pc = 0x47a00d; continue;
 }
 0x47a00d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x47a00f; continue;
 }
 0x47a00f => {
  { cpu.stos(1, false); }
  { let v: u32 = cpu.add32(cpu.esi, 0xau32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47a005; continue; }
  pc = 0x47a015; continue;
 }
 0x47a015 => {
  { let v: u32 = 0x509290u32; cpu.w32(0x4d6a0cu32, v); }
  { let v: u32 = 0xf0u32; cpu.w32(0x4d6a10u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a02a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a02a); }
 let mut pc: u32 = 0x47a02a;
 loop { match pc {
 0x47a02a => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x524a5eu32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.sub32(cpu.edi, 0xau32); cpu.edi = v; }
  pc = 0x47a03b; continue;
 }
 0x47a03b => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x47a042); f_00478676(cpu); if cpu.returned_to != 0x47a042 { return; }
  if !cpu.cf { pc = 0x47a04a; continue; }
  pc = 0x47a044; continue;
 }
 0x47a044 => {
  { let v: u8 = 0x0u8; cpu.w8(0x15u32.wrapping_add(cpu.esi), v); }
  pc = 0x47a0aa; continue;
 }
 0x47a04a => {
  { let v: u8 = 0x1u8; cpu.w8(0x15u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x13u32.wrapping_add(cpu.esi), v); }
  pc = 0x47a056; continue;
 }
 0x47a056 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x1au8); }
  if (cpu.sf == cpu.of) { pc = 0x47a0aa; continue; }
  pc = 0x47a05b; continue;
 }
 0x47a05b => {
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0xau32); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x47a056; continue; }
  pc = 0x47a066; continue;
 }
 0x47a066 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x47a0aa; continue; }
  pc = 0x47a06c; continue;
 }
 0x47a06c => {
  { cpu.sub16(cpu.r16(0x4d69d8u32), 0x0u16); }
  if cpu.zf { pc = 0x47a0aa; continue; }
  pc = 0x47a076; continue;
 }
 0x47a076 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)) | 0x18u8; cpu.w8(0x10u32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x7u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x1eu32); cpu.eax = v; }
  { let v: u32 = 0x5242ccu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x47a09d); f_00478676(cpu); if cpu.returned_to != 0x47a09d { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x47a0a9; continue; }
  pc = 0x47a0a0; continue;
 }
 0x47a0a0 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x47a0a9); f_00479a1a(cpu); if cpu.returned_to != 0x47a0a9 { return; }
  pc = 0x47a0a9; continue;
 }
 0x47a0a9 => {
  { cpu.popad(); }
  pc = 0x47a0aa; continue;
 }
 0x47a0aa => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47a03b; continue; }
  pc = 0x47a0b1; continue;
 }
 0x47a0b1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a0b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a0b2); }
 let mut pc: u32 = 0x47a0b2;
 loop { match pc {
 0x47a0b2 => {
  cpu.push(0x47a0b7); f_00478632(cpu); if cpu.returned_to != 0x47a0b7 { return; }
  if cpu.cf { pc = 0x47a0dd; continue; }
  pc = 0x47a0b9; continue;
 }
 0x47a0b9 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x47a0ca); f_0047a2d9(cpu); if cpu.returned_to != 0x47a0ca { return; }
  if cpu.cf { pc = 0x47a0ce; continue; }
  pc = 0x47a0cc; continue;
 }
 0x47a0cc => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x47a0ce; continue;
 }
 0x47a0ce => {
  { let v: u8 = 0x8u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x47a0dd; continue;
 }
 0x47a0dd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a0de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a0de); }
 let mut pc: u32 = 0x47a0de;
 loop { match pc {
 0x47a0de => {
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x5d385cu32, v); }
  pc = 0x47a0ee; continue;
 }
 0x47a0ee => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x47a13a; continue; }
  pc = 0x47a0f6; continue;
 }
 0x47a0f6 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x47a13a; continue; }
  pc = 0x47a0ff; continue;
 }
 0x47a0ff => {
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x47a13a; continue; }
  pc = 0x47a106; continue;
 }
 0x47a106 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x47a119); f_0047a2d9(cpu); if cpu.returned_to != 0x47a119 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x47a13a; continue; }
  pc = 0x47a121; continue;
 }
 0x47a121 => {
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.zf { pc = 0x47a13a; continue; }
  pc = 0x47a125; continue;
 }
 0x47a125 => {
  { let v: u8 = cpu.r8(0x5d385cu32) | 0x1u8; cpu.w8(0x5d385cu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x5d385du32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5d31c4u32, v); }
  pc = 0x47a143; continue;
 }
 0x47a13a => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47a0ee; continue; }
  pc = 0x47a143; continue;
 }
 0x47a143 => {
  { let v: u32 = 0x559754u32; cpu.ebp = v; }
  { let v: u8 = 0xc8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x47a14c; continue;
 }
 0x47a14c => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47a1f8; continue; }
  pc = 0x47a15a; continue;
 }
 0x47a15a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x47a1f8; continue; }
  pc = 0x47a166; continue;
 }
 0x47a166 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4b839cu32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xc8u32); }
  if cpu.zf { pc = 0x47a1c9; continue; }
  pc = 0x47a174; continue;
 }
 0x47a174 => {
  { cpu.sub32(cpu.ebx, 0x271u32); }
  if cpu.zf { pc = 0x47a1c9; continue; }
  pc = 0x47a17c; continue;
 }
 0x47a17c => {
  { cpu.sub32(cpu.ebx, 0x2d5u32); }
  if cpu.zf { pc = 0x47a1c9; continue; }
  pc = 0x47a184; continue;
 }
 0x47a184 => {
  { cpu.sub32(cpu.ebx, 0x307u32); }
  if cpu.zf { pc = 0x47a1c9; continue; }
  pc = 0x47a18c; continue;
 }
 0x47a18c => {
  { cpu.sub32(cpu.ebx, 0x20du32); }
  if cpu.zf { pc = 0x47a1c9; continue; }
  pc = 0x47a194; continue;
 }
 0x47a194 => {
  { cpu.sub32(cpu.ebx, 0x226u32); }
  if cpu.zf { pc = 0x47a1c9; continue; }
  pc = 0x47a19c; continue;
 }
 0x47a19c => {
  { cpu.sub32(cpu.ebx, 0x23fu32); }
  if cpu.zf { pc = 0x47a1c9; continue; }
  pc = 0x47a1a4; continue;
 }
 0x47a1a4 => {
  { cpu.sub32(cpu.ebx, 0x258u32); }
  if cpu.zf { pc = 0x47a1c9; continue; }
  pc = 0x47a1ac; continue;
 }
 0x47a1ac => {
  { cpu.sub32(cpu.ebx, 0x339u32); }
  if cpu.zf { pc = 0x47a1c9; continue; }
  pc = 0x47a1b4; continue;
 }
 0x47a1b4 => {
  { cpu.sub32(cpu.ebx, 0x36bu32); }
  if cpu.zf { pc = 0x47a1c9; continue; }
  pc = 0x47a1bc; continue;
 }
 0x47a1bc => {
  { cpu.sub32(cpu.ebx, 0x39du32); }
  if cpu.zf { pc = 0x47a1c9; continue; }
  pc = 0x47a1c4; continue;
 }
 0x47a1c4 => {
  { cpu.sub32(cpu.ebx, 0x7du32); }
  if !cpu.zf { pc = 0x47a1f8; continue; }
  pc = 0x47a1c9; continue;
 }
 0x47a1c9 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x47a1d7); f_0047a2d9(cpu); if cpu.returned_to != 0x47a1d7 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x47a1f8; continue; }
  pc = 0x47a1df; continue;
 }
 0x47a1df => {
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.zf { pc = 0x47a1f8; continue; }
  pc = 0x47a1e3; continue;
 }
 0x47a1e3 => {
  { let v: u8 = cpu.r8(0x5d385cu32) | 0x2u8; cpu.w8(0x5d385cu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x5d385du32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5d31c4u32, v); }
  pc = 0x47a205; continue;
 }
 0x47a1f8 => {
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp.wrapping_add(0x28u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47a14c; continue; }
  pc = 0x47a205; continue;
 }
 0x47a205 => {
  { let v: u8 = cpu.r8(0x5d385cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5d385du32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x5d31c4u32); cpu.ebp = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x47a21c; continue; }
  pc = 0x47a21a; continue;
 }
 0x47a21a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47a21c => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a21e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a21e); }
 let mut pc: u32 = 0x47a21e;
 loop { match pc {
 0x47a21e => {
  { let v: u32 = 0x52bbf0u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x524b80u32; cpu.edx = v; }
  pc = 0x47a22d; continue;
 }
 0x47a22d => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x47a266; continue; }
  pc = 0x47a233; continue;
 }
 0x47a233 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.edi)), 0xffu8); }
  if cpu.zf { pc = 0x47a266; continue; }
  pc = 0x47a239; continue;
 }
 0x47a239 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x47a259); f_0047a2d9(cpu); if cpu.returned_to != 0x47a259 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if !cpu.cf { pc = 0x47a263; continue; }
  pc = 0x47a25e; continue;
 }
 0x47a25e => {
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  pc = 0x47a263; continue;
 }
 0x47a263 => {
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x47a266; continue;
 }
 0x47a266 => {
  { let v: u32 = cpu.add32(cpu.edx, 0x16u32); cpu.edx = v; }
  { let v: u32 = 0x48u32.wrapping_add(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47a22d; continue; }
  pc = 0x47a26e; continue;
 }
 0x47a26e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a26f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a26f); }
 let mut pc: u32 = 0x47a26f;
 loop { match pc {
 0x47a26f => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x47a279; continue;
 }
 0x47a279 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)) & 0xfeu8; cpu.w8(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47a279; continue; }
  pc = 0x47a282; continue;
 }
 0x47a282 => {
  { let v: u8 = 0xffu8; cpu.w8(0x5d3438u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5d31bcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a294(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a294); }
 let mut pc: u32 = 0x47a294;
 loop { match pc {
 0x47a294 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x47a2d8; continue; }
  pc = 0x47a29b; continue;
 }
 0x47a29b => {
  { cpu.pushad(); }
  cpu.push(0x47a2a1); f_00478632(cpu); if cpu.returned_to != 0x47a2a1 { return; }
  if cpu.cf { pc = 0x47a2d7; continue; }
  pc = 0x47a2a3; continue;
 }
 0x47a2a3 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x10u8); }
  if !cpu.zf { pc = 0x47a2d7; continue; }
  pc = 0x47a2a9; continue;
 }
 0x47a2a9 => {
  { let v: u32 = 0x5d3438u32; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0xffu8); }
  if cpu.zf { pc = 0x47a2c5; continue; }
  pc = 0x47a2b3; continue;
 }
 0x47a2b3 => {
  { let v: u32 = cpu.r32(0x5d31bcu32); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x47a2c5; continue; }
  pc = 0x47a2bd; continue;
 }
 0x47a2bd => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x47a2d7; continue; }
  pc = 0x47a2c5; continue;
 }
 0x47a2c5 => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5d31bcu32)); cpu.edi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5d31bcu32)); cpu.w32(0x5d31bcu32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x47a2d7; continue;
 }
 0x47a2d7 => {
  { cpu.popad(); }
  pc = 0x47a2d8; continue;
 }
 0x47a2d8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a2d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a2d9); }
 let mut pc: u32 = 0x47a2d9;
 loop { match pc {
 0x47a2d9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u8 = 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x47a2eb; continue;
 }
 0x47a2eb => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if !cpu.zf { pc = 0x47a2fe; continue; }
  pc = 0x47a2f2; continue;
 }
 0x47a2f2 => {
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47a2eb; continue; }
  pc = 0x47a2f9; continue;
 }
 0x47a2f9 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47a2fe => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.cf { cpu.poll(); pc = 0x47a2f2; continue; }
  pc = 0x47a303; continue;
 }
 0x47a303 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4u32.wrapping_add(cpu.esi))); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x47a2f2; continue; }
  pc = 0x47a309; continue;
 }
 0x47a309 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if cpu.cf { cpu.poll(); pc = 0x47a2f2; continue; }
  pc = 0x47a30f; continue;
 }
 0x47a30f => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x6u32.wrapping_add(cpu.esi))); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x47a2f2; continue; }
  pc = 0x47a315; continue;
 }
 0x47a315 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a31a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a31a); }
 let mut pc: u32 = 0x47a31a;
 loop { match pc {
 0x47a31a => {
  { let v: u32 = cpu.esp; cpu.w32(0x5d31c0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  cpu.push(0x47a330); f_00420989(cpu); if cpu.returned_to != 0x47a330 { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x400u32; cpu.ebx = v; }
  cpu.push(0x47a33b); f_0041b922(cpu); if cpu.returned_to != 0x47a33b { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x47a439; continue; }
  pc = 0x47a342; continue;
 }
 0x47a342 => {
  { let v: u32 = cpu.r32(0x4d6ac8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x1000u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.esi.wrapping_sub(0x4d954cu32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x47a35f; continue;
 }
 0x47a35f => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x47a42c; continue; }
  pc = 0x47a367; continue;
 }
 0x47a367 => {
  { cpu.sub32(cpu.ecx, 0x3e8u32); }
  if !cpu.cf { pc = 0x47a439; continue; }
  pc = 0x47a373; continue;
 }
 0x47a373 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0x100u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47a439; continue; }
  pc = 0x47a381; continue;
 }
 0x47a381 => {
  { cpu.sub32(cpu.esi, 0x3f00u32); }
  if !cpu.cf { pc = 0x47a439; continue; }
  pc = 0x47a38d; continue;
 }
 0x47a38d => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x4dd54cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if !cpu.zf { pc = 0x47a3a8; continue; }
  pc = 0x47a39d; continue;
 }
 0x47a39d => {
  cpu.push(0x47a3a2); f_0047a446(cpu); if cpu.returned_to != 0x47a3a2 { return; }
  if cpu.cf { pc = 0x47a439; continue; }
  pc = 0x47a3a8; continue;
 }
 0x47a3a8 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x0u8); }
  if !cpu.zf { pc = 0x47a3c5; continue; }
  pc = 0x47a3af; continue;
 }
 0x47a3af => {
  { let v: u8 = cpu.r8(0x4dd54du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e9091u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x47a3c5; continue; }
  pc = 0x47a3c0; continue;
 }
 0x47a3c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x47a3c5; continue;
 }
 0x47a3c5 => {
  { cpu.sub8(cpu.r8(0xffffff80u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x0u8); }
  if !cpu.zf { pc = 0x47a3e6; continue; }
  pc = 0x47a3cc; continue;
 }
 0x47a3cc => {
  { let v: u8 = cpu.r8(0x4dd4ccu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e9091u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x47a3e6; continue; }
  pc = 0x47a3dd; continue;
 }
 0x47a3dd => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)).wrapping_sub(0x80u32); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x47a3e6; continue;
 }
 0x47a3e6 => {
  { cpu.sub8(cpu.r8(0x80u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x0u8); }
  if !cpu.zf { pc = 0x47a40a; continue; }
  pc = 0x47a3f0; continue;
 }
 0x47a3f0 => {
  { let v: u8 = cpu.r8(0x4dd5ccu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e9091u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x47a40a; continue; }
  pc = 0x47a401; continue;
 }
 0x47a401 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)).wrapping_add(0x80u32); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x47a40a; continue;
 }
 0x47a40a => {
  { cpu.sub8(cpu.r8(0xffffffffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x0u8); }
  if !cpu.zf { pc = 0x47a427; continue; }
  pc = 0x47a411; continue;
 }
 0x47a411 => {
  { let v: u8 = cpu.r8(0x4dd54bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e9091u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x47a427; continue; }
  pc = 0x47a422; continue;
 }
 0x47a422 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x47a427; continue;
 }
 0x47a427 => {
  cpu.poll(); pc = 0x47a35f; continue;
 }
 0x47a42c => {
  cpu.push(0x47a431); f_0041b99b(cpu); if cpu.returned_to != 0x47a431 { return; }
  { let v: u32 = cpu.r32(0x5d31c0u32); cpu.esp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47a439 => {
  cpu.push(0x47a43e); f_0041b99b(cpu); if cpu.returned_to != 0x47a43e { return; }
  { let v: u32 = cpu.r32(0x5d31c0u32); cpu.esp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a446(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a446); }
 let mut pc: u32 = 0x47a446;
 loop { match pc {
 0x47a446 => {
  { cpu.pushad(); }
  { let v: u32 = 0x4dd54cu32.wrapping_add(cpu.esi); cpu.eax = v; }
  cpu.push(0x47a452); f_0042c9e5(cpu); if cpu.returned_to != 0x47a452 { return; }
  cpu.push(0x47a457); f_0047a2d9(cpu); if cpu.returned_to != 0x47a457 { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.ebx = v; }
  cpu.push(0x47a468); f_0047a2d9(cpu); if cpu.returned_to != 0x47a468 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x47a470; continue; }
  pc = 0x47a46e; continue;
 }
 0x47a46e => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47a470 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a472(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a472); }
 let mut pc: u32 = 0x47a472;
 loop { match pc {
 0x47a472 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x47a4ba; continue; }
  pc = 0x47a47c; continue;
 }
 0x47a47c => {
  { let v: u32 = 0x16u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { cpu.sub8(cpu.r8(0x524b95u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x47a4ba; continue; }
  pc = 0x47a48e; continue;
 }
 0x47a48e => {
  { cpu.and8(cpu.r8(0x524b90u32.wrapping_add(cpu.eax)), 0x8u8); }
  if cpu.zf { pc = 0x47a4ba; continue; }
  pc = 0x47a497; continue;
 }
 0x47a497 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x47a4a6); f_0047a2d9(cpu); if cpu.returned_to != 0x47a4a6 { return; }
  if cpu.cf { pc = 0x47a4ba; continue; }
  pc = 0x47a4a8; continue;
 }
 0x47a4a8 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x47a4ba; continue; }
  pc = 0x47a4ae; continue;
 }
 0x47a4ae => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x47a4bd; continue; }
  pc = 0x47a4ba; continue;
 }
 0x47a4ba => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47a4bd => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a4c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a4c0); }
 let mut pc: u32 = 0x47a4c0;
 loop { match pc {
 0x47a4c0 => {
  { cpu.sub8(cpu.r8(0x5d3860u32), 0x0u8); }
  if !cpu.zf { pc = 0x47a4d0; continue; }
  pc = 0x47a4c9; continue;
 }
 0x47a4c9 => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x47a4e0; continue; }
  pc = 0x47a4d0; continue;
 }
 0x47a4d0 => {
  { let v: u8 = cpu.or8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x2u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x49u32; cpu.ebx = v; }
  cpu.push(0x47a4df); f_0047a938(cpu); if cpu.returned_to != 0x47a4df { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47a4e0 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x47a4ea); f_00454fa7(cpu); if cpu.returned_to != 0x47a4ea { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0x1f4u32); }
  if cpu.cf { cpu.poll(); pc = 0x47a4d0; continue; }
  pc = 0x47a4f2; continue;
 }
 0x47a4f2 => {
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x47a4fa); f_0044e7d7(cpu); if cpu.returned_to != 0x47a4fa { return; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  { cpu.popad(); }
  if !cpu.zf { cpu.poll(); pc = 0x47a4d0; continue; }
  pc = 0x47a4ff; continue;
 }
 0x47a4ff => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x47a50b); f_0044e818(cpu); if cpu.returned_to != 0x47a50b { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x47a4d0; continue; }
  pc = 0x47a510; continue;
 }
 0x47a510 => {
  cpu.push(0x47a515); f_0044e7a2(cpu); if cpu.returned_to != 0x47a515 { return; }
  if cpu.cf { cpu.poll(); pc = 0x47a4d0; continue; }
  pc = 0x47a517; continue;
 }
 0x47a517 => {
  { let v: u32 = 0x49u32; cpu.ebx = v; }
  cpu.push(0x47a521); f_0047a911(cpu); if cpu.returned_to != 0x47a521 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a522(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a522); }
 let mut pc: u32 = 0x47a522;
 loop { match pc {
 0x47a522 => {
  { cpu.sub8(cpu.r8(0x5d3860u32), 0x0u8); }
  if !cpu.zf { pc = 0x47a532; continue; }
  pc = 0x47a52b; continue;
 }
 0x47a52b => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x47a542; continue; }
  pc = 0x47a532; continue;
 }
 0x47a532 => {
  { let v: u8 = cpu.or8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x2u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4au32; cpu.ebx = v; }
  cpu.push(0x47a541); f_0047a95f(cpu); if cpu.returned_to != 0x47a541 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47a542 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x3e8u32); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x47a532; continue; }
  pc = 0x47a54c; continue;
 }
 0x47a54c => {
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x47a554); f_0044e7d7(cpu); if cpu.returned_to != 0x47a554 { return; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  { cpu.popad(); }
  if !cpu.zf { cpu.poll(); pc = 0x47a532; continue; }
  pc = 0x47a559; continue;
 }
 0x47a559 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x47a563); f_00454fa7(cpu); if cpu.returned_to != 0x47a563 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0x1f4u32); }
  if cpu.cf { cpu.poll(); pc = 0x47a532; continue; }
  pc = 0x47a56b; continue;
 }
 0x47a56b => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x47a577); f_0044e818(cpu); if cpu.returned_to != 0x47a577 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x47a532; continue; }
  pc = 0x47a57c; continue;
 }
 0x47a57c => {
  cpu.push(0x47a581); f_0044e7a2(cpu); if cpu.returned_to != 0x47a581 { return; }
  if cpu.cf { cpu.poll(); pc = 0x47a532; continue; }
  pc = 0x47a583; continue;
 }
 0x47a583 => {
  { let v: u32 = 0x4au32; cpu.ebx = v; }
  cpu.push(0x47a58d); f_0047a911(cpu); if cpu.returned_to != 0x47a58d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a58e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a58e); }
 let mut pc: u32 = 0x47a58e;
 loop { match pc {
 0x47a58e => {
  { cpu.sub8(cpu.r8(0x5d3860u32), 0x0u8); }
  if !cpu.zf { pc = 0x47a59e; continue; }
  pc = 0x47a597; continue;
 }
 0x47a597 => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x47a5ae; continue; }
  pc = 0x47a59e; continue;
 }
 0x47a59e => {
  { let v: u8 = cpu.or8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x2u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  cpu.push(0x47a5ad); f_0047a986(cpu); if cpu.returned_to != 0x47a5ad { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47a5ae => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x3e8u32); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x47a59e; continue; }
  pc = 0x47a5b8; continue;
 }
 0x47a5b8 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x47a5c2); f_00454fa7(cpu); if cpu.returned_to != 0x47a5c2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0x1f4u32); }
  if cpu.cf { cpu.poll(); pc = 0x47a59e; continue; }
  pc = 0x47a5ca; continue;
 }
 0x47a5ca => {
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x47a5d2); f_0044e7d7(cpu); if cpu.returned_to != 0x47a5d2 { return; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  { cpu.popad(); }
  if !cpu.zf { cpu.poll(); pc = 0x47a59e; continue; }
  pc = 0x47a5d7; continue;
 }
 0x47a5d7 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x47a5e3); f_0044e818(cpu); if cpu.returned_to != 0x47a5e3 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x47a59e; continue; }
  pc = 0x47a5e8; continue;
 }
 0x47a5e8 => {
  cpu.push(0x47a5ed); f_0044e7a2(cpu); if cpu.returned_to != 0x47a5ed { return; }
  if cpu.cf { cpu.poll(); pc = 0x47a59e; continue; }
  pc = 0x47a5ef; continue;
 }
 0x47a5ef => {
  { let v: u32 = 0x4au32; cpu.ebx = v; }
  cpu.push(0x47a5f9); f_0047a911(cpu); if cpu.returned_to != 0x47a5f9 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a5fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a5fa); }
 let mut pc: u32 = 0x47a5fa;
 loop { match pc {
 0x47a5fa => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d6bd8u32, v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.edi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6be4u32, v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.edi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x47a6c6; continue; }
  pc = 0x47a631; continue;
 }
 0x47a631 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x47a63b; continue;
 }
 0x47a63b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x56cf0au32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x41u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x56d162u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x41u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x5b3908u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b390au32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6792u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6794u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0xcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x47a695); f_0044fb8f(cpu); if cpu.returned_to != 0x47a695 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x47a6b6; continue; }
  pc = 0x47a69c; continue;
 }
 0x47a69c => {
  { let v: u32 = cpu.edi; cpu.w32(0x5b3598u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5d3986u32, v); }
  cpu.push(0x47a6ae); f_0047a7ac(cpu); if cpu.returned_to != 0x47a6ae { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47a6b6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x6u32); }
  if !cpu.zf { cpu.poll(); pc = 0x47a63b; continue; }
  pc = 0x47a6c1; continue;
 }
 0x47a6c1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x47a6c6; continue;
 }
 0x47a6c6 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x5d39cau32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x5d39ccu32, v); }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5d627cu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5d39ceu32, v); }
  { let v: u8 = 0x2du8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x47a6ee); f_0047a861(cpu); if cpu.returned_to != 0x47a6ee { return; }
  if cpu.cf { pc = 0x47a714; continue; }
  pc = 0x47a6f0; continue;
 }
 0x47a6f0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d39cau32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5d39ccu32, v); }
  cpu.push(0x47a702); f_0041c743(cpu); if cpu.returned_to != 0x47a702 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x5d39ceu32)); }
  if (cpu.sf == cpu.of) { pc = 0x47a714; continue; }
  pc = 0x47a70f; continue;
 }
 0x47a70f => {
  pc = 0x47a79f; continue;
 }
 0x47a714 => {
  { let v: u8 = 0x35u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x47a71b); f_0047a861(cpu); if cpu.returned_to != 0x47a71b { return; }
  if cpu.cf { pc = 0x47a73e; continue; }
  pc = 0x47a71d; continue;
 }
 0x47a71d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d39cau32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5d39ccu32, v); }
  cpu.push(0x47a72f); f_0041c743(cpu); if cpu.returned_to != 0x47a72f { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x5d39ceu32)); }
  if (cpu.sf == cpu.of) { pc = 0x47a73e; continue; }
  pc = 0x47a73c; continue;
 }
 0x47a73c => {
  pc = 0x47a79f; continue;
 }
 0x47a73e => {
  { let v: u8 = 0x36u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x47a745); f_0047a861(cpu); if cpu.returned_to != 0x47a745 { return; }
  if cpu.cf { pc = 0x47a768; continue; }
  pc = 0x47a747; continue;
 }
 0x47a747 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d39cau32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5d39ccu32, v); }
  cpu.push(0x47a759); f_0041c743(cpu); if cpu.returned_to != 0x47a759 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x5d39ceu32)); }
  if (cpu.sf == cpu.of) { pc = 0x47a768; continue; }
  pc = 0x47a766; continue;
 }
 0x47a766 => {
  pc = 0x47a79f; continue;
 }
 0x47a768 => {
  { let v: u8 = 0x37u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x47a76f); f_0047a861(cpu); if cpu.returned_to != 0x47a76f { return; }
  if cpu.cf { pc = 0x47a792; continue; }
  pc = 0x47a771; continue;
 }
 0x47a771 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d39cau32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5d39ccu32, v); }
  cpu.push(0x47a783); f_0041c743(cpu); if cpu.returned_to != 0x47a783 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x5d39ceu32)); }
  if (cpu.sf == cpu.of) { pc = 0x47a792; continue; }
  pc = 0x47a790; continue;
 }
 0x47a790 => {
  pc = 0x47a79f; continue;
 }
 0x47a792 => {
  { cpu.sub16(cpu.r16(0x5d39cau32), 0xffffu16); }
  if !cpu.zf { pc = 0x47a79f; continue; }
  pc = 0x47a79d; continue;
 }
 0x47a79d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47a79f => {
  { let v: u16 = cpu.r16(0x5d39cau32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d39ccu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  f_0047a7ac(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a7ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a7ac); }
 let mut pc: u32 = 0x47a7ac;
 loop { match pc {
 0x47a7ac => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x4e1fccu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x52u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0xcu32); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x47a802; continue; }
  pc = 0x47a7fc; continue;
 }
 0x47a7fc => {
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  pc = 0x47a802; continue;
 }
 0x47a802 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d69e2u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d69e4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1e47u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1fccu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x4e16b8u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1fcau32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5928d1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1e46u32, v); }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.push(0x47a83f); f_00449b96(cpu); if cpu.returned_to != 0x47a83f { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5b2754u32); cpu.ebp = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x81u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5928d1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fcau32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a861(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a861); }
 let mut pc: u32 = 0x47a861;
 loop { match pc {
 0x47a861 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5d3987u32, v); }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.edi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6be4u32, v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.edi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  pc = 0x47a88d; continue;
 }
 0x47a88d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x47a894); f_00420989(cpu); if cpu.returned_to != 0x47a894 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x5d3987u32)); }
  if !cpu.zf { pc = 0x47a8cd; continue; }
  pc = 0x47a8a7; continue;
 }
 0x47a8a7 => {
  { let v: u32 = cpu.edi; cpu.w32(0x5b3598u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5d3986u32, v); }
  { cpu.pushad(); }
  cpu.push(0x47a8ba); f_0044f988(cpu); if cpu.returned_to != 0x47a8ba { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x5d3986u32, v); }
  if cpu.cf { pc = 0x47a8cd; continue; }
  pc = 0x47a8c4; continue;
 }
 0x47a8c4 => {
  { let v: u8 = cpu.and8(cpu.r8(0x4000u32.wrapping_add(cpu.esi)), 0x80u8); cpu.w8(0x4000u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47a8cd => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6be0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x47a88d; continue; }
  pc = 0x47a8d8; continue;
 }
 0x47a8d8 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4d6be4u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x47a88d; continue; }
  pc = 0x47a8e9; continue;
 }
 0x47a8e9 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a8eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a8eb); }
 let mut pc: u32 = 0x47a8eb;
 loop { match pc {
 0x47a8eb => {
  { let v: u32 = cpu.r32(0x4b94d4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4b94d8u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x47a90c; continue; }
  pc = 0x47a909; continue;
 }
 0x47a909 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x47a90c; continue;
 }
 0x47a90c => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a911(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a911); }
 let mut pc: u32 = 0x47a911;
 loop { match pc {
 0x47a911 => {
  { cpu.pushad(); }
  cpu.push(0x47a917); f_00485770(cpu); if cpu.returned_to != 0x47a917 { return; }
  cpu.push(0x47a91c); f_0048cbe8(cpu); if cpu.returned_to != 0x47a91c { return; }
  { cpu.popad(); }
  { let v: u32 = 0x5d39a1u32; cpu.esi = v; }
  cpu.push(0x47a927); f_0047a9b2(cpu); if cpu.returned_to != 0x47a927 { return; }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffu32); cpu.ecx = v; }
  { let v: u32 = 0x5d3d43u32; cpu.esi = v; }
  cpu.push(0x47a937); f_0047a8eb(cpu); if cpu.returned_to != 0x47a937 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a938(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a938); }
 let mut pc: u32 = 0x47a938;
 loop { match pc {
 0x47a938 => {
  { cpu.pushad(); }
  cpu.push(0x47a93e); f_00485770(cpu); if cpu.returned_to != 0x47a93e { return; }
  cpu.push(0x47a943); f_0048cbe8(cpu); if cpu.returned_to != 0x47a943 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x5d398fu32; cpu.esi = v; }
  cpu.push(0x47a94e); f_0047a9b2(cpu); if cpu.returned_to != 0x47a94e { return; }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffu32); cpu.ecx = v; }
  { let v: u32 = 0x5d3d43u32; cpu.esi = v; }
  cpu.push(0x47a95e); f_0047a8eb(cpu); if cpu.returned_to != 0x47a95e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a95f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a95f); }
 let mut pc: u32 = 0x47a95f;
 loop { match pc {
 0x47a95f => {
  { cpu.pushad(); }
  cpu.push(0x47a965); f_00485770(cpu); if cpu.returned_to != 0x47a965 { return; }
  cpu.push(0x47a96a); f_0048cbe8(cpu); if cpu.returned_to != 0x47a96a { return; }
  { cpu.popad(); }
  { let v: u32 = 0x5d3988u32; cpu.esi = v; }
  cpu.push(0x47a975); f_0047a9b2(cpu); if cpu.returned_to != 0x47a975 { return; }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffu32); cpu.ecx = v; }
  { let v: u32 = 0x5d3d43u32; cpu.esi = v; }
  cpu.push(0x47a985); f_0047a8eb(cpu); if cpu.returned_to != 0x47a985 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a986(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a986); }
 let mut pc: u32 = 0x47a986;
 loop { match pc {
 0x47a986 => {
  { cpu.pushad(); }
  cpu.push(0x47a98c); f_00485770(cpu); if cpu.returned_to != 0x47a98c { return; }
  cpu.push(0x47a991); f_0048cbe8(cpu); if cpu.returned_to != 0x47a991 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x5d3995u32; cpu.esi = v; }
  cpu.push(0x47a99c); f_0047a9b2(cpu); if cpu.returned_to != 0x47a99c { return; }
  cpu.push(0x47a9a1); f_0047aaae(cpu); if cpu.returned_to != 0x47a9a1 { return; }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffu32); cpu.ecx = v; }
  { let v: u32 = 0x5d3d6bu32; cpu.esi = v; }
  cpu.push(0x47a9b1); f_0047a8eb(cpu); if cpu.returned_to != 0x47a9b1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047a9b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47a9b2); }
 let mut pc: u32 = 0x47a9b2;
 loop { match pc {
 0x47a9b2 => {
  { let v: u32 = 0x5d3418u32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0x3e8u32; cpu.edx = v; }
  pc = 0x47a9c6; continue;
 }
 0x47a9c6 => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x47a9cb; continue;
 }
 0x47a9cb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = ((cpu.ecx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))) as u32; cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x47aa3a; continue; }
  pc = 0x47a9e7; continue;
 }
 0x47a9e7 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x47aa3a; continue; }
  pc = 0x47a9f1; continue;
 }
 0x47a9f1 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x47aa3a; continue; }
  pc = 0x47a9f6; continue;
 }
 0x47a9f6 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x47aa3a; continue; }
  pc = 0x47a9fe; continue;
 }
 0x47a9fe => {
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x47aa2a); f_0047ab16(cpu); if cpu.returned_to != 0x47aa2a { return; }
  if cpu.cf { pc = 0x47aa3a; continue; }
  pc = 0x47aa2c; continue;
 }
 0x47aa2c => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x4d6bd8u32)); cpu.w8(0x4d6bd8u32, v); }
  cpu.push(0x47aa3a); f_0047aa5d(cpu); if cpu.returned_to != 0x47aa3a { return; }
  pc = 0x47aa3a; continue;
 }
 0x47aa3a => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47a9cb; continue; }
  pc = 0x47aa41; continue;
 }
 0x47aa41 => {
  { let v: u32 = cpu.edx.wrapping_add(0x3e8u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x47a9c6; continue; }
  pc = 0x47aa51; continue;
 }
 0x47aa51 => {
  { let v: u32 = 0x5d3418u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047aa5d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47aa5d); }
 let mut pc: u32 = 0x47aa5d;
 loop { match pc {
 0x47aa5d => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6bd8u32), 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x47aaac; continue; }
  pc = 0x47aa67; continue;
 }
 0x47aa67 => {
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0xffffffffu32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x47aa80); f_00485b77(cpu); if cpu.returned_to != 0x47aa80 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0xfffffffeu32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x47aa9d); f_00485b77(cpu); if cpu.returned_to != 0x47aa9d { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x47aaac; continue; }
  pc = 0x47aaa3; continue;
 }
 0x47aaa3 => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0xfffffffeu32.wrapping_add(cpu.edi)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0xfffffffeu32.wrapping_add(cpu.edi), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  pc = 0x47aaac; continue;
 }
 0x47aaac => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047aaae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47aaae); }
 let mut pc: u32 = 0x47aaae;
 loop { match pc {
 0x47aaae => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47ab15; continue; }
  pc = 0x47aab3; continue;
 }
 0x47aab3 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0x54a7d0u32; cpu.edi = v; }
  pc = 0x47aabc; continue;
 }
 0x47aabc => {
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x47aad6); f_00485b77(cpu); if cpu.returned_to != 0x47aad6 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47aabc; continue; }
  pc = 0x47aae3; continue;
 }
 0x47aae3 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x47aaeb; continue;
 }
 0x47aaeb => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x54a7d0u32; cpu.esi = v; }
  pc = 0x47aaf2; continue;
 }
 0x47aaf2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.cf { pc = 0x47ab05; continue; }
  pc = 0x47aaf9; continue;
 }
 0x47aaf9 => {
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x1u32.wrapping_add(cpu.edi)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x47ab05; continue;
 }
 0x47ab05 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47aaf2; continue; }
  pc = 0x47ab0d; continue;
 }
 0x47ab0d => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47aaeb; continue; }
  pc = 0x47ab13; continue;
 }
 0x47ab13 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x47ab15; continue;
 }
 0x47ab15 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ab16(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ab16); }
 let mut pc: u32 = 0x47ab16;
 loop { match pc {
 0x47ab16 => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x47ab17; continue;
 }
 0x47ab17 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x47ab26; continue; }
  pc = 0x47ab1c; continue;
 }
 0x47ab1c => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x47ab23; continue; }
  pc = 0x47ab20; continue;
 }
 0x47ab20 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x47ab17; continue;
 }
 0x47ab23 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47ab26 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ab2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ab2c); }
 let mut pc: u32 = 0x47ab2c;
 loop { match pc {
 0x47ab2c => {
  { let v: u16 = 0x1u16; cpu.w16(0x4d67ecu32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x5b3945u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1fd0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5922f9u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5922fbu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x56c6fau32, v); }
  { let v: u8 = 0x14u8; cpu.w8(0x5adc53u32, v); }
  { let v: u16 = 0x258u16; cpu.w16(0x4d69bcu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d69a2u32, v); }
  { let v: u32 = 0x1bu32; cpu.w32(0x4d6bc8u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d69b6u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d69d6u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d69d8u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d69e0u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4e1fc3u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d69bau32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d69b8u32, v); }
  { let v: u32 = 0x190u32; cpu.w32(0x4d6bccu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5cea3cu32, v); }
  { let v: u8 = 0x18u8; cpu.w8(0x56d396u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047abd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47abd8); }
 let mut pc: u32 = 0x47abd8;
 loop { match pc {
 0x47abd8 => {
  { let v: u32 = 0x0u32; cpu.w32(0x576470u32, v); }
  { let v: u32 = 0x4ae77cu32; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  cpu.push(0x47ac01); f_00415463(cpu); if cpu.returned_to != 0x47ac01 { return; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x576470u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ac00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ac00); }
 let mut pc: u32 = 0x47ac00;
 loop { match pc {
 0x47ac00 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffe2u32.wrapping_add(cpu.ecx))); cpu.w32(0xffffe2u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x57647015u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x57647015u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32; cpu.w32(0x576478u32, v); }
  { let v: u32 = 0x54a7d0u32; cpu.ebp = v; }
  { let v: u32 = 0x5d4094u32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x47ac27; continue;
 }
 0x47ac27 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x47ac4d; continue; }
  pc = 0x47ac2e; continue;
 }
 0x47ac2e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x2u32); }
  if cpu.zf { pc = 0x47ac3d; continue; }
  pc = 0x47ac39; continue;
 }
 0x47ac39 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x47ac40; continue;
 }
 0x47ac3d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x47ac40; continue;
 }
 0x47ac40 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  cpu.poll(); pc = 0x47ac27; continue;
 }
 0x47ac4d => {
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x47ac68; continue; }
  pc = 0x47ac52; continue;
 }
 0x47ac52 => {
  { let v: u32 = 0x54a7d0u32; cpu.esi = v; }
  cpu.push(0x47ac5c); f_00415463(cpu); if cpu.returned_to != 0x47ac5c { return; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x576478u32, v); }
  pc = 0x47ac68; continue;
 }
 0x47ac68 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ac69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ac69); }
 let mut pc: u32 = 0x47ac69;
 loop { match pc {
 0x47ac69 => {
  { let v: u32 = cpu.r32(0x4d6a38u32); cpu.esi = v; }
  { let v: u32 = 0x61a8u32; cpu.ecx = v; }
  cpu.push(0x47ac79); f_00415463(cpu); if cpu.returned_to != 0x47ac79 { return; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x576474u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ac88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ac88); }
 let mut pc: u32 = 0x47ac88;
 loop { match pc {
 0x47ac88 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56d395u32, v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x57585au32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x575f0fu32, v); }
  { let v: u8 = cpu.r8(0x4e15b1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  cpu.push(0x47acc6); f_0047ad40(cpu); if cpu.returned_to != 0x47acc6 { return; }
  { let v: u32 = 0x575ef2u32; cpu.esi = v; }
  cpu.push(0x47acd0); f_0047b0ac(cpu); if cpu.returned_to != 0x47acd0 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56d395u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e15b1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ad0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ad0f); }
 let mut pc: u32 = 0x47ad0f;
 loop { match pc {
 0x47ad0f => {
  { let v: u8 = 0x1u8; cpu.w8(0x56d395u32, v); }
  { let v: u8 = cpu.r8(0x4e15b1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  cpu.push(0x47ad28); f_0047ad40(cpu); if cpu.returned_to != 0x47ad28 { return; }
  { let v: u32 = 0x575f14u32; cpu.esi = v; }
  cpu.push(0x47ad32); f_0047b0ac(cpu); if cpu.returned_to != 0x47ad32 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56d395u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e15b1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ad40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ad40); }
 let mut pc: u32 = 0x47ad40;
 loop { match pc {
 0x47ad40 => {
  cpu.push(0x47ad45); f_004478a1(cpu); if cpu.returned_to != 0x47ad45 { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x47ad4a; continue;
 }
 0x47ad4a => {
  { let v: u8 = 0x0u8; cpu.w8(0x509190u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x32u32); }
  if !cpu.zf { cpu.poll(); pc = 0x47ad4a; continue; }
  pc = 0x47ad57; continue;
 }
 0x47ad57 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ad58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ad58); }
 let mut pc: u32 = 0x47ad58;
 loop { match pc {
 0x47ad58 => {
  { let v: u32 = cpu.r32(0x4d6ab0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6ab8u32)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x59f43cu32), 0xffffffffu32); }
  if cpu.zf { pc = 0x47ad74; continue; }
  pc = 0x47ad6c; continue;
 }
 0x47ad6c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x59f43cu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47ad79; continue; }
  pc = 0x47ad74; continue;
 }
 0x47ad74 => {
  { let v: u32 = cpu.eax; cpu.w32(0x59f43cu32, v); }
  pc = 0x47ad79; continue;
 }
 0x47ad79 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ae6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ae6f); }
 let mut pc: u32 = 0x47ae6f;
 loop { match pc {
 0x47ae6f => {
  { cpu.sub8(cpu.r8(0x56d395u32), 0x0u8); }
  if !cpu.zf { pc = 0x47ae87; continue; }
  pc = 0x47ae78; continue;
 }
 0x47ae78 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x47ae87; continue; }
  pc = 0x47ae7c; continue;
 }
 0x47ae7c => {
  { cpu.sub8(cpu.r8(0x56d229u32), 0x0u8); }
  if cpu.sf { pc = 0x47ae87; continue; }
  pc = 0x47ae85; continue;
 }
 0x47ae85 => {
  if !cpu.zf { pc = 0x47aed0; continue; }
  pc = 0x47ae87; continue;
 }
 0x47ae87 => {
  { let v: u32 = cpu.r32(0x4d6afcu32); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x47ae91; continue;
 }
 0x47ae91 => {
  { cpu.lods(1, false); }
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x47aebb; continue; }
  pc = 0x47ae96; continue;
 }
 0x47ae96 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5d4750u32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x5d4754u32, v); }
  cpu.push(0x47aeaa); f_0047aed1(cpu); if cpu.returned_to != 0x47aeaa { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x5d4748u32)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x5d4748u32)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  pc = 0x47aebb; continue;
 }
 0x47aebb => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x4d6858u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x47ae91; continue; }
  pc = 0x47aec5; continue;
 }
 0x47aec5 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x509190u32.wrapping_add(cpu.ebx), v); }
  pc = 0x47aed0; continue;
 }
 0x47aed0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b0ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b0ac); }
 let mut pc: u32 = 0x47b0ac;
 loop { match pc {
 0x47b0ac => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x47b0ba; continue; }
  pc = 0x47b0b1; continue;
 }
 0x47b0b1 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x47b0b7); f_0047ae6f(cpu); if cpu.returned_to != 0x47b0b7 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.poll(); pc = 0x47b0ac; continue;
 }
 0x47b0ba => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b0bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b0bb); }
 let mut pc: u32 = 0x47b0bb;
 loop { match pc {
 0x47b0bb => {
  { cpu.sub8(cpu.r8(0x56d229u32), 0x1u8); }
  if cpu.zf { pc = 0x47b0cf; continue; }
  pc = 0x47b0c4; continue;
 }
 0x47b0c4 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x509190u32.wrapping_add(cpu.ebx), v); }
  pc = 0x47b0cf; continue;
 }
 0x47b0cf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b0d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b0d0); }
 let mut pc: u32 = 0x47b0d0;
 loop { match pc {
 0x47b0d0 => {
  { cpu.sub8(cpu.r8(0x56d229u32), 0x1u8); }
  if cpu.zf { pc = 0x47b0eb; continue; }
  pc = 0x47b0d9; continue;
 }
 0x47b0d9 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x47b0db; continue;
 }
 0x47b0db => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x47b0eb; continue; }
  pc = 0x47b0e0; continue;
 }
 0x47b0e0 => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x509190u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x47b0db; continue;
 }
 0x47b0eb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b0ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b0ec); }
 let mut pc: u32 = 0x47b0ec;
 loop { match pc {
 0x47b0ec => {
  { cpu.sub8(cpu.r8(0x56d393u32), 0x0u8); }
  if !cpu.zf { pc = 0x47b0fe; continue; }
  pc = 0x47b0f5; continue;
 }
 0x47b0f5 => {
  { cpu.sub8(cpu.r8(0x56d229u32), 0x1u8); }
  if cpu.zf { pc = 0x47b14d; continue; }
  pc = 0x47b0fe; continue;
 }
 0x47b0fe => {
  { let v: u16 = cpu.r16(0x4d6858u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d68f0u32, v); }
  { let v: u32 = cpu.r32(0x4d6afcu32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  pc = 0x47b115; continue;
 }
 0x47b115 => {
  { let v: u16 = cpu.or16((cpu.ebp as u16), (cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47b148; continue; }
  pc = 0x47b11a; continue;
 }
 0x47b11a => {
  { cpu.lods(1, false); }
  { cpu.sub8(cpu.r8(0x56d393u32), 0x0u8); }
  if !cpu.zf { pc = 0x47b128; continue; }
  pc = 0x47b124; continue;
 }
 0x47b124 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x47b13f; continue; }
  pc = 0x47b128; continue;
 }
 0x47b128 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x509190u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if cpu.zf { pc = 0x47b13f; continue; }
  pc = 0x47b135; continue;
 }
 0x47b135 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f5au32, v); }
  cpu.push(0x47b13f); f_0047b22e(cpu); if cpu.returned_to != 0x47b13f { return; }
  pc = 0x47b13f; continue;
 }
 0x47b13f => {
  { let v: u16 = cpu.dec16(cpu.r16(0x4d68f0u32)); cpu.w16(0x4d68f0u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x47b115; continue; }
  pc = 0x47b148; continue;
 }
 0x47b148 => {
  cpu.push(0x47b14d); f_0047b14e(cpu); if cpu.returned_to != 0x47b14d { return; }
  pc = 0x47b14d; continue;
 }
 0x47b14d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b14e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b14e); }
 let mut pc: u32 = 0x47b14e;
 loop { match pc {
 0x47b14e => {
  { let v: u32 = 0x5431b4u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x4d6858u32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x47b15e; continue;
 }
 0x47b15e => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x47b169; continue; }
  pc = 0x47b163; continue;
 }
 0x47b163 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47b169; continue; }
  pc = 0x47b167; continue;
 }
 0x47b167 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  pc = 0x47b169; continue;
 }
 0x47b169 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47b15e; continue; }
  pc = 0x47b16e; continue;
 }
 0x47b16e => {
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x47b19a; continue; }
  pc = 0x47b173; continue;
 }
 0x47b173 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x47b183; continue;
 }
 0x47b183 => {
  { cpu.and32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x47b18e; continue; }
  pc = 0x47b18b; continue;
 }
 0x47b18b => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x47b183; continue;
 }
 0x47b18e => {
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6ab0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47b19a => {
  { let v: u32 = cpu.r32(0x4d6ab8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6ab0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b1a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b1a7); }
 let mut pc: u32 = 0x47b1a7;
 loop { match pc {
 0x47b1a7 => {
  { cpu.sub8(cpu.r8(0x4e1f5bu32), 0x0u8); }
  if cpu.zf { pc = 0x47b22d; continue; }
  pc = 0x47b1b4; continue;
 }
 0x47b1b4 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u16 = 0x80u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x20u32; cpu.ebp = v; }
  cpu.push(0x47b1d1); f_0043df25(cpu); if cpu.returned_to != 0x47b1d1 { return; }
  { let v: u32 = 0x24u32; cpu.eax = v; }
  { let v: u32 = 0x44u32; cpu.ebx = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u32 = 0x7cu32; cpu.edx = v; }
  { let v: u32 = 0x14u32; cpu.ebp = v; }
  cpu.push(0x47b1ef); f_0043df25(cpu); if cpu.returned_to != 0x47b1ef { return; }
  { let v: u32 = cpu.r32(0x4d6b00u32); cpu.ebx = v; }
  { let v: u32 = 0x38u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6ab0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6ab8u32)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 10u32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = 0x7cu32; cpu.ebx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x24u32; cpu.eax = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u32 = 0x7cu32; cpu.edx = v; }
  { let v: u32 = 0x18u32; cpu.ebp = v; }
  cpu.push(0x47b22d); f_0043df25(cpu); if cpu.returned_to != 0x47b22d { return; }
  pc = 0x47b22d; continue;
 }
 0x47b22d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b22e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b22e); }
 let mut pc: u32 = 0x47b22e;
 loop { match pc {
 0x47b22e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x4d6afcu32)); cpu.esi = v; }
  { let v: u32 = 0x5431b0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.esi = v; }
  pc = 0x47b247; continue;
 }
 0x47b247 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x47b252; continue; }
  pc = 0x47b24c; continue;
 }
 0x47b24c => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x47b324; continue;
 }
 0x47b252 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6b68u32, v); }
  pc = 0x47b25c; continue;
 }
 0x47b25c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x5431b4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x58e95cu32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x47b280; continue; }
  pc = 0x47b279; continue;
 }
 0x47b279 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58e95cu32.wrapping_add(cpu.eax), v); }
  pc = 0x47b280; continue;
 }
 0x47b280 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x47b291; continue;
 }
 0x47b291 => {
  { cpu.and32(cpu.ebx, 0x3u32); }
  if cpu.zf { pc = 0x47b29c; continue; }
  pc = 0x47b299; continue;
 }
 0x47b299 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x47b291; continue;
 }
 0x47b29c => {
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  if cpu.sf { pc = 0x47b2e8; continue; }
  pc = 0x47b2a3; continue;
 }
 0x47b2a3 => {
  { let v: u32 = cpu.r32(0x4d6b68u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x4d6858u32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x5431b4u32; cpu.edi = v; }
  pc = 0x47b2b9; continue;
 }
 0x47b2b9 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.esi); }
  if !cpu.zf { pc = 0x47b2e1; continue; }
  pc = 0x47b2bd; continue;
 }
 0x47b2bd => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x5431b4u32); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 2u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4d6afcu32)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1f5au32)); }
  if !cpu.zf { pc = 0x47b2e6; continue; }
  pc = 0x47b2d8; continue;
 }
 0x47b2d8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  cpu.poll(); pc = 0x47b25c; continue;
 }
 0x47b2e1 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47b2b9; continue; }
  pc = 0x47b2e6; continue;
 }
 0x47b2e6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x47b2e8; continue;
 }
 0x47b2e8 => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x47b2ea; continue;
 }
 0x47b2ea => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6b68u32); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x4d6858u32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x5431b4u32; cpu.edi = v; }
  pc = 0x47b301; continue;
 }
 0x47b301 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x47b30c; continue; }
  pc = 0x47b305; continue;
 }
 0x47b305 => {
  cpu.push(0x47b30a); f_0047b32a(cpu); if cpu.returned_to != 0x47b30a { return; }
  cpu.poll(); pc = 0x47b2ea; continue;
 }
 0x47b30c => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47b301; continue; }
  pc = 0x47b311; continue;
 }
 0x47b311 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x47b324; continue; }
  pc = 0x47b318; continue;
 }
 0x47b318 => {
  { let v: u16 = cpu.shr16((cpu.ebp as u16), 4u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebp as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x47b324; continue; }
  pc = 0x47b321; continue;
 }
 0x47b321 => {
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x47b324; continue;
 }
 0x47b324 => {
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b32a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b32a); }
 let mut pc: u32 = 0x47b32a;
 loop { match pc {
 0x47b32a => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_sub(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6b68u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x47b344; continue;
 }
 0x47b344 => {
  { cpu.and32(cpu.ebx, 0x3u32); }
  if cpu.zf { pc = 0x47b34f; continue; }
  pc = 0x47b34c; continue;
 }
 0x47b34c => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x47b344; continue;
 }
 0x47b34f => {
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6b68u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b363(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b363); }
 let mut pc: u32 = 0x47b363;
 loop { match pc {
 0x47b363 => {
  cpu.push(0x47b368); f_00447791(cpu); if cpu.returned_to != 0x47b368 { return; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x4d6858u32) as u32; cpu.edx = v; }
  { let v: u16 = cpu.shl16((cpu.edx as u16), 2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), 0x2u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x47b37e); f_00447781(cpu); if cpu.returned_to != 0x47b37e { return; }
  { let v: u32 = cpu.r16(0x4d6858u32) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6afcu32); cpu.edx = v; }
  cpu.push(0x47b390); f_0041c901(cpu); if cpu.returned_to != 0x47b390 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b394(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b394); }
 let mut pc: u32 = 0x47b394;
 loop { match pc {
 0x47b394 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b41e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b41e); }
 let mut pc: u32 = 0x47b41e;
 loop { match pc {
 0x47b41e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x575565u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x575567u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x575564u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b433(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b433); }
 let mut pc: u32 = 0x47b433;
 loop { match pc {
 0x47b433 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x575543u32, v); }
  { cpu.pushad(); }
  { let v: u32 = 0x5293a0u32; cpu.edi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  pc = 0x47b445; continue;
 }
 0x47b445 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x47b479; continue; }
  pc = 0x47b44c; continue;
 }
 0x47b44c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x400u32); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x47b479; continue; }
  pc = 0x47b477; continue;
 }
 0x47b477 => {
  pc = 0x47b481; continue;
 }
 0x47b479 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x40u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47b445; continue; }
  pc = 0x47b47e; continue;
 }
 0x47b47e => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47b481 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b484(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b484); }
 let mut pc: u32 = 0x47b484;
 loop { match pc {
 0x47b484 => {
  { let v: u32 = 0x56d528u32; cpu.edi = v; }
  { let v: u32 = 0x2000u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x56d528u32; cpu.esi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x575528u32, v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d954au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47b4ab; continue;
 }
 0x47b4ab => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d9548u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x47b4b5; continue;
 }
 0x47b4b5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4dd54cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47b4eb; continue; }
  pc = 0x47b4df; continue;
 }
 0x47b4df => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x47b4eb; continue; }
  pc = 0x47b4e3; continue;
 }
 0x47b4e3 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x47b517; continue; }
  pc = 0x47b4e7; continue;
 }
 0x47b4e7 => {
  { cpu.sub8((cpu.eax as u8), 0x28u8); }
  if cpu.zf { pc = 0x47b4eb; continue; }
  pc = 0x47b4eb; continue;
 }
 0x47b4eb => {
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u16 = cpu.inc16(cpu.r16(0x575528u32)); cpu.w16(0x575528u32, v); }
  { cpu.sub16(cpu.r16(0x575528u32), 0x3fffu16); }
  if (cpu.sf != cpu.of) { pc = 0x47b517; continue; }
  pc = 0x47b512; continue;
 }
 0x47b512 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x47b520; continue;
 }
 0x47b517 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.sf { cpu.poll(); pc = 0x47b4b5; continue; }
  pc = 0x47b51d; continue;
 }
 0x47b51d => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.sf { cpu.poll(); pc = 0x47b4ab; continue; }
  pc = 0x47b520; continue;
 }
 0x47b520 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b521(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b521); }
 let mut pc: u32 = 0x47b521;
 loop { match pc {
 0x47b521 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x47b52a); f_00420989(cpu); if cpu.returned_to != 0x47b52a { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x11u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4000u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b541(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b541); }
 let mut pc: u32 = 0x47b541;
 loop { match pc {
 0x47b541 => {
  cpu.push(0x47b546); f_0041c743(cpu); if cpu.returned_to != 0x47b546 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u16 = 0x14u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47b550; continue;
 }
 0x47b550 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x47b556); f_0041c743(cpu); if cpu.returned_to != 0x47b556 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  cpu.push(0x47b564); f_0041c743(cpu); if cpu.returned_to != 0x47b564 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x47b56f); f_0041c743(cpu); if cpu.returned_to != 0x47b56f { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x47b57b); f_0041c743(cpu); if cpu.returned_to != 0x47b57b { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x2u32); }
  if !cpu.zf { pc = 0x47b5a0; continue; }
  pc = 0x47b59e; continue;
 }
 0x47b59e => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x47b5a0; continue;
 }
 0x47b5a0 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x47b5a8); f_0047d7e2(cpu); if cpu.returned_to != 0x47b5a8 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47b550; continue; }
  pc = 0x47b5ab; continue;
 }
 0x47b5ab => {
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x47b5b5; continue;
 }
 0x47b5b5 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x47b5f6; continue; }
  pc = 0x47b5be; continue;
 }
 0x47b5be => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x47b5cb); f_0047b6c8(cpu); if cpu.returned_to != 0x47b5cb { return; }
  if cpu.cf { pc = 0x47b5fc; continue; }
  pc = 0x47b5cd; continue;
 }
 0x47b5cd => {
  { cpu.sub16((cpu.eax as u16), 0x8f8u16); }
  if !cpu.zf { pc = 0x47b5f6; continue; }
  pc = 0x47b5d3; continue;
 }
 0x47b5d3 => {
  { cpu.pushad(); }
  { let v: u16 = 0x14u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x50u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x47b5e1); f_0041c808(cpu); if cpu.returned_to != 0x47b5e1 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x47b5f5); f_0047badd(cpu); if cpu.returned_to != 0x47b5f5 { return; }
  { cpu.popad(); }
  pc = 0x47b5f6; continue;
 }
 0x47b5f6 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47b5b5; continue; }
  pc = 0x47b5fb; continue;
 }
 0x47b5fb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47b5fc => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)) & 0x7fu8; cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x559754u32); cpu.eax = v; }
  { let v: u8 = 0x28u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x47b622); f_0047cb11(cpu); if cpu.returned_to != 0x47b622 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x5d47dau32, v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  pc = 0x47b633; continue;
 }
 0x47b633 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub32(cpu.ecx, 0x3u32); }
  if (cpu.sf == cpu.of) { pc = 0x47b652; continue; }
  pc = 0x47b63c; continue;
 }
 0x47b63c => {
  cpu.push(0x47b641); f_0041c743(cpu); if cpu.returned_to != 0x47b641 { return; }
  { let v: u16 = cpu.r16(0x5d47dau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.and16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47b652; continue; }
  pc = 0x47b64d; continue;
 }
 0x47b64d => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x47b6b9; continue;
 }
 0x47b652 => {
  { let v: u16 = cpu.r16(0x5d47dau32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d47dau32, v); }
  { let v: u16 = 0x20u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2au16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x47b672); f_0041c808(cpu); if cpu.returned_to != 0x47b672 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x2u16; cpu.w16(0x4d6934u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5d47e1u32; cpu.edi = v; }
  cpu.push(0x47b68f); f_0041c743(cpu); if cpu.returned_to != 0x47b68f { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = 0xff02u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x80u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x6u16; cpu.w16(0x4d6938u32, v); }
  cpu.push(0x47b6b4); f_0047b521(cpu); if cpu.returned_to != 0x47b6b4 { return; }
  cpu.push(0x47b6b9); f_0047b6fe(cpu); if cpu.returned_to != 0x47b6b9 { return; }
  pc = 0x47b6b9; continue;
 }
 0x47b6b9 => {
  { cpu.popad(); }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47b633; continue; }
  pc = 0x47b6c2; continue;
 }
 0x47b6c2 => {
  { cpu.popad(); }
  cpu.poll(); pc = 0x47b5f6; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b6c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b6c8); }
 let mut pc: u32 = 0x47b6c8;
 loop { match pc {
 0x47b6c8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x5d4811u32; cpu.edi = v; }
  { let v: u16 = 0x10u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47b6d3; continue;
 }
 0x47b6d3 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.edi))); }
  if cpu.zf { pc = 0x47b6e1; continue; }
  pc = 0x47b6d8; continue;
 }
 0x47b6d8 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47b6d3; continue; }
  pc = 0x47b6dd; continue;
 }
 0x47b6dd => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47b6e1 => {
  { let v: u16 = 0x10u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d693au32, v); }
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b6fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b6fe); }
 let mut pc: u32 = 0x47b6fe;
 loop { match pc {
 0x47b6fe => {
  { cpu.pushad(); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x4d69b6u32), 0x0u16); }
  if cpu.zf { pc = 0x47b863; continue; }
  pc = 0x47b70e; continue;
 }
 0x47b70e => {
  cpu.push(0x47b713); f_0047cb29(cpu); if cpu.returned_to != 0x47b713 { return; }
  if cpu.cf { pc = 0x47b86f; continue; }
  pc = 0x47b719; continue;
 }
 0x47b719 => {
  { let v: u8 = 0x0u8; cpu.w8(0x576bd5u32, v); }
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x47b730; continue; }
  pc = 0x47b726; continue;
 }
 0x47b726 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x576bd5u32, v); }
  pc = 0x47b730; continue;
 }
 0x47b730 => {
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.sf { pc = 0x47b739; continue; }
  pc = 0x47b736; continue;
 }
 0x47b736 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x47b739; continue;
 }
 0x47b739 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x27u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4d6938u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x34u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x2fu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x35u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x36u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x37u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x5d4915u32; cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.zf { pc = 0x47b7c6; continue; }
  pc = 0x47b7c2; continue;
 }
 0x47b7c2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x37u32.wrapping_add(cpu.edi), v); }
  pc = 0x47b7c6; continue;
 }
 0x47b7c6 => {
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3du32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r8(0x2du32.wrapping_add(cpu.edi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xe0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r8(0x21u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x1cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r8(0x21u32.wrapping_add(cpu.edi)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x23u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x576bd5u32), 0x0u8); }
  if cpu.zf { pc = 0x47b826; continue; }
  pc = 0x47b822; continue;
 }
 0x47b822 => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.edi)) | 0x10u8; cpu.w8(0x35u32.wrapping_add(cpu.edi), v); }
  pc = 0x47b826; continue;
 }
 0x47b826 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x25u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x38u32.wrapping_add(cpu.edi), v); }
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.sf { pc = 0x47b83d; continue; }
  pc = 0x47b832; continue;
 }
 0x47b832 => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.edi)) | 0x20u8; cpu.w8(0x35u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x25u32.wrapping_add(cpu.edi), v); }
  pc = 0x47b83d; continue;
 }
 0x47b83d => {
  { let v: u8 = 0x0u8; cpu.w8(0x2eu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x39u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2fu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x31u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x30u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x33u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x47b860; continue; }
  pc = 0x47b85c; continue;
 }
 0x47b85c => {
  { let v: u8 = cpu.or8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x44u8); cpu.w8(0x35u32.wrapping_add(cpu.edi), v); }
  pc = 0x47b860; continue;
 }
 0x47b860 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47b863 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47b86f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b872(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b872); }
 let mut pc: u32 = 0x47b872;
 loop { match pc {
 0x47b872 => {
  { cpu.sub16(cpu.r16(0x575528u32), 0x0u16); }
  if !cpu.zf { pc = 0x47b87d; continue; }
  pc = 0x47b87c; continue;
 }
 0x47b87c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47b87d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x4d69b8u32), 0x0u16); }
  if cpu.zf { pc = 0x47b9e9; continue; }
  pc = 0x47b88f; continue;
 }
 0x47b88f => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = 0x4ae7eau32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d6b64u32, v); }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x64u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x47b8b9); f_0041c808(cpu); if cpu.returned_to != 0x47b8b9 { return; }
  { cpu.sub16((cpu.eax as u16), 0xau16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47b9e9; continue; }
  pc = 0x47b8c3; continue;
 }
 0x47b8c3 => {
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x575528u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x47b8d6); f_0041c808(cpu); if cpu.returned_to != 0x47b8d6 { return; }
  { cpu.sub16(cpu.r16(0x575528u32), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x47b8c3; continue; }
  pc = 0x47b8df; continue;
 }
 0x47b8df => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x56d529u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x56d528u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u16 = 0x20u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x47b907); f_0047b433(cpu); if cpu.returned_to != 0x47b907 { return; }
  if cpu.cf { pc = 0x47b9e9; continue; }
  pc = 0x47b90d; continue;
 }
 0x47b90d => {
  { cpu.sub8(cpu.r8(0x4d6b64u32), 0x4u8); }
  if cpu.zf { pc = 0x47b966; continue; }
  pc = 0x47b916; continue;
 }
 0x47b916 => {
  { cpu.sub8(cpu.r8(0x4d6b64u32), 0x3u8); }
  if cpu.zf { pc = 0x47b95c; continue; }
  pc = 0x47b91f; continue;
 }
 0x47b91f => {
  { cpu.sub8(cpu.r8(0x4d6b64u32), 0x1u8); }
  if cpu.zf { pc = 0x47b9c3; continue; }
  pc = 0x47b92c; continue;
 }
 0x47b92c => {
  { cpu.sub8(cpu.r8(0x4d6b64u32), 0x0u8); }
  if !cpu.zf { pc = 0x47b99b; continue; }
  pc = 0x47b935; continue;
 }
 0x47b935 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47b93b); f_0041c743(cpu); if cpu.returned_to != 0x47b93b { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16((cpu.ecx as u16), 0x2u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47b952; continue; }
  pc = 0x47b948; continue;
 }
 0x47b948 => {
  cpu.push(0x47b94d); f_0047c7f1(cpu); if cpu.returned_to != 0x47b94d { return; }
  pc = 0x47b9e9; continue;
 }
 0x47b952 => {
  cpu.push(0x47b957); f_0047c787(cpu); if cpu.returned_to != 0x47b957 { return; }
  pc = 0x47b9e9; continue;
 }
 0x47b95c => {
  cpu.push(0x47b961); f_0047b9ee(cpu); if cpu.returned_to != 0x47b961 { return; }
  pc = 0x47b9e9; continue;
 }
 0x47b966 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47b96c); f_0041c743(cpu); if cpu.returned_to != 0x47b96c { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x47b978); f_0041c743(cpu); if cpu.returned_to != 0x47b978 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47b993; continue; }
  pc = 0x47b97e; continue;
 }
 0x47b97e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = 0x40u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x47b989); f_0047b433(cpu); if cpu.returned_to != 0x47b989 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x47b9e9; continue; }
  pc = 0x47b98c; continue;
 }
 0x47b98c => {
  cpu.push(0x47b991); f_0047bfdd(cpu); if cpu.returned_to != 0x47b991 { return; }
  pc = 0x47b9e9; continue;
 }
 0x47b993 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x47b999); f_0047c164(cpu); if cpu.returned_to != 0x47b999 { return; }
  pc = 0x47b9e9; continue;
 }
 0x47b99b => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47b9a1); f_0041c743(cpu); if cpu.returned_to != 0x47b9a1 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x47b9ad); f_0041c743(cpu); if cpu.returned_to != 0x47b9ad { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47b9bb; continue; }
  pc = 0x47b9b3; continue;
 }
 0x47b9b3 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x47b9b9); f_0047c69d(cpu); if cpu.returned_to != 0x47b9b9 { return; }
  pc = 0x47b9e9; continue;
 }
 0x47b9bb => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x47b9c1); f_0047c38c(cpu); if cpu.returned_to != 0x47b9c1 { return; }
  pc = 0x47b9e9; continue;
 }
 0x47b9c3 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47b9c9); f_0041c743(cpu); if cpu.returned_to != 0x47b9c9 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x47b9d5); f_0041c743(cpu); if cpu.returned_to != 0x47b9d5 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47b9e3; continue; }
  pc = 0x47b9db; continue;
 }
 0x47b9db => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x47b9e1); f_0047bef3(cpu); if cpu.returned_to != 0x47b9e1 { return; }
  pc = 0x47b9e9; continue;
 }
 0x47b9e3 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x47b9e9); f_0047bbe2(cpu); if cpu.returned_to != 0x47b9e9 { return; }
  pc = 0x47b9e9; continue;
 }
 0x47b9e9 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047b9ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47b9ee); }
 let mut pc: u32 = 0x47b9ee;
 loop { match pc {
 0x47b9ee => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x47b9f5); f_0047ca8f(cpu); if cpu.returned_to != 0x47b9f5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x47ba2a; continue; }
  pc = 0x47b9f9; continue;
 }
 0x47b9f9 => {
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = 0xa75u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x7u16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693au32, v); }
  cpu.push(0x47ba2a); f_00486419(cpu); if cpu.returned_to != 0x47ba2a { return; }
  pc = 0x47ba2a; continue;
 }
 0x47ba2a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ba2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ba2b); }
 let mut pc: u32 = 0x47ba2b;
 loop { match pc {
 0x47ba2b => {
  cpu.push(0x47ba30); f_004679cb(cpu); if cpu.returned_to != 0x47ba30 { return; }
  if cpu.cf { pc = 0x47ba98; continue; }
  pc = 0x47ba32; continue;
 }
 0x47ba32 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x47ba59; continue; }
  pc = 0x47ba49; continue;
 }
 0x47ba49 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47ba50); f_00401130(cpu); if cpu.returned_to != 0x47ba50 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x47ba59; continue;
 }
 0x47ba59 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x592301u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x47ba64); f_00415d61(cpu); if cpu.returned_to != 0x47ba64 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x73u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x47ba8b; continue; }
  pc = 0x47ba7b; continue;
 }
 0x47ba7b => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47ba82); f_00401130(cpu); if cpu.returned_to != 0x47ba82 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x47ba8b; continue;
 }
 0x47ba8b => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x592301u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x47ba97); f_00415d61(cpu); if cpu.returned_to != 0x47ba97 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x47ba98; continue;
 }
 0x47ba98 => {
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f79u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6960u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67c0u32, v); }
  cpu.push(0x47badb); f_00483307(cpu); if cpu.returned_to != 0x47badb { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047badd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47badd); } r_0047badd(cpu, 0x47badd); }
pub fn f_0047bb00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47bb00); } r_0047badd(cpu, 0x47bb00); }
fn r_0047badd(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x47badd => {
  cpu.push(0x47bae2); f_0047ca54(cpu); if cpu.returned_to != 0x47bae2 { return; }
  if cpu.cf { pc = 0x47bb1b; continue; }
  pc = 0x47bae4; continue;
 }
 0x47bae4 => {
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.and16((cpu.ebx as u16), 0xfff0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  { let v: u32 = 0x47bb1cu32; cpu.esi = v; }
  cpu.push(0x47bb1b); f_0047b6fe(cpu); if cpu.returned_to != 0x47bb1b { return; }
  pc = 0x47bb1b; continue;
 }
 0x47bb00 => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffffffu32.wrapping_add(cpu.edi)), ((cpu.ebx >> 8) as u8)); cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  { let v: u32 = 0x47bb1cu32; cpu.esi = v; }
  cpu.push(0x47bb1b); f_0047b6fe(cpu); if cpu.returned_to != 0x47bb1b { return; }
  pc = 0x47bb1b; continue;
 }
 0x47bb1b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047bb1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47bb1c); }
 let mut pc: u32 = 0x47bb1c;
 loop { match pc {
 0x47bb1c => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0xff05u32; cpu.ecx = v; }
  { let v: u8 = 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47bbb0u32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47bb4e); f_0047b6fe(cpu); if cpu.returned_to != 0x47bb4e { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x5u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x100cu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47bb7eu32; cpu.esi = v; }
  { let v: u8 = 0x5u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47bb7c); f_0047b6fe(cpu); if cpu.returned_to != 0x47bb7c { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047bb7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47bb7e); }
 let mut pc: u32 = 0x47bb7e;
 loop { match pc {
 0x47bb7e => {
  { cpu.pushad(); }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47bbae); f_0047b6fe(cpu); if cpu.returned_to != 0x47bbae { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047bbb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47bbb0); }
 let mut pc: u32 = 0x47bbb0;
 loop { match pc {
 0x47bbb0 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0xeu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47bbe0); f_0047b6fe(cpu); if cpu.returned_to != 0x47bbe0 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047bbe2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47bbe2); }
 let mut pc: u32 = 0x47bbe2;
 loop { match pc {
 0x47bbe2 => {
  { cpu.pushad(); }
  cpu.push(0x47bbe8); f_0047ca7e(cpu); if cpu.returned_to != 0x47bbe8 { return; }
  if cpu.cf { pc = 0x47bc5f; continue; }
  pc = 0x47bbea; continue;
 }
 0x47bbea => {
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x7u32); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47bc2c); f_0047b6fe(cpu); if cpu.returned_to != 0x47bc2c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x24u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47bc61u32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47bc5f); f_0047b6fe(cpu); if cpu.returned_to != 0x47bc5f { return; }
  pc = 0x47bc5f; continue;
 }
 0x47bc5f => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047bc61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47bc61); }
 let mut pc: u32 = 0x47bc61;
 loop { match pc {
 0x47bc61 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47bc68); f_0041c743(cpu); if cpu.returned_to != 0x47bc68 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.and32(cpu.edx, 0x3u32); cpu.edx = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47bcb0u32; cpu.esi = v; }
  cpu.push(0x47bcae); f_0047b6fe(cpu); if cpu.returned_to != 0x47bcae { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047bcb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47bcb0); }
 let mut pc: u32 = 0x47bcb0;
 loop { match pc {
 0x47bcb0 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47bcb7); f_0041c743(cpu); if cpu.returned_to != 0x47bcb7 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x47bcc3; continue; }
  pc = 0x47bcbd; continue;
 }
 0x47bcbd => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47bcc7; continue;
 }
 0x47bcc3 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47bcc7; continue;
 }
 0x47bcc7 => {
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47bd45u32; cpu.esi = v; }
  cpu.push(0x47bd09); f_0047b6fe(cpu); if cpu.returned_to != 0x47bd09 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47bd43); f_0047b6fe(cpu); if cpu.returned_to != 0x47bd43 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047bd45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47bd45); }
 let mut pc: u32 = 0x47bd45;
 loop { match pc {
 0x47bd45 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47bd4c); f_0041c743(cpu); if cpu.returned_to != 0x47bd4c { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x47bd58; continue; }
  pc = 0x47bd52; continue;
 }
 0x47bd52 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47bd5c; continue;
 }
 0x47bd58 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47bd5c; continue;
 }
 0x47bd5c => {
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47bddau32; cpu.esi = v; }
  cpu.push(0x47bd9e); f_0047b6fe(cpu); if cpu.returned_to != 0x47bd9e { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47bdd8); f_0047b6fe(cpu); if cpu.returned_to != 0x47bdd8 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047bdda(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47bdda); }
 let mut pc: u32 = 0x47bdda;
 loop { match pc {
 0x47bdda => {
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47be13); f_0047b6fe(cpu); if cpu.returned_to != 0x47be13 { return; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x24u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47be48); f_0047b6fe(cpu); if cpu.returned_to != 0x47be48 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 6u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x5d4c69u32.wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5d47dcu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x5d47deu32, v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x32u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47beaf); f_0047b6fe(cpu); if cpu.returned_to != 0x47beaf { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x32u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x15u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47bf8cu32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d47dcu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5d47deu32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x47bef1); f_0047b6fe(cpu); if cpu.returned_to != 0x47bef1 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047bef3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47bef3); }
 let mut pc: u32 = 0x47bef3;
 loop { match pc {
 0x47bef3 => {
  cpu.push(0x47bef8); f_0047ca7e(cpu); if cpu.returned_to != 0x47bef8 { return; }
  if cpu.cf { pc = 0x47bf8b; continue; }
  pc = 0x47befe; continue;
 }
 0x47befe => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x5d4c69u32.wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47bf57); f_0047b6fe(cpu); if cpu.returned_to != 0x47bf57 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.add32(cpu.ecx, 0x15u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47bf8cu32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47bf8b); f_0047b6fe(cpu); if cpu.returned_to != 0x47bf8b { return; }
  pc = 0x47bf8b; continue;
 }
 0x47bf8b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047bf8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47bf8c); }
 let mut pc: u32 = 0x47bf8c;
 loop { match pc {
 0x47bf8c => {
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x5d4c71u32.wrapping_add(cpu.edx.wrapping_mul(2)); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47bfdb); f_0047b6fe(cpu); if cpu.returned_to != 0x47bfdb { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047bfdd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47bfdd); }
 let mut pc: u32 = 0x47bfdd;
 loop { match pc {
 0x47bfdd => {
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d4c89u32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5d4c8bu32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x57556eu32, v); }
  cpu.push(0x47bffc); f_0047ca7e(cpu); if cpu.returned_to != 0x47bffc { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x57556eu32, v); }
  if cpu.cf { pc = 0x47c082; continue; }
  pc = 0x47c005; continue;
 }
 0x47c005 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.ecx, 6u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x31u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c083u32; cpu.esi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x47c03a); f_0047b6fe(cpu); if cpu.returned_to != 0x47c03a { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.ecx, 6u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d4c79u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5d4c7bu32.wrapping_add(cpu.ecx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x31u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c0d2u32; cpu.esi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c082); f_0047b6fe(cpu); if cpu.returned_to != 0x47c082 { return; }
  pc = 0x47c082; continue;
 }
 0x47c082 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c083(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c083); }
 let mut pc: u32 = 0x47c083;
 loop { match pc {
 0x47c083 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xeu32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x5d4c89u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x5d4c8bu32.wrapping_add(cpu.ecx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  { let v: u32 = 0x2du32; cpu.ecx = v; }
  { let v: u8 = 0x4u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c121u32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  cpu.push(0x47c0d0); f_0047b6fe(cpu); if cpu.returned_to != 0x47c0d0 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c0d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c0d2); }
 let mut pc: u32 = 0x47c0d2;
 loop { match pc {
 0x47c0d2 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xeu32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x5d4c89u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x5d4c8bu32.wrapping_add(cpu.ecx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  { let v: u32 = 0x2fu32; cpu.ecx = v; }
  { let v: u8 = 0x4u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c121u32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  cpu.push(0x47c11f); f_0047b6fe(cpu); if cpu.returned_to != 0x47c11f { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c121(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c121); }
 let mut pc: u32 = 0x47c121;
 loop { match pc {
 0x47c121 => {
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x40u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6938u32, v); }
  { let v: u32 = 0x30u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x47c162); f_0047b6fe(cpu); if cpu.returned_to != 0x47c162 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c164(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c164); }
 let mut pc: u32 = 0x47c164;
 loop { match pc {
 0x47c164 => {
  { cpu.pushad(); }
  cpu.push(0x47c16a); f_0047ca7e(cpu); if cpu.returned_to != 0x47c16a { return; }
  if cpu.cf { pc = 0x47c1e1; continue; }
  pc = 0x47c16c; continue;
 }
 0x47c16c => {
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x7u32); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c1ae); f_0047b6fe(cpu); if cpu.returned_to != 0x47c1ae { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  { let v: u32 = 0x2eu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c1e3u32; cpu.esi = v; }
  cpu.push(0x47c1e1); f_0047b6fe(cpu); if cpu.returned_to != 0x47c1e1 { return; }
  pc = 0x47c1e1; continue;
 }
 0x47c1e1 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c1e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c1e3); }
 let mut pc: u32 = 0x47c1e3;
 loop { match pc {
 0x47c1e3 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47c1ea); f_0041c743(cpu); if cpu.returned_to != 0x47c1ea { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6938u32, v); }
  { let v: u32 = 0x2eu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c22du32; cpu.esi = v; }
  cpu.push(0x47c22b); f_0047b6fe(cpu); if cpu.returned_to != 0x47c22b { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c22d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c22d); }
 let mut pc: u32 = 0x47c22d;
 loop { match pc {
 0x47c22d => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47c234); f_0041c743(cpu); if cpu.returned_to != 0x47c234 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x47c240; continue; }
  pc = 0x47c23a; continue;
 }
 0x47c23a => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47c244; continue;
 }
 0x47c240 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47c244; continue;
 }
 0x47c244 => {
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6938u32, v); }
  { let v: u32 = 0x2eu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c2c1u32; cpu.esi = v; }
  cpu.push(0x47c285); f_0047b6fe(cpu); if cpu.returned_to != 0x47c285 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c2bf); f_0047b6fe(cpu); if cpu.returned_to != 0x47c2bf { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c2c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c2c1); }
 let mut pc: u32 = 0x47c2c1;
 loop { match pc {
 0x47c2c1 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47c2c8); f_0041c743(cpu); if cpu.returned_to != 0x47c2c8 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x47c2d4; continue; }
  pc = 0x47c2ce; continue;
 }
 0x47c2ce => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47c2d8; continue;
 }
 0x47c2d4 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47c2d8; continue;
 }
 0x47c2d8 => {
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x2eu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0x47c351u32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6938u32, v); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x47c315); f_0047b6fe(cpu); if cpu.returned_to != 0x47c315 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c34f); f_0047b6fe(cpu); if cpu.returned_to != 0x47c34f { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c351(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c351); }
 let mut pc: u32 = 0x47c351;
 loop { match pc {
 0x47c351 => {
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x33u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c38a); f_0047b6fe(cpu); if cpu.returned_to != 0x47c38a { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c38c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c38c); }
 let mut pc: u32 = 0x47c38c;
 loop { match pc {
 0x47c38c => {
  { cpu.pushad(); }
  cpu.push(0x47c392); f_0047ca7e(cpu); if cpu.returned_to != 0x47c392 { return; }
  if cpu.cf { pc = 0x47c409; continue; }
  pc = 0x47c394; continue;
 }
 0x47c394 => {
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x7u32); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x1fu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c3d6); f_0047b6fe(cpu); if cpu.returned_to != 0x47c3d6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x2cu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c40bu32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c409); f_0047b6fe(cpu); if cpu.returned_to != 0x47c409 { return; }
  pc = 0x47c409; continue;
 }
 0x47c409 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c40b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c40b); }
 let mut pc: u32 = 0x47c40b;
 loop { match pc {
 0x47c40b => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47c412); f_0041c743(cpu); if cpu.returned_to != 0x47c412 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.and32(cpu.edx, 0x3u32); cpu.edx = v; }
  { let v: u32 = 0x2bu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c45au32; cpu.esi = v; }
  cpu.push(0x47c458); f_0047b6fe(cpu); if cpu.returned_to != 0x47c458 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c45a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47c45a); } r_0047c45a(cpu, 0x47c45a); }
pub fn f_0047c470(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47c470); } r_0047c45a(cpu, 0x47c470); }
fn r_0047c45a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x47c45a => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47c461); f_0041c743(cpu); if cpu.returned_to != 0x47c461 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x47c46d; continue; }
  pc = 0x47c467; continue;
 }
 0x47c467 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47c471; continue;
 }
 0x47c46d => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47c471; continue;
 }
 0x47c470 => {
  { let v: u8 = cpu.r8(0xffe1u32.wrapping_add(cpu.ecx)) & (cpu.eax as u8); cpu.w8(0xffe1u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0xffffff89u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffff89u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax | 0x4d6934u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x2bu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c4efu32; cpu.esi = v; }
  cpu.push(0x47c4b3); f_0047b6fe(cpu); if cpu.returned_to != 0x47c4b3 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1fu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c4ed); f_0047b6fe(cpu); if cpu.returned_to != 0x47c4ed { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47c471 => {
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x2bu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c4efu32; cpu.esi = v; }
  cpu.push(0x47c4b3); f_0047b6fe(cpu); if cpu.returned_to != 0x47c4b3 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1fu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c4ed); f_0047b6fe(cpu); if cpu.returned_to != 0x47c4ed { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c4ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c4ef); }
 let mut pc: u32 = 0x47c4ef;
 loop { match pc {
 0x47c4ef => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47c4f6); f_0041c743(cpu); if cpu.returned_to != 0x47c4f6 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x47c502; continue; }
  pc = 0x47c4fc; continue;
 }
 0x47c4fc => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47c506; continue;
 }
 0x47c502 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47c506; continue;
 }
 0x47c506 => {
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x2bu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c584u32; cpu.esi = v; }
  cpu.push(0x47c548); f_0047b6fe(cpu); if cpu.returned_to != 0x47c548 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1fu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c582); f_0047b6fe(cpu); if cpu.returned_to != 0x47c582 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c584(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c584); }
 let mut pc: u32 = 0x47c584;
 loop { match pc {
 0x47c584 => {
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1fu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c5bd); f_0047b6fe(cpu); if cpu.returned_to != 0x47c5bd { return; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x2cu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c5f2); f_0047b6fe(cpu); if cpu.returned_to != 0x47c5f2 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 6u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x5d4c69u32.wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5d47dcu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x5d47deu32, v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x32u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x1fu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c659); f_0047b6fe(cpu); if cpu.returned_to != 0x47c659 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x32u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x1bu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c736u32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x5d47dcu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x5d47deu32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x47c69b); f_0047b6fe(cpu); if cpu.returned_to != 0x47c69b { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c69d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c69d); }
 let mut pc: u32 = 0x47c69d;
 loop { match pc {
 0x47c69d => {
  cpu.push(0x47c6a2); f_0047ca7e(cpu); if cpu.returned_to != 0x47c6a2 { return; }
  if cpu.cf { pc = 0x47c735; continue; }
  pc = 0x47c6a8; continue;
 }
 0x47c6a8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x5d4c69u32.wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x1fu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c701); f_0047b6fe(cpu); if cpu.returned_to != 0x47c701 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.add32(cpu.ecx, 0x1bu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c736u32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c735); f_0047b6fe(cpu); if cpu.returned_to != 0x47c735 { return; }
  pc = 0x47c735; continue;
 }
 0x47c735 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c736(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c736); }
 let mut pc: u32 = 0x47c736;
 loop { match pc {
 0x47c736 => {
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x5d4c71u32.wrapping_add(cpu.edx.wrapping_mul(2)); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1fu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c785); f_0047b6fe(cpu); if cpu.returned_to != 0x47c785 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c787(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c787); }
 let mut pc: u32 = 0x47c787;
 loop { match pc {
 0x47c787 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x47c78e); f_00420989(cpu); if cpu.returned_to != 0x47c78e { return; }
  cpu.push(0x47c793); f_0047c8f2(cpu); if cpu.returned_to != 0x47c793 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x47c7f0; continue; }
  pc = 0x47c797; continue;
 }
 0x47c797 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c887u32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6938u32, v); }
  cpu.push(0x47c7cc); f_0047b6fe(cpu); if cpu.returned_to != 0x47c7cc { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0xffd2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c7f0); f_0047b6fe(cpu); if cpu.returned_to != 0x47c7f0 { return; }
  pc = 0x47c7f0; continue;
 }
 0x47c7f0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c7f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c7f1); }
 let mut pc: u32 = 0x47c7f1;
 loop { match pc {
 0x47c7f1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x47c7f8); f_00420989(cpu); if cpu.returned_to != 0x47c7f8 { return; }
  cpu.push(0x47c7fd); f_0047c8f2(cpu); if cpu.returned_to != 0x47c7fd { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x47c886; continue; }
  pc = 0x47c805; continue;
 }
 0x47c805 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub16((cpu.ecx as u16), 0x1u16); }
  if cpu.zf { pc = 0x47c81d; continue; }
  pc = 0x47c80d; continue;
 }
 0x47c80d => {
  { let v: u32 = 0x26u32; cpu.ecx = v; }
  { let v: u16 = 0x40u16; cpu.w16(0x4d6934u32, v); }
  pc = 0x47c82b; continue;
 }
 0x47c81d => {
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u16 = 0xc0u16; cpu.w16(0x4d6934u32, v); }
  pc = 0x47c82b; continue;
 }
 0x47c82b => {
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x47c887u32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6938u32, v); }
  cpu.push(0x47c850); f_0047b6fe(cpu); if cpu.returned_to != 0x47c850 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0xffd2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u8 = 0x30u8; cpu.w8(0x4d6938u32, v); }
  cpu.push(0x47c886); f_0047b6fe(cpu); if cpu.returned_to != 0x47c886 { return; }
  pc = 0x47c886; continue;
 }
 0x47c886 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c887(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c887); }
 let mut pc: u32 = 0x47c887;
 loop { match pc {
 0x47c887 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6938u32, v); }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  cpu.push(0x47c8ba); f_0047b6fe(cpu); if cpu.returned_to != 0x47c8ba { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6936u32, v); }
  { let v: u8 = 0x30u8; cpu.w8(0x4d6938u32, v); }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  cpu.push(0x47c8f0); f_0047b6fe(cpu); if cpu.returned_to != 0x47c8f0 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c8f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c8f2); }
 let mut pc: u32 = 0x47c8f2;
 loop { match pc {
 0x47c8f2 => {
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47c908; continue; }
  pc = 0x47c8fe; continue;
 }
 0x47c8fe => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x47c908; continue; }
  pc = 0x47c902; continue;
 }
 0x47c902 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x47c908; continue; }
  pc = 0x47c906; continue;
 }
 0x47c906 => {
  pc = 0x47c960; continue;
 }
 0x47c908 => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47c91e; continue; }
  pc = 0x47c914; continue;
 }
 0x47c914 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x47c91e; continue; }
  pc = 0x47c918; continue;
 }
 0x47c918 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x47c91e; continue; }
  pc = 0x47c91c; continue;
 }
 0x47c91c => {
  pc = 0x47c960; continue;
 }
 0x47c91e => {
  { let v: u8 = cpu.r8(0x3fffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47c934; continue; }
  pc = 0x47c92a; continue;
 }
 0x47c92a => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x47c934; continue; }
  pc = 0x47c92e; continue;
 }
 0x47c92e => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x47c934; continue; }
  pc = 0x47c932; continue;
 }
 0x47c932 => {
  pc = 0x47c960; continue;
 }
 0x47c934 => {
  { let v: u8 = cpu.r8(0x3f80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47c94a; continue; }
  pc = 0x47c940; continue;
 }
 0x47c940 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x47c94a; continue; }
  pc = 0x47c944; continue;
 }
 0x47c944 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x47c94a; continue; }
  pc = 0x47c948; continue;
 }
 0x47c948 => {
  pc = 0x47c960; continue;
 }
 0x47c94a => {
  { let v: u8 = cpu.r8(0x4080u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47c95e; continue; }
  pc = 0x47c956; continue;
 }
 0x47c956 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x47c95e; continue; }
  pc = 0x47c95a; continue;
 }
 0x47c95a => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x47c960; continue; }
  pc = 0x47c95e; continue;
 }
 0x47c95e => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47c960 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c962(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c962); }
 let mut pc: u32 = 0x47c962;
 loop { match pc {
 0x47c962 => {
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47c978; continue; }
  pc = 0x47c96e; continue;
 }
 0x47c96e => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x47c978; continue; }
  pc = 0x47c972; continue;
 }
 0x47c972 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x47c978; continue; }
  pc = 0x47c976; continue;
 }
 0x47c976 => {
  pc = 0x47c9ec; continue;
 }
 0x47c978 => {
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if cpu.zf { pc = 0x47c9b2; continue; }
  pc = 0x47c97e; continue;
 }
 0x47c97e => {
  { cpu.sub16((cpu.ecx as u16), 0x2u16); }
  if cpu.zf { pc = 0x47c9c6; continue; }
  pc = 0x47c984; continue;
 }
 0x47c984 => {
  { cpu.sub16((cpu.ecx as u16), 0x3u16); }
  if cpu.zf { pc = 0x47c99e; continue; }
  pc = 0x47c98a; continue;
 }
 0x47c98a => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47c9da; continue; }
  pc = 0x47c996; continue;
 }
 0x47c996 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x47c9da; continue; }
  pc = 0x47c99a; continue;
 }
 0x47c99a => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x47c9ec; continue; }
  pc = 0x47c99e; continue;
 }
 0x47c99e => {
  { let v: u8 = cpu.r8(0x3fffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47c9da; continue; }
  pc = 0x47c9aa; continue;
 }
 0x47c9aa => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x47c9da; continue; }
  pc = 0x47c9ae; continue;
 }
 0x47c9ae => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x47c9ec; continue; }
  pc = 0x47c9b2; continue;
 }
 0x47c9b2 => {
  { let v: u8 = cpu.r8(0x3f80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47c9da; continue; }
  pc = 0x47c9be; continue;
 }
 0x47c9be => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x47c9da; continue; }
  pc = 0x47c9c2; continue;
 }
 0x47c9c2 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x47c9ec; continue; }
  pc = 0x47c9c6; continue;
 }
 0x47c9c6 => {
  { let v: u8 = cpu.r8(0x4080u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47c9da; continue; }
  pc = 0x47c9d2; continue;
 }
 0x47c9d2 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x47c9da; continue; }
  pc = 0x47c9d6; continue;
 }
 0x47c9d6 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x47c9ec; continue; }
  pc = 0x47c9da; continue;
 }
 0x47c9da => {
  { cpu.sub8(cpu.r8(0x57556eu32), 0x0u8); }
  if cpu.zf { pc = 0x47c9ea; continue; }
  pc = 0x47c9e3; continue;
 }
 0x47c9e3 => {
  cpu.push(0x47c9e8); f_0047c9ee(cpu); if cpu.returned_to != 0x47c9e8 { return; }
  if cpu.cf { pc = 0x47c9ec; continue; }
  pc = 0x47c9ea; continue;
 }
 0x47c9ea => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47c9ec => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047c9ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47c9ee); }
 let mut pc: u32 = 0x47c9ee;
 loop { match pc {
 0x47c9ee => {
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if cpu.zf { pc = 0x47ca3c; continue; }
  pc = 0x47c9f4; continue;
 }
 0x47c9f4 => {
  { cpu.sub16((cpu.ecx as u16), 0x2u16); }
  if cpu.zf { pc = 0x47ca28; continue; }
  pc = 0x47c9fa; continue;
 }
 0x47c9fa => {
  { cpu.sub16((cpu.ecx as u16), 0x3u16); }
  if cpu.zf { pc = 0x47ca00; continue; }
  pc = 0x47ca00; continue;
 }
 0x47ca00 => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47ca50; continue; }
  pc = 0x47ca0c; continue;
 }
 0x47ca0c => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x47ca50; continue; }
  pc = 0x47ca10; continue;
 }
 0x47ca10 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x47ca52; continue; }
  pc = 0x47ca14; continue;
 }
 0x47ca14 => {
  { let v: u8 = cpu.r8(0x3fffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47ca50; continue; }
  pc = 0x47ca20; continue;
 }
 0x47ca20 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x47ca50; continue; }
  pc = 0x47ca24; continue;
 }
 0x47ca24 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x47ca52; continue; }
  pc = 0x47ca28; continue;
 }
 0x47ca28 => {
  { let v: u8 = cpu.r8(0x3f80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47ca50; continue; }
  pc = 0x47ca34; continue;
 }
 0x47ca34 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x47ca50; continue; }
  pc = 0x47ca38; continue;
 }
 0x47ca38 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x47ca52; continue; }
  pc = 0x47ca3c; continue;
 }
 0x47ca3c => {
  { let v: u8 = cpu.r8(0x4080u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47ca50; continue; }
  pc = 0x47ca48; continue;
 }
 0x47ca48 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x47ca50; continue; }
  pc = 0x47ca4c; continue;
 }
 0x47ca4c => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x47ca52; continue; }
  pc = 0x47ca50; continue;
 }
 0x47ca50 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47ca52 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ca54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ca54); }
 let mut pc: u32 = 0x47ca54;
 loop { match pc {
 0x47ca54 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x4dd54cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x47ca79; continue; }
  pc = 0x47ca74; continue;
 }
 0x47ca74 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47ca79 => {
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ca7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ca7e); }
 let mut pc: u32 = 0x47ca7e;
 loop { match pc {
 0x47ca7e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x47ca86); f_00420989(cpu); if cpu.returned_to != 0x47ca86 { return; }
  cpu.push(0x47ca8b); f_0047c962(cpu); if cpu.returned_to != 0x47ca8b { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ca8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ca8f); }
 let mut pc: u32 = 0x47ca8f;
 loop { match pc {
 0x47ca8f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x47ca97); f_00420989(cpu); if cpu.returned_to != 0x47ca97 { return; }
  cpu.push(0x47ca9c); f_0047c8f2(cpu); if cpu.returned_to != 0x47ca9c { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047cabe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47cabe); }
 let mut pc: u32 = 0x47cabe;
 loop { match pc {
 0x47cabe => {
  { let v: u32 = 0x5293a0u32; cpu.edi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x47cacd; continue;
 }
 0x47cacd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x40u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47cacd; continue; }
  pc = 0x47cad5; continue;
 }
 0x47cad5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047cad6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47cad6); }
 let mut pc: u32 = 0x47cad6;
 loop { match pc {
 0x47cad6 => {
  { let v: u32 = 0x52aca0u32; cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x47cae5; continue;
 }
 0x47cae5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0xau32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47cae5; continue; }
  pc = 0x47caed; continue;
 }
 0x47caed => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047caee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47caee); }
 let mut pc: u32 = 0x47caee;
 loop { match pc {
 0x47caee => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x52aca0u32; cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x47caff; continue;
 }
 0x47caff => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x47cb0d; continue; }
  pc = 0x47cb04; continue;
 }
 0x47cb04 => {
  { let v: u32 = cpu.add32(cpu.edi, 0xau32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47caff; continue; }
  pc = 0x47cb09; continue;
 }
 0x47cb09 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47cb0d => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047cb11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47cb11); }
 let mut pc: u32 = 0x47cb11;
 loop { match pc {
 0x47cb11 => {
  { cpu.pushad(); }
  cpu.push(0x47cb17); f_0047caee(cpu); if cpu.returned_to != 0x47cb17 { return; }
  if cpu.cf { pc = 0x47cb26; continue; }
  pc = 0x47cb19; continue;
 }
 0x47cb19 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47cb26 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047cb29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47cb29); }
 let mut pc: u32 = 0x47cb29;
 loop { match pc {
 0x47cb29 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x5293a0u32; cpu.edi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x47cb3a; continue;
 }
 0x47cb3a => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x47cb48; continue; }
  pc = 0x47cb3f; continue;
 }
 0x47cb3f => {
  { let v: u32 = cpu.add32(cpu.edi, 0x40u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47cb3a; continue; }
  pc = 0x47cb44; continue;
 }
 0x47cb44 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47cb48 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047cb4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47cb4c); }
 let mut pc: u32 = 0x47cb4c;
 loop { match pc {
 0x47cb4c => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.esi)), 0xfu8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47cbaa; continue; }
  pc = 0x47cb53; continue;
 }
 0x47cb53 => {
  { let v: u32 = 0x52ae94u32; cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.sf { pc = 0x47cbaa; continue; }
  pc = 0x47cb64; continue;
 }
 0x47cb64 => {
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x47cbaa; continue; }
  pc = 0x47cb6a; continue;
 }
 0x47cb6a => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x47cba4; continue; }
  pc = 0x47cb71; continue;
 }
 0x47cb71 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1000u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47cba4; continue; }
  pc = 0x47cb9d; continue;
 }
 0x47cb9d => {
  { let v: u8 = cpu.or8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x81u8); cpu.w8(0x35u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = true; }
  pc = 0x47cbaa; continue;
 }
 0x47cba4 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x3cu32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47cb6a; continue; }
  pc = 0x47cba9; continue;
 }
 0x47cba9 => {
  { cpu.cf = false; }
  pc = 0x47cbaa; continue;
 }
 0x47cbaa => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047cbac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47cbac); }
 let mut pc: u32 = 0x47cbac;
 loop { match pc {
 0x47cbac => {
  { cpu.pushad(); }
  { let v: u32 = 0x5293a0u32; cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  pc = 0x47cbb7; continue;
 }
 0x47cbb7 => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x47cc33; continue; }
  pc = 0x47cbbf; continue;
 }
 0x47cbbf => {
  { cpu.sub8(cpu.r8(0x27u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x47cc33; continue; }
  pc = 0x47cbc5; continue;
 }
 0x47cbc5 => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x30u8); }
  if !cpu.zf { pc = 0x47cc33; continue; }
  pc = 0x47cbcb; continue;
 }
 0x47cbcb => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.zf { pc = 0x47cc33; continue; }
  pc = 0x47cbd1; continue;
 }
 0x47cbd1 => {
  { cpu.sub8(cpu.r8(0x20u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x47cc33; continue; }
  pc = 0x47cbd7; continue;
 }
 0x47cbd7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x47cbed; continue; }
  pc = 0x47cbea; continue;
 }
 0x47cbea => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47cbed; continue;
 }
 0x47cbed => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x47cbf6; continue; }
  pc = 0x47cbf3; continue;
 }
 0x47cbf3 => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x47cbf6; continue;
 }
 0x47cbf6 => {
  { cpu.sub16((cpu.ecx as u16), 0x18u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47cc33; continue; }
  pc = 0x47cbfc; continue;
 }
 0x47cbfc => {
  { cpu.sub16((cpu.edx as u16), 0x18u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47cc33; continue; }
  pc = 0x47cc02; continue;
 }
 0x47cc02 => {
  { let v: u8 = 0x0u8; cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x35u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x36u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x21u32.wrapping_add(cpu.esi)) as u32; cpu.edx = v; }
  { cpu.and16((cpu.ecx as u16), (cpu.ecx as u16)); }
  if !cpu.sf { pc = 0x47cc2e; continue; }
  pc = 0x47cc2b; continue;
 }
 0x47cc2b => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47cc2e; continue;
 }
 0x47cc2e => {
  cpu.push(0x47cc33); f_0047ba2b(cpu); if cpu.returned_to != 0x47cc33 { return; }
  pc = 0x47cc33; continue;
 }
 0x47cc33 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47cbb7; continue; }
  pc = 0x47cc3e; continue;
 }
 0x47cc3e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047cc40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47cc40); }
 let mut pc: u32 = 0x47cc40;
 loop { match pc {
 0x47cc40 => {
  { cpu.pushad(); }
  { let v: u32 = 0x5293a0u32; cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  pc = 0x47cc4b; continue;
 }
 0x47cc4b => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x47ccd1; continue; }
  pc = 0x47cc53; continue;
 }
 0x47cc53 => {
  { cpu.sub8(cpu.r8(0x27u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x47ccd1; continue; }
  pc = 0x47cc59; continue;
 }
 0x47cc59 => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x30u8); }
  if !cpu.zf { pc = 0x47ccd1; continue; }
  pc = 0x47cc5f; continue;
 }
 0x47cc5f => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.zf { pc = 0x47ccd1; continue; }
  pc = 0x47cc65; continue;
 }
 0x47cc65 => {
  { cpu.sub8(cpu.r8(0x20u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x47ccd1; continue; }
  pc = 0x47cc6b; continue;
 }
 0x47cc6b => {
  { cpu.sub8(cpu.r8(0x20u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x47ccd1; continue; }
  pc = 0x47cc71; continue;
 }
 0x47cc71 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x47cc87; continue; }
  pc = 0x47cc84; continue;
 }
 0x47cc84 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47cc87; continue;
 }
 0x47cc87 => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x47cc90; continue; }
  pc = 0x47cc8d; continue;
 }
 0x47cc8d => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x47cc90; continue;
 }
 0x47cc90 => {
  { cpu.sub16((cpu.ecx as u16), 0x10u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47ccd1; continue; }
  pc = 0x47cc96; continue;
 }
 0x47cc96 => {
  { cpu.sub16((cpu.edx as u16), 0x10u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47ccd1; continue; }
  pc = 0x47cc9c; continue;
 }
 0x47cc9c => {
  { let v: u8 = 0x0u8; cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x35u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x36u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x21u32.wrapping_add(cpu.esi)) as u32; cpu.edx = v; }
  { cpu.and16((cpu.ecx as u16), (cpu.ecx as u16)); }
  if !cpu.sf { pc = 0x47ccc8; continue; }
  pc = 0x47ccc5; continue;
 }
 0x47ccc5 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47ccc8; continue;
 }
 0x47ccc8 => {
  cpu.push(0x47cccd); f_0047ba2b(cpu); if cpu.returned_to != 0x47cccd { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47ccd1 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47cc4b; continue; }
  pc = 0x47ccdc; continue;
 }
 0x47ccdc => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047cd44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47cd44); }
 let mut pc: u32 = 0x47cd44;
 loop { match pc {
 0x47cd44 => {
  { let v: u32 = 0x5293a0u32; cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  pc = 0x47cd54; continue;
 }
 0x47cd54 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x47cd64; continue; }
  pc = 0x47cd5b; continue;
 }
 0x47cd5b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x47cd62); f_0047cd70(cpu); if cpu.returned_to != 0x47cd62 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x47cd64; continue;
 }
 0x47cd64 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x40u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47cd54; continue; }
  pc = 0x47cd69; continue;
 }
 0x47cd69 => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047cd70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47cd70); }
 let mut pc: u32 = 0x47cd70;
 loop { match pc {
 0x47cd70 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2bu32.wrapping_add(cpu.esi), v); }
  { cpu.sub16(cpu.r16(0x27u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x47cd9a; continue; }
  pc = 0x47cd91; continue;
 }
 0x47cd91 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x27u32.wrapping_add(cpu.esi))); cpu.w16(0x27u32.wrapping_add(cpu.esi), v); }
  pc = 0x47d04b; continue;
 }
 0x47cd9a => {
  cpu.push(0x47cd9f); f_0047d3ad(cpu); if cpu.returned_to != 0x47cd9f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x20u8); }
  if !cpu.zf { pc = 0x47d04b; continue; }
  pc = 0x47cdad; continue;
 }
 0x47cdad => {
  cpu.push(0x47cdb2); f_0047d675(cpu); if cpu.returned_to != 0x47cdb2 { return; }
  cpu.push(0x47cdb7); f_0047d378(cpu); if cpu.returned_to != 0x47cdb7 { return; }
  if cpu.cf { pc = 0x47d04b; continue; }
  pc = 0x47cdbd; continue;
 }
 0x47cdbd => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x40u8); }
  if !cpu.zf { pc = 0x47ce09; continue; }
  pc = 0x47cdc3; continue;
 }
 0x47cdc3 => {
  cpu.push(0x47cdc8); f_0047d346(cpu); if cpu.returned_to != 0x47cdc8 { return; }
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2du32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.esi)) & 0xfeu8; cpu.w8(0x36u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) & 0xffu8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x2du32.wrapping_add(cpu.esi)) & 0xffu8; cpu.w8(0x2du32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x47cdee; continue; }
  pc = 0x47cde0; continue;
 }
 0x47cde0 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3au32.wrapping_add(cpu.esi))); cpu.w8(0x3au32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x47d04b; continue; }
  pc = 0x47cde9; continue;
 }
 0x47cde9 => {
  pc = 0x47ce9f; continue;
 }
 0x47cdee => {
  cpu.push(0x47cdf3); f_0047d31e(cpu); if cpu.returned_to != 0x47cdf3 { return; }
  cpu.push(0x47cdf8); f_0047d2aa(cpu); if cpu.returned_to != 0x47cdf8 { return; }
  if cpu.cf { pc = 0x47d04b; continue; }
  pc = 0x47cdfe; continue;
 }
 0x47cdfe => {
  cpu.push(0x47ce03); f_0047d276(cpu); if cpu.returned_to != 0x47ce03 { return; }
  if cpu.cf { pc = 0x47d04b; continue; }
  pc = 0x47ce09; continue;
 }
 0x47ce09 => {
  cpu.push(0x47ce0e); f_0047d22f(cpu); if cpu.returned_to != 0x47ce0e { return; }
  if cpu.cf { pc = 0x47d04b; continue; }
  pc = 0x47ce14; continue;
 }
 0x47ce14 => {
  cpu.push(0x47ce19); f_0047d1d3(cpu); if cpu.returned_to != 0x47ce19 { return; }
  if cpu.cf { pc = 0x47d04b; continue; }
  pc = 0x47ce1f; continue;
 }
 0x47ce1f => {
  cpu.push(0x47ce24); f_0047d214(cpu); if cpu.returned_to != 0x47ce24 { return; }
  cpu.push(0x47ce29); f_0047d1be(cpu); if cpu.returned_to != 0x47ce29 { return; }
  if cpu.cf { pc = 0x47ce4a; continue; }
  pc = 0x47ce2b; continue;
 }
 0x47ce2b => {
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x47d04b; continue; }
  pc = 0x47ce35; continue;
 }
 0x47ce35 => {
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.esi)) & 0xbfu8; cpu.w8(0x36u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x47ce7f; continue; }
  pc = 0x47ce43; continue;
 }
 0x47ce43 => {
  cpu.push(0x47ce48); f_0047d11a(cpu); if cpu.returned_to != 0x47ce48 { return; }
  if !cpu.cf { pc = 0x47ce76; continue; }
  pc = 0x47ce4a; continue;
 }
 0x47ce4a => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x47cf93; continue; }
  pc = 0x47ce54; continue;
 }
 0x47ce54 => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)) & 0x7eu8; cpu.w8(0x35u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x47ce5d); f_0041c743(cpu); if cpu.returned_to != 0x47ce5d { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x27u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0xfu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x47d04b; continue;
 }
 0x47ce76 => {
  cpu.push(0x47ce7b); f_0047d151(cpu); if cpu.returned_to != 0x47ce7b { return; }
  if cpu.cf { cpu.poll(); pc = 0x47ce4a; continue; }
  pc = 0x47ce7d; continue;
 }
 0x47ce7d => {
  pc = 0x47ce95; continue;
 }
 0x47ce7f => {
  cpu.push(0x47ce84); f_0047d79a(cpu); if cpu.returned_to != 0x47ce84 { return; }
  if !cpu.cf { pc = 0x47ce8c; continue; }
  pc = 0x47ce86; continue;
 }
 0x47ce86 => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)) | 0x81u8; cpu.w8(0x35u32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x47ce35; continue;
 }
 0x47ce8c => {
  cpu.push(0x47ce91); f_0047cb4c(cpu); if cpu.returned_to != 0x47ce91 { return; }
  if !cpu.cf { pc = 0x47ce9f; continue; }
  pc = 0x47ce93; continue;
 }
 0x47ce93 => {
  cpu.poll(); pc = 0x47ce35; continue;
 }
 0x47ce95 => {
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x2du32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  pc = 0x47cecb; continue;
 }
 0x47ce9f => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x47d04b; continue; }
  pc = 0x47cea9; continue;
 }
 0x47cea9 => {
  cpu.push(0x47ceae); f_0041c743(cpu); if cpu.returned_to != 0x47ceae { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47cebe; continue; }
  pc = 0x47ceb4; continue;
 }
 0x47ceb4 => {
  { let v: u16 = cpu.shl16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x47cecb; continue; }
  pc = 0x47cebe; continue;
 }
 0x47cebe => {
  cpu.push(0x47cec3); f_0047d090(cpu); if cpu.returned_to != 0x47cec3 { return; }
  if cpu.cf { pc = 0x47d04b; continue; }
  pc = 0x47cec9; continue;
 }
 0x47cec9 => {
  pc = 0x47cedb; continue;
 }
 0x47cecb => {
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.esi)) & 0xfdu8; cpu.w8(0x36u32.wrapping_add(cpu.esi), v); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.sf { pc = 0x47cedb; continue; }
  pc = 0x47ced4; continue;
 }
 0x47ced4 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.esi)) | 0x2u8; cpu.w8(0x36u32.wrapping_add(cpu.esi), v); }
  pc = 0x47cedb; continue;
 }
 0x47cedb => {
  { let v: u8 = cpu.inc8(cpu.r8(0x22u32.wrapping_add(cpu.esi))); cpu.w8(0x22u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x22u32.wrapping_add(cpu.esi)), 0xau8); }
  if !cpu.zf { pc = 0x47ceee; continue; }
  pc = 0x47cee4; continue;
 }
 0x47cee4 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x47d04b; continue;
 }
 0x47ceee => {
  { let v: u8 = cpu.r8(0x2du32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x47cefb; continue; }
  pc = 0x47cef7; continue;
 }
 0x47cef7 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x47cefd; continue;
 }
 0x47cefb => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x47cefd; continue;
 }
 0x47cefd => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x2du32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = (cpu.ebx as u8) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { let v: u32 = 0x5d47acu32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as i8 as i16 as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 4u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x3du32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x47cf41); f_00420989(cpu); if cpu.returned_to != 0x47cf41 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x47cf5e; continue; }
  pc = 0x47cf4e; continue;
 }
 0x47cf4e => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47d003; continue; }
  pc = 0x47cf56; continue;
 }
 0x47cf56 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x47d003; continue; }
  pc = 0x47cf5e; continue;
 }
 0x47cf5e => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4ea020u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x47d003; continue; }
  pc = 0x47cf70; continue;
 }
 0x47cf70 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x47cf84; continue; }
  pc = 0x47cf7a; continue;
 }
 0x47cf7a => {
  { cpu.sub8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x47cf82; continue; }
  pc = 0x47cf7e; continue;
 }
 0x47cf7e => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x47cf84; continue; }
  pc = 0x47cf82; continue;
 }
 0x47cf82 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x47cf84; continue;
 }
 0x47cf84 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.poll(); pc = 0x47cedb; continue;
 }
 0x47cf93 => {
  { cpu.pushad(); }
  { let v: u16 = 0x20u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x40u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x47cfa1); f_0041c808(cpu); if cpu.returned_to != 0x47cfa1 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x47cfb0); f_0047badd(cpu); if cpu.returned_to != 0x47cfb0 { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x2du32.wrapping_add(cpu.esi)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x12u32.wrapping_add(cpu.esi)); cpu.push(0x47cfc5); cpu.call(t); if cpu.returned_to != 0x47cfc5 { return; }
  pc = 0x47d04b; continue;
 }
 0x47d003 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x8u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(cpu.r16(0xau32.wrapping_add(cpu.esi))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x47d018; continue; }
  pc = 0x47d015; continue;
 }
 0x47d015 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47d018; continue;
 }
 0x47d018 => {
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x47d021; continue; }
  pc = 0x47d01e; continue;
 }
 0x47d01e => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47d021; continue;
 }
 0x47d021 => {
  { cpu.sub16((cpu.ebx as u16), 0x50u16); }
  if (cpu.sf != cpu.of) { pc = 0x47d02b; continue; }
  pc = 0x47d027; continue;
 }
 0x47d027 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x47d03a; continue;
 }
 0x47d02b => {
  { cpu.sub16((cpu.ecx as u16), 0x50u16); }
  if (cpu.sf != cpu.of) { pc = 0x47d035; continue; }
  pc = 0x47d031; continue;
 }
 0x47d031 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x47d03a; continue;
 }
 0x47d035 => {
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x47d043; continue;
 }
 0x47d03a => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x47cedb; continue;
 }
 0x47d043 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.sf { pc = 0x47d04b; continue; }
  pc = 0x47d04b; continue;
 }
 0x47d04b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d090(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d090); }
 let mut pc: u32 = 0x47d090;
 loop { match pc {
 0x47d090 => {
  { cpu.sub16(cpu.r16(0x1au32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x47d112; continue; }
  pc = 0x47d098; continue;
 }
 0x47d098 => {
  { let v = cpu.eax; cpu.push(v); }
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x47d0eb; continue; }
  pc = 0x47d09f; continue;
 }
 0x47d09f => {
  cpu.push(0x47d0a4); f_0041c743(cpu); if cpu.returned_to != 0x47d0a4 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0xfu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47d0eb; continue; }
  pc = 0x47d0aa; continue;
 }
 0x47d0aa => {
  { cpu.sub8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x47d0b8; continue; }
  pc = 0x47d0b0; continue;
 }
 0x47d0b0 => {
  { cpu.sub8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x47d0b8; continue; }
  pc = 0x47d0b6; continue;
 }
 0x47d0b6 => {
  pc = 0x47d0eb; continue;
 }
 0x47d0b8 => {
  { let v: u8 = cpu.r8(0x2du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3fu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x47d0eb; continue; }
  pc = 0x47d0bf; continue;
 }
 0x47d0bf => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.esi)) | 0x40u8; cpu.w8(0x36u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0x2u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x47d0d1); f_0041c808(cpu); if cpu.returned_to != 0x47d0d1 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x6u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x6u8; cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x47d0e2); f_0041c743(cpu); if cpu.returned_to != 0x47d0e2 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  pc = 0x47d108; continue;
 }
 0x47d0eb => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x47d10a; continue; }
  pc = 0x47d0f2; continue;
 }
 0x47d0f2 => {
  cpu.push(0x47d0f7); f_0041c743(cpu); if cpu.returned_to != 0x47d0f7 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0xfu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47d10a; continue; }
  pc = 0x47d0fd; continue;
 }
 0x47d0fd => {
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3au32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  pc = 0x47d108; continue;
 }
 0x47d108 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d10a => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d112 => {
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0xfdu8); cpu.w8(0x36u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d11a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d11a); }
 let mut pc: u32 = 0x47d11a;
 loop { match pc {
 0x47d11a => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.sf { pc = 0x47d134; continue; }
  pc = 0x47d131; continue;
 }
 0x47d131 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x47d134; continue;
 }
 0x47d134 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x47d13d; continue; }
  pc = 0x47d13a; continue;
 }
 0x47d13a => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47d13d; continue;
 }
 0x47d13d => {
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47d14d; continue; }
  pc = 0x47d143; continue;
 }
 0x47d143 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47d14d; continue; }
  pc = 0x47d149; continue;
 }
 0x47d149 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d14d => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d151(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d151); }
 let mut pc: u32 = 0x47d151;
 loop { match pc {
 0x47d151 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.sf { pc = 0x47d15f; continue; }
  pc = 0x47d15c; continue;
 }
 0x47d15c => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x47d15f; continue;
 }
 0x47d15f => {
  { cpu.and16((cpu.ebx as u16), (cpu.ebx as u16)); }
  if !cpu.sf { pc = 0x47d167; continue; }
  pc = 0x47d164; continue;
 }
 0x47d164 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47d167; continue;
 }
 0x47d167 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x47d190; continue; }
  pc = 0x47d16c; continue;
 }
 0x47d16c => {
  if (cpu.sf != cpu.of) { pc = 0x47d17f; continue; }
  pc = 0x47d16e; continue;
 }
 0x47d16e => {
  { cpu.and16((cpu.ecx as u16), (cpu.ecx as u16)); }
  if !cpu.sf { pc = 0x47d179; continue; }
  pc = 0x47d173; continue;
 }
 0x47d173 => {
  { let v: u8 = 0xc0u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x47d1ba; continue;
 }
 0x47d179 => {
  { let v: u8 = 0x40u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x47d1ba; continue;
 }
 0x47d17f => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if !cpu.sf { pc = 0x47d18a; continue; }
  pc = 0x47d184; continue;
 }
 0x47d184 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x47d1ba; continue;
 }
 0x47d18a => {
  { let v: u8 = 0x80u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x47d1ba; continue;
 }
 0x47d190 => {
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if cpu.zf { pc = 0x47d1bc; continue; }
  pc = 0x47d196; continue;
 }
 0x47d196 => {
  if !cpu.sf { pc = 0x47d1aa; continue; }
  pc = 0x47d198; continue;
 }
 0x47d198 => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x47d1a4; continue; }
  pc = 0x47d19e; continue;
 }
 0x47d19e => {
  { let v: u8 = 0xe0u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x47d1ba; continue;
 }
 0x47d1a4 => {
  { let v: u8 = 0xa0u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x47d1ba; continue;
 }
 0x47d1aa => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x47d1b6; continue; }
  pc = 0x47d1b0; continue;
 }
 0x47d1b0 => {
  { let v: u8 = 0x20u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x47d1ba; continue;
 }
 0x47d1b6 => {
  { let v: u8 = 0x60u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x47d1ba; continue;
 }
 0x47d1ba => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d1bc => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d1be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d1be); }
 let mut pc: u32 = 0x47d1be;
 loop { match pc {
 0x47d1be => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x47d1d1; continue; }
  pc = 0x47d1c4; continue;
 }
 0x47d1c4 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x47d1d0); f_0047d4ee(cpu); if cpu.returned_to != 0x47d1d0 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d1d1 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d1d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d1d3); }
 let mut pc: u32 = 0x47d1d3;
 loop { match pc {
 0x47d1d3 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x47d212; continue; }
  pc = 0x47d1d9; continue;
 }
 0x47d1d9 => {
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x47d212; continue; }
  pc = 0x47d1df; continue;
 }
 0x47d1df => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x47d1ec); f_00420989(cpu); if cpu.returned_to != 0x47d1ec { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x47d1fd; continue; }
  pc = 0x47d1f9; continue;
 }
 0x47d1f9 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if !cpu.zf { pc = 0x47d212; continue; }
  pc = 0x47d1fd; continue;
 }
 0x47d1fd => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x47d20a); f_00485e18(cpu); if cpu.returned_to != 0x47d20a { return; }
  { cpu.popad(); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d212 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d214(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d214); }
 let mut pc: u32 = 0x47d214;
 loop { match pc {
 0x47d214 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.esi))); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x47d22a; continue; }
  pc = 0x47d219; continue;
 }
 0x47d219 => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x40u8); }
  if !cpu.zf { pc = 0x47d22e; continue; }
  pc = 0x47d21f; continue;
 }
 0x47d21f => {
  { cpu.and8(cpu.r8(0x37u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x47d22e; continue; }
  pc = 0x47d225; continue;
 }
 0x47d225 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.esi))); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x47d22e; continue; }
  pc = 0x47d22a; continue;
 }
 0x47d22a => {
  { let v: u8 = 0x10u8; cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  pc = 0x47d22e; continue;
 }
 0x47d22e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d22f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d22f); }
 let mut pc: u32 = 0x47d22f;
 loop { match pc {
 0x47d22f => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x47d241; continue; }
  pc = 0x47d235; continue;
 }
 0x47d235 => {
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x47d274; continue; }
  pc = 0x47d23b; continue;
 }
 0x47d23b => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x40u8); }
  if !cpu.zf { pc = 0x47d272; continue; }
  pc = 0x47d241; continue;
 }
 0x47d241 => {
  { let v: u32 = cpu.r8(0x2du32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x5d47acu32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as i8 as i16 as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x37u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x47d26a; continue; }
  pc = 0x47d264; continue;
 }
 0x47d264 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47d26a; continue;
 }
 0x47d26a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)).wrapping_add((cpu.ebx as u16)); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.add16(cpu.r16(0x2u32.wrapping_add(cpu.esi)), (cpu.ecx as u16)); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  pc = 0x47d272; continue;
 }
 0x47d272 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d274 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d276(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d276); }
 let mut pc: u32 = 0x47d276;
 loop { match pc {
 0x47d276 => {
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x47d28c; continue; }
  pc = 0x47d27c; continue;
 }
 0x47d27c => {
  { let v: u8 = cpu.dec8(cpu.r8(0x2eu32.wrapping_add(cpu.esi))); cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x21u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x47d28e; continue; }
  pc = 0x47d28c; continue;
 }
 0x47d28c => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d28e => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x38u32.wrapping_add(cpu.esi)), 0x7fu8); }
  if !cpu.zf { pc = 0x47d2a8; continue; }
  pc = 0x47d29c; continue;
 }
 0x47d29c => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x2du32.wrapping_add(cpu.esi)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x12u32.wrapping_add(cpu.esi)); cpu.push(0x47d2a8); cpu.call(t); if cpu.returned_to != 0x47d2a8 { return; }
  pc = 0x47d2a8; continue;
 }
 0x47d2a8 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d2aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d2aa); }
 let mut pc: u32 = 0x47d2aa;
 loop { match pc {
 0x47d2aa => {
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x47d2ee; continue; }
  pc = 0x47d2b0; continue;
 }
 0x47d2b0 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x2eu32.wrapping_add(cpu.esi))); cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x47d2df; continue; }
  pc = 0x47d2b9; continue;
 }
 0x47d2b9 => {
  { let v: u16 = cpu.r8(0x2eu32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x16u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x23u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x47d2ee; continue; }
  pc = 0x47d2c8; continue;
 }
 0x47d2c8 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x47d2d9; continue; }
  pc = 0x47d2ce; continue;
 }
 0x47d2ce => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x47d2f0; continue; }
  pc = 0x47d2d4; continue;
 }
 0x47d2d4 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x30u32.wrapping_add(cpu.esi))); cpu.w8(0x30u32.wrapping_add(cpu.esi), v); }
  if cpu.sf { pc = 0x47d2f0; continue; }
  pc = 0x47d2d9; continue;
 }
 0x47d2d9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  pc = 0x47d2ee; continue;
 }
 0x47d2df => {
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x47d2ee; continue; }
  pc = 0x47d2e7; continue;
 }
 0x47d2e7 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x47d2ec; continue;
 }
 0x47d2ec => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d2ee => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d2f0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x38u32.wrapping_add(cpu.esi)), 0x7fu8); }
  if !cpu.zf { cpu.poll(); pc = 0x47d2ec; continue; }
  pc = 0x47d2fe; continue;
 }
 0x47d2fe => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x2du32.wrapping_add(cpu.esi)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.r32(0x12u32.wrapping_add(cpu.esi)), 0x40cf50u32); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x47d2ec; continue; }
  pc = 0x47d310; continue;
 }
 0x47d310 => {
  { cpu.sub32(cpu.r32(0x12u32.wrapping_add(cpu.esi)), 0x48cd30u32); }
  if !cpu.cf { cpu.poll(); pc = 0x47d2ec; continue; }
  pc = 0x47d319; continue;
 }
 0x47d319 => {
  let t = cpu.r32(0x12u32.wrapping_add(cpu.esi)); cpu.push(0x47d31c); cpu.call(t); if cpu.returned_to != 0x47d31c { return; }
  cpu.poll(); pc = 0x47d2ec; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d31e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d31e); }
 let mut pc: u32 = 0x47d31e;
 loop { match pc {
 0x47d31e => {
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x47d345; continue; }
  pc = 0x47d324; continue;
 }
 0x47d324 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x25u32.wrapping_add(cpu.esi))); cpu.w8(0x25u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x7fu8); }
  if cpu.zf { pc = 0x47d345; continue; }
  pc = 0x47d32e; continue;
 }
 0x47d32e => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x25u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x47d345; continue; }
  pc = 0x47d333; continue;
 }
 0x47d333 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x2du32.wrapping_add(cpu.esi)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x12u32.wrapping_add(cpu.esi)); cpu.push(0x47d345); cpu.call(t); if cpu.returned_to != 0x47d345 { return; }
  pc = 0x47d345; continue;
 }
 0x47d345 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d346(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d346); }
 let mut pc: u32 = 0x47d346;
 loop { match pc {
 0x47d346 => {
  { let v: u16 = cpu.r8(0x2du32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xe0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r8(0x21u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x1cu32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r8(0x21u32.wrapping_add(cpu.esi)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x23u32.wrapping_add(cpu.esi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d378(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d378); }
 let mut pc: u32 = 0x47d378;
 loop { match pc {
 0x47d378 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x2fu32.wrapping_add(cpu.esi))); cpu.w8(0x2fu32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x47d3ab; continue; }
  pc = 0x47d37d; continue;
 }
 0x47d37d => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x40u8); }
  if !cpu.zf { pc = 0x47d390; continue; }
  pc = 0x47d386; continue;
 }
 0x47d386 => {
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x47d3a6; continue; }
  pc = 0x47d38c; continue;
 }
 0x47d38c => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x47d3a6; continue;
 }
 0x47d390 => {
  { cpu.sub8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x47d39c; continue; }
  pc = 0x47d396; continue;
 }
 0x47d396 => {
  { cpu.sub8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x47d3a6; continue; }
  pc = 0x47d39c; continue;
 }
 0x47d39c => {
  { let v: u8 = cpu.shl8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x2eu32.wrapping_add(cpu.esi))); cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), 0x3u8); cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  pc = 0x47d3a6; continue;
 }
 0x47d3a6 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2fu32.wrapping_add(cpu.esi), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d3ab => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d3ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d3ad); }
 let mut pc: u32 = 0x47d3ad;
 loop { match pc {
 0x47d3ad => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x47d3c0; continue; }
  pc = 0x47d3ba; continue;
 }
 0x47d3ba => {
  { let v: u32 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  pc = 0x47d3ec; continue;
 }
 0x47d3c0 => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x47d3d0; continue; }
  pc = 0x47d3c6; continue;
 }
 0x47d3c6 => {
  { let v: u8 = 0x10u8; cpu.w8(0x34u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r16(0x18u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  pc = 0x47d3f2; continue;
 }
 0x47d3d0 => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x47d3e8; continue; }
  pc = 0x47d3d6; continue;
 }
 0x47d3d6 => {
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x2du32.wrapping_add(cpu.esi)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xcu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  pc = 0x47d3ec; continue;
 }
 0x47d3e8 => {
  { let v: u32 = cpu.r16(0x16u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  pc = 0x47d3ec; continue;
 }
 0x47d3ec => {
  { let v: u32 = cpu.r8(0x2eu32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  pc = 0x47d3f2; continue;
 }
 0x47d3f2 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47d47e; continue; }
  pc = 0x47d409; continue;
 }
 0x47d409 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47d47e; continue; }
  pc = 0x47d412; continue;
 }
 0x47d412 => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x4d6b9cu32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xff80u16); }
  if (cpu.sf != cpu.of) { pc = 0x47d47e; continue; }
  pc = 0x47d422; continue;
 }
 0x47d422 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x4d6ba0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xff80u16); }
  if (cpu.sf != cpu.of) { pc = 0x47d47e; continue; }
  pc = 0x47d432; continue;
 }
 0x47d432 => {
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x47d453; continue; }
  pc = 0x47d43b; continue;
 }
 0x47d43b => {
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  if (cpu.sf == cpu.of) { pc = 0x47d453; continue; }
  pc = 0x47d443; continue;
 }
 0x47d443 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x47d45c; continue; }
  pc = 0x47d453; continue;
 }
 0x47d453 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x47d47e; continue;
 }
 0x47d45c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0xcu8); }
  if !cpu.zf { pc = 0x47d47f; continue; }
  pc = 0x47d46d; continue;
 }
 0x47d46d => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x34u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  pc = 0x47d47e; continue;
 }
 0x47d47e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d47f => {
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4d8236u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x34u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x21u8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x47d4d0; continue; }
  pc = 0x47d4c4; continue;
 }
 0x47d4c4 => {
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47d4d0; continue;
 }
 0x47d4d0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x47d4e6; continue; }
  pc = 0x47d4db; continue;
 }
 0x47d4db => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47d4e6; continue;
 }
 0x47d4e6 => {
  { let v: u32 = cpu.add32(cpu.r32(0x4d6ac0u32), 0x10u32); cpu.w32(0x4d6ac0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d4ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d4ee); }
 let mut pc: u32 = 0x47d4ee;
 loop { match pc {
 0x47d4ee => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x52aca0u32; cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x47d506; continue;
 }
 0x47d506 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x47d52c; continue; }
  pc = 0x47d50f; continue;
 }
 0x47d50f => {
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x47d517; continue; }
  pc = 0x47d514; continue;
 }
 0x47d514 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x47d517; continue;
 }
 0x47d517 => {
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47d52c; continue; }
  pc = 0x47d51d; continue;
 }
 0x47d51d => {
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x47d526; continue; }
  pc = 0x47d523; continue;
 }
 0x47d523 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47d526; continue;
 }
 0x47d526 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x47d536; continue; }
  pc = 0x47d52c; continue;
 }
 0x47d52c => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0xau32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47d506; continue; }
  pc = 0x47d533; continue;
 }
 0x47d533 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d536 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d675(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d675); }
 let mut pc: u32 = 0x47d675;
 loop { match pc {
 0x47d675 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x47d691; continue; }
  pc = 0x47d67f; continue;
 }
 0x47d67f => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x47d691; continue; }
  pc = 0x47d686; continue;
 }
 0x47d686 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x47d795; continue; }
  pc = 0x47d691; continue;
 }
 0x47d691 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x47d6b2); f_004400bf(cpu); if cpu.returned_to != 0x47d6b2 { return; }
  { let v: u32 = 0x55755du32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x10u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x10u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d693au32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0xau16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xau16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x6u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x5u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x5u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x5u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6950u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6952u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6954u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6956u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  pc = 0x47d734; continue;
 }
 0x47d734 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x47d797; continue; }
  pc = 0x47d739; continue;
 }
 0x47d739 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x47d757; continue; }
  pc = 0x47d74e; continue;
 }
 0x47d74e => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x47d757; continue; }
  pc = 0x47d755; continue;
 }
 0x47d755 => {
  cpu.poll(); pc = 0x47d734; continue;
 }
 0x47d757 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6934u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x47d734; continue; }
  pc = 0x47d765; continue;
 }
 0x47d765 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6938u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x47d734; continue; }
  pc = 0x47d76e; continue;
 }
 0x47d76e => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6936u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x47d734; continue; }
  pc = 0x47d77c; continue;
 }
 0x47d77c => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d693au32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x47d734; continue; }
  pc = 0x47d785; continue;
 }
 0x47d785 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x36u32.wrapping_add(cpu.ebp)), 0x20u8); cpu.w8(0x36u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7fu8; cpu.w8(0x2fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x47d795; continue;
 }
 0x47d795 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d797 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d79a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d79a); }
 let mut pc: u32 = 0x47d79a;
 loop { match pc {
 0x47d79a => {
  { cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x47d7a7; continue; }
  pc = 0x47d7a1; continue;
 }
 0x47d7a1 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x10u32.wrapping_add(cpu.esi))); cpu.w16(0x10u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d7a7 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x47d7dc; continue; }
  pc = 0x47d7b0; continue;
 }
 0x47d7b0 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x47d7dc; continue; }
  pc = 0x47d7b6; continue;
 }
 0x47d7b6 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.zf { pc = 0x47d7dc; continue; }
  pc = 0x47d7bc; continue;
 }
 0x47d7bc => {
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x47d7dc; continue; }
  pc = 0x47d7c2; continue;
 }
 0x47d7c2 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x47d7d2); f_004400bf(cpu); if cpu.returned_to != 0x47d7d2 { return; }
  { let v: u32 = 0x55755du32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { pc = 0x47d7df; continue; }
  pc = 0x47d7dc; continue;
 }
 0x47d7dc => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d7df => {
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d7e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d7e2); }
 let mut pc: u32 = 0x47d7e2;
 loop { match pc {
 0x47d7e2 => {
  { cpu.pushad(); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x4d69bau32), 0x0u16); }
  if cpu.zf { pc = 0x47d8a9; continue; }
  pc = 0x47d7f2; continue;
 }
 0x47d7f2 => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x3u8); }
  if !cpu.zf { pc = 0x47d8a9; continue; }
  pc = 0x47d7ff; continue;
 }
 0x47d7ff => {
  cpu.push(0x47d804); f_0047d8cd(cpu); if cpu.returned_to != 0x47d804 { return; }
  if cpu.cf { pc = 0x47d8a9; continue; }
  pc = 0x47d80a; continue;
 }
 0x47d80a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2au32.wrapping_add(cpu.edi), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x5d4855u32; cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x29u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x33u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x8u8; cpu.w8(0x39u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x2bu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x35u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2cu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2eu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x2du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2fu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x30u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x31u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1eu32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x47d8a9; continue; }
  pc = 0x47d8a5; continue;
 }
 0x47d8a5 => {
  { let v: u8 = cpu.or8(cpu.r8(0x32u32.wrapping_add(cpu.edi)), 0x44u8); cpu.w8(0x32u32.wrapping_add(cpu.edi), v); }
  pc = 0x47d8a9; continue;
 }
 0x47d8a9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d8b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d8b5); }
 let mut pc: u32 = 0x47d8b5;
 loop { match pc {
 0x47d8b5 => {
  { let v: u32 = 0x52ae94u32; cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x47d8c4; continue;
 }
 0x47d8c4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x3cu32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47d8c4; continue; }
  pc = 0x47d8cc; continue;
 }
 0x47d8cc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d8cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d8cd); }
 let mut pc: u32 = 0x47d8cd;
 loop { match pc {
 0x47d8cd => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x52ae94u32; cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x47d8de; continue;
 }
 0x47d8de => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x47d8ec; continue; }
  pc = 0x47d8e3; continue;
 }
 0x47d8e3 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x3cu32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47d8de; continue; }
  pc = 0x47d8e8; continue;
 }
 0x47d8e8 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47d8ec => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d8f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d8f0); }
 let mut pc: u32 = 0x47d8f0;
 loop { match pc {
 0x47d8f0 => {
  { cpu.sub16(cpu.r16(0x4d69bau32), 0x0u16); }
  if cpu.zf { pc = 0x47d92e; continue; }
  pc = 0x47d8fa; continue;
 }
 0x47d8fa => {
  { cpu.sub8(cpu.r8(0x4e1fbcu32), 0x3u8); }
  if !cpu.zf { pc = 0x47d92e; continue; }
  pc = 0x47d903; continue;
 }
 0x47d903 => {
  { let v: u32 = 0x52ae94u32; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  pc = 0x47d913; continue;
 }
 0x47d913 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x47d923; continue; }
  pc = 0x47d91a; continue;
 }
 0x47d91a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x47d921); f_0047d92f(cpu); if cpu.returned_to != 0x47d921 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x47d923; continue;
 }
 0x47d923 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x3cu32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47d913; continue; }
  pc = 0x47d928; continue;
 }
 0x47d928 => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  pc = 0x47d92e; continue;
 }
 0x47d92e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047d92f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47d92f); }
 let mut pc: u32 = 0x47d92f;
 loop { match pc {
 0x47d92f => {
  { cpu.sub16(cpu.r16(0x1au32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x47d940; continue; }
  pc = 0x47d937; continue;
 }
 0x47d937 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x1au32.wrapping_add(cpu.esi))); cpu.w16(0x1au32.wrapping_add(cpu.esi), v); }
  pc = 0x47df38; continue;
 }
 0x47d940 => {
  { cpu.sub16(cpu.r16(0x1eu32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.sf { pc = 0x47d94e; continue; }
  pc = 0x47d948; continue;
 }
 0x47d948 => {
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.esi), v); }
  pc = 0x47d94e; continue;
 }
 0x47d94e => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x47d961; continue; }
  pc = 0x47d95b; continue;
 }
 0x47d95b => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x47d96f; continue; }
  pc = 0x47d961; continue;
 }
 0x47d961 => {
  { let v: u32 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 6u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x8u32.wrapping_add(cpu.esi))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47d9ad; continue;
 }
 0x47d96f => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x18u8); }
  if cpu.zf { pc = 0x47d98f; continue; }
  pc = 0x47d975; continue;
 }
 0x47d975 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x2au32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x47d985; continue; }
  pc = 0x47d97f; continue;
 }
 0x47d97f => {
  { cpu.sub8(cpu.r8(0x2au32.wrapping_add(cpu.esi)), 0xc0u8); }
  if !cpu.zf { pc = 0x47d989; continue; }
  pc = 0x47d985; continue;
 }
 0x47d985 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x47d989; continue;
 }
 0x47d989 => {
  { let v: u32 = cpu.r16(0x22u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  pc = 0x47d9a7; continue;
 }
 0x47d98f => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x47d9a3; continue; }
  pc = 0x47d995; continue;
 }
 0x47d995 => {
  { let v: u32 = cpu.r16(0x24u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47d9a7; continue;
 }
 0x47d9a3 => {
  { let v: u32 = cpu.r16(0x12u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  pc = 0x47d9a7; continue;
 }
 0x47d9a7 => {
  { let v: u32 = cpu.r8(0x2bu32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  pc = 0x47d9ad; continue;
 }
 0x47d9ad => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47da8c; continue; }
  pc = 0x47d9c8; continue;
 }
 0x47d9c8 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47da8c; continue; }
  pc = 0x47d9d5; continue;
 }
 0x47d9d5 => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x4d6b9cu32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xff80u16); }
  if (cpu.sf != cpu.of) { pc = 0x47da8c; continue; }
  pc = 0x47d9e9; continue;
 }
 0x47d9e9 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x4d6ba0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xff80u16); }
  if (cpu.sf != cpu.of) { pc = 0x47da8c; continue; }
  pc = 0x47d9fd; continue;
 }
 0x47d9fd => {
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x47da1e; continue; }
  pc = 0x47da06; continue;
 }
 0x47da06 => {
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  if (cpu.sf == cpu.of) { pc = 0x47da1e; continue; }
  pc = 0x47da0e; continue;
 }
 0x47da0e => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x47da27; continue; }
  pc = 0x47da1e; continue;
 }
 0x47da1e => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x47da8c; continue;
 }
 0x47da27 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  cpu.push(0x47da38); f_0041c6a6(cpu); if cpu.returned_to != 0x47da38 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x47da6e; continue; }
  pc = 0x47da3d; continue;
 }
 0x47da3d => {
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x7au32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x47da63; continue; }
  pc = 0x47da53; continue;
 }
 0x47da53 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47da5a); f_00401130(cpu); if cpu.returned_to != 0x47da5a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x592301u32, v); }
  { cpu.popad(); }
  pc = 0x47da63; continue;
 }
 0x47da63 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x592301u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x47da6e); f_00415d61(cpu); if cpu.returned_to != 0x47da6e { return; }
  pc = 0x47da6e; continue;
 }
 0x47da6e => {
  { cpu.popad(); }
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0xcu8); }
  if !cpu.zf { pc = 0x47df39; continue; }
  pc = 0x47da79; continue;
 }
 0x47da79 => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x40u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  pc = 0x47da8c; continue;
 }
 0x47da8c => {
  { let v: u8 = cpu.dec8(cpu.r8(0x2cu32.wrapping_add(cpu.esi))); cpu.w8(0x2cu32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x47da98; continue; }
  pc = 0x47da91; continue;
 }
 0x47da91 => {
  if cpu.sf { pc = 0x47da98; continue; }
  pc = 0x47da93; continue;
 }
 0x47da93 => {
  pc = 0x47df38; continue;
 }
 0x47da98 => {
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x47daa3; continue; }
  pc = 0x47daa1; continue;
 }
 0x47daa1 => {
  { let v: u8 = cpu.shl8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x47daa3; continue;
 }
 0x47daa3 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2cu32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x47db1c; continue; }
  pc = 0x47daac; continue;
 }
 0x47daac => {
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)).wrapping_add(0x10u8); cpu.w8(0x30u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)) & 0xffu8; cpu.w8(0x30u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x1eu32.wrapping_add(cpu.esi)), 0x2u16); cpu.w16(0x1eu32.wrapping_add(cpu.esi), v); }
  if !cpu.sf { pc = 0x47df38; continue; }
  pc = 0x47dac0; continue;
 }
 0x47dac0 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x47dacd); f_0048327f(cpu); if cpu.returned_to != 0x47dacd { return; }
  { cpu.popad(); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  { let v: u16 = 0x80u16; cpu.w16(0x4d6934u32, v); }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffc0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x2u32); }
  if !cpu.zf { pc = 0x47db08; continue; }
  pc = 0x47db06; continue;
 }
 0x47db06 => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x47db08; continue;
 }
 0x47db08 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x47db10); f_0047d7e2(cpu); if cpu.returned_to != 0x47db10 { return; }
  { cpu.popad(); }
  { let v: u16 = 0x64u16; cpu.w16(0x1au32.wrapping_add(cpu.esi), v); }
  pc = 0x47df38; continue;
 }
 0x47db1c => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x38u8); }
  if !cpu.zf { pc = 0x47db30; continue; }
  pc = 0x47db22; continue;
 }
 0x47db22 => {
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x31u32.wrapping_add(cpu.esi), v); }
  pc = 0x47db51; continue;
 }
 0x47db30 => {
  cpu.push(0x47db35); f_0047e092(cpu); if cpu.returned_to != 0x47db35 { return; }
  if cpu.cf { pc = 0x47df38; continue; }
  pc = 0x47db3b; continue;
 }
 0x47db3b => {
  cpu.push(0x47db40); f_0047e014(cpu); if cpu.returned_to != 0x47db40 { return; }
  if cpu.cf { pc = 0x47df38; continue; }
  pc = 0x47db46; continue;
 }
 0x47db46 => {
  cpu.push(0x47db4b); f_0047dfb1(cpu); if cpu.returned_to != 0x47db4b { return; }
  if cpu.cf { pc = 0x47df38; continue; }
  pc = 0x47db51; continue;
 }
 0x47db51 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x47db5d); f_0047e25b(cpu); if cpu.returned_to != 0x47db5d { return; }
  { let v: u16 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x3fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x30u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xc0u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r8(0x29u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x14u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r8(0x29u32.wrapping_add(cpu.esi)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x47dbc0; continue; }
  pc = 0x47db98; continue;
 }
 0x47db98 => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x37u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x47dba8; continue; }
  pc = 0x47dba2; continue;
 }
 0x47dba2 => {
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) & 0xfeu8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  pc = 0x47dbc0; continue;
 }
 0x47dba8 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x47dbb4; continue; }
  pc = 0x47dbae; continue;
 }
 0x47dbae => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)).wrapping_add(0x2u8); cpu.w8(0x2au32.wrapping_add(cpu.esi), v); }
  pc = 0x47dbb8; continue;
 }
 0x47dbb4 => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)).wrapping_sub(0x2u8); cpu.w8(0x2au32.wrapping_add(cpu.esi), v); }
  pc = 0x47dbb8; continue;
 }
 0x47dbb8 => {
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)) & 0xffu8; cpu.w8(0x30u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) & 0xffu8; cpu.w8(0x2au32.wrapping_add(cpu.esi), v); }
  pc = 0x47dbc0; continue;
 }
 0x47dbc0 => {
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x47dbdf; continue; }
  pc = 0x47dbc6; continue;
 }
 0x47dbc6 => {
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x47dbd6; continue; }
  pc = 0x47dbcc; continue;
 }
 0x47dbcc => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)) ^ 0x1u8; cpu.w8(0x35u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x47dbdf; continue; }
  pc = 0x47dbd6; continue;
 }
 0x47dbd6 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x40u8); }
  if !cpu.zf { pc = 0x47dbdf; continue; }
  pc = 0x47dbdc; continue;
 }
 0x47dbdc => {
  { let v: u8 = cpu.inc8(cpu.r8(0x2bu32.wrapping_add(cpu.esi))); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  pc = 0x47dbdf; continue;
 }
 0x47dbdf => {
  { let v: u16 = cpu.r8(0x2bu32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x12u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x16u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x47dbf2; continue; }
  pc = 0x47dbee; continue;
 }
 0x47dbee => {
  { let v: u8 = 0x0u8; cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  pc = 0x47dbf2; continue;
 }
 0x47dbf2 => {
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x47dc1a; continue; }
  pc = 0x47dbf8; continue;
 }
 0x47dbf8 => {
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x20u32.wrapping_add(cpu.esi))); }
  if cpu.cf { pc = 0x47dc0a; continue; }
  pc = 0x47dc06; continue;
 }
 0x47dc06 => {
  { let v: u16 = cpu.r16(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x47dc0a; continue;
 }
 0x47dc0a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.esi), v); }
  { cpu.sub16(cpu.r16(0x20u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x47dc78; continue; }
  pc = 0x47dc14; continue;
 }
 0x47dc14 => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)) & 0xfdu8; cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  pc = 0x47dc78; continue;
 }
 0x47dc1a => {
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x47dc44; continue; }
  pc = 0x47dc20; continue;
 }
 0x47dc20 => {
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x20u32.wrapping_add(cpu.esi))); }
  if cpu.sf { pc = 0x47dc30; continue; }
  pc = 0x47dc2e; continue;
 }
 0x47dc2e => {
  if (!cpu.cf && !cpu.zf) { pc = 0x47dc34; continue; }
  pc = 0x47dc30; continue;
 }
 0x47dc30 => {
  { let v: u16 = cpu.r16(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x47dc34; continue;
 }
 0x47dc34 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.esi), v); }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x20u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x47dc78; continue; }
  pc = 0x47dc3e; continue;
 }
 0x47dc3e => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)) & 0xfeu8; cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  pc = 0x47dc78; continue;
 }
 0x47dc44 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x47dc78; continue; }
  pc = 0x47dc4a; continue;
 }
 0x47dc4a => {
  { let v: u16 = cpu.dec16(cpu.r16(0x4u32.wrapping_add(cpu.esi))); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x47dc78; continue; }
  pc = 0x47dc50; continue;
 }
 0x47dc50 => {
  { let v: u16 = 0x4u16; cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x20u32.wrapping_add(cpu.esi))); }
  if cpu.sf { pc = 0x47dc66; continue; }
  pc = 0x47dc64; continue;
 }
 0x47dc64 => {
  if (!cpu.cf && !cpu.zf) { pc = 0x47dc6a; continue; }
  pc = 0x47dc66; continue;
 }
 0x47dc66 => {
  { let v: u16 = cpu.r16(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x47dc6a; continue;
 }
 0x47dc6a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.esi), v); }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x20u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x47dc78; continue; }
  pc = 0x47dc74; continue;
 }
 0x47dc74 => {
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) & 0xbfu8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  pc = 0x47dc78; continue;
 }
 0x47dc78 => {
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x47df38; continue; }
  pc = 0x47dc82; continue;
 }
 0x47dc82 => {
  { let v: u32 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf8u32; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x5d476cu32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as i8 as i16 as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.imul8(v); }
  { let v: u8 = 0x22u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.idiv8(v); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6936u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.imul8(v); }
  { let v: u8 = 0x22u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.idiv8(v); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6938u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x47dd43; continue; }
  pc = 0x47dcf1; continue;
 }
 0x47dcf1 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x47dd43; continue; }
  pc = 0x47dcf7; continue;
 }
 0x47dcf7 => {
  cpu.push(0x47dcfc); f_0047e1ab(cpu); if cpu.returned_to != 0x47dcfc { return; }
  if cpu.cf { pc = 0x47dd43; continue; }
  pc = 0x47dcfe; continue;
 }
 0x47dcfe => {
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x200u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x47dd0b); f_0041c808(cpu); if cpu.returned_to != 0x47dd0b { return; }
  { cpu.sub16((cpu.eax as u16), 0x10u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47dd43; continue; }
  pc = 0x47dd11; continue;
 }
 0x47dd11 => {
  cpu.push(0x47dd16); f_0041c743(cpu); if cpu.returned_to != 0x47dd16 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x30u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0xc0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) & 0xf0u8; cpu.w8(0x2au32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x37u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) | 0x1u8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) & 0xfdu8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.sf { pc = 0x47dd43; continue; }
  pc = 0x47dd3f; continue;
 }
 0x47dd3f => {
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) | 0x2u8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  pc = 0x47dd43; continue;
 }
 0x47dd43 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x47df38; continue; }
  pc = 0x47dd4e; continue;
 }
 0x47dd4e => {
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x47df38; continue; }
  pc = 0x47dd58; continue;
 }
 0x47dd58 => {
  cpu.push(0x47dd5d); f_0041c743(cpu); if cpu.returned_to != 0x47dd5d { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x10u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47dda2; continue; }
  pc = 0x47dd67; continue;
 }
 0x47dd67 => {
  { cpu.sub16(cpu.r16(0x1eu32.wrapping_add(cpu.esi)), 0x22u16); }
  if cpu.zf { pc = 0x47dda2; continue; }
  pc = 0x47dd6f; continue;
 }
 0x47dd6f => {
  { let v: u16 = 0x4u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x14u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x47dd7c); f_0041c808(cpu); if cpu.returned_to != 0x47dd7c { return; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x22u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47dd8d; continue; }
  pc = 0x47dd89; continue;
 }
 0x47dd89 => {
  { let v: u16 = 0x22u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47dd8d; continue;
 }
 0x47dd8d => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)) & 0xfeu8; cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) & 0xbfu8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x2u8); cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  pc = 0x47df38; continue;
 }
 0x47dda2 => {
  { cpu.sub16(cpu.r16(0x1eu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x47de3d; continue; }
  pc = 0x47ddae; continue;
 }
 0x47ddae => {
  { cpu.sub8(cpu.r8(0x2bu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x47de3d; continue; }
  pc = 0x47ddb8; continue;
 }
 0x47ddb8 => {
  cpu.push(0x47ddbd); f_0041c743(cpu); if cpu.returned_to != 0x47ddbd { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x10u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47ddfe; continue; }
  pc = 0x47ddc7; continue;
 }
 0x47ddc7 => {
  { let v: u16 = 0xcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x14u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x47ddd4); f_0041c808(cpu); if cpu.returned_to != 0x47ddd4 { return; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x47dde5; continue; }
  pc = 0x47dde1; continue;
 }
 0x47dde1 => {
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47dde5; continue;
 }
 0x47dde5 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)) & 0xfdu8; cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) & 0xbfu8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x1u8); cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  pc = 0x47df38; continue;
 }
 0x47ddfe => {
  { cpu.sub16((cpu.eax as u16), 0x20u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47de3d; continue; }
  pc = 0x47de04; continue;
 }
 0x47de04 => {
  { let v: u16 = 0x2u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x14u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x47de11); f_0041c808(cpu); if cpu.returned_to != 0x47de11 { return; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x47de22; continue; }
  pc = 0x47de1e; continue;
 }
 0x47de1e => {
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47de22; continue;
 }
 0x47de22 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)) & 0xfcu8; cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x40u8); cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0x4u16; cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  pc = 0x47df38; continue;
 }
 0x47de3d => {
  { cpu.sub8(cpu.r8(0x1eu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x47df38; continue; }
  pc = 0x47de47; continue;
 }
 0x47de47 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x31u8); }
  if !cpu.zf { pc = 0x47df38; continue; }
  pc = 0x47de51; continue;
 }
 0x47de51 => {
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x47de5f; continue; }
  pc = 0x47de57; continue;
 }
 0x47de57 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.esi))); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  pc = 0x47df38; continue;
 }
 0x47de5f => {
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x47df38; continue; }
  pc = 0x47de69; continue;
 }
 0x47de69 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x40u8); }
  if !cpu.zf { pc = 0x47df38; continue; }
  pc = 0x47de73; continue;
 }
 0x47de73 => {
  { cpu.sub16(cpu.r16(0x22u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x47df38; continue; }
  pc = 0x47de7f; continue;
 }
 0x47de7f => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3fu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x47df38; continue; }
  pc = 0x47de8a; continue;
 }
 0x47de8a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xc0u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x5d47c5u32.wrapping_add(cpu.eax)) as i8 as i16 as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x5d47c4u32.wrapping_add(cpu.eax)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6934u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6936u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x47ded0); f_00420989(cpu); if cpu.returned_to != 0x47ded0 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x47df08; continue; }
  pc = 0x47dede; continue;
 }
 0x47dede => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x47df08; continue; }
  pc = 0x47deec; continue;
 }
 0x47deec => {
  { let v: u8 = cpu.r8(0x4080u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x47df08; continue; }
  pc = 0x47defa; continue;
 }
 0x47defa => {
  { let v: u8 = cpu.r8(0x4081u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x47df0b; continue; }
  pc = 0x47df08; continue;
 }
 0x47df08 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x47df38; continue;
 }
 0x47df0b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x3au32.wrapping_add(cpu.esi), v); }
  cpu.push(0x47df1b); f_0041c743(cpu); if cpu.returned_to != 0x47df1b { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x3fu16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47df38; continue; }
  pc = 0x47df25; continue;
 }
 0x47df25 => {
  { let v: u32 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x8u8); cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  pc = 0x47df38; continue;
 }
 0x47df38 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47df39 => {
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4d8236u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x60u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x60u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x21u8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x47df91; continue; }
  pc = 0x47df85; continue;
 }
 0x47df85 => {
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47df91; continue;
 }
 0x47df91 => {
  { cpu.sub16(cpu.r16(0x1eu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x47dfa5; continue; }
  pc = 0x47df98; continue;
 }
 0x47df98 => {
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47dfa5; continue;
 }
 0x47dfa5 => {
  { let v: u32 = cpu.add32(cpu.r32(0x4d6ac0u32), 0x10u32); cpu.w32(0x4d6ac0u32, v); }
  cpu.poll(); pc = 0x47da8c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047dfb1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47dfb1); }
 let mut pc: u32 = 0x47dfb1;
 loop { match pc {
 0x47dfb1 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x47dfb9; continue; }
  pc = 0x47dfb7; continue;
 }
 0x47dfb7 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47dfb9 => {
  { let v: u16 = 0x9u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x39u32.wrapping_add(cpu.esi)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x47dfca); f_0047e159(cpu); if cpu.returned_to != 0x47dfca { return; }
  { let v: u32 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.dec8(cpu.r8(0x31u32.wrapping_add(cpu.esi))); cpu.w8(0x31u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x2bu32.wrapping_add(cpu.esi))); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x2bu32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x47e012; continue; }
  pc = 0x47dfe1; continue;
 }
 0x47dfe1 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.esi))); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x47e012; continue; }
  pc = 0x47dff1; continue;
 }
 0x47dff1 => {
  { cpu.sub8(cpu.r8(0x2au32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x47e000; continue; }
  pc = 0x47dff7; continue;
 }
 0x47dff7 => {
  { let v: u16 = cpu.r8(0x31u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  pc = 0x47e000; continue;
 }
 0x47e000 => {
  { let v: u8 = cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0xefu8); cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x14u8; cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x2cu32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x47dfb7; continue;
 }
 0x47e012 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e014(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e014); }
 let mut pc: u32 = 0x47e014;
 loop { match pc {
 0x47e014 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x20u8); }
  if !cpu.zf { pc = 0x47e01c; continue; }
  pc = 0x47e01a; continue;
 }
 0x47e01a => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47e01c => {
  { let v: u32 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x2bu32.wrapping_add(cpu.esi))); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x2bu32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x47e090; continue; }
  pc = 0x47e034; continue;
 }
 0x47e034 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47e03a); f_0041c743(cpu); if cpu.returned_to != 0x47e03a { return; }
  { let v: u16 = (cpu.eax as u16) & 0x2u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.esi))); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x47e090; continue; }
  pc = 0x47e053; continue;
 }
 0x47e053 => {
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) & 0xdfu8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) | 0x10u8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x40u16); }
  if !cpu.zf { pc = 0x47e06c; continue; }
  pc = 0x47e065; continue;
 }
 0x47e065 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)).wrapping_add(0x2u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x47e077; continue;
 }
 0x47e06c => {
  { cpu.sub16((cpu.eax as u16), 0xc0u16); }
  if !cpu.zf { pc = 0x47e077; continue; }
  pc = 0x47e072; continue;
 }
 0x47e072 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)).wrapping_sub(0x2u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x47e077; continue;
 }
 0x47e077 => {
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  pc = 0x47e090; continue;
 }
 0x47e090 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e092(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e092); }
 let mut pc: u32 = 0x47e092;
 loop { match pc {
 0x47e092 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x8u8); }
  if !cpu.zf { pc = 0x47e09a; continue; }
  pc = 0x47e098; continue;
 }
 0x47e098 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47e09a => {
  { let v: u16 = cpu.r8(0x39u32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x47e0a4); f_0047e107(cpu); if cpu.returned_to != 0x47e0a4 { return; }
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xe0u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x5d47bcu32.wrapping_add(cpu.eax)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x31u32.wrapping_add(cpu.esi))); cpu.w8(0x31u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x2bu32.wrapping_add(cpu.esi))); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x2bu32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x47e105; continue; }
  pc = 0x47e0d9; continue;
 }
 0x47e0d9 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.esi))); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x47e105; continue; }
  pc = 0x47e0e7; continue;
 }
 0x47e0e7 => {
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) & 0xf7u8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) | 0x20u8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)).wrapping_add(0x1u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x47e0fa); f_0041c743(cpu); if cpu.returned_to != 0x47e0fa { return; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  pc = 0x47e105; continue;
 }
 0x47e105 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e107(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e107); }
 let mut pc: u32 = 0x47e107;
 loop { match pc {
 0x47e107 => {
  { cpu.sub16((cpu.eax as u16), 0x7u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47e113; continue; }
  pc = 0x47e10d; continue;
 }
 0x47e10d => {
  { let v: u16 = 0x3u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47e123; continue;
 }
 0x47e113 => {
  { cpu.sub16((cpu.eax as u16), 0x4u16); }
  if (cpu.sf == cpu.of) { pc = 0x47e11f; continue; }
  pc = 0x47e119; continue;
 }
 0x47e119 => {
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47e123; continue;
 }
 0x47e11f => {
  { let v: u16 = 0x2u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47e123; continue;
 }
 0x47e123 => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xe0u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r8(0x5d47bcu32.wrapping_add(cpu.eax)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.imul16(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r8(0x5d47bdu32.wrapping_add(cpu.eax)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if cpu.zf { pc = 0x47e151; continue; }
  pc = 0x47e14d; continue;
 }
 0x47e14d => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x1u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47e151; continue;
 }
 0x47e151 => {
  { let v = (cpu.ebx as u16); cpu.imul16(v); }
  { let v: u16 = cpu.add16(cpu.r16(0x2u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e159(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e159); }
 let mut pc: u32 = 0x47e159;
 loop { match pc {
 0x47e159 => {
  { cpu.sub16((cpu.eax as u16), 0x5u16); }
  if cpu.cf { pc = 0x47e165; continue; }
  pc = 0x47e15f; continue;
 }
 0x47e15f => {
  { let v: u16 = 0x3u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47e175; continue;
 }
 0x47e165 => {
  { cpu.sub16((cpu.eax as u16), 0x2u16); }
  if (cpu.sf == cpu.of) { pc = 0x47e171; continue; }
  pc = 0x47e16b; continue;
 }
 0x47e16b => {
  { let v: u16 = 0x2u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47e175; continue;
 }
 0x47e171 => {
  { let v: u16 = 0x2u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47e175; continue;
 }
 0x47e175 => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xe0u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r8(0x5d47bcu32.wrapping_add(cpu.eax)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.imul16(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r8(0x5d47bdu32.wrapping_add(cpu.eax)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if cpu.zf { pc = 0x47e1a3; continue; }
  pc = 0x47e19f; continue;
 }
 0x47e19f => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x1u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x47e1a3; continue;
 }
 0x47e1a3 => {
  { let v = (cpu.ebx as u16); cpu.imul16(v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e1ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e1ab); }
 let mut pc: u32 = 0x47e1ab;
 loop { match pc {
 0x47e1ab => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d678eu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x40u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6790u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0x40u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x47e1f5; continue; }
  pc = 0x47e1cb; continue;
 }
 0x47e1cb => {
  { cpu.sub16((cpu.eax as u16), 0x40u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47e1f5; continue; }
  pc = 0x47e1d1; continue;
 }
 0x47e1d1 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x47e20a; continue; }
  pc = 0x47e1da; continue;
 }
 0x47e1da => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x47e21f; continue; }
  pc = 0x47e1e4; continue;
 }
 0x47e1e4 => {
  { cpu.sub16((cpu.eax as u16), 0x40u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47e21f; continue; }
  pc = 0x47e1ea; continue;
 }
 0x47e1ea => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x47e238; continue; }
  pc = 0x47e1f3; continue;
 }
 0x47e1f3 => {
  pc = 0x47e258; continue;
 }
 0x47e1f5 => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x47e258; continue; }
  pc = 0x47e1fc; continue;
 }
 0x47e1fc => {
  { cpu.sub8((cpu.eax as u8), 0x80u8); }
  if cpu.cf { pc = 0x47e258; continue; }
  pc = 0x47e200; continue;
 }
 0x47e200 => {
  { let v: u8 = 0x80u8; cpu.w8(0x37u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) | 0x2u8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  pc = 0x47e251; continue;
 }
 0x47e20a => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x47e258; continue; }
  pc = 0x47e211; continue;
 }
 0x47e211 => {
  { cpu.sub8((cpu.eax as u8), 0x80u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47e258; continue; }
  pc = 0x47e215; continue;
 }
 0x47e215 => {
  { let v: u8 = 0x0u8; cpu.w8(0x37u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) | 0x2u8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  pc = 0x47e251; continue;
 }
 0x47e21f => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xc0u8); }
  if cpu.zf { pc = 0x47e258; continue; }
  pc = 0x47e226; continue;
 }
 0x47e226 => {
  { cpu.sub8((cpu.eax as u8), 0x40u8); }
  if cpu.cf { pc = 0x47e22e; continue; }
  pc = 0x47e22a; continue;
 }
 0x47e22a => {
  { cpu.sub8((cpu.eax as u8), 0xc0u8); }
  if cpu.cf { pc = 0x47e258; continue; }
  pc = 0x47e22e; continue;
 }
 0x47e22e => {
  { let v: u8 = 0xc0u8; cpu.w8(0x37u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) | 0x2u8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  pc = 0x47e251; continue;
 }
 0x47e238 => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x40u8); }
  if cpu.zf { pc = 0x47e258; continue; }
  pc = 0x47e23f; continue;
 }
 0x47e23f => {
  { cpu.sub8((cpu.eax as u8), 0x40u8); }
  if cpu.cf { pc = 0x47e258; continue; }
  pc = 0x47e243; continue;
 }
 0x47e243 => {
  { cpu.sub8((cpu.eax as u8), 0xc0u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47e258; continue; }
  pc = 0x47e247; continue;
 }
 0x47e247 => {
  { let v: u8 = 0x40u8; cpu.w8(0x37u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) | 0x2u8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  pc = 0x47e251; continue;
 }
 0x47e251 => {
  { let v: u8 = cpu.or8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x1u8); cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47e258 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e25b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e25b); }
 let mut pc: u32 = 0x47e25b;
 loop { match pc {
 0x47e25b => {
  { cpu.pushad(); }
  cpu.push(0x47e261); f_0041c743(cpu); if cpu.returned_to != 0x47e261 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x4u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47e2e7; continue; }
  pc = 0x47e26b; continue;
 }
 0x47e26b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d4758u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5d475au32, v); }
  { let v: u32 = 0x509290u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x47e282; continue;
 }
 0x47e282 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x47e2df; continue; }
  pc = 0x47e288; continue;
 }
 0x47e288 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x47e2df; continue; }
  pc = 0x47e28e; continue;
 }
 0x47e28e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { pc = 0x47e2df; continue; }
  pc = 0x47e294; continue;
 }
 0x47e294 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x47e2a0; continue; }
  pc = 0x47e29a; continue;
 }
 0x47e29a => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { pc = 0x47e2df; continue; }
  pc = 0x47e2a0; continue;
 }
 0x47e2a0 => {
  { let v: u16 = cpu.r16(0x5d4758u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x5d475au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x100u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47e2df; continue; }
  pc = 0x47e2d2; continue;
 }
 0x47e2d2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  cpu.push(0x47e2dc); f_0045ae1d(cpu); if cpu.returned_to != 0x47e2dc { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x47e2df; continue;
 }
 0x47e2df => {
  { let v: u32 = cpu.add32(cpu.edi, 0xb8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47e282; continue; }
  pc = 0x47e2e7; continue;
 }
 0x47e2e7 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e2e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e2e9); }
 let mut pc: u32 = 0x47e2e9;
 loop { match pc {
 0x47e2e9 => {
  { let v: u32 = 0x561eb8u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x55feb8u32; cpu.edi = v; }
  { let v: u32 = 0x5293a0u32; cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x47e30e; continue;
 }
 0x47e30e => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi)), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x47e32d; continue; }
  pc = 0x47e313; continue;
 }
 0x47e313 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x2bu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x47e32d; continue; }
  pc = 0x47e320; continue;
 }
 0x47e320 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  pc = 0x47e32d; continue;
 }
 0x47e32d => {
  { let v: u32 = cpu.add32(cpu.esi, 0x40u32); cpu.esi = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47e30e; continue; }
  pc = 0x47e335; continue;
 }
 0x47e335 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e338(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e338); }
 let mut pc: u32 = 0x47e338;
 loop { match pc {
 0x47e338 => {
  cpu.push(0x47e33d); f_0046a3a3(cpu); if cpu.returned_to != 0x47e33d { return; }
  { let v: u32 = 0x52300cu32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x4e1fa7u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x47e3da; continue; }
  pc = 0x47e350; continue;
 }
 0x47e350 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { cpu.sub8(cpu.r8(0x52bc2au32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x47e3cf; continue; }
  pc = 0x47e364; continue;
 }
 0x47e364 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x47e372; continue; }
  pc = 0x47e36d; continue;
 }
 0x47e36d => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x47e377; continue; }
  pc = 0x47e372; continue;
 }
 0x47e372 => {
  { let v: u32 = 0x5b355cu32; cpu.eax = v; }
  pc = 0x47e377; continue;
 }
 0x47e377 => {
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x47e3cf; continue; }
  pc = 0x47e37d; continue;
 }
 0x47e37d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x47e38a); f_0047e4ab(cpu); if cpu.returned_to != 0x47e38a { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x47e3cf; continue; }
  pc = 0x47e390; continue;
 }
 0x47e390 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { cpu.sub8(cpu.r8(0x52bc2au32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x47e3cf; continue; }
  pc = 0x47e3a3; continue;
 }
 0x47e3a3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x47e3b0; continue; }
  pc = 0x47e3ab; continue;
 }
 0x47e3ab => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x47e3b5; continue; }
  pc = 0x47e3b0; continue;
 }
 0x47e3b0 => {
  { let v: u32 = 0x5b355cu32; cpu.eax = v; }
  pc = 0x47e3b5; continue;
 }
 0x47e3b5 => {
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x47e3cf; continue; }
  pc = 0x47e3bb; continue;
 }
 0x47e3bb => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x47e3c8); f_0047111f(cpu); if cpu.returned_to != 0x47e3c8 { return; }
  if cpu.cf { pc = 0x47e3cf; continue; }
  pc = 0x47e3ca; continue;
 }
 0x47e3ca => {
  cpu.push(0x47e3cf); f_0047e3db(cpu); if cpu.returned_to != 0x47e3cf { return; }
  pc = 0x47e3cf; continue;
 }
 0x47e3cf => {
  { let v: u32 = cpu.add32(cpu.esi, 0x14u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47e350; continue; }
  pc = 0x47e3da; continue;
 }
 0x47e3da => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e3db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e3db); }
 let mut pc: u32 = 0x47e3db;
 loop { match pc {
 0x47e3db => {
  { cpu.pushad(); }
  cpu.push(0x47e3e1); f_0041c743(cpu); if cpu.returned_to != 0x47e3e1 { return; }
  { let v: u32 = cpu.eax & 0x38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d4cdcu32, v); }
  { let v: u8 = 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x47e3eb; continue;
 }
 0x47e3eb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5d4cdcu32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5d4c9cu32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x5d4ca0u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x47e417); f_0047e4ab(cpu); if cpu.returned_to != 0x47e417 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x47e493; continue; }
  pc = 0x47e41d; continue;
 }
 0x47e41d => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = 0x4dd54cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e97b1u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x47e493; continue; }
  pc = 0x47e44c; continue;
 }
 0x47e44c => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e97b1u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x47e493; continue; }
  pc = 0x47e45b; continue;
 }
 0x47e45b => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e97b1u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x47e493; continue; }
  pc = 0x47e46d; continue;
 }
 0x47e46d => {
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e97b1u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x47e493; continue; }
  pc = 0x47e47f; continue;
 }
 0x47e47f => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.ebx = v; }
  cpu.push(0x47e491); f_00489637(cpu); if cpu.returned_to != 0x47e491 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47e493 => {
  { let v: u32 = cpu.r32(0x5d4cdcu32).wrapping_add(0x8u32); cpu.w32(0x5d4cdcu32, v); }
  { let v: u32 = cpu.and32(cpu.r32(0x5d4cdcu32), 0x38u32); cpu.w32(0x5d4cdcu32, v); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47e3eb; continue; }
  pc = 0x47e4a9; continue;
 }
 0x47e4a9 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e4ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e4ab); }
 let mut pc: u32 = 0x47e4ab;
 loop { match pc {
 0x47e4ab => {
  { let v: u32 = 0x52300cu32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x4e1fa7u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x47e4b6; continue;
 }
 0x47e4b6 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x47e4fb; continue; }
  pc = 0x47e4ba; continue;
 }
 0x47e4ba => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x47e4d2; continue; }
  pc = 0x47e4c1; continue;
 }
 0x47e4c1 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.edx as u8), ((cpu.edx >> 8) as u8)); }
  if !cpu.zf { pc = 0x47e4fb; continue; }
  pc = 0x47e4d2; continue;
 }
 0x47e4d2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x40u32); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x47e4fb; continue; }
  pc = 0x47e4f9; continue;
 }
 0x47e4f9 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47e4fb => {
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47e4b6; continue; }
  pc = 0x47e502; continue;
 }
 0x47e502 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e504(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e504); }
 let mut pc: u32 = 0x47e504;
 loop { match pc {
 0x47e504 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x47e50c); f_0047e817(cpu); if cpu.returned_to != 0x47e50c { return; }
  if cpu.cf { pc = 0x47e59c; continue; }
  pc = 0x47e512; continue;
 }
 0x47e512 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd4u32, v); }
  { let v: u32 = cpu.r32(0x4d6bd4u32).wrapping_add(0x4u32); cpu.w32(0x4d6bd4u32, v); }
  cpu.push(0x47e53a); f_0041c6a6(cpu); if cpu.returned_to != 0x47e53a { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47e560; continue; }
  pc = 0x47e540; continue;
 }
 0x47e540 => {
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x32u8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x47e560); f_0047e7a8(cpu); if cpu.returned_to != 0x47e560 { return; }
  pc = 0x47e560; continue;
 }
 0x47e560 => {
  cpu.push(0x47e565); f_0047e817(cpu); if cpu.returned_to != 0x47e565 { return; }
  if cpu.cf { pc = 0x47e59c; continue; }
  pc = 0x47e567; continue;
 }
 0x47e567 => {
  { let v: u8 = 0x32u8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x47e575); f_0041c6a6(cpu); if cpu.returned_to != 0x47e575 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47e59c; continue; }
  pc = 0x47e57b; continue;
 }
 0x47e57b => {
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x47e59c); f_0047e7a8(cpu); if cpu.returned_to != 0x47e59c { return; }
  pc = 0x47e59c; continue;
 }
 0x47e59c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e5a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e5a0); }
 let mut pc: u32 = 0x47e5a0;
 loop { match pc {
 0x47e5a0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x47e5a8); f_0047e817(cpu); if cpu.returned_to != 0x47e5a8 { return; }
  if cpu.cf { pc = 0x47e63a; continue; }
  pc = 0x47e5ae; continue;
 }
 0x47e5ae => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd4u32, v); }
  { let v: u32 = cpu.r32(0x4d6bd4u32).wrapping_add(0x4u32); cpu.w32(0x4d6bd4u32, v); }
  cpu.push(0x47e5d8); f_0041c6a6(cpu); if cpu.returned_to != 0x47e5d8 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47e5fe; continue; }
  pc = 0x47e5de; continue;
 }
 0x47e5de => {
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x32u8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x47e5fe); f_0047e7a8(cpu); if cpu.returned_to != 0x47e5fe { return; }
  pc = 0x47e5fe; continue;
 }
 0x47e5fe => {
  cpu.push(0x47e603); f_0047e817(cpu); if cpu.returned_to != 0x47e603 { return; }
  if cpu.cf { pc = 0x47e63a; continue; }
  pc = 0x47e605; continue;
 }
 0x47e605 => {
  { let v: u8 = 0x32u8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x47e613); f_0041c6a6(cpu); if cpu.returned_to != 0x47e613 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47e63a; continue; }
  pc = 0x47e619; continue;
 }
 0x47e619 => {
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x47e63a); f_0047e7a8(cpu); if cpu.returned_to != 0x47e63a { return; }
  pc = 0x47e63a; continue;
 }
 0x47e63a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e63e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e63e); }
 let mut pc: u32 = 0x47e63e;
 loop { match pc {
 0x47e63e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x47e646); f_0047e817(cpu); if cpu.returned_to != 0x47e646 { return; }
  if cpu.cf { pc = 0x47e696; continue; }
  pc = 0x47e648; continue;
 }
 0x47e648 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { cpu.lods(1, false); }
  cpu.push(0x47e65c); f_0041c6a6(cpu); if cpu.returned_to != 0x47e65c { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6934u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x4d6936u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7cu8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47e696; continue;
 }
 0x47e696 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e69a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e69a); }
 let mut pc: u32 = 0x47e69a;
 loop { match pc {
 0x47e69a => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x47e6a2); f_0047e817(cpu); if cpu.returned_to != 0x47e6a2 { return; }
  if cpu.cf { pc = 0x47e749; continue; }
  pc = 0x47e6a8; continue;
 }
 0x47e6a8 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd4u32, v); }
  { let v: u32 = cpu.r32(0x4d6bd4u32).wrapping_add(0x4u32); cpu.w32(0x4d6bd4u32, v); }
  cpu.push(0x47e6d1); f_0041c6a6(cpu); if cpu.returned_to != 0x47e6d1 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47e702; continue; }
  pc = 0x47e6d7; continue;
 }
 0x47e6d7 => {
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x47e6f2); f_0041c6a6(cpu); if cpu.returned_to != 0x47e6f2 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x47e6fd); f_0047e7a8(cpu); if cpu.returned_to != 0x47e6fd { return; }
  { let v: u16 = cpu.neg16(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47e702; continue;
 }
 0x47e702 => {
  cpu.push(0x47e707); f_0041c6a6(cpu); if cpu.returned_to != 0x47e707 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47e749; continue; }
  pc = 0x47e70d; continue;
 }
 0x47e70d => {
  cpu.push(0x47e712); f_0047e817(cpu); if cpu.returned_to != 0x47e712 { return; }
  if cpu.cf { pc = 0x47e749; continue; }
  pc = 0x47e714; continue;
 }
 0x47e714 => {
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x47e739); f_0041c6a6(cpu); if cpu.returned_to != 0x47e739 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x47e744); f_0047e7a8(cpu); if cpu.returned_to != 0x47e744 { return; }
  { let v: u16 = cpu.neg16(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47e749; continue;
 }
 0x47e749 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e74b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e74b); }
 let mut pc: u32 = 0x47e74b;
 loop { match pc {
 0x47e74b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x47e75d); f_00420989(cpu); if cpu.returned_to != 0x47e75d { return; }
  cpu.push(0x47e762); f_0047e788(cpu); if cpu.returned_to != 0x47e762 { return; }
  if cpu.cf { pc = 0x47e783; continue; }
  pc = 0x47e764; continue;
 }
 0x47e764 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x47e76a); f_0047e788(cpu); if cpu.returned_to != 0x47e76a { return; }
  if cpu.cf { pc = 0x47e783; continue; }
  pc = 0x47e76c; continue;
 }
 0x47e76c => {
  { let v: u32 = cpu.add32(cpu.esi, 0x7fu32); cpu.esi = v; }
  cpu.push(0x47e774); f_0047e788(cpu); if cpu.returned_to != 0x47e774 { return; }
  if cpu.cf { pc = 0x47e783; continue; }
  pc = 0x47e776; continue;
 }
 0x47e776 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x47e77c); f_0047e788(cpu); if cpu.returned_to != 0x47e77c { return; }
  if cpu.cf { pc = 0x47e783; continue; }
  pc = 0x47e77e; continue;
 }
 0x47e77e => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47e783 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e788(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e788); }
 let mut pc: u32 = 0x47e788;
 loop { match pc {
 0x47e788 => {
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1eu8); }
  if cpu.zf { pc = 0x47e7a6; continue; }
  pc = 0x47e794; continue;
 }
 0x47e794 => {
  { cpu.sub8((cpu.eax as u8), 0x14u8); }
  if cpu.zf { pc = 0x47e7a6; continue; }
  pc = 0x47e798; continue;
 }
 0x47e798 => {
  { cpu.sub8((cpu.eax as u8), 0x15u8); }
  if cpu.zf { pc = 0x47e7a6; continue; }
  pc = 0x47e79c; continue;
 }
 0x47e79c => {
  { cpu.sub8((cpu.eax as u8), 0x3cu8); }
  if cpu.zf { pc = 0x47e7a6; continue; }
  pc = 0x47e7a0; continue;
 }
 0x47e7a0 => {
  { cpu.sub8((cpu.eax as u8), 0x3du8); }
  if cpu.zf { pc = 0x47e7a6; continue; }
  pc = 0x47e7a4; continue;
 }
 0x47e7a4 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47e7a6 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e7a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e7a8); }
 let mut pc: u32 = 0x47e7a8;
 loop { match pc {
 0x47e7a8 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x47e7c0); f_00420989(cpu); if cpu.returned_to != 0x47e7c0 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x47e80c; continue; }
  pc = 0x47e7cc; continue;
 }
 0x47e7cc => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x47e80c; continue; }
  pc = 0x47e7d0; continue;
 }
 0x47e7d0 => {
  { cpu.sub8((cpu.eax as u8), 0x16u8); }
  if cpu.zf { pc = 0x47e80c; continue; }
  pc = 0x47e7d4; continue;
 }
 0x47e7d4 => {
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if cpu.zf { pc = 0x47e80c; continue; }
  pc = 0x47e7d8; continue;
 }
 0x47e7d8 => {
  { cpu.sub8((cpu.eax as u8), 0x24u8); }
  if cpu.zf { pc = 0x47e80c; continue; }
  pc = 0x47e7dc; continue;
 }
 0x47e7dc => {
  { cpu.sub8((cpu.eax as u8), 0x2du8); }
  if cpu.zf { pc = 0x47e80c; continue; }
  pc = 0x47e7e0; continue;
 }
 0x47e7e0 => {
  { cpu.sub8((cpu.eax as u8), 0x35u8); }
  if cpu.zf { pc = 0x47e80c; continue; }
  pc = 0x47e7e4; continue;
 }
 0x47e7e4 => {
  { cpu.sub8((cpu.eax as u8), 0x36u8); }
  if cpu.zf { pc = 0x47e80c; continue; }
  pc = 0x47e7e8; continue;
 }
 0x47e7e8 => {
  { cpu.sub8((cpu.eax as u8), 0x37u8); }
  if cpu.zf { pc = 0x47e80c; continue; }
  pc = 0x47e7ec; continue;
 }
 0x47e7ec => {
  pc = 0x47e80e; continue;
 }
 0x47e80c => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47e80e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e817(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e817); }
 let mut pc: u32 = 0x47e817;
 loop { match pc {
 0x47e817 => {
  { let v: u32 = 0x52195cu32; cpu.ebp = v; }
  { let v: u32 = 0x1f4u32; cpu.ecx = v; }
  pc = 0x47e821; continue;
 }
 0x47e821 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x47e830; continue; }
  pc = 0x47e829; continue;
 }
 0x47e829 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47e821; continue; }
  pc = 0x47e82e; continue;
 }
 0x47e82e => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47e830 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047e832(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47e832); }
 let mut pc: u32 = 0x47e832;
 loop { match pc {
 0x47e832 => {
  { let v: u32 = 0x52195cu32; cpu.ebp = v; }
  { let v: u32 = 0x1f4u32; cpu.ecx = v; }
  pc = 0x47e83c; continue;
 }
 0x47e83c => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if !cpu.zf { pc = 0x47e84b; continue; }
  pc = 0x47e845; continue;
 }
 0x47e845 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47e83c; continue; }
  pc = 0x47e84a; continue;
 }
 0x47e84a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47e84b => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.sf { pc = 0x47e8fb; continue; }
  pc = 0x47e858; continue;
 }
 0x47e858 => {
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x47e9e4; continue; }
  pc = 0x47e863; continue;
 }
 0x47e863 => {
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x47ea6f; continue; }
  pc = 0x47e86e; continue;
 }
 0x47e86e => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47e8d8; continue; }
  pc = 0x47e880; continue;
 }
 0x47e880 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x47e8d8; continue; }
  pc = 0x47e889; continue;
 }
 0x47e889 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47e8d8; continue; }
  pc = 0x47e89b; continue;
 }
 0x47e89b => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x47e8d8; continue; }
  pc = 0x47e8a4; continue;
 }
 0x47e8a4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ecx >> 8) as u8), 2u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x2u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47e8b4; continue; }
  pc = 0x47e8b2; continue;
 }
 0x47e8b2 => {
  { let v: u8 = 0x2u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x47e8b4; continue;
 }
 0x47e8b4 => {
  { let v: u8 = 0x2u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub16(cpu.r16(0x5d4ce0u32), 0xffffu16); }
  if cpu.zf { pc = 0x47e8d6; continue; }
  pc = 0x47e8c3; continue;
 }
 0x47e8c3 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x5u32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x5d4ce0u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x47e8d6); cpu.call(t); if cpu.returned_to != 0x47e8d6 { return; }
  pc = 0x47e8d6; continue;
 }
 0x47e8d6 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x47e8d8; continue;
 }
 0x47e8d8 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x47e845; continue; }
  pc = 0x47e8e5; continue;
 }
 0x47e8e5 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { cpu.poll(); pc = 0x47e845; continue; }
  pc = 0x47e8ef; continue;
 }
 0x47e8ef => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x47e845; continue;
 }
 0x47e8fb => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47e984; continue; }
  pc = 0x47e90d; continue;
 }
 0x47e90d => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x47e984; continue; }
  pc = 0x47e916; continue;
 }
 0x47e916 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47e984; continue; }
  pc = 0x47e92b; continue;
 }
 0x47e92b => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x47e984; continue; }
  pc = 0x47e934; continue;
 }
 0x47e934 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x47e95d; continue; }
  pc = 0x47e93f; continue;
 }
 0x47e93f => {
  { let v: u16 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x5a2b12u32), 0xffffu16); }
  if cpu.zf { pc = 0x47e95d; continue; }
  pc = 0x47e954; continue;
 }
 0x47e954 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x5a2b12u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47e979; continue;
 }
 0x47e95d => {
  { let v: u16 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x5a2b10u32), 0xffffu16); }
  if !cpu.zf { pc = 0x47e972; continue; }
  pc = 0x47e96e; continue;
 }
 0x47e96e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x47e984; continue;
 }
 0x47e972 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x5a2b10u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47e979; continue;
 }
 0x47e979 => {
  { let v: u16 = 0xffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x47e982); f_00486542(cpu); if cpu.returned_to != 0x47e982 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x47e984; continue;
 }
 0x47e984 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x47e845; continue; }
  pc = 0x47e991; continue;
 }
 0x47e991 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = 0x5d4ce4u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)).wrapping_sub((cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x80u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub16(cpu.r16(0x5a2b10u32), 0x53du16); }
  if !cpu.zf { pc = 0x47e9c9; continue; }
  pc = 0x47e9c5; continue;
 }
 0x47e9c5 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  pc = 0x47e9cb; continue;
 }
 0x47e9c9 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  pc = 0x47e9cb; continue;
 }
 0x47e9cb => {
  if cpu.zf { pc = 0x47e9d8; continue; }
  pc = 0x47e9cd; continue;
 }
 0x47e9cd => {
  { let v: u8 = (cpu.eax as u8) | (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x47e845; continue;
 }
 0x47e9d8 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x47e845; continue;
 }
 0x47e9e4 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47ea4b; continue; }
  pc = 0x47e9f6; continue;
 }
 0x47e9f6 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x47ea4b; continue; }
  pc = 0x47e9ff; continue;
 }
 0x47e9ff => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47ea4b; continue; }
  pc = 0x47ea11; continue;
 }
 0x47ea11 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x47ea4b; continue; }
  pc = 0x47ea1a; continue;
 }
 0x47ea1a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x737u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1fu8); }
  if (cpu.sf == cpu.of) { pc = 0x47ea38; continue; }
  pc = 0x47ea32; continue;
 }
 0x47ea32 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x2u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47ea43; continue;
 }
 0x47ea38 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x3eu8); }
  if (cpu.sf == cpu.of) { pc = 0x47ea43; continue; }
  pc = 0x47ea3f; continue;
 }
 0x47ea3f => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x1u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x47ea43; continue;
 }
 0x47ea43 => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x47ea49); cpu.call(t); if cpu.returned_to != 0x47ea49 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x47ea4b; continue;
 }
 0x47ea4b => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x47e845; continue; }
  pc = 0x47ea58; continue;
 }
 0x47ea58 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x47ea63; continue; }
  pc = 0x47ea5e; continue;
 }
 0x47ea5e => {
  cpu.poll(); pc = 0x47e845; continue;
 }
 0x47ea63 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x47e845; continue;
 }
 0x47ea6f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47eade; continue; }
  pc = 0x47ea81; continue;
 }
 0x47ea81 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x47eade; continue; }
  pc = 0x47ea8a; continue;
 }
 0x47ea8a => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47eade; continue; }
  pc = 0x47ea9c; continue;
 }
 0x47ea9c => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x47eade; continue; }
  pc = 0x47eaa5; continue;
 }
 0x47eaa5 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ecx >> 8) as u8), 2u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x2u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47eab5; continue; }
  pc = 0x47eab3; continue;
 }
 0x47eab3 => {
  { let v: u8 = 0x2u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x47eab5; continue;
 }
 0x47eab5 => {
  { let v: u8 = 0x2u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub16(cpu.r16(0x5d4ce2u32), 0xffffu16); }
  if cpu.zf { pc = 0x47eadc; continue; }
  pc = 0x47eac4; continue;
 }
 0x47eac4 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0x1fu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x5d4ce2u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x47eadc); cpu.call(t); if cpu.returned_to != 0x47eadc { return; }
  pc = 0x47eadc; continue;
 }
 0x47eadc => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x47eade; continue;
 }
 0x47eade => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x47e845; continue; }
  pc = 0x47eaeb; continue;
 }
 0x47eaeb => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { cpu.poll(); pc = 0x47e845; continue; }
  pc = 0x47eaf5; continue;
 }
 0x47eaf5 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x47e845; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047eb01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47eb01); }
 let mut pc: u32 = 0x47eb01;
 loop { match pc {
 0x47eb01 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x47ed4d; continue; }
  pc = 0x47eb10; continue;
 }
 0x47eb10 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x47ed4d; continue; }
  pc = 0x47eb1b; continue;
 }
 0x47eb1b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x47eb29; continue; }
  pc = 0x47eb24; continue;
 }
 0x47eb24 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x47eb2e; continue; }
  pc = 0x47eb29; continue;
 }
 0x47eb29 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x47eb2e; continue;
 }
 0x47eb2e => {
  cpu.push(0x47eb33); f_0041c6a6(cpu); if cpu.returned_to != 0x47eb33 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47ebb2; continue; }
  pc = 0x47eb39; continue;
 }
 0x47eb39 => {
  { let v: u32 = 0x5d4d94u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x47ecbf; continue; }
  pc = 0x47eb49; continue;
 }
 0x47eb49 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x47eb91; continue; }
  pc = 0x47eb50; continue;
 }
 0x47eb50 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x47eb9f; continue; }
  pc = 0x47eb57; continue;
 }
 0x47eb57 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x47eba6; continue; }
  pc = 0x47eb5e; continue;
 }
 0x47eb5e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x47eb98; continue; }
  pc = 0x47eb65; continue;
 }
 0x47eb65 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x47eb7c; continue; }
  pc = 0x47eb6c; continue;
 }
 0x47eb6c => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x47eb83; continue; }
  pc = 0x47eb73; continue;
 }
 0x47eb73 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x47eb8a; continue; }
  pc = 0x47eb7a; continue;
 }
 0x47eb7a => {
  pc = 0x47ebad; continue;
 }
 0x47eb7c => {
  { let v: u32 = 0x5d51a8u32; cpu.esi = v; }
  pc = 0x47ebad; continue;
 }
 0x47eb83 => {
  { let v: u32 = 0x5d5268u32; cpu.esi = v; }
  pc = 0x47ebad; continue;
 }
 0x47eb8a => {
  { let v: u32 = 0x5d5328u32; cpu.esi = v; }
  pc = 0x47ebad; continue;
 }
 0x47eb91 => {
  { let v: u32 = 0x5d5028u32; cpu.esi = v; }
  pc = 0x47ebad; continue;
 }
 0x47eb98 => {
  { let v: u32 = 0x5d50e8u32; cpu.esi = v; }
  pc = 0x47ebad; continue;
 }
 0x47eb9f => {
  { let v: u32 = 0x5d4ea8u32; cpu.esi = v; }
  pc = 0x47ebad; continue;
 }
 0x47eba6 => {
  { let v: u32 = 0x5d4f68u32; cpu.esi = v; }
  pc = 0x47ebad; continue;
 }
 0x47ebad => {
  cpu.push(0x47ebb2); f_0047e69a(cpu); if cpu.returned_to != 0x47ebb2 { return; }
  pc = 0x47ebb2; continue;
 }
 0x47ebb2 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x47ebd4; continue; }
  pc = 0x47ebb9; continue;
 }
 0x47ebb9 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x47ebc9; continue; }
  pc = 0x47ebc0; continue;
 }
 0x47ebc0 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x47ebc9; continue; }
  pc = 0x47ebc7; continue;
 }
 0x47ebc7 => {
  pc = 0x47ebea; continue;
 }
 0x47ebc9 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)) & 0x3u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47ec24; continue;
 }
 0x47ebd4 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x47ed4d; continue; }
  pc = 0x47ebe3; continue;
 }
 0x47ebe3 => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47ec24; continue;
 }
 0x47ebea => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x47ebff; continue; }
  pc = 0x47ebf1; continue;
 }
 0x47ebf1 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x47ebff; continue; }
  pc = 0x47ebf8; continue;
 }
 0x47ebf8 => {
  { cpu.and8(cpu.r8(0x39u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x47ec15; continue; }
  pc = 0x47ebff; continue;
 }
 0x47ebff => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x47ed4d; continue; }
  pc = 0x47ec0e; continue;
 }
 0x47ec0e => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47ec24; continue;
 }
 0x47ec15 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x47ed4d; continue; }
  pc = 0x47ec1f; continue;
 }
 0x47ec1f => {
  { let v: u8 = 0x2u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47ec24; continue;
 }
 0x47ec24 => {
  { let v: u32 = 0x5d4d44u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x47ecbf; continue; }
  pc = 0x47ec34; continue;
 }
 0x47ec34 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x47ec9c; continue; }
  pc = 0x47ec3b; continue;
 }
 0x47ec3b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x47ecb1; continue; }
  pc = 0x47ec42; continue;
 }
 0x47ec42 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x47ecb8; continue; }
  pc = 0x47ec49; continue;
 }
 0x47ec49 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x47ecaa; continue; }
  pc = 0x47ec50; continue;
 }
 0x47ec50 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x47ec67; continue; }
  pc = 0x47ec57; continue;
 }
 0x47ec57 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x47eca3; continue; }
  pc = 0x47ec5e; continue;
 }
 0x47ec5e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x47ec90; continue; }
  pc = 0x47ec65; continue;
 }
 0x47ec65 => {
  pc = 0x47ecbf; continue;
 }
 0x47ec67 => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x47ec78; continue; }
  pc = 0x47ec6e; continue;
 }
 0x47ec6e => {
  { let v: u32 = 0x5d51f8u32; cpu.esi = v; }
  cpu.push(0x47ec78); f_0047f24e(cpu); if cpu.returned_to != 0x47ec78 { return; }
  pc = 0x47ec78; continue;
 }
 0x47ec78 => {
  { let v: u32 = 0x5d5158u32; cpu.esi = v; }
  cpu.push(0x47ec82); f_0047e5a0(cpu); if cpu.returned_to != 0x47ec82 { return; }
  pc = 0x47ecc4; continue;
 }
 0x47ec90 => {
  { let v: u32 = 0x5d52d8u32; cpu.esi = v; }
  cpu.push(0x47ec9a); f_0047e5a0(cpu); if cpu.returned_to != 0x47ec9a { return; }
  pc = 0x47ecc4; continue;
 }
 0x47ec9c => {
  { let v: u32 = 0x5d4fd8u32; cpu.esi = v; }
  pc = 0x47ecbf; continue;
 }
 0x47eca3 => {
  { let v: u32 = 0x5d5218u32; cpu.esi = v; }
  pc = 0x47ecbf; continue;
 }
 0x47ecaa => {
  { let v: u32 = 0x5d5098u32; cpu.esi = v; }
  pc = 0x47ecbf; continue;
 }
 0x47ecb1 => {
  { let v: u32 = 0x5d4e58u32; cpu.esi = v; }
  pc = 0x47ecbf; continue;
 }
 0x47ecb8 => {
  { let v: u32 = 0x5d4f18u32; cpu.esi = v; }
  pc = 0x47ecbf; continue;
 }
 0x47ecbf => {
  cpu.push(0x47ecc4); f_0047e504(cpu); if cpu.returned_to != 0x47ecc4 { return; }
  pc = 0x47ecc4; continue;
 }
 0x47ecc4 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47ece2; continue; }
  pc = 0x47ecdb; continue;
 }
 0x47ecdb => {
  cpu.push(0x47ece0); f_0047f077(cpu); if cpu.returned_to != 0x47ece0 { return; }
  pc = 0x47ed4d; continue;
 }
 0x47ece2 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47ed03; continue; }
  pc = 0x47ecfc; continue;
 }
 0x47ecfc => {
  cpu.push(0x47ed01); f_0047ef25(cpu); if cpu.returned_to != 0x47ed01 { return; }
  pc = 0x47ed4d; continue;
 }
 0x47ed03 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47ed2a; continue; }
  pc = 0x47ed23; continue;
 }
 0x47ed23 => {
  cpu.push(0x47ed28); f_0047ee86(cpu); if cpu.returned_to != 0x47ed28 { return; }
  pc = 0x47ed4d; continue;
 }
 0x47ed2a => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47ed4d; continue; }
  pc = 0x47ed48; continue;
 }
 0x47ed48 => {
  cpu.push(0x47ed4d); f_0047ed52(cpu); if cpu.returned_to != 0x47ed4d { return; }
  pc = 0x47ed4d; continue;
 }
 0x47ed4d => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ed52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ed52); }
 let mut pc: u32 = 0x47ed52;
 loop { match pc {
 0x47ed52 => {
  cpu.push(0x47ed57); f_0041c6a6(cpu); if cpu.returned_to != 0x47ed57 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x47ee85; continue; }
  pc = 0x47ed60; continue;
 }
 0x47ed60 => {
  { let v: u32 = 0x5d4de4u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x47ee68; continue; }
  pc = 0x47ed70; continue;
 }
 0x47ed70 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x47ee04; continue; }
  pc = 0x47ed7b; continue;
 }
 0x47ed7b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x47ee36; continue; }
  pc = 0x47ed86; continue;
 }
 0x47ed86 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x47ee4f; continue; }
  pc = 0x47ed91; continue;
 }
 0x47ed91 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x47ee1d; continue; }
  pc = 0x47ed9c; continue;
 }
 0x47ed9c => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x47edd2; continue; }
  pc = 0x47eda3; continue;
 }
 0x47eda3 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x47edeb; continue; }
  pc = 0x47edaa; continue;
 }
 0x47edaa => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x47edb6; continue; }
  pc = 0x47edb1; continue;
 }
 0x47edb1 => {
  pc = 0x47ee68; continue;
 }
 0x47edb6 => {
  { let v: u16 = 0x8u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d5378u32; cpu.esi = v; }
  pc = 0x47ee68; continue;
 }
 0x47edd2 => {
  { let v: u16 = 0x8u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d51f8u32; cpu.esi = v; }
  pc = 0x47ee68; continue;
 }
 0x47edeb => {
  { let v: u16 = 0xau16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d52b8u32; cpu.esi = v; }
  pc = 0x47ee68; continue;
 }
 0x47ee04 => {
  { let v: u16 = 0xau16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d5078u32; cpu.esi = v; }
  pc = 0x47ee68; continue;
 }
 0x47ee1d => {
  { let v: u16 = 0xau16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d5138u32; cpu.esi = v; }
  pc = 0x47ee68; continue;
 }
 0x47ee36 => {
  { let v: u16 = 0x7u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d4ef8u32; cpu.esi = v; }
  pc = 0x47ee68; continue;
 }
 0x47ee4f => {
  { let v: u16 = 0xcu16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d4fb8u32; cpu.esi = v; }
  pc = 0x47ee68; continue;
 }
 0x47ee68 => {
  cpu.push(0x47ee6d); f_0041c6a6(cpu); if cpu.returned_to != 0x47ee6d { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x1u32); cpu.eax = v; }
  if cpu.zf { pc = 0x47ee79; continue; }
  pc = 0x47ee72; continue;
 }
 0x47ee72 => {
  cpu.push(0x47ee77); f_0047f24e(cpu); if cpu.returned_to != 0x47ee77 { return; }
  pc = 0x47ee85; continue;
 }
 0x47ee79 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x47ee85; continue; }
  pc = 0x47ee80; continue;
 }
 0x47ee80 => {
  cpu.push(0x47ee85); f_0047f286(cpu); if cpu.returned_to != 0x47ee85 { return; }
  pc = 0x47ee85; continue;
 }
 0x47ee85 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ee86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ee86); }
 let mut pc: u32 = 0x47ee86;
 loop { match pc {
 0x47ee86 => {
  cpu.push(0x47ee8b); f_0041c6a6(cpu); if cpu.returned_to != 0x47ee8b { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x47ef24; continue; }
  pc = 0x47ee94; continue;
 }
 0x47ee94 => {
  { let v: u32 = 0x5d4de4u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x47ef06; continue; }
  pc = 0x47eea0; continue;
 }
 0x47eea0 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x47eeea; continue; }
  pc = 0x47eea7; continue;
 }
 0x47eea7 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x47eef8; continue; }
  pc = 0x47eeae; continue;
 }
 0x47eeae => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x47eeff; continue; }
  pc = 0x47eeb5; continue;
 }
 0x47eeb5 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x47eef1; continue; }
  pc = 0x47eebc; continue;
 }
 0x47eebc => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x47eee3; continue; }
  pc = 0x47eec3; continue;
 }
 0x47eec3 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x47eedc; continue; }
  pc = 0x47eeca; continue;
 }
 0x47eeca => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x47eed5; continue; }
  pc = 0x47eed1; continue;
 }
 0x47eed1 => {
  pc = 0x47ef06; continue;
 }
 0x47eed5 => {
  { let v: u32 = 0x5d5378u32; cpu.esi = v; }
  pc = 0x47ef06; continue;
 }
 0x47eedc => {
  { let v: u32 = 0x5d52b8u32; cpu.esi = v; }
  pc = 0x47ef06; continue;
 }
 0x47eee3 => {
  { let v: u32 = 0x5d51f8u32; cpu.esi = v; }
  pc = 0x47ef06; continue;
 }
 0x47eeea => {
  { let v: u32 = 0x5d5078u32; cpu.esi = v; }
  pc = 0x47ef06; continue;
 }
 0x47eef1 => {
  { let v: u32 = 0x5d5138u32; cpu.esi = v; }
  pc = 0x47ef06; continue;
 }
 0x47eef8 => {
  { let v: u32 = 0x5d4ef8u32; cpu.esi = v; }
  pc = 0x47ef06; continue;
 }
 0x47eeff => {
  { let v: u32 = 0x5d4fb8u32; cpu.esi = v; }
  pc = 0x47ef06; continue;
 }
 0x47ef06 => {
  cpu.push(0x47ef0b); f_0041c6a6(cpu); if cpu.returned_to != 0x47ef0b { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47ef18; continue; }
  pc = 0x47ef11; continue;
 }
 0x47ef11 => {
  cpu.push(0x47ef16); f_0047f24e(cpu); if cpu.returned_to != 0x47ef16 { return; }
  pc = 0x47ef24; continue;
 }
 0x47ef18 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x47ef24; continue; }
  pc = 0x47ef1f; continue;
 }
 0x47ef1f => {
  cpu.push(0x47ef24); f_0047f286(cpu); if cpu.returned_to != 0x47ef24 { return; }
  pc = 0x47ef24; continue;
 }
 0x47ef24 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ef25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ef25); }
 let mut pc: u32 = 0x47ef25;
 loop { match pc {
 0x47ef25 => {
  { let v: u16 = 0xcu16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d4de4u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x47f03d; continue; }
  pc = 0x47ef47; continue;
 }
 0x47ef47 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x47efdb; continue; }
  pc = 0x47ef52; continue;
 }
 0x47ef52 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x47f00d; continue; }
  pc = 0x47ef5d; continue;
 }
 0x47ef5d => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x47f026; continue; }
  pc = 0x47ef68; continue;
 }
 0x47ef68 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x47eff4; continue; }
  pc = 0x47ef73; continue;
 }
 0x47ef73 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x47efc2; continue; }
  pc = 0x47ef7a; continue;
 }
 0x47ef7a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x47efa9; continue; }
  pc = 0x47ef81; continue;
 }
 0x47ef81 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x47ef8d; continue; }
  pc = 0x47ef88; continue;
 }
 0x47ef88 => {
  pc = 0x47f03d; continue;
 }
 0x47ef8d => {
  { let v: u16 = 0x8u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d5378u32; cpu.esi = v; }
  pc = 0x47f03d; continue;
 }
 0x47efa9 => {
  { let v: u16 = 0xau16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d52b8u32; cpu.esi = v; }
  pc = 0x47f03d; continue;
 }
 0x47efc2 => {
  { let v: u16 = 0x8u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d51f8u32; cpu.esi = v; }
  pc = 0x47f03d; continue;
 }
 0x47efdb => {
  { let v: u16 = 0xau16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d5078u32; cpu.esi = v; }
  pc = 0x47f03d; continue;
 }
 0x47eff4 => {
  { let v: u16 = 0xau16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d5138u32; cpu.esi = v; }
  pc = 0x47f03d; continue;
 }
 0x47f00d => {
  { let v: u16 = 0x7u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d4ef8u32; cpu.esi = v; }
  pc = 0x47f03d; continue;
 }
 0x47f026 => {
  { let v: u16 = 0xcu16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d4fb8u32; cpu.esi = v; }
  pc = 0x47f03d; continue;
 }
 0x47f03d => {
  cpu.push(0x47f042); f_0041c6a6(cpu); if cpu.returned_to != 0x47f042 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x7u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x47f04d; continue; }
  pc = 0x47f048; continue;
 }
 0x47f048 => {
  cpu.push(0x47f04d); f_0047e63e(cpu); if cpu.returned_to != 0x47f04d { return; }
  pc = 0x47f04d; continue;
 }
 0x47f04d => {
  cpu.push(0x47f052); f_0041c6a6(cpu); if cpu.returned_to != 0x47f052 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x47f076; continue; }
  pc = 0x47f058; continue;
 }
 0x47f058 => {
  cpu.push(0x47f05d); f_0041c6a6(cpu); if cpu.returned_to != 0x47f05d { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47f06a; continue; }
  pc = 0x47f063; continue;
 }
 0x47f063 => {
  cpu.push(0x47f068); f_0047f2bf(cpu); if cpu.returned_to != 0x47f068 { return; }
  pc = 0x47f076; continue;
 }
 0x47f06a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x47f076; continue; }
  pc = 0x47f071; continue;
 }
 0x47f071 => {
  cpu.push(0x47f076); f_0047f2fa(cpu); if cpu.returned_to != 0x47f076 { return; }
  pc = 0x47f076; continue;
 }
 0x47f076 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f077(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f077); }
 let mut pc: u32 = 0x47f077;
 loop { match pc {
 0x47f077 => {
  { let v: u16 = 0xcu16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d4de4u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x47f18f; continue; }
  pc = 0x47f099; continue;
 }
 0x47f099 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x47f12d; continue; }
  pc = 0x47f0a4; continue;
 }
 0x47f0a4 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x47f15f; continue; }
  pc = 0x47f0af; continue;
 }
 0x47f0af => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x47f178; continue; }
  pc = 0x47f0ba; continue;
 }
 0x47f0ba => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x47f146; continue; }
  pc = 0x47f0c5; continue;
 }
 0x47f0c5 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x47f114; continue; }
  pc = 0x47f0cc; continue;
 }
 0x47f0cc => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x47f0fb; continue; }
  pc = 0x47f0d3; continue;
 }
 0x47f0d3 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x47f0df; continue; }
  pc = 0x47f0da; continue;
 }
 0x47f0da => {
  pc = 0x47f18f; continue;
 }
 0x47f0df => {
  { let v: u16 = 0x8u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d5378u32; cpu.esi = v; }
  pc = 0x47f18f; continue;
 }
 0x47f0fb => {
  { let v: u16 = 0xau16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d52b8u32; cpu.esi = v; }
  pc = 0x47f18f; continue;
 }
 0x47f114 => {
  { let v: u16 = 0x8u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d51f8u32; cpu.esi = v; }
  pc = 0x47f18f; continue;
 }
 0x47f12d => {
  { let v: u16 = 0xau16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d5078u32; cpu.esi = v; }
  pc = 0x47f18f; continue;
 }
 0x47f146 => {
  { let v: u16 = 0xau16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x5d5138u32; cpu.esi = v; }
  pc = 0x47f18f; continue;
 }
 0x47f15f => {
  { let v: u32 = 0x5d4ef8u32; cpu.esi = v; }
  { let v: u16 = 0x7u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x4d6936u32, v); }
  pc = 0x47f18f; continue;
 }
 0x47f178 => {
  { let v: u32 = 0x5d4ef8u32; cpu.esi = v; }
  { let v: u16 = 0xcu16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x4d6936u32, v); }
  pc = 0x47f18f; continue;
 }
 0x47f18f => {
  cpu.push(0x47f194); f_0041c6a6(cpu); if cpu.returned_to != 0x47f194 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0xfu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x47f19f; continue; }
  pc = 0x47f19a; continue;
 }
 0x47f19a => {
  cpu.push(0x47f19f); f_0047e63e(cpu); if cpu.returned_to != 0x47f19f { return; }
  pc = 0x47f19f; continue;
 }
 0x47f19f => {
  cpu.push(0x47f1a4); f_0041c6a6(cpu); if cpu.returned_to != 0x47f1a4 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x47f1c8; continue; }
  pc = 0x47f1aa; continue;
 }
 0x47f1aa => {
  cpu.push(0x47f1af); f_0041c6a6(cpu); if cpu.returned_to != 0x47f1af { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47f1bc; continue; }
  pc = 0x47f1b5; continue;
 }
 0x47f1b5 => {
  cpu.push(0x47f1ba); f_0047f336(cpu); if cpu.returned_to != 0x47f1ba { return; }
  pc = 0x47f1c8; continue;
 }
 0x47f1bc => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x47f1c8; continue; }
  pc = 0x47f1c3; continue;
 }
 0x47f1c3 => {
  cpu.push(0x47f1c8); f_0047f372(cpu); if cpu.returned_to != 0x47f1c8 { return; }
  pc = 0x47f1c8; continue;
 }
 0x47f1c8 => {
  cpu.push(0x47f1cd); f_0041c6a6(cpu); if cpu.returned_to != 0x47f1cd { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47f1f1; continue; }
  pc = 0x47f1d3; continue;
 }
 0x47f1d3 => {
  cpu.push(0x47f1d8); f_0041c6a6(cpu); if cpu.returned_to != 0x47f1d8 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x47f1e5; continue; }
  pc = 0x47f1de; continue;
 }
 0x47f1de => {
  cpu.push(0x47f1e3); f_0047f2bf(cpu); if cpu.returned_to != 0x47f1e3 { return; }
  pc = 0x47f1f1; continue;
 }
 0x47f1e5 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x47f1f1; continue; }
  pc = 0x47f1ec; continue;
 }
 0x47f1ec => {
  cpu.push(0x47f1f1); f_0047f2fa(cpu); if cpu.returned_to != 0x47f1f1 { return; }
  pc = 0x47f1f1; continue;
 }
 0x47f1f1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f231(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f231); }
 let mut pc: u32 = 0x47f231;
 loop { match pc {
 0x47f231 => {
  { cpu.pushad(); }
  cpu.push(0x47f237); f_0047f5a2(cpu); if cpu.returned_to != 0x47f237 { return; }
  if cpu.cf { pc = 0x47f24c; continue; }
  pc = 0x47f239; continue;
 }
 0x47f239 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x47f24c; continue;
 }
 0x47f24c => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f24e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f24e); }
 let mut pc: u32 = 0x47f24e;
 loop { match pc {
 0x47f24e => {
  cpu.push(0x47f253); f_0047e74b(cpu); if cpu.returned_to != 0x47f253 { return; }
  if cpu.cf { pc = 0x47f285; continue; }
  pc = 0x47f255; continue;
 }
 0x47f255 => {
  cpu.push(0x47f25a); f_0047f5a2(cpu); if cpu.returned_to != 0x47f25a { return; }
  if cpu.cf { pc = 0x47f285; continue; }
  pc = 0x47f25c; continue;
 }
 0x47f25c => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x47f285; continue;
 }
 0x47f285 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f286(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f286); }
 let mut pc: u32 = 0x47f286;
 loop { match pc {
 0x47f286 => {
  cpu.push(0x47f28b); f_0047e74b(cpu); if cpu.returned_to != 0x47f28b { return; }
  if cpu.cf { pc = 0x47f2be; continue; }
  pc = 0x47f28d; continue;
 }
 0x47f28d => {
  cpu.push(0x47f292); f_0047f5a2(cpu); if cpu.returned_to != 0x47f292 { return; }
  if cpu.cf { pc = 0x47f2be; continue; }
  pc = 0x47f294; continue;
 }
 0x47f294 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x47f2be; continue;
 }
 0x47f2be => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f2bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f2bf); }
 let mut pc: u32 = 0x47f2bf;
 loop { match pc {
 0x47f2bf => {
  cpu.push(0x47f2c4); f_0047e74b(cpu); if cpu.returned_to != 0x47f2c4 { return; }
  if cpu.cf { pc = 0x47f2f9; continue; }
  pc = 0x47f2c6; continue;
 }
 0x47f2c6 => {
  cpu.push(0x47f2cb); f_0047f5a2(cpu); if cpu.returned_to != 0x47f2cb { return; }
  if cpu.cf { pc = 0x47f2f9; continue; }
  pc = 0x47f2cd; continue;
 }
 0x47f2cd => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x47f2f9; continue;
 }
 0x47f2f9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f2fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f2fa); }
 let mut pc: u32 = 0x47f2fa;
 loop { match pc {
 0x47f2fa => {
  cpu.push(0x47f2ff); f_0047e74b(cpu); if cpu.returned_to != 0x47f2ff { return; }
  if cpu.cf { pc = 0x47f335; continue; }
  pc = 0x47f301; continue;
 }
 0x47f301 => {
  cpu.push(0x47f306); f_0047f5a2(cpu); if cpu.returned_to != 0x47f306 { return; }
  if cpu.cf { pc = 0x47f335; continue; }
  pc = 0x47f308; continue;
 }
 0x47f308 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x47f335; continue;
 }
 0x47f335 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f336(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f336); }
 let mut pc: u32 = 0x47f336;
 loop { match pc {
 0x47f336 => {
  { cpu.pushad(); }
  cpu.push(0x47f33c); f_0047e74b(cpu); if cpu.returned_to != 0x47f33c { return; }
  if cpu.cf { pc = 0x47f370; continue; }
  pc = 0x47f33e; continue;
 }
 0x47f33e => {
  cpu.push(0x47f343); f_0047f5a2(cpu); if cpu.returned_to != 0x47f343 { return; }
  if cpu.cf { pc = 0x47f370; continue; }
  pc = 0x47f345; continue;
 }
 0x47f345 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x80u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x47f370; continue;
 }
 0x47f370 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f372(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f372); }
 let mut pc: u32 = 0x47f372;
 loop { match pc {
 0x47f372 => {
  { cpu.pushad(); }
  cpu.push(0x47f378); f_0047e74b(cpu); if cpu.returned_to != 0x47f378 { return; }
  if cpu.cf { pc = 0x47f3ad; continue; }
  pc = 0x47f37a; continue;
 }
 0x47f37a => {
  cpu.push(0x47f37f); f_0047f5a2(cpu); if cpu.returned_to != 0x47f37f { return; }
  if cpu.cf { pc = 0x47f3ad; continue; }
  pc = 0x47f381; continue;
 }
 0x47f381 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x80u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x47f3ad; continue;
 }
 0x47f3ad => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f3af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f3af); }
 let mut pc: u32 = 0x47f3af;
 loop { match pc {
 0x47f3af => {
  { let v: u32 = 0x520e1cu32; cpu.ebp = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  pc = 0x47f3bf; continue;
 }
 0x47f3bf => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x47f590; continue; }
  pc = 0x47f3cc; continue;
 }
 0x47f3cc => {
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x47f4fc; continue; }
  pc = 0x47f3d7; continue;
 }
 0x47f3d7 => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.sf { pc = 0x47f471; continue; }
  pc = 0x47f3e4; continue;
 }
 0x47f3e4 => {
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x47f405; continue; }
  pc = 0x47f3ed; continue;
 }
 0x47f3ed => {
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  if (cpu.sf == cpu.of) { pc = 0x47f405; continue; }
  pc = 0x47f3f5; continue;
 }
 0x47f3f5 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x47f40e; continue; }
  pc = 0x47f405; continue;
 }
 0x47f405 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x47f446; continue;
 }
 0x47f40e => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4ffu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = 0xffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x47f446; continue;
 }
 0x47f446 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x47f590; continue; }
  pc = 0x47f453; continue;
 }
 0x47f453 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x47f590; continue; }
  pc = 0x47f462; continue;
 }
 0x47f462 => {
  { let v: u8 = 0x5u8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.neg16(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47f590; continue;
 }
 0x47f471 => {
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x47f492; continue; }
  pc = 0x47f47a; continue;
 }
 0x47f47a => {
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  if (cpu.sf == cpu.of) { pc = 0x47f492; continue; }
  pc = 0x47f482; continue;
 }
 0x47f482 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x47f49b; continue; }
  pc = 0x47f492; continue;
 }
 0x47f492 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x47f4d4; continue;
 }
 0x47f49b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x5u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x58bu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = 0xffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x47f4d4; continue;
 }
 0x47f4d4 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x47f590; continue; }
  pc = 0x47f4e1; continue;
 }
 0x47f4e1 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if (cpu.sf != cpu.of) { pc = 0x47f590; continue; }
  pc = 0x47f4f0; continue;
 }
 0x47f4f0 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47f590; continue;
 }
 0x47f4fc => {
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x47f51d; continue; }
  pc = 0x47f505; continue;
 }
 0x47f505 => {
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  if (cpu.sf == cpu.of) { pc = 0x47f51d; continue; }
  pc = 0x47f50d; continue;
 }
 0x47f50d => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x47f526; continue; }
  pc = 0x47f51d; continue;
 }
 0x47f51d => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x47f575; continue;
 }
 0x47f526 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x719u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = 0xffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = 0x5d4d04u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x47f575; continue;
 }
 0x47f575 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x47f590; continue; }
  pc = 0x47f57e; continue;
 }
 0x47f57e => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if (cpu.sf != cpu.of) { pc = 0x47f590; continue; }
  pc = 0x47f589; continue;
 }
 0x47f589 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47f590; continue;
 }
 0x47f590 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47f3bf; continue; }
  pc = 0x47f59b; continue;
 }
 0x47f59b => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f5a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f5a2); }
 let mut pc: u32 = 0x47f5a2;
 loop { match pc {
 0x47f5a2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x520e1cu32; cpu.ebp = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x47f5ad; continue;
 }
 0x47f5ad => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x47f5bd; continue; }
  pc = 0x47f5b5; continue;
 }
 0x47f5b5 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47f5ad; continue; }
  pc = 0x47f5ba; continue;
 }
 0x47f5ba => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47f5bd => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f5c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f5c4); }
 let mut pc: u32 = 0x47f5c4;
 loop { match pc {
 0x47f5c4 => {
  { let v: u32 = 0x56d430u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x47f5ce; continue;
 }
 0x47f5ce => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47f5ce; continue; }
  pc = 0x47f5d4; continue;
 }
 0x47f5d4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f5d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f5d5); }
 let mut pc: u32 = 0x47f5d5;
 loop { match pc {
 0x47f5d5 => {
  { let v: u16 = 0x0u16; cpu.w16(0x56d520u32, v); }
  { let v: u32 = 0x56d430u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x47f5e8; continue;
 }
 0x47f5e8 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x47f5f6; continue; }
  pc = 0x47f5ed; continue;
 }
 0x47f5ed => {
  { let v: u8 = cpu.dec8(cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x56d520u32)); cpu.w16(0x56d520u32, v); }
  pc = 0x47f5f6; continue;
 }
 0x47f5f6 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47f5e8; continue; }
  pc = 0x47f5f9; continue;
 }
 0x47f5f9 => {
  { let v: u16 = cpu.r16(0x56d520u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x56d522u32), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x47f60e; continue; }
  pc = 0x47f608; continue;
 }
 0x47f608 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x56d522u32, v); }
  pc = 0x47f60e; continue;
 }
 0x47f60e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f60f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f60f); }
 let mut pc: u32 = 0x47f60f;
 loop { match pc {
 0x47f60f => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x12u32.wrapping_add(cpu.eax))); }
  if (cpu.sf == cpu.of) { pc = 0x47f6ce; continue; }
  pc = 0x47f618; continue;
 }
 0x47f618 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x5d5470u32, v); }
  pc = 0x47f62f; continue;
 }
 0x47f62f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x47f636); f_0047ff60(cpu); if cpu.returned_to != 0x47f636 { return; }
  if !cpu.cf { pc = 0x47f63f; continue; }
  pc = 0x47f638; continue;
 }
 0x47f638 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x47f6ce; continue;
 }
 0x47f63f => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x47f646); f_00489968(cpu); if cpu.returned_to != 0x47f646 { return; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x581bffu32, v); }
  cpu.push(0x47f657); f_0047f7c2(cpu); if cpu.returned_to != 0x47f657 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5du32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5d5470u32), 0xffu8); }
  if cpu.zf { pc = 0x47f681; continue; }
  pc = 0x47f665; continue;
 }
 0x47f665 => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = cpu.r8(0x5d5470u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x47f67b); f_00480006(cpu); if cpu.returned_to != 0x47f67b { return; }
  { let v: u8 = 0x4u8; cpu.w8(0x3fu32.wrapping_add(cpu.esi), v); }
  pc = 0x47f6bb; continue;
 }
 0x47f681 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5d5471u32, v); }
  cpu.push(0x47f68d); f_0047fcdb(cpu); if cpu.returned_to != 0x47f68d { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5d5471u32, v); }
  cpu.push(0x47f699); f_0047a0b2(cpu); if cpu.returned_to != 0x47f699 { return; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x47f6cc; continue; }
  pc = 0x47f6a2; continue;
 }
 0x47f6a2 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5d5470u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5d5468u32, v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  pc = 0x47f6bb; continue;
 }
 0x47f6bb => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47f62f; continue; }
  pc = 0x47f6c5; continue;
 }
 0x47f6c5 => {
  { let v: u32 = cpu.r32(0x5d5468u32); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47f6cc => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x47f6ce; continue;
 }
 0x47f6ce => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f6d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f6d0); }
 let mut pc: u32 = 0x47f6d0;
 loop { match pc {
 0x47f6d0 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x47f768; continue; }
  pc = 0x47f6da; continue;
 }
 0x47f6da => {
  { let v: u8 = 0xffu8; cpu.w8(0x5d5470u32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x47f6e8); f_00489968(cpu); if cpu.returned_to != 0x47f6e8 { return; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x581bffu32, v); }
  cpu.push(0x47f6f9); f_0047f7c2(cpu); if cpu.returned_to != 0x47f6f9 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5du32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5d5470u32), 0xffu8); }
  if cpu.zf { pc = 0x47f723; continue; }
  pc = 0x47f707; continue;
 }
 0x47f707 => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = cpu.r8(0x5d5470u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x47f71d); f_00480006(cpu); if cpu.returned_to != 0x47f71d { return; }
  { let v: u8 = 0x4u8; cpu.w8(0x3fu32.wrapping_add(cpu.esi), v); }
  pc = 0x47f75d; continue;
 }
 0x47f723 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5d5471u32, v); }
  cpu.push(0x47f72f); f_0047fcdb(cpu); if cpu.returned_to != 0x47f72f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5d5471u32, v); }
  cpu.push(0x47f73b); f_0047a0b2(cpu); if cpu.returned_to != 0x47f73b { return; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x47f766; continue; }
  pc = 0x47f744; continue;
 }
 0x47f744 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5d5470u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5d5468u32, v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  pc = 0x47f75d; continue;
 }
 0x47f75d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5d5468u32); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47f766 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x47f768; continue;
 }
 0x47f768 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f76a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f76a); }
 let mut pc: u32 = 0x47f76a;
 loop { match pc {
 0x47f76a => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x47f7c1; continue; }
  pc = 0x47f771; continue;
 }
 0x47f771 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  pc = 0x47f77c; continue;
 }
 0x47f77c => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x47f7bc; continue; }
  pc = 0x47f781; continue;
 }
 0x47f781 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  cpu.push(0x47f796); f_0043ffdb(cpu); if cpu.returned_to != 0x47f796 { return; }
  cpu.push(0x47f79b); f_0041c743(cpu); if cpu.returned_to != 0x47f79b { return; }
  { let v: u32 = cpu.eax & 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x47f7ab); f_0041c743(cpu); if cpu.returned_to != 0x47f7ab { return; }
  { let v: u32 = cpu.eax & 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x47f7b9); f_00489637(cpu); if cpu.returned_to != 0x47f7b9 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x47f7bc; continue;
 }
 0x47f7bc => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47f77c; continue; }
  pc = 0x47f7c1; continue;
 }
 0x47f7c1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047f7c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47f7c2); }
 let mut pc: u32 = 0x47f7c2;
 loop { match pc {
 0x47f7c2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x581b68u32, v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1559u32), (cpu.eax as u8)); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x47f7fa; continue; }
  pc = 0x47f7e5; continue;
 }
 0x47f7e5 => {
  { let v: u8 = 0x1u8; cpu.w8(0x59d977u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x59d978u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x59d979u32, v); }
  pc = 0x47f7fa; continue;
 }
 0x47f7fa => {
  cpu.push(0x47f7ff); f_0047fb4b(cpu); if cpu.returned_to != 0x47f7ff { return; }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x47f807); f_0045955b(cpu); if cpu.returned_to != 0x47f807 { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0x7fu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x47f86d; continue; }
  pc = 0x47f814; continue;
 }
 0x47f814 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x5cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x47f82d; continue; }
  pc = 0x47f82b; continue;
 }
 0x47f82b => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x47f82d; continue;
 }
 0x47f82d => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x13u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x56d524u32, v); }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x5du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x12u32.wrapping_add(cpu.ebx))); cpu.w8(0x12u32.wrapping_add(cpu.ebx), v); }
  if cpu.zf { pc = 0x47f873; continue; }
  pc = 0x47f858; continue;
 }
 0x47f858 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x47f86e; continue; }
  pc = 0x47f85c; continue;
 }
 0x47f85c => {
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0xfeu8); cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x47f868); f_0047fb95(cpu); if cpu.returned_to != 0x47f868 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x47f873; continue; }
  pc = 0x47f86d; continue;
 }
 0x47f86d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47f86e => {
  { let v: u8 = cpu.or8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0x40u8); cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47f873 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x56d524u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xc8u8; cpu.w8(0x56d430u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x16u32; cpu.eax = v; }
  { let v = cpu.r8(0x56d524u32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x524b80u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x47f8b1; continue; }
  pc = 0x47f8a3; continue;
 }
 0x47f8a3 => {
  cpu.push(0x47f8a8); f_00430114(cpu); if cpu.returned_to != 0x47f8a8 { return; }
  if !cpu.cf { pc = 0x47f8b1; continue; }
  pc = 0x47f8aa; continue;
 }
 0x47f8aa => {
  { let v: u8 = 0x1u8; cpu.w8(0x59d97bu32, v); }
  pc = 0x47f8b1; continue;
 }
 0x47f8b1 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x56c6f6u32)); }
  if !cpu.zf { pc = 0x47f8cd; continue; }
  pc = 0x47f8b9; continue;
 }
 0x47f8b9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x56c6f6u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b358cu32, v); }
  pc = 0x47f8cd; continue;
 }
 0x47f8cd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1570u32.wrapping_add(cpu.eax))); cpu.w8(0x4e1570u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x47f8f1; continue; }
  pc = 0x47f8e4; continue;
 }
 0x47f8e4 => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x47f8f1; continue; }
  pc = 0x47f8e8; continue;
 }
 0x47f8e8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x47f8ef); f_004496fc(cpu); if cpu.returned_to != 0x47f8ef { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x47f8f1; continue;
 }
 0x47f8f1 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x4d6c10u32)); }
  if !cpu.zf { pc = 0x47f903; continue; }
  pc = 0x47f8f9; continue;
 }
 0x47f8f9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c10u32, v); }
  pc = 0x47f903; continue;
 }
 0x47f903 => {
  { cpu.sub32(cpu.r32(0x5d546cu32), 0xffffffffu32); }
  if cpu.zf { pc = 0x47f967; continue; }
  pc = 0x47f910; continue;
 }
 0x47f910 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1559u32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x47f93a; continue; }
  pc = 0x47f921; continue;
 }
 0x47f921 => {
  { let v: u32 = cpu.r32(0x5d546cu32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1559u32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x47f93a; continue; }
  pc = 0x47f935; continue;
 }
 0x47f935 => {
  cpu.push(0x47f93a); f_004166ad(cpu); if cpu.returned_to != 0x47f93a { return; }
  pc = 0x47f93a; continue;
 }
 0x47f93a => {
  { cpu.popad(); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5d546cu32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x47f95c; continue; }
  pc = 0x47f947; continue;
 }
 0x47f947 => {
  { cpu.sub32(cpu.ebp, 0x509290u32); }
  if (cpu.sf != cpu.of) { pc = 0x47f95c; continue; }
  pc = 0x47f94f; continue;
 }
 0x47f94f => {
  { cpu.sub32(cpu.ebp, 0x513f10u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47f95c; continue; }
  pc = 0x47f957; continue;
 }
 0x47f957 => {
  cpu.push(0x47f95c); f_00454204(cpu); if cpu.returned_to != 0x47f95c { return; }
  pc = 0x47f95c; continue;
 }
 0x47f95c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5d546cu32, v); }
  pc = 0x47f967; continue;
 }
 0x47f967 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x56d524u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x47f977); f_0044c7f9(cpu); if cpu.returned_to != 0x47f977 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x47fa76; continue; }
  pc = 0x47f97f; continue;
 }
 0x47f97f => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x47f985); f_0044c896(cpu); if cpu.returned_to != 0x47f985 { return; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x4d6758u32)); }
  if !cpu.zf { pc = 0x47fa76; continue; }
  pc = 0x47f999; continue;
 }
 0x47f999 => {
  { let v: u16 = cpu.dec16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x47f9b5; continue; }
  pc = 0x47f9a0; continue;
 }
 0x47f9a0 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x47fa49; continue;
 }
 0x47f9b5 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x56d38au32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x56d38bu32, v); }
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x47f9cf; continue;
 }
 0x47f9cf => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if cpu.zf { pc = 0x47fa3a; continue; }
  pc = 0x47f9d8; continue;
 }
 0x47f9d8 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x47fa3a; continue; }
  pc = 0x47f9eb; continue;
 }
 0x47f9eb => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x509290u32); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x56d38bu32)); }
  if !cpu.cf { pc = 0x47fa3a; continue; }
  pc = 0x47fa2e; continue;
 }
 0x47fa2e => {
  { let v: u32 = cpu.ebx; cpu.w32(0x56d38bu32, v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x56d38au32, v); }
  pc = 0x47fa3a; continue;
 }
 0x47fa3a => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47f9cf; continue; }
  pc = 0x47fa3f; continue;
 }
 0x47fa3f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x56d38au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x47fa76; continue; }
  pc = 0x47fa49; continue;
 }
 0x47fa49 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x56d389u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47fa5a); f_0041a8a2(cpu); if cpu.returned_to != 0x47fa5a { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x56d389u32, v); }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x509290u32); cpu.ebp = v; }
  cpu.push(0x47fa76); f_0044c686(cpu); if cpu.returned_to != 0x47fa76 { return; }
  pc = 0x47fa76; continue;
 }
 0x47fa76 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x4d6758u32)); }
  if !cpu.zf { pc = 0x47fa8a; continue; }
  pc = 0x47fa85; continue;
 }
 0x47fa85 => {
  cpu.push(0x47fa8a); f_0041b24f(cpu); if cpu.returned_to != 0x47fa8a { return; }
  pc = 0x47fa8a; continue;
 }
 0x47fa8a => {
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) & 0xfeu8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x28u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2au32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x56d524u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x4d6b90u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x581bffu32), 0x0u8); }
  if cpu.zf { pc = 0x47fb48; continue; }
  pc = 0x47fabb; continue;
 }
 0x47fabb => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x47fb0a; continue; }
  pc = 0x47faca; continue;
 }
 0x47faca => {
  { let v: u16 = cpu.inc16(cpu.r16(0x4b94c2u32)); cpu.w16(0x4b94c2u32, v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x56d32eu32)); cpu.w16(0x56d32eu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6a38u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x47faf7; continue; }
  pc = 0x47fae6; continue;
 }
 0x47fae6 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x61a8u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47faf7; continue; }
  pc = 0x47faf0; continue;
 }
 0x47faf0 => {
  cpu.push(0x47faf5); f_00485b77(cpu); if cpu.returned_to != 0x47faf5 { return; }
  pc = 0x47fb00; continue;
 }
 0x47faf7 => {
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x47fb00; continue;
 }
 0x47fb00 => {
  { let v: u32 = cpu.add32(cpu.r32(0x4b94c8u32), cpu.eax); cpu.w32(0x4b94c8u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x47fb48; continue;
 }
 0x47fb0a => {
  { let v: u16 = cpu.inc16(cpu.r16(0x4b94c0u32)); cpu.w16(0x4b94c0u32, v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x56d328u32)); cpu.w16(0x56d328u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6a38u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x47fb37; continue; }
  pc = 0x47fb26; continue;
 }
 0x47fb26 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x61a8u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47fb37; continue; }
  pc = 0x47fb30; continue;
 }
 0x47fb30 => {
  cpu.push(0x47fb35); f_00485b77(cpu); if cpu.returned_to != 0x47fb35 { return; }
  pc = 0x47fb40; continue;
 }
 0x47fb37 => {
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x47fb40; continue;
 }
 0x47fb40 => {
  { let v: u32 = cpu.add32(cpu.r32(0x4b94c4u32), cpu.eax); cpu.w32(0x4b94c4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x47fb48; continue;
 }
 0x47fb48 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047fb4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47fb4b); }
 let mut pc: u32 = 0x47fb4b;
 loop { match pc {
 0x47fb4b => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5d546cu32, v); }
  { let v: u32 = 0x52bbf0u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x47fb6a; continue;
 }
 0x47fb6a => {
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x47fb8f; continue; }
  pc = 0x47fb6f; continue;
 }
 0x47fb6f => {
  { let v: u8 = 0xffu8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x47fb8e; continue; }
  pc = 0x47fb7a; continue;
 }
 0x47fb7a => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x5d546cu32, v); }
  pc = 0x47fb8e; continue;
 }
 0x47fb8e => {
  { cpu.popad(); }
  pc = 0x47fb8f; continue;
 }
 0x47fb8f => {
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47fb6a; continue; }
  pc = 0x47fb94; continue;
 }
 0x47fb94 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047fb95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47fb95); }
 let mut pc: u32 = 0x47fb95;
 loop { match pc {
 0x47fb95 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 1); }
  if cpu.zf { pc = 0x47fc42; continue; }
  pc = 0x47fba9; continue;
 }
 0x47fba9 => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x13u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x47fbb8); f_0047fc76(cpu); if cpu.returned_to != 0x47fbb8 { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x5au32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5fu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.pushad(); }
  cpu.push(0x47fbf2); f_00489968(cpu); if cpu.returned_to != 0x47fbf2 { return; }
  { cpu.popad(); }
  { let v: u8 = 0xffu8; cpu.w8(0x5cu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x47fc00); f_0042f53c(cpu); if cpu.returned_to != 0x47fc00 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x47fc19; continue; }
  pc = 0x47fc08; continue;
 }
 0x47fc08 => {
  { cpu.sub8((cpu.ecx as u8), 0x5u8); }
  if cpu.zf { pc = 0x47fc14; continue; }
  pc = 0x47fc0d; continue;
 }
 0x47fc0d => {
  { cpu.sub8((cpu.ecx as u8), 0xfbu8); }
  if cpu.zf { pc = 0x47fc14; continue; }
  pc = 0x47fc12; continue;
 }
 0x47fc12 => {
  pc = 0x47fc19; continue;
 }
 0x47fc14 => {
  cpu.push(0x47fc19); f_0045427c(cpu); if cpu.returned_to != 0x47fc19 { return; }
  pc = 0x47fc19; continue;
 }
 0x47fc19 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x54u32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x52u32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x50u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if !cpu.zf { pc = 0x47fc3e; continue; }
  pc = 0x47fc39; continue;
 }
 0x47fc39 => {
  cpu.push(0x47fc3e); f_0043f7a4(cpu); if cpu.returned_to != 0x47fc3e { return; }
  pc = 0x47fc3e; continue;
 }
 0x47fc3e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47fc42 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x28u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2au32.wrapping_add(cpu.ebx), v); }
  { cpu.pushad(); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x4d6758u32)); }
  if !cpu.zf { pc = 0x47fc6c; continue; }
  pc = 0x47fc67; continue;
 }
 0x47fc67 => {
  cpu.push(0x47fc6c); f_0041879e(cpu); if cpu.returned_to != 0x47fc6c { return; }
  pc = 0x47fc6c; continue;
 }
 0x47fc6c => {
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x47fc74); f_00489968(cpu); if cpu.returned_to != 0x47fc74 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047fc76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47fc76); }
 let mut pc: u32 = 0x47fc76;
 loop { match pc {
 0x47fc76 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x4d6758u32)); }
  if !cpu.zf { pc = 0x47fc98; continue; }
  pc = 0x47fc88; continue;
 }
 0x47fc88 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6758u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d688au32, v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x47fc98; continue;
 }
 0x47fc98 => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x4d6888u32)); }
  if !cpu.zf { pc = 0x47fcab; continue; }
  pc = 0x47fca1; continue;
 }
 0x47fca1 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6888u32, v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x47fcab; continue;
 }
 0x47fcab => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4d6c10u32), cpu.eax); }
  if !cpu.zf { pc = 0x47fcd7; continue; }
  pc = 0x47fcc5; continue;
 }
 0x47fcc5 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c10u32, v); }
  pc = 0x47fcd7; continue;
 }
 0x47fcd7 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047fcdb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47fcdb); } r_0047fcdb(cpu, 0x47fcdb); }
pub fn f_0047fd3c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47fd3c); } r_0047fcdb(cpu, 0x47fd3c); }
fn r_0047fcdb(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x47fcdb => {
  { cpu.sub8(cpu.r8(0x4e1fcau32), 0x0u8); }
  if !cpu.zf { pc = 0x47fcf9; continue; }
  pc = 0x47fce4; continue;
 }
 0x47fce4 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  pc = 0x47fcf9; continue;
 }
 0x47fcf9 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x47fd10; continue; }
  pc = 0x47fd02; continue;
 }
 0x47fd02 => {
  { let v: u32 = 0x52bbf0u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x47fd3c; continue;
 }
 0x47fd10 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x47fd02; continue; }
  pc = 0x47fd1a; continue;
 }
 0x47fd1a => {
  { let v: u8 = 0x78u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  cpu.push(0x47fd34); f_0047fd3c(cpu); if cpu.returned_to != 0x47fd34 { return; }
  if !cpu.cf { pc = 0x47fdd3; continue; }
  pc = 0x47fd3a; continue;
 }
 0x47fd3a => {
  cpu.poll(); pc = 0x47fd02; continue;
 }
 0x47fd3c => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x47fd59; continue; }
  pc = 0x47fd42; continue;
 }
 0x47fd42 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { pc = 0x47fd59; continue; }
  pc = 0x47fd48; continue;
 }
 0x47fd48 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x56d430u32.wrapping_add(cpu.ebx)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.zf { pc = 0x47fd84; continue; }
  pc = 0x47fd59; continue;
 }
 0x47fd59 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x48u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47fd3c; continue; }
  pc = 0x47fd60; continue;
 }
 0x47fd60 => {
  {  }
  { let v: u32 = 0x52bbf0u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x47fd6d; continue;
 }
 0x47fd6d => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x47fd7b; continue; }
  pc = 0x47fd73; continue;
 }
 0x47fd73 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { pc = 0x47fd7b; continue; }
  pc = 0x47fd79; continue;
 }
 0x47fd79 => {
  pc = 0x47fd84; continue;
 }
 0x47fd7b => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47fd6d; continue; }
  pc = 0x47fd82; continue;
 }
 0x47fd82 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47fd84 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x4e1570u32.wrapping_add(cpu.eax))); cpu.w8(0x4e1570u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6a1cu32, v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.ebx >> 8) as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x47fdac); f_00480006(cpu); if cpu.returned_to != 0x47fdac { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.pushad(); }
  cpu.push(0x47fdb4); f_0047a0b2(cpu); if cpu.returned_to != 0x47fdb4 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x4u8; cpu.w8(0x3fu32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x4e1fcau32), 0x0u8); }
  if !cpu.zf { pc = 0x47fdd3; continue; }
  pc = 0x47fdc2; continue;
 }
 0x47fdc2 => {
  { cpu.sub8(cpu.r8(0x5d5471u32), 0x0u8); }
  if cpu.zf { pc = 0x47fdd5; continue; }
  pc = 0x47fdcb; continue;
 }
 0x47fdcb => {
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x47fdd3; continue;
 }
 0x47fdd3 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47fdd5 => {
  { let v: u32 = 0x52ff70u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x47fddf; continue;
 }
 0x47fddf => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x47fdec; continue; }
  pc = 0x47fde4; continue;
 }
 0x47fde4 => {
  { let v: u32 = cpu.esi.wrapping_add(0x52u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47fddf; continue; }
  pc = 0x47fde9; continue;
 }
 0x47fde9 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x52u32); cpu.esi = v; }
  pc = 0x47fdec; continue;
 }
 0x47fdec => {
  { let v = cpu.r32(0x4d6bd8u32); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x2u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x47fe1b; continue; }
  pc = 0x47fe07; continue;
 }
 0x47fe07 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x47fe1b; continue; }
  pc = 0x47fe0e; continue;
 }
 0x47fe0e => {
  { let v: u8 = cpu.r8(0x79u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5d55afu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x47fe26; continue;
 }
 0x47fe1b => {
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5d5477u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x47fe26; continue;
 }
 0x47fe26 => {
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047fe31(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47fe31); }
 let mut pc: u32 = 0x47fe31;
 loop { match pc {
 0x47fe31 => {
  { cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x47fe52; continue; }
  pc = 0x47fe39; continue;
 }
 0x47fe39 => {
  cpu.push(0x47fe3e); f_00489897(cpu); if cpu.returned_to != 0x47fe3e { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x47fe52; continue; }
  pc = 0x47fe44; continue;
 }
 0x47fe44 => {
  cpu.push(0x47fe49); f_00489881(cpu); if cpu.returned_to != 0x47fe49 { return; }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)) & 0x3u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47fe52; continue;
 }
 0x47fe52 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x47fe87; continue; }
  pc = 0x47fe5b; continue;
 }
 0x47fe5b => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x47fe62); f_0043ff77(cpu); if cpu.returned_to != 0x47fe62 { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x47fefe; continue; }
  pc = 0x47fe6b; continue;
 }
 0x47fe6b => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47fefe; continue; }
  pc = 0x47fe77; continue;
 }
 0x47fe77 => {
  cpu.push(0x47fe7c); f_00458731(cpu); if cpu.returned_to != 0x47fe7c { return; }
  if cpu.cf { pc = 0x47fefe; continue; }
  pc = 0x47fe82; continue;
 }
 0x47fe82 => {
  pc = 0x47ff5f; continue;
 }
 0x47fe87 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x47fefe; continue; }
  pc = 0x47fe92; continue;
 }
 0x47fe92 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x47fed1; continue; }
  pc = 0x47fe9e; continue;
 }
 0x47fe9e => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x47fed1; continue; }
  pc = 0x47fea4; continue;
 }
 0x47fea4 => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x47fec7; continue; }
  pc = 0x47feba; continue;
 }
 0x47feba => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.esi)), 0xfbu8); }
  if cpu.zf { pc = 0x47fec7; continue; }
  pc = 0x47fec0; continue;
 }
 0x47fec0 => {
  cpu.push(0x47fec5); f_0045bbbe(cpu); if cpu.returned_to != 0x47fec5 { return; }
  if cpu.cf { pc = 0x47fed1; continue; }
  pc = 0x47fec7; continue;
 }
 0x47fec7 => {
  cpu.push(0x47fecc); f_0045302a(cpu); if cpu.returned_to != 0x47fecc { return; }
  pc = 0x47ff5f; continue;
 }
 0x47fed1 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x47fefe; continue; }
  pc = 0x47fed7; continue;
 }
 0x47fed7 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x47fede); f_0043fede(cpu); if cpu.returned_to != 0x47fede { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x47fefe; continue; }
  pc = 0x47fee3; continue;
 }
 0x47fee3 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47fefe; continue; }
  pc = 0x47feeb; continue;
 }
 0x47feeb => {
  cpu.push(0x47fef0); f_004782f8(cpu); if cpu.returned_to != 0x47fef0 { return; }
  if cpu.cf { pc = 0x47fefe; continue; }
  pc = 0x47fef2; continue;
 }
 0x47fef2 => {
  cpu.push(0x47fef7); f_0045b5ab(cpu); if cpu.returned_to != 0x47fef7 { return; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x47ff4b; continue; }
  pc = 0x47fefe; continue;
 }
 0x47fefe => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47ff11); f_0043fced(cpu); if cpu.returned_to != 0x47ff11 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0x7u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x47ff32; continue; }
  pc = 0x47ff29; continue;
 }
 0x47ff29 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x47ff32); f_00451950(cpu); if cpu.returned_to != 0x47ff32 { return; }
  pc = 0x47ff32; continue;
 }
 0x47ff32 => {
  { let v: u32 = cpu.r32(0x4d6a28u32); cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x47ff4c; continue; }
  pc = 0x47ff40; continue;
 }
 0x47ff40 => {
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if !cpu.zf { pc = 0x47ff4c; continue; }
  pc = 0x47ff46; continue;
 }
 0x47ff46 => {
  cpu.push(0x47ff4b); f_0043ffdb(cpu); if cpu.returned_to != 0x47ff4b { return; }
  pc = 0x47ff4b; continue;
 }
 0x47ff4b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47ff4c => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x47ff5f; continue; }
  pc = 0x47ff53; continue;
 }
 0x47ff53 => {
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x47ff5f); f_00451950(cpu); if cpu.returned_to != 0x47ff5f { return; }
  pc = 0x47ff5f; continue;
 }
 0x47ff5f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ff60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ff60); }
 let mut pc: u32 = 0x47ff60;
 loop { match pc {
 0x47ff60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bc17u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x14u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x47ff73; continue;
 }
 0x47ff73 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if !cpu.zf { pc = 0x47ff81; continue; }
  pc = 0x47ff78; continue;
 }
 0x47ff78 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x47ff73; continue; }
  pc = 0x47ff7d; continue;
 }
 0x47ff7d => {
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47ff81 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x509290u32); cpu.eax = v; }
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ff91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47ff91); }
 let mut pc: u32 = 0x47ff91;
 loop { match pc {
 0x47ff91 => {
  { let v: u32 = 0x4b8dfcu32; cpu.esi = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u32 = 0x4efa88u32; cpu.ebp = v; }
  { let v: u32 = 0x5d5472u32; cpu.edx = v; }
  pc = 0x47ffa5; continue;
 }
 0x47ffa5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x4e15c6u32, v); }
  cpu.push(0x47ffb6); f_0047ffc7(cpu); if cpu.returned_to != 0x47ffb6 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xe1u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x28u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47ffa5; continue; }
  pc = 0x47ffc6; continue;
 }
 0x47ffc6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047ffc7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47ffc7); } r_0047ffc7(cpu, 0x47ffc7); }
pub fn f_00480000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480000); } r_0047ffc7(cpu, 0x480000); }
fn r_0047ffc7(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x47ffc7 => {
  { let v: u32 = 0xe1u32; cpu.ecx = v; }
  { let v: u8 = 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x47ffd0; continue;
 }
 0x47ffd0 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x47fff8; continue; }
  pc = 0x47ffd5; continue;
 }
 0x47ffd5 => {
  if cpu.sf { pc = 0x47fff8; continue; }
  pc = 0x47ffd7; continue;
 }
 0x47ffd7 => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4e15c6u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), (cpu.ecx & 0xff)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8(((cpu.edx >> 8) as u8), (cpu.ecx & 0xff)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x47fff8; continue;
 }
 0x47fff8 => {
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x480003; continue; }
  pc = 0x47ffff; continue;
 }
 0x47ffff => {
  { let v: u8 = 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x480003; continue;
 }
 0x480000 => {
  { cpu.cf = true; }
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x480003; continue;
 }
 0x480003 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47ffd0; continue; }
  pc = 0x480005; continue;
 }
 0x480005 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480004(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480004); }
 let mut pc: u32 = 0x480004;
 loop { match pc {
 0x480004 => {
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480006(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480006); } r_00480006(cpu, 0x480006); }
pub fn f_00480008(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480008); } r_00480006(cpu, 0x480008); }
pub fn f_00480012(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480012); } r_00480006(cpu, 0x480012); }
pub fn f_0048001c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48001c); } r_00480006(cpu, 0x48001c); }
pub fn f_00480020(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480020); } r_00480006(cpu, 0x480020); }
pub fn f_00480021(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480021); } r_00480006(cpu, 0x480021); }
pub fn f_00480022(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480022); } r_00480006(cpu, 0x480022); }
pub fn f_00480023(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480023); } r_00480006(cpu, 0x480023); }
pub fn f_00480025(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480025); } r_00480006(cpu, 0x480025); }
pub fn f_00480026(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480026); } r_00480006(cpu, 0x480026); }
pub fn f_0048002c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48002c); } r_00480006(cpu, 0x48002c); }
pub fn f_0048002d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48002d); } r_00480006(cpu, 0x48002d); }
pub fn f_0048002f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48002f); } r_00480006(cpu, 0x48002f); }
pub fn f_00480030(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480030); } r_00480006(cpu, 0x480030); }
pub fn f_00480031(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480031); } r_00480006(cpu, 0x480031); }
pub fn f_00480032(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480032); } r_00480006(cpu, 0x480032); }
pub fn f_00480034(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480034); } r_00480006(cpu, 0x480034); }
pub fn f_0048003c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48003c); } r_00480006(cpu, 0x48003c); }
pub fn f_00480041(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480041); } r_00480006(cpu, 0x480041); }
pub fn f_00480046(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480046); } r_00480006(cpu, 0x480046); }
pub fn f_00480048(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480048); } r_00480006(cpu, 0x480048); }
pub fn f_0048004c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48004c); } r_00480006(cpu, 0x48004c); }
pub fn f_0048005e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48005e); } r_00480006(cpu, 0x48005e); }
pub fn f_00480069(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480069); } r_00480006(cpu, 0x480069); }
pub fn f_00480073(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480073); } r_00480006(cpu, 0x480073); }
pub fn f_00480080(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480080); } r_00480006(cpu, 0x480080); }
pub fn f_00480090(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480090); } r_00480006(cpu, 0x480090); }
pub fn f_0048009e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48009e); } r_00480006(cpu, 0x48009e); }
pub fn f_004800a0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4800a0); } r_00480006(cpu, 0x4800a0); }
pub fn f_004800ac(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4800ac); } r_00480006(cpu, 0x4800ac); }
fn r_00480006(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x480006 => {
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x48008d; continue; }
  pc = 0x48000f; continue;
 }
 0x480008 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), ((cpu.ebx >> 8) as u8)); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x48000f; continue;
 }
 0x48000f => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x52bbf0u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x480020; continue;
 }
 0x480012 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x52bbf0u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x480020; continue;
 }
 0x48001c => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.add8(cpu.r8(0xf6b8b0f7u32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); cpu.w8(0xf6b8b0f7u32.wrapping_add(cpu.ebx), v); }
  pc = 0x480023; continue;
 }
 0x480020 => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x480022; continue;
 }
 0x480021 => {
  { let v: u32 = 0xb70fe1f6u32; cpu.eax = v; }
  pc = 0x480026; continue;
 }
 0x480022 => {
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x509290u32); cpu.ebp = v; }
  pc = 0x48002d; continue;
 }
 0x480023 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { pc = 0x480034; continue; }
  pc = 0x480025; continue;
 }
 0x480025 => {
  { let v: u8 = 0xe8u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp.wrapping_add(0x509290u32); cpu.ebp = v; }
  pc = 0x48002d; continue;
 }
 0x480026 => {
  cpu.push(0x48002b); cpu.call(0x92d8c5ac); if cpu.returned_to != 0x48002b { return; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x48002c; continue;
 }
 0x48002c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5du32.wrapping_add(cpu.ebp), v); }
  pc = 0x480031; continue;
 }
 0x48002d => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5du32.wrapping_add(cpu.ebp), v); }
  pc = 0x480031; continue;
 }
 0x48002f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x480030; continue;
 }
 0x480030 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x480031; continue;
 }
 0x480031 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { pc = 0x480085; continue; }
  pc = 0x480037; continue;
 }
 0x480032 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x480046; continue; }
  pc = 0x480034; continue;
 }
 0x480034 => {
  { let v: u8 = cpu.r8(0x4eu32.wrapping_add(cpu.ebp)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x4eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x480037; continue;
 }
 0x480037 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  pc = 0x480041; continue;
 }
 0x48003c => {
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x47fe134fu32.wrapping_add(cpu.eax))); cpu.w32(0x47fe134fu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(((cpu.eax >> 8) as u8), cpu.r8(0xffffff89u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x480046; continue;
 }
 0x480041 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x12u32.wrapping_add(cpu.edi))); cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x480048; continue;
 }
 0x480046 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x47881147u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0xff3b47c6u32); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0xf4u8); cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48006a); f_0041b794(cpu); if cpu.returned_to != 0x48006a { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x43u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6c30u32); cpu.edx = v; }
  pc = 0x480080; continue;
 }
 0x480048 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x3bu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0xf4u8); cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48006a); f_0041b794(cpu); if cpu.returned_to != 0x48006a { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x43u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6c30u32); cpu.edx = v; }
  pc = 0x480080; continue;
 }
 0x48004c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.eax, 0xff3b47c6u32); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0xf4u8); cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48006a); f_0041b794(cpu); if cpu.returned_to != 0x48006a { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x43u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6c30u32); cpu.edx = v; }
  pc = 0x480080; continue;
 }
 0x48005e => {
  { let v: u8 = cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0xf4u8); cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48006a); f_0041b794(cpu); if cpu.returned_to != 0x48006a { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x43u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6c30u32); cpu.edx = v; }
  pc = 0x480080; continue;
 }
 0x480069 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x480073; continue;
 }
 0x480073 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffff8bu32.wrapping_add(cpu.edx)), (cpu.edx as u8)); cpu.w8(0xffffff8bu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x4d6c30u32); cpu.eax = v; }
  pc = 0x480080; continue;
 }
 0x480080 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x480085 => {
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x15u8); }
  if (cpu.sf != cpu.of) { pc = 0x48008f; continue; }
  pc = 0x48008c; continue;
 }
 0x48008c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x48008d; continue;
 }
 0x48008d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48008f => {
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x13u32); cpu.edi = v; }
  { let v: u32 = 0x16u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  if cpu.ecx & 0xffff == 0 { cpu.poll(); pc = 0x48008c; continue; }
  pc = 0x4800a0; continue;
 }
 0x480090 => {
  { let v: u32 = cpu.rol32(cpu.r32(0x16b913c7u32.wrapping_add(cpu.ebx)), 1u32); cpu.w32(0x16b913c7u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x67aef2ffu32.wrapping_add(cpu.eax)), ((cpu.edx >> 8) as u8)); cpu.w8(0x67aef2ffu32.wrapping_add(cpu.eax), v); }
  pc = 0x48009e; continue;
 }
 0x48009e => {
  if cpu.ecx == 0 { cpu.poll(); pc = 0x48008c; continue; }
  pc = 0x4800a0; continue;
 }
 0x4800a0 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x12u32.wrapping_add(cpu.esi))); cpu.w8(0x12u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x14u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.ecx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4800ac; continue;
 }
 0x4800ac => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004800b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4800b3); }
 let mut pc: u32 = 0x4800b3;
 loop { match pc {
 0x4800b3 => {
  { let v: u32 = 0x52bbf0u32; cpu.edi = v; }
  { let v: u32 = 0x10e0u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x52ff70u32; cpu.edi = v; }
  { let v: u32 = 0x4ce0u32; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u8 = 0xf0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x52bc03u32; cpu.edi = v; }
  pc = 0x4800d8; continue;
 }
 0x4800d8 => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.add32(cpu.edi, 0x33u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4800d8; continue; }
  pc = 0x4800e6; continue;
 }
 0x4800e6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004800e7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4800e7); } r_004800e7(cpu, 0x4800e7); }
pub fn f_0048010a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48010a); } r_004800e7(cpu, 0x48010a); }
pub fn f_00480113(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480113); } r_004800e7(cpu, 0x480113); }
pub fn f_00480116(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480116); } r_004800e7(cpu, 0x480116); }
pub fn f_0048011f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48011f); } r_004800e7(cpu, 0x48011f); }
pub fn f_00480120(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x480120); } r_004800e7(cpu, 0x480120); }
pub fn f_0048012a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x48012a); } r_004800e7(cpu, 0x48012a); }
fn r_004800e7(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4800e7 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4b94ccu32, v); }
  { let v: u32 = 0x52bbf0u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x4800fa; continue;
 }
 0x4800fa => {
  { cpu.sub8(cpu.r8(0x42u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x480103; continue; }
  pc = 0x480100; continue;
 }
 0x480100 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x42u32.wrapping_add(cpu.esi))); cpu.w8(0x42u32.wrapping_add(cpu.esi), v); }
  pc = 0x480103; continue;
 }
 0x480103 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3du32.wrapping_add(cpu.esi))); cpu.w8(0x3du32.wrapping_add(cpu.esi), v); }
  if !cpu.sf { pc = 0x48010b; continue; }
  pc = 0x480108; continue;
 }
 0x480108 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x3du32.wrapping_add(cpu.esi))); cpu.w8(0x3du32.wrapping_add(cpu.esi), v); }
  pc = 0x48010b; continue;
 }
 0x48010a => {
  { cpu.sub32(cpu.eax, 0x400946f6u32); }
  if cpu.zf { pc = 0x480119; continue; }
  pc = 0x480111; continue;
 }
 0x48010b => {
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x480119; continue; }
  pc = 0x480111; continue;
 }
 0x480111 => {
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x480126; continue; }
  pc = 0x480117; continue;
 }
 0x480113 => {
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x480126; continue; }
  pc = 0x480117; continue;
 }
 0x480116 => {
  cpu.unimplemented(0x480116, "Por");
  pc = 0x480119; continue;
 }
 0x480117 => {
  pc = 0x48012b; continue;
 }
 0x480119 => {
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x480134; continue; }
  pc = 0x48011f; continue;
 }
 0x48011f => {
  { let v: u16 = cpu.inc16(cpu.r16(0x4b94ccu32)); cpu.w16(0x4b94ccu32, v); }
  pc = 0x480126; continue;
 }
 0x480120 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4b94ccu32)); cpu.w32(0x4b94ccu32, v); }
  pc = 0x480126; continue;
 }
 0x480126 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10u32.wrapping_add(cpu.esi))); cpu.w8(0x10u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x48013c; continue; }
  pc = 0x48012b; continue;
 }
 0x48012a => {
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)) & 0xbfu8; cpu.w8(0x9u32.wrapping_add(cpu.esi), v); }
  pc = 0x480134; continue;
 }
 0x48012b => {
  cpu.push(0x480130); f_00480142(cpu); if cpu.returned_to != 0x480130 { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)) & 0xbfu8; cpu.w8(0x9u32.wrapping_add(cpu.esi), v); }
  pc = 0x480134; continue;
 }
 0x480134 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x40u32.wrapping_add(cpu.esi), v); }
  pc = 0x48013c; continue;
 }
 0x48013c => {
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4800fa; continue; }
  pc = 0x480141; continue;
 }
 0x480141 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480142(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480142); }
 let mut pc: u32 = 0x480142;
 loop { match pc {
 0x480142 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  pc = 0x480147; continue;
 }
 0x480147 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x480154; continue;
 }
 0x480154 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x48016b; continue; }
  pc = 0x480159; continue;
 }
 0x480159 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x480161; continue; }
  pc = 0x48015e; continue;
 }
 0x48015e => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.ebx = v; }
  pc = 0x480161; continue;
 }
 0x480161 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x480154; continue; }
  pc = 0x480163; continue;
 }
 0x480163 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x480147; continue; }
  pc = 0x480168; continue;
 }
 0x480168 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48016b => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { cpu.poll(); pc = 0x480161; continue; }
  pc = 0x480170; continue;
 }
 0x480170 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0xffu8; cpu.w8(0xffffffffu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x509290u32); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5cu32.wrapping_add(cpu.ebp)).wrapping_sub((cpu.eax as u8)); cpu.w8(0x5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x480161; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480193(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480193); }
 let mut pc: u32 = 0x480193;
 loop { match pc {
 0x480193 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)) | 0x1u8; cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)) & 0xefu8; cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4801a6; continue;
 }
 0x4801a6 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4801c1; continue; }
  pc = 0x4801ab; continue;
 }
 0x4801ab => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4801bf); f_0043fced(cpu); if cpu.returned_to != 0x4801bf { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x4801c1; continue;
 }
 0x4801c1 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4801a6; continue; }
  pc = 0x4801c6; continue;
 }
 0x4801c6 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004801d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4801d0); }
 let mut pc: u32 = 0x4801d0;
 loop { match pc {
 0x4801d0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x513fc8u32; cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  pc = 0x4801e0; continue;
 }
 0x4801e0 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x4801ef; continue; }
  pc = 0x4801e8; continue;
 }
 0x4801e8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4801e0; continue; }
  pc = 0x4801ec; continue;
 }
 0x4801ec => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4801ef => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004801f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4801f2); }
 let mut pc: u32 = 0x4801f2;
 loop { match pc {
 0x4801f2 => {
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = 0x513fc8u32; cpu.edi = v; }
  pc = 0x480200; continue;
 }
 0x480200 => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2au32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x480200; continue; }
  pc = 0x480207; continue;
 }
 0x480207 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480208(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480208); }
 let mut pc: u32 = 0x480208;
 loop { match pc {
 0x480208 => {
  { let v: u32 = 0x513fc8u32; cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x480212; continue;
 }
 0x480212 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x480223; continue; }
  pc = 0x48021a; continue;
 }
 0x48021a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x480221); f_00480229(cpu); if cpu.returned_to != 0x480221 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x480223; continue;
 }
 0x480223 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x2cu32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x480212; continue; }
  pc = 0x480228; continue;
 }
 0x480228 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480229(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480229); }
 let mut pc: u32 = 0x480229;
 loop { match pc {
 0x480229 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x28u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x48024a); f_00480657(cpu); if cpu.returned_to != 0x48024a { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685au32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d693cu32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d685cu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d693eu32, v); }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x5d5ef0u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.push(0x480279); cpu.call(t); if cpu.returned_to != 0x480279 { return; }
  if !cpu.zf { pc = 0x480376; continue; }
  pc = 0x48027f; continue;
 }
 0x48027f => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f86u32, v); }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x4802dd; continue; }
  pc = 0x48029a; continue;
 }
 0x48029a => {
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0xffu8; cpu.w8(0x5cc622u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc64cu32, v); }
  cpu.push(0x4802b7); f_00467adc(cpu); if cpu.returned_to != 0x4802b7 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc64cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc622u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x32u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f86u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4802dd => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x18u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x2au32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x480305; continue; }
  pc = 0x4802f0; continue;
 }
 0x4802f0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x4802f8); f_00480532(cpu); if cpu.returned_to != 0x4802f8 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x480300); f_004805c4(cpu); if cpu.returned_to != 0x480300 { return; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x32u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x480367; continue;
 }
 0x480305 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0xffu8; cpu.w8(0x5cc622u32, v); }
  cpu.push(0x480314); f_00467adc(cpu); if cpu.returned_to != 0x480314 { return; }
  cpu.push(0x480319); f_004805c4(cpu); if cpu.returned_to != 0x480319 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc622u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x32u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x480367; continue; }
  pc = 0x480328; continue;
 }
 0x480328 => {
  { let v: u32 = cpu.r32(0x5cc634u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x9u8); }
  if cpu.zf { pc = 0x480367; continue; }
  pc = 0x480338; continue;
 }
 0x480338 => {
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x2au8); }
  if cpu.zf { pc = 0x480367; continue; }
  pc = 0x48033d; continue;
 }
 0x48033d => {
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x26u8); }
  if cpu.zf { pc = 0x480367; continue; }
  pc = 0x480342; continue;
 }
 0x480342 => {
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x28u8); }
  if cpu.zf { pc = 0x480367; continue; }
  pc = 0x480347; continue;
 }
 0x480347 => {
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x27u8); }
  if cpu.zf { pc = 0x480367; continue; }
  pc = 0x48034c; continue;
 }
 0x48034c => {
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x1du8); }
  if cpu.zf { pc = 0x480367; continue; }
  pc = 0x480351; continue;
 }
 0x480351 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f86u32, v); }
  cpu.push(0x48035d); f_00480e4e(cpu); if cpu.returned_to != 0x48035d { return; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x32u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x480367; continue; }
  pc = 0x480362; continue;
 }
 0x480362 => {
  cpu.push(0x480367); f_00480e4e(cpu); if cpu.returned_to != 0x480367 { return; }
  pc = 0x480367; continue;
 }
 0x480367 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f86u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x480376 => {
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.mul16(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x1au32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x4803c3; continue; }
  pc = 0x4803b4; continue;
 }
 0x4803b4 => {
  if cpu.cf { cpu.poll(); pc = 0x48027f; continue; }
  pc = 0x4803ba; continue;
 }
 0x4803ba => {
  { cpu.sub16((cpu.ebx as u16), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x48027f; continue; }
  pc = 0x4803c3; continue;
 }
 0x4803c3 => {
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x4804ff; continue; }
  pc = 0x4803ce; continue;
 }
 0x4803ce => {
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x480510; continue; }
  pc = 0x4803d9; continue;
 }
 0x4803d9 => {
  { cpu.sub8(cpu.r8(0x2au32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x480521; continue; }
  pc = 0x4803e4; continue;
 }
 0x4803e4 => {
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if (cpu.sf == cpu.of) { pc = 0x4803fa; continue; }
  pc = 0x4803ec; continue;
 }
 0x4803ec => {
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4803fa; continue;
 }
 0x4803fa => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.lods(1, false); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x10u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x480422); f_00420989(cpu); if cpu.returned_to != 0x480422 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x4e9e4bu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x2au32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x48043b; continue; }
  pc = 0x480436; continue;
 }
 0x480436 => {
  { let v: u32 = 0x4e84a7u32; cpu.ebx = v; }
  pc = 0x48043b; continue;
 }
 0x48043b => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if (cpu.cf || cpu.zf) { pc = 0x48045d; continue; }
  pc = 0x480440; continue;
 }
 0x480440 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x48027f; continue;
 }
 0x48045d => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x4d677eu32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x480478); cpu.call(t); if cpu.returned_to != 0x480478 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.push(0x48047e); f_00480740(cpu); if cpu.returned_to != 0x48047e { return; }
  { let v: u32 = 0x66du32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x4804a7; continue; }
  pc = 0x48048a; continue;
 }
 0x48048a => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6bd0u32, v); }
  { let v: u16 = cpu.shl16(cpu.r16(0x4ac410u32), 1u32); cpu.w16(0x4ac410u32, v); }
  { let v: u32 = 0x543u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4d677eu32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x4804a7; continue;
 }
 0x4804a7 => {
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x4804b5; continue; }
  pc = 0x4804ae; continue;
 }
 0x4804ae => {
  { let v: u16 = cpu.shl16(cpu.r16(0x4ac410u32), 1u32); cpu.w16(0x4ac410u32, v); }
  pc = 0x4804b5; continue;
 }
 0x4804b5 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x4d6f44u32); cpu.push(0x4804e3); cpu.call(t); if cpu.returned_to != 0x4804e3 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x4804f9; continue; }
  pc = 0x4804ed; continue;
 }
 0x4804ed => {
  { let v: u8 = cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0xfbu8); cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4804f8); f_004813fe(cpu); if cpu.returned_to != 0x4804f8 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4804f9; continue;
 }
 0x4804f9 => {
  cpu.push(0x4804fe); f_004807f0(cpu); if cpu.returned_to != 0x4804fe { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4804ff => {
  cpu.push(0x480504); f_004809b6(cpu); if cpu.returned_to != 0x480504 { return; }
  if cpu.cf { cpu.poll(); pc = 0x48027f; continue; }
  pc = 0x48050a; continue;
 }
 0x48050a => {
  cpu.push(0x48050f); f_004807f0(cpu); if cpu.returned_to != 0x48050f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x480510 => {
  cpu.push(0x480515); f_00480af9(cpu); if cpu.returned_to != 0x480515 { return; }
  if cpu.cf { cpu.poll(); pc = 0x48027f; continue; }
  pc = 0x48051b; continue;
 }
 0x48051b => {
  cpu.push(0x480520); f_004807f0(cpu); if cpu.returned_to != 0x480520 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x480521 => {
  cpu.push(0x480526); f_00480bcf(cpu); if cpu.returned_to != 0x480526 { return; }
  if cpu.cf { cpu.poll(); pc = 0x48027f; continue; }
  pc = 0x48052c; continue;
 }
 0x48052c => {
  cpu.push(0x480531); f_004807f0(cpu); if cpu.returned_to != 0x480531 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480532(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480532); }
 let mut pc: u32 = 0x480532;
 loop { match pc {
 0x480532 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x9u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x15u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d693au32, v); }
  cpu.push(0x480554); f_00420989(cpu); if cpu.returned_to != 0x480554 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e84a7u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d693au32, v); }
  { let v: u16 = 0x1u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x48056e; continue;
 }
 0x48056e => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x480574); f_0041c743(cpu); if cpu.returned_to != 0x480574 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x48057e; continue; }
  pc = 0x480579; continue;
 }
 0x480579 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x48057e; continue;
 }
 0x48057e => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x46u32); cpu.ecx = v; }
  cpu.push(0x480588); f_0041c743(cpu); if cpu.returned_to != 0x480588 { return; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x48059b); f_0041c743(cpu); if cpu.returned_to != 0x48059b { return; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6934u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x4d6936u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d693au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x4805bf); f_004805ee(cpu); if cpu.returned_to != 0x4805bf { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x48056e; continue; }
  pc = 0x4805c2; continue;
 }
 0x4805c2 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004805c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4805c4); }
 let mut pc: u32 = 0x4805c4;
 loop { match pc {
 0x4805c4 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4805d6); f_004712e3(cpu); if cpu.returned_to != 0x4805d6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if cpu.cf { pc = 0x4805dc; continue; }
  pc = 0x4805da; continue;
 }
 0x4805da => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4805dc => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x15u32.wrapping_add(cpu.eax), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004805ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4805ee); }
 let mut pc: u32 = 0x4805ee;
 loop { match pc {
 0x4805ee => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4805f9); f_00448783(cpu); if cpu.returned_to != 0x4805f9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x480655; continue; }
  pc = 0x480600; continue;
 }
 0x480600 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4b839cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 7u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.edx as u16), 4u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x26u32.wrapping_add(cpu.edi), v); }
  pc = 0x480655; continue;
 }
 0x480655 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480657(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480657); }
 let mut pc: u32 = 0x480657;
 loop { match pc {
 0x480657 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x480663); f_004400bf(cpu); if cpu.returned_to != 0x480663 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x55755du32; cpu.esi = v; }
  pc = 0x480669; continue;
 }
 0x480669 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4806da; continue; }
  pc = 0x48066e; continue;
 }
 0x48066e => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x4806d7; continue; }
  pc = 0x480681; continue;
 }
 0x480681 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { pc = 0x4806d7; continue; }
  pc = 0x480687; continue;
 }
 0x480687 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x480693; continue; }
  pc = 0x48068d; continue;
 }
 0x48068d => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { pc = 0x4806d7; continue; }
  pc = 0x480693; continue;
 }
 0x480693 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x40u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x4806d7; continue; }
  pc = 0x4806c0; continue;
 }
 0x4806c0 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x2bu8); }
  if !cpu.zf { pc = 0x4806cc; continue; }
  pc = 0x4806c6; continue;
 }
 0x4806c6 => {
  { let v: u8 = 0x2u8; cpu.w8(0x7du32.wrapping_add(cpu.edi), v); }
  pc = 0x4806d7; continue;
 }
 0x4806cc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  cpu.push(0x4806d5); f_0045ae1d(cpu); if cpu.returned_to != 0x4806d5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x4806d7; continue;
 }
 0x4806d7 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x480669; continue;
 }
 0x4806da => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480740); }
 let mut pc: u32 = 0x480740;
 loop { match pc {
 0x480740 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x1au32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d685au32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d685cu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  if (cpu.cf || cpu.zf) { pc = 0x4807b7; continue; }
  pc = 0x4807b5; continue;
 }
 0x4807b5 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  pc = 0x4807b7; continue;
 }
 0x4807b7 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x4807bf; continue; }
  pc = 0x4807bd; continue;
 }
 0x4807bd => {
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  pc = 0x4807bf; continue;
 }
 0x4807bf => {
  { let v: u32 = cpu.shr32(cpu.ebx, 8u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x400u32); }
  if cpu.cf { pc = 0x4807ce; continue; }
  pc = 0x4807ca; continue;
 }
 0x4807ca => {
  { let v: u8 = 0x2eu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4807d6; continue;
 }
 0x4807ce => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6accu32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4807d6; continue;
 }
 0x4807d6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4ac410u32, v); }
  { let v: u32 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd0u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004807f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4807f0); }
 let mut pc: u32 = 0x4807f0;
 loop { match pc {
 0x4807f0 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x22u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0x24u32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 7u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x48080b); f_004400bf(cpu); if cpu.returned_to != 0x48080b { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6b64u32, v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u8 = cpu.r8(0x5092c1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5d5f1eu32, v); }
  { let v: u32 = 0x55755du32; cpu.esi = v; }
  pc = 0x480830; continue;
 }
 0x480830 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x4809b4; continue; }
  pc = 0x48083b; continue;
 }
 0x48083b => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5d5f1eu32)); }
  if cpu.zf { cpu.poll(); pc = 0x480830; continue; }
  pc = 0x480857; continue;
 }
 0x480857 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x480830; continue; }
  pc = 0x48085d; continue;
 }
 0x48085d => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.edi)), 0x40u8); }
  if !cpu.zf { cpu.poll(); pc = 0x480830; continue; }
  pc = 0x480863; continue;
 }
 0x480863 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x40u8); }
  if !cpu.zf { cpu.poll(); pc = 0x480830; continue; }
  pc = 0x480869; continue;
 }
 0x480869 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x480830; continue; }
  pc = 0x48086f; continue;
 }
 0x48086f => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x7u8); }
  if cpu.zf { cpu.poll(); pc = 0x480830; continue; }
  pc = 0x480875; continue;
 }
 0x480875 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x12u8); }
  if cpu.zf { cpu.poll(); pc = 0x480830; continue; }
  pc = 0x48087b; continue;
 }
 0x48087b => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x22u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x20u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x480830; continue; }
  pc = 0x480896; continue;
 }
 0x480896 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x24u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x20u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x480830; continue; }
  pc = 0x4808b5; continue;
 }
 0x4808b5 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x4808f4; continue; }
  pc = 0x4808ea; continue;
 }
 0x4808ea => {
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x4808f1; continue; }
  pc = 0x4808ef; continue;
 }
 0x4808ef => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4808f1; continue;
 }
 0x4808f1 => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4808f4; continue;
 }
 0x4808f4 => {
  { let v = (cpu.eax as u8); cpu.mul8(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x480830; continue; }
  pc = 0x480901; continue;
 }
 0x480901 => {
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x480830; continue; }
  pc = 0x480909; continue;
 }
 0x480909 => {
  cpu.push(0x48090e); f_0041c743(cpu); if cpu.returned_to != 0x48090e { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x48091b); f_0041c743(cpu); if cpu.returned_to != 0x48091b { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x8u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x22u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48094a; continue; }
  pc = 0x48093d; continue;
 }
 0x48093d => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x28u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x480955; continue;
 }
 0x48094a => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x28u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x480955; continue;
 }
 0x480955 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x24u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x48096c; continue; }
  pc = 0x48095f; continue;
 }
 0x48095f => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x480977; continue;
 }
 0x48096c => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x480977; continue;
 }
 0x480977 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x480985); f_004895c7(cpu); if cpu.returned_to != 0x480985 { return; }
  cpu.push(0x48098a); f_00489637(cpu); if cpu.returned_to != 0x48098a { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x4809ad; continue; }
  pc = 0x480991; continue;
 }
 0x480991 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x0u32); }
  if cpu.zf { pc = 0x48099f; continue; }
  pc = 0x48099a; continue;
 }
 0x48099a => {
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x4809a4; continue; }
  pc = 0x48099f; continue;
 }
 0x48099f => {
  { let v: u32 = 0x5b355cu32; cpu.esi = v; }
  pc = 0x4809a4; continue;
 }
 0x4809a4 => {
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x4809ad; continue; }
  pc = 0x4809a9; continue;
 }
 0x4809a9 => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.esi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  pc = 0x4809ad; continue;
 }
 0x4809ad => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.poll(); pc = 0x480830; continue;
 }
 0x4809b4 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004809b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4809b6); }
 let mut pc: u32 = 0x4809b6;
 loop { match pc {
 0x4809b6 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)) & 0x3u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x17u32.wrapping_add(cpu.ebp)), 0xefu8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x4809cb; continue; }
  pc = 0x4809c6; continue;
 }
 0x4809c6 => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)).wrapping_add(0x10u8); cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4809cb; continue;
 }
 0x4809cb => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x480a0c; continue; }
  pc = 0x4809d2; continue;
 }
 0x4809d2 => {
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x4809fe; continue; }
  pc = 0x4809d9; continue;
 }
 0x4809d9 => {
  { let v: u16 = 0x2au16; cpu.w16(0x4d6934u32, v); }
  { cpu.sub16(cpu.r16(0x26u32.wrapping_add(cpu.ebp)), 0x5u16); }
  if (cpu.sf == cpu.of) { pc = 0x480a0c; continue; }
  pc = 0x4809eb; continue;
 }
 0x4809eb => {
  { let v: u16 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6936u32, v); }
  cpu.push(0x4809fc); f_00480ca5(cpu); if cpu.returned_to != 0x4809fc { return; }
  pc = 0x480a0c; continue;
 }
 0x4809fe => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  cpu.push(0x480a0c); f_00480ca5(cpu); if cpu.returned_to != 0x480a0c { return; }
  pc = 0x480a0c; continue;
 }
 0x480a0c => {
  { let v: u16 = cpu.inc16(cpu.r16(0x26u32.wrapping_add(cpu.ebp))); cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x480a23); f_00420989(cpu); if cpu.returned_to != 0x480a23 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e9e4bu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x480a51; continue; }
  pc = 0x480a38; continue;
 }
 0x480a38 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x480a51 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x480a88; continue; }
  pc = 0x480a81; continue;
 }
 0x480a81 => {
  { let v: u32 = 0x14f4u32; cpu.ecx = v; }
  pc = 0x480a9c; continue;
 }
 0x480a88 => {
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x480a96; continue; }
  pc = 0x480a8f; continue;
 }
 0x480a8f => {
  { let v: u32 = 0x1544u32; cpu.ecx = v; }
  pc = 0x480a9c; continue;
 }
 0x480a96 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x661u32); cpu.ecx = v; }
  pc = 0x480a9c; continue;
 }
 0x480a9c => {
  { let v: u16 = 0x1u16; cpu.w16(0x4ac410u32, v); }
  { let v: u32 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x480ac2); cpu.call(t); if cpu.returned_to != 0x480ac2 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd0u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x480ad1); f_004418e0(cpu); if cpu.returned_to != 0x480ad1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x480af7; continue; }
  pc = 0x480adb; continue;
 }
 0x480adb => {
  { let v: u8 = cpu.r8(0x21u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d693cu32).wrapping_add(0x6u16); cpu.w16(0x4d693cu32, v); }
  { let v: u16 = cpu.add16(cpu.r16(0x4d693eu32), 0x4u16); cpu.w16(0x4d693eu32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x480af6); f_004813fe(cpu); if cpu.returned_to != 0x480af6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x480af7; continue;
 }
 0x480af7 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480af9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480af9); }
 let mut pc: u32 = 0x480af9;
 loop { match pc {
 0x480af9 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x480b09); f_004679cb(cpu); if cpu.returned_to != 0x480b09 { return; }
  if cpu.cf { pc = 0x480b1e; continue; }
  pc = 0x480b0b; continue;
 }
 0x480b0b => {
  { let v: u8 = 0x4u8; cpu.w8(0x59dadau32, v); }
  { let v: u16 = cpu.r16(0x5cc64eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x59dad8u32, v); }
  pc = 0x480b1e; continue;
 }
 0x480b1e => {
  { cpu.popad(); }
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)) & 0x3u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x17u32.wrapping_add(cpu.ebp)), 0xefu8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x480b34; continue; }
  pc = 0x480b2f; continue;
 }
 0x480b2f => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)).wrapping_add(0x10u8); cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  pc = 0x480b34; continue;
 }
 0x480b34 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x26u32.wrapping_add(cpu.ebp))); cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x480b4b); f_00420989(cpu); if cpu.returned_to != 0x480b4b { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e9e4bu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x480b79; continue; }
  pc = 0x480b60; continue;
 }
 0x480b60 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x480b79 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x661u32); cpu.ecx = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x4ac410u32, v); }
  { let v: u32 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd0u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x480bcb); f_004418e0(cpu); if cpu.returned_to != 0x480bcb { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480bcf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480bcf); }
 let mut pc: u32 = 0x480bcf;
 loop { match pc {
 0x480bcf => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x480bdf); f_004679cb(cpu); if cpu.returned_to != 0x480bdf { return; }
  if cpu.cf { pc = 0x480bf4; continue; }
  pc = 0x480be1; continue;
 }
 0x480be1 => {
  { let v: u8 = 0x5u8; cpu.w8(0x59dadau32, v); }
  { let v: u16 = cpu.r16(0x5cc64eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x59dad8u32, v); }
  pc = 0x480bf4; continue;
 }
 0x480bf4 => {
  { cpu.popad(); }
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)) & 0x3u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x17u32.wrapping_add(cpu.ebp)), 0xefu8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x480c0a; continue; }
  pc = 0x480c05; continue;
 }
 0x480c05 => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)).wrapping_add(0x10u8); cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  pc = 0x480c0a; continue;
 }
 0x480c0a => {
  { let v: u16 = cpu.inc16(cpu.r16(0x26u32.wrapping_add(cpu.ebp))); cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x480c21); f_00420989(cpu); if cpu.returned_to != 0x480c21 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e9e4bu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x480c4f; continue; }
  pc = 0x480c36; continue;
 }
 0x480c36 => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x480c4f => {
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x17f1u32); cpu.ecx = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x4ac410u32, v); }
  { let v: u32 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd0u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x480ca1); f_004418e0(cpu); if cpu.returned_to != 0x480ca1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480ca5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480ca5); }
 let mut pc: u32 = 0x480ca5;
 loop { match pc {
 0x480ca5 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x4d685au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d685cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x2u32; cpu.edi = v; }
  pc = 0x480cb8; continue;
 }
 0x480cb8 => {
  cpu.push(0x480cbd); f_00480d54(cpu); if cpu.returned_to != 0x480cbd { return; }
  if cpu.cf { pc = 0x480d52; continue; }
  pc = 0x480cc3; continue;
 }
 0x480cc3 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.cf { pc = 0x480cdc; continue; }
  pc = 0x480cd4; continue;
 }
 0x480cd4 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x3u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x480cdc; continue;
 }
 0x480cdc => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2au16); }
  if !cpu.zf { pc = 0x480d0c; continue; }
  pc = 0x480d08; continue;
 }
 0x480d08 => {
  { let v: u8 = 0x1u8; cpu.w8(0x7u32.wrapping_add(cpu.esi), v); }
  pc = 0x480d0c; continue;
 }
 0x480d0c => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x480d52; continue; }
  pc = 0x480d11; continue;
 }
 0x480d11 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(cpu.r16(0x4d685au32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x4d685cu32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x480d40; continue; }
  pc = 0x480d2f; continue;
 }
 0x480d2f => {
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x480cb8; continue;
 }
 0x480d40 => {
  { cpu.sub16((cpu.ecx as u16), 0xfff8u16); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x480d2f; continue; }
  pc = 0x480d46; continue;
 }
 0x480d46 => {
  { cpu.sub16((cpu.edx as u16), 0x8u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x480d2f; continue; }
  pc = 0x480d4c; continue;
 }
 0x480d4c => {
  { cpu.sub16((cpu.edx as u16), 0xfff8u16); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x480d2f; continue; }
  pc = 0x480d52; continue;
 }
 0x480d52 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480d54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480d54); }
 let mut pc: u32 = 0x480d54;
 loop { match pc {
 0x480d54 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x514860u32; cpu.esi = v; }
  { let v: u32 = 0x190u32; cpu.ecx = v; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  pc = 0x480d64; continue;
 }
 0x480d64 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x480d71; continue; }
  pc = 0x480d6a; continue;
 }
 0x480d6a => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x480d64; continue; }
  pc = 0x480d6e; continue;
 }
 0x480d6e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x480d71 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480d74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480d74); }
 let mut pc: u32 = 0x480d74;
 loop { match pc {
 0x480d74 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = 0x190u32; cpu.ecx = v; }
  { let v: u32 = 0x514860u32; cpu.edi = v; }
  pc = 0x480d83; continue;
 }
 0x480d83 => {
  { cpu.stos(4, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0x6u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x480d83; continue; }
  pc = 0x480d89; continue;
 }
 0x480d89 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480d8a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480d8a); }
 let mut pc: u32 = 0x480d8a;
 loop { match pc {
 0x480d8a => {
  { let v: u32 = 0x514860u32; cpu.esi = v; }
  { let v: u32 = 0x190u32; cpu.ecx = v; }
  pc = 0x480d94; continue;
 }
 0x480d94 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x480e43; continue; }
  pc = 0x480d9f; continue;
 }
 0x480d9f => {
  { cpu.sub8(cpu.r8(0x6u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x480dad; continue; }
  pc = 0x480da5; continue;
 }
 0x480da5 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x6u32.wrapping_add(cpu.esi))); cpu.w8(0x6u32.wrapping_add(cpu.esi), v); }
  pc = 0x480e43; continue;
 }
 0x480dad => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x480e29; continue; }
  pc = 0x480dc8; continue;
 }
 0x480dc8 => {
  { cpu.sub16((cpu.ebx as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x480e29; continue; }
  pc = 0x480dce; continue;
 }
 0x480dce => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x480e29; continue; }
  pc = 0x480dd7; continue;
 }
 0x480dd7 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x480e29; continue; }
  pc = 0x480de0; continue;
 }
 0x480de0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x480e07; continue; }
  pc = 0x480deb; continue;
 }
 0x480deb => {
  { let v: u32 = cpu.ecx.wrapping_add(0x665u32); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd0u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x480e17; continue;
 }
 0x480e07 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x665u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6bd0u32, v); }
  pc = 0x480e17; continue;
 }
 0x480e17 => {
  { cpu.pushad(); }
  { let v: u16 = 0x1u16; cpu.w16(0x4ac410u32, v); }
  let t = cpu.r32(0x4d6f44u32); cpu.push(0x480e27); cpu.call(t); if cpu.returned_to != 0x480e27 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x480e29; continue;
 }
 0x480e29 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5u32.wrapping_add(cpu.esi))); cpu.w8(0x5u32.wrapping_add(cpu.esi), v); }
  if !cpu.sf { pc = 0x480e43; continue; }
  pc = 0x480e2e; continue;
 }
 0x480e2e => {
  { let v: u8 = 0x2u8; cpu.w8(0x5u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x4u32.wrapping_add(cpu.esi))); cpu.w8(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x4u32.wrapping_add(cpu.esi))); cpu.w8(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esi)), 0x7u8); }
  if (cpu.sf != cpu.of) { pc = 0x480e43; continue; }
  pc = 0x480e3e; continue;
 }
 0x480e3e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x480e43; continue;
 }
 0x480e43 => {
  { let v: u32 = cpu.add32(cpu.esi, 0xau32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x480d94; continue; }
  pc = 0x480e4d; continue;
 }
 0x480e4d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480e4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480e4e); }
 let mut pc: u32 = 0x480e4e;
 loop { match pc {
 0x480e4e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x480e59); f_0041c743(cpu); if cpu.returned_to != 0x480e59 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x480e69); f_0041c743(cpu); if cpu.returned_to != 0x480e69 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x480e7e); f_00467adc(cpu); if cpu.returned_to != 0x480e7e { return; }
  cpu.push(0x480e83); f_004805c4(cpu); if cpu.returned_to != 0x480e83 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00480e87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x480e87); }
 let mut pc: u32 = 0x480e87;
 loop { match pc {
 0x480e87 => {
  cpu.push(0x480e8c); f_004824a8(cpu); if cpu.returned_to != 0x480e8c { return; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if !cpu.zf { pc = 0x480e98; continue; }
  pc = 0x480e91; continue;
 }
 0x480e91 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x480e98; continue; }
  pc = 0x480e96; continue;
 }
 0x480e96 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x480e98 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { cpu.poll(); pc = 0x480e96; continue; }
  pc = 0x480e9c; continue;
 }
 0x480e9c => {
  cpu.push(0x480ea1); f_004813d3(cpu); if cpu.returned_to != 0x480ea1 { return; }
  if cpu.cf { pc = 0x481012; continue; }
  pc = 0x480ea7; continue;
 }
 0x480ea7 => {
  cpu.push(0x480eac); f_00481184(cpu); if cpu.returned_to != 0x480eac { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6b64u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x480ebd); f_004679cb(cpu); if cpu.returned_to != 0x480ebd { return; }
  if cpu.cf { pc = 0x480f0a; continue; }
  pc = 0x480ebf; continue;
 }
 0x480ebf => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5d5f23u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x480f0a; continue; }
  pc = 0x480eca; continue;
 }
 0x480eca => {
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x480f0a; continue; }
  pc = 0x480ed3; continue;
 }
 0x480ed3 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x480eda); f_00401130(cpu); if cpu.returned_to != 0x480eda { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d5f3cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x5d5f3cu32); cpu.eax = v; }
  cpu.push(0x480eed); f_00415d61(cpu); if cpu.returned_to != 0x480eed { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x5d5f26u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if cpu.zf { pc = 0x480f0a; continue; }
  pc = 0x480efa; continue;
 }
 0x480efa => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x480f0a; continue; }
  pc = 0x480efe; continue;
 }
 0x480efe => {
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x480f06); f_004010d0(cpu); if cpu.returned_to != 0x480f06 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { cpu.popad(); }
  pc = 0x480f0a; continue;
 }
 0x480f0a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x480f16); f_00481441(cpu); if cpu.returned_to != 0x480f16 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x480e96; continue; }
  pc = 0x480f21; continue;
 }
 0x480f21 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x2au16); }
  if !cpu.zf { pc = 0x480f61; continue; }
  pc = 0x480f2b; continue;
 }
 0x480f2b => {
  { let v: u8 = 0x15u8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.eax & cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx & cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.edx & cpu.edi; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x480f51); f_0040d710(cpu); if cpu.returned_to != 0x480f51 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d5f3cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x5d5f3cu32); cpu.eax = v; }
  pc = 0x480fd5; continue;
 }
 0x480f61 => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x29u16); }
  if !cpu.zf { pc = 0x480fa1; continue; }
  pc = 0x480f6b; continue;
 }
 0x480f6b => {
  { let v: u8 = 0x25u8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.eax & cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx & cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.edx & cpu.edi; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x480f91); f_0040d710(cpu); if cpu.returned_to != 0x480f91 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d5f3cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x5d5f3cu32); cpu.eax = v; }
  pc = 0x480fd5; continue;
 }
 0x480fa1 => {
  { let v: u8 = 0x3u8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.eax & cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx & cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.edx & cpu.edi; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x480fc7); f_0040d710(cpu); if cpu.returned_to != 0x480fc7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d5f3cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x5d5f3cu32); cpu.eax = v; }
  pc = 0x480fd5; continue;
 }
 0x480fd5 => {
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x168u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x4e9e4bu32; cpu.ebx = v; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  pc = 0x481012; continue;
 }
 0x481012 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00481013(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x481013); }
 let mut pc: u32 = 0x481013;
 loop { match pc {
 0x481013 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if !cpu.zf { pc = 0x48101f; continue; }
  pc = 0x481018; continue;
 }
 0x481018 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x48101f; continue; }
  pc = 0x48101d; continue;
 }
 0x48101d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x48101f => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { cpu.poll(); pc = 0x48101d; continue; }
  pc = 0x481023; continue;
 }
 0x481023 => {
  cpu.push(0x481028); f_004813d3(cpu); if cpu.returned_to != 0x481028 { return; }
  if cpu.cf { pc = 0x4810ca; continue; }
  pc = 0x48102e; continue;
 }
 0x48102e => {
  cpu.push(0x481033); f_00481184(cpu); if cpu.returned_to != 0x481033 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4d6b64u32, v); }
  cpu.push(0x481042); f_004679cb(cpu); if cpu.returned_to != 0x481042 { return; }
  if cpu.cf { pc = 0x481044; continue; }
  pc = 0x481044; continue;
 }
 0x481044 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x48104e); f_00481441(cpu); if cpu.returned_to != 0x48104e { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x48101d; continue; }
  pc = 0x481055; continue;
 }
 0x481055 => {
  { let v: u8 = 0x42u8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.eax & cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx & cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.edx & cpu.edi; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x48107b); f_0040d710(cpu); if cpu.returned_to != 0x48107b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d5f3cu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x5d5f3cu32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x168u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x40u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x4e9e4bu32; cpu.ebx = v; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  pc = 0x4810ca; continue;
 }
 0x4810ca => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
