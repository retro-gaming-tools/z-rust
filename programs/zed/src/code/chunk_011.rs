#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_0106d795(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106d795); }
 let mut pc: u32 = 0x106d795;
 loop { match pc {
 0x106d795 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106d7a5); f_0104f36d(cpu); if cpu.returned_to != 0x106d7a5 { return; }
  cpu.push(0x106d7aa); f_0106d7d0(cpu); if cpu.returned_to != 0x106d7aa { return; }
  if cpu.cf { pc = 0x106d7cb; continue; }
  pc = 0x106d7ac; continue;
 }
 0x106d7ac => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x106d7b2); f_0106d7d0(cpu); if cpu.returned_to != 0x106d7b2 { return; }
  if cpu.cf { pc = 0x106d7cb; continue; }
  pc = 0x106d7b4; continue;
 }
 0x106d7b4 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x7fu32); cpu.esi = v; }
  cpu.push(0x106d7bc); f_0106d7d0(cpu); if cpu.returned_to != 0x106d7bc { return; }
  if cpu.cf { pc = 0x106d7cb; continue; }
  pc = 0x106d7be; continue;
 }
 0x106d7be => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x106d7c4); f_0106d7d0(cpu); if cpu.returned_to != 0x106d7c4 { return; }
  if cpu.cf { pc = 0x106d7cb; continue; }
  pc = 0x106d7c6; continue;
 }
 0x106d7c6 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106d7cb => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106d7d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106d7d0); }
 let mut pc: u32 = 0x106d7d0;
 loop { match pc {
 0x106d7d0 => {
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1eu8); }
  if cpu.zf { pc = 0x106d7ee; continue; }
  pc = 0x106d7dc; continue;
 }
 0x106d7dc => {
  { cpu.sub8((cpu.eax as u8), 0x14u8); }
  if cpu.zf { pc = 0x106d7ee; continue; }
  pc = 0x106d7e0; continue;
 }
 0x106d7e0 => {
  { cpu.sub8((cpu.eax as u8), 0x15u8); }
  if cpu.zf { pc = 0x106d7ee; continue; }
  pc = 0x106d7e4; continue;
 }
 0x106d7e4 => {
  { cpu.sub8((cpu.eax as u8), 0x3cu8); }
  if cpu.zf { pc = 0x106d7ee; continue; }
  pc = 0x106d7e8; continue;
 }
 0x106d7e8 => {
  { cpu.sub8((cpu.eax as u8), 0x3du8); }
  if cpu.zf { pc = 0x106d7ee; continue; }
  pc = 0x106d7ec; continue;
 }
 0x106d7ec => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106d7ee => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106d7f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106d7f0); }
 let mut pc: u32 = 0x106d7f0;
 loop { match pc {
 0x106d7f0 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106d806); f_0104f36d(cpu); if cpu.returned_to != 0x106d806 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x106d852; continue; }
  pc = 0x106d812; continue;
 }
 0x106d812 => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x106d852; continue; }
  pc = 0x106d816; continue;
 }
 0x106d816 => {
  { cpu.sub8((cpu.eax as u8), 0x16u8); }
  if cpu.zf { pc = 0x106d852; continue; }
  pc = 0x106d81a; continue;
 }
 0x106d81a => {
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if cpu.zf { pc = 0x106d852; continue; }
  pc = 0x106d81e; continue;
 }
 0x106d81e => {
  { cpu.sub8((cpu.eax as u8), 0x24u8); }
  if cpu.zf { pc = 0x106d852; continue; }
  pc = 0x106d822; continue;
 }
 0x106d822 => {
  { cpu.sub8((cpu.eax as u8), 0x2du8); }
  if cpu.zf { pc = 0x106d852; continue; }
  pc = 0x106d826; continue;
 }
 0x106d826 => {
  { cpu.sub8((cpu.eax as u8), 0x35u8); }
  if cpu.zf { pc = 0x106d852; continue; }
  pc = 0x106d82a; continue;
 }
 0x106d82a => {
  { cpu.sub8((cpu.eax as u8), 0x36u8); }
  if cpu.zf { pc = 0x106d852; continue; }
  pc = 0x106d82e; continue;
 }
 0x106d82e => {
  { cpu.sub8((cpu.eax as u8), 0x37u8); }
  if cpu.zf { pc = 0x106d852; continue; }
  pc = 0x106d832; continue;
 }
 0x106d832 => {
  pc = 0x106d854; continue;
 }
 0x106d852 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106d854 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106d85c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106d85c); }
 let mut pc: u32 = 0x106d85c;
 loop { match pc {
 0x106d85c => {
  { let v: u32 = 0x116b520u32; cpu.ebp = v; }
  { let v: u32 = 0x1f4u32; cpu.ecx = v; }
  pc = 0x106d866; continue;
 }
 0x106d866 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x106d874; continue; }
  pc = 0x106d86d; continue;
 }
 0x106d86d => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106d866; continue; }
  pc = 0x106d872; continue;
 }
 0x106d872 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106d874 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106d876(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106d876); }
 let mut pc: u32 = 0x106d876;
 loop { match pc {
 0x106d876 => {
  { let v: u32 = 0x116b520u32; cpu.ebp = v; }
  { let v: u32 = 0x1f4u32; cpu.ecx = v; }
  pc = 0x106d880; continue;
 }
 0x106d880 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if !cpu.zf { pc = 0x106d88e; continue; }
  pc = 0x106d888; continue;
 }
 0x106d888 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106d880; continue; }
  pc = 0x106d88d; continue;
 }
 0x106d88d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106d88e => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.sf { pc = 0x106d935; continue; }
  pc = 0x106d89a; continue;
 }
 0x106d89a => {
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x106da13; continue; }
  pc = 0x106d8a4; continue;
 }
 0x106d8a4 => {
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x106da97; continue; }
  pc = 0x106d8ae; continue;
 }
 0x106d8ae => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106d914; continue; }
  pc = 0x106d8bf; continue;
 }
 0x106d8bf => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106d914; continue; }
  pc = 0x106d8c8; continue;
 }
 0x106d8c8 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106d914; continue; }
  pc = 0x106d8d9; continue;
 }
 0x106d8d9 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106d914; continue; }
  pc = 0x106d8e2; continue;
 }
 0x106d8e2 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ecx >> 8) as u8), 2u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x2u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106d8f1; continue; }
  pc = 0x106d8ef; continue;
 }
 0x106d8ef => {
  { let v: u8 = 0x2u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x106d8f1; continue;
 }
 0x106d8f1 => {
  { let v: u8 = 0x2u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub16(cpu.r16(0x112be28u32), 0xffffu16); }
  if cpu.zf { pc = 0x106d912; continue; }
  pc = 0x106d900; continue;
 }
 0x106d900 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x5u32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x112be28u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x106d912); cpu.call(t); if cpu.returned_to != 0x106d912 { return; }
  pc = 0x106d912; continue;
 }
 0x106d912 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x106d914; continue;
 }
 0x106d914 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x106d888; continue; }
  pc = 0x106d921; continue;
 }
 0x106d921 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { cpu.poll(); pc = 0x106d888; continue; }
  pc = 0x106d92a; continue;
 }
 0x106d92a => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x106d888; continue;
 }
 0x106d935 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106d9b9; continue; }
  pc = 0x106d946; continue;
 }
 0x106d946 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106d9b9; continue; }
  pc = 0x106d94f; continue;
 }
 0x106d94f => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106d9b9; continue; }
  pc = 0x106d963; continue;
 }
 0x106d963 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106d9b9; continue; }
  pc = 0x106d96c; continue;
 }
 0x106d96c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x106d993; continue; }
  pc = 0x106d976; continue;
 }
 0x106d976 => {
  { let v: u16 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x112309au32), 0xffffu16); }
  if cpu.zf { pc = 0x106d993; continue; }
  pc = 0x106d98a; continue;
 }
 0x106d98a => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x112309au32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x106d9ae; continue;
 }
 0x106d993 => {
  { let v: u16 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1123098u32), 0xffffu16); }
  if !cpu.zf { pc = 0x106d9a7; continue; }
  pc = 0x106d9a3; continue;
 }
 0x106d9a3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x106d9b9; continue;
 }
 0x106d9a7 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x1123098u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x106d9ae; continue;
 }
 0x106d9ae => {
  { let v: u16 = 0xffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x106d9b7); f_0106b882(cpu); if cpu.returned_to != 0x106d9b7 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x106d9b9; continue;
 }
 0x106d9b9 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x106d888; continue; }
  pc = 0x106d9c6; continue;
 }
 0x106d9c6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = 0x112be2cu32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
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
  { cpu.sub16(cpu.r16(0x1123098u32), 0x532u16); }
  if !cpu.zf { pc = 0x106d9fa; continue; }
  pc = 0x106d9f6; continue;
 }
 0x106d9f6 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  pc = 0x106d9fc; continue;
 }
 0x106d9fa => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  pc = 0x106d9fc; continue;
 }
 0x106d9fc => {
  if cpu.zf { pc = 0x106da08; continue; }
  pc = 0x106d9fe; continue;
 }
 0x106d9fe => {
  { let v: u8 = (cpu.eax as u8) | (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x106d888; continue;
 }
 0x106da08 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x106d888; continue;
 }
 0x106da13 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106da75; continue; }
  pc = 0x106da24; continue;
 }
 0x106da24 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106da75; continue; }
  pc = 0x106da2d; continue;
 }
 0x106da2d => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106da75; continue; }
  pc = 0x106da3e; continue;
 }
 0x106da3e => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106da75; continue; }
  pc = 0x106da47; continue;
 }
 0x106da47 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x72cu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1fu8); }
  if (cpu.sf == cpu.of) { pc = 0x106da63; continue; }
  pc = 0x106da5d; continue;
 }
 0x106da5d => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x2u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x106da6d; continue;
 }
 0x106da63 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x3eu8); }
  if (cpu.sf == cpu.of) { pc = 0x106da6d; continue; }
  pc = 0x106da69; continue;
 }
 0x106da69 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x1u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x106da6d; continue;
 }
 0x106da6d => {
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x106da73); cpu.call(t); if cpu.returned_to != 0x106da73 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x106da75; continue;
 }
 0x106da75 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x106d888; continue; }
  pc = 0x106da82; continue;
 }
 0x106da82 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x106da8c; continue; }
  pc = 0x106da87; continue;
 }
 0x106da87 => {
  cpu.poll(); pc = 0x106d888; continue;
 }
 0x106da8c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x106d888; continue;
 }
 0x106da97 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106db02; continue; }
  pc = 0x106daa8; continue;
 }
 0x106daa8 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106db02; continue; }
  pc = 0x106dab1; continue;
 }
 0x106dab1 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106db02; continue; }
  pc = 0x106dac2; continue;
 }
 0x106dac2 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106db02; continue; }
  pc = 0x106dacb; continue;
 }
 0x106dacb => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ecx >> 8) as u8), 2u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x2u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106dada; continue; }
  pc = 0x106dad8; continue;
 }
 0x106dad8 => {
  { let v: u8 = 0x2u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x106dada; continue;
 }
 0x106dada => {
  { let v: u8 = 0x2u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub16(cpu.r16(0x112be2au32), 0xffffu16); }
  if cpu.zf { pc = 0x106db00; continue; }
  pc = 0x106dae9; continue;
 }
 0x106dae9 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0x1fu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x112be2au32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x106db00); cpu.call(t); if cpu.returned_to != 0x106db00 { return; }
  pc = 0x106db00; continue;
 }
 0x106db00 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x106db02; continue;
 }
 0x106db02 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x106d888; continue; }
  pc = 0x106db0f; continue;
 }
 0x106db0f => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { cpu.poll(); pc = 0x106d888; continue; }
  pc = 0x106db18; continue;
 }
 0x106db18 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x106d888; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106db23(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x106db23); } r_0106db23(cpu, 0x106db23); }
pub fn f_0106dc73(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x106dc73); } r_0106db23(cpu, 0x106dc73); }
fn r_0106db23(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x106db23 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x106dd3c; continue; }
  pc = 0x106db31; continue;
 }
 0x106db31 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x106dd3c; continue; }
  pc = 0x106db3b; continue;
 }
 0x106db3b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x106db43); f_0105f178(cpu); if cpu.returned_to != 0x106db43 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x106dbba; continue; }
  pc = 0x106db49; continue;
 }
 0x106db49 => {
  { let v: u32 = 0x112bedcu32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x106dcae; continue; }
  pc = 0x106db58; continue;
 }
 0x106db58 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x106db99; continue; }
  pc = 0x106db5e; continue;
 }
 0x106db5e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x106dba7; continue; }
  pc = 0x106db64; continue;
 }
 0x106db64 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x106dbae; continue; }
  pc = 0x106db6a; continue;
 }
 0x106db6a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x106dba0; continue; }
  pc = 0x106db70; continue;
 }
 0x106db70 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106db84; continue; }
  pc = 0x106db76; continue;
 }
 0x106db76 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x106db8b; continue; }
  pc = 0x106db7c; continue;
 }
 0x106db7c => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x106db92; continue; }
  pc = 0x106db82; continue;
 }
 0x106db82 => {
  pc = 0x106dbb5; continue;
 }
 0x106db84 => {
  { let v: u32 = 0x112c2f0u32; cpu.esi = v; }
  pc = 0x106dbb5; continue;
 }
 0x106db8b => {
  { let v: u32 = 0x112c3b0u32; cpu.esi = v; }
  pc = 0x106dbb5; continue;
 }
 0x106db92 => {
  { let v: u32 = 0x112c470u32; cpu.esi = v; }
  pc = 0x106dbb5; continue;
 }
 0x106db99 => {
  { let v: u32 = 0x112c170u32; cpu.esi = v; }
  pc = 0x106dbb5; continue;
 }
 0x106dba0 => {
  { let v: u32 = 0x112c230u32; cpu.esi = v; }
  pc = 0x106dbb5; continue;
 }
 0x106dba7 => {
  { let v: u32 = 0x112bff0u32; cpu.esi = v; }
  pc = 0x106dbb5; continue;
 }
 0x106dbae => {
  { let v: u32 = 0x112c0b0u32; cpu.esi = v; }
  pc = 0x106dbb5; continue;
 }
 0x106dbb5 => {
  cpu.push(0x106dbba); f_0106d6ee(cpu); if cpu.returned_to != 0x106dbba { return; }
  pc = 0x106dbba; continue;
 }
 0x106dbba => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x106dbd7; continue; }
  pc = 0x106dbc0; continue;
 }
 0x106dbc0 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106dbce; continue; }
  pc = 0x106dbc6; continue;
 }
 0x106dbc6 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x106dbce; continue; }
  pc = 0x106dbcc; continue;
 }
 0x106dbcc => {
  pc = 0x106dbea; continue;
 }
 0x106dbce => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)) & 0x3u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106dc1c; continue;
 }
 0x106dbd7 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x106dd3c; continue; }
  pc = 0x106dbe4; continue;
 }
 0x106dbe4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106dc1c; continue;
 }
 0x106dbea => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x106dbfc; continue; }
  pc = 0x106dbf0; continue;
 }
 0x106dbf0 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x106dbfc; continue; }
  pc = 0x106dbf6; continue;
 }
 0x106dbf6 => {
  { cpu.and8(cpu.r8(0x39u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x106dc0f; continue; }
  pc = 0x106dbfc; continue;
 }
 0x106dbfc => {
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x106dd3c; continue; }
  pc = 0x106dc09; continue;
 }
 0x106dc09 => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106dc1c; continue;
 }
 0x106dc0f => {
  { let v: u8 = cpu.dec8(cpu.r8(0x32u32.wrapping_add(cpu.ebp))); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x106dd3c; continue; }
  pc = 0x106dc18; continue;
 }
 0x106dc18 => {
  { let v: u8 = 0x2u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106dc1c; continue;
 }
 0x106dc1c => {
  { let v: u32 = 0x112be8cu32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x106dcae; continue; }
  pc = 0x106dc2b; continue;
 }
 0x106dc2b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x106dc8b; continue; }
  pc = 0x106dc31; continue;
 }
 0x106dc31 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x106dca0; continue; }
  pc = 0x106dc37; continue;
 }
 0x106dc37 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x106dca7; continue; }
  pc = 0x106dc3d; continue;
 }
 0x106dc3d => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x106dc99; continue; }
  pc = 0x106dc43; continue;
 }
 0x106dc43 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106dc57; continue; }
  pc = 0x106dc49; continue;
 }
 0x106dc49 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x106dc92; continue; }
  pc = 0x106dc4f; continue;
 }
 0x106dc4f => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x106dc7f; continue; }
  pc = 0x106dc55; continue;
 }
 0x106dc55 => {
  pc = 0x106dcae; continue;
 }
 0x106dc57 => {
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x106dc67; continue; }
  pc = 0x106dc5d; continue;
 }
 0x106dc5d => {
  { let v: u32 = 0x112c340u32; cpu.esi = v; }
  cpu.push(0x106dc67); f_0106e1fd(cpu); if cpu.returned_to != 0x106dc67 { return; }
  pc = 0x106dc67; continue;
 }
 0x106dc67 => {
  { let v: u32 = 0x112c2a0u32; cpu.esi = v; }
  cpu.push(0x106dc71); f_0106d600(cpu); if cpu.returned_to != 0x106dc71 { return; }
  pc = 0x106dcb3; continue;
 }
 0x106dc73 => {
  { let v: u32 = 0x112c4e0u32; cpu.esi = v; }
  cpu.push(0x106dc7d); f_0106d600(cpu); if cpu.returned_to != 0x106dc7d { return; }
  pc = 0x106dcb3; continue;
 }
 0x106dc7f => {
  { let v: u32 = 0x112c420u32; cpu.esi = v; }
  cpu.push(0x106dc89); f_0106d600(cpu); if cpu.returned_to != 0x106dc89 { return; }
  pc = 0x106dcb3; continue;
 }
 0x106dc8b => {
  { let v: u32 = 0x112c120u32; cpu.esi = v; }
  pc = 0x106dcae; continue;
 }
 0x106dc92 => {
  { let v: u32 = 0x112c360u32; cpu.esi = v; }
  pc = 0x106dcae; continue;
 }
 0x106dc99 => {
  { let v: u32 = 0x112c1e0u32; cpu.esi = v; }
  pc = 0x106dcae; continue;
 }
 0x106dca0 => {
  { let v: u32 = 0x112bfa0u32; cpu.esi = v; }
  pc = 0x106dcae; continue;
 }
 0x106dca7 => {
  { let v: u32 = 0x112c060u32; cpu.esi = v; }
  pc = 0x106dcae; continue;
 }
 0x106dcae => {
  cpu.push(0x106dcb3); f_0106d56c(cpu); if cpu.returned_to != 0x106dcb3 { return; }
  pc = 0x106dcb3; continue;
 }
 0x106dcb3 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x106dcd1; continue; }
  pc = 0x106dcca; continue;
 }
 0x106dcca => {
  cpu.push(0x106dccf); f_0106e03b(cpu); if cpu.returned_to != 0x106dccf { return; }
  pc = 0x106dd3c; continue;
 }
 0x106dcd1 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x106dcf2; continue; }
  pc = 0x106dceb; continue;
 }
 0x106dceb => {
  cpu.push(0x106dcf0); f_0106defa(cpu); if cpu.returned_to != 0x106dcf0 { return; }
  pc = 0x106dd3c; continue;
 }
 0x106dcf2 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x106dd19; continue; }
  pc = 0x106dd12; continue;
 }
 0x106dd12 => {
  cpu.push(0x106dd17); f_0106de64(cpu); if cpu.returned_to != 0x106dd17 { return; }
  pc = 0x106dd3c; continue;
 }
 0x106dd19 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x106dd3c; continue; }
  pc = 0x106dd37; continue;
 }
 0x106dd37 => {
  cpu.push(0x106dd3c); f_0106dd41(cpu); if cpu.returned_to != 0x106dd3c { return; }
  pc = 0x106dd3c; continue;
 }
 0x106dd3c => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106dd41(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106dd41); }
 let mut pc: u32 = 0x106dd41;
 loop { match pc {
 0x106dd41 => {
  cpu.push(0x106dd46); f_0105f178(cpu); if cpu.returned_to != 0x106dd46 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x106de63; continue; }
  pc = 0x106dd4f; continue;
 }
 0x106dd4f => {
  { let v: u32 = 0x112bf2cu32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x106de47; continue; }
  pc = 0x106dd5e; continue;
 }
 0x106dd5e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x106dde3; continue; }
  pc = 0x106dd64; continue;
 }
 0x106dd64 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x106de15; continue; }
  pc = 0x106dd6e; continue;
 }
 0x106dd6e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x106de2e; continue; }
  pc = 0x106dd78; continue;
 }
 0x106dd78 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x106ddfc; continue; }
  pc = 0x106dd7e; continue;
 }
 0x106dd7e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106ddb1; continue; }
  pc = 0x106dd84; continue;
 }
 0x106dd84 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x106ddca; continue; }
  pc = 0x106dd8a; continue;
 }
 0x106dd8a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x106dd95; continue; }
  pc = 0x106dd90; continue;
 }
 0x106dd90 => {
  pc = 0x106de47; continue;
 }
 0x106dd95 => {
  { let v: u16 = 0x8u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c4c0u32; cpu.esi = v; }
  pc = 0x106de47; continue;
 }
 0x106ddb1 => {
  { let v: u16 = 0x8u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c340u32; cpu.esi = v; }
  pc = 0x106de47; continue;
 }
 0x106ddca => {
  { let v: u16 = 0xau16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c400u32; cpu.esi = v; }
  pc = 0x106de47; continue;
 }
 0x106dde3 => {
  { let v: u16 = 0xau16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c1c0u32; cpu.esi = v; }
  pc = 0x106de47; continue;
 }
 0x106ddfc => {
  { let v: u16 = 0xau16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c280u32; cpu.esi = v; }
  pc = 0x106de47; continue;
 }
 0x106de15 => {
  { let v: u16 = 0x7u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c040u32; cpu.esi = v; }
  pc = 0x106de47; continue;
 }
 0x106de2e => {
  { let v: u16 = 0xcu16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c100u32; cpu.esi = v; }
  pc = 0x106de47; continue;
 }
 0x106de47 => {
  cpu.push(0x106de4c); f_0105f178(cpu); if cpu.returned_to != 0x106de4c { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x1u32); cpu.eax = v; }
  if cpu.zf { pc = 0x106de58; continue; }
  pc = 0x106de51; continue;
 }
 0x106de51 => {
  cpu.push(0x106de56); f_0106e1fd(cpu); if cpu.returned_to != 0x106de56 { return; }
  pc = 0x106de63; continue;
 }
 0x106de58 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106de63; continue; }
  pc = 0x106de5e; continue;
 }
 0x106de5e => {
  cpu.push(0x106de63); f_0106e232(cpu); if cpu.returned_to != 0x106de63 { return; }
  pc = 0x106de63; continue;
 }
 0x106de63 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106de64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106de64); }
 let mut pc: u32 = 0x106de64;
 loop { match pc {
 0x106de64 => {
  cpu.push(0x106de69); f_0105f178(cpu); if cpu.returned_to != 0x106de69 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x106def9; continue; }
  pc = 0x106de72; continue;
 }
 0x106de72 => {
  { let v: u32 = 0x112bf2cu32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x106dedc; continue; }
  pc = 0x106de7d; continue;
 }
 0x106de7d => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x106dec0; continue; }
  pc = 0x106de83; continue;
 }
 0x106de83 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x106dece; continue; }
  pc = 0x106de89; continue;
 }
 0x106de89 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x106ded5; continue; }
  pc = 0x106de8f; continue;
 }
 0x106de8f => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x106dec7; continue; }
  pc = 0x106de95; continue;
 }
 0x106de95 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106deb9; continue; }
  pc = 0x106de9b; continue;
 }
 0x106de9b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x106deb2; continue; }
  pc = 0x106dea1; continue;
 }
 0x106dea1 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x106deab; continue; }
  pc = 0x106dea7; continue;
 }
 0x106dea7 => {
  pc = 0x106dedc; continue;
 }
 0x106deab => {
  { let v: u32 = 0x112c4c0u32; cpu.esi = v; }
  pc = 0x106dedc; continue;
 }
 0x106deb2 => {
  { let v: u32 = 0x112c400u32; cpu.esi = v; }
  pc = 0x106dedc; continue;
 }
 0x106deb9 => {
  { let v: u32 = 0x112c340u32; cpu.esi = v; }
  pc = 0x106dedc; continue;
 }
 0x106dec0 => {
  { let v: u32 = 0x112c1c0u32; cpu.esi = v; }
  pc = 0x106dedc; continue;
 }
 0x106dec7 => {
  { let v: u32 = 0x112c280u32; cpu.esi = v; }
  pc = 0x106dedc; continue;
 }
 0x106dece => {
  { let v: u32 = 0x112c040u32; cpu.esi = v; }
  pc = 0x106dedc; continue;
 }
 0x106ded5 => {
  { let v: u32 = 0x112c100u32; cpu.esi = v; }
  pc = 0x106dedc; continue;
 }
 0x106dedc => {
  cpu.push(0x106dee1); f_0105f178(cpu); if cpu.returned_to != 0x106dee1 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x106deee; continue; }
  pc = 0x106dee7; continue;
 }
 0x106dee7 => {
  cpu.push(0x106deec); f_0106e1fd(cpu); if cpu.returned_to != 0x106deec { return; }
  pc = 0x106def9; continue;
 }
 0x106deee => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106def9; continue; }
  pc = 0x106def4; continue;
 }
 0x106def4 => {
  cpu.push(0x106def9); f_0106e232(cpu); if cpu.returned_to != 0x106def9 { return; }
  pc = 0x106def9; continue;
 }
 0x106def9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106defa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106defa); }
 let mut pc: u32 = 0x106defa;
 loop { match pc {
 0x106defa => {
  { let v: u16 = 0xcu16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112bf2cu32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x106e002; continue; }
  pc = 0x106df1b; continue;
 }
 0x106df1b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x106dfa0; continue; }
  pc = 0x106df21; continue;
 }
 0x106df21 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x106dfd2; continue; }
  pc = 0x106df2b; continue;
 }
 0x106df2b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x106dfeb; continue; }
  pc = 0x106df35; continue;
 }
 0x106df35 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x106dfb9; continue; }
  pc = 0x106df3b; continue;
 }
 0x106df3b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106df87; continue; }
  pc = 0x106df41; continue;
 }
 0x106df41 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x106df6e; continue; }
  pc = 0x106df47; continue;
 }
 0x106df47 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x106df52; continue; }
  pc = 0x106df4d; continue;
 }
 0x106df4d => {
  pc = 0x106e002; continue;
 }
 0x106df52 => {
  { let v: u16 = 0x8u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c4c0u32; cpu.esi = v; }
  pc = 0x106e002; continue;
 }
 0x106df6e => {
  { let v: u16 = 0xau16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c400u32; cpu.esi = v; }
  pc = 0x106e002; continue;
 }
 0x106df87 => {
  { let v: u16 = 0x8u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c340u32; cpu.esi = v; }
  pc = 0x106e002; continue;
 }
 0x106dfa0 => {
  { let v: u16 = 0xau16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c1c0u32; cpu.esi = v; }
  pc = 0x106e002; continue;
 }
 0x106dfb9 => {
  { let v: u16 = 0xau16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c280u32; cpu.esi = v; }
  pc = 0x106e002; continue;
 }
 0x106dfd2 => {
  { let v: u16 = 0x7u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c040u32; cpu.esi = v; }
  pc = 0x106e002; continue;
 }
 0x106dfeb => {
  { let v: u16 = 0xcu16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c100u32; cpu.esi = v; }
  pc = 0x106e002; continue;
 }
 0x106e002 => {
  cpu.push(0x106e007); f_0105f178(cpu); if cpu.returned_to != 0x106e007 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x7u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x106e012; continue; }
  pc = 0x106e00d; continue;
 }
 0x106e00d => {
  cpu.push(0x106e012); f_0106d696(cpu); if cpu.returned_to != 0x106e012 { return; }
  pc = 0x106e012; continue;
 }
 0x106e012 => {
  cpu.push(0x106e017); f_0105f178(cpu); if cpu.returned_to != 0x106e017 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x106e03a; continue; }
  pc = 0x106e01d; continue;
 }
 0x106e01d => {
  cpu.push(0x106e022); f_0105f178(cpu); if cpu.returned_to != 0x106e022 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x106e02f; continue; }
  pc = 0x106e028; continue;
 }
 0x106e028 => {
  cpu.push(0x106e02d); f_0106e268(cpu); if cpu.returned_to != 0x106e02d { return; }
  pc = 0x106e03a; continue;
 }
 0x106e02f => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106e03a; continue; }
  pc = 0x106e035; continue;
 }
 0x106e035 => {
  cpu.push(0x106e03a); f_0106e2a0(cpu); if cpu.returned_to != 0x106e03a { return; }
  pc = 0x106e03a; continue;
 }
 0x106e03a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e03b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106e03b); }
 let mut pc: u32 = 0x106e03b;
 loop { match pc {
 0x106e03b => {
  { let v: u16 = 0xcu16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112bf2cu32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x106e143; continue; }
  pc = 0x106e05c; continue;
 }
 0x106e05c => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x106e0e1; continue; }
  pc = 0x106e062; continue;
 }
 0x106e062 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x106e113; continue; }
  pc = 0x106e06c; continue;
 }
 0x106e06c => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x106e12c; continue; }
  pc = 0x106e076; continue;
 }
 0x106e076 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x106e0fa; continue; }
  pc = 0x106e07c; continue;
 }
 0x106e07c => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106e0c8; continue; }
  pc = 0x106e082; continue;
 }
 0x106e082 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x106e0af; continue; }
  pc = 0x106e088; continue;
 }
 0x106e088 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x106e093; continue; }
  pc = 0x106e08e; continue;
 }
 0x106e08e => {
  pc = 0x106e143; continue;
 }
 0x106e093 => {
  { let v: u16 = 0x8u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c4c0u32; cpu.esi = v; }
  pc = 0x106e143; continue;
 }
 0x106e0af => {
  { let v: u16 = 0xau16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c400u32; cpu.esi = v; }
  pc = 0x106e143; continue;
 }
 0x106e0c8 => {
  { let v: u16 = 0x8u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c340u32; cpu.esi = v; }
  pc = 0x106e143; continue;
 }
 0x106e0e1 => {
  { let v: u16 = 0xau16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c1c0u32; cpu.esi = v; }
  pc = 0x106e143; continue;
 }
 0x106e0fa => {
  { let v: u16 = 0xau16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xau16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x112c280u32; cpu.esi = v; }
  pc = 0x106e143; continue;
 }
 0x106e113 => {
  { let v: u32 = 0x112c040u32; cpu.esi = v; }
  { let v: u16 = 0x7u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x10e3e95u32, v); }
  pc = 0x106e143; continue;
 }
 0x106e12c => {
  { let v: u32 = 0x112c040u32; cpu.esi = v; }
  { let v: u16 = 0xcu16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0xcu16; cpu.w16(0x10e3e95u32, v); }
  pc = 0x106e143; continue;
 }
 0x106e143 => {
  cpu.push(0x106e148); f_0105f178(cpu); if cpu.returned_to != 0x106e148 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0xfu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x106e153; continue; }
  pc = 0x106e14e; continue;
 }
 0x106e14e => {
  cpu.push(0x106e153); f_0106d696(cpu); if cpu.returned_to != 0x106e153 { return; }
  pc = 0x106e153; continue;
 }
 0x106e153 => {
  cpu.push(0x106e158); f_0105f178(cpu); if cpu.returned_to != 0x106e158 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x106e17b; continue; }
  pc = 0x106e15e; continue;
 }
 0x106e15e => {
  cpu.push(0x106e163); f_0105f178(cpu); if cpu.returned_to != 0x106e163 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x106e170; continue; }
  pc = 0x106e169; continue;
 }
 0x106e169 => {
  cpu.push(0x106e16e); f_0106e2d9(cpu); if cpu.returned_to != 0x106e16e { return; }
  pc = 0x106e17b; continue;
 }
 0x106e170 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106e17b; continue; }
  pc = 0x106e176; continue;
 }
 0x106e176 => {
  cpu.push(0x106e17b); f_0106e312(cpu); if cpu.returned_to != 0x106e17b { return; }
  pc = 0x106e17b; continue;
 }
 0x106e17b => {
  cpu.push(0x106e180); f_0105f178(cpu); if cpu.returned_to != 0x106e180 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x106e1a3; continue; }
  pc = 0x106e186; continue;
 }
 0x106e186 => {
  cpu.push(0x106e18b); f_0105f178(cpu); if cpu.returned_to != 0x106e18b { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x106e198; continue; }
  pc = 0x106e191; continue;
 }
 0x106e191 => {
  cpu.push(0x106e196); f_0106e268(cpu); if cpu.returned_to != 0x106e196 { return; }
  pc = 0x106e1a3; continue;
 }
 0x106e198 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x106e1a3; continue; }
  pc = 0x106e19e; continue;
 }
 0x106e19e => {
  cpu.push(0x106e1a3); f_0106e2a0(cpu); if cpu.returned_to != 0x106e1a3 { return; }
  pc = 0x106e1a3; continue;
 }
 0x106e1a3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e1a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106e1a4); }
 let mut pc: u32 = 0x106e1a4;
 loop { match pc {
 0x106e1a4 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r8(0x112c5a0u32.wrapping_add(cpu.eax.wrapping_mul(2))) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x112c5a1u32.wrapping_add(cpu.eax.wrapping_mul(2))) as i8 as i16 as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x727u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x106e1de); cpu.call(t); if cpu.returned_to != 0x106e1de { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e1e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106e1e0); }
 let mut pc: u32 = 0x106e1e0;
 loop { match pc {
 0x106e1e0 => {
  { cpu.pushad(); }
  cpu.push(0x106e1e6); f_0106e523(cpu); if cpu.returned_to != 0x106e1e6 { return; }
  if cpu.cf { pc = 0x106e1fb; continue; }
  pc = 0x106e1e8; continue;
 }
 0x106e1e8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x106e1fb; continue;
 }
 0x106e1fb => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e1fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106e1fd); }
 let mut pc: u32 = 0x106e1fd;
 loop { match pc {
 0x106e1fd => {
  cpu.push(0x106e202); f_0106d795(cpu); if cpu.returned_to != 0x106e202 { return; }
  if cpu.cf { pc = 0x106e231; continue; }
  pc = 0x106e204; continue;
 }
 0x106e204 => {
  cpu.push(0x106e209); f_0106e523(cpu); if cpu.returned_to != 0x106e209 { return; }
  if cpu.cf { pc = 0x106e231; continue; }
  pc = 0x106e20b; continue;
 }
 0x106e20b => {
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
  pc = 0x106e231; continue;
 }
 0x106e231 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e232(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106e232); }
 let mut pc: u32 = 0x106e232;
 loop { match pc {
 0x106e232 => {
  cpu.push(0x106e237); f_0106d795(cpu); if cpu.returned_to != 0x106e237 { return; }
  if cpu.cf { pc = 0x106e267; continue; }
  pc = 0x106e239; continue;
 }
 0x106e239 => {
  cpu.push(0x106e23e); f_0106e523(cpu); if cpu.returned_to != 0x106e23e { return; }
  if cpu.cf { pc = 0x106e267; continue; }
  pc = 0x106e240; continue;
 }
 0x106e240 => {
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
  pc = 0x106e267; continue;
 }
 0x106e267 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e268(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106e268); }
 let mut pc: u32 = 0x106e268;
 loop { match pc {
 0x106e268 => {
  cpu.push(0x106e26d); f_0106d795(cpu); if cpu.returned_to != 0x106e26d { return; }
  if cpu.cf { pc = 0x106e29f; continue; }
  pc = 0x106e26f; continue;
 }
 0x106e26f => {
  cpu.push(0x106e274); f_0106e523(cpu); if cpu.returned_to != 0x106e274 { return; }
  if cpu.cf { pc = 0x106e29f; continue; }
  pc = 0x106e276; continue;
 }
 0x106e276 => {
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
  pc = 0x106e29f; continue;
 }
 0x106e29f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e2a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106e2a0); }
 let mut pc: u32 = 0x106e2a0;
 loop { match pc {
 0x106e2a0 => {
  cpu.push(0x106e2a5); f_0106d795(cpu); if cpu.returned_to != 0x106e2a5 { return; }
  if cpu.cf { pc = 0x106e2d8; continue; }
  pc = 0x106e2a7; continue;
 }
 0x106e2a7 => {
  cpu.push(0x106e2ac); f_0106e523(cpu); if cpu.returned_to != 0x106e2ac { return; }
  if cpu.cf { pc = 0x106e2d8; continue; }
  pc = 0x106e2ae; continue;
 }
 0x106e2ae => {
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
  pc = 0x106e2d8; continue;
 }
 0x106e2d8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e2d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106e2d9); }
 let mut pc: u32 = 0x106e2d9;
 loop { match pc {
 0x106e2d9 => {
  { cpu.pushad(); }
  cpu.push(0x106e2df); f_0106d795(cpu); if cpu.returned_to != 0x106e2df { return; }
  if cpu.cf { pc = 0x106e310; continue; }
  pc = 0x106e2e1; continue;
 }
 0x106e2e1 => {
  cpu.push(0x106e2e6); f_0106e523(cpu); if cpu.returned_to != 0x106e2e6 { return; }
  if cpu.cf { pc = 0x106e310; continue; }
  pc = 0x106e2e8; continue;
 }
 0x106e2e8 => {
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
  pc = 0x106e310; continue;
 }
 0x106e310 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e312(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106e312); }
 let mut pc: u32 = 0x106e312;
 loop { match pc {
 0x106e312 => {
  { cpu.pushad(); }
  cpu.push(0x106e318); f_0106d795(cpu); if cpu.returned_to != 0x106e318 { return; }
  if cpu.cf { pc = 0x106e34a; continue; }
  pc = 0x106e31a; continue;
 }
 0x106e31a => {
  cpu.push(0x106e31f); f_0106e523(cpu); if cpu.returned_to != 0x106e31f { return; }
  if cpu.cf { pc = 0x106e34a; continue; }
  pc = 0x106e321; continue;
 }
 0x106e321 => {
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
  pc = 0x106e34a; continue;
 }
 0x106e34a => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e34c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106e34c); }
 let mut pc: u32 = 0x106e34c;
 loop { match pc {
 0x106e34c => {
  { let v: u32 = 0x116a9e0u32; cpu.ebp = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edi = v; }
  pc = 0x106e35c; continue;
 }
 0x106e35c => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x106e511; continue; }
  pc = 0x106e368; continue;
 }
 0x106e368 => {
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x106e486; continue; }
  pc = 0x106e372; continue;
 }
 0x106e372 => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.sf { pc = 0x106e403; continue; }
  pc = 0x106e37e; continue;
 }
 0x106e37e => {
  { cpu.sub8(cpu.r8(0x11b4c6cu32), 0x0u8); }
  if !cpu.zf { pc = 0x106e39f; continue; }
  pc = 0x106e387; continue;
 }
 0x106e387 => {
  { cpu.sub32(cpu.edi, 0x10e55f8u32); }
  if (cpu.sf == cpu.of) { pc = 0x106e39f; continue; }
  pc = 0x106e38f; continue;
 }
 0x106e38f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e68bau32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x106e3a8; continue; }
  pc = 0x106e39f; continue;
 }
 0x106e39f => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b4c6cu32, v); }
  pc = 0x106e3dc; continue;
 }
 0x106e3a8 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4f4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = 0xffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x106e3dc; continue;
 }
 0x106e3dc => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { pc = 0x106e511; continue; }
  pc = 0x106e3e9; continue;
 }
 0x106e3e9 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x106e511; continue; }
  pc = 0x106e3f6; continue;
 }
 0x106e3f6 => {
  { let v: u8 = 0x5u8; cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.neg16(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106e511; continue;
 }
 0x106e403 => {
  { cpu.sub8(cpu.r8(0x11b4c6cu32), 0x0u8); }
  if !cpu.zf { pc = 0x106e424; continue; }
  pc = 0x106e40c; continue;
 }
 0x106e40c => {
  { cpu.sub32(cpu.edi, 0x10e55f8u32); }
  if (cpu.sf == cpu.of) { pc = 0x106e424; continue; }
  pc = 0x106e414; continue;
 }
 0x106e414 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e68bau32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x106e42d; continue; }
  pc = 0x106e424; continue;
 }
 0x106e424 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b4c6cu32, v); }
  pc = 0x106e461; continue;
 }
 0x106e42d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x5u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x580u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = 0xffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x106e461; continue;
 }
 0x106e461 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { pc = 0x106e511; continue; }
  pc = 0x106e46e; continue;
 }
 0x106e46e => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if (cpu.sf != cpu.of) { pc = 0x106e511; continue; }
  pc = 0x106e47b; continue;
 }
 0x106e47b => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106e511; continue;
 }
 0x106e486 => {
  { cpu.sub8(cpu.r8(0x11b4c6cu32), 0x0u8); }
  if !cpu.zf { pc = 0x106e4a7; continue; }
  pc = 0x106e48f; continue;
 }
 0x106e48f => {
  { cpu.sub32(cpu.edi, 0x10e55f8u32); }
  if (cpu.sf == cpu.of) { pc = 0x106e4a7; continue; }
  pc = 0x106e497; continue;
 }
 0x106e497 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e68bau32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x106e4b0; continue; }
  pc = 0x106e4a7; continue;
 }
 0x106e4a7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b4c6cu32, v); }
  pc = 0x106e4f9; continue;
 }
 0x106e4b0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x70eu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = 0xffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = 0x112be4cu32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.lods(2, false); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x106e4f9; continue;
 }
 0x106e4f9 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { pc = 0x106e511; continue; }
  pc = 0x106e502; continue;
 }
 0x106e502 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if (cpu.sf != cpu.of) { pc = 0x106e511; continue; }
  pc = 0x106e50b; continue;
 }
 0x106e50b => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x106e511; continue;
 }
 0x106e511 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x106e35c; continue; }
  pc = 0x106e51c; continue;
 }
 0x106e51c => {
  { let v: u32 = cpu.edi; cpu.w32(0x10e4010u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e523(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106e523); }
 let mut pc: u32 = 0x106e523;
 loop { match pc {
 0x106e523 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x116a9e0u32; cpu.ebp = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x106e52e; continue;
 }
 0x106e52e => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x106e53d; continue; }
  pc = 0x106e535; continue;
 }
 0x106e535 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106e52e; continue; }
  pc = 0x106e53a; continue;
 }
 0x106e53a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106e53d => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e544(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106e544); }
 let mut pc: u32 = 0x106e544;
 loop { match pc {
 0x106e544 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10e3d41u32, v); }
  { cpu.sub16(cpu.r16(0x10e3d3fu32), 0x0u16); }
  if cpu.zf { pc = 0x106e561; continue; }
  pc = 0x106e555; continue;
 }
 0x106e555 => {
  { let v: u8 = cpu.r8(0x10e3f60u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3d41u32, v); }
  pc = 0x106e561; continue;
 }
 0x106e561 => {
  cpu.push(0x106e566); f_0107161e(cpu); if cpu.returned_to != 0x106e566 { return; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xf02u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u16 = cpu.r16(0x10e3f60u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xfffcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3f64u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x106e58a); f_0104f36d(cpu); if cpu.returned_to != 0x106e58a { return; }
  { let v: u32 = 0xa81fu32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  cpu.push(0x106e597); f_0106e604(cpu); if cpu.returned_to != 0x106e597 { return; }
  { let v: u8 = 0x9u8; cpu.w8(0x10ee917u32, v); }
  { cpu.sub8(cpu.r8(0x10ee97au32), 0x0u8); }
  if !cpu.zf { pc = 0x106e5ad; continue; }
  pc = 0x106e5a7; continue;
 }
 0x106e5a7 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10ee917u32)); cpu.w8(0x10ee917u32, v); }
  pc = 0x106e5ad; continue;
 }
 0x106e5ad => {
  { let v: u8 = 0x13u8; cpu.w8(0x10ee918u32, v); }
  { let v: u16 = 0x140u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e4104u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x10ee918u32).wrapping_sub((cpu.eax as u8)); cpu.w8(0x10ee918u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x106e5cf); f_0106e76c(cpu); if cpu.returned_to != 0x106e5cf { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  if cpu.sf { pc = 0x106e5e4; continue; }
  pc = 0x106e5d7; continue;
 }
 0x106e5d7 => {
  cpu.push(0x106e5dc); f_0106e8e8(cpu); if cpu.returned_to != 0x106e5dc { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x106e5d7; continue; }
  pc = 0x106e5e4; continue;
 }
 0x106e5e4 => {
  cpu.push(0x106e5e9); f_0106e82b(cpu); if cpu.returned_to != 0x106e5e9 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x520u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x10e4020u32)); cpu.esi = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee917u32)); cpu.w8(0x10ee917u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x106e5ad; continue; }
  pc = 0x106e5fe; continue;
 }
 0x106e5fe => {
  cpu.push(0x106e603); f_0106e6bc(cpu); if cpu.returned_to != 0x106e603 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e604(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x106e604); } r_0106e604(cpu, 0x106e604); }
pub fn f_0106e6bc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x106e6bc); } r_0106e604(cpu, 0x106e6bc); }
fn r_0106e604(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x106e604 => {
  { let v: u32 = 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0x10e3f64u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10c6874u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u16 = 0x3cu16; cpu.w16(0x10e3ff4u32, v); }
  { let v: u32 = cpu.r32(0x10e3ff4u32).wrapping_sub(cpu.eax); cpu.w32(0x10e3ff4u32, v); }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x52u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3f60u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u8 = 0x13u8; cpu.w8(0x10ee918u32, v); }
  { let v: u32 = 0x6bu32; cpu.w32(0x10e4020u32, v); }
  { let v: u32 = cpu.r16(0x10e4104u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x140u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ee918u32).wrapping_sub((cpu.eax as u8)); cpu.w8(0x10ee918u32, v); }
  { let v: u32 = cpu.r32(0x10e4020u32).wrapping_add(cpu.eax); cpu.w32(0x10e4020u32, v); }
  cpu.push(0x106e68d); f_0106f58d(cpu); if cpu.returned_to != 0x106e68d { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  pc = 0x106e693; continue;
 }
 0x106e693 => {
  cpu.push(0x106e698); f_0106f54c(cpu); if cpu.returned_to != 0x106e698 { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x106e693; continue; }
  pc = 0x106e6a0; continue;
 }
 0x106e6a0 => {
  cpu.push(0x106e6a5); f_0106f5dc(cpu); if cpu.returned_to != 0x106e6a5 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r16(0x10e3f60u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x10e4020u32)); cpu.esi = v; }
  pc = 0x106e6bb; continue;
 }
 0x106e6bb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106e6bc => {
  { let v: u8 = 0x17u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x10c6874u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x106e6bb; continue; }
  pc = 0x106e6c8; continue;
 }
 0x106e6c8 => {
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8((cpu.eax as u8), 0xfu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106e6cf; continue; }
  pc = 0x106e6cd; continue;
 }
 0x106e6cd => {
  { let v: u8 = 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x106e6cf; continue;
 }
 0x106e6cf => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10c6874u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3ff4u32, v); }
  { let v: u8 = 0x13u8; cpu.w8(0x10ee918u32, v); }
  { let v: u16 = 0x140u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e4104u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x10ee918u32).wrapping_sub((cpu.eax as u8)); cpu.w8(0x10ee918u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x106e6ff); f_0106f58d(cpu); if cpu.returned_to != 0x106e6ff { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  pc = 0x106e705; continue;
 }
 0x106e705 => {
  cpu.push(0x106e70a); f_0106f54c(cpu); if cpu.returned_to != 0x106e70a { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x106e705; continue; }
  pc = 0x106e712; continue;
 }
 0x106e712 => {
  cpu.push(0x106e717); f_0106f5dc(cpu); if cpu.returned_to != 0x106e717 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x106e6bb; continue; }
  pc = 0x106e71d; continue;
 }
 0x106e71d => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x10u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10c6874u32, v); }
  { let v: u32 = cpu.edx.wrapping_add(0x520u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10e4020u32)); cpu.esi = v; }
  { let v: u8 = 0x13u8; cpu.w8(0x10ee918u32, v); }
  { let v: u16 = 0x140u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e4104u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.sub8(cpu.r8(0x10ee918u32), (cpu.eax as u8)); cpu.w8(0x10ee918u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x106e752); f_0106f58d(cpu); if cpu.returned_to != 0x106e752 { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  pc = 0x106e758; continue;
 }
 0x106e758 => {
  cpu.push(0x106e75d); f_0106f54c(cpu); if cpu.returned_to != 0x106e75d { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x106e758; continue; }
  pc = 0x106e765; continue;
 }
 0x106e765 => {
  cpu.push(0x106e76a); f_0106f5dc(cpu); if cpu.returned_to != 0x106e76a { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e76c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106e76c); }
 let mut pc: u32 = 0x106e76c;
 loop { match pc {
 0x106e76c => {
  { cpu.and16(cpu.r16(0x10e3f60u32), 0x8u16); }
  if cpu.zf { f_0106e8e8(cpu); return; }
  pc = 0x106e77b; continue;
 }
 0x106e77b => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.lods(1, false); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x106e78f; continue; }
  pc = 0x106e78b; continue;
 }
 0x106e78b => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106e78f; continue;
 }
 0x106e78f => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c052au32)); }
  if cpu.zf { pc = 0x106e827; continue; }
  pc = 0x106e79c; continue;
 }
 0x106e79c => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  pc = 0x106e827; continue;
 }
 0x106e827 => {
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e82b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106e82b); }
 let mut pc: u32 = 0x106e82b;
 loop { match pc {
 0x106e82b => {
  { cpu.and16(cpu.r16(0x10e3f60u32), 0x8u16); }
  if !cpu.zf { f_0106e8e8(cpu); return; }
  pc = 0x106e83a; continue;
 }
 0x106e83a => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.lods(1, false); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x106e84e; continue; }
  pc = 0x106e84a; continue;
 }
 0x106e84a => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106e84e; continue;
 }
 0x106e84e => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c052au32)); }
  if cpu.zf { pc = 0x106e8e4; continue; }
  pc = 0x106e85b; continue;
 }
 0x106e85b => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  pc = 0x106e8e4; continue;
 }
 0x106e8e4 => {
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106e8e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106e8e8); }
 let mut pc: u32 = 0x106e8e8;
 loop { match pc {
 0x106e8e8 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.lods(1, false); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x106e903; continue; }
  pc = 0x106e8ef; continue;
 }
 0x106e8ef => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x3fffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), cpu.r8(0x4000u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.sf { pc = 0x106eb63; continue; }
  pc = 0x106e903; continue;
 }
 0x106e903 => {
  { let v: u16 = cpu.shl16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x106e914; continue; }
  pc = 0x106e910; continue;
 }
 0x106e910 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106e914; continue;
 }
 0x106e914 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c052au32)); }
  if cpu.zf { pc = 0x106eb53; continue; }
  pc = 0x106e921; continue;
 }
 0x106e921 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x52u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x53u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x54u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x55u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xa4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0xa5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xa6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0xa7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xf6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0xf7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xf8u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0xf9u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x148u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x149u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x14au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x14bu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x19au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x19bu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x19cu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x19du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1ecu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x1edu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1eeu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x1efu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x23eu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x23fu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x240u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x241u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x290u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x21u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x291u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x292u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x293u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x24u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x2e2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x25u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x2e3u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x26u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x2e4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x2e5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x334u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x335u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x336u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x2bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x337u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x386u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x2du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x387u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x388u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x2fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x389u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x3d8u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x3d9u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x3dau32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x3dbu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x34u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x42au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x42bu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x42cu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x37u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x42du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x47cu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x47du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x47eu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x47fu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4ceu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x3du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x4cfu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x3eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4d0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x3fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x4d1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  pc = 0x106eb53; continue;
 }
 0x106eb53 => {
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106eb57 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.poll(); pc = 0x106e903; continue;
 }
 0x106eb63 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  if cpu.sf { cpu.poll(); pc = 0x106eb57; continue; }
  pc = 0x106eb6c; continue;
 }
 0x106eb6c => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x106ecb5; continue; }
  pc = 0x106eb75; continue;
 }
 0x106eb75 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x3fffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), cpu.r8(0x4000u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.sf { pc = 0x106ecb5; continue; }
  pc = 0x106eb89; continue;
 }
 0x106eb89 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  if cpu.sf { pc = 0x106ecaf; continue; }
  pc = 0x106eb95; continue;
 }
 0x106eb95 => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x106ecb5; continue; }
  pc = 0x106eba0; continue;
 }
 0x106eba0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x3fffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), cpu.r8(0x4000u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.sf { pc = 0x106ecb5; continue; }
  pc = 0x106ebb4; continue;
 }
 0x106ebb4 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  if cpu.sf { pc = 0x106ecaf; continue; }
  pc = 0x106ebc0; continue;
 }
 0x106ebc0 => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x106ecb5; continue; }
  pc = 0x106ebcb; continue;
 }
 0x106ebcb => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x3fffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), cpu.r8(0x4000u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.sf { pc = 0x106ecb5; continue; }
  pc = 0x106ebdf; continue;
 }
 0x106ebdf => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  if cpu.sf { pc = 0x106ecaf; continue; }
  pc = 0x106ebeb; continue;
 }
 0x106ebeb => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x106ecb5; continue; }
  pc = 0x106ebf6; continue;
 }
 0x106ebf6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x3fffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), cpu.r8(0x4000u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.sf { pc = 0x106ecb5; continue; }
  pc = 0x106ec0a; continue;
 }
 0x106ec0a => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  if cpu.sf { pc = 0x106ecaf; continue; }
  pc = 0x106ec16; continue;
 }
 0x106ec16 => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x106ecb5; continue; }
  pc = 0x106ec21; continue;
 }
 0x106ec21 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x3fffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), cpu.r8(0x4000u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.sf { pc = 0x106ecb5; continue; }
  pc = 0x106ec35; continue;
 }
 0x106ec35 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  if cpu.sf { pc = 0x106ecaf; continue; }
  pc = 0x106ec3d; continue;
 }
 0x106ec3d => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x106ecb5; continue; }
  pc = 0x106ec44; continue;
 }
 0x106ec44 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x3fffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), cpu.r8(0x4000u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.sf { pc = 0x106ecb5; continue; }
  pc = 0x106ec54; continue;
 }
 0x106ec54 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  if cpu.sf { pc = 0x106ecaf; continue; }
  pc = 0x106ec5c; continue;
 }
 0x106ec5c => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x106ecb5; continue; }
  pc = 0x106ec63; continue;
 }
 0x106ec63 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x3fffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), cpu.r8(0x4000u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.sf { pc = 0x106ecb5; continue; }
  pc = 0x106ec73; continue;
 }
 0x106ec73 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  if cpu.sf { pc = 0x106ecaf; continue; }
  pc = 0x106ec7b; continue;
 }
 0x106ec7b => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x106ecb5; continue; }
  pc = 0x106ec82; continue;
 }
 0x106ec82 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x3fffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), cpu.r8(0x4000u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.sf { pc = 0x106ecb5; continue; }
  pc = 0x106ec92; continue;
 }
 0x106ec92 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  if cpu.sf { pc = 0x106ecaf; continue; }
  pc = 0x106ec9a; continue;
 }
 0x106ec9a => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x106ecb5; continue;
 }
 0x106ecaf => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10ee918u32)); cpu.w8(0x10ee918u32, v); }
  pc = 0x106ecb5; continue;
 }
 0x106ecb5 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x10e4534u32.wrapping_add(cpu.eax)); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106ecc5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106ecc5); }
 let mut pc: u32 = 0x106ecc5;
 loop { match pc {
 0x106ecc5 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x106ecd8; continue; }
  pc = 0x106ecd4; continue;
 }
 0x106ecd4 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106ecd8; continue;
 }
 0x106ecd8 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c052au32)); }
  if cpu.zf { pc = 0x106ee5a; continue; }
  pc = 0x106ece5; continue;
 }
 0x106ece5 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  pc = 0x106ee5a; continue;
 }
 0x106ee5a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x8u32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106ee5f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106ee5f); }
 let mut pc: u32 = 0x106ee5f;
 loop { match pc {
 0x106ee5f => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x106ee72; continue; }
  pc = 0x106ee6e; continue;
 }
 0x106ee6e => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106ee72; continue;
 }
 0x106ee72 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c052au32)); }
  if cpu.zf { pc = 0x106f149; continue; }
  pc = 0x106ee7f; continue;
 }
 0x106ee7f => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x46u32); cpu.edi = v; }
  pc = 0x106f149; continue;
 }
 0x106f149 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0xcu32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f14e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f14e); }
 let mut pc: u32 = 0x106f14e;
 loop { match pc {
 0x106f14e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x106f161; continue; }
  pc = 0x106f15d; continue;
 }
 0x106f15d => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106f161; continue;
 }
 0x106f161 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c052au32)); }
  if cpu.zf { pc = 0x106f1b5; continue; }
  pc = 0x106f16a; continue;
 }
 0x106f16a => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  pc = 0x106f17a; continue;
 }
 0x106f17a => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0xcu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0xcu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0xcu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x42u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106f17a; continue; }
  pc = 0x106f1b4; continue;
 }
 0x106f1b4 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x106f1b5; continue;
 }
 0x106f1b5 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x10u32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f1ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f1ba); }
 let mut pc: u32 = 0x106f1ba;
 loop { match pc {
 0x106f1ba => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x106f1cd; continue; }
  pc = 0x106f1c9; continue;
 }
 0x106f1c9 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106f1cd; continue;
 }
 0x106f1cd => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c052au32)); }
  if cpu.zf { pc = 0x106f22d; continue; }
  pc = 0x106f1d6; continue;
 }
 0x106f1d6 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  pc = 0x106f1e6; continue;
 }
 0x106f1e6 => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x3eu32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106f1e6; continue; }
  pc = 0x106f22c; continue;
 }
 0x106f22c => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x106f22d; continue;
 }
 0x106f22d => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x14u32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f232(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f232); }
 let mut pc: u32 = 0x106f232;
 loop { match pc {
 0x106f232 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x106f245; continue; }
  pc = 0x106f241; continue;
 }
 0x106f241 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106f245; continue;
 }
 0x106f245 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c052au32)); }
  if cpu.zf { pc = 0x106f2b1; continue; }
  pc = 0x106f24e; continue;
 }
 0x106f24e => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  pc = 0x106f25e; continue;
 }
 0x106f25e => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x14u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x14u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x14u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x3au32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106f25e; continue; }
  pc = 0x106f2b0; continue;
 }
 0x106f2b0 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x106f2b1; continue;
 }
 0x106f2b1 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x18u32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f2b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f2b6); }
 let mut pc: u32 = 0x106f2b6;
 loop { match pc {
 0x106f2b6 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x106f2c9; continue; }
  pc = 0x106f2c5; continue;
 }
 0x106f2c5 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106f2c9; continue;
 }
 0x106f2c9 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c052au32)); }
  if cpu.zf { pc = 0x106f341; continue; }
  pc = 0x106f2d2; continue;
 }
 0x106f2d2 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  pc = 0x106f2e2; continue;
 }
 0x106f2e2 => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x18u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x18u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x18u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x36u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106f2e2; continue; }
  pc = 0x106f340; continue;
 }
 0x106f340 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x106f341; continue;
 }
 0x106f341 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x1cu32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f346(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f346); }
 let mut pc: u32 = 0x106f346;
 loop { match pc {
 0x106f346 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x106f359; continue; }
  pc = 0x106f355; continue;
 }
 0x106f355 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106f359; continue;
 }
 0x106f359 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c052au32)); }
  if cpu.zf { pc = 0x106f3dd; continue; }
  pc = 0x106f362; continue;
 }
 0x106f362 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  pc = 0x106f372; continue;
 }
 0x106f372 => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x1cu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x1cu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x1cu32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x32u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106f372; continue; }
  pc = 0x106f3dc; continue;
 }
 0x106f3dc => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x106f3dd; continue;
 }
 0x106f3dd => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x20u32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f3e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f3e2); }
 let mut pc: u32 = 0x106f3e2;
 loop { match pc {
 0x106f3e2 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x106f3f5; continue; }
  pc = 0x106f3f1; continue;
 }
 0x106f3f1 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106f3f5; continue;
 }
 0x106f3f5 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c052au32)); }
  if cpu.zf { pc = 0x106f489; continue; }
  pc = 0x106f402; continue;
 }
 0x106f402 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  pc = 0x106f412; continue;
 }
 0x106f412 => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x2eu32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106f412; continue; }
  pc = 0x106f488; continue;
 }
 0x106f488 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x106f489; continue;
 }
 0x106f489 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x24u32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f48e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f48e); }
 let mut pc: u32 = 0x106f48e;
 loop { match pc {
 0x106f48e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x106f4a1; continue; }
  pc = 0x106f49d; continue;
 }
 0x106f49d => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106f4a1; continue;
 }
 0x106f4a1 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c052au32)); }
  if cpu.zf { pc = 0x106f547; continue; }
  pc = 0x106f4ae; continue;
 }
 0x106f4ae => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  pc = 0x106f4be; continue;
 }
 0x106f4be => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x24u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x24u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x24u32); cpu.edi = v; }
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x2au32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x106f4be; continue; }
  pc = 0x106f546; continue;
 }
 0x106f546 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x106f547; continue;
 }
 0x106f547 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x28u32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f54c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f54c); }
 let mut pc: u32 = 0x106f54c;
 loop { match pc {
 0x106f54c => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.lods(1, false); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x106f560; continue; }
  pc = 0x106f55c; continue;
 }
 0x106f55c => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106f560; continue;
 }
 0x106f560 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c052au32)); }
  if cpu.zf { pc = 0x106f589; continue; }
  pc = 0x106f569; continue;
 }
 0x106f569 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10e3ff4u32)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10c6874u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x106f57c; continue;
 }
 0x106f57c => {
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { let v: u8 = cpu.dec8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x106f57c; continue; }
  pc = 0x106f587; continue;
 }
 0x106f587 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  pc = 0x106f589; continue;
 }
 0x106f589 => {
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f58d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f58d); }
 let mut pc: u32 = 0x106f58d;
 loop { match pc {
 0x106f58d => {
  { cpu.and16(cpu.r16(0x10e3f60u32), 0x8u16); }
  if cpu.zf { f_0106f54c(cpu); return; }
  pc = 0x106f598; continue;
 }
 0x106f598 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.lods(1, false); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x106f5ac; continue; }
  pc = 0x106f5a8; continue;
 }
 0x106f5a8 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106f5ac; continue;
 }
 0x106f5ac => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c052au32)); }
  if cpu.zf { pc = 0x106f5d8; continue; }
  pc = 0x106f5b5; continue;
 }
 0x106f5b5 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10e3ff4u32)); cpu.esi = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10c6874u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x106f5ca; continue;
 }
 0x106f5ca => {
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x106f5ca; continue; }
  pc = 0x106f5d6; continue;
 }
 0x106f5d6 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  pc = 0x106f5d8; continue;
 }
 0x106f5d8 => {
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f5dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f5dc); }
 let mut pc: u32 = 0x106f5dc;
 loop { match pc {
 0x106f5dc => {
  { cpu.and16(cpu.r16(0x10e3f60u32), 0x8u16); }
  if !cpu.zf { f_0106f54c(cpu); return; }
  pc = 0x106f5eb; continue;
 }
 0x106f5eb => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.lods(1, false); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x3fffu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x106f5ff; continue; }
  pc = 0x106f5fb; continue;
 }
 0x106f5fb => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3c00u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106f5ff; continue;
 }
 0x106f5ff => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10c052au32)); }
  if cpu.zf { pc = 0x106f629; continue; }
  pc = 0x106f608; continue;
 }
 0x106f608 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10e3ff4u32)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10c6874u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x106f61b; continue;
 }
 0x106f61b => {
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x50u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { cpu.poll(); pc = 0x106f61b; continue; }
  pc = 0x106f627; continue;
 }
 0x106f627 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  pc = 0x106f629; continue;
 }
 0x106f629 => {
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f630); }
 let mut pc: u32 = 0x106f630;
 loop { match pc {
 0x106f630 => {
  { let v: u16 = 0x464fu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x524du16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x106f63d); f_0106f7d4(cpu); if cpu.returned_to != 0x106f63d { return; }
  if !cpu.cf { pc = 0x106f71e; continue; }
  pc = 0x106f643; continue;
 }
 0x106f643 => {
  { let v: u8 = 0x0u8; cpu.w8(0x112c5b1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c5b0u32, v); }
  { let v: u16 = 0x5042u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4d20u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x106f65f); f_0106f7d4(cpu); if cpu.returned_to != 0x106f65f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x106f67e; continue; }
  pc = 0x106f662; continue;
 }
 0x106f662 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u16 = 0x494cu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x424du16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x106f671); f_0106f7d4(cpu); if cpu.returned_to != 0x106f671 { return; }
  if !cpu.cf { pc = 0x106f71e; continue; }
  pc = 0x106f677; continue;
 }
 0x106f677 => {
  { let v: u8 = 0x1u8; cpu.w8(0x112c5b0u32, v); }
  pc = 0x106f67e; continue;
 }
 0x106f67e => {
  { let v: u16 = 0x424du16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4844u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x106f68b); f_0106f7d4(cpu); if cpu.returned_to != 0x106f68b { return; }
  if !cpu.cf { pc = 0x106f71e; continue; }
  pc = 0x106f691; continue;
 }
 0x106f691 => {
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x112c5b1u32, v); }
  { let v: u16 = 0x434du16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4150u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x106f6a6); f_0106f7d4(cpu); if cpu.returned_to != 0x106f6a6 { return; }
  if !cpu.cf { pc = 0x106f71e; continue; }
  pc = 0x106f6a8; continue;
 }
 0x106f6a8 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = 0x300u32; cpu.ecx = v; }
  pc = 0x106f6b0; continue;
 }
 0x106f6b0 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106f6b0; continue; }
  pc = 0x106f6b7; continue;
 }
 0x106f6b7 => {
  { let v: u16 = 0x424fu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4459u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x106f6c4); f_0106f7d4(cpu); if cpu.returned_to != 0x106f6c4 { return; }
  if !cpu.cf { pc = 0x106f71e; continue; }
  pc = 0x106f6c6; continue;
 }
 0x106f6c6 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x10e436cu32); cpu.push(0x106f6db); cpu.call(t); if cpu.returned_to != 0x106f6db { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x112c5b0u32), 0x0u8); }
  if !cpu.zf { pc = 0x106f720; continue; }
  pc = 0x106f6e6; continue;
 }
 0x106f6e6 => {
  { let v: u8 = 0xc8u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x106f6e8; continue;
 }
 0x106f6e8 => {
  { let v: u16 = 0x140u16; cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  cpu.push(0x106f6f1); f_0106f78d(cpu); if cpu.returned_to != 0x106f6f1 { return; }
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x112c5b2u32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x106f704; continue;
 }
 0x106f704 => {
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), 1u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x10u8); }
  if (cpu.sf != cpu.of) { pc = 0x106f712; continue; }
  pc = 0x106f70f; continue;
 }
 0x106f70f => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x106f712; continue;
 }
 0x106f712 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106f704; continue; }
  pc = 0x106f714; continue;
 }
 0x106f714 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x106f6e8; continue; }
  pc = 0x106f71c; continue;
 }
 0x106f71c => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106f71e => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106f720 => {
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u8 = 0xc8u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x106f72e; continue;
 }
 0x106f72e => {
  { let v: u16 = 0x140u16; cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x106f73a); f_0106f78d(cpu); if cpu.returned_to != 0x106f73a { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x112c5b2u32; cpu.esi = v; }
  { let v: u8 = 0x28u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x106f745; continue;
 }
 0x106f745 => {
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x106f747; continue;
 }
 0x106f747 => {
  { let v: u32 = 0x118u32; cpu.ebx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x106f74e; continue;
 }
 0x106f74e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), 1u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) | (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x28u32); cpu.ebx = v; }
  if !cpu.sf { cpu.poll(); pc = 0x106f74e; continue; }
  pc = 0x106f75e; continue;
 }
 0x106f75e => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x8u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106f76d; continue; }
  pc = 0x106f76a; continue;
 }
 0x106f76a => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x106f76d; continue;
 }
 0x106f76d => {
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.sf { cpu.poll(); pc = 0x106f747; continue; }
  pc = 0x106f77e; continue;
 }
 0x106f77e => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x106f745; continue; }
  pc = 0x106f783; continue;
 }
 0x106f783 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x106f72e; continue; }
  pc = 0x106f78b; continue;
 }
 0x106f78b => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f78d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f78d); }
 let mut pc: u32 = 0x106f78d;
 loop { match pc {
 0x106f78d => {
  { cpu.sub8(cpu.r8(0x112c5b1u32), 0x0u8); }
  if cpu.zf { pc = 0x106f7c5; continue; }
  pc = 0x106f796; continue;
 }
 0x106f796 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x112c5b2u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  pc = 0x106f7a2; continue;
 }
 0x106f7a2 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x106f7b2; continue; }
  pc = 0x106f7a7; continue;
 }
 0x106f7a7 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub((cpu.ecx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.movs(1, true); }
  pc = 0x106f7be; continue;
 }
 0x106f7b2 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub((cpu.ecx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.lods(1, false); }
  { cpu.stos(1, true); }
  pc = 0x106f7be; continue;
 }
 0x106f7be => {
  { let v: u16 = cpu.or16((cpu.ebp as u16), (cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x106f7a2; continue; }
  pc = 0x106f7c3; continue;
 }
 0x106f7c3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106f7c5 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x112c5b2u32; cpu.edi = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f7d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f7d4); }
 let mut pc: u32 = 0x106f7d4;
 loop { match pc {
 0x106f7d4 => {
  { let v: u32 = cpu.r32(0x10e3ff0u32); cpu.ecx = v; }
  pc = 0x106f7da; continue;
 }
 0x106f7da => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), ((cpu.ebx >> 8) as u8)); }
  if !cpu.zf { pc = 0x106f7f2; continue; }
  pc = 0x106f7e0; continue;
 }
 0x106f7e0 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x106f7f2; continue; }
  pc = 0x106f7e5; continue;
 }
 0x106f7e5 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), ((cpu.edx >> 8) as u8)); }
  if !cpu.zf { pc = 0x106f7f2; continue; }
  pc = 0x106f7ea; continue;
 }
 0x106f7ea => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x106f7f2; continue; }
  pc = 0x106f7ef; continue;
 }
 0x106f7ef => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106f7f2 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106f7da; continue; }
  pc = 0x106f7f6; continue;
 }
 0x106f7f6 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f7f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f7f8); }
 let mut pc: u32 = 0x106f7f8;
 loop { match pc {
 0x106f7f8 => {
  let t = cpu.r32(0x10e436cu32); cpu.push(0x106f7fe); cpu.call(t); if cpu.returned_to != 0x106f7fe { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d41u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebp as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x106f81d; continue;
 }
 0x106f81d => {
  { cpu.pushad(); }
  cpu.push(0x106f823); f_0106f8c8(cpu); if cpu.returned_to != 0x106f823 { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x106f838; continue; }
  pc = 0x106f828; continue;
 }
 0x106f828 => {
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106f84b; continue;
 }
 0x106f838 => {
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x106f84b; continue;
 }
 0x106f84b => {
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x106f81d; continue; }
  pc = 0x106f852; continue;
 }
 0x106f852 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f853(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f853); }
 let mut pc: u32 = 0x106f853;
 loop { match pc {
 0x106f853 => {
  let t = cpu.r32(0x10e436cu32); cpu.push(0x106f859); cpu.call(t); if cpu.returned_to != 0x106f859 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d41u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebp as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x10ee988u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x106f87e; continue;
 }
 0x106f87e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.or8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x106f890; continue; }
  pc = 0x106f883; continue;
 }
 0x106f883 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.pushad(); }
  cpu.push(0x106f88b); f_0106f8c8(cpu); if cpu.returned_to != 0x106f88b { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x106f890; continue;
 }
 0x106f890 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x106f8a4; continue; }
  pc = 0x106f894; continue;
 }
 0x106f894 => {
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106f8b7; continue;
 }
 0x106f8a4 => {
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x106f8b7; continue;
 }
 0x106f8b7 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x106f8be; continue; }
  pc = 0x106f8bc; continue;
 }
 0x106f8bc => {
  { let v: u8 = 0x16u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x106f8be; continue;
 }
 0x106f8be => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x1u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x106f87e; continue; }
  pc = 0x106f8c7; continue;
 }
 0x106f8c7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f8c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f8c8); }
 let mut pc: u32 = 0x106f8c8;
 loop { match pc {
 0x106f8c8 => {
  { let v: u16 = cpu.r16(0x10e4108u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e4108u32, v); }
  { let v: u16 = cpu.r16(0x10e4104u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e4104u32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4374u32); cpu.push(0x106f911); cpu.call(t); if cpu.returned_to != 0x106f911 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4374u32); cpu.push(0x106f921); cpu.call(t); if cpu.returned_to != 0x106f921 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4374u32); cpu.push(0x106f931); cpu.call(t); if cpu.returned_to != 0x106f931 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4374u32); cpu.push(0x106f941); cpu.call(t); if cpu.returned_to != 0x106f941 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let a = (cpu.edi as u16); let b = (cpu.esi as u16); { let v: u16 = b; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; } }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4374u32); cpu.push(0x106f954); cpu.call(t); if cpu.returned_to != 0x106f954 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4374u32); cpu.push(0x106f964); cpu.call(t); if cpu.returned_to != 0x106f964 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4374u32); cpu.push(0x106f974); cpu.call(t); if cpu.returned_to != 0x106f974 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4374u32); cpu.push(0x106f984); cpu.call(t); if cpu.returned_to != 0x106f984 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e4104u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e4108u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f995(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f995); }
 let mut pc: u32 = 0x106f995;
 loop { match pc {
 0x106f995 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e40fcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x106f9c3; continue; }
  pc = 0x106f99e; continue;
 }
 0x106f99e => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4100u32)); }
  if cpu.sf { pc = 0x106f9c3; continue; }
  pc = 0x106f9a7; continue;
 }
 0x106f9a7 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106f9c3; continue; }
  pc = 0x106f9b0; continue;
 }
 0x106f9b0 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106f9c3; continue; }
  pc = 0x106f9b9; continue;
 }
 0x106f9b9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x10e4328u32); cpu.push(0x106f9c1); cpu.call(t); if cpu.returned_to != 0x106f9c1 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x106f9c3; continue;
 }
 0x106f9c3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106f9c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106f9c4); }
 let mut pc: u32 = 0x106f9c4;
 loop { match pc {
 0x106f9c4 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e40fcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x106fa31; continue; }
  pc = 0x106f9cd; continue;
 }
 0x106f9cd => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4100u32)); }
  if cpu.sf { pc = 0x106fa31; continue; }
  pc = 0x106f9d6; continue;
 }
 0x106f9d6 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106fa31; continue; }
  pc = 0x106f9df; continue;
 }
 0x106f9df => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106fa31; continue; }
  pc = 0x106f9e8; continue;
 }
 0x106f9e8 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e4328u32); cpu.push(0x106f9ef); cpu.call(t); if cpu.returned_to != 0x106f9ef { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106fa31 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106fa32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106fa32); }
 let mut pc: u32 = 0x106fa32;
 loop { match pc {
 0x106fa32 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d41u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e436cu32); cpu.push(0x106fa44); cpu.call(t); if cpu.returned_to != 0x106fa44 { return; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10c0208u32.wrapping_add(cpu.ebx))); cpu.edi = v; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = 0xf02u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 3u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x106fa8d; continue;
 }
 0x106fa8d => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.stos(2, true); }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x52u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x106fa8d; continue; }
  pc = 0x106fa9f; continue;
 }
 0x106fa9f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106faa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106faa0); }
 let mut pc: u32 = 0x106faa0;
 loop { match pc {
 0x106faa0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d41u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x10e40fcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106fb1e; continue; }
  pc = 0x106fab4; continue;
 }
 0x106fab4 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106fb1e; continue; }
  pc = 0x106fabd; continue;
 }
 0x106fabd => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10e4100u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106fb1e; continue; }
  pc = 0x106fac6; continue;
 }
 0x106fac6 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106fb1e; continue; }
  pc = 0x106facf; continue;
 }
 0x106facf => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e40fcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x106fade; continue; }
  pc = 0x106fad8; continue;
 }
 0x106fad8 => {
  { let v: u16 = cpu.r16(0x10e40fcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106fade; continue;
 }
 0x106fade => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf != cpu.of) { pc = 0x106faee; continue; }
  pc = 0x106fae7; continue;
 }
 0x106fae7 => {
  { let v: u16 = cpu.r16(0x10e4104u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x106faee; continue;
 }
 0x106faee => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4100u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x106fafe; continue; }
  pc = 0x106faf7; continue;
 }
 0x106faf7 => {
  { let v: u16 = cpu.r16(0x10e4100u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x106fafe; continue;
 }
 0x106fafe => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf != cpu.of) { pc = 0x106fb0e; continue; }
  pc = 0x106fb07; continue;
 }
 0x106fb07 => {
  { let v: u16 = cpu.r16(0x10e4108u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x106fb0e; continue;
 }
 0x106fb0e => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x106fb1e; continue; }
  pc = 0x106fb13; continue;
 }
 0x106fb13 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x106fb1e; continue; }
  pc = 0x106fb18; continue;
 }
 0x106fb18 => {
  let t = cpu.r32(0x10e4318u32); cpu.push(0x106fb1e); cpu.call(t); if cpu.returned_to != 0x106fb1e { return; }
  pc = 0x106fb1e; continue;
 }
 0x106fb1e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106fb23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106fb23); }
 let mut pc: u32 = 0x106fb23;
 loop { match pc {
 0x106fb23 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d41u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e436cu32); cpu.push(0x106fb35); cpu.call(t); if cpu.returned_to != 0x106fb35 { return; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10c0208u32.wrapping_add(cpu.ebx))); cpu.edi = v; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and16((cpu.eax as u16), 0x3u16); }
  if cpu.zf { pc = 0x106fb9e; continue; }
  pc = 0x106fb65; continue;
 }
 0x106fb65 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  pc = 0x106fb8b; continue;
 }
 0x106fb8b => {
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x51u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106fb8b; continue; }
  pc = 0x106fb91; continue;
 }
 0x106fb91 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16) & 0xfffcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106fb9e; continue;
 }
 0x106fb9e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = 0xf02u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u16 = (cpu.esi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x106fbd5; continue; }
  pc = 0x106fbc8; continue;
 }
 0x106fbc8 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x52u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x106fbc8; continue; }
  pc = 0x106fbd5; continue;
 }
 0x106fbd5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and8((cpu.ecx as u8), 0x3u8); }
  if cpu.zf { pc = 0x106fc19; continue; }
  pc = 0x106fbdf; continue;
 }
 0x106fbdf => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.eax as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  pc = 0x106fc13; continue;
 }
 0x106fc13 => {
  { cpu.stos(1, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0x51u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106fc13; continue; }
  pc = 0x106fc19; continue;
 }
 0x106fc19 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106fc1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106fc1a); }
 let mut pc: u32 = 0x106fc1a;
 loop { match pc {
 0x106fc1a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d41u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e436cu32); cpu.push(0x106fc2c); cpu.call(t); if cpu.returned_to != 0x106fc2c { return; }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10c0208u32.wrapping_add(cpu.ebx))); cpu.edi = v; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and16((cpu.eax as u16), 0x3u16); }
  if cpu.zf { pc = 0x106fcbf; continue; }
  pc = 0x106fc5c; continue;
 }
 0x106fc5c => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_sub((cpu.eax as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.esi as u16), 0x4u16); }
  if (cpu.sf == cpu.of) { pc = 0x106fc7f; continue; }
  pc = 0x106fc6e; continue;
 }
 0x106fc6e => {
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.esi as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x106fc81; continue;
 }
 0x106fc7f => {
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x106fc81; continue;
 }
 0x106fc81 => {
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  pc = 0x106fc9d; continue;
 }
 0x106fc9d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x51u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106fc9d; continue; }
  pc = 0x106fca8; continue;
 }
 0x106fca8 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16) & 0xfffcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x106fd45; continue; }
  pc = 0x106fcbf; continue;
 }
 0x106fcbf => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = 0xf02u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u16 = (cpu.esi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x106fd00; continue; }
  pc = 0x106fce6; continue;
 }
 0x106fce6 => {
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x106fce7; continue;
 }
 0x106fce7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x52u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x106fce7; continue; }
  pc = 0x106fcff; continue;
 }
 0x106fcff => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x106fd00; continue;
 }
 0x106fd00 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and8((cpu.ecx as u8), 0x3u8); }
  if cpu.zf { pc = 0x106fd45; continue; }
  pc = 0x106fd0a; continue;
 }
 0x106fd0a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.eax as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  pc = 0x106fd3b; continue;
 }
 0x106fd3b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0x51u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x106fd3b; continue; }
  pc = 0x106fd45; continue;
 }
 0x106fd45 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106fd46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106fd46); }
 let mut pc: u32 = 0x106fd46;
 loop { match pc {
 0x106fd46 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d41u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x10e3d41u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e436cu32); cpu.push(0x106fd5a); cpu.call(t); if cpu.returned_to != 0x106fd5a { return; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if !cpu.zf { pc = 0x106fd6e; continue; }
  pc = 0x106fd5f; continue;
 }
 0x106fd5f => {
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x106fd6e; continue; }
  pc = 0x106fd64; continue;
 }
 0x106fd64 => {
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4324u32); cpu.push(0x106fd6d); cpu.call(t); if cpu.returned_to != 0x106fd6d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106fd6e => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106fd78; continue; }
  pc = 0x106fd73; continue;
 }
 0x106fd73 => {
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let a = (cpu.edx as u16); let b = (cpu.ebx as u16); { let v: u16 = b; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } }
  pc = 0x106fd78; continue;
 }
 0x106fd78 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x52u32; cpu.esi = v; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x106fd8a; continue; }
  pc = 0x106fd85; continue;
 }
 0x106fd85 => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x106fd8a; continue;
 }
 0x106fd8a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10c0208u32.wrapping_add(cpu.ebx))); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if cpu.cf { pc = 0x106fe01; continue; }
  pc = 0x106fdc1; continue;
 }
 0x106fdc1 => {
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3dc1u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x106fdd9; continue;
 }
 0x106fdd9 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), 1u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x8u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106fdea; continue; }
  pc = 0x106fde7; continue;
 }
 0x106fde7 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x106fdea; continue;
 }
 0x106fdea => {
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebp as u16), (cpu.edx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x106fdf7; continue; }
  pc = 0x106fdf2; continue;
 }
 0x106fdf2 => {
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.ecx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.esi); cpu.edi = v; }
  pc = 0x106fdf7; continue;
 }
 0x106fdf7 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x10e3dc1u32)); cpu.w16(0x10e3dc1u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x106fdd9; continue; }
  pc = 0x106fe00; continue;
 }
 0x106fe00 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x106fe01 => {
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3dc1u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x106fe19; continue;
 }
 0x106fe19 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u16 = cpu.sub16((cpu.ebp as u16), (cpu.ecx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x106fe38; continue; }
  pc = 0x106fe22; continue;
 }
 0x106fe22 => {
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.edx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), 1u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x8u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106fe35; continue; }
  pc = 0x106fe31; continue;
 }
 0x106fe31 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.inc16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x106fe35; continue;
 }
 0x106fe35 => {
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x106fe38; continue;
 }
 0x106fe38 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x10e3dc1u32)); cpu.w16(0x10e3dc1u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x106fe19; continue; }
  pc = 0x106fe41; continue;
 }
 0x106fe41 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106fe42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106fe42); }
 let mut pc: u32 = 0x106fe42;
 loop { match pc {
 0x106fe42 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d41u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x10e3d41u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106fe5a; continue; }
  pc = 0x106fe55; continue;
 }
 0x106fe55 => {
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let a = (cpu.edx as u16); let b = (cpu.ebx as u16); { let v: u16 = b; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } }
  pc = 0x106fe5a; continue;
 }
 0x106fe5a => {
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_sub((cpu.eax as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_sub((cpu.ebx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e40fcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x106fe99; continue; }
  pc = 0x106fe6f; continue;
 }
 0x106fe6f => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x10e40fcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x106ff43; continue; }
  pc = 0x106fe7c; continue;
 }
 0x106fe7c => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e40fcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.edi as u16); cpu.imul16(v); }
  { let v = (cpu.esi as u16); cpu.idiv16(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x10e40fcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x106fe99; continue;
 }
 0x106fe99 => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf != cpu.of) { pc = 0x106fed2; continue; }
  pc = 0x106fea2; continue;
 }
 0x106fea2 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106ff43; continue; }
  pc = 0x106feaf; continue;
 }
 0x106feaf => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e4104u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.edi as u16); cpu.imul16(v); }
  { let v = (cpu.esi as u16); cpu.idiv16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e4104u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x106fed2; continue;
 }
 0x106fed2 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x106fedc; continue; }
  pc = 0x106fed7; continue;
 }
 0x106fed7 => {
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let a = (cpu.edx as u16); let b = (cpu.ebx as u16); { let v: u16 = b; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } }
  pc = 0x106fedc; continue;
 }
 0x106fedc => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4100u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106ff0b; continue; }
  pc = 0x106fee5; continue;
 }
 0x106fee5 => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10e4100u32)); }
  if (cpu.sf != cpu.of) { pc = 0x106ff43; continue; }
  pc = 0x106feee; continue;
 }
 0x106feee => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e4100u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.esi as u16); cpu.imul16(v); }
  { let v = (cpu.edi as u16); cpu.idiv16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x10e4100u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x106ff0b; continue;
 }
 0x106ff0b => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf != cpu.of) { pc = 0x106ff3d; continue; }
  pc = 0x106ff14; continue;
 }
 0x106ff14 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x106ff43; continue; }
  pc = 0x106ff1d; continue;
 }
 0x106ff1d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e4108u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.esi as u16); cpu.imul16(v); }
  { let v = (cpu.edi as u16); cpu.idiv16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e4108u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x106ff3d; continue;
 }
 0x106ff3d => {
  let t = cpu.r32(0x10e4354u32); cpu.push(0x106ff43); cpu.call(t); if cpu.returned_to != 0x106ff43 { return; }
  pc = 0x106ff43; continue;
 }
 0x106ff43 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0106ff44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x106ff44); }
 let mut pc: u32 = 0x106ff44;
 loop { match pc {
 0x106ff44 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e432cu32); cpu.push(0x106ff52); cpu.call(t); if cpu.returned_to != 0x106ff52 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e432cu32); cpu.push(0x106ff66); cpu.call(t); if cpu.returned_to != 0x106ff66 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4330u32); cpu.push(0x106ff7a); cpu.call(t); if cpu.returned_to != 0x106ff7a { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4330u32); cpu.push(0x106ff8e); cpu.call(t); if cpu.returned_to != 0x106ff8e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01070009(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1070009); }
 let mut pc: u32 = 0x1070009;
 loop { match pc {
 0x1070009 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d41u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10c0208u32.wrapping_add(cpu.ebx))); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107004b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107004b); }
 let mut pc: u32 = 0x107004b;
 loop { match pc {
 0x107004b => {
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10c0208u32.wrapping_add(cpu.ebx))); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { cpu.sub8((cpu.ebx as u8), 0xd5u8); }
  if cpu.zf { pc = 0x10700a5; continue; }
  pc = 0x1070088; continue;
 }
 0x1070088 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x52u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), 1u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x8u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x107009b; continue; }
  pc = 0x1070098; continue;
 }
 0x1070098 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x107009b; continue;
 }
 0x107009b => {
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x52u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10700a5 => {
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x52u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), 1u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x8u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10700b8; continue; }
  pc = 0x10700b5; continue;
 }
 0x10700b5 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x10700b8; continue;
 }
 0x10700b8 => {
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x52u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010700c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10700c2); }
 let mut pc: u32 = 0x10700c2;
 loop { match pc {
 0x10700c2 => {
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10c0208u32.wrapping_add(cpu.ebx))); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x3u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x3u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x52u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), 1u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x8u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x107010f; continue; }
  pc = 0x107010c; continue;
 }
 0x107010c => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x107010f; continue;
 }
 0x107010f => {
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.sub8((cpu.ebx as u8), 0x3u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x52u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107011a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107011a); }
 let mut pc: u32 = 0x107011a;
 loop { match pc {
 0x107011a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10f5e32u32; cpu.edi = v; }
  { let v: u32 = cpu.r16(0x10f78bcu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x10701bb; continue; }
  pc = 0x107013c; continue;
 }
 0x107013c => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x112c6fbu32, v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x112c6fcu32, v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x112c6fdu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10c0208u32.wrapping_add(cpu.ebx))); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u8 = cpu.r8(0x112c6fbu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x112c6fcu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x52u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), 1u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x8u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10701ad; continue; }
  pc = 0x10701aa; continue;
 }
 0x10701aa => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x10701ad; continue;
 }
 0x10701ad => {
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x112c6fdu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x52u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10701bb => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010701c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10701c0); }
 let mut pc: u32 = 0x10701c0;
 loop { match pc {
 0x10701c0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r16(0x10f78bcu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10f5e32u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x1070240; continue; }
  pc = 0x10701e1; continue;
 }
 0x10701e1 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x112c6fbu32, v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x112c6fcu32, v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x112c6fdu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x112c6fbu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x107020c); f_0102d5d9(cpu); if cpu.returned_to != 0x107020c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x112c6fcu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x107021c); f_0102d5d9(cpu); if cpu.returned_to != 0x107021c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x112c6fcu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x107022c); f_0102d5d9(cpu); if cpu.returned_to != 0x107022c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x112c6fdu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x107023d); f_0102d5d9(cpu); if cpu.returned_to != 0x107023d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1070240 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01070245(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1070245); }
 let mut pc: u32 = 0x1070245;
 loop { match pc {
 0x1070245 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x10702fb; continue; }
  pc = 0x107024f; continue;
 }
 0x107024f => {
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1070259; continue; }
  pc = 0x1070254; continue;
 }
 0x1070254 => {
  pc = 0x10702fb; continue;
 }
 0x1070259 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1070254; continue; }
  pc = 0x1070262; continue;
 }
 0x1070262 => {
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1070271; continue; }
  pc = 0x1070267; continue;
 }
 0x1070267 => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x1070254; continue; }
  pc = 0x107026c; continue;
 }
 0x107026c => {
  if cpu.sf { cpu.poll(); pc = 0x1070254; continue; }
  pc = 0x107026e; continue;
 }
 0x107026e => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1070271; continue;
 }
 0x1070271 => {
  { let v: u16 = (cpu.eax as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.ecx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebp as u16), cpu.r16(0x10e4104u32)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x1070287; continue; }
  pc = 0x1070280; continue;
 }
 0x1070280 => {
  { let v: u16 = cpu.sub16((cpu.ecx as u16), (cpu.ebp as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x1070254; continue; }
  pc = 0x1070285; continue;
 }
 0x1070285 => {
  if cpu.zf { cpu.poll(); pc = 0x1070254; continue; }
  pc = 0x1070287; continue;
 }
 0x1070287 => {
  let t = cpu.r32(0x10e436cu32); cpu.push(0x107028d); cpu.call(t); if cpu.returned_to != 0x107028d { return; }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10c0208u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { cpu.sub16((cpu.ecx as u16), 0x4u16); }
  if (cpu.sf != cpu.of) { pc = 0x10702d5; continue; }
  pc = 0x10702d0; continue;
 }
 0x10702d0 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x10702e7; continue; }
  pc = 0x10702d5; continue;
 }
 0x10702d5 => {
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), 1u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x8u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10702f9; continue; }
  pc = 0x10702e0; continue;
 }
 0x10702e0 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub16((cpu.ecx as u16), 0x4u16); }
  if (cpu.sf != cpu.of) { pc = 0x10702f5; continue; }
  pc = 0x10702e7; continue;
 }
 0x10702e7 => {
  { let v: u8 = 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.sub16((cpu.ecx as u16), 0x4u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x10702fb; continue; }
  pc = 0x10702f3; continue;
 }
 0x10702f3 => {
  cpu.poll(); pc = 0x10702e0; continue;
 }
 0x10702f5 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10702f9; continue;
 }
 0x10702f9 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10702d5; continue; }
  pc = 0x10702fb; continue;
 }
 0x10702fb => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010702fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10702fd); }
 let mut pc: u32 = 0x10702fd;
 loop { match pc {
 0x10702fd => {
  { cpu.pushad(); }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x107039c; continue; }
  pc = 0x1070307; continue;
 }
 0x1070307 => {
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1070311; continue; }
  pc = 0x107030c; continue;
 }
 0x107030c => {
  pc = 0x107039c; continue;
 }
 0x1070311 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x107030c; continue; }
  pc = 0x107031a; continue;
 }
 0x107031a => {
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1070329; continue; }
  pc = 0x107031f; continue;
 }
 0x107031f => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.ebx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x107030c; continue; }
  pc = 0x1070324; continue;
 }
 0x1070324 => {
  if cpu.sf { cpu.poll(); pc = 0x107030c; continue; }
  pc = 0x1070326; continue;
 }
 0x1070326 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1070329; continue;
 }
 0x1070329 => {
  { let v: u16 = (cpu.ebx as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.ecx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebp as u16), cpu.r16(0x10e4108u32)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x107033f; continue; }
  pc = 0x1070338; continue;
 }
 0x1070338 => {
  { let v: u16 = cpu.sub16((cpu.ecx as u16), (cpu.ebp as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.sf { cpu.poll(); pc = 0x107030c; continue; }
  pc = 0x107033d; continue;
 }
 0x107033d => {
  if cpu.zf { cpu.poll(); pc = 0x107030c; continue; }
  pc = 0x107033f; continue;
 }
 0x107033f => {
  let t = cpu.r32(0x10e436cu32); cpu.push(0x1070345); cpu.call(t); if cpu.returned_to != 0x1070345 { return; }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10c0208u32.wrapping_add(cpu.ebx))); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = 0x3ceu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  pc = 0x1070395; continue;
 }
 0x1070395 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x52u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1070395; continue; }
  pc = 0x107039c; continue;
 }
 0x107039c => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107039e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107039e); }
 let mut pc: u32 = 0x107039e;
 loop { match pc {
 0x107039e => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x112c6fau32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d41u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebp as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x10703c7; continue;
 }
 0x10703c7 => {
  { cpu.pushad(); }
  cpu.push(0x10703cd); f_01070403(cpu); if cpu.returned_to != 0x10703cd { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x10703e2; continue; }
  pc = 0x10703d2; continue;
 }
 0x10703d2 => {
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10703f5; continue;
 }
 0x10703e2 => {
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10703f5; continue;
 }
 0x10703f5 => {
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10703c7; continue; }
  pc = 0x10703fc; continue;
 }
 0x10703fc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010703fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10703fd); }
 let mut pc: u32 = 0x10703fd;
 loop { match pc {
 0x10703fd => {
  cpu.push(0x1070402); f_0107039e(cpu); if cpu.returned_to != 0x1070402 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01070403(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1070403); }
 let mut pc: u32 = 0x1070403;
 loop { match pc {
 0x1070403 => {
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x107041f); f_0107048e(cpu); if cpu.returned_to != 0x107041f { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x107042e); f_0107048e(cpu); if cpu.returned_to != 0x107042e { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x107043d); f_0107048e(cpu); if cpu.returned_to != 0x107043d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x107044c); f_0107048e(cpu); if cpu.returned_to != 0x107044c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let a = (cpu.edi as u16); let b = (cpu.esi as u16); { let v: u16 = b; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; } }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x107045e); f_0107048e(cpu); if cpu.returned_to != 0x107045e { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x107046d); f_0107048e(cpu); if cpu.returned_to != 0x107046d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x107047c); f_0107048e(cpu); if cpu.returned_to != 0x107047c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x107048b); f_0107048e(cpu); if cpu.returned_to != 0x107048b { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107048e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107048e); }
 let mut pc: u32 = 0x107048e;
 loop { match pc {
 0x107048e => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e40fcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x10704c1; continue; }
  pc = 0x1070497; continue;
 }
 0x1070497 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4100u32)); }
  if cpu.sf { pc = 0x10704c1; continue; }
  pc = 0x10704a0; continue;
 }
 0x10704a0 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x10704c1; continue; }
  pc = 0x10704a9; continue;
 }
 0x10704a9 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x10704c1; continue; }
  pc = 0x10704b2; continue;
 }
 0x10704b2 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r8(0x112c6fau32) as u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e437cu32); cpu.push(0x10704c0); cpu.call(t); if cpu.returned_to != 0x10704c0 { return; }
  { cpu.popad(); }
  pc = 0x10704c1; continue;
 }
 0x10704c1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010704c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10704c2); }
 let mut pc: u32 = 0x10704c2;
 loop { match pc {
 0x10704c2 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x112c700u32, v); }
  { let v: u8 = cpu.r8(0x112c6feu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x112c6ffu32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if !cpu.zf { pc = 0x10704e0; continue; }
  pc = 0x10704da; continue;
 }
 0x10704da => {
  let t = cpu.r32(0x10e436cu32); cpu.push(0x10704e0); cpu.call(t); if cpu.returned_to != 0x10704e0 { return; }
  pc = 0x10704e0; continue;
 }
 0x10704e0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1070508; continue; }
  pc = 0x10704f0; continue;
 }
 0x10704f0 => {
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x1070508; continue; }
  pc = 0x10704f5; continue;
 }
 0x10704f5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r8(0x112c700u32) as u32; cpu.ebp = v; }
  cpu.push(0x1070502); f_01070516(cpu); if cpu.returned_to != 0x1070502 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  cpu.poll(); pc = 0x10704e0; continue;
 }
 0x1070508 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x112c6feu32)); cpu.w8(0x112c6feu32, v); }
  { let v: u8 = cpu.and8(cpu.r8(0x112c6feu32), 0x3u8); cpu.w8(0x112c6feu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01070516(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1070516); }
 let mut pc: u32 = 0x1070516;
 loop { match pc {
 0x1070516 => {
  { let v: u32 = cpu.eax; cpu.w32(0x112c704u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x112c708u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x112c70cu32, v); }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1070591; continue; }
  pc = 0x107052b; continue;
 }
 0x107052b => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.ebx); cpu.edx = v; }
  if !cpu.sf { pc = 0x1070538; continue; }
  pc = 0x1070534; continue;
 }
 0x1070534 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x1070538; continue;
 }
 0x1070538 => {
  { let v: u32 = cpu.esi; cpu.w32(0x112c710u32, v); }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x107056b; continue; }
  pc = 0x1070542; continue;
 }
 0x1070542 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x112c714u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x112c718u32, v); }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x112c728u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x112c720u32, v); }
  { let v: u32 = 0x1070671u32; cpu.w32(0x112c724u32, v); }
  pc = 0x10705f4; continue;
 }
 0x107056b => {
  { let v: u32 = cpu.edx; cpu.w32(0x112c714u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x112c718u32, v); }
  { let v: u32 = cpu.sar32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x112c728u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x112c71cu32, v); }
  { let v: u32 = 0x10706bcu32; cpu.w32(0x112c724u32, v); }
  pc = 0x10705f4; continue;
 }
 0x1070591 => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.ebx); cpu.edx = v; }
  if !cpu.sf { pc = 0x10705a0; continue; }
  pc = 0x107059c; continue;
 }
 0x107059c => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x10705a0; continue;
 }
 0x10705a0 => {
  { let v: u32 = cpu.esi; cpu.w32(0x112c710u32, v); }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x10705d0; continue; }
  pc = 0x10705aa; continue;
 }
 0x10705aa => {
  { let v: u32 = cpu.ecx; cpu.w32(0x112c714u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x112c718u32, v); }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x112c728u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x112c720u32, v); }
  { let v: u32 = 0x1070707u32; cpu.w32(0x112c724u32, v); }
  pc = 0x10705f4; continue;
 }
 0x10705d0 => {
  { let v: u32 = cpu.edx; cpu.w32(0x112c714u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x112c718u32, v); }
  { let v: u32 = cpu.sar32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x112c728u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x112c71cu32, v); }
  { let v: u32 = 0x1070752u32; cpu.w32(0x112c724u32, v); }
  pc = 0x10705f4; continue;
 }
 0x10705f4 => {
  { cpu.sub32(cpu.r32(0x112c714u32), 0x0u32); }
  if cpu.zf { pc = 0x1070670; continue; }
  pc = 0x10705fd; continue;
 }
 0x10705fd => {
  { let v: u32 = cpu.r8(0x112c6ffu32) as u32; cpu.ecx = v; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x107062a; continue; }
  pc = 0x1070608; continue;
 }
 0x1070608 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x112c714u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1070624; continue; }
  pc = 0x1070610; continue;
 }
 0x1070610 => {
  { let v: u32 = cpu.r32(0x112c714u32); cpu.ecx = v; }
  { let v: u8 = cpu.sub8(cpu.r8(0x112c6ffu32), (cpu.ecx as u8)); cpu.w8(0x112c6ffu32, v); }
  let t = cpu.r32(0x112c724u32); cpu.push(0x1070622); cpu.call(t); if cpu.returned_to != 0x1070622 { return; }
  pc = 0x1070670; continue;
 }
 0x1070624 => {
  let t = cpu.r32(0x112c724u32); cpu.push(0x107062a); cpu.call(t); if cpu.returned_to != 0x107062a { return; }
  pc = 0x107062a; continue;
 }
 0x107062a => {
  { let v: u8 = 0x4u8; cpu.w8(0x112c6ffu32, v); }
  { let v: u32 = cpu.r32(0x112c704u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112c708u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x112c70cu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  if cpu.sf { cpu.poll(); pc = 0x10705f4; continue; }
  pc = 0x107064a; continue;
 }
 0x107064a => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  if cpu.sf { cpu.poll(); pc = 0x10705f4; continue; }
  pc = 0x1070652; continue;
 }
 0x1070652 => {
  { let v: u32 = cpu.r32(0x10e4104u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10705f4; continue; }
  pc = 0x107065d; continue;
 }
 0x107065d => {
  { let v: u32 = cpu.r32(0x10e4108u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10705f4; continue; }
  pc = 0x1070668; continue;
 }
 0x1070668 => {
  let t = cpu.r32(0x10e4328u32); cpu.push(0x107066e); cpu.call(t); if cpu.returned_to != 0x107066e { return; }
  cpu.poll(); pc = 0x10705f4; continue;
 }
 0x1070670 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01070671(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1070671); }
 let mut pc: u32 = 0x1070671;
 loop { match pc {
 0x1070671 => {
  { let v: u32 = cpu.r32(0x112c704u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x112c710u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x112c714u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x1070688; continue; }
  pc = 0x1070686; continue;
 }
 0x1070686 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1070688; continue;
 }
 0x1070688 => {
  { let v: u32 = cpu.r32(0x112c728u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x112c720u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112c718u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x112c714u32).wrapping_sub(cpu.ecx); cpu.w32(0x112c714u32, v); }
  pc = 0x107069f; continue;
 }
 0x107069f => {
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  if !cpu.sf { pc = 0x10706ab; continue; }
  pc = 0x10706a3; continue;
 }
 0x10706a3 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x112c708u32), cpu.esi); cpu.w32(0x112c708u32, v); }
  pc = 0x10706ab; continue;
 }
 0x10706ab => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x107069f; continue; }
  pc = 0x10706af; continue;
 }
 0x10706af => {
  { let v: u32 = cpu.ebx; cpu.w32(0x112c704u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x112c728u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010706bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10706bc); }
 let mut pc: u32 = 0x10706bc;
 loop { match pc {
 0x10706bc => {
  { let v: u32 = cpu.r32(0x112c704u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x112c710u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x112c714u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x10706d3; continue; }
  pc = 0x10706d1; continue;
 }
 0x10706d1 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x10706d3; continue;
 }
 0x10706d3 => {
  { let v: u32 = cpu.r32(0x112c728u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x112c71cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112c718u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x112c714u32).wrapping_sub(cpu.ecx); cpu.w32(0x112c714u32, v); }
  pc = 0x10706ea; continue;
 }
 0x10706ea => {
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  if !cpu.sf { pc = 0x10706f1; continue; }
  pc = 0x10706ee; continue;
 }
 0x10706ee => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x10706f1; continue;
 }
 0x10706f1 => {
  { let v: u32 = cpu.add32(cpu.r32(0x112c708u32), cpu.esi); cpu.w32(0x112c708u32, v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10706ea; continue; }
  pc = 0x10706fa; continue;
 }
 0x10706fa => {
  { let v: u32 = cpu.edx; cpu.w32(0x112c728u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x112c704u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01070707(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1070707); }
 let mut pc: u32 = 0x1070707;
 loop { match pc {
 0x1070707 => {
  { let v: u32 = cpu.r32(0x112c704u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x112c710u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x112c714u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x107071e; continue; }
  pc = 0x107071c; continue;
 }
 0x107071c => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x107071e; continue;
 }
 0x107071e => {
  { let v: u32 = cpu.r32(0x112c728u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x112c720u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112c718u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x112c714u32).wrapping_sub(cpu.ecx); cpu.w32(0x112c714u32, v); }
  pc = 0x1070735; continue;
 }
 0x1070735 => {
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  if !cpu.sf { pc = 0x1070741; continue; }
  pc = 0x1070739; continue;
 }
 0x1070739 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x112c708u32), cpu.esi); cpu.w32(0x112c708u32, v); }
  pc = 0x1070741; continue;
 }
 0x1070741 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1070735; continue; }
  pc = 0x1070745; continue;
 }
 0x1070745 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x112c704u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x112c728u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01070752(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1070752); }
 let mut pc: u32 = 0x1070752;
 loop { match pc {
 0x1070752 => {
  { let v: u32 = cpu.r32(0x112c704u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x112c710u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x112c714u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x1070769; continue; }
  pc = 0x1070767; continue;
 }
 0x1070767 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1070769; continue;
 }
 0x1070769 => {
  { let v: u32 = cpu.r32(0x112c728u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x112c71cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112c718u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x112c714u32).wrapping_sub(cpu.ecx); cpu.w32(0x112c714u32, v); }
  pc = 0x1070780; continue;
 }
 0x1070780 => {
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  if !cpu.sf { pc = 0x107078b; continue; }
  pc = 0x1070784; continue;
 }
 0x1070784 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x112c718u32)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x107078b; continue;
 }
 0x107078b => {
  { let v: u32 = cpu.add32(cpu.r32(0x112c708u32), cpu.esi); cpu.w32(0x112c708u32, v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1070780; continue; }
  pc = 0x1070794; continue;
 }
 0x1070794 => {
  { let v: u32 = cpu.edx; cpu.w32(0x112c728u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x112c704u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010707a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10707a4); }
 let mut pc: u32 = 0x10707a4;
 loop { match pc {
 0x10707a4 => {
  { cpu.sub8(cpu.r8(0x105593eu32), 0x0u8); }
  if cpu.zf { pc = 0x10707ae; continue; }
  pc = 0x10707ad; continue;
 }
 0x10707ad => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10707ae => {
  { let v: u32 = 0x78u32; cpu.eax = v; }
  { let v: u32 = 0x84u32; cpu.ebx = v; }
  cpu.push(0x10707bd); f_0107081c(cpu); if cpu.returned_to != 0x10707bd { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010707be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10707be); }
 let mut pc: u32 = 0x10707be;
 loop { match pc {
 0x10707be => {
  { cpu.sub8(cpu.r8(0x105593eu32), 0x0u8); }
  if cpu.zf { pc = 0x10707c8; continue; }
  pc = 0x10707c7; continue;
 }
 0x10707c7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10707c8 => {
  { let v: u32 = 0x78u32; cpu.eax = v; }
  { let v: u32 = 0x84u32; cpu.ebx = v; }
  cpu.push(0x10707d7); f_0107099e(cpu); if cpu.returned_to != 0x10707d7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010707d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10707d8); }
 let mut pc: u32 = 0x10707d8;
 loop { match pc {
 0x10707d8 => {
  { cpu.sub8(cpu.r8(0x105593eu32), 0x0u8); }
  if cpu.zf { pc = 0x10707e2; continue; }
  pc = 0x10707e1; continue;
 }
 0x10707e1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10707e2 => {
  { cpu.sub8(cpu.r8(0x11b51b9u32), 0x0u8); }
  if !cpu.zf { pc = 0x10707ec; continue; }
  pc = 0x10707eb; continue;
 }
 0x10707eb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10707ec => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x1105668u32), 0x0u8); }
  if cpu.zf { pc = 0x107081a; continue; }
  pc = 0x10707f6; continue;
 }
 0x10707f6 => {
  cpu.push(0x10707fb); f_0107153d(cpu); if cpu.returned_to != 0x10707fb { return; }
  { cpu.and8(cpu.r8(0x1105667u32), 0x10u8); }
  if !cpu.zf { pc = 0x1070810; continue; }
  pc = 0x1070804; continue;
 }
 0x1070804 => {
  cpu.push(0x1070809); f_01070a62(cpu); if cpu.returned_to != 0x1070809 { return; }
  cpu.push(0x107080e); f_010715b4(cpu); if cpu.returned_to != 0x107080e { return; }
  pc = 0x107081a; continue;
 }
 0x1070810 => {
  cpu.push(0x1070815); f_010708e0(cpu); if cpu.returned_to != 0x1070815 { return; }
  cpu.push(0x107081a); f_010715b4(cpu); if cpu.returned_to != 0x107081a { return; }
  pc = 0x107081a; continue;
 }
 0x107081a => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107081c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107081c); }
 let mut pc: u32 = 0x107081c;
 loop { match pc {
 0x107081c => {
  { cpu.sub8(cpu.r8(0x105593eu32), 0x0u8); }
  if cpu.zf { pc = 0x1070826; continue; }
  pc = 0x1070825; continue;
 }
 0x1070825 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1070826 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10c0208u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1105669u32, v); }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0xa8080u32; cpu.edi = v; }
  cpu.push(0x1070851); f_0107161e(cpu); if cpu.returned_to != 0x1070851 { return; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xf02u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  pc = 0x1070860; continue;
 }
 0x1070860 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x3eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1070860; continue; }
  pc = 0x10708de; continue;
 }
 0x10708de => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010708e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10708e0); }
 let mut pc: u32 = 0x10708e0;
 loop { match pc {
 0x10708e0 => {
  { cpu.sub8(cpu.r8(0x105593eu32), 0x0u8); }
  if cpu.zf { pc = 0x10708ea; continue; }
  pc = 0x10708e9; continue;
 }
 0x10708e9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10708ea => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x1105669u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x107099c; continue; }
  pc = 0x10708f9; continue;
 }
 0x10708f9 => {
  { let v: u32 = cpu.r16(0x10e3d85u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0xa8080u32; cpu.esi = v; }
  cpu.push(0x107090f); f_0107161e(cpu); if cpu.returned_to != 0x107090f { return; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xf02u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  pc = 0x107091e; continue;
 }
 0x107091e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x3eu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x107091e; continue; }
  pc = 0x107099c; continue;
 }
 0x107099c => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107099e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107099e); }
 let mut pc: u32 = 0x107099e;
 loop { match pc {
 0x107099e => {
  { cpu.sub8(cpu.r8(0x105593eu32), 0x0u8); }
  if cpu.zf { pc = 0x10709a8; continue; }
  pc = 0x10709a7; continue;
 }
 0x10709a7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10709a8 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10c0208u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11c6330u32, v); }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0xa9080u32; cpu.edi = v; }
  cpu.push(0x10709d3); f_0107161e(cpu); if cpu.returned_to != 0x10709d3 { return; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xf02u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  pc = 0x10709e2; continue;
 }
 0x10709e2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x3eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10709e2; continue; }
  pc = 0x1070a60; continue;
 }
 0x1070a60 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01070a62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1070a62); }
 let mut pc: u32 = 0x1070a62;
 loop { match pc {
 0x1070a62 => {
  { cpu.sub8(cpu.r8(0x105593eu32), 0x0u8); }
  if cpu.zf { pc = 0x1070a6c; continue; }
  pc = 0x1070a6b; continue;
 }
 0x1070a6b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1070a6c => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x11c6330u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x1070b1e; continue; }
  pc = 0x1070a7b; continue;
 }
 0x1070a7b => {
  { let v: u32 = cpu.r16(0x10e3d85u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0xa9080u32; cpu.esi = v; }
  cpu.push(0x1070a91); f_0107161e(cpu); if cpu.returned_to != 0x1070a91 { return; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xf02u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  pc = 0x1070aa0; continue;
 }
 0x1070aa0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x3eu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1070aa0; continue; }
  pc = 0x1070b1e; continue;
 }
 0x1070b1e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01070b20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1070b20); }
 let mut pc: u32 = 0x1070b20;
 loop { match pc {
 0x1070b20 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x10e44ccu32; cpu.edi = v; }
  { let v: u32 = 0x104acd0u32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01070b61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1070b61); }
 let mut pc: u32 = 0x1070b61;
 loop { match pc {
 0x1070b61 => {
  { cpu.sub8(cpu.r8(0x10e68f0u32), 0x0u8); }
  if !cpu.zf { pc = 0x1070b6f; continue; }
  pc = 0x1070b6a; continue;
 }
 0x1070b6a => {
  cpu.push(0x1070b6f); f_0106257c(cpu); if cpu.returned_to != 0x1070b6f { return; }
  pc = 0x1070b6f; continue;
 }
 0x1070b6f => {
  { let v: u32 = 0x10d0u32; cpu.ebx = v; }
  cpu.push(0x1070b79); f_0105dce8(cpu); if cpu.returned_to != 0x1070b79 { return; }
  { let v: u32 = 0xa81fu32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef31eu32, v); }
  { let v: u32 = 0x10e68f0u32; cpu.edx = v; }
  cpu.push(0x1070b92); f_01070ba9(cpu); if cpu.returned_to != 0x1070b92 { return; }
  { let v: u32 = 0x10e68feu32; cpu.edx = v; }
  cpu.push(0x1070b9c); f_01070ba9(cpu); if cpu.returned_to != 0x1070b9c { return; }
  cpu.push(0x1070ba1); f_0105dd62(cpu); if cpu.returned_to != 0x1070ba1 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x11c59ceu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01070ba9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1070ba9); }
 let mut pc: u32 = 0x1070ba9;
 loop { match pc {
 0x1070ba9 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4024u32); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x1070bbb); f_01033bea(cpu); if cpu.returned_to != 0x1070bbb { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10e4024u32); cpu.esi = v; }
  { let v: u32 = 0x11a37f0u32; cpu.edi = v; }
  cpu.push(0x1070bcc); f_0106f630(cpu); if cpu.returned_to != 0x1070bcc { return; }
  if !cpu.cf { pc = 0x1070bd3; continue; }
  pc = 0x1070bce; continue;
 }
 0x1070bce => {
  f_0105f11c(cpu); return;
 }
 0x1070bd3 => {
  { let v: u32 = 0x11a37f0u32; cpu.esi = v; }
  { let v: u32 = 0x11cc2b8u32; cpu.edi = v; }
  { let v: u32 = 0x300u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.push(0x1070be9); f_0107161e(cpu); if cpu.returned_to != 0x1070be9 { return; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xf02u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u8 = 0xcu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1070c00; continue;
 }
 0x1070c00 => {
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1070c02; continue;
 }
 0x1070c02 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x1070c08; continue;
 }
 0x1070c08 => {
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4eu32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1070c08; continue; }
  pc = 0x1070c11; continue;
 }
 0x1070c11 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1070c02; continue; }
  pc = 0x1070c19; continue;
 }
 0x1070c19 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x4d0u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x1070c00; continue; }
  pc = 0x1070c23; continue;
 }
 0x1070c23 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01070c24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1070c24); }
 let mut pc: u32 = 0x1070c24;
 loop { match pc {
 0x1070c24 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x10e436cu32); cpu.push(0x1070c2b); cpu.call(t); if cpu.returned_to != 0x1070c2b { return; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u8 = 0xc8u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1070c39; continue;
 }
 0x1070c39 => {
  { let v: u8 = 0x28u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1070c3b; continue;
 }
 0x1070c3b => {
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1070c3d; continue;
 }
 0x1070c3d => {
  { let v: u32 = 0x118u32; cpu.ebx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1070c44; continue;
 }
 0x1070c44 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), 1u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) | (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x28u32); cpu.ebx = v; }
  if !cpu.sf { cpu.poll(); pc = 0x1070c44; continue; }
  pc = 0x1070c54; continue;
 }
 0x1070c54 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x8u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1070c63; continue; }
  pc = 0x1070c60; continue;
 }
 0x1070c60 => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x1070c63; continue;
 }
 0x1070c63 => {
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.sf { cpu.poll(); pc = 0x1070c3d; continue; }
  pc = 0x1070c74; continue;
 }
 0x1070c74 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x1070c3b; continue; }
  pc = 0x1070c79; continue;
 }
 0x1070c79 => {
  { let v: u32 = cpu.esi.wrapping_add(0x118u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x1070c39; continue; }
  pc = 0x1070c86; continue;
 }
 0x1070c86 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01070c88(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1070c88); } r_01070c88(cpu, 0x1070c88); }
pub fn f_01070d0b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1070d0b); } r_01070c88(cpu, 0x1070d0b); }
fn r_01070c88(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1070c88 => {
  { cpu.sub8(cpu.r8(0x10fd361u32), 0x0u8); }
  if !cpu.zf { pc = 0x1070dba; continue; }
  pc = 0x1070c95; continue;
 }
 0x1070c95 => {
  { cpu.pushad(); }
  cpu.push(0x1070c9b); f_01070dbb(cpu); if cpu.returned_to != 0x1070c9b { return; }
  { let v = cpu.r16(0x1128db2u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r32(0x10e3fd0u32); cpu.push(v); }
  { let v = cpu.r32(0x10e3fd4u32); cpu.push(v); }
  { let v = cpu.r16(0x10c0528u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3d89u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3d9du32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3d9fu32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e4104u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e4108u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r32(0x10e40fcu32); cpu.push(v); }
  { let v = cpu.r32(0x10e4100u32); cpu.push(v); }
  { let v: u16 = 0x140u16; cpu.w16(0x10e4104u32, v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x10e4108u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e40fcu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4100u32, v); }
  { let v = cpu.r16(0x10c6872u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10c6872u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r32(0x10e4114u32); cpu.push(v); }
  { let v: u32 = cpu.r16(0x10e3d85u32) as u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3d89u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10e4114u32, v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x10e4114u32), 4u32); cpu.w32(0x10e4114u32, v); }
  cpu.push(0x1070d3e); f_0107153d(cpu); if cpu.returned_to != 0x1070d3e { return; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x1070d44); cpu.call(t); if cpu.returned_to != 0x1070d44 { return; }
  { cpu.sub8(cpu.r8(0x10ee974u32), 0x0u8); }
  if cpu.zf { pc = 0x1070d52; continue; }
  pc = 0x1070d4d; continue;
 }
 0x1070d0b => {
  { let v = cpu.r32(0x10c6872u32); cpu.push(v); }
  { let v = cpu.r16(0x10c6872u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r32(0x10e4114u32); cpu.push(v); }
  { let v: u32 = cpu.r16(0x10e3d85u32) as u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3d89u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10e4114u32, v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x10e4114u32), 4u32); cpu.w32(0x10e4114u32, v); }
  cpu.push(0x1070d3e); f_0107153d(cpu); if cpu.returned_to != 0x1070d3e { return; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x1070d44); cpu.call(t); if cpu.returned_to != 0x1070d44 { return; }
  { cpu.sub8(cpu.r8(0x10ee974u32), 0x0u8); }
  if cpu.zf { pc = 0x1070d52; continue; }
  pc = 0x1070d4d; continue;
 }
 0x1070d4d => {
  cpu.push(0x1070d52); f_0104ace1(cpu); if cpu.returned_to != 0x1070d52 { return; }
  pc = 0x1070d52; continue;
 }
 0x1070d52 => {
  cpu.push(0x1070d57); f_010715b4(cpu); if cpu.returned_to != 0x1070d57 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4114u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10c6872u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10c6872u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4100u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e40fcu32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e4108u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e4104u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3d9fu32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3d9du32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3d89u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10c0528u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3fd4u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3fd0u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x1128db2u32, v); }
  { cpu.popad(); }
  cpu.push(0x1070dba); f_0104f39e(cpu); if cpu.returned_to != 0x1070dba { return; }
  pc = 0x1070dba; continue;
 }
 0x1070dba => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01070dbb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1070dbb); }
 let mut pc: u32 = 0x1070dbb;
 loop { match pc {
 0x1070dbb => {
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x1070dde; continue; }
  pc = 0x1070dd1; continue;
 }
 0x1070dd1 => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1070dde; continue;
 }
 0x1070dde => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d5fu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3d61u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1070df3; continue; }
  pc = 0x1070df1; continue;
 }
 0x1070df1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1070df3; continue;
 }
 0x1070df3 => {
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1070dfa; continue; }
  pc = 0x1070df8; continue;
 }
 0x1070df8 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x1070dfa; continue;
 }
 0x1070dfa => {
  cpu.push(0x1070dff); f_0104f385(cpu); if cpu.returned_to != 0x1070dff { return; }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10ee948u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x1070e12); cpu.call(t); if cpu.returned_to != 0x1070e12 { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x10e3db7u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x1070e29; continue; }
  pc = 0x1070e20; continue;
 }
 0x1070e20 => {
  { let v: u16 = 0xad7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1070f55; continue;
 }
 0x1070e29 => {
  { cpu.sub16((cpu.ecx as u16), 0xad8u16); }
  if !cpu.zf { pc = 0x1070e39; continue; }
  pc = 0x1070e30; continue;
 }
 0x1070e30 => {
  { let v: u16 = 0xad8u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1070f55; continue;
 }
 0x1070e39 => {
  { cpu.sub16((cpu.ecx as u16), 0x4u16); }
  if !cpu.zf { pc = 0x1070e54; continue; }
  pc = 0x1070e3f; continue;
 }
 0x1070e3f => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xadbu32); cpu.ecx = v; }
  pc = 0x1070f55; continue;
 }
 0x1070e54 => {
  { cpu.sub16((cpu.ecx as u16), 0x2du16); }
  if !cpu.zf { pc = 0x1070e6f; continue; }
  pc = 0x1070e5a; continue;
 }
 0x1070e5a => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xabfu32); cpu.ecx = v; }
  pc = 0x1070f55; continue;
 }
 0x1070e6f => {
  { cpu.sub16((cpu.ecx as u16), 0x3u16); }
  if !cpu.zf { pc = 0x1070e8a; continue; }
  pc = 0x1070e75; continue;
 }
 0x1070e75 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xab7u32); cpu.ecx = v; }
  pc = 0x1070f55; continue;
 }
 0x1070e8a => {
  { cpu.sub16((cpu.ecx as u16), 0x2bu16); }
  if !cpu.zf { pc = 0x1070ea5; continue; }
  pc = 0x1070e90; continue;
 }
 0x1070e90 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xac7u32); cpu.ecx = v; }
  pc = 0x1070f55; continue;
 }
 0x1070ea5 => {
  { cpu.sub16((cpu.ecx as u16), 0x2cu16); }
  if !cpu.zf { pc = 0x1070ec0; continue; }
  pc = 0x1070eab; continue;
 }
 0x1070eab => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xacfu32); cpu.ecx = v; }
  pc = 0x1070f55; continue;
 }
 0x1070ec0 => {
  { cpu.sub16((cpu.ecx as u16), 0xau16); }
  if !cpu.zf { pc = 0x1070edb; continue; }
  pc = 0x1070ec6; continue;
 }
 0x1070ec6 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xb15u32); cpu.ecx = v; }
  pc = 0x1070f55; continue;
 }
 0x1070edb => {
  { cpu.sub16((cpu.ecx as u16), 0xae4u16); }
  if !cpu.zf { pc = 0x1070ef7; continue; }
  pc = 0x1070ee2; continue;
 }
 0x1070ee2 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xae4u32); cpu.ecx = v; }
  pc = 0x1070f55; continue;
 }
 0x1070ef7 => {
  { cpu.sub16((cpu.ecx as u16), 0xae8u16); }
  if !cpu.zf { pc = 0x1070f10; continue; }
  pc = 0x1070efe; continue;
 }
 0x1070efe => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xae8u32); cpu.ecx = v; }
  pc = 0x1070f55; continue;
 }
 0x1070f10 => {
  { cpu.sub16((cpu.ecx as u16), 0xaecu16); }
  if !cpu.zf { pc = 0x1070f29; continue; }
  pc = 0x1070f17; continue;
 }
 0x1070f17 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xaecu32); cpu.ecx = v; }
  pc = 0x1070f55; continue;
 }
 0x1070f29 => {
  { cpu.sub16((cpu.ecx as u16), 0x6u16); }
  if !cpu.zf { pc = 0x1070f41; continue; }
  pc = 0x1070f2f; continue;
 }
 0x1070f2f => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11c59eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xb0du32); cpu.ecx = v; }
  pc = 0x1070f55; continue;
 }
 0x1070f41 => {
  { let v: u16 = cpu.r16(0x10e3db7u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x824u16); }
  if !cpu.zf { pc = 0x1070f55; continue; }
  pc = 0x1070f4f; continue;
 }
 0x1070f4f => {
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1070f55; continue;
 }
 0x1070f55 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x11c59ebu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01070f5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1070f5c); }
 let mut pc: u32 = 0x1070f5c;
 loop { match pc {
 0x1070f5c => {
  { cpu.sub8(cpu.r8(0x10fd361u32), 0x0u8); }
  if !cpu.zf { pc = 0x1071055; continue; }
  pc = 0x1070f69; continue;
 }
 0x1070f69 => {
  { cpu.sub8(cpu.r8(0x10ee974u32), 0x0u8); }
  if cpu.zf { pc = 0x1071055; continue; }
  pc = 0x1070f76; continue;
 }
 0x1070f76 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11c59eau32)); cpu.w8(0x11c59eau32, v); }
  { let v: u8 = cpu.r8(0x11c59eau32) & 0x7u8; cpu.w8(0x11c59eau32, v); }
  { cpu.pushad(); }
  cpu.push(0x1070f89); f_01070dbb(cpu); if cpu.returned_to != 0x1070f89 { return; }
  { let v = cpu.r16(0x10e3d89u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e3d89u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r32(0x1121365u32); cpu.push(v); }
  { let v = cpu.r32(0x10e4114u32); cpu.push(v); }
  { let v: u32 = cpu.r16(0x10e3d85u32) as u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3d89u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10e4114u32, v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x10e4114u32), 4u32); cpu.w32(0x10e4114u32, v); }
  { let v: u32 = cpu.r32(0x1121369u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1121365u32, v); }
  { let v = cpu.r16(0x10e4104u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e4108u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x140u16; cpu.w16(0x10e4104u32, v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x10e4108u32, v); }
  { cpu.sub8(cpu.r8(0x11b51b9u32), 0x1u8); }
  if cpu.zf { pc = 0x107101b; continue; }
  pc = 0x1070ff3; continue;
 }
 0x1070ff3 => {
  { cpu.sub8(cpu.r8(0x10ef20eu32), 0x0u8); }
  if cpu.zf { pc = 0x107101b; continue; }
  pc = 0x1070ffc; continue;
 }
 0x1070ffc => {
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x107101b; continue; }
  pc = 0x1071005; continue;
 }
 0x1071005 => {
  { let v: u16 = 0x280u16; cpu.w16(0x10e4104u32, v); }
  { let v: u32 = cpu.r32(0x10e412cu32); cpu.esi = v; }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x10e4108u32, v); }
  pc = 0x107101b; continue;
 }
 0x107101b => {
  let t = cpu.r32(0x10e433cu32); cpu.push(0x1071021); cpu.call(t); if cpu.returned_to != 0x1071021 { return; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1071027); cpu.call(t); if cpu.returned_to != 0x1071027 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e4108u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e4104u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4114u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x1121365u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3d89u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3d89u32, v); }
  { cpu.popad(); }
  cpu.push(0x1071055); f_0104f39e(cpu); if cpu.returned_to != 0x1071055 { return; }
  pc = 0x1071055; continue;
 }
 0x1071055 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071056(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071056); }
 let mut pc: u32 = 0x1071056;
 loop { match pc {
 0x1071056 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x10fd361u32), 0x0u8); }
  if !cpu.zf { pc = 0x1071199; continue; }
  pc = 0x1071066; continue;
 }
 0x1071066 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u16 = 0x105u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3ceu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfdu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee976u32), 0x0u8); }
  if cpu.zf { pc = 0x10710de; continue; }
  pc = 0x107108c; continue;
 }
 0x107108c => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3da1u32)); }
  if !cpu.zf { pc = 0x107109e; continue; }
  pc = 0x1071095; continue;
 }
 0x1071095 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3da3u32)); }
  if cpu.zf { pc = 0x10710de; continue; }
  pc = 0x107109e; continue;
 }
 0x107109e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0xa8030u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e4000u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10ee977u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x10710b2; continue;
 }
 0x10710b2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x5u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x52u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10710b2; continue; }
  pc = 0x10710d6; continue;
 }
 0x10710d6 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  pc = 0x10710de; continue;
 }
 0x10710de => {
  { cpu.sub8(cpu.r8(0x10ee976u32), 0x0u8); }
  if cpu.zf { pc = 0x10710fd; continue; }
  pc = 0x10710e7; continue;
 }
 0x10710e7 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3da1u32)); }
  if !cpu.zf { pc = 0x10710fd; continue; }
  pc = 0x10710f0; continue;
 }
 0x10710f0 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3da3u32)); }
  if cpu.zf { pc = 0x1071199; continue; }
  pc = 0x10710fd; continue;
 }
 0x10710fd => {
  { cpu.sub8(cpu.r8(0x10ee974u32), 0x0u8); }
  if cpu.zf { pc = 0x1071199; continue; }
  pc = 0x107110a; continue;
 }
 0x107110a => {
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1071111; continue; }
  pc = 0x107110f; continue;
 }
 0x107110f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1071111; continue;
 }
 0x1071111 => {
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1071118; continue; }
  pc = 0x1071116; continue;
 }
 0x1071116 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x1071118; continue;
 }
 0x1071118 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3da1u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3da3u32, v); }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.esi = v; }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10c0208u32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10e3d85u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4000u32, v); }
  { let v: u32 = 0xa8030u32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0xb8u32); }
  if (cpu.sf != cpu.of) { pc = 0x1071168; continue; }
  pc = 0x107115c; continue;
 }
 0x107115c => {
  { let v: u32 = cpu.edx.wrapping_sub(0xb8u32); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if cpu.sf { pc = 0x1071199; continue; }
  pc = 0x1071166; continue;
 }
 0x1071166 => {
  if cpu.zf { pc = 0x1071199; continue; }
  pc = 0x1071168; continue;
 }
 0x1071168 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10ee977u32, v); }
  pc = 0x107116e; continue;
 }
 0x107116e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x52u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x5u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x107116e; continue; }
  pc = 0x1071192; continue;
 }
 0x1071192 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee976u32, v); }
  pc = 0x1071199; continue;
 }
 0x1071199 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107119d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107119d); }
 let mut pc: u32 = 0x107119d;
 loop { match pc {
 0x107119d => {
  { cpu.sub8(cpu.r8(0x10fd361u32), 0x0u8); }
  if !cpu.zf { pc = 0x10712b6; continue; }
  pc = 0x10711aa; continue;
 }
 0x10711aa => {
  { cpu.sub16(cpu.r16(0x11d6896u32), 0x1bu16); }
  if cpu.zf { pc = 0x10712b6; continue; }
  pc = 0x10711b8; continue;
 }
 0x10711b8 => {
  { cpu.pushad(); }
  cpu.push(0x10711be); f_01070dbb(cpu); if cpu.returned_to != 0x10711be { return; }
  { let v = cpu.r32(0x102cf46u32); cpu.push(v); }
  { let v = cpu.r32(0x102cf4au32); cpu.push(v); }
  { let v = cpu.r32(0x102cf4eu32); cpu.push(v); }
  { let v = cpu.r32(0x102cf52u32); cpu.push(v); }
  { let v = cpu.r32(0x102cf56u32); cpu.push(v); }
  { let v = cpu.r16(0x11d6652u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x11d6652u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x11d6652u32, v); }
  { let v = cpu.r32(0x1121365u32); cpu.push(v); }
  { let v: u32 = cpu.r32(0x1121369u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1121365u32, v); }
  { let v = cpu.r16(0x10e4104u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e4108u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r32(0x10e40fcu32); cpu.push(v); }
  { let v = cpu.r32(0x10e4100u32); cpu.push(v); }
  { let v: u16 = 0x280u16; cpu.w16(0x10e4104u32, v); }
  { let v: u32 = cpu.r32(0x10e412cu32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e4108u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e40fcu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4100u32, v); }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x107124f); cpu.call(t); if cpu.returned_to != 0x107124f { return; }
  { cpu.sub8(cpu.r8(0x10ee974u32), 0x0u8); }
  if cpu.zf { pc = 0x107125e; continue; }
  pc = 0x1071258; continue;
 }
 0x1071258 => {
  let t = cpu.r32(0x10e42f0u32); cpu.push(0x107125e); cpu.call(t); if cpu.returned_to != 0x107125e { return; }
  pc = 0x107125e; continue;
 }
 0x107125e => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4100u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e40fcu32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e4108u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e4104u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x1121365u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x11d6652u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.push(0x1071292); f_01027bb1(cpu); if cpu.returned_to != 0x1071292 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x102cf56u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x102cf52u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x102cf4eu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x102cf4au32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x102cf46u32, v); }
  { cpu.popad(); }
  cpu.push(0x10712b6); f_0104f39e(cpu); if cpu.returned_to != 0x10712b6 { return; }
  pc = 0x10712b6; continue;
 }
 0x10712b6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010712b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10712b7); }
 let mut pc: u32 = 0x10712b7;
 loop { match pc {
 0x10712b7 => {
  { cpu.sub8(cpu.r8(0x10fd361u32), 0x0u8); }
  if !cpu.zf { pc = 0x107140a; continue; }
  pc = 0x10712c4; continue;
 }
 0x10712c4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.r32(0x1128db8u32); cpu.push(v); }
  { let v = cpu.r32(0x1128dbcu32); cpu.push(v); }
  { let v = cpu.r32(0x1128dc0u32); cpu.push(v); }
  { let v = cpu.r32(0x1128dc4u32); cpu.push(v); }
  { let v = cpu.r32(0x1128dc8u32); cpu.push(v); }
  { let v = cpu.r32(0x10e4104u32); cpu.push(v); }
  { let v = cpu.r32(0x10e4108u32); cpu.push(v); }
  { let v: u16 = 0x280u16; cpu.w16(0x10e4104u32, v); }
  { let v: u32 = cpu.r32(0x10e412cu32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e4108u32, v); }
  { cpu.sub8(cpu.r8(0x10ee976u32), 0x0u8); }
  if cpu.zf { pc = 0x107137f; continue; }
  pc = 0x1071310; continue;
 }
 0x1071310 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3da1u32)); }
  if !cpu.zf { pc = 0x1071322; continue; }
  pc = 0x1071319; continue;
 }
 0x1071319 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3da3u32)); }
  if cpu.zf { pc = 0x107137f; continue; }
  pc = 0x1071322; continue;
 }
 0x1071322 => {
  { cpu.pushad(); }
  { let v = cpu.r32(0x1121365u32); cpu.push(v); }
  { let v: u32 = cpu.r32(0x11c59cau32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1121365u32, v); }
  { let v: u32 = cpu.r16(0x10e3da1u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e3da3u32) as u32; cpu.ebx = v; }
  { let v = cpu.r16(0x118cd66u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x118cd66u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x1d2au16; cpu.w16(0x118cd66u32, v); }
  { let v: u32 = 0x1d26u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1071363); cpu.call(t); if cpu.returned_to != 0x1071363 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x118cd66u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x118cd66u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x1121365u32, v); }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  pc = 0x107137f; continue;
 }
 0x107137f => {
  { cpu.sub8(cpu.r8(0x10ee976u32), 0x0u8); }
  if cpu.zf { pc = 0x107139a; continue; }
  pc = 0x1071388; continue;
 }
 0x1071388 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3da1u32)); }
  if !cpu.zf { pc = 0x107139a; continue; }
  pc = 0x1071391; continue;
 }
 0x1071391 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3da3u32)); }
  if cpu.zf { pc = 0x10713dd; continue; }
  pc = 0x107139a; continue;
 }
 0x107139a => {
  { cpu.sub8(cpu.r8(0x10ee974u32), 0x0u8); }
  if cpu.zf { pc = 0x10713dd; continue; }
  pc = 0x10713a3; continue;
 }
 0x10713a3 => {
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x10713aa; continue; }
  pc = 0x10713a8; continue;
 }
 0x10713a8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10713aa; continue;
 }
 0x10713aa => {
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x10713b1; continue; }
  pc = 0x10713af; continue;
 }
 0x10713af => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10713b1; continue;
 }
 0x10713b1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3da1u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3da3u32, v); }
  { let v: u32 = cpu.r32(0x1194200u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x12u32); cpu.esi = v; }
  cpu.push(0x10713cc); f_0104b4d4(cpu); if cpu.returned_to != 0x10713cc { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee976u32, v); }
  { let v: u32 = cpu.r32(0x1121365u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11c59cau32, v); }
  pc = 0x10713dd; continue;
 }
 0x10713dd => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4108u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4104u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x1128dc8u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x1128dc4u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x1128dc0u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x1128dbcu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x1128db8u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x107140a; continue;
 }
 0x107140a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107140b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107140b); }
 let mut pc: u32 = 0x107140b;
 loop { match pc {
 0x107140b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107140c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107140c); }
 let mut pc: u32 = 0x107140c;
 loop { match pc {
 0x107140c => {
  { cpu.sub16(cpu.r16(0x10e3d3fu32), 0x0u16); }
  if !cpu.zf { pc = 0x107144f; continue; }
  pc = 0x1071416; continue;
 }
 0x1071416 => {
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e402cu32, v); }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0xa8080u32; cpu.edi = v; }
  cpu.push(0x1071439); f_0107161e(cpu); if cpu.returned_to != 0x1071439 { return; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xf02u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = 0xb8u32; cpu.ecx = v; }
  pc = 0x1071448; continue;
 }
 0x1071448 => {
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.add32(cpu.esi, 0x50u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1071448; continue; }
  pc = 0x107144f; continue;
 }
 0x107144f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071450(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071450); }
 let mut pc: u32 = 0x1071450;
 loop { match pc {
 0x1071450 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071451(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071451); }
 let mut pc: u32 = 0x1071451;
 loop { match pc {
 0x1071451 => {
  { cpu.sub16(cpu.r16(0x10e3d3fu32), 0x0u16); }
  if !cpu.zf { pc = 0x10714a4; continue; }
  pc = 0x107145b; continue;
 }
 0x107145b => {
  { cpu.sub8(cpu.r8(0x10ee97au32), 0x0u8); }
  if cpu.zf { pc = 0x10714a4; continue; }
  pc = 0x1071464; continue;
 }
 0x1071464 => {
  { cpu.sub16(cpu.r16(0x10e3dbfu32), 0x0u16); }
  if !cpu.zf { pc = 0x10714a4; continue; }
  pc = 0x107146e; continue;
 }
 0x107146e => {
  { let v: u32 = cpu.r32(0x10e402cu32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x10714a4; continue; }
  pc = 0x1071478; continue;
 }
 0x1071478 => {
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xa8080u32; cpu.esi = v; }
  cpu.push(0x107148e); f_0107161e(cpu); if cpu.returned_to != 0x107148e { return; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xf02u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = 0xb8u32; cpu.ecx = v; }
  pc = 0x107149d; continue;
 }
 0x107149d => {
  { cpu.movs(1, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0x50u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x107149d; continue; }
  pc = 0x10714a4; continue;
 }
 0x10714a4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010714a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10714a5); }
 let mut pc: u32 = 0x10714a5;
 loop { match pc {
 0x10714a5 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x10714ab; continue;
 }
 0x10714ab => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { cpu.poll(); pc = 0x10714ab; continue; }
  pc = 0x10714b0; continue;
 }
 0x10714b0 => {
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x10714b4; continue;
 }
 0x10714b4 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10714b4; continue; }
  pc = 0x10714b9; continue;
 }
 0x10714b9 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010714bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10714bc); }
 let mut pc: u32 = 0x10714bc;
 loop { match pc {
 0x10714bc => {
  { cpu.sub8(cpu.r8(0x11c59ceu32), 0x0u8); }
  if !cpu.zf { pc = 0x1071512; continue; }
  pc = 0x10714c5; continue;
 }
 0x10714c5 => {
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10714d4; continue; }
  pc = 0x10714ce; continue;
 }
 0x10714ce => {
  cpu.push(0x10714d3); f_010721f4(cpu); if cpu.returned_to != 0x10714d3 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10714d4 => {
  cpu.push(0x10714d9); f_01033a4b(cpu); if cpu.returned_to != 0x10714d9 { return; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  cpu.push(0x10714e2); f_01033a3b(cpu); if cpu.returned_to != 0x10714e2 { return; }
  { let v: u32 = 0x10e4004u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  cpu.push(0x10714f1); f_0105e746(cpu); if cpu.returned_to != 0x10714f1 { return; }
  { let v: u32 = cpu.r16(0x10e4004u32) as u32; cpu.edx = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  cpu.push(0x1071503); f_01033a3b(cpu); if cpu.returned_to != 0x1071503 { return; }
  { let v: u32 = 0x300u32; cpu.ecx = v; }
  { let v: u32 = 0x11a37f0u32; cpu.edx = v; }
  cpu.push(0x1071512); f_0105e746(cpu); if cpu.returned_to != 0x1071512 { return; }
  pc = 0x1071512; continue;
 }
 0x1071512 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071513(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071513); }
 let mut pc: u32 = 0x1071513;
 loop { match pc {
 0x1071513 => {
  { let v = (cpu.edx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3ceu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xf0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u16 = 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u16 = 0xff08u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107153d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107153d); }
 let mut pc: u32 = 0x107153d;
 loop { match pc {
 0x107153d => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x3ceu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee96fu32, v); }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee970u32, v); }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee971u32, v); }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee972u32, v); }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee975u32, v); }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee973u32, v); }
  { let v: u16 = 0x3ceu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = 0xa0000u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010715b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10715b4); }
 let mut pc: u32 = 0x10715b4;
 loop { match pc {
 0x10715b4 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x3ceu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xf0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = 0xa0000u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = 0x3ceu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10ee96fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10ee970u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10ee971u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10ee972u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10ee975u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10ee973u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107161e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107161e); }
 let mut pc: u32 = 0x107161e;
 loop { match pc {
 0x107161e => {
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = 0x3ceu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = cpu.inc16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107162e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107162e); }
 let mut pc: u32 = 0x107162e;
 loop { match pc {
 0x107162e => {
  { let v: u32 = cpu.r16(0x10e3d85u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  let t = cpu.r32(0x10e4344u32); cpu.push(0x1071648); cpu.call(t); if cpu.returned_to != 0x1071648 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071649(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071649); }
 let mut pc: u32 = 0x1071649;
 loop { match pc {
 0x1071649 => {
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  cpu.push(0x1071658); f_0107161e(cpu); if cpu.returned_to != 0x1071658 { return; }
  { let v: u32 = 0x4010u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071660); }
 let mut pc: u32 = 0x1071660;
 loop { match pc {
 0x1071660 => {
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  cpu.push(0x107166f); f_0107161e(cpu); if cpu.returned_to != 0x107166f { return; }
  { let v: u16 = 0xc8u16; cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  pc = 0x1071673; continue;
 }
 0x1071673 => {
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.add32(cpu.esi, 0x2u32); cpu.esi = v; }
  { let v: u16 = cpu.dec16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1071673; continue; }
  pc = 0x1071681; continue;
 }
 0x1071681 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010716a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10716a4); }
 let mut pc: u32 = 0x10716a4;
 loop { match pc {
 0x10716a4 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10ee974u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  cpu.push(0x10716b8); f_0102c416(cpu); if cpu.returned_to != 0x10716b8 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.interrupts_off = true; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee974u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0xfffffd44u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x10716ce); cpu.call(t); if cpu.returned_to != 0x10716ce { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  let t = cpu.r32(0x10e4360u32); cpu.push(0x10716db); cpu.call(t); if cpu.returned_to != 0x10716db { return; }
  { cpu.interrupts_off = false; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10716dd; continue;
 }
 0x10716dd => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x10716e3; continue; }
  pc = 0x10716e1; continue;
 }
 0x10716e1 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10716e3; continue;
 }
 0x10716e3 => {
  { let v: u32 = cpu.r32(0x112a5e2u32); cpu.edx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11c62f9u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c62f9u32, v); }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee96au32)); }
  if !cpu.cf { cpu.poll(); pc = 0x10716dd; continue; }
  pc = 0x10716ff; continue;
 }
 0x10716ff => {
  { let v: u8 = cpu.r8(0x10ee96au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee96du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee96au32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071804(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071804); }
 let mut pc: u32 = 0x1071804;
 loop { match pc {
 0x1071804 => {
  { let v: u16 = cpu.r16(0x10e3d89u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r16(0x10e3d85u32) as u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d85u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3d89u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10e4114u32, v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x10e4114u32), 4u32); cpu.w32(0x10e4114u32, v); }
  { let v: u32 = cpu.eax & 0xff0u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3d4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0xcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u8 = cpu.r8(0x10e3d41u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x18u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 3u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.r16(0x10e3d85u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0xfu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(((cpu.eax >> 8) as u8), (cpu.ecx as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0xdu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107185e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107185e); }
 let mut pc: u32 = 0x107185e;
 loop { match pc {
 0x107185e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  let t = cpu.r32(0x10e436cu32); cpu.push(0x1071872); cpu.call(t); if cpu.returned_to != 0x1071872 { return; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x3030303u32; cpu.eax = v; }
  { let v: u32 = 0x1004u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071890(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071890); }
 let mut pc: u32 = 0x1071890;
 loop { match pc {
 0x1071890 => {
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1071a2f; continue; }
  pc = 0x107189d; continue;
 }
 0x107189d => {
  { let v: u32 = cpu.r32(0x10e4128u32); cpu.eax = v; }
  { let v: u32 = 0x1e0u32; cpu.ebp = v; }
  { let v: u32 = 0x112c72cu32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x10e412cu32), 0x1e0u32); }
  if cpu.zf { pc = 0x10718e9; continue; }
  pc = 0x10718b8; continue;
 }
 0x10718b8 => {
  { let v: u32 = 0x190u32; cpu.ebp = v; }
  { let v: u32 = 0x112c76au32; cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))), 0x0u16); }
  if !cpu.zf { pc = 0x107190d; continue; }
  pc = 0x10718c9; continue;
 }
 0x10718c9 => {
  { let v: u32 = 0x1e0u32; cpu.ebp = v; }
  { let v: u32 = 0x112c72cu32; cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))), 0x0u16); }
  if !cpu.zf { pc = 0x107190d; continue; }
  pc = 0x10718da; continue;
 }
 0x10718da => {
  { let v: u32 = 0x1bu32; cpu.w32(0x10e4128u32, v); }
  pc = 0x1071963; continue;
 }
 0x10718e9 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))), 0x0u16); }
  if !cpu.zf { pc = 0x107190d; continue; }
  pc = 0x10718f0; continue;
 }
 0x10718f0 => {
  { let v: u32 = 0x190u32; cpu.ebp = v; }
  { let v: u32 = 0x112c76au32; cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))), 0x0u16); }
  if !cpu.zf { pc = 0x107190d; continue; }
  pc = 0x1071901; continue;
 }
 0x1071901 => {
  { let v: u32 = 0x1bu32; cpu.w32(0x10e4128u32, v); }
  pc = 0x1071963; continue;
 }
 0x107190d => {
  { let v = cpu.r32(0x10e412cu32); cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e412cu32, v); }
  { let v: u32 = cpu.r32(0x10e4128u32); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11d63f4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x400u16; cpu.w16(0x11d665au32, v); }
  { let v: u16 = 0x3d4u16; cpu.w16(0x11d664cu32, v); }
  cpu.push(0x107193c); f_0102b2bc(cpu); if cpu.returned_to != 0x107193c { return; }
  { cpu.sub8(cpu.r8(0x10e4128u32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1071962; continue; }
  pc = 0x1071944; continue;
 }
 0x1071944 => {
  { let v: u32 = cpu.r32(0x10e4128u32); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11d63f4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e412cu32, v); }
  { let v: u32 = 0x1bu32; cpu.w32(0x10e4128u32, v); }
  pc = 0x1071963; continue;
 }
 0x1071962 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1071963; continue;
 }
 0x1071963 => {
  { let v: u32 = cpu.r32(0x10e4128u32); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11d63f4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x400u16; cpu.w16(0x11d665au32, v); }
  { let v: u16 = 0x3d4u16; cpu.w16(0x11d664cu32, v); }
  { cpu.sub32(cpu.r32(0x10e4128u32), 0x1bu32); }
  if !cpu.zf { pc = 0x1071a2f; continue; }
  pc = 0x107198e; continue;
 }
 0x107198e => {
  cpu.push(0x1071993); f_0102808e(cpu); if cpu.returned_to != 0x1071993 { return; }
  { let v: u16 = 0x1bu16; cpu.w16(0x11d6896u32, v); }
  cpu.push(0x10719a1); f_0102a8cf(cpu); if cpu.returned_to != 0x10719a1 { return; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1071a19; continue; }
  pc = 0x10719a9; continue;
 }
 0x10719a9 => {
  { let v: u32 = 0x112c72cu32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x10e412cu32), 0x1e0u32); }
  if cpu.zf { pc = 0x10719ee; continue; }
  pc = 0x10719ba; continue;
 }
 0x10719ba => {
  { let v: u32 = 0x112c76au32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x36u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  cpu.push(0x10719c8); f_01028255(cpu); if cpu.returned_to != 0x10719c8 { return; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1071a2f; continue; }
  pc = 0x10719d0; continue;
 }
 0x10719d0 => {
  { let v: u32 = 0x1e0u32; cpu.w32(0x10e412cu32, v); }
  { let v: u32 = 0x112c72cu32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x36u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  cpu.push(0x10719e8); f_01028255(cpu); if cpu.returned_to != 0x10719e8 { return; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1071a2f; continue; }
  pc = 0x10719ec; continue;
 }
 0x10719ec => {
  pc = 0x1071a19; continue;
 }
 0x10719ee => {
  { let v: u32 = cpu.r16(0x36u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  cpu.push(0x10719f7); f_01028255(cpu); if cpu.returned_to != 0x10719f7 { return; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1071a2f; continue; }
  pc = 0x10719fb; continue;
 }
 0x10719fb => {
  { let v: u32 = 0x190u32; cpu.w32(0x10e412cu32, v); }
  { let v: u32 = 0x112c76au32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x36u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  cpu.push(0x1071a13); f_01028255(cpu); if cpu.returned_to != 0x1071a13 { return; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1071a2f; continue; }
  pc = 0x1071a17; continue;
 }
 0x1071a17 => {
  pc = 0x1071a19; continue;
 }
 0x1071a19 => {
  { let v: u32 = cpu.r32(0x1131050u32); cpu.edx = v; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  cpu.push(0x1071a28); f_0105eaf1(cpu); if cpu.returned_to != 0x1071a28 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef352u32, v); }
  pc = 0x1071a2f; continue;
 }
 0x1071a2f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071a30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071a30); }
 let mut pc: u32 = 0x1071a30;
 loop { match pc {
 0x1071a30 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5f4bu32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1071b1f; continue; }
  pc = 0x1071a44; continue;
 }
 0x1071a44 => {
  { let v: u8 = 0x2u8; cpu.w8(0x10ef352u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1121348u32, v); }
  { let v: u32 = 0x112c72cu32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x10e412cu32), 0x1e0u32); }
  if cpu.zf { pc = 0x1071a6f; continue; }
  pc = 0x1071a63; continue;
 }
 0x1071a63 => {
  { let v: u32 = 0x112c76au32; cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x1121348u32, v); }
  pc = 0x1071a6f; continue;
 }
 0x1071a6f => {
  { let v: u32 = cpu.r32(0x10e4128u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e4128u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x11d6896u32, v); }
  cpu.push(0x1071a8a); f_01028461(cpu); if cpu.returned_to != 0x1071a8a { return; }
  cpu.push(0x1071a8f); f_0102c544(cpu); if cpu.returned_to != 0x1071a8f { return; }
  { let v: u16 = cpu.r16(0x10e4104u32).wrapping_sub(0x60u16); cpu.w16(0x10e4104u32, v); }
  { let v: u16 = cpu.r16(0x10e4108u32).wrapping_sub(0x20u16); cpu.w16(0x10e4108u32, v); }
  { let v: u32 = cpu.r32(0x1121355u32).wrapping_sub(0x20u32); cpu.w32(0x1121355u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e417cu32, v); }
  { let v: u32 = 0x1du32; cpu.w32(0x10e4180u32, v); }
  { let v: u32 = cpu.r32(0x1121351u32).wrapping_sub(0x60u32); cpu.w32(0x1121351u32, v); }
  { let v: u32 = cpu.r32(0x1121359u32).wrapping_sub(0x6u32); cpu.w32(0x1121359u32, v); }
  cpu.push(0x1071acd); f_0105ec4c(cpu); if cpu.returned_to != 0x1071acd { return; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1121355u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xfu32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x1071ae7); f_0105f339(cpu); if cpu.returned_to != 0x1071ae7 { return; }
  cpu.push(0x1071aec); f_01072005(cpu); if cpu.returned_to != 0x1071aec { return; }
  { let v: u32 = cpu.r32(0x11d4894u32); cpu.ecx = v; }
  cpu.push(0x1071af7); f_0105e821(cpu); if cpu.returned_to != 0x1071af7 { return; }
  { let v: u32 = 0x68u32; cpu.w32(0x11296ccu32, v); }
  { let v: u32 = 0x124u32; cpu.w32(0x11296d0u32, v); }
  { let v: u32 = 0x56u32; cpu.w32(0x11296d4u32, v); }
  { let v: u32 = 0x224u32; cpu.w32(0x11296d8u32, v); }
  pc = 0x1071b1f; continue;
 }
 0x1071b1f => {
  cpu.push(0x1071b24); f_010724a7(cpu); if cpu.returned_to != 0x1071b24 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071b25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071b25); }
 let mut pc: u32 = 0x1071b25;
 loop { match pc {
 0x1071b25 => {
  { cpu.sub8(cpu.r8(0x110576bu32), 0x0u8); }
  if !cpu.zf { pc = 0x1071b39; continue; }
  pc = 0x1071b2e; continue;
 }
 0x1071b2e => {
  { cpu.sub8(cpu.r8(0x10fd360u32), 0x0u8); }
  if cpu.sf { pc = 0x1071b39; continue; }
  pc = 0x1071b37; continue;
 }
 0x1071b37 => {
  if !cpu.zf { pc = 0x1071b74; continue; }
  pc = 0x1071b39; continue;
 }
 0x1071b39 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd566u32, v); }
  { let v: u32 = 0x118cd68u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x118cd66u32) as u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1071b74; continue; }
  pc = 0x1071b50; continue;
 }
 0x1071b50 => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x1071b55; continue;
 }
 0x1071b55 => {
  { cpu.lods(4, false); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1071b6a; continue; }
  pc = 0x1071b5b; continue;
 }
 0x1071b5b => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x10e3e53u32, v); }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  let t = cpu.r32(0x10e4370u32); cpu.push(0x1071b69); cpu.call(t); if cpu.returned_to != 0x1071b69 { return; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x1071b6a; continue;
 }
 0x1071b6a => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1071b55; continue; }
  pc = 0x1071b6d; continue;
 }
 0x1071b6d => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef354u32, v); }
  pc = 0x1071b74; continue;
 }
 0x1071b74 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071b75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071b75); }
 let mut pc: u32 = 0x1071b75;
 loop { match pc {
 0x1071b75 => {
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r16(0x118cd66u32) as u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1071bb9; continue; }
  pc = 0x1071b83; continue;
 }
 0x1071b83 => {
  { let v: u32 = 0x118cd68u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e4058u32); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  pc = 0x1071b93; continue;
 }
 0x1071b93 => {
  { cpu.lods(4, false); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x1071bae; continue; }
  pc = 0x1071b98; continue;
 }
 0x1071b98 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1071bae; continue; }
  pc = 0x1071b9d; continue;
 }
 0x1071b9d => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e53u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  let t = cpu.r32(0x10e4370u32); cpu.push(0x1071bac); cpu.call(t); if cpu.returned_to != 0x1071bac { return; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x1071bae; continue;
 }
 0x1071bae => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1071b93; continue; }
  pc = 0x1071bb2; continue;
 }
 0x1071bb2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef354u32, v); }
  pc = 0x1071bb9; continue;
 }
 0x1071bb9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071bba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071bba); }
 let mut pc: u32 = 0x1071bba;
 loop { match pc {
 0x1071bba => {
  { cpu.sub8(cpu.r8(0x110576bu32), 0x0u8); }
  if !cpu.zf { pc = 0x1071bce; continue; }
  pc = 0x1071bc3; continue;
 }
 0x1071bc3 => {
  { cpu.sub8(cpu.r8(0x10fd360u32), 0x0u8); }
  if cpu.sf { pc = 0x1071bce; continue; }
  pc = 0x1071bcc; continue;
 }
 0x1071bcc => {
  if !cpu.zf { pc = 0x1071c10; continue; }
  pc = 0x1071bce; continue;
 }
 0x1071bce => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd566u32, v); }
  { let v: u32 = 0x118cd68u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x118cd66u32) as u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1071c10; continue; }
  pc = 0x1071be5; continue;
 }
 0x1071be5 => {
  { let v: u32 = cpu.r32(0x10e4058u32); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  pc = 0x1071bf0; continue;
 }
 0x1071bf0 => {
  { cpu.lods(4, false); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1071c05; continue; }
  pc = 0x1071bf6; continue;
 }
 0x1071bf6 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e53u32, v); }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  let t = cpu.r32(0x10e4388u32); cpu.push(0x1071c04); cpu.call(t); if cpu.returned_to != 0x1071c04 { return; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x1071c05; continue;
 }
 0x1071c05 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1071bf0; continue; }
  pc = 0x1071c09; continue;
 }
 0x1071c09 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef354u32, v); }
  pc = 0x1071c10; continue;
 }
 0x1071c10 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071c11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071c11); }
 let mut pc: u32 = 0x1071c11;
 loop { match pc {
 0x1071c11 => {
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r16(0x118cd66u32) as u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1071c53; continue; }
  pc = 0x1071c1f; continue;
 }
 0x1071c1f => {
  { let v: u32 = 0x118cd68u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e4058u32); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  pc = 0x1071c2f; continue;
 }
 0x1071c2f => {
  { cpu.lods(4, false); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x1071c48; continue; }
  pc = 0x1071c34; continue;
 }
 0x1071c34 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1071c48; continue; }
  pc = 0x1071c39; continue;
 }
 0x1071c39 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e53u32, v); }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  let t = cpu.r32(0x10e4388u32); cpu.push(0x1071c47); cpu.call(t); if cpu.returned_to != 0x1071c47 { return; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x1071c48; continue;
 }
 0x1071c48 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1071c2f; continue; }
  pc = 0x1071c4c; continue;
 }
 0x1071c4c => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef354u32, v); }
  pc = 0x1071c53; continue;
 }
 0x1071c53 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071c54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071c54); }
 let mut pc: u32 = 0x1071c54;
 loop { match pc {
 0x1071c54 => {
  { let v: u32 = 0x118cd68u32; cpu.edi = v; }
  { let v: u32 = cpu.r16(0x118cd66u32) as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x1071c83; continue; }
  pc = 0x1071c65; continue;
 }
 0x1071c65 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  pc = 0x1071c6f; continue;
 }
 0x1071c6f => {
  { cpu.stos(4, false); }
  { cpu.sub8(cpu.r8(0x11cccf0u32.wrapping_add(cpu.edx)), 0xffu8); }
  if cpu.zf { pc = 0x1071c80; continue; }
  pc = 0x1071c79; continue;
 }
 0x1071c79 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11cccf0u32.wrapping_add(cpu.edx), v); }
  pc = 0x1071c80; continue;
 }
 0x1071c80 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1071c6f; continue; }
  pc = 0x1071c83; continue;
 }
 0x1071c83 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071c84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071c84); }
 let mut pc: u32 = 0x1071c84;
 loop { match pc {
 0x1071c84 => {
  { cpu.sub8(cpu.r8(0x1105e6au32), 0x0u8); }
  if !cpu.zf { pc = 0x1071c8e; continue; }
  pc = 0x1071c8d; continue;
 }
 0x1071c8d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1071c8e => {
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x1071d65; continue; }
  pc = 0x1071c9b; continue;
 }
 0x1071c9b => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b05u32, v); }
  { cpu.sub8(cpu.r8(0x10e3e93u32), 0x0u8); }
  if !cpu.zf { pc = 0x1071cb5; continue; }
  pc = 0x1071cab; continue;
 }
 0x1071cab => {
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x1071cb5); f_010725e4(cpu); if cpu.returned_to != 0x1071cb5 { return; }
  pc = 0x1071cb5; continue;
 }
 0x1071cb5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1071cc7); f_010385ad(cpu); if cpu.returned_to != 0x1071cc7 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x1071ccf); cpu.call(t); if cpu.returned_to != 0x1071ccf { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef352u32, v); }
  { let v: u16 = 0x110u16; cpu.w16(0x10e4104u32, v); }
  { let v: u16 = 0xb8u16; cpu.w16(0x10e4108u32, v); }
  cpu.push(0x1071cf4); f_0100abd9(cpu); if cpu.returned_to != 0x1071cf4 { return; }
  let t = cpu.r32(0x10e434cu32); cpu.push(0x1071cfa); cpu.call(t); if cpu.returned_to != 0x1071cfa { return; }
  cpu.push(0x1071cff); f_01071f2b(cpu); if cpu.returned_to != 0x1071cff { return; }
  cpu.push(0x1071d04); f_0105ec4c(cpu); if cpu.returned_to != 0x1071d04 { return; }
  { cpu.sub8(cpu.r8(0x10e3e93u32), 0x0u8); }
  if !cpu.zf { pc = 0x1071d4b; continue; }
  pc = 0x1071d0d; continue;
 }
 0x1071d0d => {
  let t = cpu.r32(0x10e4334u32); cpu.push(0x1071d13); cpu.call(t); if cpu.returned_to != 0x1071d13 { return; }
  cpu.push(0x1071d18); f_0101d70e(cpu); if cpu.returned_to != 0x1071d18 { return; }
  cpu.push(0x1071d1d); f_01025678(cpu); if cpu.returned_to != 0x1071d1d { return; }
  cpu.push(0x1071d22); f_010256a1(cpu); if cpu.returned_to != 0x1071d22 { return; }
  cpu.push(0x1071d27); f_0102581c(cpu); if cpu.returned_to != 0x1071d27 { return; }
  cpu.push(0x1071d2c); f_010268b8(cpu); if cpu.returned_to != 0x1071d2c { return; }
  cpu.push(0x1071d31); f_01059ab7(cpu); if cpu.returned_to != 0x1071d31 { return; }
  cpu.push(0x1071d36); f_01071d66(cpu); if cpu.returned_to != 0x1071d36 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b05u32, v); }
  { let v: u32 = 0x11a3af0u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1071d4b); f_0107259c(cpu); if cpu.returned_to != 0x1071d4b { return; }
  pc = 0x1071d4b; continue;
 }
 0x1071d4b => {
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e93u32, v); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1071d65; continue; }
  pc = 0x1071d5b; continue;
 }
 0x1071d5b => {
  { let v: u8 = cpu.r8(0x11c5b21u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef36du32, v); }
  pc = 0x1071d65; continue;
 }
 0x1071d65 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071d66(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071d66); }
 let mut pc: u32 = 0x1071d66;
 loop { match pc {
 0x1071d66 => {
  { let v: u32 = cpu.r32(0x10e3f60u32).wrapping_add(0xa0u32); cpu.w32(0x10e3f60u32, v); }
  { let v: u32 = cpu.r32(0x10e3f60u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x140u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3d17u32)); }
  if cpu.cf { pc = 0x1071d93; continue; }
  pc = 0x1071d83; continue;
 }
 0x1071d83 => {
  { let v: u16 = cpu.r16(0x10e3d17u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x140u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f60u32, v); }
  pc = 0x1071d93; continue;
 }
 0x1071d93 => {
  { let v: u32 = cpu.r32(0x10e3f64u32).wrapping_add(0x64u32); cpu.w32(0x10e3f64u32, v); }
  { let v: u32 = cpu.r32(0x10e3f64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xc8u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3d19u32)); }
  if cpu.cf { pc = 0x1071dbd; continue; }
  pc = 0x1071dad; continue;
 }
 0x1071dad => {
  { let v: u16 = cpu.r16(0x10e3d19u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, 0xc8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f64u32, v); }
  pc = 0x1071dbd; continue;
 }
 0x1071dbd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071dbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071dbe); }
 let mut pc: u32 = 0x1071dbe;
 loop { match pc {
 0x1071dbe => {
  { cpu.sub8(cpu.r8(0x1105e6au32), 0x0u8); }
  if !cpu.zf { pc = 0x1071dc8; continue; }
  pc = 0x1071dc7; continue;
 }
 0x1071dc7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1071dc8 => {
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if !cpu.zf { pc = 0x1071eb9; continue; }
  pc = 0x1071dd5; continue;
 }
 0x1071dd5 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5b05u32, v); }
  { cpu.sub8(cpu.r8(0x10e3e93u32), 0x0u8); }
  if !cpu.zf { pc = 0x1071def; continue; }
  pc = 0x1071de5; continue;
 }
 0x1071de5 => {
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x1071def); f_010725e4(cpu); if cpu.returned_to != 0x1071def { return; }
  pc = 0x1071def; continue;
 }
 0x1071def => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1071e01); f_010385ad(cpu); if cpu.returned_to != 0x1071e01 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x1071e09); cpu.call(t); if cpu.returned_to != 0x1071e09 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef352u32, v); }
  { let v: u16 = 0x250u16; cpu.w16(0x10e4104u32, v); }
  { let v: u16 = 0x180u16; cpu.w16(0x10e4108u32, v); }
  cpu.push(0x1071e2e); f_0100abf0(cpu); if cpu.returned_to != 0x1071e2e { return; }
  let t = cpu.r32(0x10e434cu32); cpu.push(0x1071e34); cpu.call(t); if cpu.returned_to != 0x1071e34 { return; }
  cpu.push(0x1071e39); f_01072005(cpu); if cpu.returned_to != 0x1071e39 { return; }
  cpu.push(0x1071e3e); f_0105ec4c(cpu); if cpu.returned_to != 0x1071e3e { return; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e412cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x1071e58); f_0105f339(cpu); if cpu.returned_to != 0x1071e58 { return; }
  { cpu.sub8(cpu.r8(0x10e3e93u32), 0x0u8); }
  if !cpu.zf { pc = 0x1071e9f; continue; }
  pc = 0x1071e61; continue;
 }
 0x1071e61 => {
  let t = cpu.r32(0x10e4334u32); cpu.push(0x1071e67); cpu.call(t); if cpu.returned_to != 0x1071e67 { return; }
  cpu.push(0x1071e6c); f_0101d70e(cpu); if cpu.returned_to != 0x1071e6c { return; }
  cpu.push(0x1071e71); f_01025678(cpu); if cpu.returned_to != 0x1071e71 { return; }
  cpu.push(0x1071e76); f_010256a1(cpu); if cpu.returned_to != 0x1071e76 { return; }
  cpu.push(0x1071e7b); f_0102581c(cpu); if cpu.returned_to != 0x1071e7b { return; }
  cpu.push(0x1071e80); f_010268b8(cpu); if cpu.returned_to != 0x1071e80 { return; }
  cpu.push(0x1071e85); f_01059ab7(cpu); if cpu.returned_to != 0x1071e85 { return; }
  cpu.push(0x1071e8a); f_01071eba(cpu); if cpu.returned_to != 0x1071e8a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5b05u32, v); }
  { let v: u32 = 0x11a3af0u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1071e9f); f_0107259c(cpu); if cpu.returned_to != 0x1071e9f { return; }
  pc = 0x1071e9f; continue;
 }
 0x1071e9f => {
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e93u32, v); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1071eb9; continue; }
  pc = 0x1071eaf; continue;
 }
 0x1071eaf => {
  { let v: u8 = cpu.r8(0x11c5b22u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef36du32, v); }
  pc = 0x1071eb9; continue;
 }
 0x1071eb9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071eba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071eba); }
 let mut pc: u32 = 0x1071eba;
 loop { match pc {
 0x1071eba => {
  { let v: u32 = cpu.sub32(cpu.r32(0x10e3f60u32), 0xa0u32); cpu.w32(0x10e3f60u32, v); }
  if !cpu.sf { pc = 0x1071ed0; continue; }
  pc = 0x1071ec6; continue;
 }
 0x1071ec6 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e3f60u32, v); }
  pc = 0x1071ed0; continue;
 }
 0x1071ed0 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x10e3f64u32), 0x64u32); cpu.w32(0x10e3f64u32, v); }
  if !cpu.sf { pc = 0x1071ee3; continue; }
  pc = 0x1071ed9; continue;
 }
 0x1071ed9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e3f64u32, v); }
  pc = 0x1071ee3; continue;
 }
 0x1071ee3 => {
  { let v: u32 = cpu.r32(0x10e3f60u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x280u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3d17u32)); }
  if cpu.cf { pc = 0x1071f05; continue; }
  pc = 0x1071ef6; continue;
 }
 0x1071ef6 => {
  { let v: u16 = cpu.r16(0x10e3d17u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x280u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f60u32, v); }
  pc = 0x1071f05; continue;
 }
 0x1071f05 => {
  { let v: u32 = cpu.r32(0x10e3f64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e412cu32)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3d19u32)); }
  if cpu.cf { pc = 0x1071f2a; continue; }
  pc = 0x1071f19; continue;
 }
 0x1071f19 => {
  { let v: u16 = cpu.r16(0x10e3d19u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x10e412cu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f64u32, v); }
  pc = 0x1071f2a; continue;
 }
 0x1071f2a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01071f2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1071f2b); }
 let mut pc: u32 = 0x1071f2b;
 loop { match pc {
 0x1071f2b => {
  { cpu.sub8(cpu.r8(0x11c59cfu32), 0x0u8); }
  if cpu.zf { pc = 0x1072004; continue; }
  pc = 0x1071f38; continue;
 }
 0x1071f38 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c59cfu32, v); }
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  cpu.push(0x1071f4e); f_0107f990(cpu); if cpu.returned_to != 0x1071f4e { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e412cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x190u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10c085au32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c085au32, v); }
  { let v: u16 = cpu.r16(0x10c085eu32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c085eu32, v); }
  { let v: u16 = cpu.r16(0x10c086eu32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c086eu32, v); }
  { let v: u16 = cpu.r16(0x10c0872u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c0872u32, v); }
  { let v: u16 = cpu.r16(0x10c0882u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c0882u32, v); }
  { let v: u16 = cpu.r16(0x10c0886u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c0886u32, v); }
  { let v: u16 = cpu.r16(0x10c0896u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c0896u32, v); }
  { let v: u16 = cpu.r16(0x10c089au32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c089au32, v); }
  { let v: u16 = cpu.r16(0x10c08aau32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c08aau32, v); }
  { let v: u16 = cpu.r16(0x10c08aeu32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c08aeu32, v); }
  { let v: u16 = cpu.r16(0x10c08beu32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c08beu32, v); }
  { let v: u16 = cpu.r16(0x10c08c2u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c08c2u32, v); }
  { let v: u16 = cpu.r16(0x10c08fau32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c08fau32, v); }
  { let v: u16 = cpu.r16(0x10c08feu32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c08feu32, v); }
  { let v: u16 = cpu.r16(0x10c0912u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c0912u32, v); }
  { let v: u16 = cpu.r16(0x10c0926u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c0926u32, v); }
  { let v: u16 = cpu.r16(0x10c07fau32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c07fau32, v); }
  { let v: u16 = cpu.r16(0x10c094au32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c094au32, v); }
  { let v: u16 = cpu.r16(0x10c094eu32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c094eu32, v); }
  { let v: u16 = cpu.r16(0x10c0962u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c0962u32, v); }
  { let v: u16 = cpu.r16(0x10c0976u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c0976u32, v); }
  { let v: u16 = cpu.r16(0x10c08d2u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c08d2u32, v); }
  { let v: u16 = cpu.r16(0x10c08d6u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x10c08d6u32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x10c07e6u32), (cpu.eax as u16)); cpu.w16(0x10c07e6u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1072004; continue;
 }
 0x1072004 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072005(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072005); }
 let mut pc: u32 = 0x1072005;
 loop { match pc {
 0x1072005 => {
  { cpu.sub8(cpu.r8(0x11c59cfu32), 0x0u8); }
  if !cpu.zf { pc = 0x10721f3; continue; }
  pc = 0x1072012; continue;
 }
 0x1072012 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c59cfu32, v); }
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  cpu.push(0x1072028); f_0107f95b(cpu); if cpu.returned_to != 0x1072028 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x10e412cu32), 0x1e0u32); }
  if !cpu.zf { pc = 0x1072119; continue; }
  pc = 0x107203a; continue;
 }
 0x107203a => {
  { let v: u16 = 0x1c6u16; cpu.w16(0x10c085au32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x10c085eu32, v); }
  { let v: u16 = 0x1c6u16; cpu.w16(0x10c086eu32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x10c0872u32, v); }
  { let v: u16 = 0x1c6u16; cpu.w16(0x10c0882u32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x10c0886u32, v); }
  { let v: u16 = 0x1c6u16; cpu.w16(0x10c0896u32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x10c089au32, v); }
  { let v: u16 = 0x1c6u16; cpu.w16(0x10c08aau32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x10c08aeu32, v); }
  { let v: u16 = 0x1c6u16; cpu.w16(0x10c08beu32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x10c08c2u32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x10c08fau32, v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x10c08feu32, v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x10c0912u32, v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x10c0926u32, v); }
  { let v: u16 = 0x1c0u16; cpu.w16(0x10c07fau32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x10c094au32, v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x10c094eu32, v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x10c0962u32, v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x10c0976u32, v); }
  { let v: u16 = 0x1c6u16; cpu.w16(0x10c08d2u32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x10c08d6u32, v); }
  { let v: u16 = 0x1c0u16; cpu.w16(0x10c07e6u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10721f3; continue;
 }
 0x1072119 => {
  { let v: u16 = 0x176u16; cpu.w16(0x10c085au32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x10c085eu32, v); }
  { let v: u16 = 0x176u16; cpu.w16(0x10c086eu32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x10c0872u32, v); }
  { let v: u16 = 0x176u16; cpu.w16(0x10c0882u32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x10c0886u32, v); }
  { let v: u16 = 0x176u16; cpu.w16(0x10c0896u32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x10c089au32, v); }
  { let v: u16 = 0x176u16; cpu.w16(0x10c08aau32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x10c08aeu32, v); }
  { let v: u16 = 0x176u16; cpu.w16(0x10c08beu32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x10c08c2u32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x10c08fau32, v); }
  { let v: u16 = 0x190u16; cpu.w16(0x10c08feu32, v); }
  { let v: u16 = 0x190u16; cpu.w16(0x10c0912u32, v); }
  { let v: u16 = 0x190u16; cpu.w16(0x10c0926u32, v); }
  { let v: u16 = 0x170u16; cpu.w16(0x10c07fau32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x10c094au32, v); }
  { let v: u16 = 0x190u16; cpu.w16(0x10c094eu32, v); }
  { let v: u16 = 0x190u16; cpu.w16(0x10c0962u32, v); }
  { let v: u16 = 0x190u16; cpu.w16(0x10c0976u32, v); }
  { let v: u16 = 0x176u16; cpu.w16(0x10c08d2u32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x10c08d6u32, v); }
  { let v: u16 = 0x170u16; cpu.w16(0x10c07e6u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10721f3; continue;
 }
 0x10721f3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010721f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10721f4); }
 let mut pc: u32 = 0x10721f4;
 loop { match pc {
 0x10721f4 => {
  { let v: u32 = 0x11c59ddu32; cpu.edx = v; }
  { let v: u32 = 0x11a37f0u32; cpu.ebx = v; }
  { let v: u32 = 0x300u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef31eu32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x1072215); f_01033bea(cpu); if cpu.returned_to != 0x1072215 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef31eu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107221e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107221e); }
 let mut pc: u32 = 0x107221e;
 loop { match pc {
 0x107221e => {
  { let v: u32 = 0x11c59d0u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10e4188u32); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef31eu32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x107223b); f_01033bea(cpu); if cpu.returned_to != 0x107223b { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef31eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c59ceu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107224b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107224b); }
 let mut pc: u32 = 0x107224b;
 loop { match pc {
 0x107224b => {
  cpu.push(0x1072250); f_01072257(cpu); if cpu.returned_to != 0x1072250 { return; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x10); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072257(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072257); }
 let mut pc: u32 = 0x1072257;
 loop { match pc {
 0x1072257 => {
  { cpu.sub8(cpu.r8(0x10ee969u32), 0x0u8); }
  if !cpu.zf { pc = 0x1072269; continue; }
  pc = 0x1072260; continue;
 }
 0x1072260 => {
  { let v: u8 = 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x10); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee969u32, v); }
  pc = 0x1072269; continue;
 }
 0x1072269 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107226a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107226a); }
 let mut pc: u32 = 0x107226a;
 loop { match pc {
 0x107226a => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5f4bu32, v); }
  cpu.push(0x1072276); f_01072257(cpu); if cpu.returned_to != 0x1072276 { return; }
  cpu.push(0x107227b); f_01072737(cpu); if cpu.returned_to != 0x107227b { return; }
  { let v: u16 = 0xa001u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d85u32, v); }
  { let v: u32 = 0xa402u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d89u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4114u32, v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x10e4114u32), 4u32); cpu.w32(0x10e4114u32, v); }
  cpu.push(0x10722a1); f_0105ec25(cpu); if cpu.returned_to != 0x10722a1 { return; }
  { let v: u16 = 0x3d4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x13u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x29u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e417cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4180u32, v); }
  { let v: u32 = 0x34u32; cpu.w32(0x11296ccu32, v); }
  { let v: u32 = 0x92u32; cpu.w32(0x11296d0u32, v); }
  { let v: u32 = 0x2bu32; cpu.w32(0x11296d4u32, v); }
  { let v: u32 = 0x112u32; cpu.w32(0x11296d8u32, v); }
  cpu.push(0x10722ec); f_010724a7(cpu); if cpu.returned_to != 0x10722ec { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010722ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10722ed); }
 let mut pc: u32 = 0x10722ed;
 loop { match pc {
 0x10722ed => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x11c5f4cu32), 0x0u8); }
  if !cpu.zf { pc = 0x107235e; continue; }
  pc = 0x10722f7; continue;
 }
 0x10722f7 => {
  cpu.push(0x10722fc); f_01072257(cpu); if cpu.returned_to != 0x10722fc { return; }
  cpu.push(0x1072301); f_01072737(cpu); if cpu.returned_to != 0x1072301 { return; }
  { let v: u16 = 0xa001u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xfffeu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d85u32, v); }
  { let v: u32 = 0xa402u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16) & 0xfffeu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d89u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4114u32, v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x10e4114u32), 4u32); cpu.w32(0x10e4114u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r16(0x10e3d89u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = cpu.r16(0x10e3d85u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d89u32, v); }
  cpu.push(0x1072343); f_010723c6(cpu); if cpu.returned_to != 0x1072343 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e3d89u32, v); }
  cpu.push(0x107234f); f_010723c6(cpu); if cpu.returned_to != 0x107234f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = 0x3d4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x13u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x29u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x107235e; continue;
 }
 0x107235e => {
  { cpu.sub8(cpu.r8(0x11c5f4cu32), 0x0u8); }
  if !cpu.zf { pc = 0x10723c4; continue; }
  pc = 0x1072367; continue;
 }
 0x1072367 => {
  cpu.push(0x107236c); f_0105ec4c(cpu); if cpu.returned_to != 0x107236c { return; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x638u32; cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x1072380); f_0105f339(cpu); if cpu.returned_to != 0x1072380 { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3d5fu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3d61u32, v); }
  { let v: u32 = cpu.r32(0x10e3f60u32) & 0xfffcu32; cpu.w32(0x10e3f60u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3d41u32, v); }
  { let v: u32 = 0x11bd390u32; cpu.esi = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { let v: u32 = 0x11a37f0u32; cpu.edi = v; }
  pc = 0x10723b2; continue;
 }
 0x10723b2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10723b2; continue; }
  pc = 0x10723bf; continue;
 }
 0x10723bf => {
  cpu.push(0x10723c4); f_0107269d(cpu); if cpu.returned_to != 0x10723c4 { return; }
  pc = 0x10723c4; continue;
 }
 0x10723c4 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010723c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10723c6); }
 let mut pc: u32 = 0x10723c6;
 loop { match pc {
 0x10723c6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x10e3d89u32) as u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xffffff00u32; cpu.edi = v; }
  let t = cpu.r32(0x10e436cu32); cpu.push(0x10723e0); cpu.call(t); if cpu.returned_to != 0x10723e0 { return; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.port_out((cpu.edx as u16), 2, (cpu.eax as u16) as u32); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x1004u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010723fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10723fb); }
 let mut pc: u32 = 0x10723fb;
 loop { match pc {
 0x10723fb => {
  { cpu.pushad(); }
  { let v: u32 = 0x11c5f91u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e4014u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x118cd68u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x107241f; continue; }
  pc = 0x1072413; continue;
 }
 0x1072413 => {
  { cpu.sub32(cpu.r32(0x118cd68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1072433; continue; }
  pc = 0x107241f; continue;
 }
 0x107241f => {
  { let v: u32 = cpu.edi; cpu.w32(0x118cd68u32, v); }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  pc = 0x107242a; continue;
 }
 0x107242a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x107242a; continue; }
  pc = 0x1072433; continue;
 }
 0x1072433 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072435(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072435); }
 let mut pc: u32 = 0x1072435;
 loop { match pc {
 0x1072435 => {
  { let v: u8 = cpu.r8(0x10ee969u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x1072440; continue; }
  pc = 0x107243e; continue;
 }
 0x107243e => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1072440; continue;
 }
 0x1072440 => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x10); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072445(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072445); }
 let mut pc: u32 = 0x1072445;
 loop { match pc {
 0x1072445 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x11cc9cfu32), 0x1u8); }
  if cpu.zf { pc = 0x107245f; continue; }
  pc = 0x1072455; continue;
 }
 0x1072455 => {
  { let v: u32 = 0xffffu32; cpu.ecx = v; }
  pc = 0x107245a; continue;
 }
 0x107245a => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.poll(); pc = 0x107245a; continue; }
  pc = 0x107245f; continue;
 }
 0x107245f => {
  { let v: u32 = cpu.r16(0x10e3d8du32) as u32; cpu.ecx = v; }
  pc = 0x1072466; continue;
 }
 0x1072466 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x1072466; continue; }
  pc = 0x107246b; continue;
 }
 0x107246b => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.poll(); pc = 0x107246b; continue; }
  pc = 0x1072470; continue;
 }
 0x1072470 => {
  if cpu.zf { cpu.poll(); pc = 0x107245f; continue; }
  pc = 0x1072472; continue;
 }
 0x1072472 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072476(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072476); }
 let mut pc: u32 = 0x1072476;
 loop { match pc {
 0x1072476 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x11cc9cfu32), 0x1u8); }
  if cpu.zf { pc = 0x1072490; continue; }
  pc = 0x1072486; continue;
 }
 0x1072486 => {
  { let v: u32 = 0xffffu32; cpu.ecx = v; }
  pc = 0x107248b; continue;
 }
 0x107248b => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.poll(); pc = 0x107248b; continue; }
  pc = 0x1072490; continue;
 }
 0x1072490 => {
  { let v: u32 = cpu.r16(0x10e3d8du32) as u32; cpu.ecx = v; }
  pc = 0x1072497; continue;
 }
 0x1072497 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x1072497; continue; }
  pc = 0x107249c; continue;
 }
 0x107249c => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.poll(); pc = 0x107249c; continue; }
  pc = 0x10724a1; continue;
 }
 0x10724a1 => {
  if cpu.zf { cpu.poll(); pc = 0x1072490; continue; }
  pc = 0x10724a3; continue;
 }
 0x10724a3 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010724a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10724a7); }
 let mut pc: u32 = 0x10724a7;
 loop { match pc {
 0x10724a7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x400u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x63u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x6u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10724b4; continue;
 }
 0x10724b4 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { cpu.poll(); pc = 0x10724b4; continue; }
  pc = 0x10724b9; continue;
 }
 0x10724b9 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10724b9; continue; }
  pc = 0x10724be; continue;
 }
 0x10724be => {
  { let v: u32 = 0xffffu32; cpu.ecx = v; }
  { cpu.interrupts_off = true; }
  pc = 0x10724c4; continue;
 }
 0x10724c4 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.poll(); pc = 0x10724c4; continue; }
  pc = 0x10724c9; continue;
 }
 0x10724c9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x10724fc; continue; }
  pc = 0x10724d3; continue;
 }
 0x10724d3 => {
  { let v: u32 = 0xffffu32; cpu.ecx = v; }
  pc = 0x10724d8; continue;
 }
 0x10724d8 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x10724d8; continue; }
  pc = 0x10724dd; continue;
 }
 0x10724dd => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.poll(); pc = 0x10724dd; continue; }
  pc = 0x10724e2; continue;
 }
 0x10724e2 => {
  { cpu.interrupts_off = false; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d8du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11cc9cfu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10724fc => {
  { let v: u8 = 0x1u8; cpu.w8(0x11cc9cfu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072505(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072505); }
 let mut pc: u32 = 0x1072505;
 loop { match pc {
 0x1072505 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x107250c; continue; }
  pc = 0x107250a; continue;
 }
 0x107250a => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x107250c; continue;
 }
 0x107250c => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x40u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x11b48f2u32; cpu.edx = v; }
  { let v: u32 = 0x100u32; cpu.ebx = v; }
  cpu.push(0x107252f); f_01014921(cpu); if cpu.returned_to != 0x107252f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x11b48f2u32; cpu.esi = v; }
  { let v: u32 = 0xfu32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x1072544); f_0108cb2a(cpu); if cpu.returned_to != 0x1072544 { return; }
  pc = 0x1072544; continue;
 }
 0x1072544 => {
  cpu.push(0x1072549); f_0108cdf7(cpu); if cpu.returned_to != 0x1072549 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x107255b; continue; }
  pc = 0x107254e; continue;
 }
 0x107254e => {
  let t = cpu.r32(0x10e4350u32); cpu.push(0x1072554); cpu.call(t); if cpu.returned_to != 0x1072554 { return; }
  cpu.push(0x1072559); f_01072640(cpu); if cpu.returned_to != 0x1072559 { return; }
  cpu.poll(); pc = 0x1072544; continue;
 }
 0x107255b => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107255d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107255d); }
 let mut pc: u32 = 0x107255d;
 loop { match pc {
 0x107255d => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x11b48f2u32; cpu.edx = v; }
  { let v: u32 = 0x100u32; cpu.ebx = v; }
  cpu.push(0x107256f); f_01014921(cpu); if cpu.returned_to != 0x107256f { return; }
  { let v: u32 = 0x11b48f2u32; cpu.esi = v; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x1072583); f_0108cb2a(cpu); if cpu.returned_to != 0x1072583 { return; }
  pc = 0x1072583; continue;
 }
 0x1072583 => {
  cpu.push(0x1072588); f_0108cdf7(cpu); if cpu.returned_to != 0x1072588 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x107259a; continue; }
  pc = 0x107258d; continue;
 }
 0x107258d => {
  let t = cpu.r32(0x10e4350u32); cpu.push(0x1072593); cpu.call(t); if cpu.returned_to != 0x1072593 { return; }
  cpu.push(0x1072598); f_01072640(cpu); if cpu.returned_to != 0x1072598 { return; }
  cpu.poll(); pc = 0x1072583; continue;
 }
 0x107259a => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107259c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107259c); }
 let mut pc: u32 = 0x107259c;
 loop { match pc {
 0x107259c => {
  { cpu.pushad(); }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x10725a3; continue; }
  pc = 0x10725a1; continue;
 }
 0x10725a1 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10725a3; continue;
 }
 0x10725a3 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x40u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x11b48f2u32; cpu.edx = v; }
  { let v: u32 = 0x100u32; cpu.ebx = v; }
  cpu.push(0x10725c6); f_01014921(cpu); if cpu.returned_to != 0x10725c6 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x11b48f2u32; cpu.esi = v; }
  { let v: u32 = 0xfu32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x10725db); f_0108cb2a(cpu); if cpu.returned_to != 0x10725db { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef30au32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010725e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10725e4); }
 let mut pc: u32 = 0x10725e4;
 loop { match pc {
 0x10725e4 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x10725eb; continue; }
  pc = 0x10725e9; continue;
 }
 0x10725e9 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10725eb; continue;
 }
 0x10725eb => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x40u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0xfu32; cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.esi = v; }
  cpu.push(0x1072610); f_0108cb2a(cpu); if cpu.returned_to != 0x1072610 { return; }
  pc = 0x1072610; continue;
 }
 0x1072610 => {
  cpu.push(0x1072615); f_0108cdf7(cpu); if cpu.returned_to != 0x1072615 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x1072627; continue; }
  pc = 0x107261a; continue;
 }
 0x107261a => {
  let t = cpu.r32(0x10e4350u32); cpu.push(0x1072620); cpu.call(t); if cpu.returned_to != 0x1072620 { return; }
  cpu.push(0x1072625); f_01072640(cpu); if cpu.returned_to != 0x1072625 { return; }
  cpu.poll(); pc = 0x1072610; continue;
 }
 0x1072627 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072629(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072629); }
 let mut pc: u32 = 0x1072629;
 loop { match pc {
 0x1072629 => {
  { cpu.pushad(); }
  cpu.push(0x107262f); f_0108cdf7(cpu); if cpu.returned_to != 0x107262f { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef30au32, v); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x107263e; continue; }
  pc = 0x1072639; continue;
 }
 0x1072639 => {
  cpu.push(0x107263e); f_01072640(cpu); if cpu.returned_to != 0x107263e { return; }
  pc = 0x107263e; continue;
 }
 0x107263e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072640); }
 let mut pc: u32 = 0x1072640;
 loop { match pc {
 0x1072640 => {
  let t = cpu.r32(0x10e4350u32); cpu.push(0x1072646); cpu.call(t); if cpu.returned_to != 0x1072646 { return; }
  { let v: u16 = 0x3c8u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x11eb7dcu32; cpu.esi = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1072656; continue;
 }
 0x1072656 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x107265b; continue;
 }
 0x107265b => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x107265b; continue; }
  pc = 0x1072660; continue;
 }
 0x1072660 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.lods(2, false); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  pc = 0x107266a; continue;
 }
 0x107266a => {
  { cpu.lods(2, false); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  pc = 0x1072673; continue;
 }
 0x1072673 => {
  { cpu.lods(2, false); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.add32(cpu.esi, 0x6u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x1072656; continue; }
  pc = 0x1072681; continue;
 }
 0x1072681 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072682(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072682); }
 let mut pc: u32 = 0x1072682;
 loop { match pc {
 0x1072682 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { let v: u32 = 0x11a37f0u32; cpu.edi = v; }
  { cpu.stos(4, true); }
  cpu.push(0x1072695); f_0107269d(cpu); if cpu.returned_to != 0x1072695 { return; }
  { cpu.pushad(); }
  cpu.push(0x107269b); f_0108ca0c(cpu); if cpu.returned_to != 0x107269b { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107269d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107269d); }
 let mut pc: u32 = 0x107269d;
 loop { match pc {
 0x107269d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x10e4350u32); cpu.push(0x10726a7); cpu.call(t); if cpu.returned_to != 0x10726a7 { return; }
  { let v: u16 = 0x3c8u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x11a37f0u32; cpu.esi = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10726b7; continue;
 }
 0x10726b7 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x10726bc; continue;
 }
 0x10726bc => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x10726bc; continue; }
  pc = 0x10726c1; continue;
 }
 0x10726c1 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.lods(1, false); }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  pc = 0x10726c6; continue;
 }
 0x10726c6 => {
  { cpu.lods(1, false); }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  pc = 0x10726ca; continue;
 }
 0x10726ca => {
  { cpu.lods(1, false); }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x10726b7; continue; }
  pc = 0x10726d0; continue;
 }
 0x10726d0 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x11a37f0u32; cpu.esi = v; }
  cpu.push(0x10726df); f_0108ca8a(cpu); if cpu.returned_to != 0x10726df { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010726e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10726e1); }
 let mut pc: u32 = 0x10726e1;
 loop { match pc {
 0x10726e1 => {
  { cpu.pushad(); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010726f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10726f9); }
 let mut pc: u32 = 0x10726f9;
 loop { match pc {
 0x10726f9 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1072701; continue; }
  pc = 0x10726fd; continue;
 }
 0x10726fd => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  f_0107270b(cpu); return;
 }
 0x1072701 => {
  { cpu.sub8(cpu.r8(0x105593eu32), 0x0u8); }
  if !cpu.zf { f_0107270b(cpu); return; }
  pc = 0x107270a; continue;
 }
 0x107270a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107270b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107270b); }
 let mut pc: u32 = 0x107270b;
 loop { match pc {
 0x107270b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.interrupts_off = true; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1072718; continue;
 }
 0x1072718 => {
  { let v: u16 = 0x3c8u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  pc = 0x1072720; continue;
 }
 0x1072720 => {
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  pc = 0x1072727; continue;
 }
 0x1072727 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  pc = 0x107272c; continue;
 }
 0x107272c => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  pc = 0x1072731; continue;
 }
 0x1072731 => {
  { cpu.interrupts_off = false; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072737(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072737); }
 let mut pc: u32 = 0x1072737;
 loop { match pc {
 0x1072737 => {
  { let v: u8 = 0x13u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x10); }
  cpu.push(0x1072742); f_010724a7(cpu); if cpu.returned_to != 0x1072742 { return; }
  cpu.push(0x1072747); f_01072682(cpu); if cpu.returned_to != 0x1072747 { return; }
  { cpu.interrupts_off = true; }
  { let v: u16 = 0x3c4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xf7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = 0x3ceu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xefu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfdu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u16 = 0x3d4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x14u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xbfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x3d4u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x17u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x40u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { cpu.interrupts_off = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072794(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072794); }
 let mut pc: u32 = 0x1072794;
 loop { match pc {
 0x1072794 => {
  cpu.push(0x1072799); f_01072d00(cpu); if cpu.returned_to != 0x1072799 { return; }
  if cpu.cf { pc = 0x10727eb; continue; }
  pc = 0x107279b; continue;
 }
 0x107279b => {
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x10727eb; continue; }
  pc = 0x10727a4; continue;
 }
 0x10727a4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x112c96du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x112c970u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x112c96du32)); cpu.w8(0x112c96du32, v); }
  { let v: u8 = cpu.r8(0x112c96du32) & 0x7u8; cpu.w8(0x112c96du32, v); }
  cpu.push(0x10727c6); f_01012780(cpu); if cpu.returned_to != 0x10727c6 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x112c96eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x10727d9; continue; }
  pc = 0x10727d5; continue;
 }
 0x10727d5 => {
  { let v: u16 = 0x0u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x10727d9; continue;
 }
 0x10727d9 => {
  { cpu.sub16((cpu.edx as u16), 0x100u16); }
  if (cpu.sf != cpu.of) { pc = 0x10727e4; continue; }
  pc = 0x10727e0; continue;
 }
 0x10727e0 => {
  { let v: u16 = 0xffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x10727e4; continue;
 }
 0x10727e4 => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x10727e8; continue; }
  pc = 0x10727e8; continue;
 }
 0x10727e8 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10727eb; continue;
 }
 0x10727eb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010727ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10727ec); }
 let mut pc: u32 = 0x10727ec;
 loop { match pc {
 0x10727ec => {
  { let v: u32 = 0x115f3c4u32; cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x10727f6; continue;
 }
 0x10727f6 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if !cpu.zf { pc = 0x10727fe; continue; }
  pc = 0x10727fc; continue;
 }
 0x10727fc => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10727fe => {
  { let v: u32 = cpu.add32(cpu.edi, 0xau32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10727f6; continue; }
  pc = 0x1072803; continue;
 }
 0x1072803 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072805(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072805); }
 let mut pc: u32 = 0x1072805;
 loop { match pc {
 0x1072805 => {
  { let v: u32 = 0x115f3c4u32; cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1072813; continue;
 }
 0x1072813 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.ebp, 0xau32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1072813; continue; }
  pc = 0x107281c; continue;
 }
 0x107281c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107281d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107281d); }
 let mut pc: u32 = 0x107281d;
 loop { match pc {
 0x107281d => {
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1072826; continue;
 }
 0x1072826 => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0x10u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1072826; continue; }
  pc = 0x107282d; continue;
 }
 0x107282d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107282e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107282e); }
 let mut pc: u32 = 0x107282e;
 loop { match pc {
 0x107282e => {
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x1072833; continue;
 }
 0x1072833 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if !cpu.zf { pc = 0x107283c; continue; }
  pc = 0x107283a; continue;
 }
 0x107283a => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x107283c => {
  { let v: u32 = cpu.add32(cpu.edi, 0x12u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1072833; continue; }
  pc = 0x1072841; continue;
 }
 0x1072841 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072843(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072843); }
 let mut pc: u32 = 0x1072843;
 loop { match pc {
 0x1072843 => {
  { let v: u16 = cpu.r16(0x10e3d2bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x9u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112c95cu32, v); }
  { let v: u16 = cpu.r16(0x10e3d2fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x9u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112c95eu32, v); }
  { let v: u16 = cpu.r16(0x10e3d33u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x9u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112c960u32, v); }
  { let v: u16 = cpu.r16(0x10e3d37u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = 0x9u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112c962u32, v); }
  { let v: u32 = 0x112c7b2u32; cpu.esi = v; }
  pc = 0x107289c; continue;
 }
 0x107289c => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x1072906; continue; }
  pc = 0x10728a4; continue;
 }
 0x10728a4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.sf { pc = 0x10728bd; continue; }
  pc = 0x10728ae; continue;
 }
 0x10728ae => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x112c95cu32); cpu.mul16(v); }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10728c4; continue;
 }
 0x10728bd => {
  { let v = cpu.r16(0x112c95cu32); cpu.mul16(v); }
  pc = 0x10728c4; continue;
 }
 0x10728c4 => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d1bu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3d1du32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x115f5b8u32; cpu.edi = v; }
  cpu.push(0x1072901); f_01072acc(cpu); if cpu.returned_to != 0x1072901 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0xau32); cpu.esi = v; }
  cpu.poll(); pc = 0x107289c; continue;
 }
 0x1072906 => {
  { let v: u32 = 0x112c7b2u32; cpu.esi = v; }
  pc = 0x107290b; continue;
 }
 0x107290b => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x1072975; continue; }
  pc = 0x1072913; continue;
 }
 0x1072913 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.sf { pc = 0x107292c; continue; }
  pc = 0x107291d; continue;
 }
 0x107291d => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x112c95eu32); cpu.mul16(v); }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1072933; continue;
 }
 0x107292c => {
  { let v = cpu.r16(0x112c95eu32); cpu.mul16(v); }
  pc = 0x1072933; continue;
 }
 0x1072933 => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d1fu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3d21u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x115f93cu32; cpu.edi = v; }
  cpu.push(0x1072970); f_01072acc(cpu); if cpu.returned_to != 0x1072970 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0xau32); cpu.esi = v; }
  cpu.poll(); pc = 0x107290b; continue;
 }
 0x1072975 => {
  { let v: u32 = 0x112c7b2u32; cpu.esi = v; }
  pc = 0x107297a; continue;
 }
 0x107297a => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x10729e4; continue; }
  pc = 0x1072982; continue;
 }
 0x1072982 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.sf { pc = 0x107299b; continue; }
  pc = 0x107298c; continue;
 }
 0x107298c => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x112c960u32); cpu.mul16(v); }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10729a2; continue;
 }
 0x107299b => {
  { let v = cpu.r16(0x112c960u32); cpu.mul16(v); }
  pc = 0x10729a2; continue;
 }
 0x10729a2 => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d23u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3d25u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x115fcc0u32; cpu.edi = v; }
  cpu.push(0x10729df); f_01072acc(cpu); if cpu.returned_to != 0x10729df { return; }
  { let v: u32 = cpu.esi.wrapping_add(0xau32); cpu.esi = v; }
  cpu.poll(); pc = 0x107297a; continue;
 }
 0x10729e4 => {
  { let v: u32 = 0x112c7b2u32; cpu.esi = v; }
  pc = 0x10729e9; continue;
 }
 0x10729e9 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x1072a53; continue; }
  pc = 0x10729f1; continue;
 }
 0x10729f1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.sf { pc = 0x1072a0a; continue; }
  pc = 0x10729fb; continue;
 }
 0x10729fb => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x112c962u32); cpu.mul16(v); }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1072a11; continue;
 }
 0x1072a0a => {
  { let v = cpu.r16(0x112c962u32); cpu.mul16(v); }
  pc = 0x1072a11; continue;
 }
 0x1072a11 => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d27u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3d29u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x1160044u32; cpu.edi = v; }
  cpu.push(0x1072a4e); f_01072acc(cpu); if cpu.returned_to != 0x1072a4e { return; }
  { let v: u32 = cpu.esi.wrapping_add(0xau32); cpu.esi = v; }
  cpu.poll(); pc = 0x10729e9; continue;
 }
 0x1072a53 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072a54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072a54); }
 let mut pc: u32 = 0x1072a54;
 loop { match pc {
 0x1072a54 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112c7a8u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x112c7aau32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x112c7acu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x112c7aeu32, v); }
  pc = 0x1072a6f; continue;
 }
 0x1072a6f => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x1072aca; continue; }
  pc = 0x1072a77; continue;
 }
 0x1072a77 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x112c7a8u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x112c7aau32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfffeu16); }
  if cpu.zf { pc = 0x1072aa2; continue; }
  pc = 0x1072a9b; continue;
 }
 0x1072a9b => {
  { let v = cpu.r16(0x112c7acu32); cpu.mul16(v); }
  pc = 0x1072aa2; continue;
 }
 0x1072aa2 => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x112c7aeu32); cpu.edi = v; }
  cpu.push(0x1072ac5); f_01072acc(cpu); if cpu.returned_to != 0x1072ac5 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0xau32); cpu.esi = v; }
  cpu.poll(); pc = 0x1072a6f; continue;
 }
 0x1072aca => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072acc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072acc); }
 let mut pc: u32 = 0x1072acc;
 loop { match pc {
 0x1072acc => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1072ad5); f_0107282e(cpu); if cpu.returned_to != 0x1072ad5 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1072b14; continue; }
  pc = 0x1072adb; continue;
 }
 0x1072adb => {
  { let v: u8 = 0x0u8; cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x1072af3; continue; }
  pc = 0x1072aec; continue;
 }
 0x1072aec => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0xeu32.wrapping_add(cpu.edi)), 0x1u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x1072af3; continue;
 }
 0x1072af3 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x1072b14; continue;
 }
 0x1072b14 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072b15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072b15); }
 let mut pc: u32 = 0x1072b15;
 loop { match pc {
 0x1072b15 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112c952u32, v); }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x1072b21; continue;
 }
 0x1072b21 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x1072cb2; continue; }
  pc = 0x1072b2c; continue;
 }
 0x1072b2c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x112c952u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0xeu32.wrapping_add(cpu.edi)), 0x4u8); }
  if !cpu.zf { pc = 0x1072ca6; continue; }
  pc = 0x1072b3d; continue;
 }
 0x1072b3d => {
  { cpu.and8(cpu.r8(0xeu32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { pc = 0x1072c3a; continue; }
  pc = 0x1072b47; continue;
 }
 0x1072b47 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0xcu32.wrapping_add(cpu.edi)) as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.edx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u32 = cpu.edi; cpu.w32(0x10e40c0u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if (cpu.sf != cpu.of) { pc = 0x1072be1; continue; }
  pc = 0x1072b6e; continue;
 }
 0x1072b6e => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1072be1; continue; }
  pc = 0x1072b77; continue;
 }
 0x1072b77 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1072be1; continue; }
  pc = 0x1072b84; continue;
 }
 0x1072b84 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1072be1; continue; }
  pc = 0x1072b8d; continue;
 }
 0x1072b8d => {
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x11b4c6cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1072bbc; continue; }
  pc = 0x1072b9c; continue;
 }
 0x1072b9c => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e55f8u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf == cpu.of) { pc = 0x1072bbc; continue; }
  pc = 0x1072bac; continue;
 }
 0x1072bac => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e68bau32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x1072bc5; continue; }
  pc = 0x1072bbc; continue;
 }
 0x1072bbc => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b4c6cu32, v); }
  pc = 0x1072be1; continue;
 }
 0x1072bc5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e4010u32, v); }
  pc = 0x1072be1; continue;
 }
 0x1072be1 => {
  { cpu.sub16((cpu.ecx as u16), 0x6f6u16); }
  if !cpu.zf { pc = 0x1072c0d; continue; }
  pc = 0x1072be8; continue;
 }
 0x1072be8 => {
  cpu.push(0x1072bed); f_0105f178(cpu); if cpu.returned_to != 0x1072bed { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1fu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x1072c0d; continue; }
  pc = 0x1072bf3; continue;
 }
 0x1072bf3 => {
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1072c0d); f_0101c251(cpu); if cpu.returned_to != 0x1072c0d { return; }
  pc = 0x1072c0d; continue;
 }
 0x1072c0d => {
  { cpu.popad(); }
  { let v: u8 = cpu.dec8(cpu.r8(0xfu32.wrapping_add(cpu.edi))); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  if !cpu.zf { pc = 0x1072ca6; continue; }
  pc = 0x1072c17; continue;
 }
 0x1072c17 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0xcu32.wrapping_add(cpu.edi))); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0xdu32.wrapping_add(cpu.edi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1072ca6; continue; }
  pc = 0x1072c28; continue;
 }
 0x1072c28 => {
  { cpu.and8(cpu.r8(0xeu32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x1072c34; continue; }
  pc = 0x1072c2e; continue;
 }
 0x1072c2e => {
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.edi)) | 0x4u8; cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x1072ca6; continue;
 }
 0x1072c34 => {
  { let v: u8 = 0x0u8; cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x1072ca6; continue;
 }
 0x1072c3a => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfffeu16); }
  if !cpu.zf { pc = 0x1072c52; continue; }
  pc = 0x1072c44; continue;
 }
 0x1072c44 => {
  { cpu.sub16(cpu.r16(0x112c952u32), 0xfffeu16); }
  if !cpu.zf { pc = 0x1072ca6; continue; }
  pc = 0x1072c4e; continue;
 }
 0x1072c4e => {
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1072c52; continue;
 }
 0x1072c52 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x1072c5f; continue; }
  pc = 0x1072c58; continue;
 }
 0x1072c58 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1072c5f; continue;
 }
 0x1072c5f => {
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.edi)) & 0xfdu8; cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  { cpu.sub16((cpu.ebx as u16), (cpu.eax as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1072ca6; continue; }
  pc = 0x1072c68; continue;
 }
 0x1072c68 => {
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.edi)) | 0x2u8; cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  { cpu.sub16((cpu.ecx as u16), 0x1u16); }
  if !cpu.zf { pc = 0x1072ca6; continue; }
  pc = 0x1072c72; continue;
 }
 0x1072c72 => {
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x112c967u32, v); }
  cpu.push(0x1072c88); f_01072e0c(cpu); if cpu.returned_to != 0x1072c88 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c967u32, v); }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.edi)), 0xfffeu16); }
  if cpu.zf { pc = 0x1072ca5; continue; }
  pc = 0x1072ca0; continue;
 }
 0x1072ca0 => {
  cpu.push(0x1072ca5); f_0101c251(cpu); if cpu.returned_to != 0x1072ca5 { return; }
  pc = 0x1072ca5; continue;
 }
 0x1072ca5 => {
  { cpu.popad(); }
  pc = 0x1072ca6; continue;
 }
 0x1072ca6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x12u32); cpu.edi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1072b21; continue; }
  pc = 0x1072cb2; continue;
 }
 0x1072cb2 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072cb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072cb4); }
 let mut pc: u32 = 0x1072cb4;
 loop { match pc {
 0x1072cb4 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112c952u32, v); }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x1072cc0; continue;
 }
 0x1072cc0 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x1072cfe; continue; }
  pc = 0x1072cc7; continue;
 }
 0x1072cc7 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x112c952u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0xeu32.wrapping_add(cpu.edi)), 0x4u8); }
  if !cpu.zf { pc = 0x1072cf6; continue; }
  pc = 0x1072cd4; continue;
 }
 0x1072cd4 => {
  { cpu.and8(cpu.r8(0xeu32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { pc = 0x1072cf6; continue; }
  pc = 0x1072cda; continue;
 }
 0x1072cda => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfffeu16); }
  if cpu.zf { pc = 0x1072cf6; continue; }
  pc = 0x1072ce4; continue;
 }
 0x1072ce4 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x1072ced; continue; }
  pc = 0x1072cea; continue;
 }
 0x1072cea => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1072ced; continue;
 }
 0x1072ced => {
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1072cf6; continue; }
  pc = 0x1072cf2; continue;
 }
 0x1072cf2 => {
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.edi)) & 0xfdu8; cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x1072cf6; continue;
 }
 0x1072cf6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x12u32); cpu.edi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1072cc0; continue; }
  pc = 0x1072cfe; continue;
 }
 0x1072cfe => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072d00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072d00); }
 let mut pc: u32 = 0x1072d00;
 loop { match pc {
 0x1072d00 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffd0u16); }
  if (cpu.sf != cpu.of) { pc = 0x1072d48; continue; }
  pc = 0x1072d17; continue;
 }
 0x1072d17 => {
  { let v: u16 = cpu.r16(0x10e4104u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1072d48; continue; }
  pc = 0x1072d27; continue;
 }
 0x1072d27 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112c96eu32, v); }
  { cpu.sub16((cpu.ebx as u16), 0xffd0u16); }
  if (cpu.sf != cpu.of) { pc = 0x1072d48; continue; }
  pc = 0x1072d33; continue;
 }
 0x1072d33 => {
  { let v: u16 = cpu.r16(0x10e4108u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1072d48; continue; }
  pc = 0x1072d43; continue;
 }
 0x1072d43 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1072d48 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072d4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072d4d); }
 let mut pc: u32 = 0x1072d4d;
 loop { match pc {
 0x1072d4d => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x110572du32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ebfu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  cpu.push(0x1072db5); f_0105afc4(cpu); if cpu.returned_to != 0x1072db5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3ebfu32)); cpu.w16(0x10e3ebfu32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  cpu.push(0x1072ddf); f_0105afc4(cpu); if cpu.returned_to != 0x1072ddf { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3ebfu32)); cpu.w16(0x10e3ebfu32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  cpu.push(0x1072e09); f_0105afc4(cpu); if cpu.returned_to != 0x1072e09 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01072e0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1072e0c); }
 let mut pc: u32 = 0x1072e0c;
 loop { match pc {
 0x1072e0c => {
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if cpu.sf { pc = 0x10734ba; continue; }
  pc = 0x1072e16; continue;
 }
 0x1072e16 => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if cpu.sf { pc = 0x10734ba; continue; }
  pc = 0x1072e20; continue;
 }
 0x1072e20 => {
  { cpu.sub16(cpu.r16(0x10e3d17u32), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10734ba; continue; }
  pc = 0x1072e2d; continue;
 }
 0x1072e2d => {
  { cpu.sub16(cpu.r16(0x10e3d19u32), (cpu.ebx as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10734ba; continue; }
  pc = 0x1072e3a; continue;
 }
 0x1072e3a => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10e3e99u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e97u32, v); }
  { cpu.sub8(cpu.r8(0x112c964u32), 0x0u8); }
  if !cpu.zf { pc = 0x1072ec2; continue; }
  pc = 0x1072e52; continue;
 }
 0x1072e52 => {
  cpu.push(0x1072e57); f_01072d00(cpu); if cpu.returned_to != 0x1072e57 { return; }
  if cpu.cf { pc = 0x1072ec2; continue; }
  pc = 0x1072e59; continue;
 }
 0x1072e59 => {
  { cpu.sub8(cpu.r8(0x10f7dc8u32), 0x0u8); }
  if !cpu.zf { pc = 0x1072ec2; continue; }
  pc = 0x1072e62; continue;
 }
 0x1072e62 => {
  { cpu.sub8(cpu.r8(0x112c96cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1072ec2; continue; }
  pc = 0x1072e6b; continue;
 }
 0x1072e6b => {
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1072ec2; continue; }
  pc = 0x1072e74; continue;
 }
 0x1072e74 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x112c96du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x112c970u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x112c96du32)); cpu.w8(0x112c96du32, v); }
  { let v: u8 = cpu.r8(0x112c96du32) & 0x7u8; cpu.w8(0x112c96du32, v); }
  cpu.push(0x1072e96); f_01012780(cpu); if cpu.returned_to != 0x1072e96 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x112c96eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x1072ea9; continue; }
  pc = 0x1072ea5; continue;
 }
 0x1072ea5 => {
  { let v: u16 = 0x0u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1072ea9; continue;
 }
 0x1072ea9 => {
  { cpu.sub16((cpu.edx as u16), 0x100u16); }
  if (cpu.sf != cpu.of) { pc = 0x1072eb4; continue; }
  pc = 0x1072eb0; continue;
 }
 0x1072eb0 => {
  { let v: u16 = 0xffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1072eb4; continue;
 }
 0x1072eb4 => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x1072eb8; continue; }
  pc = 0x1072eb8; continue;
 }
 0x1072eb8 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc8u32, v); }
  pc = 0x1072ec2; continue;
 }
 0x1072ec2 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x112c942u32), 0x0u8); }
  if cpu.zf { pc = 0x1072ed5; continue; }
  pc = 0x1072ecc; continue;
 }
 0x1072ecc => {
  { cpu.sub8(cpu.r8(0x112c967u32), 0x0u8); }
  if !cpu.zf { pc = 0x1072ed5; continue; }
  pc = 0x1072ed5; continue;
 }
 0x1072ed5 => {
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  cpu.push(0x1072ee0); f_010738df(cpu); if cpu.returned_to != 0x1072ee0 { return; }
  cpu.push(0x1072ee5); f_0102f7cb(cpu); if cpu.returned_to != 0x1072ee5 { return; }
  cpu.push(0x1072eea); f_0107363d(cpu); if cpu.returned_to != 0x1072eea { return; }
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x110d6f0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x110d769u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d6f0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110d769u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x18u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1072f14); f_01018aa3(cpu); if cpu.returned_to != 0x1072f14 { return; }
  if !cpu.cf { pc = 0x1072f61; continue; }
  pc = 0x1072f16; continue;
 }
 0x1072f16 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x1072f3c; continue; }
  pc = 0x1072f2c; continue;
 }
 0x1072f2c => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1072f35); f_0104f36d(cpu); if cpu.returned_to != 0x1072f35 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)) & 0x80u8; cpu.w8(0x4000u32.wrapping_add(cpu.esi), v); }
  pc = 0x1072f3c; continue;
 }
 0x1072f3c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ef33au32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  cpu.push(0x1072f58); f_01072d4d(cpu); if cpu.returned_to != 0x1072f58 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10ef33au32, v); }
  pc = 0x1072f62; continue;
 }
 0x1072f61 => {
  {  }
  pc = 0x1072f62; continue;
 }
 0x1072f62 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x110d769u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x110d6f0u32, v); }
  { cpu.popad(); }
  { cpu.popad(); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x18u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x18u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x10ea910u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x112c954u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10e3e99u32), 0x0u8); }
  if cpu.zf { pc = 0x1073013; continue; }
  pc = 0x1072fd3; continue;
 }
 0x1072fd3 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x112c964u32), 0x0u8); }
  if !cpu.zf { pc = 0x1073011; continue; }
  pc = 0x1072fde; continue;
 }
 0x1072fde => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32cu32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x81u8; cpu.w8(0x10ef32du32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x107300e); f_0105bf68(cpu); if cpu.returned_to != 0x107300e { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1073011; continue;
 }
 0x1073011 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1073013; continue;
 }
 0x1073013 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x112c954u32, v); }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if cpu.zf { pc = 0x1073192; continue; }
  pc = 0x1073026; continue;
 }
 0x1073026 => {
  { cpu.sub8(cpu.r8(0x112c969u32), 0x0u8); }
  if !cpu.zf { pc = 0x1073043; continue; }
  pc = 0x107302f; continue;
 }
 0x107302f => {
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x10731ed; continue; }
  pc = 0x1073037; continue;
 }
 0x1073037 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1073043; continue; }
  pc = 0x107303b; continue;
 }
 0x107303b => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10731ed; continue; }
  pc = 0x1073043; continue;
 }
 0x1073043 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x112c954u32, v); }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if cpu.zf { pc = 0x1073192; continue; }
  pc = 0x1073056; continue;
 }
 0x1073056 => {
  { cpu.sub8(cpu.r8(0x112c969u32), 0x0u8); }
  if !cpu.zf { pc = 0x1073073; continue; }
  pc = 0x107305f; continue;
 }
 0x107305f => {
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x10731ed; continue; }
  pc = 0x1073067; continue;
 }
 0x1073067 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1073073; continue; }
  pc = 0x107306b; continue;
 }
 0x107306b => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10731ed; continue; }
  pc = 0x1073073; continue;
 }
 0x1073073 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x112c954u32, v); }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if cpu.zf { pc = 0x1073192; continue; }
  pc = 0x1073088; continue;
 }
 0x1073088 => {
  { cpu.sub8(cpu.r8(0x112c969u32), 0x0u8); }
  if !cpu.zf { pc = 0x10730a5; continue; }
  pc = 0x1073091; continue;
 }
 0x1073091 => {
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x10731ed; continue; }
  pc = 0x1073099; continue;
 }
 0x1073099 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10730a5; continue; }
  pc = 0x107309d; continue;
 }
 0x107309d => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10731ed; continue; }
  pc = 0x10730a5; continue;
 }
 0x10730a5 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x81u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x112c954u32, v); }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if cpu.zf { pc = 0x1073192; continue; }
  pc = 0x10730bd; continue;
 }
 0x10730bd => {
  { cpu.sub8(cpu.r8(0x112c969u32), 0x0u8); }
  if !cpu.zf { pc = 0x10730da; continue; }
  pc = 0x10730c6; continue;
 }
 0x10730c6 => {
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x10731ed; continue; }
  pc = 0x10730ce; continue;
 }
 0x10730ce => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10730da; continue; }
  pc = 0x10730d2; continue;
 }
 0x10730d2 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10731ed; continue; }
  pc = 0x10730da; continue;
 }
 0x10730da => {
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x112c954u32, v); }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if cpu.zf { pc = 0x1073192; continue; }
  pc = 0x10730f2; continue;
 }
 0x10730f2 => {
  { cpu.sub8(cpu.r8(0x112c969u32), 0x0u8); }
  if !cpu.zf { pc = 0x107310f; continue; }
  pc = 0x10730fb; continue;
 }
 0x10730fb => {
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x10731ed; continue; }
  pc = 0x1073103; continue;
 }
 0x1073103 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x107310f; continue; }
  pc = 0x1073107; continue;
 }
 0x1073107 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10731ed; continue; }
  pc = 0x107310f; continue;
 }
 0x107310f => {
  { let v: u32 = cpu.edi.wrapping_sub(0x80u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x112c954u32, v); }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10731d4; continue; }
  pc = 0x1073127; continue;
 }
 0x1073127 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x10731f4; continue; }
  pc = 0x107312f; continue;
 }
 0x107312f => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x112c954u32, v); }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10731d4; continue; }
  pc = 0x1073142; continue;
 }
 0x1073142 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x10731f4; continue; }
  pc = 0x107314a; continue;
 }
 0x107314a => {
  { let v: u32 = cpu.edi.wrapping_sub(0x2u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x112c954u32, v); }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10731d4; continue; }
  pc = 0x107315b; continue;
 }
 0x107315b => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x10731f4; continue; }
  pc = 0x1073163; continue;
 }
 0x1073163 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x7fu32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x112c954u32, v); }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10731d4; continue; }
  pc = 0x1073174; continue;
 }
 0x1073174 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x10731f4; continue; }
  pc = 0x1073178; continue;
 }
 0x1073178 => {
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x112c954u32, v); }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10731d4; continue; }
  pc = 0x107318c; continue;
 }
 0x107318c => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x10731f4; continue; }
  pc = 0x1073190; continue;
 }
 0x1073190 => {
  pc = 0x10731f9; continue;
 }
 0x1073192 => {
  cpu.push(0x1073197); f_0101aa42(cpu); if cpu.returned_to != 0x1073197 { return; }
  if !cpu.cf { pc = 0x10731f9; continue; }
  pc = 0x1073199; continue;
 }
 0x1073199 => {
  { cpu.sub8(cpu.r8(0x112c967u32), 0x0u8); }
  if !cpu.zf { pc = 0x10731f9; continue; }
  pc = 0x10731a2; continue;
 }
 0x10731a2 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x1152e54u32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0x7fu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  cpu.push(0x10731d2); f_01047379(cpu); if cpu.returned_to != 0x10731d2 { return; }
  pc = 0x10731f9; continue;
 }
 0x10731d4 => {
  { cpu.sub8(cpu.r8(0x112c964u32), 0x0u8); }
  if !cpu.zf { pc = 0x10731f9; continue; }
  pc = 0x10731dd; continue;
 }
 0x10731dd => {
  { cpu.sub8(cpu.r8(0x112c969u32), 0x0u8); }
  if !cpu.zf { pc = 0x10731f9; continue; }
  pc = 0x10731e6; continue;
 }
 0x10731e6 => {
  cpu.push(0x10731eb); f_01073deb(cpu); if cpu.returned_to != 0x10731eb { return; }
  pc = 0x10731f9; continue;
 }
 0x10731ed => {
  cpu.push(0x10731f2); f_01034288(cpu); if cpu.returned_to != 0x10731f2 { return; }
  pc = 0x10731f9; continue;
 }
 0x10731f4 => {
  cpu.push(0x10731f9); f_01073d97(cpu); if cpu.returned_to != 0x10731f9 { return; }
  pc = 0x10731f9; continue;
 }
 0x10731f9 => {
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x112c940u32), 0x0u8); }
  if !cpu.zf { pc = 0x1073329; continue; }
  pc = 0x1073207; continue;
 }
 0x1073207 => {
  { cpu.sub8(cpu.r8(0x112c966u32), 0x0u8); }
  if !cpu.zf { pc = 0x10732a4; continue; }
  pc = 0x1073214; continue;
 }
 0x1073214 => {
  { cpu.sub8(cpu.r8(0x112c942u32), 0x0u8); }
  if cpu.zf { pc = 0x10732a4; continue; }
  pc = 0x1073221; continue;
 }
 0x1073221 => {
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x10732a4; continue; }
  pc = 0x1073227; continue;
 }
 0x1073227 => {
  { cpu.popad(); }
  cpu.push(0x107322d); f_01073508(cpu); if cpu.returned_to != 0x107322d { return; }
  if cpu.cf { pc = 0x1073329; continue; }
  pc = 0x1073233; continue;
 }
 0x1073233 => {
  { cpu.pushad(); }
  cpu.push(0x1073239); f_01018e6e(cpu); if cpu.returned_to != 0x1073239 { return; }
  if cpu.cf { pc = 0x1073265; continue; }
  pc = 0x107323b; continue;
 }
 0x107323b => {
  { cpu.popad(); }
  cpu.push(0x1073241); f_010654c6(cpu); if cpu.returned_to != 0x1073241 { return; }
  if cpu.cf { pc = 0x1073329; continue; }
  pc = 0x1073247; continue;
 }
 0x1073247 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x107325f); f_0106e1e0(cpu); if cpu.returned_to != 0x107325f { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1073329; continue;
 }
 0x1073265 => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1cu32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x107329e; continue; }
  pc = 0x1073275; continue;
 }
 0x1073275 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10e3e99u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.edx as u16)); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x107328b; continue; }
  pc = 0x1073287; continue;
 }
 0x1073287 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x107328b; continue;
 }
 0x107328b => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x107329e); f_01072cb4(cpu); if cpu.returned_to != 0x107329e { return; }
  pc = 0x107329e; continue;
 }
 0x107329e => {
  { cpu.popad(); }
  pc = 0x1073329; continue;
 }
 0x10732a4 => {
  { let v: u32 = cpu.r32(0x112c954u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x1du8); }
  if cpu.zf { pc = 0x107332a; continue; }
  pc = 0x10732b4; continue;
 }
 0x10732b4 => {
  { cpu.sub8((cpu.ecx as u8), 0x2au8); }
  if cpu.zf { pc = 0x107332a; continue; }
  pc = 0x10732b9; continue;
 }
 0x10732b9 => {
  { cpu.sub8((cpu.ecx as u8), 0x9u8); }
  if cpu.zf { pc = 0x107332a; continue; }
  pc = 0x10732be; continue;
 }
 0x10732be => {
  { cpu.sub8((cpu.ecx as u8), 0x27u8); }
  if cpu.zf { pc = 0x107332a; continue; }
  pc = 0x10732c3; continue;
 }
 0x10732c3 => {
  { cpu.sub8((cpu.ecx as u8), 0x26u8); }
  if cpu.zf { pc = 0x107332a; continue; }
  pc = 0x10732c8; continue;
 }
 0x10732c8 => {
  { cpu.sub8((cpu.ecx as u8), 0x28u8); }
  if cpu.zf { pc = 0x107332a; continue; }
  pc = 0x10732cd; continue;
 }
 0x10732cd => {
  cpu.push(0x10732d2); f_010727ec(cpu); if cpu.returned_to != 0x10732d2 { return; }
  if cpu.cf { pc = 0x1073329; continue; }
  pc = 0x10732d4; continue;
 }
 0x10732d4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.sub16(cpu.r16(0x10e3e97u32), 0x2au16); }
  if !cpu.zf { pc = 0x10732ee; continue; }
  pc = 0x10732e6; continue;
 }
 0x10732e6 => {
  { let v: u8 = 0x4u8; cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xbu8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x10732ee; continue;
 }
 0x10732ee => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x112c967u32), 0x0u8); }
  if cpu.zf { pc = 0x107331b; continue; }
  pc = 0x1073301; continue;
 }
 0x1073301 => {
  { let v: u8 = 0xffu8; cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x112c96bu32), 0x0u8); }
  if cpu.zf { pc = 0x1073329; continue; }
  pc = 0x1073312; continue;
 }
 0x1073312 => {
  { let v: u8 = cpu.r8(0x1124f58u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x107331b => {
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1073329; continue;
 }
 0x1073329 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x107332a => {
  cpu.push(0x107332f); f_010727ec(cpu); if cpu.returned_to != 0x107332f { return; }
  if cpu.cf { cpu.poll(); pc = 0x1073329; continue; }
  pc = 0x1073331; continue;
 }
 0x1073331 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x112c967u32), 0x0u8); }
  if cpu.zf { pc = 0x1073353; continue; }
  pc = 0x1073349; continue;
 }
 0x1073349 => {
  { let v: u8 = 0xffu8; cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1073361; continue;
 }
 0x1073353 => {
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1073361; continue;
 }
 0x1073361 => {
  { let v: u8 = 0x1u8; cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x5cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x18u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u16 = cpu.r16(0x1123072u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x10733cf; continue; }
  pc = 0x10733b8; continue;
 }
 0x10733b8 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x10733d4; continue; }
  pc = 0x10733c1; continue;
 }
 0x10733c1 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x10733d9; continue; }
  pc = 0x10733ca; continue;
 }
 0x10733ca => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x10733dc; continue;
 }
 0x10733cf => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  pc = 0x10733dc; continue;
 }
 0x10733d4 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x10733dc; continue;
 }
 0x10733d9 => {
  { let v: u32 = cpu.eax.wrapping_add(0x6u32); cpu.eax = v; }
  pc = 0x10733dc; continue;
 }
 0x10733dc => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x10733e4); f_0105ba7e(cpu); if cpu.returned_to != 0x10733e4 { return; }
  { let v: u16 = cpu.r16(0x1123080u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x107342f; continue; }
  pc = 0x1073418; continue;
 }
 0x1073418 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x1073434; continue; }
  pc = 0x1073421; continue;
 }
 0x1073421 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x1073439; continue; }
  pc = 0x107342a; continue;
 }
 0x107342a => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x107343c; continue;
 }
 0x107342f => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  pc = 0x107343c; continue;
 }
 0x1073434 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x107343c; continue;
 }
 0x1073439 => {
  { let v: u32 = cpu.eax.wrapping_add(0x6u32); cpu.eax = v; }
  pc = 0x107343c; continue;
 }
 0x107343c => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1073444); f_0105ba7e(cpu); if cpu.returned_to != 0x1073444 { return; }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = 0x10c2402u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x10734b9; continue; }
  pc = 0x107345b; continue;
 }
 0x107345b => {
  { let v: u16 = 0xdd1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u8 = 0xau8; cpu.w8(0x10ef342u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x10734a4; continue; }
  pc = 0x107348d; continue;
 }
 0x107348d => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x10734a9; continue; }
  pc = 0x1073496; continue;
 }
 0x1073496 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x10734ae; continue; }
  pc = 0x107349f; continue;
 }
 0x107349f => {
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  pc = 0x10734b1; continue;
 }
 0x10734a4 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  pc = 0x10734b1; continue;
 }
 0x10734a9 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x10734b1; continue;
 }
 0x10734ae => {
  { let v: u32 = cpu.add32(cpu.eax, 0x6u32); cpu.eax = v; }
  pc = 0x10734b1; continue;
 }
 0x10734b1 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x10734b9); f_0105bc4e(cpu); if cpu.returned_to != 0x10734b9 { return; }
  pc = 0x10734b9; continue;
 }
 0x10734b9 => {
  { cpu.popad(); }
  pc = 0x10734ba; continue;
 }
 0x10734ba => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010734bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10734bb); }
 let mut pc: u32 = 0x10734bb;
 loop { match pc {
 0x10734bb => {
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if !cpu.zf { pc = 0x10734c8; continue; }
  pc = 0x10734c3; continue;
 }
 0x10734c3 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x10734c8; continue;
 }
 0x10734c8 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x5cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u16 = 0xa1au16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  cpu.push(0x1073507); f_0105bdd1(cpu); if cpu.returned_to != 0x1073507 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01073508(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1073508); }
 let mut pc: u32 = 0x1073508;
 loop { match pc {
 0x1073508 => {
  { cpu.pushad(); }
  cpu.push(0x107350e); f_010735a6(cpu); if cpu.returned_to != 0x107350e { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x10735a4; continue; }
  pc = 0x1073515; continue;
 }
 0x1073515 => {
  cpu.push(0x107351a); f_010727ec(cpu); if cpu.returned_to != 0x107351a { return; }
  if cpu.cf { pc = 0x10735a4; continue; }
  pc = 0x1073520; continue;
 }
 0x1073520 => {
  { let v: u8 = 0x2u8; cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x107354a; continue; }
  pc = 0x107353a; continue;
 }
 0x107353a => {
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1073552; continue;
 }
 0x107354a => {
  { let v: u8 = 0xffu8; cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  pc = 0x1073552; continue;
 }
 0x1073552 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x5cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u16 = 0x799u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x10735a1); f_0105bbc5(cpu); if cpu.returned_to != 0x10735a1 { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10735a4 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010735a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10735a6); }
 let mut pc: u32 = 0x10735a6;
 loop { match pc {
 0x10735a6 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4134u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e40c0u32, v); }
  { let v: u32 = 0x115f3c4u32; cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x10735c7; continue;
 }
 0x10735c7 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x10735fe; continue; }
  pc = 0x10735cd; continue;
 }
 0x10735cd => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x10735fe; continue; }
  pc = 0x10735d3; continue;
 }
 0x10735d3 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.edi = v; }
  cpu.push(0x10735f7); f_01018ffd(cpu); if cpu.returned_to != 0x10735f7 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if !cpu.cf { pc = 0x10735fe; continue; }
  pc = 0x10735fb; continue;
 }
 0x10735fb => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10735fe => {
  { let v: u32 = cpu.add32(cpu.edi, 0xau32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10735c7; continue; }
  pc = 0x1073603; continue;
 }
 0x1073603 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01073606(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1073606); }
 let mut pc: u32 = 0x1073606;
 loop { match pc {
 0x1073606 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x112ad0cu32, v); }
  cpu.push(0x1073628); f_01018e6e(cpu); if cpu.returned_to != 0x1073628 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112ad0cu32, v); }
  if !cpu.cf { pc = 0x107363a; continue; }
  pc = 0x1073631; continue;
 }
 0x1073631 => {
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x107363a => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107363d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107363d); }
 let mut pc: u32 = 0x107363d;
 loop { match pc {
 0x107363d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112c958u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x112c95au32, v); }
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x18u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1073721; continue; }
  pc = 0x1073660; continue;
 }
 0x1073660 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x18u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u32 = (cpu.ebx as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xc0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xc0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x1073699); f_01018ffd(cpu); if cpu.returned_to != 0x1073699 { return; }
  if !cpu.cf { pc = 0x10736ab; continue; }
  pc = 0x107369b; continue;
 }
 0x107369b => {
  cpu.push(0x10736a0); f_01073606(cpu); if cpu.returned_to != 0x10736a0 { return; }
  if !cpu.cf { pc = 0x10736ab; continue; }
  pc = 0x10736a2; continue;
 }
 0x10736a2 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x15u32.wrapping_add(cpu.eax), v); }
  pc = 0x10736ab; continue;
 }
 0x10736ab => {
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x10736ee; continue; }
  pc = 0x10736b7; continue;
 }
 0x10736b7 => {
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), 0x10u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x10736ce); f_010738b1(cpu); if cpu.returned_to != 0x10736ce { return; }
  if !cpu.cf { pc = 0x10737fe; continue; }
  pc = 0x10736d4; continue;
 }
 0x10736d4 => {
  { let v: u16 = cpu.r8(0x10e3e99u32) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16(cpu.r16(0x10e3d2bu32), (cpu.eax as u16)); cpu.w16(0x10e3d2bu32, v); }
  if cpu.sf { pc = 0x1073838; continue; }
  pc = 0x10736e9; continue;
 }
 0x10736e9 => {
  pc = 0x10737fe; continue;
 }
 0x10736ee => {
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1073701); f_010738b1(cpu); if cpu.returned_to != 0x1073701 { return; }
  if !cpu.cf { pc = 0x10737fe; continue; }
  pc = 0x1073707; continue;
 }
 0x1073707 => {
  { let v: u16 = cpu.r8(0x10e3e99u32) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16(cpu.r16(0x10e3d2fu32), (cpu.eax as u16)); cpu.w16(0x10e3d2fu32, v); }
  if cpu.sf { pc = 0x1073844; continue; }
  pc = 0x107371c; continue;
 }
 0x107371c => {
  pc = 0x10737fe; continue;
 }
 0x1073721 => {
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { cpu.sub8(cpu.r8(0x112c967u32), 0x0u8); }
  if !cpu.zf { pc = 0x1073742; continue; }
  pc = 0x1073738; continue;
 }
 0x1073738 => {
  { cpu.sub16(cpu.r16(0x10e3e93u32), 0x1u16); }
  if cpu.zf { pc = 0x1073764; continue; }
  pc = 0x1073742; continue;
 }
 0x1073742 => {
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), 0x10u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10e3d2bu32; cpu.edi = v; }
  cpu.push(0x107375e); f_01073800(cpu); if cpu.returned_to != 0x107375e { return; }
  if cpu.cf { pc = 0x10737f1; continue; }
  pc = 0x1073764; continue;
 }
 0x1073764 => {
  { cpu.sub8(cpu.r8(0x112c967u32), 0x0u8); }
  if !cpu.zf { pc = 0x1073777; continue; }
  pc = 0x107376d; continue;
 }
 0x107376d => {
  { cpu.sub16(cpu.r16(0x10e3e93u32), 0x0u16); }
  if cpu.zf { pc = 0x1073791; continue; }
  pc = 0x1073777; continue;
 }
 0x1073777 => {
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10e3d2fu32; cpu.edi = v; }
  cpu.push(0x107378f); f_01073800(cpu); if cpu.returned_to != 0x107378f { return; }
  if cpu.cf { pc = 0x10737f1; continue; }
  pc = 0x1073791; continue;
 }
 0x1073791 => {
  { cpu.sub8(cpu.r8(0x112c967u32), 0x0u8); }
  if !cpu.zf { pc = 0x10737a4; continue; }
  pc = 0x107379a; continue;
 }
 0x107379a => {
  { cpu.sub16(cpu.r16(0x10e3e93u32), 0x2u16); }
  if cpu.zf { pc = 0x10737be; continue; }
  pc = 0x10737a4; continue;
 }
 0x10737a4 => {
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10e3d33u32; cpu.edi = v; }
  cpu.push(0x10737bc); f_01073800(cpu); if cpu.returned_to != 0x10737bc { return; }
  if cpu.cf { pc = 0x10737f1; continue; }
  pc = 0x10737be; continue;
 }
 0x10737be => {
  { cpu.sub8(cpu.r8(0x112c967u32), 0x0u8); }
  if !cpu.zf { pc = 0x10737d1; continue; }
  pc = 0x10737c7; continue;
 }
 0x10737c7 => {
  { cpu.sub16(cpu.r16(0x10e3e93u32), 0x3u16); }
  if cpu.zf { pc = 0x10737fe; continue; }
  pc = 0x10737d1; continue;
 }
 0x10737d1 => {
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), 0x10u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10e3d37u32; cpu.edi = v; }
  cpu.push(0x10737ed); f_01073800(cpu); if cpu.returned_to != 0x10737ed { return; }
  if cpu.cf { pc = 0x10737f1; continue; }
  pc = 0x10737ef; continue;
 }
 0x10737ef => {
  pc = 0x10737fe; continue;
 }
 0x10737f1 => {
  { let v: u16 = cpu.r8(0x10e3e99u32) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  if cpu.sf { pc = 0x1073808; continue; }
  pc = 0x10737fe; continue;
 }
 0x10737fe => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1073808 => {
  { cpu.sub32(cpu.edi, 0x10e3d2bu32); }
  if cpu.zf { pc = 0x1073838; continue; }
  pc = 0x1073810; continue;
 }
 0x1073810 => {
  { cpu.sub32(cpu.edi, 0x10e3d2fu32); }
  if cpu.zf { pc = 0x1073844; continue; }
  pc = 0x1073818; continue;
 }
 0x1073818 => {
  { cpu.sub32(cpu.edi, 0x10e3d33u32); }
  if cpu.zf { pc = 0x107382c; continue; }
  pc = 0x1073820; continue;
 }
 0x1073820 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x107382a); f_01002b38(cpu); if cpu.returned_to != 0x107382a { return; }
  cpu.poll(); pc = 0x10737fe; continue;
 }
 0x107382c => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x1073836); f_01002a3a(cpu); if cpu.returned_to != 0x1073836 { return; }
  cpu.poll(); pc = 0x10737fe; continue;
 }
 0x1073838 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x1073842); f_01002935(cpu); if cpu.returned_to != 0x1073842 { return; }
  cpu.poll(); pc = 0x10737fe; continue;
 }
 0x1073844 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x107384e); f_01002832(cpu); if cpu.returned_to != 0x107384e { return; }
  cpu.poll(); pc = 0x10737fe; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01073800(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1073800); }
 let mut pc: u32 = 0x1073800;
 loop { match pc {
 0x1073800 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1073806); f_010738b1(cpu); if cpu.returned_to != 0x1073806 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01073850(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1073850); }
 let mut pc: u32 = 0x1073850;
 loop { match pc {
 0x1073850 => {
  { let v: u32 = 0x115f5b8u32; cpu.edi = v; }
  { let v: u32 = cpu.r16(0x10e3d2du32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3d2bu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1073868); f_01072b15(cpu); if cpu.returned_to != 0x1073868 { return; }
  { let v: u32 = 0x115f93cu32; cpu.edi = v; }
  { let v: u32 = cpu.r16(0x10e3d31u32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3d2fu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1073880); f_01072b15(cpu); if cpu.returned_to != 0x1073880 { return; }
  { let v: u32 = 0x115fcc0u32; cpu.edi = v; }
  { let v: u32 = cpu.r16(0x10e3d35u32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3d33u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1073898); f_01072b15(cpu); if cpu.returned_to != 0x1073898 { return; }
  { let v: u32 = 0x1160044u32; cpu.edi = v; }
  { let v: u32 = cpu.r16(0x10e3d39u32) as u32; cpu.eax = v; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3d37u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10738b0); f_01072b15(cpu); if cpu.returned_to != 0x10738b0 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010738b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10738b1); }
 let mut pc: u32 = 0x10738b1;
 loop { match pc {
 0x10738b1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10738d9; continue; }
  pc = 0x10738ba; continue;
 }
 0x10738ba => {
  { cpu.sub16((cpu.edx as u16), (cpu.ebx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10738d9; continue; }
  pc = 0x10738bf; continue;
 }
 0x10738bf => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xa0u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0xa0u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x10738d9; continue; }
  pc = 0x10738ce; continue;
 }
 0x10738ce => {
  { cpu.sub16((cpu.edx as u16), (cpu.ebx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x10738d9; continue; }
  pc = 0x10738d3; continue;
 }
 0x10738d3 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10738d9 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010738df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10738df); }
 let mut pc: u32 = 0x10738df;
 loop { match pc {
 0x10738df => {
  { cpu.sub8(cpu.r8(0x112c966u32), 0x0u8); }
  if !cpu.zf { pc = 0x1073c03; continue; }
  pc = 0x10738ec; continue;
 }
 0x10738ec => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x10u32); cpu.ebx = v; }
  cpu.push(0x1073903); f_010192da(cpu); if cpu.returned_to != 0x1073903 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x107394f; continue; }
  pc = 0x107390c; continue;
 }
 0x107390c => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x10u32); cpu.ebx = v; }
  cpu.push(0x1073918); f_010192da(cpu); if cpu.returned_to != 0x1073918 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x107394f; continue; }
  pc = 0x1073921; continue;
 }
 0x1073921 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x20u32); cpu.ebx = v; }
  cpu.push(0x107392d); f_010192da(cpu); if cpu.returned_to != 0x107392d { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { cpu.popad(); }
  if cpu.cf { pc = 0x107394f; continue; }
  pc = 0x1073936; continue;
 }
 0x1073936 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x20u32); cpu.ebx = v; }
  cpu.push(0x1073942); f_010192da(cpu); if cpu.returned_to != 0x1073942 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1073c04; continue; }
  pc = 0x107394f; continue;
 }
 0x107394f => {
  { cpu.sub8(cpu.r8(0x112c967u32), 0x0u8); }
  if !cpu.zf { pc = 0x107396b; continue; }
  pc = 0x1073958; continue;
 }
 0x1073958 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x107396b; continue;
 }
 0x107396b => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x9au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0xfau16; cpu.w16(0x10e3ebfu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x10739c2; continue; }
  pc = 0x10739ab; continue;
 }
 0x10739ab => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x10739c7; continue; }
  pc = 0x10739b4; continue;
 }
 0x10739b4 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x10739cc; continue; }
  pc = 0x10739bd; continue;
 }
 0x10739bd => {
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  pc = 0x10739cf; continue;
 }
 0x10739c2 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x10739cf; continue;
 }
 0x10739c7 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x10739cf; continue;
 }
 0x10739cc => {
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  pc = 0x10739cf; continue;
 }
 0x10739cf => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x10739d7); f_0105a7f5(cpu); if cpu.returned_to != 0x10739d7 { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x1124f12u32), 0x0u8); }
  if !cpu.zf { pc = 0x10739f8; continue; }
  pc = 0x10739e1; continue;
 }
 0x10739e1 => {
  { let a = cpu.r32(0x10e4134u32); let b = cpu.esi; { let v: u32 = b; cpu.w32(0x10e4134u32, v); } { let v: u32 = a; cpu.esi = v; } }
  { let v: u8 = cpu.sub8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), (cpu.edx as u8)); cpu.w8(0xfu32.wrapping_add(cpu.esi), v); }
  { let a = cpu.r32(0x10e4134u32); let b = cpu.esi; { let v: u32 = b; cpu.w32(0x10e4134u32, v); } { let v: u32 = a; cpu.esi = v; } }
  if cpu.zf { pc = 0x10739f8; continue; }
  pc = 0x10739f2; continue;
 }
 0x10739f2 => {
  if !cpu.sf { pc = 0x1073c03; continue; }
  pc = 0x10739f8; continue;
 }
 0x10739f8 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x9au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x22du16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x10ef342u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x1073a59; continue; }
  pc = 0x1073a42; continue;
 }
 0x1073a42 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x1073a5e; continue; }
  pc = 0x1073a4b; continue;
 }
 0x1073a4b => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x1073a63; continue; }
  pc = 0x1073a54; continue;
 }
 0x1073a54 => {
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  pc = 0x1073a66; continue;
 }
 0x1073a59 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x1073a66; continue;
 }
 0x1073a5e => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x1073a66; continue;
 }
 0x1073a63 => {
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  pc = 0x1073a66; continue;
 }
 0x1073a66 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1073a6e); f_0105aa11(cpu); if cpu.returned_to != 0x1073a6e { return; }
  { cpu.popad(); }
  cpu.push(0x1073a74); f_01073cda(cpu); if cpu.returned_to != 0x1073a74 { return; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112c94cu32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112c94eu32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.eax)), 0x4u8); }
  if cpu.zf { pc = 0x1073b80; continue; }
  pc = 0x1073a9b; continue;
 }
 0x1073a9b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10c5974u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u16 = cpu.r16(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112c950u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x1073acb; continue; }
  pc = 0x1073ac7; continue;
 }
 0x1073ac7 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1073acb; continue;
 }
 0x1073acb => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1073ad6); f_0104f36d(cpu); if cpu.returned_to != 0x1073ad6 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x1073ae9; continue; }
  pc = 0x1073ae5; continue;
 }
 0x1073ae5 => {
  { let v: u8 = (cpu.eax as u8) | 0x16u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1073ae9; continue;
 }
 0x1073ae9 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x1073bf8; continue; }
  pc = 0x1073af5; continue;
 }
 0x1073af5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = cpu.r16(0x10e3e97u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  cpu.push(0x1073b5e); f_0105af81(cpu); if cpu.returned_to != 0x1073b5e { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x1124ef0u32), 0x1u8); }
  if !cpu.zf { pc = 0x1073bf8; continue; }
  pc = 0x1073b6e; continue;
 }
 0x1073b6e => {
  { cpu.sub16(cpu.r16(0x10e3e97u32), 0x1748u16); }
  if !cpu.zf { pc = 0x1073bf8; continue; }
  pc = 0x1073b79; continue;
 }
 0x1073b79 => {
  cpu.push(0x1073b7e); f_01039f69(cpu); if cpu.returned_to != 0x1073b7e { return; }
  pc = 0x1073bf8; continue;
 }
 0x1073b80 => {
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10c5974u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { cpu.sub8((cpu.eax as u8), 0x1eu8); }
  if cpu.zf { pc = 0x1073bba; continue; }
  pc = 0x1073b8d; continue;
 }
 0x1073b8d => {
  { cpu.sub8((cpu.eax as u8), 0x19u8); }
  if cpu.zf { pc = 0x1073bba; continue; }
  pc = 0x1073b91; continue;
 }
 0x1073b91 => {
  { cpu.sub8((cpu.eax as u8), 0x18u8); }
  if cpu.zf { pc = 0x1073bba; continue; }
  pc = 0x1073b95; continue;
 }
 0x1073b95 => {
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if cpu.zf { pc = 0x1073bba; continue; }
  pc = 0x1073b99; continue;
 }
 0x1073b99 => {
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x1073bba; continue; }
  pc = 0x1073b9d; continue;
 }
 0x1073b9d => {
  { cpu.sub8((cpu.eax as u8), 0x1fu8); }
  if cpu.zf { pc = 0x1073bb3; continue; }
  pc = 0x1073ba1; continue;
 }
 0x1073ba1 => {
  { cpu.sub8((cpu.eax as u8), 0x22u8); }
  if cpu.zf { pc = 0x1073bba; continue; }
  pc = 0x1073ba5; continue;
 }
 0x1073ba5 => {
  { cpu.sub8((cpu.eax as u8), 0x24u8); }
  if cpu.zf { pc = 0x1073bba; continue; }
  pc = 0x1073ba9; continue;
 }
 0x1073ba9 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x1073bba; continue; }
  pc = 0x1073bad; continue;
 }
 0x1073bad => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if !cpu.zf { pc = 0x1073bf8; continue; }
  pc = 0x1073bb1; continue;
 }
 0x1073bb1 => {
  pc = 0x1073bba; continue;
 }
 0x1073bb3 => {
  cpu.push(0x1073bb8); f_01073c05(cpu); if cpu.returned_to != 0x1073bb8 { return; }
  pc = 0x1073bf8; continue;
 }
 0x1073bba => {
  { cpu.sub8(cpu.r8(0x112b65au32), 0x0u8); }
  if !cpu.zf { pc = 0x1073bf8; continue; }
  pc = 0x1073bc3; continue;
 }
 0x1073bc3 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1073bd3); f_0104f36d(cpu); if cpu.returned_to != 0x1073bd3 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x14u32); cpu.edx = v; }
  cpu.push(0x1073be2); f_01073cc7(cpu); if cpu.returned_to != 0x1073be2 { return; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.push(0x1073be8); f_01073cc7(cpu); if cpu.returned_to != 0x1073be8 { return; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x7eu32); cpu.esi = v; }
  cpu.push(0x1073bf1); f_01073cc7(cpu); if cpu.returned_to != 0x1073bf1 { return; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.push(0x1073bf7); f_01073cc7(cpu); if cpu.returned_to != 0x1073bf7 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1073bf8; continue;
 }
 0x1073bf8 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.popad(); }
  cpu.push(0x1073c03); f_01073c57(cpu); if cpu.returned_to != 0x1073c03 { return; }
  pc = 0x1073c03; continue;
 }
 0x1073c03 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1073c04 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01073c05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1073c05); }
 let mut pc: u32 = 0x1073c05;
 loop { match pc {
 0x1073c05 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x11a188cu32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x28u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  pc = 0x1073c2a; continue;
 }
 0x1073c2a => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x3eu8); }
  if cpu.zf { pc = 0x1073c39; continue; }
  pc = 0x1073c30; continue;
 }
 0x1073c30 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x1073c4c; continue; }
  pc = 0x1073c39; continue;
 }
 0x1073c39 => {
  { cpu.sub8(cpu.r8(0xafu32.wrapping_add(cpu.ebp)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1073c4c; continue; }
  pc = 0x1073c41; continue;
 }
 0x1073c41 => {
  { cpu.pushad(); }
  { let v: u8 = 0xffu8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1073c4b); f_010504cc(cpu); if cpu.returned_to != 0x1073c4b { return; }
  { cpu.popad(); }
  pc = 0x1073c4c; continue;
 }
 0x1073c4c => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1073c2a; continue; }
  pc = 0x1073c55; continue;
 }
 0x1073c55 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01073c57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1073c57); }
 let mut pc: u32 = 0x1073c57;
 loop { match pc {
 0x1073c57 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x112c94cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112c94eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1073c6a); f_0104f36d(cpu); if cpu.returned_to != 0x1073c6a { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1cu8); }
  if !cpu.zf { pc = 0x1073c84; continue; }
  pc = 0x1073c78; continue;
 }
 0x1073c78 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x112c948u32)); cpu.w32(0x112c948u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) & 0x80u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) | 0x16u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1073c84; continue;
 }
 0x1073c84 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1cu8); }
  if !cpu.zf { pc = 0x1073c99; continue; }
  pc = 0x1073c8d; continue;
 }
 0x1073c8d => {
  { let v: u32 = cpu.inc32(cpu.r32(0x112c948u32)); cpu.w32(0x112c948u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) & 0x80u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) | 0x16u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1073c99; continue;
 }
 0x1073c99 => {
  { let v: u32 = cpu.esi.wrapping_add(0x7fu32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1cu8); }
  if !cpu.zf { pc = 0x1073cb0; continue; }
  pc = 0x1073ca4; continue;
 }
 0x1073ca4 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x112c948u32)); cpu.w32(0x112c948u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) & 0x80u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) | 0x16u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1073cb0; continue;
 }
 0x1073cb0 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1cu8); }
  if !cpu.zf { pc = 0x1073cc5; continue; }
  pc = 0x1073cb9; continue;
 }
 0x1073cb9 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x112c948u32)); cpu.w32(0x112c948u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) & 0x80u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x16u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1073cc5; continue;
 }
 0x1073cc5 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01073cc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1073cc7); }
 let mut pc: u32 = 0x1073cc7;
 loop { match pc {
 0x1073cc7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1cu8); }
  if !cpu.zf { pc = 0x1073cd3; continue; }
  pc = 0x1073ccf; continue;
 }
 0x1073ccf => {
  { let v: u8 = 0x16u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1073cd3; continue;
 }
 0x1073cd3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) & 0x80u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01073cda(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1073cda); }
 let mut pc: u32 = 0x1073cda;
 loop { match pc {
 0x1073cda => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x15eu16); }
  if cpu.zf { pc = 0x1073cfe; continue; }
  pc = 0x1073cf0; continue;
 }
 0x1073cf0 => {
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x15fu16); }
  if cpu.zf { pc = 0x1073cfe; continue; }
  pc = 0x1073cfc; continue;
 }
 0x1073cfc => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1073cfe => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1073d0d); f_0104f36d(cpu); if cpu.returned_to != 0x1073d0d { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x16u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112c966u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112c967u32, v); }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x1073d3e); f_01072e0c(cpu); if cpu.returned_to != 0x1073d3e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c967u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c966u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x3cu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x538u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x10ef342u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  cpu.push(0x1073d95); f_0105b824(cpu); if cpu.returned_to != 0x1073d95 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01073d97(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1073d97); } r_01073d97(cpu, 0x1073d97); }
pub fn f_01073deb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1073deb); } r_01073d97(cpu, 0x1073deb); }
fn r_01073d97(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1073d97 => {
  { cpu.pushad(); }
  { let v: u8 = 0x7u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10e3e93u32, v); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  cpu.push(0x1073da7); f_01073ed0(cpu); if cpu.returned_to != 0x1073da7 { return; }
  if cpu.cf { pc = 0x1073de9; continue; }
  pc = 0x1073da9; continue;
 }
 0x1073da9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x80u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1073db3; continue; }
  pc = 0x1073db1; continue;
 }
 0x1073db1 => {
  { let v: u8 = 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1073db3; continue;
 }
 0x1073db3 => {
  { let v: u16 = cpu.r8(0xffffc000u32.wrapping_add(cpu.esi)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x10f7625u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x1073de9; continue; }
  pc = 0x1073dcc; continue;
 }
 0x1073dcc => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x1073de9; continue; }
  pc = 0x1073dd0; continue;
 }
 0x1073dd0 => {
  cpu.push(0x1073dd5); f_0105f215(cpu); if cpu.returned_to != 0x1073dd5 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10f784au32); cpu.edi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x1073e56; continue;
 }
 0x1073de9 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1073deb => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x80u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x112c97au32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u8 = 0x16u8; cpu.w8(0x10e3e93u32, v); }
  cpu.push(0x1073e07); f_01073e87(cpu); if cpu.returned_to != 0x1073e07 { return; }
  if cpu.cf { pc = 0x1073e75; continue; }
  pc = 0x1073e09; continue;
 }
 0x1073e09 => {
  { cpu.sub32(cpu.esi, 0x10ea910u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1073e75; continue; }
  pc = 0x1073e11; continue;
 }
 0x1073e11 => {
  { cpu.sub32(cpu.esi, 0x10ee910u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1073e75; continue; }
  pc = 0x1073e19; continue;
 }
 0x1073e19 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x80u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1073e23; continue; }
  pc = 0x1073e21; continue;
 }
 0x1073e21 => {
  { let v: u8 = 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1073e23; continue;
 }
 0x1073e23 => {
  { let v: u16 = cpu.r8(0xffffc000u32.wrapping_add(cpu.esi)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x10f7625u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x1073e75; continue; }
  pc = 0x1073e3c; continue;
 }
 0x1073e3c => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1073e75; continue; }
  pc = 0x1073e40; continue;
 }
 0x1073e40 => {
  { let v: u32 = (cpu.eax as u8) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1073e4c); f_0105f215(cpu); if cpu.returned_to != 0x1073e4c { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = 0x10f782au32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.edi = v; }
  pc = 0x1073e56; continue;
 }
 0x1073e56 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffc000u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) & 0x7fu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) | ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1073e75); f_0103443a(cpu); if cpu.returned_to != 0x1073e75 { return; }
  pc = 0x1073e75; continue;
 }
 0x1073e75 => {
  cpu.push(0x1073e7a); f_01073f23(cpu); if cpu.returned_to != 0x1073e7a { return; }
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x41u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1073e86; continue; }
  pc = 0x1073e85; continue;
 }
 0x1073e85 => {
  {  }
  pc = 0x1073e86; continue;
 }
 0x1073e86 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01073e87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1073e87); }
 let mut pc: u32 = 0x1073e87;
 loop { match pc {
 0x1073e87 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x80u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1073e91; continue; }
  pc = 0x1073e8f; continue;
 }
 0x1073e8f => {
  { let v: u8 = 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1073e91; continue;
 }
 0x1073e91 => {
  { let v: u16 = cpu.r8(0xffffc000u32.wrapping_add(cpu.edi)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x10f7625u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xffffc000u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x10f782au32; cpu.edi = v; }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  pc = 0x1073ebd; continue;
 }
 0x1073ebd => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x1073ec2; continue;
 }
 0x1073ec2 => {
  { cpu.scas(2, 0); }
  if cpu.zf { pc = 0x1073ece; continue; }
  pc = 0x1073ec6; continue;
 }
 0x1073ec6 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1073ec2; continue; }
  pc = 0x1073ec8; continue;
 }
 0x1073ec8 => {
  { let v: u16 = cpu.dec16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1073ebd; continue; }
  pc = 0x1073ecc; continue;
 }
 0x1073ecc => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1073ece => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01073ed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1073ed0); }
 let mut pc: u32 = 0x1073ed0;
 loop { match pc {
 0x1073ed0 => {
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x80u8); }
  if cpu.zf { pc = 0x1073f1f; continue; }
  pc = 0x1073ed5; continue;
 }
 0x1073ed5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x80u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1073edf; continue; }
  pc = 0x1073edd; continue;
 }
 0x1073edd => {
  { let v: u8 = 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1073edf; continue;
 }
 0x1073edf => {
  { let v: u16 = cpu.r8(0xffffc000u32.wrapping_add(cpu.edi)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x10f7625u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x1073f1f; continue; }
  pc = 0x1073efb; continue;
 }
 0x1073efb => {
  { let v: u8 = cpu.r8(0xffffc000u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x10f784au32; cpu.edi = v; }
  { let v: u32 = 0x2u32; cpu.ebp = v; }
  pc = 0x1073f10; continue;
 }
 0x1073f10 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x1073f15; continue;
 }
 0x1073f15 => {
  { cpu.scas(2, 0); }
  if cpu.zf { pc = 0x1073f21; continue; }
  pc = 0x1073f19; continue;
 }
 0x1073f19 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1073f15; continue; }
  pc = 0x1073f1b; continue;
 }
 0x1073f1b => {
  { let v: u16 = cpu.dec16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1073f10; continue; }
  pc = 0x1073f1f; continue;
 }
 0x1073f1f => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1073f21 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01073f23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1073f23); }
 let mut pc: u32 = 0x1073f23;
 loop { match pc {
 0x1073f23 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  cpu.push(0x1073f2c); f_01074124(cpu); if cpu.returned_to != 0x1073f2c { return; }
  if !cpu.cf { pc = 0x1074046; continue; }
  pc = 0x1073f32; continue;
 }
 0x1073f32 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10e3e99u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x1073f3f); f_01074124(cpu); if cpu.returned_to != 0x1073f3f { return; }
  if !cpu.cf { pc = 0x1073f79; continue; }
  pc = 0x1073f41; continue;
 }
 0x1073f41 => {
  { cpu.sub8(cpu.r8(0x10e3e99u32), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1073f79; continue; }
  pc = 0x1073f49; continue;
 }
 0x1073f49 => {
  { let v: u32 = cpu.sub32(cpu.edi, 0x80u32); cpu.edi = v; }
  cpu.push(0x1073f54); f_01074124(cpu); if cpu.returned_to != 0x1073f54 { return; }
  if !cpu.cf { pc = 0x1073f79; continue; }
  pc = 0x1073f56; continue;
 }
 0x1073f56 => {
  { cpu.sub8(cpu.r8(0x10e3e99u32), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1073f79; continue; }
  pc = 0x1073f5e; continue;
 }
 0x1073f5e => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  cpu.push(0x1073f64); f_01074124(cpu); if cpu.returned_to != 0x1073f64 { return; }
  if !cpu.cf { pc = 0x1073f79; continue; }
  pc = 0x1073f66; continue;
 }
 0x1073f66 => {
  { cpu.sub8(cpu.r8(0x10e3e99u32), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1073f79; continue; }
  pc = 0x1073f6e; continue;
 }
 0x1073f6e => {
  cpu.push(0x1073f73); f_0107404c(cpu); if cpu.returned_to != 0x1073f73 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x1074049; continue;
 }
 0x1073f79 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x80u32); cpu.edi = v; }
  cpu.push(0x1073f86); f_01074124(cpu); if cpu.returned_to != 0x1073f86 { return; }
  if !cpu.cf { pc = 0x1073fc1; continue; }
  pc = 0x1073f88; continue;
 }
 0x1073f88 => {
  { cpu.sub8(cpu.r8(0x10e3e99u32), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1073fc1; continue; }
  pc = 0x1073f90; continue;
 }
 0x1073f90 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x1073f96); f_01074124(cpu); if cpu.returned_to != 0x1073f96 { return; }
  if !cpu.cf { pc = 0x1073fc1; continue; }
  pc = 0x1073f98; continue;
 }
 0x1073f98 => {
  { cpu.sub8(cpu.r8(0x10e3e99u32), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1073fc1; continue; }
  pc = 0x1073fa0; continue;
 }
 0x1073fa0 => {
  { let v: u32 = cpu.sub32(cpu.edi, 0x80u32); cpu.edi = v; }
  cpu.push(0x1073fab); f_01074124(cpu); if cpu.returned_to != 0x1073fab { return; }
  if !cpu.cf { pc = 0x1073fc1; continue; }
  pc = 0x1073fad; continue;
 }
 0x1073fad => {
  { cpu.sub8(cpu.r8(0x10e3e99u32), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1073fc1; continue; }
  pc = 0x1073fb5; continue;
 }
 0x1073fb5 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  cpu.push(0x1073fbb); f_0107404c(cpu); if cpu.returned_to != 0x1073fbb { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x1074049; continue;
 }
 0x1073fc1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x80u32); cpu.edi = v; }
  cpu.push(0x1073fce); f_01074124(cpu); if cpu.returned_to != 0x1073fce { return; }
  if !cpu.cf { pc = 0x1074005; continue; }
  pc = 0x1073fd0; continue;
 }
 0x1073fd0 => {
  { cpu.sub8(cpu.r8(0x10e3e99u32), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1074005; continue; }
  pc = 0x1073fd8; continue;
 }
 0x1073fd8 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  cpu.push(0x1073fde); f_01074124(cpu); if cpu.returned_to != 0x1073fde { return; }
  if !cpu.cf { pc = 0x1074005; continue; }
  pc = 0x1073fe0; continue;
 }
 0x1073fe0 => {
  { cpu.sub8(cpu.r8(0x10e3e99u32), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1074005; continue; }
  pc = 0x1073fe8; continue;
 }
 0x1073fe8 => {
  { let v: u32 = cpu.sub32(cpu.edi, 0x80u32); cpu.edi = v; }
  cpu.push(0x1073ff3); f_01074124(cpu); if cpu.returned_to != 0x1073ff3 { return; }
  if !cpu.cf { pc = 0x1074005; continue; }
  pc = 0x1073ff5; continue;
 }
 0x1073ff5 => {
  { cpu.sub8(cpu.r8(0x10e3e99u32), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1074005; continue; }
  pc = 0x1073ffd; continue;
 }
 0x1073ffd => {
  cpu.push(0x1074002); f_0107404c(cpu); if cpu.returned_to != 0x1074002 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x1074049; continue;
 }
 0x1074005 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  cpu.push(0x107400d); f_01074124(cpu); if cpu.returned_to != 0x107400d { return; }
  if !cpu.cf { pc = 0x1074045; continue; }
  pc = 0x107400f; continue;
 }
 0x107400f => {
  { cpu.sub8(cpu.r8(0x10e3e99u32), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1074045; continue; }
  pc = 0x1074017; continue;
 }
 0x1074017 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x80u32); cpu.edi = v; }
  cpu.push(0x1074023); f_01074124(cpu); if cpu.returned_to != 0x1074023 { return; }
  if !cpu.cf { pc = 0x1074045; continue; }
  pc = 0x1074025; continue;
 }
 0x1074025 => {
  { cpu.sub8(cpu.r8(0x10e3e99u32), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1074045; continue; }
  pc = 0x107402d; continue;
 }
 0x107402d => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  cpu.push(0x1074033); f_01074124(cpu); if cpu.returned_to != 0x1074033 { return; }
  if !cpu.cf { pc = 0x1074045; continue; }
  pc = 0x1074035; continue;
 }
 0x1074035 => {
  { cpu.sub8(cpu.r8(0x10e3e99u32), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1074045; continue; }
  pc = 0x107403d; continue;
 }
 0x107403d => {
  cpu.push(0x1074042); f_0107404c(cpu); if cpu.returned_to != 0x1074042 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x1074049; continue;
 }
 0x1074045 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x1074046; continue;
 }
 0x1074046 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1074049 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107404c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107404c); }
 let mut pc: u32 = 0x107404c;
 loop { match pc {
 0x107404c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10e3e99u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x1074120; continue; }
  pc = 0x107405c; continue;
 }
 0x107405c => {
  { let v: u32 = (cpu.edx as u8) as u32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  cpu.push(0x1074067); f_0105f215(cpu); if cpu.returned_to != 0x1074067 { return; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x10f785au32); cpu.esi = v; }
  { cpu.lods(2, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffc000u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x16u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)) | ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  cpu.push(0x107408e); f_0103443a(cpu); if cpu.returned_to != 0x107408e { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x41u8); }
  if cpu.cf { pc = 0x107409c; continue; }
  pc = 0x1074097; continue;
 }
 0x1074097 => {
  cpu.push(0x107409c); f_01074122(cpu); if cpu.returned_to != 0x107409c { return; }
  pc = 0x107409c; continue;
 }
 0x107409c => {
  { cpu.lods(2, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffc001u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x16u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)) | ((cpu.eax >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  cpu.push(0x10740b6); f_0103443a(cpu); if cpu.returned_to != 0x10740b6 { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x41u8); }
  if cpu.cf { pc = 0x10740c4; continue; }
  pc = 0x10740bf; continue;
 }
 0x10740bf => {
  cpu.push(0x10740c4); f_01074122(cpu); if cpu.returned_to != 0x10740c4 { return; }
  pc = 0x10740c4; continue;
 }
 0x10740c4 => {
  { cpu.lods(2, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffc080u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x16u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x80u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.edi)) | ((cpu.eax >> 8) as u8); cpu.w8(0x80u32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  cpu.push(0x10740e4); f_0103443a(cpu); if cpu.returned_to != 0x10740e4 { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x41u8); }
  if cpu.cf { pc = 0x10740f2; continue; }
  pc = 0x10740ed; continue;
 }
 0x10740ed => {
  cpu.push(0x10740f2); f_01074122(cpu); if cpu.returned_to != 0x10740f2 { return; }
  pc = 0x10740f2; continue;
 }
 0x10740f2 => {
  { cpu.lods(2, false); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffc081u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x16u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x81u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.edi)) | ((cpu.eax >> 8) as u8); cpu.w8(0x81u32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  cpu.push(0x1074112); f_0103443a(cpu); if cpu.returned_to != 0x1074112 { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x41u8); }
  if cpu.cf { pc = 0x1074120; continue; }
  pc = 0x107411b; continue;
 }
 0x107411b => {
  cpu.push(0x1074120); f_01074122(cpu); if cpu.returned_to != 0x1074120 { return; }
  pc = 0x1074120; continue;
 }
 0x1074120 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01074122(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1074122); }
 let mut pc: u32 = 0x1074122;
 loop { match pc {
 0x1074122 => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01074124(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1074124); }
 let mut pc: u32 = 0x1074124;
 loop { match pc {
 0x1074124 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0xffffc000u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x10f782au32; cpu.edi = v; }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  pc = 0x107413f; continue;
 }
 0x107413f => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x1074144; continue;
 }
 0x1074144 => {
  { cpu.scas(2, 0); }
  if cpu.zf { pc = 0x1074151; continue; }
  pc = 0x1074148; continue;
 }
 0x1074148 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1074144; continue; }
  pc = 0x107414a; continue;
 }
 0x107414a => {
  { let v: u16 = cpu.dec16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x107413f; continue; }
  pc = 0x107414e; continue;
 }
 0x107414e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1074151 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01074154(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1074154); }
 let mut pc: u32 = 0x1074154;
 loop { match pc {
 0x1074154 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x107415f; continue; }
  pc = 0x1074159; continue;
 }
 0x1074159 => {
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x1074154; continue; }
  pc = 0x107415d; continue;
 }
 0x107415d => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x107415f => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010741a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10741a3); }
 let mut pc: u32 = 0x10741a3;
 loop { match pc {
 0x10741a3 => {
  { let v: u32 = 0x115f3c4u32; cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edi = v; }
  pc = 0x10741b3; continue;
 }
 0x10741b3 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x10741c3; continue; }
  pc = 0x10741ba; continue;
 }
 0x10741ba => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10741c1); f_010741cf(cpu); if cpu.returned_to != 0x10741c1 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x10741c3; continue;
 }
 0x10741c3 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xau32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10741b3; continue; }
  pc = 0x10741c8; continue;
 }
 0x10741c8 => {
  { let v: u32 = cpu.edi; cpu.w32(0x10e4010u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010741cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10741cf); }
 let mut pc: u32 = 0x10741cf;
 loop { match pc {
 0x10741cf => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if (cpu.sf != cpu.of) { pc = 0x10742aa; continue; }
  pc = 0x10741e4; continue;
 }
 0x10741e4 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x10742aa; continue; }
  pc = 0x10741f1; continue;
 }
 0x10741f1 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10742aa; continue; }
  pc = 0x1074206; continue;
 }
 0x1074206 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x10742aa; continue; }
  pc = 0x1074213; continue;
 }
 0x1074213 => {
  { cpu.sub8(cpu.r8(0x11b4c6cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1074234; continue; }
  pc = 0x107421c; continue;
 }
 0x107421c => {
  { cpu.sub32(cpu.edi, 0x10e55f8u32); }
  if (cpu.sf == cpu.of) { pc = 0x1074234; continue; }
  pc = 0x1074224; continue;
 }
 0x1074224 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e68bau32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x107423d; continue; }
  pc = 0x1074234; continue;
 }
 0x1074234 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b4c6cu32, v); }
  pc = 0x10742aa; continue;
 }
 0x107423d => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1074251; continue; }
  pc = 0x1074249; continue;
 }
 0x1074249 => {
  { let v: u8 = cpu.r8(0x112c934u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1074253; continue;
 }
 0x1074251 => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1074253; continue;
 }
 0x1074253 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1074271; continue; }
  pc = 0x1074267; continue;
 }
 0x1074267 => {
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x30u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1074311; continue; }
  pc = 0x1074271; continue;
 }
 0x1074271 => {
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1074298; continue; }
  pc = 0x107427a; continue;
 }
 0x107427a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x107429e; continue; }
  pc = 0x1074280; continue;
 }
 0x1074280 => {
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x107428f; continue; }
  pc = 0x1074286; continue;
 }
 0x1074286 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x1123076u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10742a2; continue;
 }
 0x107428f => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x1123074u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10742a2; continue;
 }
 0x1074298 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1707u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10742a2; continue;
 }
 0x107429e => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x18bu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10742a2; continue;
 }
 0x10742a2 => {
  { cpu.stos(2, false); }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xf0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.stos(2, false); }
  pc = 0x10742aa; continue;
 }
 0x10742aa => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { pc = 0x1074310; continue; }
  pc = 0x10742b3; continue;
 }
 0x10742b3 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x10742c4; continue; }
  pc = 0x10742b9; continue;
 }
 0x10742b9 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1074310; continue; }
  pc = 0x10742c2; continue;
 }
 0x10742c2 => {
  pc = 0x107430a; continue;
 }
 0x10742c4 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x1074384; continue; }
  pc = 0x10742d1; continue;
 }
 0x10742d1 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x10742f6; continue; }
  pc = 0x10742d7; continue;
 }
 0x10742d7 => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10742e7; continue; }
  pc = 0x10742de; continue;
 }
 0x10742de => {
  { let v: u16 = cpu.r16(0x112307au32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10742ee; continue;
 }
 0x10742e7 => {
  { let v: u16 = cpu.r16(0x1123078u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10742ee; continue;
 }
 0x10742ee => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), (cpu.ecx as u8)); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x1074310; continue; }
  pc = 0x10742f4; continue;
 }
 0x10742f4 => {
  pc = 0x107430a; continue;
 }
 0x10742f6 => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1074304; continue; }
  pc = 0x10742fc; continue;
 }
 0x10742fc => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if (cpu.sf != cpu.of) { pc = 0x1074310; continue; }
  pc = 0x1074302; continue;
 }
 0x1074302 => {
  pc = 0x107430a; continue;
 }
 0x1074304 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xcu8); }
  if (cpu.sf != cpu.of) { pc = 0x1074310; continue; }
  pc = 0x107430a; continue;
 }
 0x107430a => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1074310; continue;
 }
 0x1074310 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1074311 => {
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10e55fau32); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.esi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1074335; continue; }
  pc = 0x107432c; continue;
 }
 0x107432c => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x1123074u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x107433a; continue;
 }
 0x1074335 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x18bu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x107433a; continue;
 }
 0x107433a => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)).wrapping_sub((cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10e401cu32).wrapping_add(0x10u32); cpu.w32(0x10e401cu32, v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.edi = v; }
  cpu.poll(); pc = 0x10742aa; continue;
 }
 0x1074384 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xau16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x2cu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x20u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x10743a7); f_01077143(cpu); if cpu.returned_to != 0x10743a7 { return; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10743ae); f_0105381d(cpu); if cpu.returned_to != 0x10743ae { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x11a0f95u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0xb8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10743bf; continue;
 }
 0x10743bf => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x10744ba; continue; }
  pc = 0x10743c8; continue;
 }
 0x10743c8 => {
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x1152e54u32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x10743bf; continue; }
  pc = 0x10743da; continue;
 }
 0x10743da => {
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x107441a; continue; }
  pc = 0x10743df; continue;
 }
 0x10743df => {
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x10743bf; continue; }
  pc = 0x10743e5; continue;
 }
 0x10743e5 => {
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x107441a; continue; }
  pc = 0x10743ea; continue;
 }
 0x10743ea => {
  { cpu.pushad(); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = ((cpu.ebx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  cpu.push(0x1074403); f_01065635(cpu); if cpu.returned_to != 0x1074403 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x107440d); f_0108c181(cpu); if cpu.returned_to != 0x107440d { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.ebp = v; }
  cpu.push(0x1074419); f_0108b513(cpu); if cpu.returned_to != 0x1074419 { return; }
  { cpu.popad(); }
  pc = 0x107441a; continue;
 }
 0x107441a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x10744bc; continue; }
  pc = 0x1074424; continue;
 }
 0x1074424 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x92u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x52u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0xfau16; cpu.w16(0x10e3ebfu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x1074483; continue; }
  pc = 0x107446c; continue;
 }
 0x107446c => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x1074488; continue; }
  pc = 0x1074475; continue;
 }
 0x1074475 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x107448d; continue; }
  pc = 0x107447e; continue;
 }
 0x107447e => {
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  pc = 0x1074490; continue;
 }
 0x1074483 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x1074490; continue;
 }
 0x1074488 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  pc = 0x1074490; continue;
 }
 0x107448d => {
  { let v: u32 = cpu.add32(cpu.eax, 0x10u32); cpu.eax = v; }
  pc = 0x1074490; continue;
 }
 0x1074490 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1074499); f_0105a7f5(cpu); if cpu.returned_to != 0x1074499 { return; }
  { cpu.popad(); }
  cpu.push(0x107449f); f_01075344(cpu); if cpu.returned_to != 0x107449f { return; }
  if !cpu.sf { cpu.poll(); pc = 0x10743bf; continue; }
  pc = 0x10744a5; continue;
 }
 0x10744a5 => {
  cpu.push(0x10744aa); f_010745fe(cpu); if cpu.returned_to != 0x10744aa { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10744b2); f_010504cc(cpu); if cpu.returned_to != 0x10744b2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.poll(); pc = 0x10743bf; continue;
 }
 0x10744ba => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10744bc => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { cpu.poll(); pc = 0x10743bf; continue; }
  pc = 0x10744c6; continue;
 }
 0x10744c6 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x92u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x52u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0xfau16; cpu.w16(0x10e3ebfu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x1074528; continue; }
  pc = 0x1074511; continue;
 }
 0x1074511 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x107452d; continue; }
  pc = 0x107451a; continue;
 }
 0x107451a => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x1074532; continue; }
  pc = 0x1074523; continue;
 }
 0x1074523 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x8u32); cpu.eax = v; }
  pc = 0x1074535; continue;
 }
 0x1074528 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x1u32); cpu.eax = v; }
  pc = 0x1074535; continue;
 }
 0x107452d => {
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  pc = 0x1074535; continue;
 }
 0x1074532 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x1074535; continue;
 }
 0x1074535 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x107453e); f_0105a869(cpu); if cpu.returned_to != 0x107453e { return; }
  { cpu.popad(); }
  cpu.push(0x1074544); f_01075344(cpu); if cpu.returned_to != 0x1074544 { return; }
  if !cpu.sf { cpu.poll(); pc = 0x10743bf; continue; }
  pc = 0x107454a; continue;
 }
 0x107454a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x107455c; continue; }
  pc = 0x1074550; continue;
 }
 0x1074550 => {
  { cpu.pushad(); }
  cpu.push(0x1074556); f_0107479d(cpu); if cpu.returned_to != 0x1074556 { return; }
  { cpu.popad(); }
  cpu.poll(); pc = 0x10743bf; continue;
 }
 0x107455c => {
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x11b543eu32, v); }
  cpu.push(0x1074569); f_010745fe(cpu); if cpu.returned_to != 0x1074569 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11b543eu32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x107457f); f_010504cc(cpu); if cpu.returned_to != 0x107457f { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x9au16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x22du16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x10ef342u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x10745e2; continue; }
  pc = 0x10745cb; continue;
 }
 0x10745cb => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x10745e7; continue; }
  pc = 0x10745d4; continue;
 }
 0x10745d4 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x10745ec; continue; }
  pc = 0x10745dd; continue;
 }
 0x10745dd => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x10745ef; continue;
 }
 0x10745e2 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  pc = 0x10745ef; continue;
 }
 0x10745e7 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x10745ef; continue;
 }
 0x10745ec => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x10745ef; continue;
 }
 0x10745ef => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x10745f7); f_0105aa11(cpu); if cpu.returned_to != 0x10745f7 { return; }
  { cpu.popad(); }
  { cpu.popad(); }
  cpu.poll(); pc = 0x10743bf; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010745fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10745fe); }
 let mut pc: u32 = 0x10745fe;
 loop { match pc {
 0x10745fe => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x107460e); f_01072d00(cpu); if cpu.returned_to != 0x107460e { return; }
  if cpu.cf { pc = 0x1074654; continue; }
  pc = 0x1074610; continue;
 }
 0x1074610 => {
  { cpu.sub8(cpu.r8(0x10f7dc8u32), 0x0u8); }
  if !cpu.zf { pc = 0x1074654; continue; }
  pc = 0x1074619; continue;
 }
 0x1074619 => {
  { cpu.sub8(cpu.r8(0x11b543eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1074654; continue; }
  pc = 0x1074622; continue;
 }
 0x1074622 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1074642; continue; }
  pc = 0x1074638; continue;
 }
 0x1074638 => {
  cpu.push(0x107463d); f_01012780(cpu); if cpu.returned_to != 0x107463d { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1074642; continue;
 }
 0x1074642 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x107464d); f_010356fe(cpu); if cpu.returned_to != 0x107464d { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc8u32, v); }
  pc = 0x1074654; continue;
 }
 0x1074654 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x112c965u32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x22du16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x10746bd; continue; }
  pc = 0x10746a6; continue;
 }
 0x10746a6 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x10746c2; continue; }
  pc = 0x10746af; continue;
 }
 0x10746af => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x10746c7; continue; }
  pc = 0x10746b8; continue;
 }
 0x10746b8 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x10746ca; continue;
 }
 0x10746bd => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  pc = 0x10746ca; continue;
 }
 0x10746c2 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x10746ca; continue;
 }
 0x10746c7 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x10746ca; continue;
 }
 0x10746ca => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x10746d2); f_0105aa11(cpu); if cpu.returned_to != 0x10746d2 { return; }
  { let v: u16 = 0x616u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u8 = cpu.r8(0x112c965u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1eu8); }
  if cpu.zf { pc = 0x1074725; continue; }
  pc = 0x10746eb; continue;
 }
 0x10746eb => {
  { cpu.sub8((cpu.eax as u8), 0x22u8); }
  if cpu.zf { pc = 0x1074737; continue; }
  pc = 0x10746ef; continue;
 }
 0x10746ef => {
  { cpu.sub8((cpu.eax as u8), 0x16u8); }
  if cpu.zf { pc = 0x107479b; continue; }
  pc = 0x10746f7; continue;
 }
 0x10746f7 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x107478a; continue; }
  pc = 0x10746ff; continue;
 }
 0x10746ff => {
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x1074739; continue; }
  pc = 0x1074703; continue;
 }
 0x1074703 => {
  { cpu.sub8((cpu.eax as u8), 0x18u8); }
  if cpu.zf { pc = 0x1074766; continue; }
  pc = 0x1074707; continue;
 }
 0x1074707 => {
  { cpu.sub8((cpu.eax as u8), 0x19u8); }
  if cpu.zf { pc = 0x1074778; continue; }
  pc = 0x107470b; continue;
 }
 0x107470b => {
  { cpu.sub8((cpu.eax as u8), 0x24u8); }
  if cpu.zf { pc = 0x107474b; continue; }
  pc = 0x107470f; continue;
 }
 0x107470f => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x107475d; continue; }
  pc = 0x1074713; continue;
 }
 0x1074713 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x107479b; continue; }
  pc = 0x107471b; continue;
 }
 0x107471b => {
  { cpu.sub8((cpu.eax as u8), 0x15u8); }
  if cpu.zf { pc = 0x107479b; continue; }
  pc = 0x107471f; continue;
 }
 0x107471f => {
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if cpu.zf { pc = 0x107479b; continue; }
  pc = 0x1074723; continue;
 }
 0x1074723 => {
  pc = 0x107479b; continue;
 }
 0x1074725 => {
  { let v: u16 = 0x166bu16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  pc = 0x107478a; continue;
 }
 0x1074737 => {
  pc = 0x107479b; continue;
 }
 0x1074739 => {
  { let v: u16 = 0x162fu16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  pc = 0x107478a; continue;
 }
 0x107474b => {
  { let v: u16 = 0x16bau16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  pc = 0x107478a; continue;
 }
 0x107475d => {
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  pc = 0x107478a; continue;
 }
 0x1074766 => {
  { let v: u16 = 0x74du16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  pc = 0x107478a; continue;
 }
 0x1074778 => {
  { let v: u16 = 0x14d9u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  pc = 0x107478a; continue;
 }
 0x107478a => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x107479b); f_0105aa75(cpu); if cpu.returned_to != 0x107479b { return; }
  pc = 0x107479b; continue;
 }
 0x107479b => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107479d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107479d); }
 let mut pc: u32 = 0x107479d;
 loop { match pc {
 0x107479d => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { f_01074827(cpu); return; }
  pc = 0x10747a7; continue;
 }
 0x10747a7 => {
  { cpu.sub8(cpu.r8(0x1128d9cu32), 0x0u8); }
  if !cpu.zf { f_01074827(cpu); return; }
  pc = 0x10747b0; continue;
 }
 0x10747b0 => {
  cpu.push(0x10747b5); f_01074827(cpu); if cpu.returned_to != 0x10747b5 { return; }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.eax = v; }
  { let v: u8 = 0xf4u8; cpu.w8(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x1074821; continue; }
  pc = 0x10747cb; continue;
 }
 0x10747cb => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x10747f7; continue; }
  pc = 0x10747d1; continue;
 }
 0x10747d1 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if cpu.zf { pc = 0x10747fd; continue; }
  pc = 0x10747d7; continue;
 }
 0x10747d7 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x1074803; continue; }
  pc = 0x10747dd; continue;
 }
 0x10747dd => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x1074809; continue; }
  pc = 0x10747e3; continue;
 }
 0x10747e3 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x107480f; continue; }
  pc = 0x10747e9; continue;
 }
 0x10747e9 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x1074815; continue; }
  pc = 0x10747ef; continue;
 }
 0x10747ef => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x107481b; continue; }
  pc = 0x10747f5; continue;
 }
 0x10747f5 => {
  f_01074827(cpu); return;
 }
 0x10747f7 => {
  cpu.push(0x10747fc); f_01074a26(cpu); if cpu.returned_to != 0x10747fc { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10747fd => {
  cpu.push(0x1074802); f_01074b1b(cpu); if cpu.returned_to != 0x1074802 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1074803 => {
  cpu.push(0x1074808); f_01074d30(cpu); if cpu.returned_to != 0x1074808 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1074809 => {
  cpu.push(0x107480e); f_01074c3b(cpu); if cpu.returned_to != 0x107480e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x107480f => {
  cpu.push(0x1074814); f_01074dfa(cpu); if cpu.returned_to != 0x1074814 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1074815 => {
  cpu.push(0x107481a); f_01074ec4(cpu); if cpu.returned_to != 0x107481a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x107481b => {
  cpu.push(0x1074820); f_01074f8e(cpu); if cpu.returned_to != 0x1074820 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1074821 => {
  cpu.push(0x1074826); f_0107490b(cpu); if cpu.returned_to != 0x1074826 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01074827(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1074827); }
 let mut pc: u32 = 0x1074827;
 loop { match pc {
 0x1074827 => {
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x112c941u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef334u32, v); }
  cpu.push(0x107483b); f_010504cc(cpu); if cpu.returned_to != 0x107483b { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef334u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c941u32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x76fu16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x1074898); f_0105a7f5(cpu); if cpu.returned_to != 0x1074898 { return; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010748a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10748a0); }
 let mut pc: u32 = 0x10748a0;
 loop { match pc {
 0x10748a0 => {
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef334u32, v); }
  cpu.push(0x10748ad); f_010504cc(cpu); if cpu.returned_to != 0x10748ad { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef334u32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x76fu16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x1074903); f_0105a7f5(cpu); if cpu.returned_to != 0x1074903 { return; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107490b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107490b); }
 let mut pc: u32 = 0x107490b;
 loop { match pc {
 0x107490b => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x107491b); f_01072d00(cpu); if cpu.returned_to != 0x107491b { return; }
  if cpu.cf { pc = 0x1074958; continue; }
  pc = 0x107491d; continue;
 }
 0x107491d => {
  { cpu.sub8(cpu.r8(0x10f7dc8u32), 0x0u8); }
  if !cpu.zf { pc = 0x1074958; continue; }
  pc = 0x1074926; continue;
 }
 0x1074926 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1074946; continue; }
  pc = 0x107493c; continue;
 }
 0x107493c => {
  cpu.push(0x1074941); f_01012780(cpu); if cpu.returned_to != 0x1074941 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1074946; continue;
 }
 0x1074946 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1074951); f_010356fe(cpu); if cpu.returned_to != 0x1074951 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc8u32, v); }
  pc = 0x1074958; continue;
 }
 0x1074958 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x1641u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x3u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x10749a8); f_0105aa75(cpu); if cpu.returned_to != 0x10749a8 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1637u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x10749d3); f_0105aa75(cpu); if cpu.returned_to != 0x10749d3 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010749d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10749d5); }
 let mut pc: u32 = 0x10749d5;
 loop { match pc {
 0x10749d5 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x3au16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074a24); f_0105aa75(cpu); if cpu.returned_to != 0x1074a24 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01074a26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1074a26); }
 let mut pc: u32 = 0x1074a26;
 loop { match pc {
 0x1074a26 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1074a36); f_01072d00(cpu); if cpu.returned_to != 0x1074a36 { return; }
  if cpu.cf { pc = 0x1074a73; continue; }
  pc = 0x1074a38; continue;
 }
 0x1074a38 => {
  { cpu.sub8(cpu.r8(0x10f7dc8u32), 0x0u8); }
  if !cpu.zf { pc = 0x1074a73; continue; }
  pc = 0x1074a41; continue;
 }
 0x1074a41 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1074a61; continue; }
  pc = 0x1074a57; continue;
 }
 0x1074a57 => {
  cpu.push(0x1074a5c); f_01012780(cpu); if cpu.returned_to != 0x1074a5c { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1074a61; continue;
 }
 0x1074a61 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1074a6c); f_010356fe(cpu); if cpu.returned_to != 0x1074a6c { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc8u32, v); }
  pc = 0x1074a73; continue;
 }
 0x1074a73 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x166bu16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074ac3); f_0105aa75(cpu); if cpu.returned_to != 0x1074ac3 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1673u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074aee); f_0105aa75(cpu); if cpu.returned_to != 0x1074aee { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x215u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074b19); f_0105aa75(cpu); if cpu.returned_to != 0x1074b19 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01074b1b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1074b1b); }
 let mut pc: u32 = 0x1074b1b;
 loop { match pc {
 0x1074b1b => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1074b2b); f_01072d00(cpu); if cpu.returned_to != 0x1074b2b { return; }
  if cpu.cf { pc = 0x1074b68; continue; }
  pc = 0x1074b2d; continue;
 }
 0x1074b2d => {
  { cpu.sub8(cpu.r8(0x10f7dc8u32), 0x0u8); }
  if !cpu.zf { pc = 0x1074b68; continue; }
  pc = 0x1074b36; continue;
 }
 0x1074b36 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1074b56; continue; }
  pc = 0x1074b4c; continue;
 }
 0x1074b4c => {
  cpu.push(0x1074b51); f_01012780(cpu); if cpu.returned_to != 0x1074b51 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1074b56; continue;
 }
 0x1074b56 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1074b61); f_010356fe(cpu); if cpu.returned_to != 0x1074b61 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc8u32, v); }
  pc = 0x1074b68; continue;
 }
 0x1074b68 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x616u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074bb8); f_0105aa75(cpu); if cpu.returned_to != 0x1074bb8 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x200u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074be3); f_0105aa75(cpu); if cpu.returned_to != 0x1074be3 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x209u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074c0e); f_0105aa75(cpu); if cpu.returned_to != 0x1074c0e { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x215u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074c39); f_0105aa75(cpu); if cpu.returned_to != 0x1074c39 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01074c3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1074c3b); }
 let mut pc: u32 = 0x1074c3b;
 loop { match pc {
 0x1074c3b => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1074c4b); f_01072d00(cpu); if cpu.returned_to != 0x1074c4b { return; }
  if cpu.cf { pc = 0x1074c88; continue; }
  pc = 0x1074c4d; continue;
 }
 0x1074c4d => {
  { cpu.sub8(cpu.r8(0x10f7dc8u32), 0x0u8); }
  if !cpu.zf { pc = 0x1074c88; continue; }
  pc = 0x1074c56; continue;
 }
 0x1074c56 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1074c76; continue; }
  pc = 0x1074c6c; continue;
 }
 0x1074c6c => {
  cpu.push(0x1074c71); f_01012780(cpu); if cpu.returned_to != 0x1074c71 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1074c76; continue;
 }
 0x1074c76 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1074c81); f_010356fe(cpu); if cpu.returned_to != 0x1074c81 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc8u32, v); }
  pc = 0x1074c88; continue;
 }
 0x1074c88 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x14d9u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074cd8); f_0105aa75(cpu); if cpu.returned_to != 0x1074cd8 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x14c5u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074d03); f_0105aa75(cpu); if cpu.returned_to != 0x1074d03 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x215u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074d2e); f_0105aa75(cpu); if cpu.returned_to != 0x1074d2e { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01074d30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1074d30); }
 let mut pc: u32 = 0x1074d30;
 loop { match pc {
 0x1074d30 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1074d40); f_01072d00(cpu); if cpu.returned_to != 0x1074d40 { return; }
  if cpu.cf { pc = 0x1074d7d; continue; }
  pc = 0x1074d42; continue;
 }
 0x1074d42 => {
  { cpu.sub8(cpu.r8(0x10f7dc8u32), 0x0u8); }
  if !cpu.zf { pc = 0x1074d7d; continue; }
  pc = 0x1074d4b; continue;
 }
 0x1074d4b => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1074d6b; continue; }
  pc = 0x1074d61; continue;
 }
 0x1074d61 => {
  cpu.push(0x1074d66); f_01012780(cpu); if cpu.returned_to != 0x1074d66 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1074d6b; continue;
 }
 0x1074d6b => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1074d76); f_010356fe(cpu); if cpu.returned_to != 0x1074d76 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc8u32, v); }
  pc = 0x1074d7d; continue;
 }
 0x1074d7d => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x74du16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074dcd); f_0105aa75(cpu); if cpu.returned_to != 0x1074dcd { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x735u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074df8); f_0105aa75(cpu); if cpu.returned_to != 0x1074df8 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01074dfa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1074dfa); }
 let mut pc: u32 = 0x1074dfa;
 loop { match pc {
 0x1074dfa => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1074e0a); f_01072d00(cpu); if cpu.returned_to != 0x1074e0a { return; }
  if cpu.cf { pc = 0x1074e47; continue; }
  pc = 0x1074e0c; continue;
 }
 0x1074e0c => {
  { cpu.sub8(cpu.r8(0x10f7dc8u32), 0x0u8); }
  if !cpu.zf { pc = 0x1074e47; continue; }
  pc = 0x1074e15; continue;
 }
 0x1074e15 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1074e35; continue; }
  pc = 0x1074e2b; continue;
 }
 0x1074e2b => {
  cpu.push(0x1074e30); f_01012780(cpu); if cpu.returned_to != 0x1074e30 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1074e35; continue;
 }
 0x1074e35 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1074e40); f_010356fe(cpu); if cpu.returned_to != 0x1074e40 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc8u32, v); }
  pc = 0x1074e47; continue;
 }
 0x1074e47 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x162fu16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074e97); f_0105aa75(cpu); if cpu.returned_to != 0x1074e97 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1638u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074ec2); f_0105aa75(cpu); if cpu.returned_to != 0x1074ec2 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01074ec4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1074ec4); }
 let mut pc: u32 = 0x1074ec4;
 loop { match pc {
 0x1074ec4 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1074ed4); f_01072d00(cpu); if cpu.returned_to != 0x1074ed4 { return; }
  if cpu.cf { pc = 0x1074f11; continue; }
  pc = 0x1074ed6; continue;
 }
 0x1074ed6 => {
  { cpu.sub8(cpu.r8(0x10f7dc8u32), 0x0u8); }
  if !cpu.zf { pc = 0x1074f11; continue; }
  pc = 0x1074edf; continue;
 }
 0x1074edf => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1074eff; continue; }
  pc = 0x1074ef5; continue;
 }
 0x1074ef5 => {
  cpu.push(0x1074efa); f_01012780(cpu); if cpu.returned_to != 0x1074efa { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1074eff; continue;
 }
 0x1074eff => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1074f0a); f_010356fe(cpu); if cpu.returned_to != 0x1074f0a { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc8u32, v); }
  pc = 0x1074f11; continue;
 }
 0x1074f11 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x16bau16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074f61); f_0105aa75(cpu); if cpu.returned_to != 0x1074f61 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1640u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x1074f8c); f_0105aa75(cpu); if cpu.returned_to != 0x1074f8c { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01074f8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1074f8e); }
 let mut pc: u32 = 0x1074f8e;
 loop { match pc {
 0x1074f8e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01074f8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1074f8f); }
 let mut pc: u32 = 0x1074f8f;
 loop { match pc {
 0x1074f8f => {
  { let v: u8 = 0x1u8; cpu.w8(0x112c940u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112c967u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112c969u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112c96bu32, v); }
  { cpu.pushad(); }
  cpu.push(0x1074fb1); f_01072e0c(cpu); if cpu.returned_to != 0x1074fb1 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c969u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c96bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c967u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c940u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x112c968u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x112c96au32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112c96bu32, v); }
  f_01075035(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01074ff6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1074ff6); }
 let mut pc: u32 = 0x1074ff6;
 loop { match pc {
 0x1074ff6 => {
  cpu.push(0x1074ffb); f_01075035(cpu); if cpu.returned_to != 0x1074ffb { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c96bu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075003(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075003); }
 let mut pc: u32 = 0x1075003;
 loop { match pc {
 0x1075003 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x112c968u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112c940u32, v); }
  { cpu.pushad(); }
  cpu.push(0x1075016); f_01072e0c(cpu); if cpu.returned_to != 0x1075016 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c940u32, v); }
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x5u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x40u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), 0x2eu16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  f_01075035(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075035(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075035); }
 let mut pc: u32 = 0x1075035;
 loop { match pc {
 0x1075035 => {
  cpu.push(0x107503a); f_01077143(cpu); if cpu.returned_to != 0x107503a { return; }
  { cpu.pushad(); }
  cpu.push(0x1075040); f_0105381d(cpu); if cpu.returned_to != 0x1075040 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x11a0f95u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x112c968u32); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0xb8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x107504e; continue;
 }
 0x107504e => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x1075105; continue; }
  pc = 0x1075057; continue;
 }
 0x1075057 => {
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x1152e54u32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x107504e; continue; }
  pc = 0x1075069; continue;
 }
 0x1075069 => {
  { cpu.sub8(cpu.r8(0x112c96au32), 0x0u8); }
  if cpu.zf { pc = 0x107507c; continue; }
  pc = 0x1075072; continue;
 }
 0x1075072 => {
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1124f58u32)); }
  if cpu.zf { cpu.poll(); pc = 0x107504e; continue; }
  pc = 0x107507c; continue;
 }
 0x107507c => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x10752b8; continue; }
  pc = 0x1075086; continue;
 }
 0x1075086 => {
  { cpu.sub8(cpu.r8(0x112c964u32), 0x0u8); }
  if !cpu.zf { pc = 0x10750de; continue; }
  pc = 0x107508f; continue;
 }
 0x107508f => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x92u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x52u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0xfau16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  cpu.push(0x10750dd); f_0105a7f5(cpu); if cpu.returned_to != 0x10750dd { return; }
  { cpu.popad(); }
  pc = 0x10750de; continue;
 }
 0x10750de => {
  cpu.push(0x10750e3); f_01075344(cpu); if cpu.returned_to != 0x10750e3 { return; }
  if !cpu.sf { cpu.poll(); pc = 0x107504e; continue; }
  pc = 0x10750e9; continue;
 }
 0x10750e9 => {
  cpu.push(0x10750ee); f_01075115(cpu); if cpu.returned_to != 0x10750ee { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10750f6); f_010504cc(cpu); if cpu.returned_to != 0x10750f6 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11296abu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.poll(); pc = 0x107504e; continue;
 }
 0x1075105 => {
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c96au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c96bu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10752b8 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { cpu.poll(); pc = 0x107504e; continue; }
  pc = 0x10752c2; continue;
 }
 0x10752c2 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x92u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x52u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0xfau16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  cpu.push(0x1075310); f_0105a869(cpu); if cpu.returned_to != 0x1075310 { return; }
  { cpu.popad(); }
  cpu.push(0x1075316); f_01075344(cpu); if cpu.returned_to != 0x1075316 { return; }
  if !cpu.sf { cpu.poll(); pc = 0x107504e; continue; }
  pc = 0x107531c; continue;
 }
 0x107531c => {
  { cpu.sub8(cpu.r8(0x112c96au32), 0x0u8); }
  if cpu.zf { pc = 0x107533a; continue; }
  pc = 0x1075325; continue;
 }
 0x1075325 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11296abu32, v); }
  cpu.push(0x1075331); f_010504cc(cpu); if cpu.returned_to != 0x1075331 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11296abu32, v); }
  pc = 0x107533f; continue;
 }
 0x107533a => {
  cpu.push(0x107533f); f_0107479d(cpu); if cpu.returned_to != 0x107533f { return; }
  pc = 0x107533f; continue;
 }
 0x107533f => {
  cpu.poll(); pc = 0x107504e; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075115(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075115); }
 let mut pc: u32 = 0x1075115;
 loop { match pc {
 0x1075115 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1075131); f_01072d00(cpu); if cpu.returned_to != 0x1075131 { return; }
  if cpu.cf { pc = 0x107516e; continue; }
  pc = 0x1075133; continue;
 }
 0x1075133 => {
  { cpu.sub8(cpu.r8(0x10f7dc8u32), 0x0u8); }
  if !cpu.zf { pc = 0x107516e; continue; }
  pc = 0x107513c; continue;
 }
 0x107513c => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x107515c; continue; }
  pc = 0x1075152; continue;
 }
 0x1075152 => {
  cpu.push(0x1075157); f_01012780(cpu); if cpu.returned_to != 0x1075157 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x107515c; continue;
 }
 0x107515c => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1075167); f_010356fe(cpu); if cpu.returned_to != 0x1075167 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10f7dc8u32, v); }
  pc = 0x107516e; continue;
 }
 0x107516e => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x98u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x22du16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x10ef342u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x10751cf; continue; }
  pc = 0x10751b8; continue;
 }
 0x10751b8 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x10751d4; continue; }
  pc = 0x10751c1; continue;
 }
 0x10751c1 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x10751d9; continue; }
  pc = 0x10751ca; continue;
 }
 0x10751ca => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x10751dc; continue;
 }
 0x10751cf => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  pc = 0x10751dc; continue;
 }
 0x10751d4 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  pc = 0x10751dc; continue;
 }
 0x10751d9 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  pc = 0x10751dc; continue;
 }
 0x10751dc => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x10751e4); f_0105aa11(cpu); if cpu.returned_to != 0x10751e4 { return; }
  { let v: u16 = 0x616u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1eu8); }
  if cpu.zf { pc = 0x1075291; continue; }
  pc = 0x1075202; continue;
 }
 0x1075202 => {
  { cpu.sub8((cpu.eax as u8), 0x22u8); }
  if cpu.zf { pc = 0x10752a3; continue; }
  pc = 0x107520a; continue;
 }
 0x107520a => {
  { cpu.sub8((cpu.eax as u8), 0x16u8); }
  if cpu.zf { pc = 0x10752b6; continue; }
  pc = 0x1075212; continue;
 }
 0x1075212 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x10752a5; continue; }
  pc = 0x107521a; continue;
 }
 0x107521a => {
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x1075240; continue; }
  pc = 0x107521e; continue;
 }
 0x107521e => {
  { cpu.sub8((cpu.eax as u8), 0x18u8); }
  if cpu.zf { pc = 0x107526d; continue; }
  pc = 0x1075222; continue;
 }
 0x1075222 => {
  { cpu.sub8((cpu.eax as u8), 0x19u8); }
  if cpu.zf { pc = 0x107527f; continue; }
  pc = 0x1075226; continue;
 }
 0x1075226 => {
  { cpu.sub8((cpu.eax as u8), 0x24u8); }
  if cpu.zf { pc = 0x1075252; continue; }
  pc = 0x107522a; continue;
 }
 0x107522a => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x1075264; continue; }
  pc = 0x107522e; continue;
 }
 0x107522e => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x10752b6; continue; }
  pc = 0x1075236; continue;
 }
 0x1075236 => {
  { cpu.sub8((cpu.eax as u8), 0x15u8); }
  if cpu.zf { pc = 0x10752b6; continue; }
  pc = 0x107523a; continue;
 }
 0x107523a => {
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if cpu.zf { pc = 0x10752b6; continue; }
  pc = 0x107523e; continue;
 }
 0x107523e => {
  pc = 0x10752b6; continue;
 }
 0x1075240 => {
  { let v: u16 = 0x162fu16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  pc = 0x10752a5; continue;
 }
 0x1075252 => {
  { let v: u16 = 0x16bau16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  pc = 0x10752a5; continue;
 }
 0x1075264 => {
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  pc = 0x10752a5; continue;
 }
 0x107526d => {
  { let v: u16 = 0x74du16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  pc = 0x10752a5; continue;
 }
 0x107527f => {
  { let v: u16 = 0x14d9u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  pc = 0x10752a5; continue;
 }
 0x1075291 => {
  { let v: u16 = 0x166bu16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x7u8; cpu.w8(0x10ef342u32, v); }
  pc = 0x10752a5; continue;
 }
 0x10752a3 => {
  pc = 0x10752b6; continue;
 }
 0x10752a5 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef32fu32, v); }
  cpu.push(0x10752b6); f_0105aa75(cpu); if cpu.returned_to != 0x10752b6 { return; }
  pc = 0x10752b6; continue;
 }
 0x10752b6 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075344(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075344); }
 let mut pc: u32 = 0x1075344;
 loop { match pc {
 0x1075344 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x94u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x14u8); }
  if !cpu.zf { pc = 0x10753b6; continue; }
  pc = 0x107535d; continue;
 }
 0x107535d => {
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) & 0xfdu8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x112c96bu32), 0x0u8); }
  if cpu.zf { pc = 0x1075389; continue; }
  pc = 0x107536d; continue;
 }
 0x107536d => {
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) | 0x2u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if !cpu.zf { pc = 0x1075389; continue; }
  pc = 0x107537a; continue;
 }
 0x107537a => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x2u16; cpu.w16(0x8eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10753b6; continue;
 }
 0x1075389 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x10753a4; continue; }
  pc = 0x1075391; continue;
 }
 0x1075391 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x107539a); f_010753f5(cpu); if cpu.returned_to != 0x107539a { return; }
  { let v: u16 = cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  if !cpu.sf { pc = 0x10753b6; continue; }
  pc = 0x10753a0; continue;
 }
 0x10753a0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.ebp), v); }
  pc = 0x10753a4; continue;
 }
 0x10753a4 => {
  { let v: u8 = cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.ebp)), ((cpu.edx >> 8) as u8)); cpu.w8(0x3au32.wrapping_add(cpu.ebp), v); }
  pc = 0x10753a7; continue;
 }
 0x10753a7 => {
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  cpu.push(0x10753af); f_01064250(cpu); if cpu.returned_to != 0x10753af { return; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10753b6 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x10753a7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010753bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10753bf); }
 let mut pc: u32 = 0x10753bf;
 loop { match pc {
 0x10753bf => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x14u8); }
  if !cpu.zf { pc = 0x10753ec; continue; }
  pc = 0x10753ce; continue;
 }
 0x10753ce => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x94u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.ebp)), ((cpu.edx >> 8) as u8)); cpu.w8(0x3au32.wrapping_add(cpu.ebp), v); }
  pc = 0x10753dc; continue;
 }
 0x10753dc => {
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  cpu.push(0x10753e4); f_01064250(cpu); if cpu.returned_to != 0x10753e4 { return; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10753ec => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x10753dc; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010753f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10753f5); }
 let mut pc: u32 = 0x10753f5;
 loop { match pc {
 0x10753f5 => {
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x107545d; continue; }
  pc = 0x10753fa; continue;
 }
 0x10753fa => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub((cpu.eax as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = 0x50u32; cpu.esi = v; }
  { cpu.sub16((cpu.ebp as u16), (cpu.ebx as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1075431; continue; }
  pc = 0x1075427; continue;
 }
 0x1075427 => {
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.eax as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), (cpu.ebx as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x107545e; continue; }
  pc = 0x107542f; continue;
 }
 0x107542f => {
  pc = 0x1075457; continue;
 }
 0x1075431 => {
  { let v: u32 = 0x55u32; cpu.esi = v; }
  { cpu.sub16((cpu.ebp as u16), (cpu.ecx as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1075445; continue; }
  pc = 0x107543b; continue;
 }
 0x107543b => {
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.eax as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), (cpu.ecx as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x107545e; continue; }
  pc = 0x1075443; continue;
 }
 0x1075443 => {
  pc = 0x1075457; continue;
 }
 0x1075445 => {
  { let v: u32 = 0x5au32; cpu.esi = v; }
  { cpu.sub16((cpu.ebp as u16), (cpu.edx as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1075457; continue; }
  pc = 0x107544f; continue;
 }
 0x107544f => {
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.eax as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), (cpu.edx as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x107545e; continue; }
  pc = 0x1075457; continue;
 }
 0x1075457 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x107545d; continue;
 }
 0x107545d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x107545e => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v = (cpu.ecx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v = (cpu.ecx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x107548d; continue; }
  pc = 0x1075487; continue;
 }
 0x1075487 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x16u8); }
  if !cpu.zf { pc = 0x1075490; continue; }
  pc = 0x107548d; continue;
 }
 0x107548d => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x20u32.wrapping_add(cpu.eax), v); }
  pc = 0x1075490; continue;
 }
 0x1075490 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v = (cpu.ecx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v = (cpu.ecx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1075457; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010754a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10754a6); }
 let mut pc: u32 = 0x10754a6;
 loop { match pc {
 0x10754a6 => {
  { let v: u32 = 0x10f7625u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x10754b5; continue;
 }
 0x10754b5 => {
  { cpu.stos(4, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10754b5; continue; }
  pc = 0x10754b8; continue;
 }
 0x10754b8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x10f7625u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10f7825u32); cpu.esi = v; }
  pc = 0x10754c5; continue;
 }
 0x10754c5 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x10754d3; continue; }
  pc = 0x10754cd; continue;
 }
 0x10754cd => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  cpu.poll(); pc = 0x10754c5; continue;
 }
 0x10754d3 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x10754e1; continue; }
  pc = 0x10754db; continue;
 }
 0x10754db => {
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  cpu.poll(); pc = 0x10754d3; continue;
 }
 0x10754e1 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x10754ef; continue; }
  pc = 0x10754e9; continue;
 }
 0x10754e9 => {
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  cpu.poll(); pc = 0x10754e1; continue;
 }
 0x10754ef => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010754f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10754f0); }
 let mut pc: u32 = 0x10754f0;
 loop { match pc {
 0x10754f0 => {
  cpu.push(0x10754f5); f_010754f6(cpu); if cpu.returned_to != 0x10754f5 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010754f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10754f6); }
 let mut pc: u32 = 0x10754f6;
 loop { match pc {
 0x10754f6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x10754fe); f_01075512(cpu); if cpu.returned_to != 0x10754fe { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x1075504); f_01075512(cpu); if cpu.returned_to != 0x1075504 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x107550a); f_01075512(cpu); if cpu.returned_to != 0x107550a { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x1075510); f_01075512(cpu); if cpu.returned_to != 0x1075510 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075512(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075512); }
 let mut pc: u32 = 0x1075512;
 loop { match pc {
 0x1075512 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x11c5b2cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075548(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075548); }
 let mut pc: u32 = 0x1075548;
 loop { match pc {
 0x1075548 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef327u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x11dc81cu32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4078u32, v); }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x11dcd9cu32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e407cu32, v); }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x11dd31cu32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4080u32, v); }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x11dd89cu32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4084u32, v); }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33du32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010755cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10755cf); }
 let mut pc: u32 = 0x10755cf;
 loop { match pc {
 0x10755cf => {
  { cpu.sub16(cpu.r16(0x1124e8au32), 0x0u16); }
  if cpu.zf { pc = 0x10755e0; continue; }
  pc = 0x10755d9; continue;
 }
 0x10755d9 => {
  { let v: u8 = 0x6u8; cpu.w8(0x10ef328u32, v); }
  pc = 0x10755e0; continue;
 }
 0x10755e0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010755e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10755e1); }
 let mut pc: u32 = 0x10755e1;
 loop { match pc {
 0x10755e1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fd3e9u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd3e8u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10b032fu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11dde24u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee91eu32, v); }
  cpu.push(0x1075611); f_010754f0(cpu); if cpu.returned_to != 0x1075611 { return; }
  { let v: u16 = 0x37u16; cpu.w16(0x10e3e5fu32, v); }
  { let v: u16 = 0xb1u16; cpu.w16(0x10e3e61u32, v); }
  { let v: u32 = 0x1eu32; cpu.w32(0x10e3fdcu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11b4dceu32, v); }
  cpu.push(0x1075639); f_01037ef8(cpu); if cpu.returned_to != 0x1075639 { return; }
  if cpu.cf { pc = 0x1075864; continue; }
  pc = 0x107563f; continue;
 }
 0x107563f => {
  { let v: u32 = 0x78u32; cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef322u32, v); }
  pc = 0x107564b; continue;
 }
 0x107564b => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10ef322u32)); cpu.w8(0x10ef322u32, v); }
  if cpu.sf { pc = 0x1075662; continue; }
  pc = 0x1075653; continue;
 }
 0x1075653 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x107565f); cpu.call(t); if cpu.returned_to != 0x107565f { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.poll(); pc = 0x107564b; continue;
 }
 0x1075662 => {
  { let v: u8 = 0x3u8; cpu.w8(0x10ef322u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.pushad(); }
  cpu.push(0x1075670); f_01042066(cpu); if cpu.returned_to != 0x1075670 { return; }
  { cpu.popad(); }
  cpu.push(0x1075676); f_01059ae3(cpu); if cpu.returned_to != 0x1075676 { return; }
  { cpu.pushad(); }
  { cpu.popad(); }
  cpu.push(0x107567d); f_0105eb1d(cpu); if cpu.returned_to != 0x107567d { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x1075689; continue; }
  pc = 0x1075680; continue;
 }
 0x1075680 => {
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if !cpu.zf { pc = 0x1075689; continue; }
  pc = 0x1075684; continue;
 }
 0x1075684 => {
  pc = 0x107584b; continue;
 }
 0x1075689 => {
  { cpu.sub8(cpu.r8(0x10fd3e8u32), 0x0u8); }
  if !cpu.zf { pc = 0x107584b; continue; }
  pc = 0x1075696; continue;
 }
 0x1075696 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x112c98au32; cpu.esi = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee91eu32, v); }
  cpu.push(0x10756ad); f_01037d64(cpu); if cpu.returned_to != 0x10756ad { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee91eu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x10756c2; continue; }
  pc = 0x10756bd; continue;
 }
 0x10756bd => {
  cpu.push(0x10756c2); f_01059985(cpu); if cpu.returned_to != 0x10756c2 { return; }
  pc = 0x10756c2; continue;
 }
 0x10756c2 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x10756cd); cpu.call(t); if cpu.returned_to != 0x10756cd { return; }
  { cpu.sub8(cpu.r8(0x112c97cu32), 0x2u8); }
  if cpu.cf { pc = 0x10756e7; continue; }
  pc = 0x10756d6; continue;
 }
 0x10756d6 => {
  { let v: u32 = cpu.r8(0x11b4dceu32) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10e3d7fu32)); }
  if !cpu.zf { pc = 0x10756fa; continue; }
  pc = 0x10756e5; continue;
 }
 0x10756e5 => {
  pc = 0x1075709; continue;
 }
 0x10756e7 => {
  { let v: u32 = 0x112c98au32; cpu.esi = v; }
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x10756f8); f_01037ba3(cpu); if cpu.returned_to != 0x10756f8 { return; }
  if cpu.cf { pc = 0x1075709; continue; }
  pc = 0x10756fa; continue;
 }
 0x10756fa => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = cpu.dec16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x107564b; continue; }
  pc = 0x1075703; continue;
 }
 0x1075703 => {
  pc = 0x1075864; continue;
 }
 0x1075709 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee91eu32, v); }
  { cpu.sub8(cpu.r8(0x112c97cu32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x107580c; continue; }
  pc = 0x107571e; continue;
 }
 0x107571e => {
  { let v: u32 = 0x15eu32; cpu.ecx = v; }
  { let v: u32 = 0x10c6878u32; cpu.esi = v; }
  cpu.push(0x107572d); f_01075770(cpu); if cpu.returned_to != 0x107572d { return; }
  cpu.push(0x1075732); f_01075770(cpu); if cpu.returned_to != 0x1075732 { return; }
  pc = 0x1075732; continue;
 }
 0x1075732 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xfu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x107574a; continue; }
  pc = 0x1075739; continue;
 }
 0x1075739 => {
  cpu.push(0x107573e); f_01075770(cpu); if cpu.returned_to != 0x107573e { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x112c98au32; cpu.esi = v; }
  cpu.push(0x1075749); f_01075770(cpu); if cpu.returned_to != 0x1075749 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x107574a; continue;
 }
 0x107574a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1075751); f_01045e5f(cpu); if cpu.returned_to != 0x1075751 { return; }
  let t = cpu.r32(0x10e4350u32); cpu.push(0x1075757); cpu.call(t); if cpu.returned_to != 0x1075757 { return; }
  let t = cpu.r32(0x10e4340u32); cpu.push(0x107575d); cpu.call(t); if cpu.returned_to != 0x107575d { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x1124ee4u32), 0x0u8); }
  if !cpu.zf { pc = 0x107578e; continue; }
  pc = 0x1075768; continue;
 }
 0x1075768 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1075732; continue; }
  pc = 0x107576b; continue;
 }
 0x107576b => {
  pc = 0x1075864; continue;
 }
 0x107578e => {
  cpu.push(0x1075793); f_01075770(cpu); if cpu.returned_to != 0x1075793 { return; }
  cpu.push(0x1075798); f_01075770(cpu); if cpu.returned_to != 0x1075798 { return; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x10757a2); f_01039358(cpu); if cpu.returned_to != 0x10757a2 { return; }
  pc = 0x10757a2; continue;
 }
 0x10757a2 => {
  cpu.push(0x10757a7); f_01037576(cpu); if cpu.returned_to != 0x10757a7 { return; }
  { let v: u32 = 0x5u32; cpu.eax = v; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x10757b2); cpu.call(t); if cpu.returned_to != 0x10757b2 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x10b0330u32.wrapping_add(cpu.ebp)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10757a2; continue; }
  pc = 0x10757c3; continue;
 }
 0x10757c3 => {
  { let v: u32 = 0x14u32; cpu.eax = v; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x10757ce); cpu.call(t); if cpu.returned_to != 0x10757ce { return; }
  { cpu.interrupts_off = true; }
  cpu.push(0x10757d4); f_01037576(cpu); if cpu.returned_to != 0x10757d4 { return; }
  cpu.push(0x10757d9); f_01084fb7(cpu); if cpu.returned_to != 0x10757d9 { return; }
  cpu.push(0x10757de); f_0103938c(cpu); if cpu.returned_to != 0x10757de { return; }
  cpu.push(0x10757e3); f_010754f0(cpu); if cpu.returned_to != 0x10757e3 { return; }
  { cpu.interrupts_off = false; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.ebp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10b032fu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11dde24u32, v); }
  { let v: u32 = 0x14u32; cpu.eax = v; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x1075805); cpu.call(t); if cpu.returned_to != 0x1075805 { return; }
  cpu.push(0x107580a); f_010391e8(cpu); if cpu.returned_to != 0x107580a { return; }
  if cpu.cf { pc = 0x1075864; continue; }
  pc = 0x107580c; continue;
 }
 0x107580c => {
  { let v: u32 = 0x3020100u32; cpu.w32(0x10ee950u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3dd5u32, v); }
  { let v: u32 = 0x11dc39cu32; cpu.w32(0x1124eacu32, v); }
  { let v: u32 = 0x11dc39cu32; cpu.w32(0x1124ec4u32, v); }
  { cpu.sub8(cpu.r8(0x112c97cu32), 0x2u8); }
  if !cpu.zf { pc = 0x1075848; continue; }
  pc = 0x107583c; continue;
 }
 0x107583c => {
  { let v: u32 = cpu.r32(0x1131054u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1105873u32, v); }
  pc = 0x1075848; continue;
 }
 0x1075848 => {
  { cpu.cf = false; }
  pc = 0x107588c; continue;
 }
 0x107584b => {
  { let v: u8 = 0x2u8; cpu.w8(0x10fd3eau32, v); }
  { cpu.pushad(); }
  cpu.push(0x1075858); f_01042066(cpu); if cpu.returned_to != 0x1075858 { return; }
  { cpu.popad(); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x1075864); cpu.call(t); if cpu.returned_to != 0x1075864 { return; }
  pc = 0x1075864; continue;
 }
 0x1075864 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee91eu32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10b032fu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11dde24u32, v); }
  cpu.push(0x1075886); f_01038580(cpu); if cpu.returned_to != 0x1075886 { return; }
  cpu.push(0x107588b); f_01037e00(cpu); if cpu.returned_to != 0x107588b { return; }
  { cpu.cf = true; }
  pc = 0x107588c; continue;
 }
 0x107588c => {
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x1075893); f_010754f0(cpu); if cpu.returned_to != 0x1075893 { return; }
  { let v = cpu.r16(cpu.esp) as u32; cpu.esp = cpu.esp.wrapping_add(2); let h = cpu.eflags_high; cpu.set_flags(v); cpu.eflags_high = h; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075770); }
 let mut pc: u32 = 0x1075770;
 loop { match pc {
 0x1075770 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0xffu32; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x107578b); f_01037d64(cpu); if cpu.returned_to != 0x107578b { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075897(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075897); }
 let mut pc: u32 = 0x1075897;
 loop { match pc {
 0x1075897 => {
  { cpu.pushad(); }
  { let v: u32 = 0x10fd47fu32; cpu.edi = v; }
  { let v: u32 = 0x10fd374u32; cpu.esi = v; }
  pc = 0x10758a2; continue;
 }
 0x10758a2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10758ae; continue; }
  pc = 0x10758a8; continue;
 }
 0x10758a8 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10758a2; continue;
 }
 0x10758ae => {
  { let v: u32 = 0x110777eu32; cpu.esi = v; }
  pc = 0x10758b3; continue;
 }
 0x10758b3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10758bf; continue; }
  pc = 0x10758b9; continue;
 }
 0x10758b9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10758b3; continue;
 }
 0x10758bf => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10fcb43u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1131398u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x10758ce; continue;
 }
 0x10758ce => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10758da; continue; }
  pc = 0x10758d4; continue;
 }
 0x10758d4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10758ce; continue;
 }
 0x10758da => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010758df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10758df); }
 let mut pc: u32 = 0x10758df;
 loop { match pc {
 0x10758df => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x10758e3; continue;
 }
 0x10758e3 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x10758f1; continue; }
  pc = 0x10758e9; continue;
 }
 0x10758e9 => {
  if cpu.sf { pc = 0x10758f1; continue; }
  pc = 0x10758eb; continue;
 }
 0x10758eb => {
  { let v: u16 = (cpu.ebp as u16).wrapping_sub(0x4u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x10758e3; continue;
 }
 0x10758f1 => {
  { let v: u16 = cpu.and16((cpu.ebp as u16), 0xfff8u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010758fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10758fb); }
 let mut pc: u32 = 0x10758fb;
 loop { match pc {
 0x10758fb => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x112ca23u32; cpu.edi = v; }
  { let v: u32 = 0xbu32; cpu.ecx = v; }
  pc = 0x1075907; continue;
 }
 0x1075907 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1075913; continue; }
  pc = 0x107590d; continue;
 }
 0x107590d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1075907; continue; }
  pc = 0x1075913; continue;
 }
 0x1075913 => {
  { let v: u32 = cpu.r32(0x1131074u32); cpu.esi = v; }
  pc = 0x1075919; continue;
 }
 0x1075919 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1075919; continue; }
  pc = 0x1075923; continue;
 }
 0x1075923 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075928(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075928); }
 let mut pc: u32 = 0x1075928;
 loop { match pc {
 0x1075928 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075929(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075929); }
 let mut pc: u32 = 0x1075929;
 loop { match pc {
 0x1075929 => {
  { cpu.sub8(cpu.r8(0x110569bu32), 0x0u8); }
  if cpu.zf { pc = 0x10759a9; continue; }
  pc = 0x1075932; continue;
 }
 0x1075932 => {
  { let v: u16 = cpu.r16(0x110569cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x107599a; continue; }
  pc = 0x1075945; continue;
 }
 0x1075945 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x107599a; continue; }
  pc = 0x1075952; continue;
 }
 0x1075952 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110569eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xffd0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x107599a; continue; }
  pc = 0x107596a; continue;
 }
 0x107596a => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x107599a; continue; }
  pc = 0x1075977; continue;
 }
 0x1075977 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x110569bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x110567du32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x10759aa; continue; }
  pc = 0x107598e; continue;
 }
 0x107598e => {
  { let v: u32 = cpu.ecx.wrapping_add(0x178u32); cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x107599a); cpu.call(t); if cpu.returned_to != 0x107599a { return; }
  pc = 0x107599a; continue;
 }
 0x107599a => {
  { cpu.and8(cpu.r8(0x10e3d07u32), 0x1u8); }
  if cpu.zf { pc = 0x10759a9; continue; }
  pc = 0x10759a3; continue;
 }
 0x10759a3 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x110569bu32)); cpu.w8(0x110569bu32, v); }
  pc = 0x10759a9; continue;
 }
 0x10759a9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10759aa => {
  { let v: u8 = 0x0u8; cpu.w8(0x110569bu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010759b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10759b2); }
 let mut pc: u32 = 0x10759b2;
 loop { match pc {
 0x10759b2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110569cu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x110569eu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110569bu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010759c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10759c7); }
 let mut pc: u32 = 0x10759c7;
 loop { match pc {
 0x10759c7 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x110567au32, v); }
  { cpu.pushad(); }
  { let v: u32 = 0x1172f54u32; cpu.edi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  pc = 0x10759d9; continue;
 }
 0x10759d9 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x1075a0d; continue; }
  pc = 0x10759e0; continue;
 }
 0x10759e0 => {
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
  if (!cpu.cf && !cpu.zf) { pc = 0x1075a0d; continue; }
  pc = 0x1075a0b; continue;
 }
 0x1075a0b => {
  pc = 0x1075a15; continue;
 }
 0x1075a0d => {
  { let v: u32 = cpu.add32(cpu.edi, 0x40u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10759d9; continue; }
  pc = 0x1075a12; continue;
 }
 0x1075a12 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1075a15 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075a18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075a18); }
 let mut pc: u32 = 0x1075a18;
 loop { match pc {
 0x1075a18 => {
  { let v: u32 = 0x10fd65fu32; cpu.edi = v; }
  { let v: u32 = 0x2000u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x10fd65fu32; cpu.esi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x110565fu32, v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e690eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1075a3f; continue;
 }
 0x1075a3f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e690cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1075a49; continue;
 }
 0x1075a49 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ea910u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1075a7e; continue; }
  pc = 0x1075a72; continue;
 }
 0x1075a72 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1075a7e; continue; }
  pc = 0x1075a76; continue;
 }
 0x1075a76 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x1075aaa; continue; }
  pc = 0x1075a7a; continue;
 }
 0x1075a7a => {
  { cpu.sub8((cpu.eax as u8), 0x28u8); }
  if cpu.zf { pc = 0x1075a7e; continue; }
  pc = 0x1075a7e; continue;
 }
 0x1075a7e => {
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u16 = cpu.inc16(cpu.r16(0x110565fu32)); cpu.w16(0x110565fu32, v); }
  { cpu.sub16(cpu.r16(0x110565fu32), 0x3fffu16); }
  if (cpu.sf != cpu.of) { pc = 0x1075aaa; continue; }
  pc = 0x1075aa5; continue;
 }
 0x1075aa5 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1075ab3; continue;
 }
 0x1075aaa => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.sf { cpu.poll(); pc = 0x1075a49; continue; }
  pc = 0x1075ab0; continue;
 }
 0x1075ab0 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.sf { cpu.poll(); pc = 0x1075a3f; continue; }
  pc = 0x1075ab3; continue;
 }
 0x1075ab3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075ab4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075ab4); }
 let mut pc: u32 = 0x1075ab4;
 loop { match pc {
 0x1075ab4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1075abd); f_0104f36d(cpu); if cpu.returned_to != 0x1075abd { return; }
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
pub fn f_01075ad4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075ad4); }
 let mut pc: u32 = 0x1075ad4;
 loop { match pc {
 0x1075ad4 => {
  cpu.push(0x1075ad9); f_0105f215(cpu); if cpu.returned_to != 0x1075ad9 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u16 = 0x14u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1075ae3; continue;
 }
 0x1075ae3 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1075ae9); f_0105f215(cpu); if cpu.returned_to != 0x1075ae9 { return; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  cpu.push(0x1075af7); f_0105f215(cpu); if cpu.returned_to != 0x1075af7 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x1075b02); f_0105f215(cpu); if cpu.returned_to != 0x1075b02 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1075b0e); f_0105f215(cpu); if cpu.returned_to != 0x1075b0e { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x2u32); }
  if !cpu.zf { pc = 0x1075b2e; continue; }
  pc = 0x1075b2c; continue;
 }
 0x1075b2c => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1075b2e; continue;
 }
 0x1075b2e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x1075b36); f_01077d60(cpu); if cpu.returned_to != 0x1075b36 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1075ae3; continue; }
  pc = 0x1075b39; continue;
 }
 0x1075b39 => {
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x1075b43; continue;
 }
 0x1075b43 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x1075b83; continue; }
  pc = 0x1075b4c; continue;
 }
 0x1075b4c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1075b58); f_01075c55(cpu); if cpu.returned_to != 0x1075b58 { return; }
  if cpu.cf { pc = 0x1075b89; continue; }
  pc = 0x1075b5a; continue;
 }
 0x1075b5a => {
  { cpu.sub16((cpu.eax as u16), 0x8edu16); }
  if !cpu.zf { pc = 0x1075b83; continue; }
  pc = 0x1075b60; continue;
 }
 0x1075b60 => {
  { cpu.pushad(); }
  { let v: u16 = 0x14u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x50u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1075b6e); f_0105f2ec(cpu); if cpu.returned_to != 0x1075b6e { return; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1075b82); f_01076074(cpu); if cpu.returned_to != 0x1075b82 { return; }
  { cpu.popad(); }
  pc = 0x1075b83; continue;
 }
 0x1075b83 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1075b43; continue; }
  pc = 0x1075b88; continue;
 }
 0x1075b88 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1075b89 => {
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
  { let v: u32 = cpu.eax.wrapping_sub(0x11a188cu32); cpu.eax = v; }
  { let v: u8 = 0x28u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1075baf); f_010770a8(cpu); if cpu.returned_to != 0x1075baf { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x112cadau32, v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  pc = 0x1075bc0; continue;
 }
 0x1075bc0 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub32(cpu.ecx, 0x3u32); }
  if (cpu.sf == cpu.of) { pc = 0x1075bdf; continue; }
  pc = 0x1075bc9; continue;
 }
 0x1075bc9 => {
  cpu.push(0x1075bce); f_0105f215(cpu); if cpu.returned_to != 0x1075bce { return; }
  { let v: u16 = cpu.r16(0x112cadau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.and16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1075bdf; continue; }
  pc = 0x1075bda; continue;
 }
 0x1075bda => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1075c46; continue;
 }
 0x1075bdf => {
  { let v: u16 = cpu.r16(0x112cadau32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112cadau32, v); }
  { let v: u16 = 0x20u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2au16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1075bff); f_0105f2ec(cpu); if cpu.returned_to != 0x1075bff { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x2u16; cpu.w16(0x10e3e93u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x112cae1u32; cpu.edi = v; }
  cpu.push(0x1075c1c); f_0105f215(cpu); if cpu.returned_to != 0x1075c1c { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = 0xff02u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x80u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x6u16; cpu.w16(0x10e3e97u32, v); }
  cpu.push(0x1075c41); f_01075ab4(cpu); if cpu.returned_to != 0x1075c41 { return; }
  cpu.push(0x1075c46); f_01075c8b(cpu); if cpu.returned_to != 0x1075c46 { return; }
  pc = 0x1075c46; continue;
 }
 0x1075c46 => {
  { cpu.popad(); }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1075bc0; continue; }
  pc = 0x1075c4f; continue;
 }
 0x1075c4f => {
  { cpu.popad(); }
  cpu.poll(); pc = 0x1075b83; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075c55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075c55); }
 let mut pc: u32 = 0x1075c55;
 loop { match pc {
 0x1075c55 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x112cb11u32; cpu.edi = v; }
  { let v: u16 = 0x10u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1075c60; continue;
 }
 0x1075c60 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.edi))); }
  if cpu.zf { pc = 0x1075c6e; continue; }
  pc = 0x1075c65; continue;
 }
 0x1075c65 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1075c60; continue; }
  pc = 0x1075c6a; continue;
 }
 0x1075c6a => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1075c6e => {
  { let v: u16 = 0x10u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e99u32, v); }
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075c8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075c8b); }
 let mut pc: u32 = 0x1075c8b;
 loop { match pc {
 0x1075c8b => {
  { cpu.pushad(); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x10e3f17u32), 0x0u16); }
  if cpu.zf { pc = 0x1075e0b; continue; }
  pc = 0x1075c9b; continue;
 }
 0x1075c9b => {
  cpu.push(0x1075ca0); f_010770c0(cpu); if cpu.returned_to != 0x1075ca0 { return; }
  if cpu.cf { pc = 0x1075e17; continue; }
  pc = 0x1075ca6; continue;
 }
 0x1075ca6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11b5185u32, v); }
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x1075cbd; continue; }
  pc = 0x1075cb3; continue;
 }
 0x1075cb3 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x11b5185u32, v); }
  pc = 0x1075cbd; continue;
 }
 0x1075cbd => {
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.sf { pc = 0x1075cc6; continue; }
  pc = 0x1075cc3; continue;
 }
 0x1075cc3 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1075cc6; continue;
 }
 0x1075cc6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x27u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2du32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x10e3e97u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x34u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x2fu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x35u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x36u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x37u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x112cc15u32; cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
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
  if cpu.zf { pc = 0x1075d53; continue; }
  pc = 0x1075d4f; continue;
 }
 0x1075d4f => {
  { let v: u8 = 0x1u8; cpu.w8(0x37u32.wrapping_add(cpu.edi), v); }
  pc = 0x1075d53; continue;
 }
 0x1075d53 => {
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
  { cpu.sub8(cpu.r8(0x11b5185u32), 0x0u8); }
  if cpu.zf { pc = 0x1075db3; continue; }
  pc = 0x1075daf; continue;
 }
 0x1075daf => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.edi)) | 0x10u8; cpu.w8(0x35u32.wrapping_add(cpu.edi), v); }
  pc = 0x1075db3; continue;
 }
 0x1075db3 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x25u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x38u32.wrapping_add(cpu.edi), v); }
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.sf { pc = 0x1075dca; continue; }
  pc = 0x1075dbf; continue;
 }
 0x1075dbf => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.edi)) | 0x20u8; cpu.w8(0x35u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x25u32.wrapping_add(cpu.edi), v); }
  pc = 0x1075dca; continue;
 }
 0x1075dca => {
  { let v: u8 = 0x0u8; cpu.w8(0x2eu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x39u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2fu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x31u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x30u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x33u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x1075e08; continue; }
  pc = 0x1075de9; continue;
 }
 0x1075de9 => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.edi)) | 0x44u8; cpu.w8(0x35u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7eu16); }
  if cpu.zf { pc = 0x1075e08; continue; }
  pc = 0x1075df6; continue;
 }
 0x1075df6 => {
  { cpu.sub16((cpu.eax as u16), 0x7du16); }
  if cpu.zf { pc = 0x1075e08; continue; }
  pc = 0x1075dfc; continue;
 }
 0x1075dfc => {
  { cpu.sub16((cpu.eax as u16), 0x7cu16); }
  if cpu.zf { pc = 0x1075e08; continue; }
  pc = 0x1075e02; continue;
 }
 0x1075e02 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x2u32); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x33u32.wrapping_add(cpu.edi), v); }
  pc = 0x1075e08; continue;
 }
 0x1075e08 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1075e0b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1075e17 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075e0e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075e0e); }
 let mut pc: u32 = 0x1075e0e;
 loop { match pc {
 0x1075e0e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075e1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075e1a); }
 let mut pc: u32 = 0x1075e1a;
 loop { match pc {
 0x1075e1a => {
  { cpu.sub16(cpu.r16(0x110565fu32), 0x0u16); }
  if !cpu.zf { pc = 0x1075e25; continue; }
  pc = 0x1075e24; continue;
 }
 0x1075e24 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1075e25 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x10e3f19u32), 0x0u16); }
  if cpu.zf { pc = 0x1075f8c; continue; }
  pc = 0x1075e37; continue;
 }
 0x1075e37 => {
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = 0x10c2402u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e40c0u32, v); }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x64u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1075e5c); f_0105f2ec(cpu); if cpu.returned_to != 0x1075e5c { return; }
  { cpu.sub16((cpu.eax as u16), 0xau16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1075f8c; continue; }
  pc = 0x1075e66; continue;
 }
 0x1075e66 => {
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110565fu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1075e79); f_0105f2ec(cpu); if cpu.returned_to != 0x1075e79 { return; }
  { cpu.sub16(cpu.r16(0x110565fu32), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x1075e66; continue; }
  pc = 0x1075e82; continue;
 }
 0x1075e82 => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10fd660u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10fd65fu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u16 = 0x20u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1075eaa); f_010759c7(cpu); if cpu.returned_to != 0x1075eaa { return; }
  if cpu.cf { pc = 0x1075f8c; continue; }
  pc = 0x1075eb0; continue;
 }
 0x1075eb0 => {
  { cpu.sub8(cpu.r8(0x10e40c0u32), 0x4u8); }
  if cpu.zf { pc = 0x1075f09; continue; }
  pc = 0x1075eb9; continue;
 }
 0x1075eb9 => {
  { cpu.sub8(cpu.r8(0x10e40c0u32), 0x3u8); }
  if cpu.zf { pc = 0x1075eff; continue; }
  pc = 0x1075ec2; continue;
 }
 0x1075ec2 => {
  { cpu.sub8(cpu.r8(0x10e40c0u32), 0x1u8); }
  if cpu.zf { pc = 0x1075f66; continue; }
  pc = 0x1075ecf; continue;
 }
 0x1075ecf => {
  { cpu.sub8(cpu.r8(0x10e40c0u32), 0x0u8); }
  if !cpu.zf { pc = 0x1075f3e; continue; }
  pc = 0x1075ed8; continue;
 }
 0x1075ed8 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1075ede); f_0105f215(cpu); if cpu.returned_to != 0x1075ede { return; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16((cpu.ecx as u16), 0x2u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1075ef5; continue; }
  pc = 0x1075eeb; continue;
 }
 0x1075eeb => {
  cpu.push(0x1075ef0); f_01076d88(cpu); if cpu.returned_to != 0x1075ef0 { return; }
  pc = 0x1075f8c; continue;
 }
 0x1075ef5 => {
  cpu.push(0x1075efa); f_01076d1e(cpu); if cpu.returned_to != 0x1075efa { return; }
  pc = 0x1075f8c; continue;
 }
 0x1075eff => {
  cpu.push(0x1075f04); f_01075f91(cpu); if cpu.returned_to != 0x1075f04 { return; }
  pc = 0x1075f8c; continue;
 }
 0x1075f09 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1075f0f); f_0105f215(cpu); if cpu.returned_to != 0x1075f0f { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1075f1b); f_0105f215(cpu); if cpu.returned_to != 0x1075f1b { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1075f36; continue; }
  pc = 0x1075f21; continue;
 }
 0x1075f21 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = 0x40u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1075f2c); f_010759c7(cpu); if cpu.returned_to != 0x1075f2c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x1075f8c; continue; }
  pc = 0x1075f2f; continue;
 }
 0x1075f2f => {
  cpu.push(0x1075f34); f_01076574(cpu); if cpu.returned_to != 0x1075f34 { return; }
  pc = 0x1075f8c; continue;
 }
 0x1075f36 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1075f3c); f_010766fb(cpu); if cpu.returned_to != 0x1075f3c { return; }
  pc = 0x1075f8c; continue;
 }
 0x1075f3e => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1075f44); f_0105f215(cpu); if cpu.returned_to != 0x1075f44 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1075f50); f_0105f215(cpu); if cpu.returned_to != 0x1075f50 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1075f5e; continue; }
  pc = 0x1075f56; continue;
 }
 0x1075f56 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1075f5c); f_01076c34(cpu); if cpu.returned_to != 0x1075f5c { return; }
  pc = 0x1075f8c; continue;
 }
 0x1075f5e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1075f64); f_01076923(cpu); if cpu.returned_to != 0x1075f64 { return; }
  pc = 0x1075f8c; continue;
 }
 0x1075f66 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1075f6c); f_0105f215(cpu); if cpu.returned_to != 0x1075f6c { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1075f78); f_0105f215(cpu); if cpu.returned_to != 0x1075f78 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1075f86; continue; }
  pc = 0x1075f7e; continue;
 }
 0x1075f7e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1075f84); f_0107648a(cpu); if cpu.returned_to != 0x1075f84 { return; }
  pc = 0x1075f8c; continue;
 }
 0x1075f86 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1075f8c); f_01076179(cpu); if cpu.returned_to != 0x1075f8c { return; }
  pc = 0x1075f8c; continue;
 }
 0x1075f8c => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075f91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075f91); }
 let mut pc: u32 = 0x1075f91;
 loop { match pc {
 0x1075f91 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1075f98); f_01077026(cpu); if cpu.returned_to != 0x1075f98 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1075fcd; continue; }
  pc = 0x1075f9c; continue;
 }
 0x1075f9c => {
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = 0xa6au16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x7u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e99u32, v); }
  cpu.push(0x1075fcd); f_0101c3ab(cpu); if cpu.returned_to != 0x1075fcd { return; }
  pc = 0x1075fcd; continue;
 }
 0x1075fcd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01075fce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1075fce); }
 let mut pc: u32 = 0x1075fce;
 loop { match pc {
 0x1075fce => {
  cpu.push(0x1075fd3); f_01072d00(cpu); if cpu.returned_to != 0x1075fd3 { return; }
  if cpu.cf { pc = 0x107602f; continue; }
  pc = 0x1075fd5; continue;
 }
 0x1075fd5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1075ff6; continue; }
  pc = 0x1075fec; continue;
 }
 0x1075fec => {
  cpu.push(0x1075ff1); f_01012780(cpu); if cpu.returned_to != 0x1075ff1 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1075ff6; continue;
 }
 0x1075ff6 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1076001); f_010356fe(cpu); if cpu.returned_to != 0x1076001 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x73u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1076022; continue; }
  pc = 0x1076018; continue;
 }
 0x1076018 => {
  cpu.push(0x107601d); f_01012780(cpu); if cpu.returned_to != 0x107601d { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1076022; continue;
 }
 0x1076022 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x107602e); f_010356fe(cpu); if cpu.returned_to != 0x107602e { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x107602f; continue;
 }
 0x107602f => {
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef342u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3ebfu32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  cpu.push(0x1076072); f_0105b2b7(cpu); if cpu.returned_to != 0x1076072 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076074(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076074); }
 let mut pc: u32 = 0x1076074;
 loop { match pc {
 0x1076074 => {
  cpu.push(0x1076079); f_01076feb(cpu); if cpu.returned_to != 0x1076079 { return; }
  if cpu.cf { pc = 0x10760b2; continue; }
  pc = 0x107607b; continue;
 }
 0x107607b => {
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.and16((cpu.ebx as u16), 0xfff0u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  { let v: u32 = 0x10760b3u32; cpu.esi = v; }
  cpu.push(0x10760b2); f_01075c8b(cpu); if cpu.returned_to != 0x10760b2 { return; }
  pc = 0x10760b2; continue;
 }
 0x10760b2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010760b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10760b3); }
 let mut pc: u32 = 0x10760b3;
 loop { match pc {
 0x10760b3 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0xff05u32; cpu.ecx = v; }
  { let v: u8 = 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1076147u32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x10760e5); f_01075c8b(cpu); if cpu.returned_to != 0x10760e5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x5u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x100cu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1076115u32; cpu.esi = v; }
  { let v: u8 = 0x5u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076113); f_01075c8b(cpu); if cpu.returned_to != 0x1076113 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076115(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076115); }
 let mut pc: u32 = 0x1076115;
 loop { match pc {
 0x1076115 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076145); f_01075c8b(cpu); if cpu.returned_to != 0x1076145 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076147(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076147); }
 let mut pc: u32 = 0x1076147;
 loop { match pc {
 0x1076147 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0xeu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076177); f_01075c8b(cpu); if cpu.returned_to != 0x1076177 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076179(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076179); }
 let mut pc: u32 = 0x1076179;
 loop { match pc {
 0x1076179 => {
  { cpu.pushad(); }
  cpu.push(0x107617f); f_01077015(cpu); if cpu.returned_to != 0x107617f { return; }
  if cpu.cf { pc = 0x10761f6; continue; }
  pc = 0x1076181; continue;
 }
 0x1076181 => {
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x7u32); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x10761c3); f_01075c8b(cpu); if cpu.returned_to != 0x10761c3 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x24u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x10761f8u32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x10761f6); f_01075c8b(cpu); if cpu.returned_to != 0x10761f6 { return; }
  pc = 0x10761f6; continue;
 }
 0x10761f6 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010761f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10761f8); }
 let mut pc: u32 = 0x10761f8;
 loop { match pc {
 0x10761f8 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10761ff); f_0105f215(cpu); if cpu.returned_to != 0x10761ff { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.and32(cpu.edx, 0x3u32); cpu.edx = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1076247u32; cpu.esi = v; }
  cpu.push(0x1076245); f_01075c8b(cpu); if cpu.returned_to != 0x1076245 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076247(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076247); }
 let mut pc: u32 = 0x1076247;
 loop { match pc {
 0x1076247 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x107624e); f_0105f215(cpu); if cpu.returned_to != 0x107624e { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x107625a; continue; }
  pc = 0x1076254; continue;
 }
 0x1076254 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x107625e; continue;
 }
 0x107625a => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x107625e; continue;
 }
 0x107625e => {
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x10762dcu32; cpu.esi = v; }
  cpu.push(0x10762a0); f_01075c8b(cpu); if cpu.returned_to != 0x10762a0 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x10762da); f_01075c8b(cpu); if cpu.returned_to != 0x10762da { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010762dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10762dc); }
 let mut pc: u32 = 0x10762dc;
 loop { match pc {
 0x10762dc => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10762e3); f_0105f215(cpu); if cpu.returned_to != 0x10762e3 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x10762ef; continue; }
  pc = 0x10762e9; continue;
 }
 0x10762e9 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10762f3; continue;
 }
 0x10762ef => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10762f3; continue;
 }
 0x10762f3 => {
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1076371u32; cpu.esi = v; }
  cpu.push(0x1076335); f_01075c8b(cpu); if cpu.returned_to != 0x1076335 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x107636f); f_01075c8b(cpu); if cpu.returned_to != 0x107636f { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076371(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076371); }
 let mut pc: u32 = 0x1076371;
 loop { match pc {
 0x1076371 => {
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x10763aa); f_01075c8b(cpu); if cpu.returned_to != 0x10763aa { return; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x24u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x10763df); f_01075c8b(cpu); if cpu.returned_to != 0x10763df { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 6u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x112cf69u32.wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x112cadcu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x112cadeu32, v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x32u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076446); f_01075c8b(cpu); if cpu.returned_to != 0x1076446 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x32u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x15u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1076523u32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x112cadcu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x112cadeu32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1076488); f_01075c8b(cpu); if cpu.returned_to != 0x1076488 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107648a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107648a); }
 let mut pc: u32 = 0x107648a;
 loop { match pc {
 0x107648a => {
  cpu.push(0x107648f); f_01077015(cpu); if cpu.returned_to != 0x107648f { return; }
  if cpu.cf { pc = 0x1076522; continue; }
  pc = 0x1076495; continue;
 }
 0x1076495 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x112cf69u32.wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x10764ee); f_01075c8b(cpu); if cpu.returned_to != 0x10764ee { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.add32(cpu.ecx, 0x15u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1076523u32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076522); f_01075c8b(cpu); if cpu.returned_to != 0x1076522 { return; }
  pc = 0x1076522; continue;
 }
 0x1076522 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076523(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076523); }
 let mut pc: u32 = 0x1076523;
 loop { match pc {
 0x1076523 => {
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x112cf71u32.wrapping_add(cpu.edx.wrapping_mul(2)); cpu.esi = v; }
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
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076572); f_01075c8b(cpu); if cpu.returned_to != 0x1076572 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076574(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076574); }
 let mut pc: u32 = 0x1076574;
 loop { match pc {
 0x1076574 => {
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x112cf89u32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x112cf8bu32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x11056a5u32, v); }
  cpu.push(0x1076593); f_01077015(cpu); if cpu.returned_to != 0x1076593 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11056a5u32, v); }
  if cpu.cf { pc = 0x1076619; continue; }
  pc = 0x107659c; continue;
 }
 0x107659c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.ecx, 6u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x31u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x107661au32; cpu.esi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10765d1); f_01075c8b(cpu); if cpu.returned_to != 0x10765d1 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.ecx, 6u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x112cf79u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x112cf7bu32.wrapping_add(cpu.ecx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x31u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1076669u32; cpu.esi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076619); f_01075c8b(cpu); if cpu.returned_to != 0x1076619 { return; }
  pc = 0x1076619; continue;
 }
 0x1076619 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107661a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107661a); }
 let mut pc: u32 = 0x107661a;
 loop { match pc {
 0x107661a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xeu32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x112cf89u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x112cf8bu32.wrapping_add(cpu.ecx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  { let v: u32 = 0x2du32; cpu.ecx = v; }
  { let v: u8 = 0x4u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x10766b8u32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  cpu.push(0x1076667); f_01075c8b(cpu); if cpu.returned_to != 0x1076667 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076669(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076669); }
 let mut pc: u32 = 0x1076669;
 loop { match pc {
 0x1076669 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xeu32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x112cf89u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x112cf8bu32.wrapping_add(cpu.ecx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  { let v: u32 = 0x2fu32; cpu.ecx = v; }
  { let v: u8 = 0x4u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x10766b8u32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  cpu.push(0x10766b6); f_01075c8b(cpu); if cpu.returned_to != 0x10766b6 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010766b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10766b8); }
 let mut pc: u32 = 0x10766b8;
 loop { match pc {
 0x10766b8 => {
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x40u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u32 = 0x30u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x10766f9); f_01075c8b(cpu); if cpu.returned_to != 0x10766f9 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010766fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10766fb); }
 let mut pc: u32 = 0x10766fb;
 loop { match pc {
 0x10766fb => {
  { cpu.pushad(); }
  cpu.push(0x1076701); f_01077015(cpu); if cpu.returned_to != 0x1076701 { return; }
  if cpu.cf { pc = 0x1076778; continue; }
  pc = 0x1076703; continue;
 }
 0x1076703 => {
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x7u32); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076745); f_01075c8b(cpu); if cpu.returned_to != 0x1076745 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  { let v: u32 = 0x2eu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x107677au32; cpu.esi = v; }
  cpu.push(0x1076778); f_01075c8b(cpu); if cpu.returned_to != 0x1076778 { return; }
  pc = 0x1076778; continue;
 }
 0x1076778 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107677a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107677a); }
 let mut pc: u32 = 0x107677a;
 loop { match pc {
 0x107677a => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1076781); f_0105f215(cpu); if cpu.returned_to != 0x1076781 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u32 = 0x2eu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x10767c4u32; cpu.esi = v; }
  cpu.push(0x10767c2); f_01075c8b(cpu); if cpu.returned_to != 0x10767c2 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010767c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10767c4); }
 let mut pc: u32 = 0x10767c4;
 loop { match pc {
 0x10767c4 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10767cb); f_0105f215(cpu); if cpu.returned_to != 0x10767cb { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x10767d7; continue; }
  pc = 0x10767d1; continue;
 }
 0x10767d1 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10767db; continue;
 }
 0x10767d7 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10767db; continue;
 }
 0x10767db => {
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u32 = 0x2eu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1076858u32; cpu.esi = v; }
  cpu.push(0x107681c); f_01075c8b(cpu); if cpu.returned_to != 0x107681c { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076856); f_01075c8b(cpu); if cpu.returned_to != 0x1076856 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076858(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076858); }
 let mut pc: u32 = 0x1076858;
 loop { match pc {
 0x1076858 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x107685f); f_0105f215(cpu); if cpu.returned_to != 0x107685f { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x107686b; continue; }
  pc = 0x1076865; continue;
 }
 0x1076865 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x107686f; continue;
 }
 0x107686b => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x107686f; continue;
 }
 0x107686f => {
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x2eu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0x10768e8u32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x10768ac); f_01075c8b(cpu); if cpu.returned_to != 0x10768ac { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x10768e6); f_01075c8b(cpu); if cpu.returned_to != 0x10768e6 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010768e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10768e8); }
 let mut pc: u32 = 0x10768e8;
 loop { match pc {
 0x10768e8 => {
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x33u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076921); f_01075c8b(cpu); if cpu.returned_to != 0x1076921 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076923(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076923); }
 let mut pc: u32 = 0x1076923;
 loop { match pc {
 0x1076923 => {
  { cpu.pushad(); }
  cpu.push(0x1076929); f_01077015(cpu); if cpu.returned_to != 0x1076929 { return; }
  if cpu.cf { pc = 0x10769a0; continue; }
  pc = 0x107692b; continue;
 }
 0x107692b => {
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x7u32); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x1fu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x107696d); f_01075c8b(cpu); if cpu.returned_to != 0x107696d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x2cu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x10769a2u32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x10769a0); f_01075c8b(cpu); if cpu.returned_to != 0x10769a0 { return; }
  pc = 0x10769a0; continue;
 }
 0x10769a0 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010769a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10769a2); }
 let mut pc: u32 = 0x10769a2;
 loop { match pc {
 0x10769a2 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10769a9); f_0105f215(cpu); if cpu.returned_to != 0x10769a9 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.and32(cpu.edx, 0x3u32); cpu.edx = v; }
  { let v: u32 = 0x2bu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x10769f1u32; cpu.esi = v; }
  cpu.push(0x10769ef); f_01075c8b(cpu); if cpu.returned_to != 0x10769ef { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010769f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10769f1); }
 let mut pc: u32 = 0x10769f1;
 loop { match pc {
 0x10769f1 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10769f8); f_0105f215(cpu); if cpu.returned_to != 0x10769f8 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x1076a04; continue; }
  pc = 0x10769fe; continue;
 }
 0x10769fe => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1076a08; continue;
 }
 0x1076a04 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1076a08; continue;
 }
 0x1076a08 => {
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x2bu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1076a86u32; cpu.esi = v; }
  cpu.push(0x1076a4a); f_01075c8b(cpu); if cpu.returned_to != 0x1076a4a { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1fu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076a84); f_01075c8b(cpu); if cpu.returned_to != 0x1076a84 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076a86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076a86); }
 let mut pc: u32 = 0x1076a86;
 loop { match pc {
 0x1076a86 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1076a8d); f_0105f215(cpu); if cpu.returned_to != 0x1076a8d { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x1076a99; continue; }
  pc = 0x1076a93; continue;
 }
 0x1076a93 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1076a9d; continue;
 }
 0x1076a99 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1076a9d; continue;
 }
 0x1076a9d => {
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x2bu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  { let v: u8 = 0x14u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1076b1bu32; cpu.esi = v; }
  cpu.push(0x1076adf); f_01075c8b(cpu); if cpu.returned_to != 0x1076adf { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1fu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076b19); f_01075c8b(cpu); if cpu.returned_to != 0x1076b19 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076b1b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076b1b); }
 let mut pc: u32 = 0x1076b1b;
 loop { match pc {
 0x1076b1b => {
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1fu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076b54); f_01075c8b(cpu); if cpu.returned_to != 0x1076b54 { return; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x2cu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076b89); f_01075c8b(cpu); if cpu.returned_to != 0x1076b89 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 6u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x112cf69u32.wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x112cadcu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x112cadeu32, v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x32u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x1fu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076bf0); f_01075c8b(cpu); if cpu.returned_to != 0x1076bf0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x32u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x1bu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1076ccdu32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x112cadcu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x112cadeu32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1076c32); f_01075c8b(cpu); if cpu.returned_to != 0x1076c32 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076c34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076c34); }
 let mut pc: u32 = 0x1076c34;
 loop { match pc {
 0x1076c34 => {
  cpu.push(0x1076c39); f_01077015(cpu); if cpu.returned_to != 0x1076c39 { return; }
  if cpu.cf { pc = 0x1076ccc; continue; }
  pc = 0x1076c3f; continue;
 }
 0x1076c3f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x112cf69u32.wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = 0x1fu32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076c98); f_01075c8b(cpu); if cpu.returned_to != 0x1076c98 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.add32(cpu.ecx, 0x1bu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1076ccdu32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076ccc); f_01075c8b(cpu); if cpu.returned_to != 0x1076ccc { return; }
  pc = 0x1076ccc; continue;
 }
 0x1076ccc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076ccd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076ccd); }
 let mut pc: u32 = 0x1076ccd;
 loop { match pc {
 0x1076ccd => {
  { cpu.pushad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u32 = 0x112cf71u32.wrapping_add(cpu.edx.wrapping_mul(2)); cpu.esi = v; }
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
  { let v: u8 = 0x10u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076d1c); f_01075c8b(cpu); if cpu.returned_to != 0x1076d1c { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076d1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076d1e); }
 let mut pc: u32 = 0x1076d1e;
 loop { match pc {
 0x1076d1e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1076d25); f_0104f36d(cpu); if cpu.returned_to != 0x1076d25 { return; }
  cpu.push(0x1076d2a); f_01076e89(cpu); if cpu.returned_to != 0x1076d2a { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1076d87; continue; }
  pc = 0x1076d2e; continue;
 }
 0x1076d2e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1076e1eu32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e97u32, v); }
  cpu.push(0x1076d63); f_01075c8b(cpu); if cpu.returned_to != 0x1076d63 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0xffd2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076d87); f_01075c8b(cpu); if cpu.returned_to != 0x1076d87 { return; }
  pc = 0x1076d87; continue;
 }
 0x1076d87 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076d88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076d88); }
 let mut pc: u32 = 0x1076d88;
 loop { match pc {
 0x1076d88 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1076d8f); f_0104f36d(cpu); if cpu.returned_to != 0x1076d8f { return; }
  cpu.push(0x1076d94); f_01076e89(cpu); if cpu.returned_to != 0x1076d94 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1076e1d; continue; }
  pc = 0x1076d9c; continue;
 }
 0x1076d9c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub16((cpu.ecx as u16), 0x1u16); }
  if cpu.zf { pc = 0x1076db4; continue; }
  pc = 0x1076da4; continue;
 }
 0x1076da4 => {
  { let v: u32 = 0x26u32; cpu.ecx = v; }
  { let v: u16 = 0x40u16; cpu.w16(0x10e3e93u32, v); }
  pc = 0x1076dc2; continue;
 }
 0x1076db4 => {
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u16 = 0xc0u16; cpu.w16(0x10e3e93u32, v); }
  pc = 0x1076dc2; continue;
 }
 0x1076dc2 => {
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1076e1eu32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e97u32, v); }
  cpu.push(0x1076de7); f_01075c8b(cpu); if cpu.returned_to != 0x1076de7 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0xffd2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u8 = 0x30u8; cpu.w8(0x10e3e97u32, v); }
  cpu.push(0x1076e1d); f_01075c8b(cpu); if cpu.returned_to != 0x1076e1d { return; }
  pc = 0x1076e1d; continue;
 }
 0x1076e1d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076e1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076e1e); }
 let mut pc: u32 = 0x1076e1e;
 loop { match pc {
 0x1076e1e => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  cpu.push(0x1076e51); f_01075c8b(cpu); if cpu.returned_to != 0x1076e51 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u8 = 0x30u8; cpu.w8(0x10e3e97u32, v); }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  cpu.push(0x1076e87); f_01075c8b(cpu); if cpu.returned_to != 0x1076e87 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076e89(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076e89); }
 let mut pc: u32 = 0x1076e89;
 loop { match pc {
 0x1076e89 => {
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1076e9f; continue; }
  pc = 0x1076e95; continue;
 }
 0x1076e95 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1076e9f; continue; }
  pc = 0x1076e99; continue;
 }
 0x1076e99 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x1076e9f; continue; }
  pc = 0x1076e9d; continue;
 }
 0x1076e9d => {
  pc = 0x1076ef7; continue;
 }
 0x1076e9f => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1076eb5; continue; }
  pc = 0x1076eab; continue;
 }
 0x1076eab => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1076eb5; continue; }
  pc = 0x1076eaf; continue;
 }
 0x1076eaf => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x1076eb5; continue; }
  pc = 0x1076eb3; continue;
 }
 0x1076eb3 => {
  pc = 0x1076ef7; continue;
 }
 0x1076eb5 => {
  { let v: u8 = cpu.r8(0x3fffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1076ecb; continue; }
  pc = 0x1076ec1; continue;
 }
 0x1076ec1 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1076ecb; continue; }
  pc = 0x1076ec5; continue;
 }
 0x1076ec5 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x1076ecb; continue; }
  pc = 0x1076ec9; continue;
 }
 0x1076ec9 => {
  pc = 0x1076ef7; continue;
 }
 0x1076ecb => {
  { let v: u8 = cpu.r8(0x3f80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1076ee1; continue; }
  pc = 0x1076ed7; continue;
 }
 0x1076ed7 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1076ee1; continue; }
  pc = 0x1076edb; continue;
 }
 0x1076edb => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x1076ee1; continue; }
  pc = 0x1076edf; continue;
 }
 0x1076edf => {
  pc = 0x1076ef7; continue;
 }
 0x1076ee1 => {
  { let v: u8 = cpu.r8(0x4080u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1076ef5; continue; }
  pc = 0x1076eed; continue;
 }
 0x1076eed => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1076ef5; continue; }
  pc = 0x1076ef1; continue;
 }
 0x1076ef1 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x1076ef7; continue; }
  pc = 0x1076ef5; continue;
 }
 0x1076ef5 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1076ef7 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076ef9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076ef9); }
 let mut pc: u32 = 0x1076ef9;
 loop { match pc {
 0x1076ef9 => {
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1076f0f; continue; }
  pc = 0x1076f05; continue;
 }
 0x1076f05 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1076f0f; continue; }
  pc = 0x1076f09; continue;
 }
 0x1076f09 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x1076f0f; continue; }
  pc = 0x1076f0d; continue;
 }
 0x1076f0d => {
  pc = 0x1076f83; continue;
 }
 0x1076f0f => {
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if cpu.zf { pc = 0x1076f49; continue; }
  pc = 0x1076f15; continue;
 }
 0x1076f15 => {
  { cpu.sub16((cpu.ecx as u16), 0x2u16); }
  if cpu.zf { pc = 0x1076f5d; continue; }
  pc = 0x1076f1b; continue;
 }
 0x1076f1b => {
  { cpu.sub16((cpu.ecx as u16), 0x3u16); }
  if cpu.zf { pc = 0x1076f35; continue; }
  pc = 0x1076f21; continue;
 }
 0x1076f21 => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1076f71; continue; }
  pc = 0x1076f2d; continue;
 }
 0x1076f2d => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1076f71; continue; }
  pc = 0x1076f31; continue;
 }
 0x1076f31 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x1076f83; continue; }
  pc = 0x1076f35; continue;
 }
 0x1076f35 => {
  { let v: u8 = cpu.r8(0x3fffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1076f71; continue; }
  pc = 0x1076f41; continue;
 }
 0x1076f41 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1076f71; continue; }
  pc = 0x1076f45; continue;
 }
 0x1076f45 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x1076f83; continue; }
  pc = 0x1076f49; continue;
 }
 0x1076f49 => {
  { let v: u8 = cpu.r8(0x3f80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1076f71; continue; }
  pc = 0x1076f55; continue;
 }
 0x1076f55 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1076f71; continue; }
  pc = 0x1076f59; continue;
 }
 0x1076f59 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x1076f83; continue; }
  pc = 0x1076f5d; continue;
 }
 0x1076f5d => {
  { let v: u8 = cpu.r8(0x4080u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1076f71; continue; }
  pc = 0x1076f69; continue;
 }
 0x1076f69 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1076f71; continue; }
  pc = 0x1076f6d; continue;
 }
 0x1076f6d => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x1076f83; continue; }
  pc = 0x1076f71; continue;
 }
 0x1076f71 => {
  { cpu.sub8(cpu.r8(0x11056a5u32), 0x0u8); }
  if cpu.zf { pc = 0x1076f81; continue; }
  pc = 0x1076f7a; continue;
 }
 0x1076f7a => {
  cpu.push(0x1076f7f); f_01076f85(cpu); if cpu.returned_to != 0x1076f7f { return; }
  if cpu.cf { pc = 0x1076f83; continue; }
  pc = 0x1076f81; continue;
 }
 0x1076f81 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1076f83 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076f85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076f85); }
 let mut pc: u32 = 0x1076f85;
 loop { match pc {
 0x1076f85 => {
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if cpu.zf { pc = 0x1076fd3; continue; }
  pc = 0x1076f8b; continue;
 }
 0x1076f8b => {
  { cpu.sub16((cpu.ecx as u16), 0x2u16); }
  if cpu.zf { pc = 0x1076fbf; continue; }
  pc = 0x1076f91; continue;
 }
 0x1076f91 => {
  { cpu.sub16((cpu.ecx as u16), 0x3u16); }
  if cpu.zf { pc = 0x1076f97; continue; }
  pc = 0x1076f97; continue;
 }
 0x1076f97 => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1076fe7; continue; }
  pc = 0x1076fa3; continue;
 }
 0x1076fa3 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1076fe7; continue; }
  pc = 0x1076fa7; continue;
 }
 0x1076fa7 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x1076fe9; continue; }
  pc = 0x1076fab; continue;
 }
 0x1076fab => {
  { let v: u8 = cpu.r8(0x3fffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1076fe7; continue; }
  pc = 0x1076fb7; continue;
 }
 0x1076fb7 => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1076fe7; continue; }
  pc = 0x1076fbb; continue;
 }
 0x1076fbb => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x1076fe9; continue; }
  pc = 0x1076fbf; continue;
 }
 0x1076fbf => {
  { let v: u8 = cpu.r8(0x3f80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1076fe7; continue; }
  pc = 0x1076fcb; continue;
 }
 0x1076fcb => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1076fe7; continue; }
  pc = 0x1076fcf; continue;
 }
 0x1076fcf => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x1076fe9; continue; }
  pc = 0x1076fd3; continue;
 }
 0x1076fd3 => {
  { let v: u8 = cpu.r8(0x4080u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1076fe7; continue; }
  pc = 0x1076fdf; continue;
 }
 0x1076fdf => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1076fe7; continue; }
  pc = 0x1076fe3; continue;
 }
 0x1076fe3 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if !cpu.zf { pc = 0x1076fe9; continue; }
  pc = 0x1076fe7; continue;
 }
 0x1076fe7 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1076fe9 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01076feb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1076feb); }
 let mut pc: u32 = 0x1076feb;
 loop { match pc {
 0x1076feb => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x10ea910u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x1077010; continue; }
  pc = 0x107700b; continue;
 }
 0x107700b => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077010 => {
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077015(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077015); }
 let mut pc: u32 = 0x1077015;
 loop { match pc {
 0x1077015 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x107701d); f_0104f36d(cpu); if cpu.returned_to != 0x107701d { return; }
  cpu.push(0x1077022); f_01076ef9(cpu); if cpu.returned_to != 0x1077022 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077026(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077026); }
 let mut pc: u32 = 0x1077026;
 loop { match pc {
 0x1077026 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x107702e); f_0104f36d(cpu); if cpu.returned_to != 0x107702e { return; }
  cpu.push(0x1077033); f_01076e89(cpu); if cpu.returned_to != 0x1077033 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077037(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077037); }
 let mut pc: u32 = 0x1077037;
 loop { match pc {
 0x1077037 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x10ea910u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077055(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077055); }
 let mut pc: u32 = 0x1077055;
 loop { match pc {
 0x1077055 => {
  { let v: u32 = 0x1172f54u32; cpu.edi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x1077064; continue;
 }
 0x1077064 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x40u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1077064; continue; }
  pc = 0x107706c; continue;
 }
 0x107706c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107706d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107706d); }
 let mut pc: u32 = 0x107706d;
 loop { match pc {
 0x107706d => {
  { let v: u32 = 0x1174854u32; cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x107707c; continue;
 }
 0x107707c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0xau32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x107707c; continue; }
  pc = 0x1077084; continue;
 }
 0x1077084 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077085(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077085); }
 let mut pc: u32 = 0x1077085;
 loop { match pc {
 0x1077085 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1174854u32; cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x1077096; continue;
 }
 0x1077096 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x10770a4; continue; }
  pc = 0x107709b; continue;
 }
 0x107709b => {
  { let v: u32 = cpu.add32(cpu.edi, 0xau32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1077096; continue; }
  pc = 0x10770a0; continue;
 }
 0x10770a0 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10770a4 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010770a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10770a8); }
 let mut pc: u32 = 0x10770a8;
 loop { match pc {
 0x10770a8 => {
  { cpu.pushad(); }
  cpu.push(0x10770ae); f_01077085(cpu); if cpu.returned_to != 0x10770ae { return; }
  if cpu.cf { pc = 0x10770bd; continue; }
  pc = 0x10770b0; continue;
 }
 0x10770b0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10770bd => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010770c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10770c0); }
 let mut pc: u32 = 0x10770c0;
 loop { match pc {
 0x10770c0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1172f54u32; cpu.edi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10770d1; continue;
 }
 0x10770d1 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x10770df; continue; }
  pc = 0x10770d6; continue;
 }
 0x10770d6 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x40u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10770d1; continue; }
  pc = 0x10770db; continue;
 }
 0x10770db => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10770df => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010770e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10770e3); }
 let mut pc: u32 = 0x10770e3;
 loop { match pc {
 0x10770e3 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.esi)), 0xfu8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1077141; continue; }
  pc = 0x10770ea; continue;
 }
 0x10770ea => {
  { let v: u32 = 0x1174a48u32; cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.sf { pc = 0x1077141; continue; }
  pc = 0x10770fb; continue;
 }
 0x10770fb => {
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x1077141; continue; }
  pc = 0x1077101; continue;
 }
 0x1077101 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x107713b; continue; }
  pc = 0x1077108; continue;
 }
 0x1077108 => {
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
  if (!cpu.cf && !cpu.zf) { pc = 0x107713b; continue; }
  pc = 0x1077134; continue;
 }
 0x1077134 => {
  { let v: u8 = cpu.or8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x81u8); cpu.w8(0x35u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = true; }
  pc = 0x1077141; continue;
 }
 0x107713b => {
  { let v: u32 = cpu.add32(cpu.edi, 0x3cu32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1077101; continue; }
  pc = 0x1077140; continue;
 }
 0x1077140 => {
  { cpu.cf = false; }
  pc = 0x1077141; continue;
 }
 0x1077141 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077143(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077143); }
 let mut pc: u32 = 0x1077143;
 loop { match pc {
 0x1077143 => {
  { cpu.pushad(); }
  { let v: u32 = 0x1172f54u32; cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  pc = 0x107714e; continue;
 }
 0x107714e => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x10771ca; continue; }
  pc = 0x1077156; continue;
 }
 0x1077156 => {
  { cpu.sub8(cpu.r8(0x27u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x10771ca; continue; }
  pc = 0x107715c; continue;
 }
 0x107715c => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x30u8); }
  if !cpu.zf { pc = 0x10771ca; continue; }
  pc = 0x1077162; continue;
 }
 0x1077162 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.zf { pc = 0x10771ca; continue; }
  pc = 0x1077168; continue;
 }
 0x1077168 => {
  { cpu.sub8(cpu.r8(0x20u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x10771ca; continue; }
  pc = 0x107716e; continue;
 }
 0x107716e => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x1077184; continue; }
  pc = 0x1077181; continue;
 }
 0x1077181 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1077184; continue;
 }
 0x1077184 => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x107718d; continue; }
  pc = 0x107718a; continue;
 }
 0x107718a => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x107718d; continue;
 }
 0x107718d => {
  { cpu.sub16((cpu.ecx as u16), 0x18u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10771ca; continue; }
  pc = 0x1077193; continue;
 }
 0x1077193 => {
  { cpu.sub16((cpu.edx as u16), 0x18u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10771ca; continue; }
  pc = 0x1077199; continue;
 }
 0x1077199 => {
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
  if !cpu.sf { pc = 0x10771c5; continue; }
  pc = 0x10771c2; continue;
 }
 0x10771c2 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10771c5; continue;
 }
 0x10771c5 => {
  cpu.push(0x10771ca); f_01075fce(cpu); if cpu.returned_to != 0x10771ca { return; }
  pc = 0x10771ca; continue;
 }
 0x10771ca => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x107714e; continue; }
  pc = 0x10771d5; continue;
 }
 0x10771d5 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010771d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10771d7); }
 let mut pc: u32 = 0x10771d7;
 loop { match pc {
 0x10771d7 => {
  { cpu.pushad(); }
  { let v: u32 = 0x1172f54u32; cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  pc = 0x10771e2; continue;
 }
 0x10771e2 => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1077268; continue; }
  pc = 0x10771ea; continue;
 }
 0x10771ea => {
  { cpu.sub8(cpu.r8(0x27u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x1077268; continue; }
  pc = 0x10771f0; continue;
 }
 0x10771f0 => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x30u8); }
  if !cpu.zf { pc = 0x1077268; continue; }
  pc = 0x10771f6; continue;
 }
 0x10771f6 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.zf { pc = 0x1077268; continue; }
  pc = 0x10771fc; continue;
 }
 0x10771fc => {
  { cpu.sub8(cpu.r8(0x20u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1077268; continue; }
  pc = 0x1077202; continue;
 }
 0x1077202 => {
  { cpu.sub8(cpu.r8(0x20u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1077268; continue; }
  pc = 0x1077208; continue;
 }
 0x1077208 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x107721e; continue; }
  pc = 0x107721b; continue;
 }
 0x107721b => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x107721e; continue;
 }
 0x107721e => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x1077227; continue; }
  pc = 0x1077224; continue;
 }
 0x1077224 => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1077227; continue;
 }
 0x1077227 => {
  { cpu.sub16((cpu.ecx as u16), 0x10u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1077268; continue; }
  pc = 0x107722d; continue;
 }
 0x107722d => {
  { cpu.sub16((cpu.edx as u16), 0x10u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1077268; continue; }
  pc = 0x1077233; continue;
 }
 0x1077233 => {
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
  if !cpu.sf { pc = 0x107725f; continue; }
  pc = 0x107725c; continue;
 }
 0x107725c => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x107725f; continue;
 }
 0x107725f => {
  cpu.push(0x1077264); f_01075fce(cpu); if cpu.returned_to != 0x1077264 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077268 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10771e2; continue; }
  pc = 0x1077273; continue;
 }
 0x1077273 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077276(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077276); }
 let mut pc: u32 = 0x1077276;
 loop { match pc {
 0x1077276 => {
  { cpu.pushad(); }
  { let v: u32 = 0x1172f54u32; cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  pc = 0x1077281; continue;
 }
 0x1077281 => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x10772c2; continue; }
  pc = 0x1077289; continue;
 }
 0x1077289 => {
  { cpu.sub8(cpu.r8(0x27u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x10772c2; continue; }
  pc = 0x107728f; continue;
 }
 0x107728f => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x30u8); }
  if !cpu.zf { pc = 0x10772c2; continue; }
  pc = 0x1077295; continue;
 }
 0x1077295 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x10772ab; continue; }
  pc = 0x10772a8; continue;
 }
 0x10772a8 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10772ab; continue;
 }
 0x10772ab => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x10772b4; continue; }
  pc = 0x10772b1; continue;
 }
 0x10772b1 => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x10772b4; continue;
 }
 0x10772b4 => {
  { cpu.sub16((cpu.ecx as u16), 0x18u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10772c2; continue; }
  pc = 0x10772ba; continue;
 }
 0x10772ba => {
  { cpu.sub16((cpu.edx as u16), 0x18u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10772c2; continue; }
  pc = 0x10772c0; continue;
 }
 0x10772c0 => {
  pc = 0x10772cb; continue;
 }
 0x10772c2 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x40u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1077281; continue; }
  pc = 0x10772c8; continue;
 }
 0x10772c8 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10772cb => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010772db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10772db); }
 let mut pc: u32 = 0x10772db;
 loop { match pc {
 0x10772db => {
  { let v: u32 = 0x1172f54u32; cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edi = v; }
  pc = 0x10772eb; continue;
 }
 0x10772eb => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x10772fb; continue; }
  pc = 0x10772f2; continue;
 }
 0x10772f2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10772f9); f_01077307(cpu); if cpu.returned_to != 0x10772f9 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x10772fb; continue;
 }
 0x10772fb => {
  { let v: u32 = cpu.add32(cpu.esi, 0x40u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10772eb; continue; }
  pc = 0x1077300; continue;
 }
 0x1077300 => {
  { let v: u32 = cpu.edi; cpu.w32(0x10e4010u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077307(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1077307); } r_01077307(cpu, 0x1077307); }
pub fn f_01077561(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1077561); } r_01077307(cpu, 0x1077561); }
fn r_01077307(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1077307 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2bu32.wrapping_add(cpu.esi), v); }
  { cpu.sub16(cpu.r16(0x27u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x1077331; continue; }
  pc = 0x1077328; continue;
 }
 0x1077328 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x27u32.wrapping_add(cpu.esi))); cpu.w16(0x27u32.wrapping_add(cpu.esi), v); }
  pc = 0x10775e2; continue;
 }
 0x1077331 => {
  cpu.push(0x1077336); f_01077944(cpu); if cpu.returned_to != 0x1077336 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x20u8); }
  if !cpu.zf { pc = 0x10775e2; continue; }
  pc = 0x1077344; continue;
 }
 0x1077344 => {
  cpu.push(0x1077349); f_01077c02(cpu); if cpu.returned_to != 0x1077349 { return; }
  cpu.push(0x107734e); f_0107790f(cpu); if cpu.returned_to != 0x107734e { return; }
  if cpu.cf { pc = 0x10775e2; continue; }
  pc = 0x1077354; continue;
 }
 0x1077354 => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x40u8); }
  if !cpu.zf { pc = 0x10773a0; continue; }
  pc = 0x107735a; continue;
 }
 0x107735a => {
  cpu.push(0x107735f); f_010778dd(cpu); if cpu.returned_to != 0x107735f { return; }
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2du32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.esi)) & 0xfeu8; cpu.w8(0x36u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) & 0xffu8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x2du32.wrapping_add(cpu.esi)) & 0xffu8; cpu.w8(0x2du32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1077385; continue; }
  pc = 0x1077377; continue;
 }
 0x1077377 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3au32.wrapping_add(cpu.esi))); cpu.w8(0x3au32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x10775e2; continue; }
  pc = 0x1077380; continue;
 }
 0x1077380 => {
  pc = 0x1077436; continue;
 }
 0x1077385 => {
  cpu.push(0x107738a); f_010778b5(cpu); if cpu.returned_to != 0x107738a { return; }
  cpu.push(0x107738f); f_01077841(cpu); if cpu.returned_to != 0x107738f { return; }
  if cpu.cf { pc = 0x10775e2; continue; }
  pc = 0x1077395; continue;
 }
 0x1077395 => {
  cpu.push(0x107739a); f_0107780d(cpu); if cpu.returned_to != 0x107739a { return; }
  if cpu.cf { pc = 0x10775e2; continue; }
  pc = 0x10773a0; continue;
 }
 0x10773a0 => {
  cpu.push(0x10773a5); f_010777c6(cpu); if cpu.returned_to != 0x10773a5 { return; }
  if cpu.cf { pc = 0x10775e2; continue; }
  pc = 0x10773ab; continue;
 }
 0x10773ab => {
  cpu.push(0x10773b0); f_0107776a(cpu); if cpu.returned_to != 0x10773b0 { return; }
  if cpu.cf { pc = 0x10775e2; continue; }
  pc = 0x10773b6; continue;
 }
 0x10773b6 => {
  cpu.push(0x10773bb); f_010777ab(cpu); if cpu.returned_to != 0x10773bb { return; }
  cpu.push(0x10773c0); f_01077755(cpu); if cpu.returned_to != 0x10773c0 { return; }
  if cpu.cf { pc = 0x10773e1; continue; }
  pc = 0x10773c2; continue;
 }
 0x10773c2 => {
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x10775e2; continue; }
  pc = 0x10773cc; continue;
 }
 0x10773cc => {
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.esi)) & 0xbfu8; cpu.w8(0x36u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x1077416; continue; }
  pc = 0x10773da; continue;
 }
 0x10773da => {
  cpu.push(0x10773df); f_010776b1(cpu); if cpu.returned_to != 0x10773df { return; }
  if !cpu.cf { pc = 0x107740d; continue; }
  pc = 0x10773e1; continue;
 }
 0x10773e1 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x107752a; continue; }
  pc = 0x10773eb; continue;
 }
 0x10773eb => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)) & 0x7eu8; cpu.w8(0x35u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x10773f4); f_0105f215(cpu); if cpu.returned_to != 0x10773f4 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x27u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0xfu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x10775e2; continue;
 }
 0x107740d => {
  cpu.push(0x1077412); f_010776e8(cpu); if cpu.returned_to != 0x1077412 { return; }
  if cpu.cf { cpu.poll(); pc = 0x10773e1; continue; }
  pc = 0x1077414; continue;
 }
 0x1077414 => {
  pc = 0x107742c; continue;
 }
 0x1077416 => {
  cpu.push(0x107741b); f_01077d19(cpu); if cpu.returned_to != 0x107741b { return; }
  if !cpu.cf { pc = 0x1077423; continue; }
  pc = 0x107741d; continue;
 }
 0x107741d => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)) | 0x81u8; cpu.w8(0x35u32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x10773cc; continue;
 }
 0x1077423 => {
  cpu.push(0x1077428); f_010770e3(cpu); if cpu.returned_to != 0x1077428 { return; }
  if !cpu.cf { pc = 0x1077436; continue; }
  pc = 0x107742a; continue;
 }
 0x107742a => {
  cpu.poll(); pc = 0x10773cc; continue;
 }
 0x107742c => {
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x2du32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  pc = 0x1077462; continue;
 }
 0x1077436 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x10775e2; continue; }
  pc = 0x1077440; continue;
 }
 0x1077440 => {
  cpu.push(0x1077445); f_0105f215(cpu); if cpu.returned_to != 0x1077445 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1077455; continue; }
  pc = 0x107744b; continue;
 }
 0x107744b => {
  { let v: u16 = cpu.shl16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x1077462; continue; }
  pc = 0x1077455; continue;
 }
 0x1077455 => {
  cpu.push(0x107745a); f_01077627(cpu); if cpu.returned_to != 0x107745a { return; }
  if cpu.cf { pc = 0x10775e2; continue; }
  pc = 0x1077460; continue;
 }
 0x1077460 => {
  pc = 0x1077472; continue;
 }
 0x1077462 => {
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.esi)) & 0xfdu8; cpu.w8(0x36u32.wrapping_add(cpu.esi), v); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.sf { pc = 0x1077472; continue; }
  pc = 0x107746b; continue;
 }
 0x107746b => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.esi)) | 0x2u8; cpu.w8(0x36u32.wrapping_add(cpu.esi), v); }
  pc = 0x1077472; continue;
 }
 0x1077472 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x22u32.wrapping_add(cpu.esi))); cpu.w8(0x22u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x22u32.wrapping_add(cpu.esi)), 0xau8); }
  if !cpu.zf { pc = 0x1077485; continue; }
  pc = 0x107747b; continue;
 }
 0x107747b => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x10775e2; continue;
 }
 0x1077485 => {
  { let v: u8 = cpu.r8(0x2du32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x1077492; continue; }
  pc = 0x107748e; continue;
 }
 0x107748e => {
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1077494; continue;
 }
 0x1077492 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1077494; continue;
 }
 0x1077494 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x2du32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = (cpu.ebx as u8) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { let v: u32 = 0x112caacu32; cpu.edi = v; }
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
  cpu.push(0x10774d8); f_0104f36d(cpu); if cpu.returned_to != 0x10774d8 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x10774f5; continue; }
  pc = 0x10774e5; continue;
 }
 0x10774e5 => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x107759a; continue; }
  pc = 0x10774ed; continue;
 }
 0x10774ed => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x107759a; continue; }
  pc = 0x10774f5; continue;
 }
 0x10774f5 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10f72ddu32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x107759a; continue; }
  pc = 0x1077507; continue;
 }
 0x1077507 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x107751b; continue; }
  pc = 0x1077511; continue;
 }
 0x1077511 => {
  { cpu.sub8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x1077519; continue; }
  pc = 0x1077515; continue;
 }
 0x1077515 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x107751b; continue; }
  pc = 0x1077519; continue;
 }
 0x1077519 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x107751b; continue;
 }
 0x107751b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.poll(); pc = 0x1077472; continue;
 }
 0x107752a => {
  { cpu.pushad(); }
  { let v: u16 = 0x20u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x40u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1077538); f_0105f2ec(cpu); if cpu.returned_to != 0x1077538 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1077547); f_01076074(cpu); if cpu.returned_to != 0x1077547 { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x2du32.wrapping_add(cpu.esi)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x12u32.wrapping_add(cpu.esi)); cpu.push(0x107755c); cpu.call(t); if cpu.returned_to != 0x107755c { return; }
  pc = 0x10775e2; continue;
 }
 0x1077561 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1077579); f_01077b15(cpu); if cpu.returned_to != 0x1077579 { return; }
  if cpu.cf { pc = 0x107758e; continue; }
  pc = 0x107757b; continue;
 }
 0x107757b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xau32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.poll(); pc = 0x10773cc; continue;
 }
 0x107758e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x10u8); cpu.w8(0x36u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10775e2; continue;
 }
 0x107759a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x8u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(cpu.r16(0xau32.wrapping_add(cpu.esi))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x10775af; continue; }
  pc = 0x10775ac; continue;
 }
 0x10775ac => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10775af; continue;
 }
 0x10775af => {
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x10775b8; continue; }
  pc = 0x10775b5; continue;
 }
 0x10775b5 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10775b8; continue;
 }
 0x10775b8 => {
  { cpu.sub16((cpu.ebx as u16), 0x50u16); }
  if (cpu.sf != cpu.of) { pc = 0x10775c2; continue; }
  pc = 0x10775be; continue;
 }
 0x10775be => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10775d1; continue;
 }
 0x10775c2 => {
  { cpu.sub16((cpu.ecx as u16), 0x50u16); }
  if (cpu.sf != cpu.of) { pc = 0x10775cc; continue; }
  pc = 0x10775c8; continue;
 }
 0x10775c8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10775d1; continue;
 }
 0x10775cc => {
  { let v: u8 = cpu.r8(0x3bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10775da; continue;
 }
 0x10775d1 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x1077472; continue;
 }
 0x10775da => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.sf { pc = 0x10775e2; continue; }
  pc = 0x10775e2; continue;
 }
 0x10775e2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010775e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10775e3); }
 let mut pc: u32 = 0x10775e3;
 loop { match pc {
 0x10775e3 => {
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x1077625; continue; }
  pc = 0x10775e9; continue;
 }
 0x10775e9 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x2fu32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1077625; continue; }
  pc = 0x10775f1; continue;
 }
 0x10775f1 => {
  { let v: u32 = cpu.r8(0x3bu32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2du32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x112caacu32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as i8 as i16 as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x37u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x107761d; continue; }
  pc = 0x1077617; continue;
 }
 0x1077617 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x107761d; continue;
 }
 0x107761d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)).wrapping_add((cpu.ebx as u16)); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.add16(cpu.r16(0x2u32.wrapping_add(cpu.esi)), (cpu.ecx as u16)); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  pc = 0x1077625; continue;
 }
 0x1077625 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077627(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077627); }
 let mut pc: u32 = 0x1077627;
 loop { match pc {
 0x1077627 => {
  { cpu.sub16(cpu.r16(0x1au32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x10776a9; continue; }
  pc = 0x107762f; continue;
 }
 0x107762f => {
  { let v = cpu.eax; cpu.push(v); }
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x1077682; continue; }
  pc = 0x1077636; continue;
 }
 0x1077636 => {
  cpu.push(0x107763b); f_0105f215(cpu); if cpu.returned_to != 0x107763b { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0xfu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1077682; continue; }
  pc = 0x1077641; continue;
 }
 0x1077641 => {
  { cpu.sub8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x107764f; continue; }
  pc = 0x1077647; continue;
 }
 0x1077647 => {
  { cpu.sub8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x107764f; continue; }
  pc = 0x107764d; continue;
 }
 0x107764d => {
  pc = 0x1077682; continue;
 }
 0x107764f => {
  { let v: u8 = cpu.r8(0x2du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3fu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x1077682; continue; }
  pc = 0x1077656; continue;
 }
 0x1077656 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.esi)) | 0x40u8; cpu.w8(0x36u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0x2u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1077668); f_0105f2ec(cpu); if cpu.returned_to != 0x1077668 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x6u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x6u8; cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1077679); f_0105f215(cpu); if cpu.returned_to != 0x1077679 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  pc = 0x107769f; continue;
 }
 0x1077682 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x10776a1; continue; }
  pc = 0x1077689; continue;
 }
 0x1077689 => {
  cpu.push(0x107768e); f_0105f215(cpu); if cpu.returned_to != 0x107768e { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0xfu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x10776a1; continue; }
  pc = 0x1077694; continue;
 }
 0x1077694 => {
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3au32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  pc = 0x107769f; continue;
 }
 0x107769f => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10776a1 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10776a9 => {
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0xfdu8); cpu.w8(0x36u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010776b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10776b1); }
 let mut pc: u32 = 0x10776b1;
 loop { match pc {
 0x10776b1 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.sf { pc = 0x10776cb; continue; }
  pc = 0x10776c8; continue;
 }
 0x10776c8 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10776cb; continue;
 }
 0x10776cb => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x10776d4; continue; }
  pc = 0x10776d1; continue;
 }
 0x10776d1 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10776d4; continue;
 }
 0x10776d4 => {
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10776e4; continue; }
  pc = 0x10776da; continue;
 }
 0x10776da => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10776e4; continue; }
  pc = 0x10776e0; continue;
 }
 0x10776e0 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10776e4 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010776e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10776e8); }
 let mut pc: u32 = 0x10776e8;
 loop { match pc {
 0x10776e8 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.sf { pc = 0x10776f6; continue; }
  pc = 0x10776f3; continue;
 }
 0x10776f3 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10776f6; continue;
 }
 0x10776f6 => {
  { cpu.and16((cpu.ebx as u16), (cpu.ebx as u16)); }
  if !cpu.sf { pc = 0x10776fe; continue; }
  pc = 0x10776fb; continue;
 }
 0x10776fb => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10776fe; continue;
 }
 0x10776fe => {
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1077727; continue; }
  pc = 0x1077703; continue;
 }
 0x1077703 => {
  if (cpu.sf != cpu.of) { pc = 0x1077716; continue; }
  pc = 0x1077705; continue;
 }
 0x1077705 => {
  { cpu.and16((cpu.ecx as u16), (cpu.ecx as u16)); }
  if !cpu.sf { pc = 0x1077710; continue; }
  pc = 0x107770a; continue;
 }
 0x107770a => {
  { let v: u8 = 0xc0u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x1077751; continue;
 }
 0x1077710 => {
  { let v: u8 = 0x40u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x1077751; continue;
 }
 0x1077716 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if !cpu.sf { pc = 0x1077721; continue; }
  pc = 0x107771b; continue;
 }
 0x107771b => {
  { let v: u8 = 0x0u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x1077751; continue;
 }
 0x1077721 => {
  { let v: u8 = 0x80u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x1077751; continue;
 }
 0x1077727 => {
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if cpu.zf { pc = 0x1077753; continue; }
  pc = 0x107772d; continue;
 }
 0x107772d => {
  if !cpu.sf { pc = 0x1077741; continue; }
  pc = 0x107772f; continue;
 }
 0x107772f => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x107773b; continue; }
  pc = 0x1077735; continue;
 }
 0x1077735 => {
  { let v: u8 = 0xe0u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x1077751; continue;
 }
 0x107773b => {
  { let v: u8 = 0xa0u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x1077751; continue;
 }
 0x1077741 => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x107774d; continue; }
  pc = 0x1077747; continue;
 }
 0x1077747 => {
  { let v: u8 = 0x20u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x1077751; continue;
 }
 0x107774d => {
  { let v: u8 = 0x60u8; cpu.w8(0x3bu32.wrapping_add(cpu.esi), v); }
  pc = 0x1077751; continue;
 }
 0x1077751 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077753 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077755(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077755); }
 let mut pc: u32 = 0x1077755;
 loop { match pc {
 0x1077755 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x1077768; continue; }
  pc = 0x107775b; continue;
 }
 0x107775b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1077767); f_01077a7b(cpu); if cpu.returned_to != 0x1077767 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077768 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107776a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107776a); }
 let mut pc: u32 = 0x107776a;
 loop { match pc {
 0x107776a => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x10777a9; continue; }
  pc = 0x1077770; continue;
 }
 0x1077770 => {
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x10777a9; continue; }
  pc = 0x1077776; continue;
 }
 0x1077776 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1077783); f_0104f36d(cpu); if cpu.returned_to != 0x1077783 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1077794; continue; }
  pc = 0x1077790; continue;
 }
 0x1077790 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if !cpu.zf { pc = 0x10777a9; continue; }
  pc = 0x1077794; continue;
 }
 0x1077794 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10777a1); f_0101bdb4(cpu); if cpu.returned_to != 0x10777a1 { return; }
  { cpu.popad(); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10777a9 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010777ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10777ab); }
 let mut pc: u32 = 0x10777ab;
 loop { match pc {
 0x10777ab => {
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.esi))); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x10777c1; continue; }
  pc = 0x10777b0; continue;
 }
 0x10777b0 => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x40u8); }
  if !cpu.zf { pc = 0x10777c5; continue; }
  pc = 0x10777b6; continue;
 }
 0x10777b6 => {
  { cpu.and8(cpu.r8(0x37u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x10777c5; continue; }
  pc = 0x10777bc; continue;
 }
 0x10777bc => {
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.esi))); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x10777c5; continue; }
  pc = 0x10777c1; continue;
 }
 0x10777c1 => {
  { let v: u8 = 0x10u8; cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  pc = 0x10777c5; continue;
 }
 0x10777c5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010777c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10777c6); }
 let mut pc: u32 = 0x10777c6;
 loop { match pc {
 0x10777c6 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x10777d8; continue; }
  pc = 0x10777cc; continue;
 }
 0x10777cc => {
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x107780b; continue; }
  pc = 0x10777d2; continue;
 }
 0x10777d2 => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x40u8); }
  if !cpu.zf { pc = 0x1077809; continue; }
  pc = 0x10777d8; continue;
 }
 0x10777d8 => {
  { let v: u32 = cpu.r8(0x2du32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x112caacu32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as i8 as i16 as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x37u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x1077801; continue; }
  pc = 0x10777fb; continue;
 }
 0x10777fb => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1077801; continue;
 }
 0x1077801 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)).wrapping_add((cpu.ebx as u16)); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.add16(cpu.r16(0x2u32.wrapping_add(cpu.esi)), (cpu.ecx as u16)); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  pc = 0x1077809; continue;
 }
 0x1077809 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x107780b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107780d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107780d); }
 let mut pc: u32 = 0x107780d;
 loop { match pc {
 0x107780d => {
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x1077823; continue; }
  pc = 0x1077813; continue;
 }
 0x1077813 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x2eu32.wrapping_add(cpu.esi))); cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x21u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1077825; continue; }
  pc = 0x1077823; continue;
 }
 0x1077823 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077825 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x38u32.wrapping_add(cpu.esi)), 0x7fu8); }
  if !cpu.zf { pc = 0x107783f; continue; }
  pc = 0x1077833; continue;
 }
 0x1077833 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x2du32.wrapping_add(cpu.esi)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x12u32.wrapping_add(cpu.esi)); cpu.push(0x107783f); cpu.call(t); if cpu.returned_to != 0x107783f { return; }
  pc = 0x107783f; continue;
 }
 0x107783f => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077841(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077841); }
 let mut pc: u32 = 0x1077841;
 loop { match pc {
 0x1077841 => {
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x1077885; continue; }
  pc = 0x1077847; continue;
 }
 0x1077847 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x2eu32.wrapping_add(cpu.esi))); cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x1077876; continue; }
  pc = 0x1077850; continue;
 }
 0x1077850 => {
  { let v: u16 = cpu.r8(0x2eu32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x16u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x23u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x1077885; continue; }
  pc = 0x107785f; continue;
 }
 0x107785f => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1077870; continue; }
  pc = 0x1077865; continue;
 }
 0x1077865 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1077887; continue; }
  pc = 0x107786b; continue;
 }
 0x107786b => {
  { let v: u8 = cpu.dec8(cpu.r8(0x30u32.wrapping_add(cpu.esi))); cpu.w8(0x30u32.wrapping_add(cpu.esi), v); }
  if cpu.sf { pc = 0x1077887; continue; }
  pc = 0x1077870; continue;
 }
 0x1077870 => {
  { let v: u8 = 0x0u8; cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  pc = 0x1077885; continue;
 }
 0x1077876 => {
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1077885; continue; }
  pc = 0x107787e; continue;
 }
 0x107787e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1077883; continue;
 }
 0x1077883 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077885 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077887 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x38u32.wrapping_add(cpu.esi)), 0x7fu8); }
  if !cpu.zf { cpu.poll(); pc = 0x1077883; continue; }
  pc = 0x1077895; continue;
 }
 0x1077895 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x2du32.wrapping_add(cpu.esi)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.r32(0x12u32.wrapping_add(cpu.esi)), 0x100001fu32); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x1077883; continue; }
  pc = 0x10778a7; continue;
 }
 0x10778a7 => {
  { cpu.sub32(cpu.r32(0x12u32.wrapping_add(cpu.esi)), 0x108cf1cu32); }
  if !cpu.cf { cpu.poll(); pc = 0x1077883; continue; }
  pc = 0x10778b0; continue;
 }
 0x10778b0 => {
  let t = cpu.r32(0x12u32.wrapping_add(cpu.esi)); cpu.push(0x10778b3); cpu.call(t); if cpu.returned_to != 0x10778b3 { return; }
  cpu.poll(); pc = 0x1077883; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010778b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10778b5); }
 let mut pc: u32 = 0x10778b5;
 loop { match pc {
 0x10778b5 => {
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x10778dc; continue; }
  pc = 0x10778bb; continue;
 }
 0x10778bb => {
  { let v: u8 = cpu.inc8(cpu.r8(0x25u32.wrapping_add(cpu.esi))); cpu.w8(0x25u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x7fu8); }
  if cpu.zf { pc = 0x10778dc; continue; }
  pc = 0x10778c5; continue;
 }
 0x10778c5 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x25u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x10778dc; continue; }
  pc = 0x10778ca; continue;
 }
 0x10778ca => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x2du32.wrapping_add(cpu.esi)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x12u32.wrapping_add(cpu.esi)); cpu.push(0x10778dc); cpu.call(t); if cpu.returned_to != 0x10778dc { return; }
  pc = 0x10778dc; continue;
 }
 0x10778dc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010778dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10778dd); }
 let mut pc: u32 = 0x10778dd;
 loop { match pc {
 0x10778dd => {
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
pub fn f_0107790f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107790f); }
 let mut pc: u32 = 0x107790f;
 loop { match pc {
 0x107790f => {
  { let v: u8 = cpu.dec8(cpu.r8(0x2fu32.wrapping_add(cpu.esi))); cpu.w8(0x2fu32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x1077942; continue; }
  pc = 0x1077914; continue;
 }
 0x1077914 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x40u8); }
  if !cpu.zf { pc = 0x1077927; continue; }
  pc = 0x107791d; continue;
 }
 0x107791d => {
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x107793d; continue; }
  pc = 0x1077923; continue;
 }
 0x1077923 => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x107793d; continue;
 }
 0x1077927 => {
  { cpu.sub8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1077933; continue; }
  pc = 0x107792d; continue;
 }
 0x107792d => {
  { cpu.sub8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x107793d; continue; }
  pc = 0x1077933; continue;
 }
 0x1077933 => {
  { let v: u8 = cpu.shl8((cpu.eax as u8), 3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x2eu32.wrapping_add(cpu.esi))); cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x2eu32.wrapping_add(cpu.esi)), 0x3u8); cpu.w8(0x2eu32.wrapping_add(cpu.esi), v); }
  pc = 0x107793d; continue;
 }
 0x107793d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2fu32.wrapping_add(cpu.esi), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077942 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077944(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077944); }
 let mut pc: u32 = 0x1077944;
 loop { match pc {
 0x1077944 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x1077957; continue; }
  pc = 0x1077951; continue;
 }
 0x1077951 => {
  { let v: u32 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  pc = 0x1077983; continue;
 }
 0x1077957 => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x1077967; continue; }
  pc = 0x107795d; continue;
 }
 0x107795d => {
  { let v: u8 = 0x10u8; cpu.w8(0x34u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r16(0x18u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  pc = 0x1077989; continue;
 }
 0x1077967 => {
  { cpu.and8(cpu.r8(0x36u32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x107797f; continue; }
  pc = 0x107796d; continue;
 }
 0x107796d => {
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x2du32.wrapping_add(cpu.esi)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xcu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  pc = 0x1077983; continue;
 }
 0x107797f => {
  { let v: u32 = cpu.r16(0x16u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  pc = 0x1077983; continue;
 }
 0x1077983 => {
  { let v: u32 = cpu.r8(0x2eu32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  pc = 0x1077989; continue;
 }
 0x1077989 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1077a15; continue; }
  pc = 0x10779a0; continue;
 }
 0x10779a0 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1077a15; continue; }
  pc = 0x10779a9; continue;
 }
 0x10779a9 => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x10e40fcu32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xff80u16); }
  if (cpu.sf != cpu.of) { pc = 0x1077a15; continue; }
  pc = 0x10779b9; continue;
 }
 0x10779b9 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x10e4100u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xff80u16); }
  if (cpu.sf != cpu.of) { pc = 0x1077a15; continue; }
  pc = 0x10779c9; continue;
 }
 0x10779c9 => {
  { cpu.sub8(cpu.r8(0x11b4c6cu32), 0x0u8); }
  if !cpu.zf { pc = 0x10779ea; continue; }
  pc = 0x10779d2; continue;
 }
 0x10779d2 => {
  { cpu.sub32(cpu.edi, 0x10e55f8u32); }
  if (cpu.sf == cpu.of) { pc = 0x10779ea; continue; }
  pc = 0x10779da; continue;
 }
 0x10779da => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e68bau32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x10779f3; continue; }
  pc = 0x10779ea; continue;
 }
 0x10779ea => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b4c6cu32, v); }
  pc = 0x1077a15; continue;
 }
 0x10779f3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0xcu8); }
  if !cpu.zf { pc = 0x1077a16; continue; }
  pc = 0x1077a04; continue;
 }
 0x1077a04 => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x34u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  pc = 0x1077a15; continue;
 }
 0x1077a15 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077a16 => {
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x10e55fau32); cpu.edx = v; }
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
  if cpu.zf { pc = 0x1077a5f; continue; }
  pc = 0x1077a55; continue;
 }
 0x1077a55 => {
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1077a5f; continue;
 }
 0x1077a5f => {
  { let v: u8 = 0x0u8; cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x1077a73; continue; }
  pc = 0x1077a69; continue;
 }
 0x1077a69 => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1077a73; continue;
 }
 0x1077a73 => {
  { let v: u32 = cpu.add32(cpu.r32(0x10e401cu32), 0x10u32); cpu.w32(0x10e401cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077a7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077a7b); }
 let mut pc: u32 = 0x1077a7b;
 loop { match pc {
 0x1077a7b => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x1174854u32; cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x1077a93; continue;
 }
 0x1077a93 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x1077ab9; continue; }
  pc = 0x1077a9c; continue;
 }
 0x1077a9c => {
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1077aa4; continue; }
  pc = 0x1077aa1; continue;
 }
 0x1077aa1 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1077aa4; continue;
 }
 0x1077aa4 => {
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1077ab9; continue; }
  pc = 0x1077aaa; continue;
 }
 0x1077aaa => {
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1077ab3; continue; }
  pc = 0x1077ab0; continue;
 }
 0x1077ab0 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1077ab3; continue;
 }
 0x1077ab3 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x1077ac3; continue; }
  pc = 0x1077ab9; continue;
 }
 0x1077ab9 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0xau32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1077a93; continue; }
  pc = 0x1077ac0; continue;
 }
 0x1077ac0 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077ac3 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077ac8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077ac8); }
 let mut pc: u32 = 0x1077ac8;
 loop { match pc {
 0x1077ac8 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x1174854u32; cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x1077ae0; continue;
 }
 0x1077ae0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x1077b06; continue; }
  pc = 0x1077ae9; continue;
 }
 0x1077ae9 => {
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1077af1; continue; }
  pc = 0x1077aee; continue;
 }
 0x1077aee => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1077af1; continue;
 }
 0x1077af1 => {
  { cpu.sub16((cpu.eax as u16), 0x2u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1077b06; continue; }
  pc = 0x1077af7; continue;
 }
 0x1077af7 => {
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1077b00; continue; }
  pc = 0x1077afd; continue;
 }
 0x1077afd => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1077b00; continue;
 }
 0x1077b00 => {
  { cpu.sub16((cpu.ebx as u16), 0x2u16); }
  if (cpu.cf || cpu.zf) { pc = 0x1077b10; continue; }
  pc = 0x1077b06; continue;
 }
 0x1077b06 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0xau32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1077ae0; continue; }
  pc = 0x1077b0d; continue;
 }
 0x1077b0d => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077b10 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077b15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077b15); }
 let mut pc: u32 = 0x1077b15;
 loop { match pc {
 0x1077b15 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e99u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3e99u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u32 = 0x1174854u32; cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112cad4u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112cad6u32, v); }
  pc = 0x1077b79; continue;
 }
 0x1077b79 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x1077bdf; continue; }
  pc = 0x1077b80; continue;
 }
 0x1077b80 => {
  { let v: u16 = cpu.r16(0x10e3e97u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x1077b99; continue; }
  pc = 0x1077b8b; continue;
 }
 0x1077b8b => {
  { let v: u16 = cpu.r16(0x10e3e99u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x1077b99; continue; }
  pc = 0x1077b97; continue;
 }
 0x1077b97 => {
  pc = 0x1077bdf; continue;
 }
 0x1077b99 => {
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if cpu.zf { pc = 0x1077bdf; continue; }
  pc = 0x1077bc6; continue;
 }
 0x1077bc6 => {
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x1077bdf; continue; }
  pc = 0x1077bca; continue;
 }
 0x1077bca => {
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112cad4u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112cad6u32, v); }
  pc = 0x1077bdf; continue;
 }
 0x1077bdf => {
  { let v: u32 = cpu.edi.wrapping_add(0xau32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1077b79; continue; }
  pc = 0x1077be4; continue;
 }
 0x1077be4 => {
  { cpu.sub16(cpu.r16(0x112cad4u32), 0xffffu16); }
  if cpu.zf { pc = 0x1077bff; continue; }
  pc = 0x1077bef; continue;
 }
 0x1077bef => {
  { cpu.popad(); }
  { cpu.cf = false; }
  { let v: u16 = cpu.r16(0x112cad4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x112cad6u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077bff => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077c02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077c02); }
 let mut pc: u32 = 0x1077c02;
 loop { match pc {
 0x1077c02 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1077c1b; continue; }
  pc = 0x1077c0b; continue;
 }
 0x1077c0b => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1077c1b; continue; }
  pc = 0x1077c11; continue;
 }
 0x1077c11 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x1077d14; continue; }
  pc = 0x1077c1b; continue;
 }
 0x1077c1b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x1077c3a); f_010676ce(cpu); if cpu.returned_to != 0x1077c3a { return; }
  { let v: u32 = 0x11a0f95u32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x10u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x10u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e99u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xau16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0xau16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xau16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x6u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x5u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x5u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x5u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3eafu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3eb1u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3eb3u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3eb5u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  pc = 0x1077cba; continue;
 }
 0x1077cba => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1077d16; continue; }
  pc = 0x1077cbf; continue;
 }
 0x1077cbf => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1077cdb; continue; }
  pc = 0x1077cd3; continue;
 }
 0x1077cd3 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x1077cdb; continue; }
  pc = 0x1077cd9; continue;
 }
 0x1077cd9 => {
  cpu.poll(); pc = 0x1077cba; continue;
 }
 0x1077cdb => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e93u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1077cba; continue; }
  pc = 0x1077ce8; continue;
 }
 0x1077ce8 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e97u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1077cba; continue; }
  pc = 0x1077cf1; continue;
 }
 0x1077cf1 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e95u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1077cba; continue; }
  pc = 0x1077cfe; continue;
 }
 0x1077cfe => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e99u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1077cba; continue; }
  pc = 0x1077d07; continue;
 }
 0x1077d07 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x36u32.wrapping_add(cpu.ebp)), 0x20u8); cpu.w8(0x36u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7fu8; cpu.w8(0x2fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1077d14; continue;
 }
 0x1077d14 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077d16 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077d19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077d19); }
 let mut pc: u32 = 0x1077d19;
 loop { match pc {
 0x1077d19 => {
  { cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x1077d26; continue; }
  pc = 0x1077d20; continue;
 }
 0x1077d20 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x10u32.wrapping_add(cpu.esi))); cpu.w16(0x10u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077d26 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1077d5a; continue; }
  pc = 0x1077d2f; continue;
 }
 0x1077d2f => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x1077d5a; continue; }
  pc = 0x1077d35; continue;
 }
 0x1077d35 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.zf { pc = 0x1077d5a; continue; }
  pc = 0x1077d3b; continue;
 }
 0x1077d3b => {
  { cpu.and8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x1077d5a; continue; }
  pc = 0x1077d41; continue;
 }
 0x1077d41 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x1077d50); f_010676ce(cpu); if cpu.returned_to != 0x1077d50 { return; }
  { let v: u32 = 0x11a0f95u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { pc = 0x1077d5d; continue; }
  pc = 0x1077d5a; continue;
 }
 0x1077d5a => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077d5d => {
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077d60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077d60); }
 let mut pc: u32 = 0x1077d60;
 loop { match pc {
 0x1077d60 => {
  { cpu.pushad(); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x10e3f1bu32), 0x0u16); }
  if cpu.zf { pc = 0x1077e45; continue; }
  pc = 0x1077d70; continue;
 }
 0x1077d70 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x3u8); }
  if !cpu.zf { pc = 0x1077e45; continue; }
  pc = 0x1077d7d; continue;
 }
 0x1077d7d => {
  cpu.push(0x1077d82); f_01077e69(cpu); if cpu.returned_to != 0x1077d82 { return; }
  if cpu.cf { pc = 0x1077e45; continue; }
  pc = 0x1077d88; continue;
 }
 0x1077d88 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2au32.wrapping_add(cpu.edi), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x112cb55u32; cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
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
  if cpu.zf { pc = 0x1077e45; continue; }
  pc = 0x1077e23; continue;
 }
 0x1077e23 => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.edi)) | 0x44u8; cpu.w8(0x32u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7eu16); }
  if cpu.zf { pc = 0x1077e42; continue; }
  pc = 0x1077e30; continue;
 }
 0x1077e30 => {
  { cpu.sub16((cpu.eax as u16), 0x7du16); }
  if cpu.zf { pc = 0x1077e42; continue; }
  pc = 0x1077e36; continue;
 }
 0x1077e36 => {
  { cpu.sub16((cpu.eax as u16), 0x7cu16); }
  if cpu.zf { pc = 0x1077e42; continue; }
  pc = 0x1077e3c; continue;
 }
 0x1077e3c => {
  { let v: u32 = cpu.add32(cpu.esi, 0x2u32); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x30u32.wrapping_add(cpu.edi), v); }
  pc = 0x1077e42; continue;
 }
 0x1077e42 => {
  { let v: u32 = cpu.esi; cpu.w32(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x1077e45; continue;
 }
 0x1077e45 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077e48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077e48); }
 let mut pc: u32 = 0x1077e48;
 loop { match pc {
 0x1077e48 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077e51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077e51); }
 let mut pc: u32 = 0x1077e51;
 loop { match pc {
 0x1077e51 => {
  { let v: u32 = 0x1174a48u32; cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x1077e60; continue;
 }
 0x1077e60 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x3cu32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1077e60; continue; }
  pc = 0x1077e68; continue;
 }
 0x1077e68 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077e69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077e69); }
 let mut pc: u32 = 0x1077e69;
 loop { match pc {
 0x1077e69 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1174a48u32; cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x1077e7a; continue;
 }
 0x1077e7a => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x1077e88; continue; }
  pc = 0x1077e7f; continue;
 }
 0x1077e7f => {
  { let v: u32 = cpu.add32(cpu.edi, 0x3cu32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1077e7a; continue; }
  pc = 0x1077e84; continue;
 }
 0x1077e84 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1077e88 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077e8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077e8c); }
 let mut pc: u32 = 0x1077e8c;
 loop { match pc {
 0x1077e8c => {
  { cpu.sub16(cpu.r16(0x10e3f1bu32), 0x0u16); }
  if cpu.zf { pc = 0x1077eca; continue; }
  pc = 0x1077e96; continue;
 }
 0x1077e96 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x3u8); }
  if !cpu.zf { pc = 0x1077eca; continue; }
  pc = 0x1077e9f; continue;
 }
 0x1077e9f => {
  { let v: u32 = 0x1174a48u32; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edi = v; }
  pc = 0x1077eaf; continue;
 }
 0x1077eaf => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1077ebf; continue; }
  pc = 0x1077eb6; continue;
 }
 0x1077eb6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1077ebd); f_01077ecb(cpu); if cpu.returned_to != 0x1077ebd { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1077ebf; continue;
 }
 0x1077ebf => {
  { let v: u32 = cpu.add32(cpu.esi, 0x3cu32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1077eaf; continue; }
  pc = 0x1077ec4; continue;
 }
 0x1077ec4 => {
  { let v: u32 = cpu.edi; cpu.w32(0x10e4010u32, v); }
  pc = 0x1077eca; continue;
 }
 0x1077eca => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01077ecb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1077ecb); }
 let mut pc: u32 = 0x1077ecb;
 loop { match pc {
 0x1077ecb => {
  { cpu.sub16(cpu.r16(0x1au32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x1077edc; continue; }
  pc = 0x1077ed3; continue;
 }
 0x1077ed3 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x1au32.wrapping_add(cpu.esi))); cpu.w16(0x1au32.wrapping_add(cpu.esi), v); }
  pc = 0x10784c9; continue;
 }
 0x1077edc => {
  { cpu.sub16(cpu.r16(0x1eu32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.sf { pc = 0x1077eea; continue; }
  pc = 0x1077ee4; continue;
 }
 0x1077ee4 => {
  { let v: u16 = 0x0u16; cpu.w16(0x1eu32.wrapping_add(cpu.esi), v); }
  pc = 0x1077eea; continue;
 }
 0x1077eea => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x1077efd; continue; }
  pc = 0x1077ef7; continue;
 }
 0x1077ef7 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x1077f0b; continue; }
  pc = 0x1077efd; continue;
 }
 0x1077efd => {
  { let v: u32 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 6u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x8u32.wrapping_add(cpu.esi))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1077f49; continue;
 }
 0x1077f0b => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x18u8); }
  if cpu.zf { pc = 0x1077f2b; continue; }
  pc = 0x1077f11; continue;
 }
 0x1077f11 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x2au32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x1077f21; continue; }
  pc = 0x1077f1b; continue;
 }
 0x1077f1b => {
  { cpu.sub8(cpu.r8(0x2au32.wrapping_add(cpu.esi)), 0xc0u8); }
  if !cpu.zf { pc = 0x1077f25; continue; }
  pc = 0x1077f21; continue;
 }
 0x1077f21 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1077f25; continue;
 }
 0x1077f25 => {
  { let v: u32 = cpu.r16(0x22u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  pc = 0x1077f43; continue;
 }
 0x1077f2b => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x1077f3f; continue; }
  pc = 0x1077f31; continue;
 }
 0x1077f31 => {
  { let v: u32 = cpu.r16(0x24u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1077f43; continue;
 }
 0x1077f3f => {
  { let v: u32 = cpu.r16(0x12u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  pc = 0x1077f43; continue;
 }
 0x1077f43 => {
  { let v: u32 = cpu.r8(0x2bu32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  pc = 0x1077f49; continue;
 }
 0x1077f49 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1078022; continue; }
  pc = 0x1077f64; continue;
 }
 0x1077f64 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1078022; continue; }
  pc = 0x1077f71; continue;
 }
 0x1077f71 => {
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x10e40fcu32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xff80u16); }
  if (cpu.sf != cpu.of) { pc = 0x1078022; continue; }
  pc = 0x1077f85; continue;
 }
 0x1077f85 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x10e4100u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xff80u16); }
  if (cpu.sf != cpu.of) { pc = 0x1078022; continue; }
  pc = 0x1077f99; continue;
 }
 0x1077f99 => {
  { cpu.sub8(cpu.r8(0x11b4c6cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1077fba; continue; }
  pc = 0x1077fa2; continue;
 }
 0x1077fa2 => {
  { cpu.sub32(cpu.edi, 0x10e55f8u32); }
  if (cpu.sf == cpu.of) { pc = 0x1077fba; continue; }
  pc = 0x1077faa; continue;
 }
 0x1077faa => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e68bau32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x1077fc3; continue; }
  pc = 0x1077fba; continue;
 }
 0x1077fba => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b4c6cu32, v); }
  pc = 0x1078022; continue;
 }
 0x1077fc3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  cpu.push(0x1077fd4); f_0105f178(cpu); if cpu.returned_to != 0x1077fd4 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x1078004; continue; }
  pc = 0x1077fd9; continue;
 }
 0x1077fd9 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x7au32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x1077ff9; continue; }
  pc = 0x1077fef; continue;
 }
 0x1077fef => {
  cpu.push(0x1077ff4); f_01012780(cpu); if cpu.returned_to != 0x1077ff4 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x1077ff9; continue;
 }
 0x1077ff9 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1078004); f_010356fe(cpu); if cpu.returned_to != 0x1078004 { return; }
  pc = 0x1078004; continue;
 }
 0x1078004 => {
  { cpu.popad(); }
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0xcu8); }
  if !cpu.zf { pc = 0x10784ca; continue; }
  pc = 0x107800f; continue;
 }
 0x107800f => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x40u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  pc = 0x1078022; continue;
 }
 0x1078022 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x2cu32.wrapping_add(cpu.esi))); cpu.w8(0x2cu32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x107802e; continue; }
  pc = 0x1078027; continue;
 }
 0x1078027 => {
  if cpu.sf { pc = 0x107802e; continue; }
  pc = 0x1078029; continue;
 }
 0x1078029 => {
  pc = 0x10784c9; continue;
 }
 0x107802e => {
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x1078039; continue; }
  pc = 0x1078037; continue;
 }
 0x1078037 => {
  { let v: u8 = cpu.shl8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1078039; continue;
 }
 0x1078039 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2cu32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x10780ad; continue; }
  pc = 0x1078042; continue;
 }
 0x1078042 => {
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)).wrapping_add(0x10u8); cpu.w8(0x30u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)) & 0xffu8; cpu.w8(0x30u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x1eu32.wrapping_add(cpu.esi)), 0x2u16); cpu.w16(0x1eu32.wrapping_add(cpu.esi), v); }
  if !cpu.sf { pc = 0x10784c9; continue; }
  pc = 0x1078056; continue;
 }
 0x1078056 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1078063); f_0105b236(cpu); if cpu.returned_to != 0x1078063 { return; }
  { cpu.popad(); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  { let v: u16 = 0x80u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffc0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x2u32); }
  if !cpu.zf { pc = 0x1078099; continue; }
  pc = 0x1078097; continue;
 }
 0x1078097 => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1078099; continue;
 }
 0x1078099 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x10780a1); f_01077d60(cpu); if cpu.returned_to != 0x10780a1 { return; }
  { cpu.popad(); }
  { let v: u16 = 0x64u16; cpu.w16(0x1au32.wrapping_add(cpu.esi), v); }
  pc = 0x10784c9; continue;
 }
 0x10780ad => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x38u8); }
  if !cpu.zf { pc = 0x10780c1; continue; }
  pc = 0x10780b3; continue;
 }
 0x10780b3 => {
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x31u32.wrapping_add(cpu.esi), v); }
  pc = 0x10780e2; continue;
 }
 0x10780c1 => {
  cpu.push(0x10780c6); f_0107861a(cpu); if cpu.returned_to != 0x10780c6 { return; }
  if cpu.cf { pc = 0x10784c9; continue; }
  pc = 0x10780cc; continue;
 }
 0x10780cc => {
  cpu.push(0x10780d1); f_0107859c(cpu); if cpu.returned_to != 0x10780d1 { return; }
  if cpu.cf { pc = 0x10784c9; continue; }
  pc = 0x10780d7; continue;
 }
 0x10780d7 => {
  cpu.push(0x10780dc); f_01078539(cpu); if cpu.returned_to != 0x10780dc { return; }
  if cpu.cf { pc = 0x10784c9; continue; }
  pc = 0x10780e2; continue;
 }
 0x10780e2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10780ee); f_010787e3(cpu); if cpu.returned_to != 0x10780ee { return; }
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
  if cpu.zf { pc = 0x1078151; continue; }
  pc = 0x1078129; continue;
 }
 0x1078129 => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x37u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1078139; continue; }
  pc = 0x1078133; continue;
 }
 0x1078133 => {
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) & 0xfeu8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  pc = 0x1078151; continue;
 }
 0x1078139 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x1078145; continue; }
  pc = 0x107813f; continue;
 }
 0x107813f => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)).wrapping_add(0x2u8); cpu.w8(0x2au32.wrapping_add(cpu.esi), v); }
  pc = 0x1078149; continue;
 }
 0x1078145 => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)).wrapping_sub(0x2u8); cpu.w8(0x2au32.wrapping_add(cpu.esi), v); }
  pc = 0x1078149; continue;
 }
 0x1078149 => {
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)) & 0xffu8; cpu.w8(0x30u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) & 0xffu8; cpu.w8(0x2au32.wrapping_add(cpu.esi), v); }
  pc = 0x1078151; continue;
 }
 0x1078151 => {
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x1078170; continue; }
  pc = 0x1078157; continue;
 }
 0x1078157 => {
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x1078167; continue; }
  pc = 0x107815d; continue;
 }
 0x107815d => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)) ^ 0x1u8; cpu.w8(0x35u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1078170; continue; }
  pc = 0x1078167; continue;
 }
 0x1078167 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x40u8); }
  if !cpu.zf { pc = 0x1078170; continue; }
  pc = 0x107816d; continue;
 }
 0x107816d => {
  { let v: u8 = cpu.inc8(cpu.r8(0x2bu32.wrapping_add(cpu.esi))); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  pc = 0x1078170; continue;
 }
 0x1078170 => {
  { let v: u16 = cpu.r8(0x2bu32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x12u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x16u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x1078183; continue; }
  pc = 0x107817f; continue;
 }
 0x107817f => {
  { let v: u8 = 0x0u8; cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  pc = 0x1078183; continue;
 }
 0x1078183 => {
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x10781ab; continue; }
  pc = 0x1078189; continue;
 }
 0x1078189 => {
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x20u32.wrapping_add(cpu.esi))); }
  if cpu.cf { pc = 0x107819b; continue; }
  pc = 0x1078197; continue;
 }
 0x1078197 => {
  { let v: u16 = cpu.r16(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x107819b; continue;
 }
 0x107819b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.esi), v); }
  { cpu.sub16(cpu.r16(0x20u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x1078209; continue; }
  pc = 0x10781a5; continue;
 }
 0x10781a5 => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)) & 0xfdu8; cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  pc = 0x1078209; continue;
 }
 0x10781ab => {
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x10781d5; continue; }
  pc = 0x10781b1; continue;
 }
 0x10781b1 => {
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x20u32.wrapping_add(cpu.esi))); }
  if cpu.sf { pc = 0x10781c1; continue; }
  pc = 0x10781bf; continue;
 }
 0x10781bf => {
  if (!cpu.cf && !cpu.zf) { pc = 0x10781c5; continue; }
  pc = 0x10781c1; continue;
 }
 0x10781c1 => {
  { let v: u16 = cpu.r16(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10781c5; continue;
 }
 0x10781c5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.esi), v); }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x20u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x1078209; continue; }
  pc = 0x10781cf; continue;
 }
 0x10781cf => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)) & 0xfeu8; cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  pc = 0x1078209; continue;
 }
 0x10781d5 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x1078209; continue; }
  pc = 0x10781db; continue;
 }
 0x10781db => {
  { let v: u16 = cpu.dec16(cpu.r16(0x4u32.wrapping_add(cpu.esi))); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x1078209; continue; }
  pc = 0x10781e1; continue;
 }
 0x10781e1 => {
  { let v: u16 = 0x4u16; cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x20u32.wrapping_add(cpu.esi))); }
  if cpu.sf { pc = 0x10781f7; continue; }
  pc = 0x10781f5; continue;
 }
 0x10781f5 => {
  if (!cpu.cf && !cpu.zf) { pc = 0x10781fb; continue; }
  pc = 0x10781f7; continue;
 }
 0x10781f7 => {
  { let v: u16 = cpu.r16(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10781fb; continue;
 }
 0x10781fb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.esi), v); }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x20u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x1078209; continue; }
  pc = 0x1078205; continue;
 }
 0x1078205 => {
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) & 0xbfu8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  pc = 0x1078209; continue;
 }
 0x1078209 => {
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x40u8); }
  if cpu.zf { pc = 0x10784c9; continue; }
  pc = 0x1078213; continue;
 }
 0x1078213 => {
  { let v: u32 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf8u32; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x112ca6cu32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as i8 as i16 as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as i8 as i16 as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.imul8(v); }
  { let v: u8 = 0x22u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.idiv8(v); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3e95u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.imul8(v); }
  { let v: u8 = 0x22u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.idiv8(v); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3e97u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x10782d4; continue; }
  pc = 0x1078282; continue;
 }
 0x1078282 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x10782d4; continue; }
  pc = 0x1078288; continue;
 }
 0x1078288 => {
  cpu.push(0x107828d); f_01078733(cpu); if cpu.returned_to != 0x107828d { return; }
  if cpu.cf { pc = 0x10782d4; continue; }
  pc = 0x107828f; continue;
 }
 0x107828f => {
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x200u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x107829c); f_0105f2ec(cpu); if cpu.returned_to != 0x107829c { return; }
  { cpu.sub16((cpu.eax as u16), 0x10u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10782d4; continue; }
  pc = 0x10782a2; continue;
 }
 0x10782a2 => {
  cpu.push(0x10782a7); f_0105f215(cpu); if cpu.returned_to != 0x10782a7 { return; }
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
  if !cpu.sf { pc = 0x10782d4; continue; }
  pc = 0x10782d0; continue;
 }
 0x10782d0 => {
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) | 0x2u8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  pc = 0x10782d4; continue;
 }
 0x10782d4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x10784c9; continue; }
  pc = 0x10782df; continue;
 }
 0x10782df => {
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x10784c9; continue; }
  pc = 0x10782e9; continue;
 }
 0x10782e9 => {
  cpu.push(0x10782ee); f_0105f215(cpu); if cpu.returned_to != 0x10782ee { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x10u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1078333; continue; }
  pc = 0x10782f8; continue;
 }
 0x10782f8 => {
  { cpu.sub16(cpu.r16(0x1eu32.wrapping_add(cpu.esi)), 0x22u16); }
  if cpu.zf { pc = 0x1078333; continue; }
  pc = 0x1078300; continue;
 }
 0x1078300 => {
  { let v: u16 = 0x4u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x14u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x107830d); f_0105f2ec(cpu); if cpu.returned_to != 0x107830d { return; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x22u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x107831e; continue; }
  pc = 0x107831a; continue;
 }
 0x107831a => {
  { let v: u16 = 0x22u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x107831e; continue;
 }
 0x107831e => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)) & 0xfeu8; cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) & 0xbfu8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x2u8); cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  pc = 0x10784c9; continue;
 }
 0x1078333 => {
  { cpu.sub16(cpu.r16(0x1eu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x10783ce; continue; }
  pc = 0x107833f; continue;
 }
 0x107833f => {
  { cpu.sub8(cpu.r8(0x2bu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x10783ce; continue; }
  pc = 0x1078349; continue;
 }
 0x1078349 => {
  cpu.push(0x107834e); f_0105f215(cpu); if cpu.returned_to != 0x107834e { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x10u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x107838f; continue; }
  pc = 0x1078358; continue;
 }
 0x1078358 => {
  { let v: u16 = 0xcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x14u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1078365); f_0105f2ec(cpu); if cpu.returned_to != 0x1078365 { return; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x1078376; continue; }
  pc = 0x1078372; continue;
 }
 0x1078372 => {
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1078376; continue;
 }
 0x1078376 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)) & 0xfdu8; cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) & 0xbfu8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x1u8); cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  pc = 0x10784c9; continue;
 }
 0x107838f => {
  { cpu.sub16((cpu.eax as u16), 0x20u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10783ce; continue; }
  pc = 0x1078395; continue;
 }
 0x1078395 => {
  { let v: u16 = 0x2u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x14u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x10783a2); f_0105f2ec(cpu); if cpu.returned_to != 0x10783a2 { return; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x10783b3; continue; }
  pc = 0x10783af; continue;
 }
 0x10783af => {
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10783b3; continue;
 }
 0x10783b3 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.esi)) & 0xfcu8; cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x40u8); cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0x4u16; cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  pc = 0x10784c9; continue;
 }
 0x10783ce => {
  { cpu.sub8(cpu.r8(0x1eu32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x10784c9; continue; }
  pc = 0x10783d8; continue;
 }
 0x10783d8 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x31u8); }
  if !cpu.zf { pc = 0x10784c9; continue; }
  pc = 0x10783e2; continue;
 }
 0x10783e2 => {
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x10783f0; continue; }
  pc = 0x10783e8; continue;
 }
 0x10783e8 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.esi))); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  pc = 0x10784c9; continue;
 }
 0x10783f0 => {
  { cpu.and8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x10784c9; continue; }
  pc = 0x10783fa; continue;
 }
 0x10783fa => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x40u8); }
  if !cpu.zf { pc = 0x10784c9; continue; }
  pc = 0x1078404; continue;
 }
 0x1078404 => {
  { cpu.sub16(cpu.r16(0x22u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x10784c9; continue; }
  pc = 0x1078410; continue;
 }
 0x1078410 => {
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x3fu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x10784c9; continue; }
  pc = 0x107841b; continue;
 }
 0x107841b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xc0u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x112cac5u32.wrapping_add(cpu.eax)) as i8 as i16 as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x112cac4u32.wrapping_add(cpu.eax)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3e93u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3e95u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1078461); f_0104f36d(cpu); if cpu.returned_to != 0x1078461 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x1078499; continue; }
  pc = 0x107846f; continue;
 }
 0x107846f => {
  { let v: u8 = cpu.r8(0x4001u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x1078499; continue; }
  pc = 0x107847d; continue;
 }
 0x107847d => {
  { let v: u8 = cpu.r8(0x4080u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x1078499; continue; }
  pc = 0x107848b; continue;
 }
 0x107848b => {
  { let v: u8 = cpu.r8(0x4081u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x107849c; continue; }
  pc = 0x1078499; continue;
 }
 0x1078499 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x10784c9; continue;
 }
 0x107849c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x3au32.wrapping_add(cpu.esi), v); }
  cpu.push(0x10784ac); f_0105f215(cpu); if cpu.returned_to != 0x10784ac { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x3fu16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10784c9; continue; }
  pc = 0x10784b6; continue;
 }
 0x10784b6 => {
  { let v: u32 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x8u8); cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  pc = 0x10784c9; continue;
 }
 0x10784c9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10784ca => {
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x10e55fau32); cpu.edx = v; }
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
  if cpu.zf { pc = 0x107851a; continue; }
  pc = 0x1078510; continue;
 }
 0x1078510 => {
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x9u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  pc = 0x107851a; continue;
 }
 0x107851a => {
  { cpu.sub16(cpu.r16(0x1eu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x107852d; continue; }
  pc = 0x1078521; continue;
 }
 0x1078521 => {
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x107852d; continue;
 }
 0x107852d => {
  { let v: u32 = cpu.add32(cpu.r32(0x10e401cu32), 0x10u32); cpu.w32(0x10e401cu32, v); }
  cpu.poll(); pc = 0x1078022; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01078539(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1078539); }
 let mut pc: u32 = 0x1078539;
 loop { match pc {
 0x1078539 => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x1078541; continue; }
  pc = 0x107853f; continue;
 }
 0x107853f => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1078541 => {
  { let v: u16 = 0x9u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x39u32.wrapping_add(cpu.esi)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1078552); f_010786e1(cpu); if cpu.returned_to != 0x1078552 { return; }
  { let v: u32 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.dec8(cpu.r8(0x31u32.wrapping_add(cpu.esi))); cpu.w8(0x31u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x2bu32.wrapping_add(cpu.esi))); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x2bu32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x107859a; continue; }
  pc = 0x1078569; continue;
 }
 0x1078569 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.esi))); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x107859a; continue; }
  pc = 0x1078579; continue;
 }
 0x1078579 => {
  { cpu.sub8(cpu.r8(0x2au32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x1078588; continue; }
  pc = 0x107857f; continue;
 }
 0x107857f => {
  { let v: u16 = cpu.r8(0x31u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  pc = 0x1078588; continue;
 }
 0x1078588 => {
  { let v: u8 = cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0xefu8); cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x14u8; cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x2cu32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x107853f; continue;
 }
 0x107859a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0107859c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x107859c); }
 let mut pc: u32 = 0x107859c;
 loop { match pc {
 0x107859c => {
  { cpu.and8(cpu.r8(0x33u32.wrapping_add(cpu.esi)), 0x20u8); }
  if !cpu.zf { pc = 0x10785a4; continue; }
  pc = 0x10785a2; continue;
 }
 0x10785a2 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10785a4 => {
  { let v: u32 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.inc8(cpu.r8(0x2bu32.wrapping_add(cpu.esi))); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x2bu32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1078618; continue; }
  pc = 0x10785bc; continue;
 }
 0x10785bc => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10785c2); f_0105f215(cpu); if cpu.returned_to != 0x10785c2 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x2u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.esi))); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x39u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x1078618; continue; }
  pc = 0x10785db; continue;
 }
 0x10785db => {
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) & 0xdfu8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.esi)) | 0x10u8; cpu.w8(0x33u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x40u16); }
  if !cpu.zf { pc = 0x10785f4; continue; }
  pc = 0x10785ed; continue;
 }
 0x10785ed => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)).wrapping_add(0x2u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x10785ff; continue;
 }
 0x10785f4 => {
  { cpu.sub16((cpu.eax as u16), 0xc0u16); }
  if !cpu.zf { pc = 0x10785ff; continue; }
  pc = 0x10785fa; continue;
 }
 0x10785fa => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)).wrapping_sub(0x2u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x10785ff; continue;
 }
 0x10785ff => {
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r8(0x2au32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xcu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2bu32.wrapping_add(cpu.esi), v); }
  pc = 0x1078618; continue;
 }
 0x1078618 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
