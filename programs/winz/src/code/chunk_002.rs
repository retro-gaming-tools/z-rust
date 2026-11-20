#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_00415b6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x415b6c); }
 let mut pc: u32 = 0x415b6c;
 loop { match pc {
 0x415b6c => {
  { cpu.sub32(cpu.r32(0x56c718u32), 0x0u32); }
  if !cpu.zf { pc = 0x415b95; continue; }
  pc = 0x415b75; continue;
 }
 0x415b75 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x56c720u32, v); }
  { let v: u32 = 0x509290u32; cpu.w32(0x56c71cu32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x56c718u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x588b6eu32, v); }
  pc = 0x415b95; continue;
 }
 0x415b95 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00415b96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x415b96); }
 let mut pc: u32 = 0x415b96;
 loop { match pc {
 0x415b96 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0xa9u32); }
  if (cpu.sf == cpu.of) { pc = 0x415c81; continue; }
  pc = 0x415ba6; continue;
 }
 0x415ba6 => {
  { cpu.sub32(cpu.r32(0x56c718u32), 0x0u32); }
  if cpu.zf { pc = 0x415c81; continue; }
  pc = 0x415bb3; continue;
 }
 0x415bb3 => {
  if cpu.sf { pc = 0x415c81; continue; }
  pc = 0x415bb9; continue;
 }
 0x415bb9 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x588b6eu32)); cpu.w8(0x588b6eu32, v); }
  if !cpu.sf { pc = 0x415c81; continue; }
  pc = 0x415bc5; continue;
 }
 0x415bc5 => {
  { let v: u8 = 0x11u8; cpu.w8(0x588b6eu32, v); }
  { let v: u32 = cpu.r32(0x59f446u32); cpu.eax = v; }
  cpu.push(0x415bd6); f_0045783d(cpu); if cpu.returned_to != 0x415bd6 { return; }
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x415c81; continue; }
  pc = 0x415be3; continue;
 }
 0x415be3 => {
  { let v: u32 = cpu.r32(0x56c71cu32); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x56c718u32), 0x2u32); }
  if !cpu.zf { pc = 0x415bfe; continue; }
  pc = 0x415bf2; continue;
 }
 0x415bf2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x56c718u32, v); }
  pc = 0x415c3b; continue;
 }
 0x415bfe => {
  { let v: u32 = cpu.r32(0x56c71cu32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0xffffffa4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x415c3b; continue; }
  pc = 0x415c0b; continue;
 }
 0x415c0b => {
  { let v: u8 = cpu.r8(0xffffff79u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x415c3b; continue; }
  pc = 0x415c16; continue;
 }
 0x415c16 => {
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56c720u32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x415c3b; continue; }
  pc = 0x415c21; continue;
 }
 0x415c21 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x415c3b; continue; }
  pc = 0x415c2a; continue;
 }
 0x415c2a => {
  { let v: u32 = cpu.ebp.wrapping_sub(0xb8u32); cpu.ebp = v; }
  cpu.push(0x415c35); f_00415c82(cpu); if cpu.returned_to != 0x415c35 { return; }
  { let v: u32 = cpu.r32(0x56c71cu32); cpu.ebp = v; }
  pc = 0x415c3b; continue;
 }
 0x415c3b => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x415c55; continue; }
  pc = 0x415c42; continue;
 }
 0x415c42 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x415c55; continue; }
  pc = 0x415c4a; continue;
 }
 0x415c4a => {
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56c720u32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x415c6e; continue; }
  pc = 0x415c55; continue;
 }
 0x415c55 => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x513f10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x415c3b; continue; }
  pc = 0x415c63; continue;
 }
 0x415c63 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x56c718u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x415c6e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x56c71cu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x415c81); f_00416cc3(cpu); if cpu.returned_to != 0x415c81 { return; }
  pc = 0x415c81; continue;
 }
 0x415c81 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00415c82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x415c82); }
 let mut pc: u32 = 0x415c82;
 loop { match pc {
 0x415c82 => {
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x415ccc; continue; }
  pc = 0x415c99; continue;
 }
 0x415c99 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x415ccc; continue; }
  pc = 0x415c9f; continue;
 }
 0x415c9f => {
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x415cc8; continue;
 }
 0x415ca5 => {
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x415ccc; continue; }
  pc = 0x415ca9; continue;
 }
 0x415ca9 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { pc = 0x415cb2; continue; }
  pc = 0x415caf; continue;
 }
 0x415caf => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x415ca5; continue;
 }
 0x415cb2 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x509290u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.pushad(); }
  cpu.push(0x415cc7); f_004586f4(cpu); if cpu.returned_to != 0x415cc7 { return; }
  { cpu.popad(); }
  pc = 0x415cc8; continue;
 }
 0x415cc8 => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x415ca5; continue; }
  pc = 0x415ccc; continue;
 }
 0x415ccc => {
  { cpu.popad(); }
  cpu.push(0x415cd2); f_004586f4(cpu); if cpu.returned_to != 0x415cd2 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00415cd3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x415cd3); }
 let mut pc: u32 = 0x415cd3;
 loop { match pc {
 0x415cd3 => {
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x415cf7; continue; }
  pc = 0x415cdd; continue;
 }
 0x415cdd => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x415cf7; continue; }
  pc = 0x415cf2; continue;
 }
 0x415cf2 => {
  cpu.push(0x415cf7); f_00416a62(cpu); if cpu.returned_to != 0x415cf7 { return; }
  pc = 0x415cf7; continue;
 }
 0x415cf7 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00415cf9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x415cf9); }
 let mut pc: u32 = 0x415cf9;
 loop { match pc {
 0x415cf9 => {
  { cpu.sub8(cpu.r8(0x58c88au32), 0x0u8); }
  if cpu.zf { pc = 0x415d03; continue; }
  pc = 0x415d02; continue;
 }
 0x415d02 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x415d03 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x415d5d; continue; }
  pc = 0x415d0f; continue;
 }
 0x415d0f => {
  { let v: u32 = 0x3ffu32; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x58c88bu32), 0x0u8); }
  if cpu.zf { pc = 0x415d22; continue; }
  pc = 0x415d1d; continue;
 }
 0x415d1d => {
  { let v: u32 = 0xffu32; cpu.ebp = v; }
  pc = 0x415d22; continue;
 }
 0x415d22 => {
  { let v: u16 = cpu.r16(0x56d184u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x415d2e); f_00416704(cpu); if cpu.returned_to != 0x415d2e { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d184u32, v); }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x415d3f; continue; }
  pc = 0x415d3a; continue;
 }
 0x415d3a => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x415d3f; continue;
 }
 0x415d3f => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x575885u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x592301u32, v); }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x415d5d; continue; }
  pc = 0x415d58; continue;
 }
 0x415d58 => {
  cpu.push(0x415d5d); f_00416a62(cpu); if cpu.returned_to != 0x415d5d { return; }
  pc = 0x415d5d; continue;
 }
 0x415d5d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00415d61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x415d61); }
 let mut pc: u32 = 0x415d61;
 loop { match pc {
 0x415d61 => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x415d92; continue; }
  pc = 0x415d65; continue;
 }
 0x415d65 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5cc64eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x415d7c; continue; }
  pc = 0x415d78; continue;
 }
 0x415d78 => {
  { let v: u16 = 0x0u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x415d7c; continue;
 }
 0x415d7c => {
  { cpu.sub16((cpu.edx as u16), 0x100u16); }
  if (cpu.sf != cpu.of) { pc = 0x415d87; continue; }
  pc = 0x415d83; continue;
 }
 0x415d83 => {
  { let v: u16 = 0xffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x415d87; continue;
 }
 0x415d87 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x415d8e); f_004030c0(cpu); if cpu.returned_to != 0x415d8e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { cpu.popad(); }
  pc = 0x415d92; continue;
 }
 0x415d92 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00415d93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x415d93); }
 let mut pc: u32 = 0x415d93;
 loop { match pc {
 0x415d93 => {
  { cpu.sub8(cpu.r8(0x5922f8u32), 0x0u8); }
  if !cpu.zf { pc = 0x415e0a; continue; }
  pc = 0x415da0; continue;
 }
 0x415da0 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6bfcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x56d18du32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x415db3); f_004030c0(cpu); if cpu.returned_to != 0x415db3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x2b11u32; cpu.eax = v; }
  { let v: u32 = 0x46u32; cpu.edx = v; }
  { let v: u16 = 0x2bu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x41c00eu32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x415dd1); f_00402fb0(cpu); if cpu.returned_to != 0x415dd1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  { cpu.popad(); }
  if cpu.sf { pc = 0x415e12; continue; }
  pc = 0x415dda; continue;
 }
 0x415dda => {
  { let v: u8 = 0x1u8; cpu.w8(0x5922f8u32, v); }
  cpu.push(0x415de6); f_00415f1b(cpu); if cpu.returned_to != 0x415de6 { return; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x415df2); f_004030c0(cpu); if cpu.returned_to != 0x415df2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x5922fbu32), 0x0u8); }
  if cpu.zf { pc = 0x415e0a; continue; }
  pc = 0x415dff; continue;
 }
 0x415dff => {
  { let v: u16 = 0x2u16; cpu.w16(0x4d699cu32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x415e0a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x415e12 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00415e2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x415e2b); }
 let mut pc: u32 = 0x415e2b;
 loop { match pc {
 0x415e2b => {
  { cpu.pushad(); }
  cpu.push(0x415e31); f_00401140(cpu); if cpu.returned_to != 0x415e31 { return; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x415e4e); f_00401100(cpu); if cpu.returned_to != 0x415e4e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00415e53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x415e53); }
 let mut pc: u32 = 0x415e53;
 loop { match pc {
 0x415e53 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x415e5c); f_00401080(cpu); if cpu.returned_to != 0x415e5c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x415eb8; continue; }
  pc = 0x415e65; continue;
 }
 0x415e65 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59237cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x592333u32; cpu.edi = v; }
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x592332u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x415e9e); f_00401040(cpu); if cpu.returned_to != 0x415e9e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x5922fdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x415eae); f_00401090(cpu); if cpu.returned_to != 0x415eae { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.push(0x415eb7); f_00415f1b(cpu); if cpu.returned_to != 0x415eb7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x415eb8 => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x415ec2; continue; }
  pc = 0x415ebd; continue;
 }
 0x415ebd => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x415ec2; continue;
 }
 0x415ec2 => {
  { let v: u32 = cpu.r32(0x5923b8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0x592333u32; cpu.edi = v; }
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x592332u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x415ee4); f_00401040(cpu); if cpu.returned_to != 0x415ee4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.push(0x415eed); f_00415f1b(cpu); if cpu.returned_to != 0x415eed { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00415eee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x415eee); }
 let mut pc: u32 = 0x415eee;
 loop { match pc {
 0x415eee => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x415ef7); f_00401010(cpu); if cpu.returned_to != 0x415ef7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x415f11); f_00401050(cpu); if cpu.returned_to != 0x415f11 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.push(0x415f1a); f_00415f1b(cpu); if cpu.returned_to != 0x415f1a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00415f1b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x415f1b); }
 let mut pc: u32 = 0x415f1b;
 loop { match pc {
 0x415f1b => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x5922ffu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x415f2a); f_00401150(cpu); if cpu.returned_to != 0x415f2a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5922fdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x415f3d); f_00401090(cpu); if cpu.returned_to != 0x415f3d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00415f42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x415f42); }
 let mut pc: u32 = 0x415f42;
 loop { match pc {
 0x415f42 => {
  { let v: u32 = 0x5923f4u32; cpu.edx = v; }
  { let v: u32 = 0x5bc434u32; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x415f57); f_00447961(cpu); if cpu.returned_to != 0x415f57 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5c4e71u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5c4e6du32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5c4e38u32, v); }
  cpu.push(0x415f7b); f_00448036(cpu); if cpu.returned_to != 0x415f7b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00415f7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x415f7c); }
 let mut pc: u32 = 0x415f7c;
 loop { match pc {
 0x415f7c => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x415f9f; continue; }
  pc = 0x415f85; continue;
 }
 0x415f85 => {
  { cpu.sub8(cpu.r8(0x5922f9u32), 0x0u8); }
  if cpu.zf { pc = 0x415fa0; continue; }
  pc = 0x415f8e; continue;
 }
 0x415f8e => {
  { cpu.sub16(cpu.r16(0x59dafbu32), 0x0u16); }
  if cpu.zf { pc = 0x415fa0; continue; }
  pc = 0x415f98; continue;
 }
 0x415f98 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x59dafbu32)); cpu.w16(0x59dafbu32, v); }
  pc = 0x415f9f; continue;
 }
 0x415f9f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x415fa0 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4160d6; continue; }
  pc = 0x415fad; continue;
 }
 0x415fad => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d306u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x4161c5; continue; }
  pc = 0x415fc1; continue;
 }
 0x415fc1 => {
  { let v: u32 = cpu.r32(0x59230bu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5b99fcu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b9a2cu32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5b9a2eu32, v); }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x416008); f_0041637f(cpu); if cpu.returned_to != 0x416008 { return; }
  if cpu.cf { pc = 0x41607f; continue; }
  pc = 0x41600e; continue;
 }
 0x41600e => {
  { let v: u8 = 0x0u8; cpu.w8(0x56c734u32, v); }
  { let v: u32 = cpu.r32(0x59230bu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5b9a2cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b9a2eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x416036); f_0041641c(cpu); if cpu.returned_to != 0x416036 { return; }
  if cpu.cf { pc = 0x41603d; continue; }
  pc = 0x416038; continue;
 }
 0x416038 => {
  pc = 0x4161c5; continue;
 }
 0x41603d => {
  { cpu.sub8(cpu.r8(0x4ba3dcu32), 0x7u8); }
  if cpu.zf { pc = 0x4162d4; continue; }
  pc = 0x41604a; continue;
 }
 0x41604a => {
  { cpu.sub8(cpu.r8(0x4ba3dcu32), 0x6u8); }
  if (cpu.sf == cpu.of) { pc = 0x416058; continue; }
  pc = 0x416053; continue;
 }
 0x416053 => {
  pc = 0x41626c; continue;
 }
 0x416058 => {
  { let v: u8 = 0x7u8; cpu.w8(0x4ba3dcu32, v); }
  { let v: u32 = 0xffu32; cpu.w32(0x575f6au32, v); }
  cpu.push(0x41606e); f_0041c6a6(cpu); if cpu.returned_to != 0x41606e { return; }
  { let v: u32 = cpu.eax & cpu.r32(0x5922ecu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5922e8u32)); cpu.eax = v; }
  pc = 0x41629f; continue;
 }
 0x41607f => {
  { cpu.sub8(cpu.r8(0x56c734u32), 0x0u8); }
  if !cpu.zf { pc = 0x416094; continue; }
  pc = 0x416088; continue;
 }
 0x416088 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56c734u32, v); }
  cpu.push(0x416094); f_00415a23(cpu); if cpu.returned_to != 0x416094 { return; }
  pc = 0x416094; continue;
 }
 0x416094 => {
  { cpu.sub8(cpu.r8(0x4ba3dcu32), 0x8u8); }
  if cpu.zf { pc = 0x4162d4; continue; }
  pc = 0x4160a1; continue;
 }
 0x4160a1 => {
  { cpu.sub8(cpu.r8(0x4ba3dcu32), 0x6u8); }
  if (cpu.sf == cpu.of) { pc = 0x4160af; continue; }
  pc = 0x4160aa; continue;
 }
 0x4160aa => {
  pc = 0x41626c; continue;
 }
 0x4160af => {
  { let v: u8 = 0x8u8; cpu.w8(0x4ba3dcu32, v); }
  { let v: u32 = 0xffu32; cpu.w32(0x575f6au32, v); }
  cpu.push(0x4160c5); f_0041c6a6(cpu); if cpu.returned_to != 0x4160c5 { return; }
  { let v: u32 = cpu.eax & cpu.r32(0x5922f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5922f0u32)); cpu.eax = v; }
  pc = 0x41629f; continue;
 }
 0x4160d6 => {
  { let v: u32 = cpu.r32(0x59230bu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x4160fd); f_00416310(cpu); if cpu.returned_to != 0x4160fd { return; }
  if !cpu.cf { pc = 0x416141; continue; }
  pc = 0x4160ff; continue;
 }
 0x4160ff => {
  { cpu.sub8(cpu.r8(0x4ba3dcu32), 0x7u8); }
  if cpu.zf { pc = 0x4162d4; continue; }
  pc = 0x41610c; continue;
 }
 0x41610c => {
  { cpu.sub8(cpu.r8(0x4ba3dcu32), 0x6u8); }
  if (cpu.sf == cpu.of) { pc = 0x41611a; continue; }
  pc = 0x416115; continue;
 }
 0x416115 => {
  pc = 0x41626c; continue;
 }
 0x41611a => {
  { let v: u8 = 0x7u8; cpu.w8(0x4ba3dcu32, v); }
  { let v: u32 = 0xffu32; cpu.w32(0x575f6au32, v); }
  cpu.push(0x416130); f_0041c6a6(cpu); if cpu.returned_to != 0x416130 { return; }
  { let v: u32 = cpu.eax & cpu.r32(0x5922ecu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5922e8u32)); cpu.eax = v; }
  pc = 0x41629f; continue;
 }
 0x416141 => {
  { let v: u32 = cpu.r32(0x59230bu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x416168); f_00416310(cpu); if cpu.returned_to != 0x416168 { return; }
  if !cpu.cf { pc = 0x4161c5; continue; }
  pc = 0x41616e; continue;
 }
 0x41616e => {
  { cpu.sub8(cpu.r8(0x56c734u32), 0x0u8); }
  if !cpu.zf { pc = 0x416183; continue; }
  pc = 0x416177; continue;
 }
 0x416177 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56c734u32, v); }
  cpu.push(0x416183); f_00415a23(cpu); if cpu.returned_to != 0x416183 { return; }
  pc = 0x416183; continue;
 }
 0x416183 => {
  { cpu.sub8(cpu.r8(0x4ba3dcu32), 0x8u8); }
  if cpu.zf { pc = 0x4162d4; continue; }
  pc = 0x416190; continue;
 }
 0x416190 => {
  { cpu.sub8(cpu.r8(0x4ba3dcu32), 0x6u8); }
  if (cpu.sf == cpu.of) { pc = 0x41619e; continue; }
  pc = 0x416199; continue;
 }
 0x416199 => {
  pc = 0x41626c; continue;
 }
 0x41619e => {
  { let v: u8 = 0x8u8; cpu.w8(0x4ba3dcu32, v); }
  { let v: u32 = 0xffu32; cpu.w32(0x575f6au32, v); }
  cpu.push(0x4161b4); f_0041c6a6(cpu); if cpu.returned_to != 0x4161b4 { return; }
  { let v: u32 = cpu.eax & cpu.r32(0x5922f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5922f0u32)); cpu.eax = v; }
  pc = 0x41629f; continue;
 }
 0x4161c5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x56c734u32, v); }
  { let v: u16 = cpu.r16(0x4b94ccu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x5u16); }
  if (cpu.sf == cpu.of) { pc = 0x41626c; continue; }
  pc = 0x4161dc; continue;
 }
 0x4161dc => {
  { cpu.sub16((cpu.eax as u16), 0x4u16); }
  if (cpu.sf == cpu.of) { pc = 0x416255; continue; }
  pc = 0x4161e6; continue;
 }
 0x4161e6 => {
  { cpu.sub16((cpu.eax as u16), 0x3u16); }
  if (cpu.sf == cpu.of) { pc = 0x416247; continue; }
  pc = 0x4161f0; continue;
 }
 0x4161f0 => {
  { cpu.sub16((cpu.eax as u16), 0x2u16); }
  if (cpu.sf == cpu.of) { pc = 0x416230; continue; }
  pc = 0x4161f6; continue;
 }
 0x4161f6 => {
  { cpu.sub16((cpu.eax as u16), 0x1u16); }
  if (cpu.sf == cpu.of) { pc = 0x416216; continue; }
  pc = 0x4161fc; continue;
 }
 0x4161fc => {
  { let v: u32 = 0x5922d8u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x4ba3dcu32), 0x7u8); }
  if (cpu.sf == cpu.of) { pc = 0x416247; continue; }
  pc = 0x41620a; continue;
 }
 0x41620a => {
  { let v: u8 = 0x1u8; cpu.w8(0x59230fu32, v); }
  pc = 0x416278; continue;
 }
 0x416216 => {
  { let v: u32 = 0x5922c8u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x4ba3dcu32), 0x7u8); }
  if (cpu.sf == cpu.of) { pc = 0x416247; continue; }
  pc = 0x416224; continue;
 }
 0x416224 => {
  { let v: u8 = 0x2u8; cpu.w8(0x59230fu32, v); }
  pc = 0x416278; continue;
 }
 0x416230 => {
  { let v: u32 = 0x5922b8u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x4ba3dcu32), 0x7u8); }
  if (cpu.sf == cpu.of) { pc = 0x416247; continue; }
  pc = 0x41623e; continue;
 }
 0x41623e => {
  { let v: u8 = 0x3u8; cpu.w8(0x59230fu32, v); }
  pc = 0x416278; continue;
 }
 0x416247 => {
  { let v: u32 = 0x5922a8u32; cpu.esi = v; }
  { let v: u8 = 0x4u8; cpu.w8(0x59230fu32, v); }
  pc = 0x416278; continue;
 }
 0x416255 => {
  { let v: u32 = 0x592298u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x4ba3dcu32), 0x7u8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x416247; continue; }
  pc = 0x416263; continue;
 }
 0x416263 => {
  { let v: u8 = 0x5u8; cpu.w8(0x59230fu32, v); }
  pc = 0x416278; continue;
 }
 0x41626c => {
  { let v: u32 = 0x592288u32; cpu.esi = v; }
  { let v: u8 = 0x7u8; cpu.w8(0x59230fu32, v); }
  pc = 0x416278; continue;
 }
 0x416278 => {
  { let v: u8 = cpu.r8(0x59230fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4ba3dcu32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x4162d4; continue; }
  pc = 0x416285; continue;
 }
 0x416285 => {
  { let v: u32 = 0xffu32; cpu.w32(0x575f6au32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4ba3dcu32, v); }
  cpu.push(0x416299); f_0041c6a6(cpu); if cpu.returned_to != 0x416299 { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x41629f; continue;
 }
 0x41629f => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x592310u32), 0x0u8); }
  if cpu.zf { pc = 0x4162b9; continue; }
  pc = 0x4162b4; continue;
 }
 0x4162b4 => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  pc = 0x4162b9; continue;
 }
 0x4162b9 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.edx; cpu.w32(0x4ba3e0u32, v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4162c8); f_00401050(cpu); if cpu.returned_to != 0x4162c8 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x59dafbu32, v); }
  pc = 0x4162d3; continue;
 }
 0x4162d3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4162d4 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x575f6au32)); cpu.w32(0x575f6au32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x4162d3; continue; }
  pc = 0x4162dc; continue;
 }
 0x4162dc => {
  { let v: u8 = 0x0u8; cpu.w8(0x56c734u32, v); }
  { let v: u32 = 0x5922d8u32; cpu.esi = v; }
  cpu.poll(); pc = 0x416285; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004162eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4162eb); }
 let mut pc: u32 = 0x4162eb;
 loop { match pc {
 0x4162eb => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b64u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.r32(0x4d6b64u32), cpu.eax); cpu.w32(0x4d6b64u32, v); }
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416310(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x416310); } r_00416310(cpu, 0x416310); }
pub fn f_0041636f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41636f); } r_00416310(cpu, 0x41636f); }
fn r_00416310(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x416310 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x592302u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x592304u32, v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x592306u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x592307u32, v); }
  { let v: u32 = 0x509290u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x416333; continue;
 }
 0x416333 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x416373; continue; }
  pc = 0x416339; continue;
 }
 0x416339 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x592306u32)); }
  if !cpu.zf { pc = 0x416373; continue; }
  pc = 0x416346; continue;
 }
 0x416346 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x592302u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x592304u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x41636a); f_004162eb(cpu); if cpu.returned_to != 0x41636a { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x592307u32), cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x41637d; continue; }
  pc = 0x416373; continue;
 }
 0x41636f => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x416373; continue;
 }
 0x416373 => {
  { let v: u32 = cpu.add32(cpu.edi, 0xb8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x416333; continue; }
  pc = 0x41637b; continue;
 }
 0x41637b => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41637d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041637f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41637f); }
 let mut pc: u32 = 0x41637f;
 loop { match pc {
 0x41637f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x592302u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x592304u32, v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x592306u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x592307u32, v); }
  { let v: u32 = 0x509290u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x4163a2; continue;
 }
 0x4163a2 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x416410; continue; }
  pc = 0x4163ac; continue;
 }
 0x4163ac => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x592306u32)); }
  if cpu.zf { pc = 0x416410; continue; }
  pc = 0x4163bd; continue;
 }
 0x4163bd => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4163e3; continue; }
  pc = 0x4163c6; continue;
 }
 0x4163c6 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x592306u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ecx as u8)); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.zf { pc = 0x416410; continue; }
  pc = 0x4163e3; continue;
 }
 0x4163e3 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x592302u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x592304u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x416407); f_004162eb(cpu); if cpu.returned_to != 0x416407 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x592307u32), cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x41641a; continue; }
  pc = 0x416410; continue;
 }
 0x416410 => {
  { let v: u32 = cpu.add32(cpu.edi, 0xb8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4163a2; continue; }
  pc = 0x416418; continue;
 }
 0x416418 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41641a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041641c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41641c); }
 let mut pc: u32 = 0x41641c;
 loop { match pc {
 0x41641c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x592302u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x592304u32, v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x592306u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x592307u32, v); }
  { let v: u32 = 0x509290u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x41643f; continue;
 }
 0x41643f => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x4164dc; continue; }
  pc = 0x416449; continue;
 }
 0x416449 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x592306u32)); }
  if !cpu.zf { pc = 0x4164dc; continue; }
  pc = 0x41645a; continue;
 }
 0x41645a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = 0x5b99fcu32; cpu.esi = v; }
  pc = 0x416465; continue;
 }
 0x416465 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x4d67f6u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4164d1; continue; }
  pc = 0x41647b; continue;
 }
 0x41647b => {
  { cpu.sub8(cpu.r8(0x56d301u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4164d1; continue; }
  pc = 0x416485; continue;
 }
 0x416485 => {
  { cpu.sub8(cpu.r8(0x56d305u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4164d1; continue; }
  pc = 0x41648f; continue;
 }
 0x41648f => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x50u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x50u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x592302u32)); }
  if !cpu.zf { pc = 0x4164c4; continue; }
  pc = 0x4164bb; continue;
 }
 0x4164bb => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x592304u32)); }
  if cpu.zf { pc = 0x4164d1; continue; }
  pc = 0x4164c4; continue;
 }
 0x4164c4 => {
  cpu.push(0x4164c9); f_004162eb(cpu); if cpu.returned_to != 0x4164c9 { return; }
  { cpu.sub32(cpu.r32(0x592307u32), cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x4164eb; continue; }
  pc = 0x4164d1; continue;
 }
 0x4164d1 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x5u32); }
  if !cpu.zf { cpu.poll(); pc = 0x416465; continue; }
  pc = 0x4164db; continue;
 }
 0x4164db => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x4164dc; continue;
 }
 0x4164dc => {
  { let v: u32 = cpu.edi.wrapping_add(0xb8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x41643f; continue; }
  pc = 0x4164e9; continue;
 }
 0x4164e9 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4164eb => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004164ef(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4164ef); } r_004164ef(cpu, 0x4164ef); }
pub fn f_00416567(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x416567); } r_004164ef(cpu, 0x416567); }
pub fn f_0041656c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41656c); } r_004164ef(cpu, 0x41656c); }
pub fn f_0041656d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41656d); } r_004164ef(cpu, 0x41656d); }
pub fn f_0041656e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41656e); } r_004164ef(cpu, 0x41656e); }
pub fn f_00416570(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x416570); } r_004164ef(cpu, 0x416570); }
pub fn f_00416574(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x416574); } r_004164ef(cpu, 0x416574); }
fn r_004164ef(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4164ef => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x4165a1; continue; }
  pc = 0x4164fc; continue;
 }
 0x4164fc => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x416565; continue; }
  pc = 0x416509; continue;
 }
 0x416509 => {
  { let v: u16 = cpu.r16(0xa1u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10cu32; cpu.ebx = v; }
  { cpu.sub16((cpu.edx as u16), 0x7950u16); }
  if cpu.zf { pc = 0x41655b; continue; }
  pc = 0x41651d; continue;
 }
 0x41651d => {
  { let v: u32 = 0x10bu32; cpu.ebx = v; }
  { cpu.sub16((cpu.edx as u16), 0x614cu16); }
  if cpu.zf { pc = 0x41655b; continue; }
  pc = 0x416529; continue;
 }
 0x416529 => {
  { let v: u32 = 0x10au32; cpu.ebx = v; }
  { cpu.sub16((cpu.edx as u16), 0x6f54u16); }
  if cpu.zf { pc = 0x41655b; continue; }
  pc = 0x416535; continue;
 }
 0x416535 => {
  { let v: u32 = 0x108u32; cpu.ebx = v; }
  { cpu.sub16((cpu.edx as u16), 0x7350u16); }
  if cpu.zf { pc = 0x41655b; continue; }
  pc = 0x416541; continue;
 }
 0x416540 => {
  { let v: u8 = cpu.sbb8(((cpu.ebx >> 8) as u8), cpu.r8(0x109u32.wrapping_add(cpu.ebx))); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.edx as u16), 0x6e53u16); }
  if cpu.zf { pc = 0x41655b; continue; }
  pc = 0x41654d; continue;
 }
 0x416541 => {
  { let v: u32 = 0x109u32; cpu.ebx = v; }
  { cpu.sub16((cpu.edx as u16), 0x6e53u16); }
  if cpu.zf { pc = 0x41655b; continue; }
  pc = 0x41654d; continue;
 }
 0x41654d => {
  { let v: u32 = 0x107u32; cpu.ebx = v; }
  { cpu.sub16((cpu.edx as u16), 0x7247u16); }
  if cpu.zf { pc = 0x41655b; continue; }
  pc = 0x416559; continue;
 }
 0x416559 => {
  pc = 0x416565; continue;
 }
 0x41655b => {
  cpu.push(0x416560); f_0041c6a6(cpu); if cpu.returned_to != 0x416560 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x1u32); cpu.eax = v; }
  if cpu.zf { pc = 0x41658c; continue; }
  pc = 0x416565; continue;
 }
 0x416565 => {
  { let v: u32 = 0x3fu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x56d170u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x416576); f_00416704(cpu); if cpu.returned_to != 0x416576 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d170u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x575899u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  pc = 0x41658c; continue;
 }
 0x416567 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0xffffff8bu32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff8bu32.wrapping_add(cpu.esi), v); }
  pc = 0x41656c; continue;
 }
 0x41656c => {
  { let v: u32 = cpu.sbb32(cpu.eax, 0x56d170u32); cpu.eax = v; }
  cpu.push(0x416576); f_00416704(cpu); if cpu.returned_to != 0x416576 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d170u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x575899u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  pc = 0x41658c; continue;
 }
 0x41656d => {
  if cpu.of { cpu.poll(); pc = 0x416540; continue; }
  pc = 0x41656f; continue;
 }
 0x41656e => {
  { let v: u32 = cpu.rcl32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 1u32); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x416576); f_00416704(cpu); if cpu.returned_to != 0x416576 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d170u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x575899u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  pc = 0x41658c; continue;
 }
 0x41656f => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x416570; continue;
 }
 0x416570 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let _ = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); }
  pc = 0x416574; continue;
 }
 0x416574 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d170u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x575899u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  pc = 0x41658c; continue;
 }
 0x41658c => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c10u32, v); }
  cpu.push(0x4165a0); f_00447c27(cpu); if cpu.returned_to != 0x4165a0 { return; }
  { cpu.popad(); }
  pc = 0x4165a1; continue;
 }
 0x4165a1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004165a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4165a2); }
 let mut pc: u32 = 0x4165a2;
 loop { match pc {
 0x4165a2 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x4165f9; continue; }
  pc = 0x4165ac; continue;
 }
 0x4165ac => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x4165be; continue; }
  pc = 0x4165b5; continue;
 }
 0x4165b5 => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x4165f9; continue; }
  pc = 0x4165be; continue;
 }
 0x4165be => {
  { let v: u32 = 0x3fu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x56d186u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4165cf); f_00416704(cpu); if cpu.returned_to != 0x4165cf { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d186u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5758b7u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c10u32, v); }
  cpu.push(0x4165f9); f_00447c27(cpu); if cpu.returned_to != 0x4165f9 { return; }
  pc = 0x4165f9; continue;
 }
 0x4165f9 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004165fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4165fb); }
 let mut pc: u32 = 0x4165fb;
 loop { match pc {
 0x4165fb => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x416652; continue; }
  pc = 0x416605; continue;
 }
 0x416605 => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x416617; continue; }
  pc = 0x41660e; continue;
 }
 0x41660e => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x416652; continue; }
  pc = 0x416617; continue;
 }
 0x416617 => {
  { let v: u32 = 0xffu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x56d172u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x416628); f_00416704(cpu); if cpu.returned_to != 0x416628 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d172u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5758a5u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c10u32, v); }
  cpu.push(0x416652); f_00447c27(cpu); if cpu.returned_to != 0x416652 { return; }
  pc = 0x416652; continue;
 }
 0x416652 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416654(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x416654); } r_00416654(cpu, 0x416654); }
pub fn f_00416674(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x416674); } r_00416654(cpu, 0x416674); }
fn r_00416654(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x416654 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x4166ab; continue; }
  pc = 0x41665e; continue;
 }
 0x41665e => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x416670; continue; }
  pc = 0x416667; continue;
 }
 0x416667 => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x4166ab; continue; }
  pc = 0x416670; continue;
 }
 0x416670 => {
  { let v: u32 = 0x7u32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x56d174u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x416681); f_00416704(cpu); if cpu.returned_to != 0x416681 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d174u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5758c3u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c10u32, v); }
  cpu.push(0x4166ab); f_00447c27(cpu); if cpu.returned_to != 0x4166ab { return; }
  pc = 0x4166ab; continue;
 }
 0x416674 => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffff8bu32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff8bu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.sbb32(cpu.eax, 0x56d174u32); cpu.eax = v; }
  cpu.push(0x416681); f_00416704(cpu); if cpu.returned_to != 0x416681 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d174u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5758c3u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c10u32, v); }
  cpu.push(0x4166ab); f_00447c27(cpu); if cpu.returned_to != 0x4166ab { return; }
  pc = 0x4166ab; continue;
 }
 0x4166ab => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004166ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4166ad); }
 let mut pc: u32 = 0x4166ad;
 loop { match pc {
 0x4166ad => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x416702; continue; }
  pc = 0x4166b7; continue;
 }
 0x4166b7 => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x4166c9; continue; }
  pc = 0x4166c0; continue;
 }
 0x4166c0 => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x416702; continue; }
  pc = 0x4166c9; continue;
 }
 0x4166c9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x7fu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x56d17eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4166db); f_00416704(cpu); if cpu.returned_to != 0x4166db { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d17eu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x575909u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6c10u32, v); }
  cpu.push(0x416702); f_00447c27(cpu); if cpu.returned_to != 0x416702 { return; }
  pc = 0x416702; continue;
 }
 0x416702 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416704(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416704); }
 let mut pc: u32 = 0x416704;
 loop { match pc {
 0x416704 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  pc = 0x41670c; continue;
 }
 0x41670c => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x41670c; continue; }
  pc = 0x416711; continue;
 }
 0x416711 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x416718); f_0041c845(cpu); if cpu.returned_to != 0x416718 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & cpu.ebp; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { pc = 0x416726; continue; }
  pc = 0x416721; continue;
 }
 0x416721 => {
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  pc = 0x416726; continue;
 }
 0x416726 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x416732; continue; }
  pc = 0x416730; continue;
 }
 0x416730 => {
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  pc = 0x416732; continue;
 }
 0x416732 => {
  { cpu.and16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x416748; continue; }
  pc = 0x416737; continue;
 }
 0x416737 => {
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, cpu.ebp); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x416732; continue; }
  pc = 0x41673f; continue;
 }
 0x41673f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x416732; continue;
 }
 0x416748 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041674d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41674d); } r_0041674d(cpu, 0x41674d); }
pub fn f_0041676e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41676e); } r_0041674d(cpu, 0x41676e); }
fn r_0041674d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x41674d => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x4167f3; continue; }
  pc = 0x41675b; continue;
 }
 0x41675b => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x416771; continue; }
  pc = 0x416764; continue;
 }
 0x416764 => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x4167f3; continue; }
  pc = 0x416771; continue;
 }
 0x41676e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x48u32.wrapping_add(cpu.eax)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x48u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x40u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x4167f3; continue; }
  pc = 0x416788; continue;
 }
 0x416771 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x40u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x4167f3; continue; }
  pc = 0x416788; continue;
 }
 0x416788 => {
  { cpu.sub8(cpu.r8(0x5du32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4167f3; continue; }
  pc = 0x41678e; continue;
 }
 0x41678e => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x4167f3; continue; }
  pc = 0x41679b; continue;
 }
 0x41679b => {
  { let v: u32 = 0x1fu32; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x58c88bu32), 0x0u8); }
  if cpu.zf { pc = 0x4167ae; continue; }
  pc = 0x4167a9; continue;
 }
 0x4167a9 => {
  { let v: u32 = 0xfu32; cpu.ebp = v; }
  pc = 0x4167ae; continue;
 }
 0x4167ae => {
  { let v: u16 = cpu.r16(0x56d180u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4167ba); f_00416704(cpu); if cpu.returned_to != 0x4167ba { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d180u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5758f9u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6c10u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4167e1); f_00447c27(cpu); if cpu.returned_to != 0x4167e1 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x52bbf0u32); cpu.eax = v; }
  { let v: u8 = 0x44u8; cpu.w8(0x40u32.wrapping_add(cpu.eax), v); }
  pc = 0x4167f3; continue;
 }
 0x4167f3 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004167f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4167f5); }
 let mut pc: u32 = 0x4167f5;
 loop { match pc {
 0x4167f5 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x416873; continue; }
  pc = 0x416804; continue;
 }
 0x416804 => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x416816; continue; }
  pc = 0x41680d; continue;
 }
 0x41680d => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x416873; continue; }
  pc = 0x416816; continue;
 }
 0x416816 => {
  { let v: u8 = 0x11u8; cpu.w8(0x40u32.wrapping_add(cpu.eax), v); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x41686c; continue; }
  pc = 0x416820; continue;
 }
 0x416820 => {
  cpu.push(0x416825); f_0046b31b(cpu); if cpu.returned_to != 0x416825 { return; }
  if cpu.zf { pc = 0x41686c; continue; }
  pc = 0x416827; continue;
 }
 0x416827 => {
  { let v: u32 = 0x7u32; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x58c88bu32), 0x0u8); }
  if cpu.zf { pc = 0x41683a; continue; }
  pc = 0x416835; continue;
 }
 0x416835 => {
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  pc = 0x41683a; continue;
 }
 0x41683a => {
  { let v: u16 = cpu.r16(0x56d182u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x416846); f_00416704(cpu); if cpu.returned_to != 0x416846 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d182u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x575903u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6c10u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x41686c); f_00447c27(cpu); if cpu.returned_to != 0x41686c { return; }
  pc = 0x41686c; continue;
 }
 0x41686c => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x40u32.wrapping_add(cpu.eax), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x416873 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416876(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416876); }
 let mut pc: u32 = 0x416876;
 loop { match pc {
 0x416876 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x4168c1; continue; }
  pc = 0x416880; continue;
 }
 0x416880 => {
  { cpu.sub32(cpu.r32(0x4d6c58u32), 0x725u32); }
  if cpu.zf { pc = 0x4168c1; continue; }
  pc = 0x41688c; continue;
 }
 0x41688c => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x41689e; continue; }
  pc = 0x416895; continue;
 }
 0x416895 => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x4168c1; continue; }
  pc = 0x41689e; continue;
 }
 0x41689e => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x4168c1; continue; }
  pc = 0x4168ac; continue;
 }
 0x4168ac => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6c10u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x725u32; cpu.ebx = v; }
  cpu.push(0x4168c1); f_00447c27(cpu); if cpu.returned_to != 0x4168c1 { return; }
  pc = 0x4168c1; continue;
 }
 0x4168c1 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004168c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4168c3); }
 let mut pc: u32 = 0x4168c3;
 loop { match pc {
 0x4168c3 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x416902; continue; }
  pc = 0x4168cd; continue;
 }
 0x4168cd => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x4168df; continue; }
  pc = 0x4168d6; continue;
 }
 0x4168d6 => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x416902; continue; }
  pc = 0x4168df; continue;
 }
 0x4168df => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x416902; continue; }
  pc = 0x4168ed; continue;
 }
 0x4168ed => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6c10u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x726u32; cpu.ebx = v; }
  cpu.push(0x416902); f_00447c27(cpu); if cpu.returned_to != 0x416902 { return; }
  pc = 0x416902; continue;
 }
 0x416902 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416904(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416904); }
 let mut pc: u32 = 0x416904;
 loop { match pc {
 0x416904 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x416947; continue; }
  pc = 0x41690e; continue;
 }
 0x41690e => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x416920; continue; }
  pc = 0x416917; continue;
 }
 0x416917 => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x416947; continue; }
  pc = 0x416920; continue;
 }
 0x416920 => {
  { let v: u32 = cpu.or32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if cpu.zf { pc = 0x416947; continue; }
  pc = 0x416924; continue;
 }
 0x416924 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x416947; continue; }
  pc = 0x416932; continue;
 }
 0x416932 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6c10u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x82fu32; cpu.ebx = v; }
  cpu.push(0x416947); f_00447c27(cpu); if cpu.returned_to != 0x416947 { return; }
  pc = 0x416947; continue;
 }
 0x416947 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416949(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416949); }
 let mut pc: u32 = 0x416949;
 loop { match pc {
 0x416949 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x41698c; continue; }
  pc = 0x416953; continue;
 }
 0x416953 => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x416965; continue; }
  pc = 0x41695c; continue;
 }
 0x41695c => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x41698c; continue; }
  pc = 0x416965; continue;
 }
 0x416965 => {
  { let v: u32 = cpu.or32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if cpu.zf { pc = 0x41698c; continue; }
  pc = 0x416969; continue;
 }
 0x416969 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x41698c; continue; }
  pc = 0x416977; continue;
 }
 0x416977 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6c10u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x727u32; cpu.ebx = v; }
  cpu.push(0x41698c); f_00447c27(cpu); if cpu.returned_to != 0x41698c { return; }
  pc = 0x41698c; continue;
 }
 0x41698c => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041698e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41698e); }
 let mut pc: u32 = 0x41698e;
 loop { match pc {
 0x41698e => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x4169d1; continue; }
  pc = 0x416998; continue;
 }
 0x416998 => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x4169aa; continue; }
  pc = 0x4169a1; continue;
 }
 0x4169a1 => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x4169d1; continue; }
  pc = 0x4169aa; continue;
 }
 0x4169aa => {
  { let v: u32 = cpu.or32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if cpu.zf { pc = 0x4169d1; continue; }
  pc = 0x4169ae; continue;
 }
 0x4169ae => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x4169d1; continue; }
  pc = 0x4169bc; continue;
 }
 0x4169bc => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6c10u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x830u32; cpu.ebx = v; }
  cpu.push(0x4169d1); f_00447c27(cpu); if cpu.returned_to != 0x4169d1 { return; }
  pc = 0x4169d1; continue;
 }
 0x4169d1 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004169d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4169d3); }
 let mut pc: u32 = 0x4169d3;
 loop { match pc {
 0x4169d3 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x416a12; continue; }
  pc = 0x4169dd; continue;
 }
 0x4169dd => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x4169ef; continue; }
  pc = 0x4169e6; continue;
 }
 0x4169e6 => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x416a12; continue; }
  pc = 0x4169ef; continue;
 }
 0x4169ef => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x416a12; continue; }
  pc = 0x4169fd; continue;
 }
 0x4169fd => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6c10u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x831u32; cpu.ebx = v; }
  cpu.push(0x416a12); f_00447c27(cpu); if cpu.returned_to != 0x416a12 { return; }
  pc = 0x416a12; continue;
 }
 0x416a12 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416a14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416a14); }
 let mut pc: u32 = 0x416a14;
 loop { match pc {
 0x416a14 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x416a1f); f_00416a62(cpu); if cpu.returned_to != 0x416a1f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416a21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416a21); }
 let mut pc: u32 = 0x416a21;
 loop { match pc {
 0x416a21 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x416a2c); f_00416a62(cpu); if cpu.returned_to != 0x416a2c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416a48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416a48); }
 let mut pc: u32 = 0x416a48;
 loop { match pc {
 0x416a48 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.push(0x416a53); f_00416a62(cpu); if cpu.returned_to != 0x416a53 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416a55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416a55); }
 let mut pc: u32 = 0x416a55;
 loop { match pc {
 0x416a55 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  cpu.push(0x416a60); f_00416a62(cpu); if cpu.returned_to != 0x416a60 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416a62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416a62); }
 let mut pc: u32 = 0x416a62;
 loop { match pc {
 0x416a62 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d306u32.wrapping_add(cpu.eax)), 0xffu8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x416a92; continue; }
  pc = 0x416a74; continue;
 }
 0x416a74 => {
  { let v = cpu.eax; cpu.push(v); }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x416a7c); f_00401110(cpu); if cpu.returned_to != 0x416a7c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = 0x56cc1eu32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x56c737u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x416a92; continue;
 }
 0x416a92 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416ad8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416ad8); }
 let mut pc: u32 = 0x416ad8;
 loop { match pc {
 0x416ad8 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x416b1b; continue; }
  pc = 0x416ae2; continue;
 }
 0x416ae2 => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x416af4; continue; }
  pc = 0x416aeb; continue;
 }
 0x416aeb => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x416b1b; continue; }
  pc = 0x416af4; continue;
 }
 0x416af4 => {
  { let v: u32 = cpu.or32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if cpu.zf { pc = 0x416b1b; continue; }
  pc = 0x416af8; continue;
 }
 0x416af8 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x416b1b; continue; }
  pc = 0x416b06; continue;
 }
 0x416b06 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6c10u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x833u32; cpu.ebx = v; }
  cpu.push(0x416b1b); f_00447c27(cpu); if cpu.returned_to != 0x416b1b { return; }
  pc = 0x416b1b; continue;
 }
 0x416b1b => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416b1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416b1d); }
 let mut pc: u32 = 0x416b1d;
 loop { match pc {
 0x416b1d => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x416b60; continue; }
  pc = 0x416b27; continue;
 }
 0x416b27 => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x416b39; continue; }
  pc = 0x416b30; continue;
 }
 0x416b30 => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x416b60; continue; }
  pc = 0x416b39; continue;
 }
 0x416b39 => {
  { let v: u32 = cpu.or32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if cpu.zf { pc = 0x416b60; continue; }
  pc = 0x416b3d; continue;
 }
 0x416b3d => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x416b60; continue; }
  pc = 0x416b4b; continue;
 }
 0x416b4b => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6c10u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x834u32; cpu.ebx = v; }
  cpu.push(0x416b60); f_00447c27(cpu); if cpu.returned_to != 0x416b60 { return; }
  pc = 0x416b60; continue;
 }
 0x416b60 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416b62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416b62); }
 let mut pc: u32 = 0x416b62;
 loop { match pc {
 0x416b62 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x416ba5; continue; }
  pc = 0x416b6c; continue;
 }
 0x416b6c => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x416b7e; continue; }
  pc = 0x416b75; continue;
 }
 0x416b75 => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x416ba5; continue; }
  pc = 0x416b7e; continue;
 }
 0x416b7e => {
  { let v: u32 = cpu.or32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if cpu.zf { pc = 0x416ba5; continue; }
  pc = 0x416b82; continue;
 }
 0x416b82 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x416ba5; continue; }
  pc = 0x416b90; continue;
 }
 0x416b90 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6c10u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x835u32; cpu.ebx = v; }
  cpu.push(0x416ba5); f_00447c27(cpu); if cpu.returned_to != 0x416ba5 { return; }
  pc = 0x416ba5; continue;
 }
 0x416ba5 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416ba7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416ba7); }
 let mut pc: u32 = 0x416ba7;
 loop { match pc {
 0x416ba7 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x416bfc; continue; }
  pc = 0x416bb1; continue;
 }
 0x416bb1 => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x416bc3; continue; }
  pc = 0x416bba; continue;
 }
 0x416bba => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x416bfc; continue; }
  pc = 0x416bc3; continue;
 }
 0x416bc3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0xfu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x56d176u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x416bd5); f_00416704(cpu); if cpu.returned_to != 0x416bd5 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d176u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5758c9u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6c10u32, v); }
  cpu.push(0x416bfc); f_00447c27(cpu); if cpu.returned_to != 0x416bfc { return; }
  pc = 0x416bfc; continue;
 }
 0x416bfc => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416bfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416bfe); }
 let mut pc: u32 = 0x416bfe;
 loop { match pc {
 0x416bfe => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x416c53; continue; }
  pc = 0x416c08; continue;
 }
 0x416c08 => {
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x416c1a; continue; }
  pc = 0x416c11; continue;
 }
 0x416c11 => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x416c53; continue; }
  pc = 0x416c1a; continue;
 }
 0x416c1a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x7fu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x56d178u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x416c2c); f_00416704(cpu); if cpu.returned_to != 0x416c2c { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d178u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5758d1u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6c10u32, v); }
  cpu.push(0x416c53); f_00447c27(cpu); if cpu.returned_to != 0x416c53 { return; }
  pc = 0x416c53; continue;
 }
 0x416c53 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416c55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416c55); }
 let mut pc: u32 = 0x416c55;
 loop { match pc {
 0x416c55 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x416c68; continue; }
  pc = 0x416c5f; continue;
 }
 0x416c5f => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x416cc1; continue; }
  pc = 0x416c68; continue;
 }
 0x416c68 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3fu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x56d17au32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x416c7a); f_00416704(cpu); if cpu.returned_to != 0x416c7a { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d17au32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5758dfu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6c10u32, v); }
  { let v = cpu.r32(0x4d6a08u32); cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6a08u32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x5c4e3cu32, v); }
  cpu.push(0x416cbb); f_00447c27(cpu); if cpu.returned_to != 0x416cbb { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6a08u32, v); }
  pc = 0x416cc1; continue;
 }
 0x416cc1 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416cc3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416cc3); }
 let mut pc: u32 = 0x416cc3;
 loop { match pc {
 0x416cc3 => {
  { cpu.sub8(cpu.r8(0x58c88au32), 0x0u8); }
  if cpu.zf { pc = 0x416ccd; continue; }
  pc = 0x416ccc; continue;
 }
 0x416ccc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x416ccd => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x56c6fau32), 0x0u32); }
  if cpu.zf { pc = 0x416ce0; continue; }
  pc = 0x416cd7; continue;
 }
 0x416cd7 => {
  { cpu.sub32(cpu.r32(0x56c6eeu32), 0x0u32); }
  if !cpu.zf { pc = 0x416d47; continue; }
  pc = 0x416ce0; continue;
 }
 0x416ce0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x7fu32; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x58c88bu32), 0x0u8); }
  if cpu.zf { pc = 0x416cf4; continue; }
  pc = 0x416cef; continue;
 }
 0x416cef => {
  { let v: u32 = 0xfu32; cpu.ebp = v; }
  pc = 0x416cf4; continue;
 }
 0x416cf4 => {
  { let v: u16 = cpu.r16(0x56d17cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x416d00); f_00416704(cpu); if cpu.returned_to != 0x416d00 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x56d17cu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5758ebu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6c10u32, v); }
  { let v = cpu.r32(0x4d6a08u32); cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6a08u32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x5c4e3cu32, v); }
  cpu.push(0x416d41); f_00447c27(cpu); if cpu.returned_to != 0x416d41 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6a08u32, v); }
  pc = 0x416d47; continue;
 }
 0x416d47 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416d49(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x416d49); } r_00416d49(cpu, 0x416d49); }
pub fn f_00416d61(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x416d61); } r_00416d49(cpu, 0x416d61); }
fn r_00416d49(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x416d49 => {
  { cpu.sub8(cpu.r8(0x5922f9u32), 0x0u8); }
  if cpu.zf { pc = 0x416d8c; continue; }
  pc = 0x416d52; continue;
 }
 0x416d52 => {
  { cpu.sub8(cpu.r8(0x5921acu32), 0x0u8); }
  if cpu.zf { pc = 0x416d8c; continue; }
  pc = 0x416d5b; continue;
 }
 0x416d5b => {
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0xau32; cpu.ecx = v; }
  cpu.push(0x416d6a); f_0041c845(cpu); if cpu.returned_to != 0x416d6a { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5au32); cpu.eax = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x416d77); f_00401130(cpu); if cpu.returned_to != 0x416d77 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x592418u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x592418u32); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5921acu32, v); }
  pc = 0x416d8c; continue;
 }
 0x416d61 => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x416d6a); f_0041c845(cpu); if cpu.returned_to != 0x416d6a { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5au32); cpu.eax = v; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x416d77); f_00401130(cpu); if cpu.returned_to != 0x416d77 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x592418u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x592418u32); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5921acu32, v); }
  pc = 0x416d8c; continue;
 }
 0x416d8c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416d8d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416d8d); }
 let mut pc: u32 = 0x416d8d;
 loop { match pc {
 0x416d8d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416da6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416da6); }
 let mut pc: u32 = 0x416da6;
 loop { match pc {
 0x416da6 => {
  { let v: u32 = 0x31u32; cpu.w32(0x58c6fdu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416db1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416db1); }
 let mut pc: u32 = 0x416db1;
 loop { match pc {
 0x416db1 => {
  { let v: u32 = 0x78u32; cpu.eax = v; }
  { let v: u32 = 0x84u32; cpu.ebx = v; }
  cpu.push(0x416dc0); f_00416e5b(cpu); if cpu.returned_to != 0x416dc0 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416dc1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416dc1); }
 let mut pc: u32 = 0x416dc1;
 loop { match pc {
 0x416dc1 => {
  { let v: u32 = 0x78u32; cpu.eax = v; }
  { let v: u32 = 0x84u32; cpu.ebx = v; }
  cpu.push(0x416dd0); f_00416e5d(cpu); if cpu.returned_to != 0x416dd0 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416e5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416e5b); }
 let mut pc: u32 = 0x416e5b;
 loop { match pc {
 0x416e5b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416e5d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416e5d); }
 let mut pc: u32 = 0x416e5d;
 loop { match pc {
 0x416e5d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416e6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416e6f); }
 let mut pc: u32 = 0x416e6f;
 loop { match pc {
 0x416e6f => {
  { let v: u8 = cpu.add8(cpu.r8(0x4u32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); cpu.w8(0x4u32.wrapping_add(cpu.ecx), v); }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416ea4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416ea4); }
 let mut pc: u32 = 0x416ea4;
 loop { match pc {
 0x416ea4 => {
  { cpu.pushad(); }
  cpu.push(0x416eaa); f_00405360(cpu); if cpu.returned_to != 0x416eaa { return; }
  { cpu.popad(); }
  cpu.push(0x416eb0); f_00416eb9(cpu); if cpu.returned_to != 0x416eb0 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416eb1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416eb1); }
 let mut pc: u32 = 0x416eb1;
 loop { match pc {
 0x416eb1 => {
  { cpu.pushad(); }
  cpu.push(0x416eb7); f_00416eb9(cpu); if cpu.returned_to != 0x416eb7 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416eb9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416eb9); }
 let mut pc: u32 = 0x416eb9;
 loop { match pc {
 0x416eb9 => {
  { cpu.sub8(cpu.r8(0x4d952cu32), 0x0u8); }
  if !cpu.zf { pc = 0x416ec7; continue; }
  pc = 0x416ec2; continue;
 }
 0x416ec2 => {
  cpu.push(0x416ec7); f_0046c6ae(cpu); if cpu.returned_to != 0x416ec7 { return; }
  pc = 0x416ec7; continue;
 }
 0x416ec7 => {
  { let v: u32 = 0x10d0u32; cpu.ebx = v; }
  cpu.push(0x416ed1); f_0041b922(cpu); if cpu.returned_to != 0x416ed1 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f6au32, v); }
  { let v: u32 = cpu.xor32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x4d952cu32; cpu.edx = v; }
  cpu.push(0x416ee4); f_00416f00(cpu); if cpu.returned_to != 0x416ee4 { return; }
  { let v: u32 = 0xf0u32; cpu.edi = v; }
  { let v: u32 = 0x4d953au32; cpu.edx = v; }
  cpu.push(0x416ef3); f_00416f00(cpu); if cpu.returned_to != 0x416ef3 { return; }
  cpu.push(0x416ef8); f_0041b99b(cpu); if cpu.returned_to != 0x416ef8 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x581a15u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416f00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x416f00); }
 let mut pc: u32 = 0x416f00;
 loop { match pc {
 0x416f00 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac8u32); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x416f12); f_00447961(cpu); if cpu.returned_to != 0x416f12 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4d6ac8u32); cpu.esi = v; }
  { let v: u32 = 0x55b6b8u32; cpu.edi = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc8e4u32, v); }
  cpu.push(0x416f2a); f_0046c4e8(cpu); if cpu.returned_to != 0x416f2a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc8e4u32, v); }
  if !cpu.cf { pc = 0x416f38; continue; }
  pc = 0x416f33; continue;
 }
 0x416f33 => {
  f_0041c653(cpu); return;
 }
 0x416f38 => {
  { let v: u32 = 0x55b6b8u32; cpu.esi = v; }
  { let v: u32 = 0x5884d3u32; cpu.edi = v; }
  { let v: u32 = 0x300u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.pushad(); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.r32(0x4d681cu32); cpu.push(v); }
  cpu.push(0x416f57); f_00405480(cpu); if cpu.returned_to != 0x416f57 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416f5d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x416f5d); } r_00416f5d(cpu, 0x416f5d); }
pub fn f_00416f66(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x416f66); } r_00416f5d(cpu, 0x416f66); }
fn r_00416f5d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x416f5d => {
  { cpu.sub8(cpu.r8(0x56d22au32), 0x0u8); }
  if !cpu.zf { pc = 0x416f87; continue; }
  pc = 0x416f66; continue;
 }
 0x416f66 => {
  { cpu.pushad(); }
  cpu.push(0x416f6c); f_00416f88(cpu); if cpu.returned_to != 0x416f6c { return; }
  { cpu.sub8(cpu.r8(0x4e15b1u32), 0x0u8); }
  if cpu.zf { pc = 0x416f81; continue; }
  pc = 0x416f75; continue;
 }
 0x416f75 => {
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x416f7e); f_00404e80(cpu); if cpu.returned_to != 0x416f7e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x416f81; continue;
 }
 0x416f81 => {
  { cpu.popad(); }
  cpu.push(0x416f87); f_004209ba(cpu); if cpu.returned_to != 0x416f87 { return; }
  pc = 0x416f87; continue;
 }
 0x416f87 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00416f88(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x416f88); } r_00416f88(cpu, 0x416f88); }
pub fn f_00416fe8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x416fe8); } r_00416f88(cpu, 0x416fe8); }
fn r_00416f88(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x416f88 => {
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x416fab; continue; }
  pc = 0x416f9e; continue;
 }
 0x416f9e => {
  { let v: u16 = cpu.r16(0x4d67c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67cau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x416fab; continue;
 }
 0x416fab => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d67d6u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d67d8u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x416fc0; continue; }
  pc = 0x416fbe; continue;
 }
 0x416fbe => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x416fc0; continue;
 }
 0x416fc0 => {
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x416fc7; continue; }
  pc = 0x416fc5; continue;
 }
 0x416fc5 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x416fc7; continue;
 }
 0x416fc7 => {
  cpu.push(0x416fcc); f_004209a1(cpu); if cpu.returned_to != 0x416fcc { return; }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e1584u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x4d7118u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x416fdf); cpu.call(t); if cpu.returned_to != 0x416fdf { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x5820fbu32), 0x0u8); }
  if cpu.zf { pc = 0x416ff6; continue; }
  pc = 0x416fe9; continue;
 }
 0x416fe8 => {
  { let v: u32 = cpu.eax | 0xc4403d80u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x417139; continue; }
  pc = 0x416ff6; continue;
 }
 0x416fe9 => {
  { cpu.sub8(cpu.r8(0x58c440u32), 0x0u8); }
  if !cpu.zf { pc = 0x417139; continue; }
  pc = 0x416ff6; continue;
 }
 0x416ff6 => {
  { let v: u16 = cpu.r16(0x4d6856u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x41700c; continue; }
  pc = 0x417003; continue;
 }
 0x417003 => {
  { let v: u16 = 0xae2u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x417132; continue;
 }
 0x41700c => {
  { cpu.sub16((cpu.ecx as u16), 0xae3u16); }
  if !cpu.zf { pc = 0x41701c; continue; }
  pc = 0x417013; continue;
 }
 0x417013 => {
  { let v: u16 = 0xae3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x417132; continue;
 }
 0x41701c => {
  { cpu.sub16((cpu.ecx as u16), 0x4u16); }
  if !cpu.zf { pc = 0x417037; continue; }
  pc = 0x417022; continue;
 }
 0x417022 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xae6u32); cpu.ecx = v; }
  pc = 0x417132; continue;
 }
 0x417037 => {
  { cpu.sub16((cpu.ecx as u16), 0x2du16); }
  if !cpu.zf { pc = 0x417052; continue; }
  pc = 0x41703d; continue;
 }
 0x41703d => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xacau32); cpu.ecx = v; }
  pc = 0x417132; continue;
 }
 0x417052 => {
  { cpu.sub16((cpu.ecx as u16), 0x3u16); }
  if !cpu.zf { pc = 0x41706d; continue; }
  pc = 0x417058; continue;
 }
 0x417058 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xac2u32); cpu.ecx = v; }
  pc = 0x417132; continue;
 }
 0x41706d => {
  { cpu.sub16((cpu.ecx as u16), 0x2bu16); }
  if !cpu.zf { pc = 0x417088; continue; }
  pc = 0x417073; continue;
 }
 0x417073 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xad2u32); cpu.ecx = v; }
  pc = 0x417132; continue;
 }
 0x417088 => {
  { cpu.sub16((cpu.ecx as u16), 0x2cu16); }
  if !cpu.zf { pc = 0x4170a3; continue; }
  pc = 0x41708e; continue;
 }
 0x41708e => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xadau32); cpu.ecx = v; }
  pc = 0x417132; continue;
 }
 0x4170a3 => {
  { cpu.sub16((cpu.ecx as u16), 0xau16); }
  if !cpu.zf { pc = 0x4170bb; continue; }
  pc = 0x4170a9; continue;
 }
 0x4170a9 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xb20u32); cpu.ecx = v; }
  pc = 0x417132; continue;
 }
 0x4170bb => {
  { cpu.sub16((cpu.ecx as u16), 0xaefu16); }
  if !cpu.zf { pc = 0x4170d4; continue; }
  pc = 0x4170c2; continue;
 }
 0x4170c2 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xaefu32); cpu.ecx = v; }
  pc = 0x417132; continue;
 }
 0x4170d4 => {
  { cpu.sub16((cpu.ecx as u16), 0xaf3u16); }
  if !cpu.zf { pc = 0x4170ed; continue; }
  pc = 0x4170db; continue;
 }
 0x4170db => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xaf3u32); cpu.ecx = v; }
  pc = 0x417132; continue;
 }
 0x4170ed => {
  { cpu.sub16((cpu.ecx as u16), 0xaf7u16); }
  if !cpu.zf { pc = 0x417106; continue; }
  pc = 0x4170f4; continue;
 }
 0x4170f4 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xaf7u32); cpu.ecx = v; }
  pc = 0x417132; continue;
 }
 0x417106 => {
  { cpu.sub16((cpu.ecx as u16), 0x6u16); }
  if !cpu.zf { pc = 0x41711e; continue; }
  pc = 0x41710c; continue;
 }
 0x41710c => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0xb18u32); cpu.ecx = v; }
  pc = 0x417132; continue;
 }
 0x41711e => {
  { let v: u16 = cpu.r16(0x4d6856u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x824u16); }
  if !cpu.zf { pc = 0x417132; continue; }
  pc = 0x41712c; continue;
 }
 0x41712c => {
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x417132; continue;
 }
 0x417132 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x581a32u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x417139 => {
  { let v: u16 = cpu.r16(0x4d6856u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x41714f; continue; }
  pc = 0x417146; continue;
 }
 0x417146 => {
  { let v: u16 = 0x1722u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x41723e; continue;
 }
 0x41714f => {
  { cpu.sub16((cpu.ecx as u16), 0xae3u16); }
  if !cpu.zf { pc = 0x41715f; continue; }
  pc = 0x417156; continue;
 }
 0x417156 => {
  { let v: u16 = 0x1723u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x41723e; continue;
 }
 0x41715f => {
  { cpu.sub16((cpu.ecx as u16), 0x4u16); }
  if !cpu.zf { pc = 0x41717a; continue; }
  pc = 0x417165; continue;
 }
 0x417165 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x170au32); cpu.ecx = v; }
  pc = 0x41723e; continue;
 }
 0x41717a => {
  { cpu.sub16((cpu.ecx as u16), 0x2du16); }
  if !cpu.zf { pc = 0x417195; continue; }
  pc = 0x417180; continue;
 }
 0x417180 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x172au32); cpu.ecx = v; }
  pc = 0x41723e; continue;
 }
 0x417195 => {
  { cpu.sub16((cpu.ecx as u16), 0x3u16); }
  if !cpu.zf { pc = 0x4171b0; continue; }
  pc = 0x41719b; continue;
 }
 0x41719b => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x1712u32); cpu.ecx = v; }
  pc = 0x41723e; continue;
 }
 0x4171b0 => {
  { cpu.sub16((cpu.ecx as u16), 0x2bu16); }
  if !cpu.zf { pc = 0x4171c8; continue; }
  pc = 0x4171b6; continue;
 }
 0x4171b6 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x171au32); cpu.ecx = v; }
  pc = 0x41723e; continue;
 }
 0x4171c8 => {
  { cpu.sub16((cpu.ecx as u16), 0xau16); }
  if !cpu.zf { pc = 0x4171e0; continue; }
  pc = 0x4171ce; continue;
 }
 0x4171ce => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x1702u32); cpu.ecx = v; }
  pc = 0x41723e; continue;
 }
 0x4171e0 => {
  { cpu.sub16((cpu.ecx as u16), 0xaefu16); }
  if !cpu.zf { pc = 0x4171f9; continue; }
  pc = 0x4171e7; continue;
 }
 0x4171e7 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x1726u32); cpu.ecx = v; }
  pc = 0x41723e; continue;
 }
 0x4171f9 => {
  { cpu.sub16((cpu.ecx as u16), 0xaf7u16); }
  if !cpu.zf { pc = 0x417212; continue; }
  pc = 0x417200; continue;
 }
 0x417200 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x1732u32); cpu.ecx = v; }
  pc = 0x41723e; continue;
 }
 0x417212 => {
  { cpu.sub16((cpu.ecx as u16), 0x6u16); }
  if !cpu.zf { pc = 0x41722a; continue; }
  pc = 0x417218; continue;
 }
 0x417218 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x581a31u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x16fau32); cpu.ecx = v; }
  pc = 0x41723e; continue;
 }
 0x41722a => {
  { let v: u16 = cpu.r16(0x4d6856u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x824u16); }
  if !cpu.zf { pc = 0x41723e; continue; }
  pc = 0x417238; continue;
 }
 0x417238 => {
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x41723e; continue;
 }
 0x41723e => {
  { let v: u32 = cpu.ecx; cpu.w32(0x581a32u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417245(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x417245); } r_00417245(cpu, 0x417245); }
pub fn f_0041726f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41726f); } r_00417245(cpu, 0x41726f); }
fn r_00417245(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x417245 => {
  { cpu.sub8(cpu.r8(0x56d22au32), 0x0u8); }
  if !cpu.zf { pc = 0x4172da; continue; }
  pc = 0x417252; continue;
 }
 0x417252 => {
  { cpu.sub8(cpu.r8(0x4e15b1u32), 0x0u8); }
  if cpu.zf { pc = 0x4172da; continue; }
  pc = 0x41725b; continue;
 }
 0x41725b => {
  { let v: u8 = cpu.inc8(cpu.r8(0x581a31u32)); cpu.w8(0x581a31u32, v); }
  { let v: u8 = cpu.r8(0x581a31u32) & 0x7u8; cpu.w8(0x581a31u32, v); }
  { cpu.pushad(); }
  cpu.push(0x41726e); f_00416f88(cpu); if cpu.returned_to != 0x41726e { return; }
  { let v = cpu.r16(0x4d6ba4u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d6ba8u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { cpu.sub8(cpu.r8(0x4e1e4bu32), 0x0u8); }
  if cpu.zf { pc = 0x4172c6; continue; }
  pc = 0x417285; continue;
 }
 0x41726f => {
  { let v = cpu.r32(0x4d6ba4u32); cpu.push(v); }
  { let v = cpu.r16(0x4d6ba8u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { cpu.sub8(cpu.r8(0x4e1e4bu32), 0x0u8); }
  if cpu.zf { pc = 0x4172c6; continue; }
  pc = 0x417285; continue;
 }
 0x417285 => {
  { let v: u16 = 0x280u16; cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x4d6ba8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ea268u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5ea26cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1584u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5e2474u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4172b9); f_00404e80(cpu); if cpu.returned_to != 0x4172b9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x5ddbc0u32, v); }
  pc = 0x4172c6; continue;
 }
 0x4172c6 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6ba8u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6ba4u32, v); }
  { cpu.popad(); }
  cpu.push(0x4172da); f_004209ba(cpu); if cpu.returned_to != 0x4172da { return; }
  pc = 0x4172da; continue;
 }
 0x4172da => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004172db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4172db); }
 let mut pc: u32 = 0x4172db;
 loop { match pc {
 0x4172db => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004172dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4172dc); }
 let mut pc: u32 = 0x4172dc;
 loop { match pc {
 0x4172dc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004172dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4172dd); }
 let mut pc: u32 = 0x4172dd;
 loop { match pc {
 0x4172dd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004172de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4172de); }
 let mut pc: u32 = 0x4172de;
 loop { match pc {
 0x4172de => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004172df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4172df); }
 let mut pc: u32 = 0x4172df;
 loop { match pc {
 0x4172df => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004172e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4172e0); }
 let mut pc: u32 = 0x4172e0;
 loop { match pc {
 0x4172e0 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x4172e6; continue;
 }
 0x4172e6 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { cpu.poll(); pc = 0x4172e6; continue; }
  pc = 0x4172eb; continue;
 }
 0x4172eb => {
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x4172ef; continue;
 }
 0x4172ef => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x4172ef; continue; }
  pc = 0x4172f4; continue;
 }
 0x4172f4 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004172f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4172f7); }
 let mut pc: u32 = 0x4172f7;
 loop { match pc {
 0x4172f7 => {
  { cpu.sub8(cpu.r8(0x581a15u32), 0x0u8); }
  if !cpu.zf { pc = 0x41734d; continue; }
  pc = 0x417300; continue;
 }
 0x417300 => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41730f; continue; }
  pc = 0x417309; continue;
 }
 0x417309 => {
  cpu.push(0x41730e); f_00417b6f(cpu); if cpu.returned_to != 0x41730e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41730f => {
  cpu.push(0x417314); f_00447791(cpu); if cpu.returned_to != 0x417314 { return; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  cpu.push(0x41731d); f_00447781(cpu); if cpu.returned_to != 0x41731d { return; }
  { let v: u32 = 0x4d6aa8u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  cpu.push(0x41732c); f_0041c901(cpu); if cpu.returned_to != 0x41732c { return; }
  { let v: u32 = cpu.r16(0x4d6aa8u32) as u32; cpu.edx = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  cpu.push(0x41733e); f_00447781(cpu); if cpu.returned_to != 0x41733e { return; }
  { let v: u32 = 0x300u32; cpu.ecx = v; }
  { let v: u32 = 0x55b6b8u32; cpu.edx = v; }
  cpu.push(0x41734d); f_0041c901(cpu); if cpu.returned_to != 0x41734d { return; }
  pc = 0x41734d; continue;
 }
 0x41734d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041734e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41734e); }
 let mut pc: u32 = 0x41734e;
 loop { match pc {
 0x41734e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417352(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417352); }
 let mut pc: u32 = 0x417352;
 loop { match pc {
 0x417352 => {
  { let v: u32 = cpu.r32(0x4d6814u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4d681cu32); cpu.edi = v; }
  let t = cpu.r32(0x4d6f80u32); cpu.push(0x417364); cpu.call(t); if cpu.returned_to != 0x417364 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417365(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x417365); } r_00417365(cpu, 0x417365); }
pub fn f_00417367(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x417367); } r_00417365(cpu, 0x417367); }
pub fn f_00417370(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x417370); } r_00417365(cpu, 0x417370); }
fn r_00417365(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x417365 => {
  { cpu.pushad(); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x417367; continue;
 }
 0x417367 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x41736d); f_00404b90(cpu); if cpu.returned_to != 0x41736d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x417370; continue;
 }
 0x417370 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417372(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417372); }
 let mut pc: u32 = 0x417372;
 loop { match pc {
 0x417372 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417373(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417373); }
 let mut pc: u32 = 0x417373;
 loop { match pc {
 0x417373 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417374(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417374); }
 let mut pc: u32 = 0x417374;
 loop { match pc {
 0x417374 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f9cu32); cpu.push(0x41737b); cpu.call(t); if cpu.returned_to != 0x41737b { return; }
  cpu.push(0x417380); f_004052e0(cpu); if cpu.returned_to != 0x417380 { return; }
  { cpu.popad(); }
  pc = 0x417381; continue;
 }
 0x417381 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x417387; continue; }
  pc = 0x417385; continue;
 }
 0x417385 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x417387; continue;
 }
 0x417387 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e15a7u32)); }
  if !cpu.cf { cpu.poll(); pc = 0x417381; continue; }
  pc = 0x41738f; continue;
 }
 0x41738f => {
  { let v: u8 = cpu.r8(0x4e15a7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e15aau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15a7u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004173a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4173a1); }
 let mut pc: u32 = 0x4173a1;
 loop { match pc {
 0x4173a1 => {
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f9cu32); cpu.push(0x4173a8); cpu.call(t); if cpu.returned_to != 0x4173a8 { return; }
  cpu.push(0x4173ad); f_004052e0(cpu); if cpu.returned_to != 0x4173ad { return; }
  { cpu.popad(); }
  pc = 0x4173ae; continue;
 }
 0x4173ae => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x4173b4; continue; }
  pc = 0x4173b2; continue;
 }
 0x4173b2 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4173b4; continue;
 }
 0x4173b4 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e15a7u32)); }
  if !cpu.cf { cpu.poll(); pc = 0x4173ae; continue; }
  pc = 0x4173bc; continue;
 }
 0x4173bc => {
  { let v: u8 = cpu.r8(0x4e15a7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e15aau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15a7u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004173ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4173ce); }
 let mut pc: u32 = 0x4173ce;
 loop { match pc {
 0x4173ce => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004173cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4173cf); }
 let mut pc: u32 = 0x4173cf;
 loop { match pc {
 0x4173cf => {
  { let v: u32 = 0x1e0u32; cpu.w32(0x4d6bccu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004173da(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4173da); } r_004173da(cpu, 0x4173da); }
pub fn f_0041746e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41746e); } r_004173da(cpu, 0x41746e); }
fn r_004173da(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4173da => {
  { let v: u8 = 0x1u8; cpu.w8(0x5820fau32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x4e1fa1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x59c54cu32, v); }
  cpu.push(0x4173f4); f_00429846(cpu); if cpu.returned_to != 0x4173f4 { return; }
  { let v: u16 = cpu.r16(0x4d6ba4u32).wrapping_sub(0x60u16); cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = cpu.r16(0x4d6ba8u32).wrapping_sub(0x20u16); cpu.w16(0x4d6ba8u32, v); }
  { let v: u32 = cpu.r32(0x59c559u32).wrapping_sub(0x20u32); cpu.w32(0x59c559u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c18u32, v); }
  { let v: u32 = 0x1du32; cpu.w32(0x4d6c1cu32, v); }
  { let v: u32 = cpu.r32(0x59c555u32).wrapping_sub(0x60u32); cpu.w32(0x59c555u32, v); }
  { let v: u32 = cpu.r32(0x59c55du32).wrapping_sub(0x6u32); cpu.w32(0x59c55du32, v); }
  cpu.push(0x417432); f_0041c1a0(cpu); if cpu.returned_to != 0x417432 { return; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x59c559u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xfu32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x41744c); f_0041c9df(cpu); if cpu.returned_to != 0x41744c { return; }
  cpu.push(0x417451); f_0041795c(cpu); if cpu.returned_to != 0x417451 { return; }
  { let v: u32 = 0x68u32; cpu.w32(0x5928b9u32, v); }
  { let v: u32 = 0x124u32; cpu.w32(0x5928bdu32, v); }
  { let v: u32 = 0x56u32; cpu.w32(0x5928c1u32, v); }
  { let v: u32 = 0x224u32; cpu.w32(0x5928c5u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41746e => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x5928c5u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417478(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417478); }
 let mut pc: u32 = 0x417478;
 loop { match pc {
 0x417478 => {
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  f_0041747a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041747a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41747a); }
 let mut pc: u32 = 0x41747a;
 loop { match pc {
 0x41747a => {
  { cpu.sub8(cpu.r8(0x575849u32), 0x0u8); }
  if !cpu.zf { pc = 0x41748e; continue; }
  pc = 0x417483; continue;
 }
 0x417483 => {
  { cpu.sub8(cpu.r8(0x56d229u32), 0x0u8); }
  if cpu.sf { pc = 0x41748e; continue; }
  pc = 0x41748c; continue;
 }
 0x41748c => {
  if !cpu.zf { pc = 0x4174c9; continue; }
  pc = 0x41748e; continue;
 }
 0x41748e => {
  { let v: u8 = 0x1u8; cpu.w8(0x56d42fu32, v); }
  { let v: u32 = 0x5431b4u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x5431b2u32) as u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x4174c9; continue; }
  pc = 0x4174a5; continue;
 }
 0x4174a5 => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x4174aa; continue;
 }
 0x4174aa => {
  { cpu.lods(4, false); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4174bf; continue; }
  pc = 0x4174b0; continue;
 }
 0x4174b0 => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x4d68f2u32, v); }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  let t = cpu.r32(0x4d6facu32); cpu.push(0x4174be); cpu.call(t); if cpu.returned_to != 0x4174be { return; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x4174bf; continue;
 }
 0x4174bf => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4174aa; continue; }
  pc = 0x4174c2; continue;
 }
 0x4174c2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1fa3u32, v); }
  pc = 0x4174c9; continue;
 }
 0x4174c9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041750f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41750f); }
 let mut pc: u32 = 0x41750f;
 loop { match pc {
 0x41750f => {
  { cpu.sub8(cpu.r8(0x575849u32), 0x0u8); }
  if !cpu.zf { pc = 0x417523; continue; }
  pc = 0x417518; continue;
 }
 0x417518 => {
  { cpu.sub8(cpu.r8(0x56d229u32), 0x0u8); }
  if cpu.sf { pc = 0x417523; continue; }
  pc = 0x417521; continue;
 }
 0x417521 => {
  if !cpu.zf { pc = 0x417565; continue; }
  pc = 0x417523; continue;
 }
 0x417523 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56d42fu32, v); }
  { let v: u32 = 0x5431b4u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x5431b2u32) as u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x417565; continue; }
  pc = 0x41753a; continue;
 }
 0x41753a => {
  { let v: u32 = cpu.r32(0x4d6afcu32); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  pc = 0x417545; continue;
 }
 0x417545 => {
  { cpu.lods(4, false); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x41755a; continue; }
  pc = 0x41754b; continue;
 }
 0x41754b => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d68f2u32, v); }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  let t = cpu.r32(0x4d6fc4u32); cpu.push(0x417559); cpu.call(t); if cpu.returned_to != 0x417559 { return; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x41755a; continue;
 }
 0x41755a => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x417545; continue; }
  pc = 0x41755e; continue;
 }
 0x41755e => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fa3u32, v); }
  pc = 0x417565; continue;
 }
 0x417565 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041756e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41756e); }
 let mut pc: u32 = 0x41756e;
 loop { match pc {
 0x41756e => {
  { let v = cpu.esp; cpu.push(v); }
  { let v: u8 = cpu.r8(0xbe3474c9u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u8)); cpu.w8(0xbe3474c9u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x31u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4d6afc3du32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x4d6afc3du32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x417584; continue;
 }
 0x417584 => {
  { cpu.lods(4, false); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x41759d; continue; }
  pc = 0x417589; continue;
 }
 0x417589 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x41759d; continue; }
  pc = 0x41758e; continue;
 }
 0x41758e => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d68f2u32, v); }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  let t = cpu.r32(0x4d6fc4u32); cpu.push(0x41759c); cpu.call(t); if cpu.returned_to != 0x41759c { return; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x41759d; continue;
 }
 0x41759d => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x417584; continue; }
  pc = 0x4175a1; continue;
 }
 0x4175a1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fa3u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004175a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4175a9); }
 let mut pc: u32 = 0x4175a9;
 loop { match pc {
 0x4175a9 => {
  { let v: u32 = 0x5431b4u32; cpu.edi = v; }
  { let v: u32 = cpu.r16(0x5431b2u32) as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.zf { pc = 0x4175d8; continue; }
  pc = 0x4175ba; continue;
 }
 0x4175ba => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  pc = 0x4175c4; continue;
 }
 0x4175c4 => {
  { cpu.stos(4, false); }
  { cpu.sub8(cpu.r8(0x58e95cu32.wrapping_add(cpu.edx)), 0xffu8); }
  if cpu.zf { pc = 0x4175d5; continue; }
  pc = 0x4175ce; continue;
 }
 0x4175ce => {
  { let v: u8 = 0x0u8; cpu.w8(0x58e95cu32.wrapping_add(cpu.edx), v); }
  pc = 0x4175d5; continue;
 }
 0x4175d5 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4175c4; continue; }
  pc = 0x4175d8; continue;
 }
 0x4175d8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004175d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4175d9); }
 let mut pc: u32 = 0x4175d9;
 loop { match pc {
 0x4175d9 => {
  { cpu.pushad(); }
  cpu.push(0x4175df); f_00405330(cpu); if cpu.returned_to != 0x4175df { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x575f2eu32), 0x0u8); }
  if !cpu.zf { pc = 0x4175ea; continue; }
  pc = 0x4175e9; continue;
 }
 0x4175e9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4175ea => {
  { let v = cpu.r32(0x581b4du32); cpu.push(v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x4176ae; continue; }
  pc = 0x4175fd; continue;
 }
 0x4175fd => {
  { let v: u32 = 0x1u32; cpu.w32(0x581b4du32, v); }
  { cpu.sub8(cpu.r8(0x4d6932u32), 0x0u8); }
  if !cpu.zf { pc = 0x41761a; continue; }
  pc = 0x417610; continue;
 }
 0x417610 => {
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x41761a); f_00417dab(cpu); if cpu.returned_to != 0x41761a { return; }
  pc = 0x41761a; continue;
 }
 0x41761a => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x417627); f_00402d70(cpu); if cpu.returned_to != 0x417627 { return; }
  cpu.push(0x41762c); f_00405320(cpu); if cpu.returned_to != 0x41762c { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fa1u32, v); }
  { let v: u16 = 0x110u16; cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = 0xb8u16; cpu.w16(0x4d6ba8u32, v); }
  cpu.push(0x41764b); f_0042f13b(cpu); if cpu.returned_to != 0x41764b { return; }
  let t = cpu.r32(0x4d6f88u32); cpu.push(0x417651); cpu.call(t); if cpu.returned_to != 0x417651 { return; }
  cpu.push(0x417656); f_00417874(cpu); if cpu.returned_to != 0x417656 { return; }
  cpu.push(0x41765b); f_0041c1a0(cpu); if cpu.returned_to != 0x41765b { return; }
  { cpu.sub8(cpu.r8(0x4d6932u32), 0x0u8); }
  if !cpu.zf { pc = 0x4176a7; continue; }
  pc = 0x417664; continue;
 }
 0x417664 => {
  let t = cpu.r32(0x4d6f70u32); cpu.push(0x41766a); cpu.call(t); if cpu.returned_to != 0x41766a { return; }
  cpu.push(0x41766f); f_0045dcfe(cpu); if cpu.returned_to != 0x41766f { return; }
  cpu.push(0x417674); f_0046535a(cpu); if cpu.returned_to != 0x417674 { return; }
  cpu.push(0x417679); f_00465383(cpu); if cpu.returned_to != 0x417679 { return; }
  { cpu.pushad(); }
  cpu.push(0x41767f); f_00465532(cpu); if cpu.returned_to != 0x41767f { return; }
  { cpu.popad(); }
  cpu.push(0x417685); f_004665ce(cpu); if cpu.returned_to != 0x417685 { return; }
  cpu.push(0x41768a); f_0041b2d5(cpu); if cpu.returned_to != 0x41768a { return; }
  cpu.push(0x41768f); f_004176b5(cpu); if cpu.returned_to != 0x41768f { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x581b4du32, v); }
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x4176a7); f_00417d55(cpu); if cpu.returned_to != 0x4176a7 { return; }
  pc = 0x4176a7; continue;
 }
 0x4176a7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4d6932u32, v); }
  pc = 0x4176ae; continue;
 }
 0x4176ae => {
  { let v: u32 = cpu.pop(); cpu.w32(0x581b4du32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004176b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4176b5); }
 let mut pc: u32 = 0x4176b5;
 loop { match pc {
 0x4176b5 => {
  { let v: u32 = cpu.r32(0x4d6a00u32).wrapping_add(0xa0u32); cpu.w32(0x4d6a00u32, v); }
  { let v: u32 = cpu.r32(0x4d6a00u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x140u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d678eu32)); }
  if cpu.cf { pc = 0x4176e2; continue; }
  pc = 0x4176d2; continue;
 }
 0x4176d2 => {
  { let v: u16 = cpu.r16(0x4d678eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x140u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a00u32, v); }
  pc = 0x4176e2; continue;
 }
 0x4176e2 => {
  { let v: u32 = cpu.r32(0x4d6a04u32).wrapping_add(0x64u32); cpu.w32(0x4d6a04u32, v); }
  { let v: u32 = cpu.r32(0x4d6a04u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xc8u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6790u32)); }
  if cpu.cf { pc = 0x41770c; continue; }
  pc = 0x4176fc; continue;
 }
 0x4176fc => {
  { let v: u16 = cpu.r16(0x4d6790u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, 0xc8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a04u32, v); }
  pc = 0x41770c; continue;
 }
 0x41770c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041770d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41770d); } r_0041770d(cpu, 0x41770d); }
pub fn f_004177e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4177e8); } r_0041770d(cpu, 0x4177e8); }
fn r_0041770d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x41770d => {
  { cpu.pushad(); }
  cpu.push(0x417713); f_00405360(cpu); if cpu.returned_to != 0x417713 { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x575f2eu32), 0x0u8); }
  if !cpu.zf { pc = 0x41771e; continue; }
  pc = 0x41771d; continue;
 }
 0x41771d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41771e => {
  { let v = cpu.r32(0x581b4du32); cpu.push(v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if !cpu.zf { pc = 0x4177fc; continue; }
  pc = 0x417731; continue;
 }
 0x417731 => {
  { let v: u32 = 0x1u32; cpu.w32(0x581b4du32, v); }
  { cpu.sub8(cpu.r8(0x4d6932u32), 0x0u8); }
  if !cpu.zf { pc = 0x41774e; continue; }
  pc = 0x417744; continue;
 }
 0x417744 => {
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x41774e); f_00417dab(cpu); if cpu.returned_to != 0x41774e { return; }
  pc = 0x41774e; continue;
 }
 0x41774e => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x41775b); f_00402d70(cpu); if cpu.returned_to != 0x41775b { return; }
  cpu.push(0x417760); f_00405320(cpu); if cpu.returned_to != 0x417760 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1fa1u32, v); }
  { let v: u16 = 0x250u16; cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = 0x180u16; cpu.w16(0x4d6ba8u32, v); }
  cpu.push(0x41777f); f_0042f152(cpu); if cpu.returned_to != 0x41777f { return; }
  let t = cpu.r32(0x4d6f88u32); cpu.push(0x417785); cpu.call(t); if cpu.returned_to != 0x417785 { return; }
  cpu.push(0x41778a); f_0041795c(cpu); if cpu.returned_to != 0x41778a { return; }
  cpu.push(0x41778f); f_0041c1a0(cpu); if cpu.returned_to != 0x41778f { return; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x4177a9); f_0041c9df(cpu); if cpu.returned_to != 0x4177a9 { return; }
  { cpu.sub8(cpu.r8(0x4d6932u32), 0x0u8); }
  if !cpu.zf { pc = 0x4177f5; continue; }
  pc = 0x4177b2; continue;
 }
 0x4177b2 => {
  let t = cpu.r32(0x4d6f70u32); cpu.push(0x4177b8); cpu.call(t); if cpu.returned_to != 0x4177b8 { return; }
  cpu.push(0x4177bd); f_0045dcfe(cpu); if cpu.returned_to != 0x4177bd { return; }
  cpu.push(0x4177c2); f_0046535a(cpu); if cpu.returned_to != 0x4177c2 { return; }
  cpu.push(0x4177c7); f_00465383(cpu); if cpu.returned_to != 0x4177c7 { return; }
  { cpu.pushad(); }
  cpu.push(0x4177cd); f_00465532(cpu); if cpu.returned_to != 0x4177cd { return; }
  { cpu.popad(); }
  cpu.push(0x4177d3); f_004665ce(cpu); if cpu.returned_to != 0x4177d3 { return; }
  cpu.push(0x4177d8); f_0041b2d5(cpu); if cpu.returned_to != 0x4177d8 { return; }
  cpu.push(0x4177dd); f_00417803(cpu); if cpu.returned_to != 0x4177dd { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x581b4du32, v); }
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x4177f5); f_00417d55(cpu); if cpu.returned_to != 0x4177f5 { return; }
  pc = 0x4177f5; continue;
 }
 0x4177e8 => {
  { let v: u32 = 0x660055b9u32; cpu.eax = v; }
  { let v: u32 = 0x60e80100u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5c60000u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ cpu.r8(0x4du32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4177fc; continue;
 }
 0x4177f5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4d6932u32, v); }
  pc = 0x4177fc; continue;
 }
 0x4177fc => {
  { let v: u32 = cpu.pop(); cpu.w32(0x581b4du32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417803(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x417803); } r_00417803(cpu, 0x417803); }
pub fn f_00417845(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x417845); } r_00417803(cpu, 0x417845); }
pub fn f_00417865(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x417865); } r_00417803(cpu, 0x417865); }
fn r_00417803(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x417803 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x4d6a00u32), 0xa0u32); cpu.w32(0x4d6a00u32, v); }
  if !cpu.sf { pc = 0x417819; continue; }
  pc = 0x41780f; continue;
 }
 0x41780f => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6a00u32, v); }
  pc = 0x417819; continue;
 }
 0x417819 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x4d6a04u32), 0x64u32); cpu.w32(0x4d6a04u32, v); }
  if !cpu.sf { pc = 0x41782c; continue; }
  pc = 0x417822; continue;
 }
 0x417822 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6a04u32, v); }
  pc = 0x41782c; continue;
 }
 0x41782c => {
  { let v: u32 = cpu.r32(0x4d6a00u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x280u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d678eu32)); }
  if cpu.cf { pc = 0x41784e; continue; }
  pc = 0x41783f; continue;
 }
 0x41783f => {
  { let v: u16 = cpu.r16(0x4d678eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x417845; continue;
 }
 0x417845 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x280u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a00u32, v); }
  pc = 0x41784e; continue;
 }
 0x41784e => {
  { let v: u32 = cpu.r32(0x4d6a04u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6bccu32)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6790u32)); }
  if cpu.cf { pc = 0x417873; continue; }
  pc = 0x417862; continue;
 }
 0x417862 => {
  { let v: u16 = cpu.r16(0x4d6790u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x4d6bccu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a04u32, v); }
  pc = 0x417873; continue;
 }
 0x417865 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x4d6bccu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a04u32, v); }
  pc = 0x417873; continue;
 }
 0x417873 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041786f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41786f); }
 let mut pc: u32 = 0x41786f;
 loop { match pc {
 0x41786f => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x6au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  f_00417874(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417874(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417874); }
 let mut pc: u32 = 0x417874;
 loop { match pc {
 0x417874 => {
  { cpu.sub8(cpu.r8(0x581a16u32), 0x0u8); }
  if cpu.zf { pc = 0x41795b; continue; }
  pc = 0x417881; continue;
 }
 0x417881 => {
  { let v: u8 = 0x0u8; cpu.w8(0x581a16u32, v); }
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  cpu.push(0x417897); f_00439dfc(cpu); if cpu.returned_to != 0x417897 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x190u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4acba6u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acba6u32, v); }
  { let v: u16 = cpu.r16(0x4acbaau32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acbaau32, v); }
  { let v: u16 = cpu.r16(0x4acbc2u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acbc2u32, v); }
  { let v: u16 = cpu.r16(0x4acbc6u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acbc6u32, v); }
  { let v: u16 = cpu.r16(0x4acbdeu32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acbdeu32, v); }
  { let v: u16 = cpu.r16(0x4acbe2u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acbe2u32, v); }
  { let v: u16 = cpu.r16(0x4acbfau32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acbfau32, v); }
  { let v: u16 = cpu.r16(0x4acbfeu32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acbfeu32, v); }
  { let v: u16 = cpu.r16(0x4acc16u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acc16u32, v); }
  { let v: u16 = cpu.r16(0x4acc1au32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acc1au32, v); }
  { let v: u16 = cpu.r16(0x4acc32u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acc32u32, v); }
  { let v: u16 = cpu.r16(0x4acc36u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acc36u32, v); }
  { let v: u16 = cpu.r16(0x4acc86u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acc86u32, v); }
  { let v: u16 = cpu.r16(0x4acc8au32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acc8au32, v); }
  { let v: u16 = cpu.r16(0x4acca6u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acca6u32, v); }
  { let v: u16 = cpu.r16(0x4accc2u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4accc2u32, v); }
  { let v: u16 = cpu.r16(0x4acb1eu32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acb1eu32, v); }
  { let v: u16 = cpu.r16(0x4accf6u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4accf6u32, v); }
  { let v: u16 = cpu.r16(0x4accfau32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4accfau32, v); }
  { let v: u16 = cpu.r16(0x4acd16u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acd16u32, v); }
  { let v: u16 = cpu.r16(0x4acd32u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acd32u32, v); }
  { let v: u16 = cpu.r16(0x4acb02u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acb02u32, v); }
  { let v: u16 = cpu.r16(0x4acd6au32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acd6au32, v); }
  { let v: u16 = cpu.r16(0x4acc4eu32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acc4eu32, v); }
  { let v: u16 = cpu.r16(0x4acc52u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4acc52u32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x4acae6u32), (cpu.eax as u16)); cpu.w16(0x4acae6u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x41795b; continue;
 }
 0x41795b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041795c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41795c); } r_0041795c(cpu, 0x41795c); }
pub fn f_00417972(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x417972); } r_0041795c(cpu, 0x417972); }
fn r_0041795c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x41795c => {
  { cpu.sub8(cpu.r8(0x581a16u32), 0x0u8); }
  if !cpu.zf { pc = 0x417b6e; continue; }
  pc = 0x417969; continue;
 }
 0x417969 => {
  { let v: u8 = 0x1u8; cpu.w8(0x581a16u32, v); }
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  cpu.push(0x41797f); f_00439dc7(cpu); if cpu.returned_to != 0x41797f { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x4d6bccu32), 0x1e0u32); }
  if !cpu.zf { pc = 0x417a82; continue; }
  pc = 0x417991; continue;
 }
 0x417972 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x64u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x64u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x41797f); f_00439dc7(cpu); if cpu.returned_to != 0x41797f { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x4d6bccu32), 0x1e0u32); }
  if !cpu.zf { pc = 0x417a82; continue; }
  pc = 0x417991; continue;
 }
 0x417991 => {
  { let v: u16 = 0x1c6u16; cpu.w16(0x4acba6u32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x4acbaau32, v); }
  { let v: u16 = 0x1c6u16; cpu.w16(0x4acbc2u32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x4acbc6u32, v); }
  { let v: u16 = 0x1c6u16; cpu.w16(0x4acbdeu32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x4acbe2u32, v); }
  { let v: u16 = 0x1c6u16; cpu.w16(0x4acbfau32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x4acbfeu32, v); }
  { let v: u16 = 0x1c6u16; cpu.w16(0x4acc16u32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x4acc1au32, v); }
  { let v: u16 = 0x1c6u16; cpu.w16(0x4acc32u32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x4acc36u32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x4acc86u32, v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x4acc8au32, v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x4acca6u32, v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x4accc2u32, v); }
  { let v: u16 = 0x1c0u16; cpu.w16(0x4acb1eu32, v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x4acd6au32, v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x4acb02u32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x4accf6u32, v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x4accfau32, v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x4acd16u32, v); }
  { let v: u16 = 0x1e0u16; cpu.w16(0x4acd32u32, v); }
  { let v: u16 = 0x1c6u16; cpu.w16(0x4acc4eu32, v); }
  { let v: u16 = 0x1deu16; cpu.w16(0x4acc52u32, v); }
  { let v: u16 = 0x1c0u16; cpu.w16(0x4acae6u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x417b6e; continue;
 }
 0x417a82 => {
  { let v: u16 = 0x176u16; cpu.w16(0x4acba6u32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x4acbaau32, v); }
  { let v: u16 = 0x176u16; cpu.w16(0x4acbc2u32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x4acbc6u32, v); }
  { let v: u16 = 0x176u16; cpu.w16(0x4acbdeu32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x4acbe2u32, v); }
  { let v: u16 = 0x176u16; cpu.w16(0x4acbfau32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x4acbfeu32, v); }
  { let v: u16 = 0x176u16; cpu.w16(0x4acc16u32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x4acc1au32, v); }
  { let v: u16 = 0x176u16; cpu.w16(0x4acc32u32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x4acc36u32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x4acc86u32, v); }
  { let v: u16 = 0x190u16; cpu.w16(0x4acc8au32, v); }
  { let v: u16 = 0x190u16; cpu.w16(0x4acca6u32, v); }
  { let v: u16 = 0x190u16; cpu.w16(0x4acd6au32, v); }
  { let v: u16 = 0x190u16; cpu.w16(0x4acb02u32, v); }
  { let v: u16 = 0x190u16; cpu.w16(0x4accc2u32, v); }
  { let v: u16 = 0x170u16; cpu.w16(0x4acb1eu32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x4accf6u32, v); }
  { let v: u16 = 0x190u16; cpu.w16(0x4accfau32, v); }
  { let v: u16 = 0x190u16; cpu.w16(0x4acd16u32, v); }
  { let v: u16 = 0x190u16; cpu.w16(0x4acd32u32, v); }
  { let v: u16 = 0x176u16; cpu.w16(0x4acc4eu32, v); }
  { let v: u16 = 0x18eu16; cpu.w16(0x4acc52u32, v); }
  { let v: u16 = 0x170u16; cpu.w16(0x4acae6u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x417b6e; continue;
 }
 0x417b6e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417b6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417b6f); }
 let mut pc: u32 = 0x417b6f;
 loop { match pc {
 0x417b6f => {
  { let v: u32 = 0x581a24u32; cpu.edx = v; }
  { let v: u32 = 0x55b6b8u32; cpu.ebx = v; }
  { let v: u32 = 0x300u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f6au32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x417b90); f_00447961(cpu); if cpu.returned_to != 0x417b90 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f6au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417b99(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417b99); }
 let mut pc: u32 = 0x417b99;
 loop { match pc {
 0x417b99 => {
  { cpu.pushad(); }
  cpu.push(0x417b9f); f_00417ba8(cpu); if cpu.returned_to != 0x417b9f { return; }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x417ba6); f_00421064(cpu); if cpu.returned_to != 0x417ba6 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417ba8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417ba8); }
 let mut pc: u32 = 0x417ba8;
 loop { match pc {
 0x417ba8 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x4e15a6u32), 0x0u8); }
  if !cpu.zf { pc = 0x417bbc; continue; }
  pc = 0x417bb2; continue;
 }
 0x417bb2 => {
  cpu.push(0x417bb7); f_00421076(cpu); if cpu.returned_to != 0x417bb7 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e15a6u32, v); }
  pc = 0x417bbc; continue;
 }
 0x417bbc => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417bbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417bbe); }
 let mut pc: u32 = 0x417bbe;
 loop { match pc {
 0x417bbe => {
  { let v: u8 = 0x1u8; cpu.w8(0x5820fau32, v); }
  cpu.push(0x417bca); f_00417ba8(cpu); if cpu.returned_to != 0x417bca { return; }
  cpu.push(0x417bcf); f_00417eab(cpu); if cpu.returned_to != 0x417bcf { return; }
  { let v: u32 = 0x5ea260u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6814u32, v); }
  { let v: u32 = 0x5ea264u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d681cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bb4u32, v); }
  cpu.push(0x417bf1); f_0041c17d(cpu); if cpu.returned_to != 0x417bf1 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c18u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c1cu32, v); }
  { let v: u32 = 0x34u32; cpu.w32(0x5928b9u32, v); }
  { let v: u32 = 0x92u32; cpu.w32(0x5928bdu32, v); }
  { let v: u32 = 0x2bu32; cpu.w32(0x5928c1u32, v); }
  { let v: u32 = 0x112u32; cpu.w32(0x5928c5u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417c69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417c69); }
 let mut pc: u32 = 0x417c69;
 loop { match pc {
 0x417c69 => {
  { let v: u8 = cpu.r8(0x4e15a6u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x417c74; continue; }
  pc = 0x417c72; continue;
 }
 0x417c72 => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x417c74; continue;
 }
 0x417c74 => {
  cpu.push(0x417c79); f_00421064(cpu); if cpu.returned_to != 0x417c79 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417c7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417c7a); }
 let mut pc: u32 = 0x417c7a;
 loop { match pc {
 0x417c7a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x417c82); f_00417c93(cpu); if cpu.returned_to != 0x417c82 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417c86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417c86); }
 let mut pc: u32 = 0x417c86;
 loop { match pc {
 0x417c86 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x417c8e); f_00417c93(cpu); if cpu.returned_to != 0x417c8e { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417c92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417c92); }
 let mut pc: u32 = 0x417c92;
 loop { match pc {
 0x417c92 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417c93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417c93); }
 let mut pc: u32 = 0x417c93;
 loop { match pc {
 0x417c93 => {
  { cpu.pushad(); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x417c9b); f_00405250(cpu); if cpu.returned_to != 0x417c9b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417ca0); }
 let mut pc: u32 = 0x417ca0;
 loop { match pc {
 0x417ca0 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x417ca7; continue; }
  pc = 0x417ca5; continue;
 }
 0x417ca5 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x417ca7; continue;
 }
 0x417ca7 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x40u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x576164u32; cpu.edx = v; }
  { let v: u32 = 0x100u32; cpu.ebx = v; }
  { cpu.pushad(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x417cce); f_0040c240(cpu); if cpu.returned_to != 0x417cce { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x576164u32; cpu.esi = v; }
  { let v: u32 = 0xfu32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { cpu.pushad(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x417ceb); f_0040c020(cpu); if cpu.returned_to != 0x417ceb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  pc = 0x417cef; continue;
 }
 0x417cef => {
  cpu.push(0x417cf4); f_0040c190(cpu); if cpu.returned_to != 0x417cf4 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x417d06; continue; }
  pc = 0x417cf9; continue;
 }
 0x417cf9 => {
  let t = cpu.r32(0x4d6f8cu32); cpu.push(0x417cff); cpu.call(t); if cpu.returned_to != 0x417cff { return; }
  cpu.push(0x417d04); f_00417e0d(cpu); if cpu.returned_to != 0x417d04 { return; }
  cpu.poll(); pc = 0x417cef; continue;
 }
 0x417d06 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417d55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417d55); }
 let mut pc: u32 = 0x417d55;
 loop { match pc {
 0x417d55 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x417d5c; continue; }
  pc = 0x417d5a; continue;
 }
 0x417d5a => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x417d5c; continue;
 }
 0x417d5c => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x40u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x576164u32; cpu.edx = v; }
  { let v: u32 = 0x100u32; cpu.ebx = v; }
  { cpu.pushad(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x417d83); f_0040c240(cpu); if cpu.returned_to != 0x417d83 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x576164u32; cpu.esi = v; }
  { let v: u32 = 0xfu32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x417d9f); f_0040c020(cpu); if cpu.returned_to != 0x417d9f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f56u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417dab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417dab); }
 let mut pc: u32 = 0x417dab;
 loop { match pc {
 0x417dab => {
  { cpu.pushad(); }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x417db2; continue; }
  pc = 0x417db0; continue;
 }
 0x417db0 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x417db2; continue;
 }
 0x417db2 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x40u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0xfu32; cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x417dda); f_0040c020(cpu); if cpu.returned_to != 0x417dda { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x417ddd; continue;
 }
 0x417ddd => {
  cpu.push(0x417de2); f_0040c190(cpu); if cpu.returned_to != 0x417de2 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x417df4; continue; }
  pc = 0x417de7; continue;
 }
 0x417de7 => {
  let t = cpu.r32(0x4d6f8cu32); cpu.push(0x417ded); cpu.call(t); if cpu.returned_to != 0x417ded { return; }
  cpu.push(0x417df2); f_00417e0d(cpu); if cpu.returned_to != 0x417df2 { return; }
  cpu.poll(); pc = 0x417ddd; continue;
 }
 0x417df4 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417df6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417df6); }
 let mut pc: u32 = 0x417df6;
 loop { match pc {
 0x417df6 => {
  { cpu.pushad(); }
  cpu.push(0x417dfc); f_0040c190(cpu); if cpu.returned_to != 0x417dfc { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f56u32, v); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x417e0b; continue; }
  pc = 0x417e06; continue;
 }
 0x417e06 => {
  cpu.push(0x417e0b); f_00417e0d(cpu); if cpu.returned_to != 0x417e0b { return; }
  pc = 0x417e0b; continue;
 }
 0x417e0b => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417e0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417e0d); }
 let mut pc: u32 = 0x417e0d;
 loop { match pc {
 0x417e0d => {
  let t = cpu.r32(0x4d6f8cu32); cpu.push(0x417e13); cpu.call(t); if cpu.returned_to != 0x417e13 { return; }
  { cpu.pushad(); }
  { let v = 0x5dfd40u32; cpu.push(v); }
  cpu.push(0x417e1e); f_00404910(cpu); if cpu.returned_to != 0x417e1e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417e23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417e23); }
 let mut pc: u32 = 0x417e23;
 loop { match pc {
 0x417e23 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { let v: u32 = 0x55b6b8u32; cpu.edi = v; }
  { cpu.stos(4, true); }
  cpu.push(0x417e36); f_00417e3e(cpu); if cpu.returned_to != 0x417e36 { return; }
  { cpu.pushad(); }
  cpu.push(0x417e3c); f_0040bf80(cpu); if cpu.returned_to != 0x417e3c { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417e3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417e3e); }
 let mut pc: u32 = 0x417e3e;
 loop { match pc {
 0x417e3e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.pushad(); }
  { let v = 0x55b6b8u32; cpu.push(v); }
  cpu.push(0x417e4d); f_004049d0(cpu); if cpu.returned_to != 0x417e4d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x55b6b8u32; cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x417e61); f_0040bfc0(cpu); if cpu.returned_to != 0x417e61 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417e7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417e7e); }
 let mut pc: u32 = 0x417e7e;
 loop { match pc {
 0x417e7e => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x417e86; continue; }
  pc = 0x417e82; continue;
 }
 0x417e82 => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  f_00417e90(cpu); return;
 }
 0x417e86 => {
  { cpu.sub8(cpu.r8(0x5cd0bau32), 0x0u8); }
  if !cpu.zf { f_00417e90(cpu); return; }
  pc = 0x417e8f; continue;
 }
 0x417e8f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417e90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417e90); }
 let mut pc: u32 = 0x417e90;
 loop { match pc {
 0x417e90 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x417ea5); f_00404980(cpu); if cpu.returned_to != 0x417ea5 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417eab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417eab); }
 let mut pc: u32 = 0x417eab;
 loop { match pc {
 0x417eab => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417eb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417eb0); }
 let mut pc: u32 = 0x417eb0;
 loop { match pc {
 0x417eb0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b395eu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5b3960u32, v); }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.sf { pc = 0x417ec8; continue; }
  pc = 0x417ec3; continue;
 }
 0x417ec3 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x417ec8; continue;
 }
 0x417ec8 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x417ed3; continue; }
  pc = 0x417ece; continue;
 }
 0x417ece => {
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  pc = 0x417ed3; continue;
 }
 0x417ed3 => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x417f32; continue; }
  pc = 0x417edc; continue;
 }
 0x417edc => {
  { let v: u16 = cpu.r16(0x4d678eu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x220u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x417ef7; continue; }
  pc = 0x417eed; continue;
 }
 0x417eed => {
  { let v: u16 = cpu.r16(0x4d678eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x220u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x417ef7; continue;
 }
 0x417ef7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b395eu32, v); }
  { let v: u16 = cpu.r16(0x4d6790u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0x20u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x417f29; continue; }
  pc = 0x417f16; continue;
 }
 0x417f16 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6790u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x4d6bccu32)); cpu.ebx = v; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x417f29; continue;
 }
 0x417f29 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5b3960u32, v); }
  pc = 0x417f77; continue;
 }
 0x417f32 => {
  { let v: u16 = cpu.r16(0x4d678eu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x110u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x417f4d; continue; }
  pc = 0x417f43; continue;
 }
 0x417f43 => {
  { let v: u16 = cpu.r16(0x4d678eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x110u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x417f4d; continue;
 }
 0x417f4d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b395eu32, v); }
  { let v: u16 = cpu.r16(0x4d6790u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0xb8u16); }
  if (cpu.sf == cpu.of) { pc = 0x417f70; continue; }
  pc = 0x417f64; continue;
 }
 0x417f64 => {
  { let v: u16 = cpu.r16(0x4d6790u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0xb8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x417f70; continue;
 }
 0x417f70 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5b3960u32, v); }
  pc = 0x417f77; continue;
 }
 0x417f77 => {
  { let v: u16 = cpu.r16(0x5b395eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b3960u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417f85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417f85); }
 let mut pc: u32 = 0x417f85;
 loop { match pc {
 0x417f85 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.push(0x417f8f); f_00415898(cpu); if cpu.returned_to != 0x417f8f { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x575f19u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x417f9e); f_00415898(cpu); if cpu.returned_to != 0x417f9e { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x575f1au32, v); }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x417fad); f_00415898(cpu); if cpu.returned_to != 0x417fad { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x575f1bu32, v); }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.push(0x417fbc); f_00415898(cpu); if cpu.returned_to != 0x417fbc { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x575f1cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00417fca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x417fca); }
 let mut pc: u32 = 0x417fca;
 loop { match pc {
 0x417fca => {
  { cpu.sub16(cpu.r16(0x4d675au32), 0xffffu16); }
  if cpu.zf { pc = 0x41801a; continue; }
  pc = 0x417fd4; continue;
 }
 0x417fd4 => {
  { let v: u16 = cpu.r16(0x4d675au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d675cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x4e15bau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4b929eu32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x417ff3); f_0041843d(cpu); if cpu.returned_to != 0x417ff3 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e15bau32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x4b929eu32, v); }
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x4d675au32)); }
  if !cpu.zf { pc = 0x41801a; continue; }
  pc = 0x418008; continue;
 }
 0x418008 => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x4d675cu32)); }
  if !cpu.zf { pc = 0x41801a; continue; }
  pc = 0x418011; continue;
 }
 0x418011 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x4d675au32, v); }
  pc = 0x41801a; continue;
 }
 0x41801a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041801b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41801b); }
 let mut pc: u32 = 0x41801b;
 loop { match pc {
 0x41801b => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x40u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67fcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x41808b; continue; }
  pc = 0x418030; continue;
 }
 0x418030 => {
  { let v: u16 = cpu.r16(0x4d67feu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x418057; continue; }
  pc = 0x418041; continue;
 }
 0x418041 => {
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0x20u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x110u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x41805f; continue;
 }
 0x418057 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x88u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x5cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x41805f; continue;
 }
 0x41805f => {
  cpu.push(0x418064); f_00417eb0(cpu); if cpu.returned_to != 0x418064 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d675au32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d675cu32, v); }
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d675eu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6760u32, v); }
  pc = 0x41808b; continue;
 }
 0x41808b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041808c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41808c); }
 let mut pc: u32 = 0x41808c;
 loop { match pc {
 0x41808c => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x40u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x4180bf; continue; }
  pc = 0x4180a9; continue;
 }
 0x4180a9 => {
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0x20u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x110u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x4180c7; continue;
 }
 0x4180bf => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x88u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x5cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x4180c7; continue;
 }
 0x4180c7 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67feu32.wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67fcu32.wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004180d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4180d8); } r_004180d8(cpu, 0x4180d8); }
pub fn f_00418c6d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x418c6d); } r_004180d8(cpu, 0x418c6d); }
fn r_004180d8(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4180d8 => {
  { cpu.sub8(cpu.r8(0x58c552u32), 0x0u8); }
  if !cpu.zf { pc = 0x418198; continue; }
  pc = 0x4180e5; continue;
 }
 0x4180e5 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x418198; continue; }
  pc = 0x4180f2; continue;
 }
 0x4180f2 => {
  { let v = cpu.edi; cpu.push(v); }
  { cpu.pushad(); }
  cpu.push(0x4180f9); f_00403030(cpu); if cpu.returned_to != 0x4180f9 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x418100; continue; }
  pc = 0x4180ff; continue;
 }
 0x4180ff => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418100 => {
  { let v: u8 = 0x1u8; cpu.w8(0x59d977u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x59d979u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x59d97au32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x59d978u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x59d97bu32, v); }
  { cpu.sub8(cpu.r8(0x4e1558u32), 0x0u8); }
  if cpu.zf { pc = 0x418198; continue; }
  pc = 0x41812c; continue;
 }
 0x41812c => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x418198; continue; }
  pc = 0x418135; continue;
 }
 0x418135 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x418198; continue; }
  pc = 0x41813e; continue;
 }
 0x41813e => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x2u32); }
  if cpu.zf { pc = 0x418193; continue; }
  pc = 0x41814a; continue;
 }
 0x41814a => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x418193; continue; }
  pc = 0x418154; continue;
 }
 0x418154 => {
  cpu.push(0x418159); f_00418237(cpu); if cpu.returned_to != 0x418159 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5928b5u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5928b7u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d693cu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d693eu32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5928b1u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5928b3u32, v); }
  pc = 0x418ca6; continue;
 }
 0x418193 => {
  cpu.push(0x418198); f_004181a0(cpu); if cpu.returned_to != 0x418198 { return; }
  pc = 0x418198; continue;
 }
 0x418198 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418c6d => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x418c80; continue; }
  pc = 0x418c76; continue;
 }
 0x418c76 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x418c80; continue; }
  pc = 0x418c7f; continue;
 }
 0x418c7f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418c80 => {
  cpu.push(0x418c85); f_00418beb(cpu); if cpu.returned_to != 0x418c85 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5928b1u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5928b3u32, v); }
  cpu.push(0x418c97); f_00418ba3(cpu); if cpu.returned_to != 0x418c97 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5928b5u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5928b7u32, v); }
  pc = 0x418ca6; continue;
 }
 0x418ca6 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x418cb9; continue; }
  pc = 0x418caf; continue;
 }
 0x418caf => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x418cb9; continue; }
  pc = 0x418cb8; continue;
 }
 0x418cb8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418cb9 => {
  { cpu.sub8(cpu.r8(0x58c63au32), 0x0u8); }
  if cpu.zf { pc = 0x418cc3; continue; }
  pc = 0x418cc2; continue;
 }
 0x418cc2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418cc3 => {
  { cpu.sub8(cpu.r8(0x58c632u32), 0x0u8); }
  if cpu.zf { pc = 0x418cd4; continue; }
  pc = 0x418ccc; continue;
 }
 0x418ccc => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418cd4 => {
  { cpu.sub8(cpu.r8(0x58c631u32), 0x0u8); }
  if cpu.zf { pc = 0x418ce5; continue; }
  pc = 0x418cdd; continue;
 }
 0x418cdd => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418ce5 => {
  { cpu.sub32(cpu.r32(0x58c529u32), 0x0u32); }
  if cpu.zf { pc = 0x418cef; continue; }
  pc = 0x418cee; continue;
 }
 0x418cee => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418cef => {
  { let v: u8 = 0x0u8; cpu.w8(0x5928aeu32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.pushad(); }
  cpu.push(0x418cfd); f_00403030(cpu); if cpu.returned_to != 0x418cfd { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x418d09; continue; }
  pc = 0x418d02; continue;
 }
 0x418d02 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5928aeu32, v); }
  pc = 0x418d09; continue;
 }
 0x418d09 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x58c49du32), 0x0u32); }
  if cpu.zf { pc = 0x418d24; continue; }
  pc = 0x418d13; continue;
 }
 0x418d13 => {
  { cpu.sub8(cpu.r8(0x58c633u32), 0x0u8); }
  if cpu.zf { pc = 0x418d24; continue; }
  pc = 0x418d1c; continue;
 }
 0x418d1c => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418d24 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x418d62; continue; }
  pc = 0x418d30; continue;
 }
 0x418d30 => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if cpu.zf { pc = 0x418d5b; continue; }
  pc = 0x418d39; continue;
 }
 0x418d39 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fcbu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c434u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c439u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c43au32, v); }
  cpu.push(0x418d5a); f_00419f96(cpu); if cpu.returned_to != 0x418d5a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418d5b => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c434u32, v); }
  pc = 0x418d62; continue;
 }
 0x418d62 => {
  { cpu.sub8(cpu.r8(0x54acd1u32), 0x2u8); }
  if cpu.zf { pc = 0x418d88; continue; }
  pc = 0x418d6b; continue;
 }
 0x418d6b => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x418d89; continue; }
  pc = 0x418d74; continue;
 }
 0x418d74 => {
  { cpu.sub8(cpu.r8(0x590aa1u32), 0x0u8); }
  if !cpu.zf { pc = 0x418d88; continue; }
  pc = 0x418d7d; continue;
 }
 0x418d7d => {
  { cpu.sub8(cpu.r8(0x4e1e4cu32), 0xffu8); }
  if !cpu.zf { pc = 0x418d88; continue; }
  pc = 0x418d86; continue;
 }
 0x418d86 => {
  pc = 0x418d89; continue;
 }
 0x418d88 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418d89 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x4197bf; continue; }
  pc = 0x418d96; continue;
 }
 0x418d96 => {
  { let v: u8 = 0x1u8; cpu.w8(0x59d978u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x59d977u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x59d979u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x59d97au32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x56c6d9u32, v); }
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x2u8); }
  if cpu.zf { pc = 0x418dc6; continue; }
  pc = 0x418dc5; continue;
 }
 0x418dc5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418dc6 => {
  cpu.push(0x418dcb); f_00418c13(cpu); if cpu.returned_to != 0x418dcb { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fabu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fccu32.wrapping_add(cpu.eax)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.sf { pc = 0x419da2; continue; }
  pc = 0x418de8; continue;
 }
 0x418de8 => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if cpu.zf { pc = 0x418df7; continue; }
  pc = 0x418df1; continue;
 }
 0x418df1 => {
  cpu.push(0x418df6); f_00419f0d(cpu); if cpu.returned_to != 0x418df6 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418df7 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if cpu.zf { pc = 0x418e06; continue; }
  pc = 0x418e00; continue;
 }
 0x418e00 => {
  cpu.push(0x418e05); f_0044d6ec(cpu); if cpu.returned_to != 0x418e05 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418e06 => {
  cpu.push(0x418e0b); f_004197c0(cpu); if cpu.returned_to != 0x418e0b { return; }
  if cpu.cf { pc = 0x418e0e; continue; }
  pc = 0x418e0d; continue;
 }
 0x418e0d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418e0e => {
  { cpu.pushad(); }
  cpu.push(0x418e14); f_0044e68e(cpu); if cpu.returned_to != 0x418e14 { return; }
  if !cpu.cf { pc = 0x418e7a; continue; }
  pc = 0x418e16; continue;
 }
 0x418e16 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x418e1c); f_0041879e(cpu); if cpu.returned_to != 0x418e1c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x418e23); f_00419d49(cpu); if cpu.returned_to != 0x418e23 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5b358cu32, v); }
  cpu.push(0x418e33); f_0044d89b(cpu); if cpu.returned_to != 0x418e33 { return; }
  cpu.push(0x418e38); f_00419954(cpu); if cpu.returned_to != 0x418e38 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418e7a => {
  { cpu.popad(); }
  { cpu.pushad(); }
  cpu.push(0x418e81); f_00418c13(cpu); if cpu.returned_to != 0x418e81 { return; }
  cpu.push(0x418e86); f_00447187(cpu); if cpu.returned_to != 0x418e86 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x418e8e; continue; }
  pc = 0x418e89; continue;
 }
 0x418e89 => {
  pc = 0x4197bf; continue;
 }
 0x418e8e => {
  { cpu.sub32(cpu.r32(0x5b3588u32), 0x0u32); }
  if cpu.zf { pc = 0x418e9f; continue; }
  pc = 0x418e97; continue;
 }
 0x418e97 => {
  cpu.push(0x418e9c); f_00419ea8(cpu); if cpu.returned_to != 0x418e9c { return; }
  if !cpu.cf { pc = 0x418e9f; continue; }
  pc = 0x418e9e; continue;
 }
 0x418e9e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418e9f => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x418ec4; continue; }
  pc = 0x418ea9; continue;
 }
 0x418ea9 => {
  { cpu.sub8(cpu.r8(0x4d67d4u32), 0x2u8); }
  if !cpu.zf { pc = 0x418ec4; continue; }
  pc = 0x418eb2; continue;
 }
 0x418eb2 => {
  cpu.push(0x418eb7); f_00418bd9(cpu); if cpu.returned_to != 0x418eb7 { return; }
  { cpu.pushad(); }
  cpu.push(0x418ebd); f_00472452(cpu); if cpu.returned_to != 0x418ebd { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x419389; continue; }
  pc = 0x418ec4; continue;
 }
 0x418ec4 => {
  { cpu.sub8(cpu.r8(0x4d67d4u32), 0x5u8); }
  if !cpu.zf { pc = 0x418ee5; continue; }
  pc = 0x418ecd; continue;
 }
 0x418ecd => {
  cpu.push(0x418ed2); f_004184ca(cpu); if cpu.returned_to != 0x418ed2 { return; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x418ee5; continue; }
  pc = 0x418ed9; continue;
 }
 0x418ed9 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x418ee5; continue; }
  pc = 0x418ee0; continue;
 }
 0x418ee0 => {
  pc = 0x4194e9; continue;
 }
 0x418ee5 => {
  { cpu.sub8(cpu.r8(0x4d67d4u32), 0x6u8); }
  if !cpu.zf { pc = 0x418eef; continue; }
  pc = 0x418eee; continue;
 }
 0x418eee => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418eef => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x1u32); }
  if cpu.zf { pc = 0x418fce; continue; }
  pc = 0x418eff; continue;
 }
 0x418eff => {
  { cpu.sub8(cpu.r8(0x5928aeu32), 0x0u8); }
  if !cpu.zf { pc = 0x418fce; continue; }
  pc = 0x418f0c; continue;
 }
 0x418f0c => {
  { cpu.pushad(); }
  cpu.push(0x418f12); f_00418bd9(cpu); if cpu.returned_to != 0x418f12 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  cpu.push(0x418f1e); f_00471211(cpu); if cpu.returned_to != 0x418f1e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x418fce; continue; }
  pc = 0x418f2c; continue;
 }
 0x418f2c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x418fbe; continue; }
  pc = 0x418f43; continue;
 }
 0x418f43 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x418fc2; continue; }
  pc = 0x418f4a; continue;
 }
 0x418f4a => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x418f6d; continue; }
  pc = 0x418f51; continue;
 }
 0x418f51 => {
  { cpu.pushad(); }
  cpu.push(0x418f57); f_00418bd9(cpu); if cpu.returned_to != 0x418f57 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c889u32, v); }
  cpu.push(0x418f63); f_0046d5b9(cpu); if cpu.returned_to != 0x418f63 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c889u32, v); }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x418fc2; continue; }
  pc = 0x418f6d; continue;
 }
 0x418f6d => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x418fc2; continue; }
  pc = 0x418f74; continue;
 }
 0x418f74 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x418f76; continue;
 }
 0x418f76 => {
  cpu.push(0x418f7b); f_0041879e(cpu); if cpu.returned_to != 0x418f7b { return; }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x418f87); f_00419d49(cpu); if cpu.returned_to != 0x418f87 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x5b3588u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3856u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3857u32, v); }
  cpu.push(0x418fa1); f_00471e80(cpu); if cpu.returned_to != 0x418fa1 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c465u32, v); }
  cpu.push(0x418fad); f_0044cbe1(cpu); if cpu.returned_to != 0x418fad { return; }
  { let v: u16 = cpu.r16(0x4d675au32).wrapping_sub(0x40u16); cpu.w16(0x4d675au32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x4d675cu32), 0x20u16); cpu.w16(0x4d675cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418fbe => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x418fce; continue;
 }
 0x418fc2 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4d67d4u32), 0x6u8); }
  if !cpu.zf { pc = 0x418fce; continue; }
  pc = 0x418fcd; continue;
 }
 0x418fcd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418fce => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c889u32, v); }
  cpu.push(0x418fda); f_0046d6dc(cpu); if cpu.returned_to != 0x418fda { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c889u32, v); }
  if !cpu.cf { pc = 0x419001; continue; }
  pc = 0x418fe3; continue;
 }
 0x418fe3 => {
  cpu.push(0x418fe8); f_00418bd9(cpu); if cpu.returned_to != 0x418fe8 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c889u32, v); }
  cpu.push(0x418ff4); f_0046d5b9(cpu); if cpu.returned_to != 0x418ff4 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c889u32, v); }
  if cpu.cf { pc = 0x41910d; continue; }
  pc = 0x419001; continue;
 }
 0x419001 => {
  { let v: u16 = 0xf1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x509290u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0xa5u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x41910d; continue; }
  pc = 0x419028; continue;
 }
 0x419028 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x41910d; continue; }
  pc = 0x419033; continue;
 }
 0x419033 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x40u8); }
  { cpu.sub8(cpu.r8(0x4e1fc0u32), 0x0u8); }
  if cpu.zf { pc = 0x419054; continue; }
  pc = 0x419041; continue;
 }
 0x419041 => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if !cpu.zf { pc = 0x419054; continue; }
  pc = 0x41904b; continue;
 }
 0x41904b => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x419096; continue; }
  pc = 0x419054; continue;
 }
 0x419054 => {
  cpu.push(0x419059); f_0041a3a3(cpu); if cpu.returned_to != 0x419059 { return; }
  if cpu.cf { pc = 0x4194fb; continue; }
  pc = 0x41905f; continue;
 }
 0x41905f => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if cpu.zf { pc = 0x419096; continue; }
  pc = 0x41906d; continue;
 }
 0x41906d => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x419091; continue; }
  pc = 0x419076; continue;
 }
 0x419076 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x41910d; continue; }
  pc = 0x419091; continue;
 }
 0x419091 => {
  pc = 0x4194fb; continue;
 }
 0x419096 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x4190ae; continue; }
  pc = 0x41909d; continue;
 }
 0x41909d => {
  { let v: u32 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x509290u32); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { pc = 0x4190bc; continue; }
  pc = 0x4190ae; continue;
 }
 0x4190ae => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x41910d; continue; }
  pc = 0x4190b5; continue;
 }
 0x4190b5 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4190c1; continue; }
  pc = 0x4190bc; continue;
 }
 0x4190bc => {
  cpu.push(0x4190c1); f_0041abde(cpu); if cpu.returned_to != 0x4190c1 { return; }
  pc = 0x4190c1; continue;
 }
 0x4190c1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c42cu32, v); }
  cpu.push(0x4190e2); f_0041a8a2(cpu); if cpu.returned_to != 0x4190e2 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c42cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x56c6f6u32), cpu.eax); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x41910c; continue; }
  pc = 0x4190f8; continue;
 }
 0x4190f8 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5b358cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x56c6f6u32, v); }
  pc = 0x41910c; continue;
 }
 0x41910c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41910d => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x4196d9; continue; }
  pc = 0x41911b; continue;
 }
 0x41911b => {
  cpu.push(0x419120); f_004184ca(cpu); if cpu.returned_to != 0x419120 { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x419534; continue; }
  pc = 0x41912b; continue;
 }
 0x41912b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x41914a; continue; }
  pc = 0x419132; continue;
 }
 0x419132 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x41914a; continue; }
  pc = 0x41913c; continue;
 }
 0x41913c => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0xfu8); }
  if !cpu.zf { pc = 0x419389; continue; }
  pc = 0x41914a; continue;
 }
 0x41914a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x419389; continue; }
  pc = 0x419155; continue;
 }
 0x419155 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x419548; continue; }
  pc = 0x419160; continue;
 }
 0x419160 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x41940c; continue; }
  pc = 0x41916b; continue;
 }
 0x41916b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x5928b5u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x5928b7u32) as u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x419186); f_00471538(cpu); if cpu.returned_to != 0x419186 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x4191bf; continue; }
  pc = 0x41918a; continue;
 }
 0x41918a => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x1u32); }
  if cpu.zf { pc = 0x41940b; continue; }
  pc = 0x41919a; continue;
 }
 0x41919a => {
  { cpu.pushad(); }
  cpu.push(0x4191a0); f_00418bd9(cpu); if cpu.returned_to != 0x4191a0 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  cpu.push(0x4191ac); f_00471211(cpu); if cpu.returned_to != 0x4191ac { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x41928c; continue; }
  pc = 0x4191ba; continue;
 }
 0x4191ba => {
  cpu.poll(); pc = 0x418f76; continue;
 }
 0x4191bf => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x41928c; continue; }
  pc = 0x4191cc; continue;
 }
 0x4191cc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c44au32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x4191fc; continue; }
  pc = 0x4191df; continue;
 }
 0x4191df => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x5928b5u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x5928b7u32) as u32; cpu.edi = v; }
  cpu.push(0x4191f4); f_004715d5(cpu); if cpu.returned_to != 0x4191f4 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x41940b; continue; }
  pc = 0x4191fc; continue;
 }
 0x4191fc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c44bu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x41922c; continue; }
  pc = 0x41920f; continue;
 }
 0x41920f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x5928b5u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x5928b7u32) as u32; cpu.edi = v; }
  cpu.push(0x419224); f_004715d5(cpu); if cpu.returned_to != 0x419224 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x41940b; continue; }
  pc = 0x41922c; continue;
 }
 0x41922c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c44cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x41925c; continue; }
  pc = 0x41923f; continue;
 }
 0x41923f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x5928b5u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x5928b7u32) as u32; cpu.edi = v; }
  cpu.push(0x419254); f_004715d5(cpu); if cpu.returned_to != 0x419254 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x41940b; continue; }
  pc = 0x41925c; continue;
 }
 0x41925c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c44du32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x41928c; continue; }
  pc = 0x41926f; continue;
 }
 0x41926f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x5928b5u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x5928b7u32) as u32; cpu.edi = v; }
  cpu.push(0x419284); f_004715d5(cpu); if cpu.returned_to != 0x419284 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x41940b; continue; }
  pc = 0x41928c; continue;
 }
 0x41928c => {
  cpu.push(0x419291); f_00418bd9(cpu); if cpu.returned_to != 0x419291 { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x4192d1; continue; }
  pc = 0x419298; continue;
 }
 0x419298 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x4192a8; continue; }
  pc = 0x4192a3; continue;
 }
 0x4192a3 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x4192ad; continue; }
  pc = 0x4192a8; continue;
 }
 0x4192a8 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x4192ad; continue;
 }
 0x4192ad => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x4192d1; continue; }
  pc = 0x4192b9; continue;
 }
 0x4192b9 => {
  cpu.push(0x4192be); f_00470fc5(cpu); if cpu.returned_to != 0x4192be { return; }
  if !cpu.cf { pc = 0x4192d1; continue; }
  pc = 0x4192c0; continue;
 }
 0x4192c0 => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x4e1559u32)); }
  if cpu.zf { pc = 0x4193f3; continue; }
  pc = 0x4192cc; continue;
 }
 0x4192cc => {
  pc = 0x41936f; continue;
 }
 0x4192d1 => {
  cpu.push(0x4192d6); f_0041a605(cpu); if cpu.returned_to != 0x4192d6 { return; }
  if !cpu.zf { pc = 0x4192e6; continue; }
  pc = 0x4192d8; continue;
 }
 0x4192d8 => {
  { cpu.sub16(cpu.r16(0x4d6856u32), 0x4u16); }
  if !cpu.zf { pc = 0x419389; continue; }
  pc = 0x4192e6; continue;
 }
 0x4192e6 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x419365; continue; }
  pc = 0x4192f0; continue;
 }
 0x4192f0 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x1u32); }
  if cpu.zf { pc = 0x419365; continue; }
  pc = 0x4192fc; continue;
 }
 0x4192fc => {
  cpu.push(0x419301); f_00418bd9(cpu); if cpu.returned_to != 0x419301 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d24u32, v); }
  cpu.push(0x41930d); f_0047113a(cpu); if cpu.returned_to != 0x41930d { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  if !cpu.cf { pc = 0x419365; continue; }
  pc = 0x419316; continue;
 }
 0x419316 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if cpu.zf { pc = 0x419350; continue; }
  pc = 0x41932b; continue;
 }
 0x41932b => {
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x419365; continue; }
  pc = 0x41932f; continue;
 }
 0x41932f => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x58c44au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x419365; continue; }
  pc = 0x41934a; continue;
 }
 0x41934a => {
  { cpu.popad(); }
  pc = 0x41940b; continue;
 }
 0x419350 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x419365; continue; }
  pc = 0x41935c; continue;
 }
 0x41935c => {
  { cpu.sub8(cpu.r8(0x5928aeu32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x41934a; continue; }
  pc = 0x419365; continue;
 }
 0x419365 => {
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x59da62u32), 0x2bu8); }
  if !cpu.zf { pc = 0x41936f; continue; }
  pc = 0x41936f; continue;
 }
 0x41936f => {
  { cpu.pushad(); }
  cpu.push(0x419375); f_00418bd9(cpu); if cpu.returned_to != 0x419375 { return; }
  cpu.push(0x41937a); f_0046e362(cpu); if cpu.returned_to != 0x41937a { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x41940b; continue; }
  pc = 0x419381; continue;
 }
 0x419381 => {
  cpu.push(0x419386); f_00441955(cpu); if cpu.returned_to != 0x419386 { return; }
  if cpu.cf { pc = 0x419389; continue; }
  pc = 0x419388; continue;
 }
 0x419388 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x419389 => {
  { cpu.sub16(cpu.r16(0x4d6856u32), 0x0u16); }
  if cpu.zf { pc = 0x41940b; continue; }
  pc = 0x419393; continue;
 }
 0x419393 => {
  { cpu.sub16(cpu.r16(0x4d6856u32), 0xaefu16); }
  if cpu.zf { pc = 0x41940b; continue; }
  pc = 0x41939e; continue;
 }
 0x41939e => {
  cpu.push(0x4193a3); f_0047111f(cpu); if cpu.returned_to != 0x4193a3 { return; }
  if !cpu.cf { pc = 0x4193af; continue; }
  pc = 0x4193a5; continue;
 }
 0x4193a5 => {
  { cpu.sub8(cpu.r8(0x4e1559u32), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x4193af; continue; }
  pc = 0x4193ad; continue;
 }
 0x4193ad => {
  pc = 0x41940b; continue;
 }
 0x4193af => {
  { cpu.sub8(cpu.r8(0x4d67d4u32), 0x2u8); }
  if !cpu.zf { pc = 0x4193d9; continue; }
  pc = 0x4193b8; continue;
 }
 0x4193b8 => {
  { let v: u8 = 0x1u8; cpu.w8(0x575545u32, v); }
  cpu.push(0x4193c4); f_0044237b(cpu); if cpu.returned_to != 0x4193c4 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x575545u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xaceu32; cpu.eax = v; }
  cpu.push(0x4193d6); f_0041b7b0(cpu); if cpu.returned_to != 0x4193d6 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x41940b; continue;
 }
 0x4193d9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x575545u32, v); }
  cpu.push(0x4193e5); f_0044237b(cpu); if cpu.returned_to != 0x4193e5 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xac6u32; cpu.eax = v; }
  cpu.push(0x4193f0); f_0041b7b0(cpu); if cpu.returned_to != 0x4193f0 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x41940b; continue;
 }
 0x4193f3 => {
  { let v: u8 = 0x0u8; cpu.w8(0x575545u32, v); }
  cpu.push(0x4193ff); f_0044237b(cpu); if cpu.returned_to != 0x4193ff { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xad6u32; cpu.eax = v; }
  cpu.push(0x41940a); f_0041b7b0(cpu); if cpu.returned_to != 0x41940a { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x41940b; continue;
 }
 0x41940b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41940c => {
  cpu.push(0x419411); f_00418bd9(cpu); if cpu.returned_to != 0x419411 { return; }
  { cpu.pushad(); }
  cpu.push(0x419417); f_004712e3(cpu); if cpu.returned_to != 0x419417 { return; }
  if cpu.cf { pc = 0x419450; continue; }
  pc = 0x419419; continue;
 }
 0x419419 => {
  cpu.push(0x41941e); f_004716d0(cpu); if cpu.returned_to != 0x41941e { return; }
  { let v: u8 = 0x4u8; cpu.w8(0x4ea600u32, v); }
  cpu.push(0x41942a); f_00453e97(cpu); if cpu.returned_to != 0x41942a { return; }
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4ea600u32, v); }
  if !cpu.cf { pc = 0x41944a; continue; }
  pc = 0x419439; continue;
 }
 0x419439 => {
  { let v: u32 = cpu.edi; cpu.w32(0x592498u32, v); }
  { cpu.popad(); }
  { let v: u32 = 0x2084u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4194a8; continue;
 }
 0x41944a => {
  { cpu.popad(); }
  cpu.poll(); pc = 0x419389; continue;
 }
 0x419450 => {
  { cpu.popad(); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x4194cf; continue; }
  pc = 0x419468; continue;
 }
 0x419468 => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1cu32.wrapping_add(cpu.edi))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4194cf; continue; }
  pc = 0x419472; continue;
 }
 0x419472 => {
  { let v: u32 = cpu.edi; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x4194a8; continue;
 }
 0x4194a8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4e159au32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4e159cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e15a3u32, v); }
  cpu.push(0x4194c1); f_00441955(cpu); if cpu.returned_to != 0x4194c1 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15a3u32, v); }
  if cpu.cf { cpu.poll(); pc = 0x419389; continue; }
  pc = 0x4194ce; continue;
 }
 0x4194ce => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4194cf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x575545u32, v); }
  cpu.push(0x4194dc); f_0044237b(cpu); if cpu.returned_to != 0x4194dc { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xac6u32; cpu.eax = v; }
  cpu.push(0x4194e7); f_0041b7b0(cpu); if cpu.returned_to != 0x4194e7 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4194e9 => {
  cpu.push(0x4194ee); f_00419cf0(cpu); if cpu.returned_to != 0x4194ee { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.push(0x4194f9); f_0041b7b0(cpu); if cpu.returned_to != 0x4194f9 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4194fb => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { cpu.poll(); pc = 0x41940b; continue; }
  pc = 0x419509; continue;
 }
 0x419509 => {
  cpu.push(0x41950e); f_004184ca(cpu); if cpu.returned_to != 0x41950e { return; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { cpu.poll(); pc = 0x41940c; continue; }
  pc = 0x419519; continue;
 }
 0x419519 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x419548; continue; }
  pc = 0x419520; continue;
 }
 0x419520 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x419534; continue; }
  pc = 0x419527; continue;
 }
 0x419527 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x419534; continue; }
  pc = 0x41952e; continue;
 }
 0x41952e => {
  cpu.push(0x419533); f_00441955(cpu); if cpu.returned_to != 0x419533 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x419534 => {
  cpu.push(0x419539); f_004184ca(cpu); if cpu.returned_to != 0x419539 { return; }
  { cpu.sub8(cpu.r8(0x4d67d4u32), 0x6u8); }
  if cpu.zf { pc = 0x419547; continue; }
  pc = 0x419542; continue;
 }
 0x419542 => {
  cpu.push(0x419547); f_00441ff7(cpu); if cpu.returned_to != 0x419547 { return; }
  pc = 0x419547; continue;
 }
 0x419547 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x419548 => {
  cpu.push(0x41954d); f_0041a3a3(cpu); if cpu.returned_to != 0x41954d { return; }
  if cpu.cf { cpu.poll(); pc = 0x419547; continue; }
  pc = 0x41954f; continue;
 }
 0x41954f => {
  cpu.push(0x419554); f_00418bd9(cpu); if cpu.returned_to != 0x419554 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  cpu.push(0x419566); f_004713ce(cpu); if cpu.returned_to != 0x419566 { return; }
  if !cpu.cf { pc = 0x41958c; continue; }
  pc = 0x419568; continue;
 }
 0x419568 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if cpu.zf { pc = 0x419581; continue; }
  pc = 0x41957e; continue;
 }
 0x41957e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4195b8; continue;
 }
 0x419581 => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x4u8); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x419657; continue;
 }
 0x41958c => {
  cpu.push(0x419591); f_0046e362(cpu); if cpu.returned_to != 0x419591 { return; }
  if !cpu.cf { pc = 0x4195b8; continue; }
  pc = 0x419593; continue;
 }
 0x419593 => {
  { let v: u32 = cpu.esi; cpu.w32(0x576c00u32, v); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4195a9); f_0046e362(cpu); if cpu.returned_to != 0x4195a9 { return; }
  { cpu.sub32(cpu.r32(0x576c00u32), cpu.esi); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x4195b8; continue; }
  pc = 0x4195b2; continue;
 }
 0x4195b2 => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), 0x4u8); }
  pc = 0x419607; continue;
 }
 0x4195b8 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.ebp)), 0x2084u32); }
  if cpu.zf { pc = 0x4195f2; continue; }
  pc = 0x4195c2; continue;
 }
 0x4195c2 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x4195e0; continue; }
  pc = 0x4195cc; continue;
 }
 0x4195cc => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.eax)) & 0xfbu8; cpu.w8(0xfu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x2084u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4195f2; continue;
 }
 0x4195e0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.eax)) & 0xfbu8; cpu.w8(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x2084u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4195f2; continue;
 }
 0x4195f2 => {
  { let v: u8 = 0x0u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x419389; continue;
 }
 0x419607 => {
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x419651; continue; }
  pc = 0x419649; continue;
 }
 0x419649 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), 0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x419651; continue;
 }
 0x419651 => {
  cpu.push(0x419656); f_0044257a(cpu); if cpu.returned_to != 0x419656 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x419657 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x1cu32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x419676; continue; }
  pc = 0x419671; continue;
 }
 0x419671 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x41967b; continue; }
  pc = 0x419676; continue;
 }
 0x419676 => {
  { let v: u32 = 0x5b355cu32; cpu.ebx = v; }
  pc = 0x41967b; continue;
 }
 0x41967b => {
  { let v: u32 = cpu.r8(0x6u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x419687; continue; }
  pc = 0x419685; continue;
 }
 0x419685 => {
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x419687; continue;
 }
 0x419687 => {
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x84u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
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
  cpu.push(0x4196d8); f_0044257a(cpu); if cpu.returned_to != 0x4196d8 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4196d9 => {
  cpu.push(0x4196de); f_00418bd9(cpu); if cpu.returned_to != 0x4196de { return; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4196f7; continue; }
  pc = 0x4196e7; continue;
 }
 0x4196e7 => {
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if (cpu.sf != cpu.of) { pc = 0x41977a; continue; }
  pc = 0x4196f5; continue;
 }
 0x4196f5 => {
  pc = 0x419700; continue;
 }
 0x4196f7 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x41977a; continue; }
  pc = 0x419700; continue;
 }
 0x419700 => {
  cpu.push(0x419705); f_00471211(cpu); if cpu.returned_to != 0x419705 { return; }
  if !cpu.cf { pc = 0x41977a; continue; }
  pc = 0x419707; continue;
 }
 0x419707 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x41977a; continue; }
  pc = 0x419714; continue;
 }
 0x419714 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x41977a; continue; }
  pc = 0x41971b; continue;
 }
 0x41971b => {
  { let v: u32 = 0x1f4u32; cpu.w32(0x56c6d9u32, v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x58c49cu32), 0x0u8); }
  if !cpu.zf { pc = 0x41973d; continue; }
  pc = 0x419735; continue;
 }
 0x419735 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x41977a; continue; }
  pc = 0x41973d; continue;
 }
 0x41973d => {
  { let v: u32 = cpu.ebp; cpu.w32(0x5b3588u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3857u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3856u32, v); }
  cpu.push(0x419756); f_00471e80(cpu); if cpu.returned_to != 0x419756 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c465u32, v); }
  cpu.push(0x419763); f_00419d49(cpu); if cpu.returned_to != 0x419763 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4d675au32).wrapping_sub(0x40u16); cpu.w16(0x4d675au32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x4d675cu32), 0x20u16); cpu.w16(0x4d675cu32, v); }
  cpu.push(0x419779); f_0044cbe1(cpu); if cpu.returned_to != 0x419779 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41977a => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x4197bf; continue; }
  pc = 0x419783; continue;
 }
 0x419783 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if !cpu.zf { pc = 0x4197bf; continue; }
  pc = 0x41978f; continue;
 }
 0x41978f => {
  { let v: u8 = cpu.and8(cpu.r8(0x58c432u32), 0xfeu8); cpu.w8(0x58c432u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1fcbu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  cpu.push(0x4197a9); f_00418c13(cpu); if cpu.returned_to != 0x4197a9 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c70u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6c74u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c78u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6c7cu32, v); }
  pc = 0x4197bf; continue;
 }
 0x4197bf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x419da2 => {
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x4e1fccu32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
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
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5928cfu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x5b395eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b3960u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5b394du32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5b3951u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x5b395eu32), 0xffffu16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if cpu.zf { pc = 0x419e88; continue; }
  pc = 0x419e25; continue;
 }
 0x419e25 => {
  { let v: u16 = cpu.r16(0x5b395eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5b3960u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x419e4c; continue; }
  pc = 0x419e39; continue;
 }
 0x419e39 => {
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.edx as u8), 0x4u8); }
  if cpu.zf { pc = 0x419e4c; continue; }
  pc = 0x419e40; continue;
 }
 0x419e40 => {
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.edx as u8), 0x5u8); }
  if cpu.zf { pc = 0x419e4c; continue; }
  pc = 0x419e47; continue;
 }
 0x419e47 => {
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  pc = 0x419e4c; continue;
 }
 0x419e4c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b2u32, v); }
  { let v: u8 = cpu.shl8(((cpu.edx >> 8) as u8), 6u32); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1fccu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4d6aecu32, v); }
  { let v: u8 = 0xfcu8; cpu.w8(0x4e1f4cu32, v); }
  cpu.push(0x419e87); f_00413337(cpu); if cpu.returned_to != 0x419e87 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x419e88 => {
  { cpu.sub8(cpu.r8(0x5b3867u32), 0x0u8); }
  if cpu.zf { pc = 0x419ea7; continue; }
  pc = 0x419e91; continue;
 }
 0x419e91 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5b3867u32)); cpu.w8(0x5b3867u32, v); }
  if !cpu.zf { pc = 0x419ea7; continue; }
  pc = 0x419e99; continue;
 }
 0x419e99 => {
  { let v: u8 = 0x20u8; cpu.w8(0x5b3962u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x5b3867u32, v); }
  pc = 0x419ea7; continue;
 }
 0x419ea7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004181a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4181a0); }
 let mut pc: u32 = 0x4181a0;
 loop { match pc {
 0x4181a0 => {
  { cpu.sub8(cpu.r8(0x58c552u32), 0x0u8); }
  if !cpu.zf { pc = 0x418236; continue; }
  pc = 0x4181ad; continue;
 }
 0x4181ad => {
  { cpu.sub16(cpu.r16(0x4d675au32), 0xffffu16); }
  if !cpu.zf { pc = 0x4181cf; continue; }
  pc = 0x4181b7; continue;
 }
 0x4181b7 => {
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d675eu32, v); }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6760u32, v); }
  pc = 0x4181cf; continue;
 }
 0x4181cf => {
  cpu.push(0x4181d4); f_00418237(cpu); if cpu.returned_to != 0x4181d4 { return; }
  if cpu.cf { pc = 0x418236; continue; }
  pc = 0x4181d6; continue;
 }
 0x4181d6 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x70u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x5cu32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x4181fa; continue; }
  pc = 0x4181e5; continue;
 }
 0x4181e5 => {
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x20u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x5cu32); cpu.ebx = v; }
  pc = 0x4181fa; continue;
 }
 0x4181fa => {
  { cpu.sub16(cpu.r16(0x4d67b6u32), 0x0u16); }
  if !cpu.zf { pc = 0x41820f; continue; }
  pc = 0x418204; continue;
 }
 0x418204 => {
  { let v: u32 = cpu.eax & 0xfffcu32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xfffcu32); cpu.ebx = v; }
  pc = 0x41820f; continue;
 }
 0x41820f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d675au32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d675cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4b929eu32, v); }
  cpu.push(0x41822f); f_004188ae(cpu); if cpu.returned_to != 0x41822f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7du32, v); }
  pc = 0x418236; continue;
 }
 0x418236 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00418237(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x418237); }
 let mut pc: u32 = 0x418237;
 loop { match pc {
 0x418237 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x41823e); f_00418beb(cpu); if cpu.returned_to != 0x41823e { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5928b9u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5c5a74u32)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x5928bdu32)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x418301; continue; }
  pc = 0x41825f; continue;
 }
 0x41825f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5c5a74u32)); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x418301; continue; }
  pc = 0x41826f; continue;
 }
 0x41826f => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x5c5a6cu32)); cpu.eax = v; }
  if cpu.sf { pc = 0x418301; continue; }
  pc = 0x418281; continue;
 }
 0x418281 => {
  { let v = cpu.r16(0x4d6790u32); cpu.mul16(v); }
  { let v = cpu.r16(0x5c5a74u32); cpu.div16(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5928c1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5c5a70u32)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x5928c5u32)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x418301; continue; }
  pc = 0x4182aa; continue;
 }
 0x4182aa => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5c5a70u32)); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x418301; continue; }
  pc = 0x4182b6; continue;
 }
 0x4182b6 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x5c5a68u32)); cpu.eax = v; }
  if cpu.sf { pc = 0x418301; continue; }
  pc = 0x4182c4; continue;
 }
 0x4182c4 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x110u32); cpu.eax = v; }
  if cpu.sf { pc = 0x418301; continue; }
  pc = 0x4182cb; continue;
 }
 0x4182cb => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x4182db; continue; }
  pc = 0x4182d4; continue;
 }
 0x4182d4 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x110u32); cpu.eax = v; }
  if cpu.sf { pc = 0x418301; continue; }
  pc = 0x4182db; continue;
 }
 0x4182db => {
  { let v = cpu.r16(0x4d678eu32); cpu.mul16(v); }
  { let v = cpu.r16(0x5c5a70u32); cpu.div16(v); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d678eu32)); }
  if (cpu.sf == cpu.of) { pc = 0x418301; continue; }
  pc = 0x4182f4; continue;
 }
 0x4182f4 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6790u32)); }
  if (cpu.sf == cpu.of) { pc = 0x418301; continue; }
  pc = 0x4182fd; continue;
 }
 0x4182fd => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418301 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041840f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41840f); }
 let mut pc: u32 = 0x41840f;
 loop { match pc {
 0x41840f => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x6au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x4e15bau32); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x5928d0u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4b929eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add(0x7d05c600u32); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { cpu.pop(); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041843d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41843d); }
 let mut pc: u32 = 0x41843d;
 loop { match pc {
 0x41843d => {
  { let v: u16 = cpu.r16(0x4d675eu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6760u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  f_00418459(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00418459(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x418459); }
 let mut pc: u32 = 0x418459;
 loop { match pc {
 0x418459 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x418467; continue; }
  pc = 0x418464; continue;
 }
 0x418464 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x418467; continue;
 }
 0x418467 => {
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x41846f; continue; }
  pc = 0x41846c; continue;
 }
 0x41846c => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x41846f; continue;
 }
 0x41846f => {
  { let v: u16 = cpu.r16(0x4d678eu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_sub(cpu.r16(0x4d6ba4u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.esi as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x418485; continue; }
  pc = 0x418482; continue;
 }
 0x418482 => {
  { let v: u16 = (cpu.esi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x418485; continue;
 }
 0x418485 => {
  { let v: u16 = cpu.r16(0x4d6790u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_sub(cpu.r16(0x4d6ba8u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), (cpu.esi as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41849b; continue; }
  pc = 0x418498; continue;
 }
 0x418498 => {
  { let v: u16 = (cpu.esi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x41849b; continue;
 }
 0x41849b => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d675eu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6760u32, v); }
  { cpu.sub16(cpu.r16(0x4d67b6u32), 0x0u16); }
  if !cpu.zf { pc = 0x4184bb; continue; }
  pc = 0x4184b3; continue;
 }
 0x4184b3 => {
  { let v: u16 = (cpu.ecx as u16) & 0xfffcu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.and16((cpu.edx as u16), 0xfffcu16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x4184bb; continue;
 }
 0x4184bb => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6a00u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6a04u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004184ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4184ca); }
 let mut pc: u32 = 0x4184ca;
 loop { match pc {
 0x4184ca => {
  { let v: u32 = cpu.r16(0x4d6758u32) as u32; cpu.eax = v; }
  { let v: u32 = 0xb8u32; cpu.ecx = v; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ecx as u8), 4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x14u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00418558(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x418558); }
 let mut pc: u32 = 0x418558;
 loop { match pc {
 0x418558 => {
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x2u8); }
  if !cpu.zf { pc = 0x4185ea; continue; }
  pc = 0x418565; continue;
 }
 0x418565 => {
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x41856f); f_00430178(cpu); if cpu.returned_to != 0x41856f { return; }
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if cpu.zf { pc = 0x418579; continue; }
  pc = 0x418578; continue;
 }
 0x418578 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418579 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if cpu.zf { pc = 0x418583; continue; }
  pc = 0x418582; continue;
 }
 0x418582 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418583 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x41858d; continue; }
  pc = 0x41858c; continue;
 }
 0x41858c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41858d => {
  { cpu.pushad(); }
  cpu.push(0x418593); f_00403030(cpu); if cpu.returned_to != 0x418593 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x418599; continue; }
  pc = 0x418598; continue;
 }
 0x418598 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418599 => {
  cpu.push(0x41859e); f_004188ae(cpu); if cpu.returned_to != 0x41859e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7du32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d675au32, v); }
  { cpu.sub8(cpu.r8(0x4e1556u32), 0x0u8); }
  if !cpu.zf { pc = 0x4185ea; continue; }
  pc = 0x4185b7; continue;
 }
 0x4185b7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1556u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  { let v: u16 = cpu.r16(0x4d67bcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6854u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4d6854u32, v); }
  { let v: u16 = cpu.r16(0x4d67bau32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x4d6854u32), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x4185ea; continue; }
  pc = 0x4185e4; continue;
 }
 0x4185e4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6854u32, v); }
  pc = 0x4185ea; continue;
 }
 0x4185ea => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004185eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4185eb); }
 let mut pc: u32 = 0x4185eb;
 loop { match pc {
 0x4185eb => {
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x2u8); }
  if !cpu.zf { pc = 0x41867a; continue; }
  pc = 0x4185f8; continue;
 }
 0x4185f8 => {
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x418602); f_00430178(cpu); if cpu.returned_to != 0x418602 { return; }
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if cpu.zf { pc = 0x41860c; continue; }
  pc = 0x41860b; continue;
 }
 0x41860b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41860c => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if cpu.zf { pc = 0x418616; continue; }
  pc = 0x418615; continue;
 }
 0x418615 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418616 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x418620; continue; }
  pc = 0x41861f; continue;
 }
 0x41861f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418620 => {
  { cpu.pushad(); }
  cpu.push(0x418626); f_00403030(cpu); if cpu.returned_to != 0x418626 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x41862c; continue; }
  pc = 0x41862b; continue;
 }
 0x41862b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41862c => {
  cpu.push(0x418631); f_004188ae(cpu); if cpu.returned_to != 0x418631 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7du32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d675au32, v); }
  { cpu.sub8(cpu.r8(0x4e1556u32), 0x0u8); }
  if !cpu.zf { pc = 0x41867a; continue; }
  pc = 0x41864a; continue;
 }
 0x41864a => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1556u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  { let v: u16 = cpu.r16(0x4d67bcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6854u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d6854u32, v); }
  { let v: u16 = cpu.r16(0x4d67bau32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x4d6854u32), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41867a; continue; }
  pc = 0x418674; continue;
 }
 0x418674 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6854u32, v); }
  pc = 0x41867a; continue;
 }
 0x41867a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041867b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41867b); }
 let mut pc: u32 = 0x41867b;
 loop { match pc {
 0x41867b => {
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x2u8); }
  if !cpu.zf { pc = 0x41870d; continue; }
  pc = 0x418688; continue;
 }
 0x418688 => {
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x418692); f_00430178(cpu); if cpu.returned_to != 0x418692 { return; }
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if cpu.zf { pc = 0x41869c; continue; }
  pc = 0x41869b; continue;
 }
 0x41869b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41869c => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if cpu.zf { pc = 0x4186a6; continue; }
  pc = 0x4186a5; continue;
 }
 0x4186a5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4186a6 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x4186b0; continue; }
  pc = 0x4186af; continue;
 }
 0x4186af => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4186b0 => {
  { cpu.pushad(); }
  cpu.push(0x4186b6); f_00403030(cpu); if cpu.returned_to != 0x4186b6 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x4186bc; continue; }
  pc = 0x4186bb; continue;
 }
 0x4186bb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4186bc => {
  cpu.push(0x4186c1); f_004188ae(cpu); if cpu.returned_to != 0x4186c1 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7du32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d675au32, v); }
  { cpu.sub8(cpu.r8(0x4e1555u32), 0x0u8); }
  if !cpu.zf { pc = 0x41870d; continue; }
  pc = 0x4186da; continue;
 }
 0x4186da => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1555u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  { let v: u16 = cpu.r16(0x4d67bcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6852u32).wrapping_sub((cpu.eax as u16)); cpu.w16(0x4d6852u32, v); }
  { let v: u16 = cpu.r16(0x4d67bau32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x4d6852u32), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x41870d; continue; }
  pc = 0x418707; continue;
 }
 0x418707 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6852u32, v); }
  pc = 0x41870d; continue;
 }
 0x41870d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041870e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41870e); }
 let mut pc: u32 = 0x41870e;
 loop { match pc {
 0x41870e => {
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x2u8); }
  if !cpu.zf { pc = 0x41879d; continue; }
  pc = 0x41871b; continue;
 }
 0x41871b => {
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x418725); f_00430178(cpu); if cpu.returned_to != 0x418725 { return; }
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if cpu.zf { pc = 0x41872f; continue; }
  pc = 0x41872e; continue;
 }
 0x41872e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41872f => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if cpu.zf { pc = 0x418739; continue; }
  pc = 0x418738; continue;
 }
 0x418738 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418739 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x418743; continue; }
  pc = 0x418742; continue;
 }
 0x418742 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418743 => {
  { cpu.pushad(); }
  cpu.push(0x418749); f_00403030(cpu); if cpu.returned_to != 0x418749 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x41874f; continue; }
  pc = 0x41874e; continue;
 }
 0x41874e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41874f => {
  cpu.push(0x418754); f_004188ae(cpu); if cpu.returned_to != 0x418754 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7du32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d675au32, v); }
  { cpu.sub8(cpu.r8(0x4e1555u32), 0x0u8); }
  if !cpu.zf { pc = 0x41879d; continue; }
  pc = 0x41876d; continue;
 }
 0x41876d => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1555u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  { let v: u16 = cpu.r16(0x4d67bcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6852u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d6852u32, v); }
  { let v: u16 = cpu.r16(0x4d67bau32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x4d6852u32), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41879d; continue; }
  pc = 0x418797; continue;
 }
 0x418797 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6852u32, v); }
  pc = 0x41879d; continue;
 }
 0x41879d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041879e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41879e); }
 let mut pc: u32 = 0x41879e;
 loop { match pc {
 0x41879e => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x4187ec; continue; }
  pc = 0x4187a8; continue;
 }
 0x4187a8 => {
  cpu.push(0x4187ad); f_004184ca(cpu); if cpu.returned_to != 0x4187ad { return; }
  { let v: u8 = cpu.r8(0x5cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4187c7; continue; }
  pc = 0x4187b5; continue;
 }
 0x4187b5 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.eax)), 0xfdu8); cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  pc = 0x4187c7; continue;
 }
 0x4187c7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15c1u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d686cu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6758u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1567u32, v); }
  cpu.push(0x4187ec); f_004188ae(cpu); if cpu.returned_to != 0x4187ec { return; }
  pc = 0x4187ec; continue;
 }
 0x4187ec => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004187ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4187ed); }
 let mut pc: u32 = 0x4187ed;
 loop { match pc {
 0x4187ed => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x418822; continue; }
  pc = 0x4187f7; continue;
 }
 0x4187f7 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r16(0x4d6758u32); cpu.mul16(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5091d8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x418818; continue; }
  pc = 0x418811; continue;
 }
 0x418811 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x418859; continue; }
  pc = 0x418818; continue;
 }
 0x418818 => {
  cpu.push(0x41881d); f_0041879e(cpu); if cpu.returned_to != 0x41881d { return; }
  cpu.push(0x418822); f_0041b1ef(cpu); if cpu.returned_to != 0x418822 { return; }
  pc = 0x418822; continue;
 }
 0x418822 => {
  { cpu.sub16(cpu.r16(0x4d6888u32), 0x0u16); }
  if cpu.zf { pc = 0x418858; continue; }
  pc = 0x41882c; continue;
 }
 0x41882c => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r16(0x4d6888u32); cpu.mul16(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5091d8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x418858; continue; }
  pc = 0x418846; continue;
 }
 0x418846 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d688au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6888u32, v); }
  pc = 0x418858; continue;
 }
 0x418858 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418859 => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.sf { cpu.poll(); pc = 0x418818; continue; }
  pc = 0x418862; continue;
 }
 0x418862 => {
  { cpu.sub16(cpu.r16(0x2eu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.sf { cpu.poll(); pc = 0x418818; continue; }
  pc = 0x41886b; continue;
 }
 0x41886b => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.ebp)), 0x800u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x418818; continue; }
  pc = 0x418874; continue;
 }
 0x418874 => {
  { cpu.sub16(cpu.r16(0x2eu32.wrapping_add(cpu.ebp)), 0x800u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x418818; continue; }
  pc = 0x41887d; continue;
 }
 0x41887d => {
  cpu.poll(); pc = 0x418822; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041887f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41887f); }
 let mut pc: u32 = 0x41887f;
 loop { match pc {
 0x41887f => {
  { cpu.sub16(cpu.r16(0x4d6888u32), 0x0u16); }
  if cpu.zf { pc = 0x418893; continue; }
  pc = 0x418889; continue;
 }
 0x418889 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6888u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418893 => {
  { let v: u16 = cpu.r16(0x4d688au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x4188ad; continue; }
  pc = 0x41889e; continue;
 }
 0x41889e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6888u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d675au32, v); }
  pc = 0x4188ad; continue;
 }
 0x4188ad => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004188ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4188ae); }
 let mut pc: u32 = 0x4188ae;
 loop { match pc {
 0x4188ae => {
  { cpu.sub16(cpu.r16(0x4d6888u32), 0x0u16); }
  if cpu.zf { pc = 0x4188bd; continue; }
  pc = 0x4188b8; continue;
 }
 0x4188b8 => {
  cpu.push(0x4188bd); f_0041887f(cpu); if cpu.returned_to != 0x4188bd { return; }
  pc = 0x4188bd; continue;
 }
 0x4188bd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004188be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4188be); }
 let mut pc: u32 = 0x4188be;
 loop { match pc {
 0x4188be => {
  { cpu.sub8(cpu.r8(0x4e15c1u32), 0x2u8); }
  if cpu.zf { pc = 0x4188d0; continue; }
  pc = 0x4188c7; continue;
 }
 0x4188c7 => {
  { cpu.sub8(cpu.r8(0x4e15c1u32), 0x0u8); }
  if !cpu.zf { pc = 0x4188de; continue; }
  pc = 0x4188d0; continue;
 }
 0x4188d0 => {
  { cpu.sub16(cpu.r16(0x4d6888u32), 0x0u16); }
  if !cpu.zf { pc = 0x418a08; continue; }
  pc = 0x4188de; continue;
 }
 0x4188de => {
  { cpu.sub16(cpu.r16(0x4d675au32), 0xffffu16); }
  if !cpu.zf { pc = 0x418a07; continue; }
  pc = 0x4188ec; continue;
 }
 0x4188ec => {
  { let v: u16 = cpu.r16(0x4d678eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6ba4u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r16(0x4d6852u32) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x4d6854u32) as i16 as i32 as u32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x58882eu32), 0x0u8); }
  if !cpu.zf { pc = 0x418921; continue; }
  pc = 0x418911; continue;
 }
 0x418911 => {
  { let v: u32 = cpu.r8(0x5928fcu32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x5928fcu32.wrapping_add(cpu.edx)) as i8 as i32 as u32; cpu.edx = v; }
  pc = 0x41893d; continue;
 }
 0x418921 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x5a2361u32) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x592928u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(1))) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edx.wrapping_mul(1))) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x41893d; continue;
 }
 0x41893d => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x41895e; continue; }
  pc = 0x418942; continue;
 }
 0x418942 => {
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x418950; continue; }
  pc = 0x41894d; continue;
 }
 0x41894d => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x418950; continue;
 }
 0x418950 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x418958; continue; }
  pc = 0x418955; continue;
 }
 0x418955 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x418958; continue;
 }
 0x418958 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6a00u32, v); }
  pc = 0x41895e; continue;
 }
 0x41895e => {
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x41898d; continue; }
  pc = 0x418963; continue;
 }
 0x418963 => {
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x418971; continue; }
  pc = 0x41896e; continue;
 }
 0x41896e => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x418971; continue;
 }
 0x418971 => {
  { let v: u16 = cpu.r16(0x4d6790u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub(cpu.r16(0x4d6ba8u32)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebp as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x418987; continue; }
  pc = 0x418984; continue;
 }
 0x418984 => {
  { let v: u16 = (cpu.ebp as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x418987; continue;
 }
 0x418987 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6a04u32, v); }
  pc = 0x41898d; continue;
 }
 0x41898d => {
  { let v: u16 = cpu.r16(0x4d6852u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6854u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67b6u32), 0x0u16); }
  if cpu.zf { pc = 0x4189af; continue; }
  pc = 0x4189aa; continue;
 }
 0x4189aa => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x4189af; continue;
 }
 0x4189af => {
  { cpu.sub8(cpu.r8(0x4e1555u32), 0x0u8); }
  if !cpu.zf { pc = 0x4189d4; continue; }
  pc = 0x4189b8; continue;
 }
 0x4189b8 => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x4189d4; continue; }
  pc = 0x4189bd; continue;
 }
 0x4189bd => {
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x58882eu32), 0x0u8); }
  if cpu.zf { pc = 0x4189cc; continue; }
  pc = 0x4189c9; continue;
 }
 0x4189c9 => {
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x4189cc; continue;
 }
 0x4189cc => {
  { cpu.sub16((cpu.ecx as u16), 0xffffu16); }
  if !cpu.zf { pc = 0x4189d4; continue; }
  pc = 0x4189d2; continue;
 }
 0x4189d2 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x4189d4; continue;
 }
 0x4189d4 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6852u32, v); }
  { cpu.sub8(cpu.r8(0x4e1556u32), 0x0u8); }
  if !cpu.zf { pc = 0x418a00; continue; }
  pc = 0x4189e4; continue;
 }
 0x4189e4 => {
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x418a00; continue; }
  pc = 0x4189e9; continue;
 }
 0x4189e9 => {
  { let v: u16 = cpu.sar16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x58882eu32), 0x0u8); }
  if cpu.zf { pc = 0x4189f8; continue; }
  pc = 0x4189f5; continue;
 }
 0x4189f5 => {
  { let v: u16 = cpu.sar16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x4189f8; continue;
 }
 0x4189f8 => {
  { cpu.sub16((cpu.edx as u16), 0xffffu16); }
  if !cpu.zf { pc = 0x418a00; continue; }
  pc = 0x4189fe; continue;
 }
 0x4189fe => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x418a00; continue;
 }
 0x418a00 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6854u32, v); }
  pc = 0x418a07; continue;
 }
 0x418a07 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x418a08 => {
  { cpu.sub8(cpu.r8(0x4e1568u32), 0x0u8); }
  if !cpu.zf { pc = 0x418b08; continue; }
  pc = 0x418a15; continue;
 }
 0x418a15 => {
  { cpu.sub8(cpu.r8(0x4b929eu32), 0x0u8); }
  if cpu.zf { pc = 0x418a2b; continue; }
  pc = 0x418a1e; continue;
 }
 0x418a1e => {
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x1u8); }
  if !cpu.zf { pc = 0x418b08; continue; }
  pc = 0x418a2b; continue;
 }
 0x418a2b => {
  { let v: u16 = cpu.r16(0x4d6888u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xb8u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x509290u32); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x88u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x54u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x418a66; continue; }
  pc = 0x418a5e; continue;
 }
 0x418a5e => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x418a66; continue;
 }
 0x418a66 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x4d6a00u32), 0x0u16); }
  if cpu.zf { pc = 0x418a8a; continue; }
  pc = 0x418a84; continue;
 }
 0x418a84 => {
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if (cpu.sf != cpu.of) { pc = 0x418ad0; continue; }
  pc = 0x418a8a; continue;
 }
 0x418a8a => {
  { let v: u16 = cpu.r16(0x4d678eu32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub(0x120u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x4d6a00u32)); }
  if cpu.zf { pc = 0x418aa5; continue; }
  pc = 0x418a9f; continue;
 }
 0x418a9f => {
  { cpu.sub16((cpu.eax as u16), 0x20u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x418ad0; continue; }
  pc = 0x418aa5; continue;
 }
 0x418aa5 => {
  { cpu.sub16(cpu.r16(0x4d6a04u32), 0x0u16); }
  if cpu.zf { pc = 0x418ab5; continue; }
  pc = 0x418aaf; continue;
 }
 0x418aaf => {
  { cpu.sub16((cpu.ebx as u16), 0xffe0u16); }
  if (cpu.sf != cpu.of) { pc = 0x418ad0; continue; }
  pc = 0x418ab5; continue;
 }
 0x418ab5 => {
  { let v: u16 = cpu.r16(0x4d6790u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub(0xb8u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x4d6a04u32)); }
  if cpu.zf { pc = 0x418b08; continue; }
  pc = 0x418aca; continue;
 }
 0x418aca => {
  { cpu.sub16((cpu.ebx as u16), 0x20u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x418b08; continue; }
  pc = 0x418ad0; continue;
 }
 0x418ad0 => {
  { cpu.sub16((cpu.ecx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x418ad9; continue; }
  pc = 0x418ad6; continue;
 }
 0x418ad6 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x418ad9; continue;
 }
 0x418ad9 => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x418ae2; continue; }
  pc = 0x418adf; continue;
 }
 0x418adf => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x418ae2; continue;
 }
 0x418ae2 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d675au32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d675cu32, v); }
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d675eu32, v); }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6760u32, v); }
  pc = 0x418b08; continue;
 }
 0x418b08 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00418ba3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x418ba3); }
 let mut pc: u32 = 0x418ba3;
 loop { match pc {
 0x418ba3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x418bca; continue; }
  pc = 0x418bbd; continue;
 }
 0x418bbd => {
  { let v: u16 = cpu.r16(0x4d67c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67cau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x418bca; continue;
 }
 0x418bca => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00418bd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x418bd9); }
 let mut pc: u32 = 0x418bd9;
 loop { match pc {
 0x418bd9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x5928b5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5928b7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00418beb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x418beb); }
 let mut pc: u32 = 0x418beb;
 loop { match pc {
 0x418beb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x418c12; continue; }
  pc = 0x418c05; continue;
 }
 0x418c05 => {
  { let v: u16 = cpu.r16(0x4d67c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67cau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x418c12; continue;
 }
 0x418c12 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00418c13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x418c13); }
 let mut pc: u32 = 0x418c13;
 loop { match pc {
 0x418c13 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x5928b1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5928b3u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00418c25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x418c25); }
 let mut pc: u32 = 0x418c25;
 loop { match pc {
 0x418c25 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x418c37; continue; }
  pc = 0x418c2e; continue;
 }
 0x418c2e => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x418c6c; continue; }
  pc = 0x418c37; continue;
 }
 0x418c37 => {
  { let v: u8 = cpu.r8(0x58c838u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c839u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x58c83eu32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  { let v: u32 = cpu.r32(0x58c846u32); cpu.eax = v; }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c839u32, v); }
  cpu.push(0x418c60); f_00418c6d(cpu); if cpu.returned_to != 0x418c60 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x58c839u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  pc = 0x418c6c; continue;
 }
 0x418c6c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004197c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4197c0); }
 let mut pc: u32 = 0x4197c0;
 loop { match pc {
 0x4197c0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d306u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x419952; continue; }
  pc = 0x4197d4; continue;
 }
 0x4197d4 => {
  { cpu.sub8(cpu.r8(0x5928aeu32), 0x0u8); }
  if !cpu.zf { pc = 0x419952; continue; }
  pc = 0x4197e1; continue;
 }
 0x4197e1 => {
  { cpu.sub32(cpu.r32(0x5b358cu32), 0x0u32); }
  if cpu.zf { pc = 0x419952; continue; }
  pc = 0x4197ee; continue;
 }
 0x4197ee => {
  cpu.push(0x4197f3); f_00418c13(cpu); if cpu.returned_to != 0x4197f3 { return; }
  { let v: u16 = 0x40u16; cpu.w16(0x5b385fu32, v); }
  { let v: u16 = 0xe8u16; cpu.w16(0x5b3861u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x5b3863u32, v); }
  { let v: u16 = 0x15u16; cpu.w16(0x5b3865u32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x419844; continue; }
  pc = 0x419820; continue;
 }
 0x419820 => {
  { let v: u16 = 0xe0u16; cpu.w16(0x5b385fu32, v); }
  { let v: u16 = 0x188u16; cpu.w16(0x5b3861u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x5b3863u32, v); }
  { let v: u16 = 0x15u16; cpu.w16(0x5b3865u32, v); }
  pc = 0x419844; continue;
 }
 0x419844 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x5b385fu32)); }
  if (cpu.sf != cpu.of) { pc = 0x419952; continue; }
  pc = 0x419851; continue;
 }
 0x419851 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x5b3861u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x419952; continue; }
  pc = 0x41985e; continue;
 }
 0x41985e => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x5b3863u32)); }
  if (cpu.sf != cpu.of) { pc = 0x419952; continue; }
  pc = 0x41986b; continue;
 }
 0x41986b => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x5b3865u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x419952; continue; }
  pc = 0x419878; continue;
 }
 0x419878 => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x419887; continue; }
  pc = 0x419882; continue;
 }
 0x419882 => {
  cpu.push(0x419887); f_0041879e(cpu); if cpu.returned_to != 0x419887 { return; }
  pc = 0x419887; continue;
 }
 0x419887 => {
  { cpu.sub32(cpu.r32(0x5b3594u32), 0x4d2u32); }
  if !cpu.zf { pc = 0x4198a7; continue; }
  pc = 0x419893; continue;
 }
 0x419893 => {
  cpu.push(0x419898); f_00457106(cpu); if cpu.returned_to != 0x419898 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5b358cu32, v); }
  pc = 0x419950; continue;
 }
 0x4198a7 => {
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x56c6f6u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x0u32); }
  if cpu.zf { pc = 0x4198e0; continue; }
  pc = 0x4198b4; continue;
 }
 0x4198b4 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4198e0; continue; }
  pc = 0x4198bb; continue;
 }
 0x4198bb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c42cu32, v); }
  cpu.push(0x4198ce); f_0041a8a2(cpu); if cpu.returned_to != 0x4198ce { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c42cu32, v); }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x58c839u32), 0x0u8); }
  if !cpu.zf { pc = 0x419950; continue; }
  pc = 0x4198e0; continue;
 }
 0x4198e0 => {
  { let v: u32 = cpu.r32(0x5b3594u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x4198ed); f_00419d49(cpu); if cpu.returned_to != 0x4198ed { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5b358cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x56c6f6u32, v); }
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x419950; continue; }
  pc = 0x419908; continue;
 }
 0x419908 => {
  cpu.push(0x41990d); f_0044d89b(cpu); if cpu.returned_to != 0x41990d { return; }
  cpu.push(0x419912); f_00419954(cpu); if cpu.returned_to != 0x419912 { return; }
  pc = 0x419950; continue;
 }
 0x419950 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x419952 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00419954(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x419954); }
 let mut pc: u32 = 0x419954;
 loop { match pc {
 0x419954 => {
  { let v: u16 = 0x80u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x5cu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x419974; continue; }
  pc = 0x419965; continue;
 }
 0x419965 => {
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x10u32); cpu.ebx = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x419974; continue;
 }
 0x419974 => {
  cpu.push(0x419979); f_0041c26e(cpu); if cpu.returned_to != 0x419979 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041997a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41997a); }
 let mut pc: u32 = 0x41997a;
 loop { match pc {
 0x41997a => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c439u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c43au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00419989(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x419989); }
 let mut pc: u32 = 0x419989;
 loop { match pc {
 0x419989 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x419b69; continue; }
  pc = 0x419999; continue;
 }
 0x419999 => {
  { cpu.sub8(cpu.r8(0x58c43au32), 0x0u8); }
  if !cpu.zf { pc = 0x4199b8; continue; }
  pc = 0x4199a2; continue;
 }
 0x4199a2 => {
  { cpu.sub8(cpu.r8(0x58c43fu32), 0x0u8); }
  if !cpu.zf { pc = 0x4199b8; continue; }
  pc = 0x4199ab; continue;
 }
 0x4199ab => {
  cpu.push(0x4199b0); f_00418237(cpu); if cpu.returned_to != 0x4199b0 { return; }
  if cpu.cf { pc = 0x4199b8; continue; }
  pc = 0x4199b2; continue;
 }
 0x4199b2 => {
  cpu.push(0x4199b7); f_004181a0(cpu); if cpu.returned_to != 0x4199b7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4199b8 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x419b5a; continue; }
  pc = 0x4199c6; continue;
 }
 0x4199c6 => {
  { cpu.sub8(cpu.r8(0x58c43fu32), 0x0u8); }
  if !cpu.zf { pc = 0x419b5a; continue; }
  pc = 0x4199d3; continue;
 }
 0x4199d3 => {
  { cpu.sub8(cpu.r8(0x58c43au32), 0x0u8); }
  if !cpu.zf { pc = 0x419b6a; continue; }
  pc = 0x4199e0; continue;
 }
 0x4199e0 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x419b25; continue; }
  pc = 0x4199ed; continue;
 }
 0x4199ed => {
  { cpu.sub32(cpu.r32(0x5b3588u32), 0x0u32); }
  if !cpu.zf { pc = 0x419b25; continue; }
  pc = 0x4199fa; continue;
 }
 0x4199fa => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if !cpu.zf { pc = 0x419b13; continue; }
  pc = 0x419a08; continue;
 }
 0x419a08 => {
  { cpu.pushad(); }
  cpu.push(0x419a0e); f_0044e68e(cpu); if cpu.returned_to != 0x419a0e { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x419b25; continue; }
  pc = 0x419a15; continue;
 }
 0x419a15 => {
  { cpu.sub32(cpu.r32(0x5b358cu32), 0x0u32); }
  if cpu.zf { pc = 0x419a9d; continue; }
  pc = 0x419a1e; continue;
 }
 0x419a1e => {
  cpu.push(0x419a23); f_00418beb(cpu); if cpu.returned_to != 0x419a23 { return; }
  { let v: u16 = 0x40u16; cpu.w16(0x5b385fu32, v); }
  { let v: u16 = 0xe8u16; cpu.w16(0x5b3861u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x5b3863u32, v); }
  { let v: u16 = 0x15u16; cpu.w16(0x5b3865u32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x419a74; continue; }
  pc = 0x419a50; continue;
 }
 0x419a50 => {
  { let v: u16 = 0xe0u16; cpu.w16(0x5b385fu32, v); }
  { let v: u16 = 0x188u16; cpu.w16(0x5b3861u32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x5b3863u32, v); }
  { let v: u16 = 0x15u16; cpu.w16(0x5b3865u32, v); }
  pc = 0x419a74; continue;
 }
 0x419a74 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x5b385fu32)); }
  if (cpu.sf != cpu.of) { pc = 0x419a9d; continue; }
  pc = 0x419a7d; continue;
 }
 0x419a7d => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x5b3861u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x419a9d; continue; }
  pc = 0x419a86; continue;
 }
 0x419a86 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x5b3863u32)); }
  if (cpu.sf != cpu.of) { pc = 0x419a9d; continue; }
  pc = 0x419a8f; continue;
 }
 0x419a8f => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x5b3865u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x419a9d; continue; }
  pc = 0x419a98; continue;
 }
 0x419a98 => {
  pc = 0x419b25; continue;
 }
 0x419a9d => {
  { cpu.pushad(); }
  cpu.push(0x419aa3); f_00418ba3(cpu); if cpu.returned_to != 0x419aa3 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d24u32, v); }
  cpu.push(0x419aaf); f_0047113a(cpu); if cpu.returned_to != 0x419aaf { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d24u32, v); }
  { cpu.popad(); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  if !cpu.cf { pc = 0x419ae4; continue; }
  pc = 0x419abb; continue;
 }
 0x419abb => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x419ae4; continue; }
  pc = 0x419ac8; continue;
 }
 0x419ac8 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x419ae4; continue; }
  pc = 0x419acf; continue;
 }
 0x419acf => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if cpu.zf { pc = 0x419ae0; continue; }
  pc = 0x419ade; continue;
 }
 0x419ade => {
  pc = 0x419ae4; continue;
 }
 0x419ae0 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x419b25; continue;
 }
 0x419ae4 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x58c889u32, v); }
  cpu.push(0x419af3); f_00418ba3(cpu); if cpu.returned_to != 0x419af3 { return; }
  cpu.push(0x419af8); f_0046d5b9(cpu); if cpu.returned_to != 0x419af8 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c889u32, v); }
  if cpu.cf { pc = 0x419b12; continue; }
  pc = 0x419b01; continue;
 }
 0x419b01 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x419b12; continue; }
  pc = 0x419b0f; continue;
 }
 0x419b0f => {
  { cpu.popad(); }
  pc = 0x419b25; continue;
 }
 0x419b12 => {
  { cpu.popad(); }
  pc = 0x419b13; continue;
 }
 0x419b13 => {
  { let v: u16 = cpu.r16(0x4d6758u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x419b1f); f_0042caaa(cpu); if cpu.returned_to != 0x419b1f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x419b5a; continue; }
  pc = 0x419b25; continue;
 }
 0x419b25 => {
  { let v: u8 = cpu.r8(0x58c838u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x58c839u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x58c842u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  { let v: u32 = cpu.r32(0x58c84au32); cpu.eax = v; }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c839u32, v); }
  cpu.push(0x419b4e); f_00418c6d(cpu); if cpu.returned_to != 0x419b4e { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x58c839u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c838u32, v); }
  pc = 0x419b5a; continue;
 }
 0x419b5a => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c439u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c43fu32, v); }
  { cpu.popad(); }
  pc = 0x419b69; continue;
 }
 0x419b69 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x419b6a => {
  cpu.push(0x419b6f); f_0041adda(cpu); if cpu.returned_to != 0x419b6f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1568u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d675au32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00419b81(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x419b81); } r_00419b81(cpu, 0x419b81); }
pub fn f_00419c4d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x419c4d); } r_00419b81(cpu, 0x419c4d); }
fn r_00419b81(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x419b81 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x419c7e; continue; }
  pc = 0x419b91; continue;
 }
 0x419b91 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x419c7d; continue; }
  pc = 0x419b9f; continue;
 }
 0x419b9f => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x419c7d; continue; }
  pc = 0x419bac; continue;
 }
 0x419bac => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if cpu.zf { pc = 0x419bbf; continue; }
  pc = 0x419bb5; continue;
 }
 0x419bb5 => {
  cpu.push(0x419bba); f_0044d6ec(cpu); if cpu.returned_to != 0x419bba { return; }
  pc = 0x419c7d; continue;
 }
 0x419bbf => {
  { cpu.sub8(cpu.r8(0x58c634u32), 0x0u8); }
  if cpu.zf { pc = 0x419bd4; continue; }
  pc = 0x419bc8; continue;
 }
 0x419bc8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  pc = 0x419c7d; continue;
 }
 0x419bd4 => {
  { cpu.sub32(cpu.r32(0x5b3588u32), 0x0u32); }
  if cpu.zf { pc = 0x419be2; continue; }
  pc = 0x419bdd; continue;
 }
 0x419bdd => {
  pc = 0x419c7d; continue;
 }
 0x419be2 => {
  { cpu.sub8(cpu.r8(0x58c434u32), 0x0u8); }
  if !cpu.zf { pc = 0x419c38; continue; }
  pc = 0x419beb; continue;
 }
 0x419beb => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c434u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x419c20; continue; }
  pc = 0x419c13; continue;
 }
 0x419c13 => {
  { let v: u16 = cpu.r16(0x4d67c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67cau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x419c20; continue;
 }
 0x419c20 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c70u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6c74u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c78u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6c7cu32, v); }
  pc = 0x419c7d; continue;
 }
 0x419c38 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  { let v: u32 = cpu.r32(0x4d6c70u32); cpu.eax = v; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6752u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x419c50; continue; }
  pc = 0x419c4d; continue;
 }
 0x419c4d => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x419c50; continue;
 }
 0x419c50 => {
  { cpu.sub16((cpu.eax as u16), 0x10u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x419c6f; continue; }
  pc = 0x419c56; continue;
 }
 0x419c56 => {
  { let v: u32 = cpu.r32(0x4d6c74u32); cpu.eax = v; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6754u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x419c67; continue; }
  pc = 0x419c64; continue;
 }
 0x419c64 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x419c67; continue;
 }
 0x419c67 => {
  { cpu.sub16((cpu.eax as u16), 0x10u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x419c6f; continue; }
  pc = 0x419c6d; continue;
 }
 0x419c6d => {
  pc = 0x419c7d; continue;
 }
 0x419c6f => {
  { let v: u8 = cpu.and8(cpu.r8(0x58c432u32), 0xfeu8); cpu.w8(0x58c432u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1fcbu32, v); }
  pc = 0x419c7d; continue;
 }
 0x419c7d => {
  { cpu.popad(); }
  pc = 0x419c7e; continue;
 }
 0x419c7e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00419c7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x419c7f); }
 let mut pc: u32 = 0x419c7f;
 loop { match pc {
 0x419c7f => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c498u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00419c87(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x419c87); } r_00419c87(cpu, 0x419c87); }
pub fn f_00419ca5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x419ca5); } r_00419c87(cpu, 0x419ca5); }
fn r_00419c87(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x419c87 => {
  { cpu.sub8(cpu.r8(0x58c498u32), 0x0u8); }
  if cpu.zf { pc = 0x419c91; continue; }
  pc = 0x419c90; continue;
 }
 0x419c90 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x419c91 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c87eu32, v); }
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if cpu.zf { pc = 0x419ca2; continue; }
  pc = 0x419ca1; continue;
 }
 0x419ca1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x419ca2 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if cpu.zf { pc = 0x419cac; continue; }
  pc = 0x419cab; continue;
 }
 0x419ca5 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.ebx as u8)); }
  { let v: u8 = cpu.r8(0xffffffc3u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0xffffffc3u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x419cac; continue;
 }
 0x419cab => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x419cac => {
  { cpu.pushad(); }
  cpu.push(0x419cb2); f_00403030(cpu); if cpu.returned_to != 0x419cb2 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x419cb8; continue; }
  pc = 0x419cb7; continue;
 }
 0x419cb7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x419cb8 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x419ccd; continue; }
  pc = 0x419cc1; continue;
 }
 0x419cc1 => {
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if (cpu.sf != cpu.of) { pc = 0x419cef; continue; }
  pc = 0x419ccb; continue;
 }
 0x419ccb => {
  pc = 0x419cd6; continue;
 }
 0x419ccd => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x419cef; continue; }
  pc = 0x419cd6; continue;
 }
 0x419cd6 => {
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x419ce8; continue; }
  pc = 0x419cdf; continue;
 }
 0x419cdf => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c87eu32, v); }
  pc = 0x419cef; continue;
 }
 0x419ce8 => {
  { let v: u8 = 0x2u8; cpu.w8(0x58c87eu32, v); }
  pc = 0x419cef; continue;
 }
 0x419cef => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00419cf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x419cf0); }
 let mut pc: u32 = 0x419cf0;
 loop { match pc {
 0x419cf0 => {
  { let v: u8 = 0xdu8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xac6u32; cpu.eax = v; }
  cpu.push(0x419d08); f_0041b7b0(cpu); if cpu.returned_to != 0x419d08 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x419d15); f_00413337(cpu); if cpu.returned_to != 0x419d15 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { let v: u8 = cpu.r8(0x79u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x8bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x419d31); f_004165fb(cpu); if cpu.returned_to != 0x419d31 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x419d48; continue; }
  pc = 0x419d3a; continue;
 }
 0x419d3a => {
  cpu.push(0x419d3f); f_0041879e(cpu); if cpu.returned_to != 0x419d3f { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d688au32, v); }
  pc = 0x419d48; continue;
 }
 0x419d48 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00419d49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x419d49); }
 let mut pc: u32 = 0x419d49;
 loop { match pc {
 0x419d49 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x60u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x419d7c; continue; }
  pc = 0x419d64; continue;
 }
 0x419d64 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.r32(0x4d6bccu32), 0x190u32); }
  if cpu.zf { pc = 0x419d7c; continue; }
  pc = 0x419d78; continue;
 }
 0x419d78 => {
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x28u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x419d7c; continue;
 }
 0x419d7c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d675au32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d675cu32, v); }
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d675eu32, v); }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6760u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00419ea8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x419ea8); }
 let mut pc: u32 = 0x419ea8;
 loop { match pc {
 0x419ea8 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x419ebd; continue; }
  pc = 0x419eb4; continue;
 }
 0x419eb4 => {
  { cpu.sub32(cpu.r32(0x5b3588u32), 0x0u32); }
  if cpu.zf { pc = 0x419f06; continue; }
  pc = 0x419ebd; continue;
 }
 0x419ebd => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x419ee7; continue; }
  pc = 0x419ed9; continue;
 }
 0x419ed9 => {
  { let v: u16 = cpu.r16(0x4d67c8u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67cau32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x419ee7; continue;
 }
 0x419ee7 => {
  { let v: u32 = cpu.r32(0x5b2dbcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b2dc0u32); cpu.ebx = v; }
  { let v: u32 = 0x70u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x4eu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x419efd); f_004714a5(cpu); if cpu.returned_to != 0x419efd { return; }
  if !cpu.cf { pc = 0x419f07; continue; }
  pc = 0x419eff; continue;
 }
 0x419eff => {
  cpu.push(0x419f04); f_0044d5c5(cpu); if cpu.returned_to != 0x419f04 { return; }
  { cpu.cf = true; }
  { cpu.popad(); }
  pc = 0x419f06; continue;
 }
 0x419f06 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x419f07 => {
  { cpu.popad(); }
  f_0044d990(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00419f0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x419f0d); }
 let mut pc: u32 = 0x419f0d;
 loop { match pc {
 0x419f0d => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  cpu.push(0x419f19); f_00418beb(cpu); if cpu.returned_to != 0x419f19 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c78u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6c7cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00419f25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x419f25); }
 let mut pc: u32 = 0x419f25;
 loop { match pc {
 0x419f25 => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if cpu.zf { pc = 0x419f95; continue; }
  pc = 0x419f2e; continue;
 }
 0x419f2e => {
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x1u8); }
  if !cpu.zf { pc = 0x419f60; continue; }
  pc = 0x419f37; continue;
 }
 0x419f37 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fcbu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c434u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c439u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c43au32, v); }
  cpu.push(0x419f58); f_00419f96(cpu); if cpu.returned_to != 0x419f58 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x58c63au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x419f60 => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x419f67); f_00418beb(cpu); if cpu.returned_to != 0x419f67 { return; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6ba4u32)); }
  if (cpu.sf != cpu.of) { pc = 0x419f76; continue; }
  pc = 0x419f6f; continue;
 }
 0x419f6f => {
  { let v: u32 = cpu.r32(0x4d6ba4u32); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x419f76; continue;
 }
 0x419f76 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4d6ba8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x419f86; continue; }
  pc = 0x419f7e; continue;
 }
 0x419f7e => {
  { let v: u32 = cpu.r32(0x4d6ba8u32); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x419f86; continue;
 }
 0x419f86 => {
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x419f95; continue; }
  pc = 0x419f8a; continue;
 }
 0x419f8a => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c78u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6c7cu32, v); }
  pc = 0x419f95; continue;
 }
 0x419f95 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00419f96(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x419f96); } r_00419f96(cpu, 0x419f96); }
pub fn f_0041a000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41a000); } r_00419f96(cpu, 0x41a000); }
fn r_00419f96(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x419f96 => {
  cpu.push(0x419f9b); f_0041a249(cpu); if cpu.returned_to != 0x419f9b { return; }
  { cpu.sub8(cpu.r8(0x5928b0u32), 0x0u8); }
  if cpu.zf { pc = 0x41a021; continue; }
  pc = 0x419fa4; continue;
 }
 0x419fa4 => {
  cpu.push(0x419fa9); f_0041a3e9(cpu); if cpu.returned_to != 0x419fa9 { return; }
  { let v: u8 = 0xfeu8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u16 = cpu.r16(0x59285eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6aecu32, v); }
  { let v: u16 = cpu.r16(0x592860u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = cpu.r16(0x592862u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = cpu.r16(0x592864u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b2u32, v); }
  { let v: u32 = cpu.r32(0x592866u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d68b4u32, v); }
  { let v: u32 = cpu.r32(0x59286au32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d68b8u32, v); }
  { let v: u32 = cpu.r32(0x59286eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d68bcu32, v); }
  { let v: u32 = cpu.r32(0x592872u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d68c0u32, v); }
  { let v: u32 = cpu.r32(0x592876u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d68c4u32, v); }
  { let v: u32 = cpu.r32(0x59287au32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d68c8u32, v); }
  cpu.push(0x41a021); f_00413337(cpu); if cpu.returned_to != 0x41a021 { return; }
  pc = 0x41a021; continue;
 }
 0x41a000 => {
  { let v: u8 = cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), (cpu.ebx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d68c0u32, v); }
  { let v: u32 = cpu.r32(0x592876u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d68c4u32, v); }
  { let v: u32 = cpu.r32(0x59287au32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d68c8u32, v); }
  cpu.push(0x41a021); f_00413337(cpu); if cpu.returned_to != 0x41a021 { return; }
  pc = 0x41a021; continue;
 }
 0x41a021 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a022(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41a022); }
 let mut pc: u32 = 0x41a022;
 loop { match pc {
 0x41a022 => {
  { let v: u32 = 0x59249cu32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x5928b0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x41a0e8; continue; }
  pc = 0x41a035; continue;
 }
 0x41a035 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = 0x48u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x52bbf0u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  cpu.push(0x41a047); f_0044c7f9(cpu); if cpu.returned_to != 0x41a047 { return; }
  if cpu.cf { pc = 0x41a04e; continue; }
  pc = 0x41a049; continue;
 }
 0x41a049 => {
  cpu.push(0x41a04e); f_0044c896(cpu); if cpu.returned_to != 0x41a04e { return; }
  pc = 0x41a04e; continue;
 }
 0x41a04e => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x41a035; continue; }
  pc = 0x41a057; continue;
 }
 0x41a057 => {
  { let v: u8 = 0xffu8; cpu.w8(0x4e1fbfu32, v); }
  { let v: u32 = 0x59249cu32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x5928b0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x41a069; continue;
 }
 0x41a069 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5928d3u32, v); }
  cpu.push(0x41a08d); f_0044c69e(cpu); if cpu.returned_to != 0x41a08d { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5928d3u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x41a069; continue; }
  pc = 0x41a09d; continue;
 }
 0x41a09d => {
  { cpu.sub8(cpu.r8(0x54acd1u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x41a0e8; continue; }
  pc = 0x41a0a6; continue;
 }
 0x41a0a6 => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x41a0e8; continue; }
  pc = 0x41a0bb; continue;
 }
 0x41a0bb => {
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x41a0cd); f_0041a8a2(cpu); if cpu.returned_to != 0x41a0cd { return; }
  { cpu.sub8(cpu.r8(0x58c838u32), 0x0u8); }
  if !cpu.zf { pc = 0x41a0df; continue; }
  pc = 0x41a0d6; continue;
 }
 0x41a0d6 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x41a0e8; continue; }
  pc = 0x41a0df; continue;
 }
 0x41a0df => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6888u32, v); }
  pc = 0x41a0e8; continue;
 }
 0x41a0e8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a0e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41a0e9); }
 let mut pc: u32 = 0x41a0e9;
 loop { match pc {
 0x41a0e9 => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if cpu.zf { pc = 0x41a17b; continue; }
  pc = 0x41a0f6; continue;
 }
 0x41a0f6 => {
  { let v: u32 = cpu.r8(0x4e1559u32) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x15u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4d6c70u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6c74u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4d6c78u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6c7cu32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41a11f; continue; }
  pc = 0x41a11e; continue;
 }
 0x41a11e => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x41a11f; continue;
 }
 0x41a11f => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41a125; continue; }
  pc = 0x41a123; continue;
 }
 0x41a123 => {
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x41a125; continue;
 }
 0x41a125 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x4d6a00u32)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4d6a04u32)); cpu.edx = v; }
  { let v: u32 = 0x54a7d0u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x24u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x28u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x2cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x41a176); f_0043e337(cpu); if cpu.returned_to != 0x41a176 { return; }
  cpu.push(0x41a17b); f_0041a17c(cpu); if cpu.returned_to != 0x41a17b { return; }
  pc = 0x41a17b; continue;
 }
 0x41a17b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a17c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41a17c); }
 let mut pc: u32 = 0x41a17c;
 loop { match pc {
 0x41a17c => {
  cpu.push(0x41a181); f_0041a249(cpu); if cpu.returned_to != 0x41a181 { return; }
  { let v: u32 = 0x59249cu32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x5928b0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x41a1b0; continue; }
  pc = 0x41a190; continue;
 }
 0x41a190 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x41a1a7); f_0042024a(cpu); if cpu.returned_to != 0x41a1a7 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x41a190; continue; }
  pc = 0x41a1b0; continue;
 }
 0x41a1b0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a1b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41a1b1); }
 let mut pc: u32 = 0x41a1b1;
 loop { match pc {
 0x41a1b1 => {
  { cpu.sub8(cpu.r8(0x581b02u32), 0x0u8); }
  if cpu.zf { pc = 0x41a248; continue; }
  pc = 0x41a1be; continue;
 }
 0x41a1be => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if !cpu.zf { pc = 0x41a1dd; continue; }
  pc = 0x41a1c7; continue;
 }
 0x41a1c7 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = 0x10eu32; cpu.ebx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0xb6u32; cpu.edx = v; }
  pc = 0x41a213; continue;
 }
 0x41a1dd => {
  { cpu.sub32(cpu.r32(0x4d6bccu32), 0x1e0u32); }
  if cpu.zf { pc = 0x41a1ff; continue; }
  pc = 0x41a1e9; continue;
 }
 0x41a1e9 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = 0x21cu32; cpu.ebx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0x16cu32; cpu.edx = v; }
  pc = 0x41a213; continue;
 }
 0x41a1ff => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = 0x21cu32; cpu.ebx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0x1bcu32; cpu.edx = v; }
  pc = 0x41a213; continue;
 }
 0x41a213 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c70u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6c78u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4d6c74u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4d6c7cu32, v); }
  cpu.push(0x41a22f); f_00419f96(cpu); if cpu.returned_to != 0x41a22f { return; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x41a241; continue; }
  pc = 0x41a238; continue;
 }
 0x41a238 => {
  { cpu.sub8(cpu.r8(0x581b03u32), 0x0u8); }
  if !cpu.zf { pc = 0x41a248; continue; }
  pc = 0x41a241; continue;
 }
 0x41a241 => {
  { let v: u8 = 0x0u8; cpu.w8(0x581b02u32, v); }
  pc = 0x41a248; continue;
 }
 0x41a248 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a249(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41a249); }
 let mut pc: u32 = 0x41a249;
 loop { match pc {
 0x41a249 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6c70u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6c74u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4d6c78u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x4d6a00u32)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6c7cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x4d6a04u32)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41a27e; continue; }
  pc = 0x41a27d; continue;
 }
 0x41a27d => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x41a27e; continue;
 }
 0x41a27e => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41a284; continue; }
  pc = 0x41a282; continue;
 }
 0x41a282 => {
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x41a284; continue;
 }
 0x41a284 => {
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x8u32); cpu.eax = v; }
  if !cpu.sf { pc = 0x41a291; continue; }
  pc = 0x41a28f; continue;
 }
 0x41a28f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x41a291; continue;
 }
 0x41a291 => {
  { let v: u32 = cpu.sub32(cpu.ebx, 0x8u32); cpu.ebx = v; }
  if !cpu.sf { pc = 0x41a298; continue; }
  pc = 0x41a296; continue;
 }
 0x41a296 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x41a298; continue;
 }
 0x41a298 => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x4d678eu32)); }
  if (cpu.sf != cpu.of) { pc = 0x41a2aa; continue; }
  pc = 0x41a2a1; continue;
 }
 0x41a2a1 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x4d678eu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x41a2aa; continue;
 }
 0x41a2aa => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x4d6790u32)); }
  if (cpu.sf != cpu.of) { pc = 0x41a2bc; continue; }
  pc = 0x41a2b3; continue;
 }
 0x41a2b3 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x4d6790u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x41a2bc; continue;
 }
 0x41a2bc => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4d6be4u32, v); }
  { let v: u32 = 0x52bbf0u32; cpu.esi = v; }
  { let v: u32 = 0x59249cu32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5928b0u32, v); }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x41a2e9; continue;
 }
 0x41a2e9 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x41a397; continue; }
  pc = 0x41a2f3; continue;
 }
 0x41a2f3 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x41a397; continue; }
  pc = 0x41a2fd; continue;
 }
 0x41a2fd => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x41a397; continue; }
  pc = 0x41a316; continue;
 }
 0x41a316 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x41a397; continue; }
  pc = 0x41a31d; continue;
 }
 0x41a31d => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x41a397; continue; }
  pc = 0x41a327; continue;
 }
 0x41a327 => {
  { cpu.sub8(cpu.r8(0x4e1fc0u32), 0x0u8); }
  if !cpu.zf { pc = 0x41a33e; continue; }
  pc = 0x41a330; continue;
 }
 0x41a330 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { pc = 0x41a397; continue; }
  pc = 0x41a33e; continue;
 }
 0x41a33e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x41a36c; continue; }
  pc = 0x41a359; continue;
 }
 0x41a359 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x41a36c; continue; }
  pc = 0x41a366; continue;
 }
 0x41a366 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  pc = 0x41a36c; continue;
 }
 0x41a36c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6bd8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x41a397; continue; }
  pc = 0x41a374; continue;
 }
 0x41a374 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6be0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41a397; continue; }
  pc = 0x41a37c; continue;
 }
 0x41a37c => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4d6bdcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x41a397; continue; }
  pc = 0x41a384; continue;
 }
 0x41a384 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4d6be4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41a397; continue; }
  pc = 0x41a38c; continue;
 }
 0x41a38c => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x5928b0u32)); cpu.w8(0x5928b0u32, v); }
  pc = 0x41a397; continue;
 }
 0x41a397 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x41a2e9; continue; }
  pc = 0x41a3a1; continue;
 }
 0x41a3a1 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a3a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41a3a3); }
 let mut pc: u32 = 0x41a3a3;
 loop { match pc {
 0x41a3a3 => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0xfeu8); }
  if !cpu.zf { pc = 0x41a3af; continue; }
  pc = 0x41a3ad; continue;
 }
 0x41a3ad => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41a3af => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.ebp)), 0x2084u32); }
  if cpu.zf { pc = 0x41a3c0; continue; }
  pc = 0x41a3b9; continue;
 }
 0x41a3b9 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x41a3c2; continue; }
  pc = 0x41a3c0; continue;
 }
 0x41a3c0 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41a3c2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x32u8); }
  if cpu.zf { pc = 0x41a3e6; continue; }
  pc = 0x41a3cf; continue;
 }
 0x41a3cf => {
  { cpu.sub8((cpu.eax as u8), 0x2fu8); }
  if cpu.zf { pc = 0x41a3e6; continue; }
  pc = 0x41a3d3; continue;
 }
 0x41a3d3 => {
  { cpu.sub8((cpu.eax as u8), 0x30u8); }
  if cpu.zf { pc = 0x41a3e6; continue; }
  pc = 0x41a3d7; continue;
 }
 0x41a3d7 => {
  { cpu.sub8((cpu.eax as u8), 0x3au8); }
  if cpu.zf { pc = 0x41a3e6; continue; }
  pc = 0x41a3db; continue;
 }
 0x41a3db => {
  { cpu.sub8((cpu.eax as u8), 0x38u8); }
  if cpu.zf { pc = 0x41a3e6; continue; }
  pc = 0x41a3df; continue;
 }
 0x41a3df => {
  { cpu.sub8((cpu.eax as u8), 0x39u8); }
  if cpu.zf { pc = 0x41a3e6; continue; }
  pc = 0x41a3e3; continue;
 }
 0x41a3e3 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41a3e6 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a3e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41a3e9); }
 let mut pc: u32 = 0x41a3e9;
 loop { match pc {
 0x41a3e9 => {
  { let v: u32 = cpu.r8(0x5928b0u32) as u32; cpu.ecx = v; }
  { let v: u32 = 0x59249cu32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x59285eu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x592862u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x592866u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x59286au32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x59286eu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x592872u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x592876u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x59287au32, v); }
  pc = 0x41a445; continue;
 }
 0x41a445 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x52bbf0u32); cpu.eax = v; }
  { let v: u8 = 0x48u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.w32(0x592886u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x59288au32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x59288eu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x592892u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x592896u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x59289au32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x59289eu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5928a2u32, v); }
  pc = 0x41a4a3; continue;
 }
 0x41a4a3 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x41a4db; continue; }
  pc = 0x41a4a7; continue;
 }
 0x41a4a7 => {
  { let v: u32 = cpu.shl32(cpu.r32(0x592886u32), 1u32); cpu.w32(0x592886u32, v); }
  { let v: u32 = cpu.rcl32(cpu.r32(0x59288au32), 1u32); cpu.w32(0x59288au32, v); }
  { let v: u32 = cpu.rcl32(cpu.r32(0x59288eu32), 1u32); cpu.w32(0x59288eu32, v); }
  { let v: u32 = cpu.rcl32(cpu.r32(0x592892u32), 1u32); cpu.w32(0x592892u32, v); }
  { let v: u32 = cpu.rcl32(cpu.r32(0x592896u32), 1u32); cpu.w32(0x592896u32, v); }
  { let v: u32 = cpu.rcl32(cpu.r32(0x59289au32), 1u32); cpu.w32(0x59289au32, v); }
  { let v: u32 = cpu.rcl32(cpu.r32(0x59289eu32), 1u32); cpu.w32(0x59289eu32, v); }
  { let v: u32 = cpu.rcl32(cpu.r32(0x5928a2u32), 1u32); cpu.w32(0x5928a2u32, v); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x41a4a3; continue; }
  pc = 0x41a4db; continue;
 }
 0x41a4db => {
  { let v: u32 = cpu.r32(0x592886u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59285eu32) | cpu.eax; cpu.w32(0x59285eu32, v); }
  { let v: u32 = cpu.r32(0x59288au32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x592862u32) | cpu.eax; cpu.w32(0x592862u32, v); }
  { let v: u32 = cpu.r32(0x59288eu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x592866u32) | cpu.eax; cpu.w32(0x592866u32, v); }
  { let v: u32 = cpu.r32(0x592892u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59286au32) | cpu.eax; cpu.w32(0x59286au32, v); }
  { let v: u32 = cpu.r32(0x592896u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59286eu32) | cpu.eax; cpu.w32(0x59286eu32, v); }
  { let v: u32 = cpu.r32(0x59289au32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x592872u32) | cpu.eax; cpu.w32(0x592872u32, v); }
  { let v: u32 = cpu.r32(0x59289eu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x592876u32) | cpu.eax; cpu.w32(0x592876u32, v); }
  { let v: u32 = cpu.r32(0x5928a2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x59287au32) | cpu.eax; cpu.w32(0x59287au32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x41a445; continue; }
  pc = 0x41a53e; continue;
 }
 0x41a53e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a53f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41a53f); }
 let mut pc: u32 = 0x41a53f;
 loop { match pc {
 0x41a53f => {
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x59249cu32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5928b0u32, v); }
  { let v: u32 = 0x52bbf0u32; cpu.ebp = v; }
  pc = 0x41a555; continue;
 }
 0x41a555 => {
  { let v: u32 = cpu.shr32(cpu.r32(0x59287au32), 1u32); cpu.w32(0x59287au32, v); }
  { let v: u32 = cpu.rcr32(cpu.r32(0x592876u32), 1u32); cpu.w32(0x592876u32, v); }
  { let v: u32 = cpu.rcr32(cpu.r32(0x592872u32), 1u32); cpu.w32(0x592872u32, v); }
  { let v: u32 = cpu.rcr32(cpu.r32(0x59286eu32), 1u32); cpu.w32(0x59286eu32, v); }
  { let v: u32 = cpu.rcr32(cpu.r32(0x59286au32), 1u32); cpu.w32(0x59286au32, v); }
  { let v: u32 = cpu.rcr32(cpu.r32(0x592866u32), 1u32); cpu.w32(0x592866u32, v); }
  { let v: u32 = cpu.rcr32(cpu.r32(0x592862u32), 1u32); cpu.w32(0x592862u32, v); }
  { let v: u32 = cpu.rcr32(cpu.r32(0x59285eu32), 1u32); cpu.w32(0x59285eu32, v); }
  if !cpu.cf { pc = 0x41a599; continue; }
  pc = 0x41a587; continue;
 }
 0x41a587 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x41a599; continue; }
  pc = 0x41a58e; continue;
 }
 0x41a58e => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x5928b0u32)); cpu.w8(0x5928b0u32, v); }
  pc = 0x41a599; continue;
 }
 0x41a599 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x48u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x41a555; continue; }
  pc = 0x41a59f; continue;
 }
 0x41a59f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a605(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41a605); }
 let mut pc: u32 = 0x41a605;
 loop { match pc {
 0x41a605 => {
  cpu.push(0x41a60a); f_00418bd9(cpu); if cpu.returned_to != 0x41a60a { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x41a629); f_0042e0fa(cpu); if cpu.returned_to != 0x41a629 { return; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a62f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41a62f); }
 let mut pc: u32 = 0x41a62f;
 loop { match pc {
 0x41a62f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6aecu32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u32 = cpu.r8(0x4e1fa6u32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b2u32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f4cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x590aa2u32, v); }
  cpu.push(0x41a671); f_00413337(cpu); if cpu.returned_to != 0x41a671 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  { let v: u8 = cpu.or8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x8u8); cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a689(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41a689); }
 let mut pc: u32 = 0x41a689;
 loop { match pc {
 0x41a689 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6bf8u32, v); }
  cpu.push(0x41a69d); f_0041a6bf(cpu); if cpu.returned_to != 0x41a69d { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a69f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41a69f); } r_0041a69f(cpu, 0x41a69f); }
pub fn f_0041a6bf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41a6bf); } r_0041a69f(cpu, 0x41a6bf); }
fn r_0041a69f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x41a69f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x41a6aa); f_004184ca(cpu); if cpu.returned_to != 0x41a6aa { return; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bf8u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x41a6e7; continue;
 }
 0x41a6bf => {
  { cpu.sub8((cpu.ecx as u8), 0x6u8); }
  if cpu.zf { pc = 0x41a72f; continue; }
  pc = 0x41a6c4; continue;
 }
 0x41a6c4 => {
  { cpu.sub8((cpu.ecx as u8), 0x19u8); }
  if cpu.zf { pc = 0x41a72f; continue; }
  pc = 0x41a6c9; continue;
 }
 0x41a6c9 => {
  { cpu.sub8((cpu.ecx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x41a72f; continue; }
  pc = 0x41a6ce; continue;
 }
 0x41a6ce => {
  { cpu.sub8((cpu.ecx as u8), 0x18u8); }
  if cpu.zf { pc = 0x41a72f; continue; }
  pc = 0x41a6d3; continue;
 }
 0x41a6d3 => {
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if cpu.zf { pc = 0x41a72f; continue; }
  pc = 0x41a6d8; continue;
 }
 0x41a6d8 => {
  { cpu.sub8((cpu.ecx as u8), 0x22u8); }
  if cpu.zf { pc = 0x41a72f; continue; }
  pc = 0x41a6dd; continue;
 }
 0x41a6dd => {
  { cpu.sub8((cpu.ecx as u8), 0x24u8); }
  if cpu.zf { pc = 0x41a72f; continue; }
  pc = 0x41a6e2; continue;
 }
 0x41a6e2 => {
  { cpu.sub8((cpu.ecx as u8), 0x26u8); }
  if cpu.zf { pc = 0x41a72f; continue; }
  pc = 0x41a6e7; continue;
 }
 0x41a6e7 => {
  { let v: u32 = 0xfffffff9u32; cpu.ecx = v; }
  { let v: u32 = 0xfffffff9u32; cpu.edx = v; }
  cpu.push(0x41a6f6); f_0041a7bd(cpu); if cpu.returned_to != 0x41a6f6 { return; }
  if cpu.zf { pc = 0x41a72d; continue; }
  pc = 0x41a6f8; continue;
 }
 0x41a6f8 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0xfffffff9u32; cpu.edx = v; }
  cpu.push(0x41a707); f_0041a7bd(cpu); if cpu.returned_to != 0x41a707 { return; }
  if cpu.zf { pc = 0x41a72d; continue; }
  pc = 0x41a709; continue;
 }
 0x41a709 => {
  { let v: u32 = 0xfffffff9u32; cpu.ecx = v; }
  { let v: u32 = 0x7u32; cpu.edx = v; }
  cpu.push(0x41a718); f_0041a7bd(cpu); if cpu.returned_to != 0x41a718 { return; }
  if cpu.zf { pc = 0x41a72d; continue; }
  pc = 0x41a71a; continue;
 }
 0x41a71a => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x7u32; cpu.edx = v; }
  cpu.push(0x41a729); f_0041a7bd(cpu); if cpu.returned_to != 0x41a729 { return; }
  if cpu.zf { pc = 0x41a72d; continue; }
  pc = 0x41a72b; continue;
 }
 0x41a72b => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41a72d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41a72f => {
  { let v: u32 = 0xfffffff1u32; cpu.ecx = v; }
  { let v: u32 = 0xfffffff1u32; cpu.edx = v; }
  cpu.push(0x41a73e); f_0041a7bd(cpu); if cpu.returned_to != 0x41a73e { return; }
  if cpu.zf { cpu.poll(); pc = 0x41a72d; continue; }
  pc = 0x41a740; continue;
 }
 0x41a740 => {
  { let v: u32 = 0xfu32; cpu.ecx = v; }
  { let v: u32 = 0xfffffff1u32; cpu.edx = v; }
  cpu.push(0x41a74f); f_0041a7bd(cpu); if cpu.returned_to != 0x41a74f { return; }
  if cpu.zf { cpu.poll(); pc = 0x41a72d; continue; }
  pc = 0x41a751; continue;
 }
 0x41a751 => {
  { let v: u32 = 0xfffffff1u32; cpu.ecx = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  cpu.push(0x41a760); f_0041a7bd(cpu); if cpu.returned_to != 0x41a760 { return; }
  if cpu.zf { cpu.poll(); pc = 0x41a72d; continue; }
  pc = 0x41a762; continue;
 }
 0x41a762 => {
  { let v: u32 = 0xfu32; cpu.ecx = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  cpu.push(0x41a771); f_0041a7bd(cpu); if cpu.returned_to != 0x41a771 { return; }
  if cpu.zf { cpu.poll(); pc = 0x41a72d; continue; }
  pc = 0x41a773; continue;
 }
 0x41a773 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a775(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41a775); }
 let mut pc: u32 = 0x41a775;
 loop { match pc {
 0x41a775 => {
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x41a784); f_0041a7bd(cpu); if cpu.returned_to != 0x41a784 { return; }
  if cpu.zf { pc = 0x41a7bb; continue; }
  pc = 0x41a786; continue;
 }
 0x41a786 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x41a795); f_0041a7bd(cpu); if cpu.returned_to != 0x41a795 { return; }
  if cpu.zf { pc = 0x41a7bb; continue; }
  pc = 0x41a797; continue;
 }
 0x41a797 => {
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  cpu.push(0x41a7a6); f_0041a7bd(cpu); if cpu.returned_to != 0x41a7a6 { return; }
  if cpu.zf { pc = 0x41a7bb; continue; }
  pc = 0x41a7a8; continue;
 }
 0x41a7a8 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  cpu.push(0x41a7b7); f_0041a7bd(cpu); if cpu.returned_to != 0x41a7b7 { return; }
  if cpu.zf { pc = 0x41a7bb; continue; }
  pc = 0x41a7b9; continue;
 }
 0x41a7b9 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41a7bb => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a7bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41a7bd); }
 let mut pc: u32 = 0x41a7bd;
 loop { match pc {
 0x41a7bd => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x41a7cb); f_00420989(cpu); if cpu.returned_to != 0x41a7cb { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6bf8u32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a7e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41a7e3); }
 let mut pc: u32 = 0x41a7e3;
 loop { match pc {
 0x41a7e3 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x41a7eb); f_00420989(cpu); if cpu.returned_to != 0x41a7eb { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0x14u8); }
  if cpu.zf { pc = 0x41a814; continue; }
  pc = 0x41a7f9; continue;
 }
 0x41a7f9 => {
  { cpu.sub8((cpu.ebx as u8), 0x15u8); }
  if cpu.zf { pc = 0x41a834; continue; }
  pc = 0x41a7fe; continue;
 }
 0x41a7fe => {
  { cpu.sub8((cpu.ebx as u8), 0x3cu8); }
  if cpu.zf { pc = 0x41a862; continue; }
  pc = 0x41a803; continue;
 }
 0x41a803 => {
  { cpu.sub8((cpu.ebx as u8), 0x3du8); }
  if cpu.zf { pc = 0x41a882; continue; }
  pc = 0x41a808; continue;
 }
 0x41a808 => {
  { let v: u8 = 0xffu8; cpu.w8(0x5928cdu32, v); }
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41a814 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5928cdu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41a834 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5928cdu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x56c72fu32), 0x0u8); }
  if !cpu.zf { pc = 0x41a860; continue; }
  pc = 0x41a85b; continue;
 }
 0x41a85b => {
  cpu.push(0x41a860); f_00415a3b(cpu); if cpu.returned_to != 0x41a860 { return; }
  pc = 0x41a860; continue;
 }
 0x41a860 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41a862 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5928cdu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41a882 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x5928cdu32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041a8a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41a8a2); }
 let mut pc: u32 = 0x41a8a2;
 loop { match pc {
 0x41a8a2 => {
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x41aa1d; continue; }
  pc = 0x41a8ac; continue;
 }
 0x41a8ac => {
  { cpu.sub8(cpu.r8(0x56c9b8u32), 0x0u8); }
  if !cpu.zf { pc = 0x41aa30; continue; }
  pc = 0x41a8b9; continue;
 }
 0x41a8b9 => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d67eeu32, v); }
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x41a8e1; continue; }
  pc = 0x41a8cc; continue;
 }
 0x41a8cc => {
  { let v: u16 = cpu.r16(0x4d6758u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67eeu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x41a8e0); f_0041879e(cpu); if cpu.returned_to != 0x41a8e0 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x41a8e1; continue;
 }
 0x41a8e1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6758u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d688au32, v); }
  cpu.push(0x41a8f2); f_004184ca(cpu); if cpu.returned_to != 0x41a8f2 { return; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x41aa1d; continue; }
  pc = 0x41a8fd; continue;
 }
 0x41a8fd => {
  { cpu.sub8(cpu.r8(0x4e1fc0u32), 0x0u8); }
  if !cpu.zf { pc = 0x41a918; continue; }
  pc = 0x41a906; continue;
 }
 0x41a906 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1559u32), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x41aa1d; continue; }
  pc = 0x41a918; continue;
 }
 0x41a918 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x59d9f0u32, v); }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x41aa1d; continue; }
  pc = 0x41a929; continue;
 }
 0x41a929 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c44u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15c1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f9fu32, v); }
  cpu.push(0x41a94f); f_0041abf1(cpu); if cpu.returned_to != 0x41a94f { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x59da88u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1567u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6888u32, v); }
  { let v: u32 = 0x559758u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4d6bb8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x559750u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x559752u32, v); }
  { cpu.sub8(cpu.r8(0x58c42cu32), 0x0u8); }
  if cpu.zf { pc = 0x41a997; continue; }
  pc = 0x41a98e; continue;
 }
 0x41a98e => {
  { cpu.sub8(cpu.r8(0x58c838u32), 0x0u8); }
  if !cpu.zf { pc = 0x41a99c; continue; }
  pc = 0x41a997; continue;
 }
 0x41a997 => {
  cpu.push(0x41a99c); f_0041887f(cpu); if cpu.returned_to != 0x41a99c { return; }
  pc = 0x41a99c; continue;
 }
 0x41a99c => {
  cpu.push(0x41a9a1); f_004184ca(cpu); if cpu.returned_to != 0x41a9a1 { return; }
  { let v: u8 = cpu.r8(0x5cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41aa1c; continue; }
  pc = 0x41a9a9; continue;
 }
 0x41a9a9 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)) | 0x2u8; cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x41aa1d; continue; }
  pc = 0x41a9ca; continue;
 }
 0x41a9ca => {
  { cpu.sub8(cpu.r8(0x56d389u32), 0x0u8); }
  if !cpu.zf { pc = 0x41aa1c; continue; }
  pc = 0x41a9d3; continue;
 }
 0x41a9d3 => {
  cpu.push(0x41a9d8); f_004164ef(cpu); if cpu.returned_to != 0x41a9d8 { return; }
  cpu.push(0x41a9dd); f_004184ca(cpu); if cpu.returned_to != 0x41a9dd { return; }
  { let v: u8 = 0xf8u8; cpu.w8(0x4e1f4cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x509290u32); cpu.eax = v; }
  { let v = (cpu.ecx as u8); cpu.div8(v); }
  { let v: u8 = cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6aecu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5b3d70u32, v); }
  cpu.push(0x41aa09); f_00413337(cpu); if cpu.returned_to != 0x41aa09 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3d70u32, v); }
  pc = 0x41aa17; continue;
 }
 0x41aa17 => {
  cpu.push(0x41aa1c); f_0044c666(cpu); if cpu.returned_to != 0x41aa1c { return; }
  pc = 0x41aa1c; continue;
 }
 0x41aa1c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41aa1d => {
  { let v: u16 = 0x0u16; cpu.w16(0x4d6758u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d688au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41aa30 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0xb8u32; cpu.ecx = v; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x41aa1d; continue; }
  pc = 0x41aa4d; continue;
 }
 0x41aa4d => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { cpu.poll(); pc = 0x41aa1d; continue; }
  pc = 0x41aa54; continue;
 }
 0x41aa54 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x4e1559u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x41aa1d; continue; }
  pc = 0x41aa62; continue;
 }
 0x41aa62 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x59d9f0u32, v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c44u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15c1u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6888u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f9fu32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x80u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x588b82u32), 0x0u8); }
  if cpu.zf { pc = 0x41aab1; continue; }
  pc = 0x41aaad; continue;
 }
 0x41aaad => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x41aab1; continue;
 }
 0x41aab1 => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41aac2; continue; }
  pc = 0x41aaba; continue;
 }
 0x41aaba => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x41aac2; continue;
 }
 0x41aac2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d675au32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d675cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041aad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41aad0); }
 let mut pc: u32 = 0x41aad0;
 loop { match pc {
 0x41aad0 => {
  { let v: u32 = 0x1u32; cpu.w32(0x592a81u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x41abd7; continue; }
  pc = 0x41aae5; continue;
 }
 0x41aae5 => {
  { let v: u32 = 0x2u32; cpu.w32(0x592a81u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x41abd7; continue; }
  pc = 0x41aafa; continue;
 }
 0x41aafa => {
  { let v: u32 = 0x3u32; cpu.w32(0x592a81u32, v); }
  { let v: u32 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x592a27u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x41abd7; continue; }
  pc = 0x41ab16; continue;
 }
 0x41ab16 => {
  { let v: u32 = 0x4u32; cpu.w32(0x592a81u32, v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x41abd7; continue; }
  pc = 0x41ab3d; continue;
 }
 0x41ab3d => {
  { let v: u32 = 0x5u32; cpu.w32(0x592a81u32, v); }
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x41abd7; continue; }
  pc = 0x41ab51; continue;
 }
 0x41ab51 => {
  { let v: u32 = 0x6u32; cpu.w32(0x592a81u32, v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x41abd7; continue; }
  pc = 0x41ab62; continue;
 }
 0x41ab62 => {
  { let v: u32 = 0x7u32; cpu.w32(0x592a81u32, v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x3du8); }
  if cpu.zf { pc = 0x41abd7; continue; }
  pc = 0x41ab73; continue;
 }
 0x41ab73 => {
  { let v: u32 = 0x8u32; cpu.w32(0x592a81u32, v); }
  cpu.push(0x41ab82); f_0045ad3a(cpu); if cpu.returned_to != 0x41ab82 { return; }
  { cpu.pushad(); }
  { let v = cpu.r32(0x4d6a28u32); cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6a28u32, v); }
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  { let v: u8 = 0x14u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x41ab9a; continue;
 }
 0x41ab9a => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x41abcb; continue; }
  pc = 0x41ab9f; continue;
 }
 0x41ab9f => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x592a27u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x41abcb; continue; }
  pc = 0x41abba; continue;
 }
 0x41abba => {
  { let v: u32 = cpu.add32(cpu.r32(0x592a81u32), 0x100u32); cpu.w32(0x592a81u32, v); }
  { cpu.pushad(); }
  cpu.push(0x41abca); f_0043ffdb(cpu); if cpu.returned_to != 0x41abca { return; }
  { cpu.popad(); }
  pc = 0x41abcb; continue;
 }
 0x41abcb => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x41ab9a; continue; }
  pc = 0x41abd0; continue;
 }
 0x41abd0 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6a28u32, v); }
  { cpu.popad(); }
  pc = 0x41abd7; continue;
 }
 0x41abd7 => {
  { let v: u32 = cpu.r32(0x592a81u32); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041abde(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41abde); }
 let mut pc: u32 = 0x41abde;
 loop { match pc {
 0x41abde => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u8 = cpu.r8(0x52bc03u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041abf1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41abf1); }
 let mut pc: u32 = 0x41abf1;
 loop { match pc {
 0x41abf1 => {
  { let v: u16 = cpu.r16(0x4d6758u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x509290u32); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e155bu32, v); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x41ac1c; continue; }
  pc = 0x41ac14; continue;
 }
 0x41ac14 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e155bu32, v); }
  pc = 0x41ac1b; continue;
 }
 0x41ac1b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41ac1c => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { cpu.poll(); pc = 0x41ac1b; continue; }
  pc = 0x41ac23; continue;
 }
 0x41ac23 => {
  { let v: u8 = 0x2u8; cpu.w8(0x4e155bu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041ac57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41ac57); }
 let mut pc: u32 = 0x41ac57;
 loop { match pc {
 0x41ac57 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x41ac5d); f_0041bf5f(cpu); if cpu.returned_to != 0x41ac5d { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6ad8u32); cpu.esp = v; }
  cpu.push(0x41ac69); f_00405810(cpu); if cpu.returned_to != 0x41ac69 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041ac6d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41ac6d); }
 let mut pc: u32 = 0x41ac6d;
 loop { match pc {
 0x41ac6d => {
  cpu.push(0x41ac72); f_00418beb(cpu); if cpu.returned_to != 0x41ac72 { return; }
  { let v: u32 = 0x4ac564u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41ac85; continue; }
  pc = 0x41ac80; continue;
 }
 0x41ac80 => {
  { let v: u32 = 0x4acc14u32; cpu.esi = v; }
  pc = 0x41ac85; continue;
 }
 0x41ac85 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x41ac9e; continue; }
  pc = 0x41ac8a; continue;
 }
 0x41ac8a => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41ac9e; continue; }
  pc = 0x41ac90; continue;
 }
 0x41ac90 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x41ac9e; continue; }
  pc = 0x41ac96; continue;
 }
 0x41ac96 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x6u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41ac9e; continue; }
  pc = 0x41ac9c; continue;
 }
 0x41ac9c => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41ac9e => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041aca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41aca0); }
 let mut pc: u32 = 0x41aca0;
 loop { match pc {
 0x41aca0 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x41acb3; continue; }
  pc = 0x41aca9; continue;
 }
 0x41aca9 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x41acb3; continue; }
  pc = 0x41acb2; continue;
 }
 0x41acb2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41acb3 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x41ace7; continue; }
  pc = 0x41acbf; continue;
 }
 0x41acbf => {
  { cpu.and8(cpu.r8(0x58c87eu32), 0x1u8); }
  if !cpu.zf { pc = 0x41ad5b; continue; }
  pc = 0x41accc; continue;
 }
 0x41accc => {
  { cpu.and8(cpu.r8(0x58c87eu32), 0x2u8); }
  if !cpu.zf { pc = 0x41adcc; continue; }
  pc = 0x41acd9; continue;
 }
 0x41acd9 => {
  { cpu.and8(cpu.r8(0x58c87eu32), 0x4u8); }
  if !cpu.zf { pc = 0x41add4; continue; }
  pc = 0x41ace6; continue;
 }
 0x41ace6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41ace7 => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if cpu.zf { pc = 0x41acf1; continue; }
  pc = 0x41acf0; continue;
 }
 0x41acf0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41acf1 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if cpu.zf { pc = 0x41acfb; continue; }
  pc = 0x41acfa; continue;
 }
 0x41acfa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41acfb => {
  cpu.push(0x41ad00); f_0041ac6d(cpu); if cpu.returned_to != 0x41ad00 { return; }
  if cpu.cf { cpu.poll(); pc = 0x41ace6; continue; }
  pc = 0x41ad02; continue;
 }
 0x41ad02 => {
  { cpu.pushad(); }
  cpu.push(0x41ad08); f_00403030(cpu); if cpu.returned_to != 0x41ad08 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x41ad0e; continue; }
  pc = 0x41ad0d; continue;
 }
 0x41ad0d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41ad0e => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x41ad27; continue; }
  pc = 0x41ad17; continue;
 }
 0x41ad17 => {
  { cpu.sub16(cpu.r16(0x56d2ffu32), 0x2u16); }
  if (cpu.sf != cpu.of) { pc = 0x41add9; continue; }
  pc = 0x41ad25; continue;
 }
 0x41ad25 => {
  pc = 0x41ad34; continue;
 }
 0x41ad27 => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x41add9; continue; }
  pc = 0x41ad34; continue;
 }
 0x41ad34 => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x557680u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x41add4; continue; }
  pc = 0x41ad46; continue;
 }
 0x41ad46 => {
  { cpu.and8(cpu.r8(0x4e15a5u32), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x41add4; continue; }
  pc = 0x41ad52; continue;
 }
 0x41ad52 => {
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x41adcc; continue; }
  pc = 0x41ad5b; continue;
 }
 0x41ad5b => {
  cpu.push(0x41ad60); f_0041af67(cpu); if cpu.returned_to != 0x41ad60 { return; }
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x41ad75; continue; }
  pc = 0x41ad6c; continue;
 }
 0x41ad6c => {
  { cpu.sub8(cpu.r8(0x58c498u32), 0x0u8); }
  if !cpu.zf { pc = 0x41adc3; continue; }
  pc = 0x41ad75; continue;
 }
 0x41ad75 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x41adc3; continue; }
  pc = 0x41ad81; continue;
 }
 0x41ad81 => {
  { cpu.sub8(cpu.r8(0x58c498u32), 0x0u8); }
  if !cpu.zf { pc = 0x41adc3; continue; }
  pc = 0x41ad8a; continue;
 }
 0x41ad8a => {
  { cpu.sub8(cpu.r8(0x58c43fu32), 0x0u8); }
  if !cpu.zf { pc = 0x41adc3; continue; }
  pc = 0x41ad93; continue;
 }
 0x41ad93 => {
  { let v: u16 = cpu.r16(0x58c496u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6a04u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x41ada5; continue; }
  pc = 0x41ada2; continue;
 }
 0x41ada2 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x41ada5; continue;
 }
 0x41ada5 => {
  { cpu.sub16((cpu.eax as u16), 0x4u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41adc4; continue; }
  pc = 0x41adab; continue;
 }
 0x41adab => {
  { let v: u16 = cpu.r16(0x58c494u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x41adbd; continue; }
  pc = 0x41adba; continue;
 }
 0x41adba => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x41adbd; continue;
 }
 0x41adbd => {
  { cpu.sub16((cpu.eax as u16), 0x4u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41adc4; continue; }
  pc = 0x41adc3; continue;
 }
 0x41adc3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41adc4 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c43fu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41adcc => {
  cpu.push(0x41add1); f_0041ae53(cpu); if cpu.returned_to != 0x41add1 { return; }
  if cpu.cf { pc = 0x41add4; continue; }
  pc = 0x41add3; continue;
 }
 0x41add3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41add4 => {
  cpu.push(0x41add9); f_0041adda(cpu); if cpu.returned_to != 0x41add9 { return; }
  pc = 0x41add9; continue;
 }
 0x41add9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041adda(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41adda); }
 let mut pc: u32 = 0x41adda;
 loop { match pc {
 0x41adda => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x41adf4; continue; }
  pc = 0x41ade6; continue;
 }
 0x41ade6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c439u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c43au32, v); }
  pc = 0x41adf4; continue;
 }
 0x41adf4 => {
  { cpu.sub8(cpu.r8(0x4e15b8u32), 0x0u8); }
  if cpu.zf { pc = 0x41ae1f; continue; }
  pc = 0x41adfd; continue;
 }
 0x41adfd => {
  { let v: u16 = cpu.r16(0x4d67c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67cau32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x41ae0f); f_0041c26e(cpu); if cpu.returned_to != 0x41ae0f { return; }
  { let v: u16 = 0xfc18u16; cpu.w16(0x4d67c8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b8u32, v); }
  pc = 0x41ae1f; continue;
 }
 0x41ae1f => {
  { cpu.sub8(cpu.r8(0x4e1568u32), 0x0u8); }
  if cpu.zf { pc = 0x41ae52; continue; }
  pc = 0x41ae28; continue;
 }
 0x41ae28 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x41ae42; continue; }
  pc = 0x41ae34; continue;
 }
 0x41ae34 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c439u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c43au32, v); }
  pc = 0x41ae42; continue;
 }
 0x41ae42 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1568u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d675au32, v); }
  pc = 0x41ae52; continue;
 }
 0x41ae52 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041ae53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41ae53); }
 let mut pc: u32 = 0x41ae53;
 loop { match pc {
 0x41ae53 => {
  cpu.push(0x41ae58); f_00418beb(cpu); if cpu.returned_to != 0x41ae58 { return; }
  pc = 0x41ae5a; continue;
 }
 0x41ae5a => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41aea3; continue; }
  pc = 0x41ae63; continue;
 }
 0x41ae63 => {
  { cpu.sub8(cpu.r8(0x56d16eu32), 0x0u8); }
  if cpu.zf { pc = 0x41ae86; continue; }
  pc = 0x41ae6c; continue;
 }
 0x41ae6c => {
  { cpu.sub16((cpu.eax as u16), 0x280u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41af65; continue; }
  pc = 0x41ae76; continue;
 }
 0x41ae76 => {
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.eax = v; }
  { cpu.sub16((cpu.ebx as u16), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41af65; continue; }
  pc = 0x41ae84; continue;
 }
 0x41ae84 => {
  pc = 0x41aeda; continue;
 }
 0x41ae86 => {
  { cpu.sub16((cpu.eax as u16), 0x220u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41af65; continue; }
  pc = 0x41ae90; continue;
 }
 0x41ae90 => {
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x20u32); cpu.eax = v; }
  { cpu.sub16((cpu.ebx as u16), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41af65; continue; }
  pc = 0x41aea1; continue;
 }
 0x41aea1 => {
  pc = 0x41aeda; continue;
 }
 0x41aea3 => {
  { cpu.sub8(cpu.r8(0x56d16eu32), 0x0u8); }
  if cpu.zf { pc = 0x41aec3; continue; }
  pc = 0x41aeac; continue;
 }
 0x41aeac => {
  { cpu.sub16((cpu.eax as u16), 0x140u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41af65; continue; }
  pc = 0x41aeb6; continue;
 }
 0x41aeb6 => {
  { cpu.sub16((cpu.ebx as u16), 0xc8u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41af65; continue; }
  pc = 0x41aec1; continue;
 }
 0x41aec1 => {
  pc = 0x41aeda; continue;
 }
 0x41aec3 => {
  { cpu.sub16((cpu.eax as u16), 0x110u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41af65; continue; }
  pc = 0x41aecd; continue;
 }
 0x41aecd => {
  { cpu.sub16((cpu.ebx as u16), 0xb8u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41af65; continue; }
  pc = 0x41aed8; continue;
 }
 0x41aed8 => {
  pc = 0x41aeda; continue;
 }
 0x41aeda => {
  { cpu.sub8(cpu.r8(0x4e1568u32), 0x0u8); }
  if !cpu.zf { pc = 0x41af4b; continue; }
  pc = 0x41aee3; continue;
 }
 0x41aee3 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b8u32, v); }
  cpu.push(0x41aeef); f_004188ae(cpu); if cpu.returned_to != 0x41aeef { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7du32, v); }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67c8u32, v); }
  { let v: u16 = 0xa0u16; cpu.w16(0x4d6762u32, v); }
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d677au32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67cau32, v); }
  { let v: u16 = 0x64u16; cpu.w16(0x4d6764u32, v); }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d677cu32, v); }
  { let v: u16 = 0xa0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x64u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x41af49); f_0041c26e(cpu); if cpu.returned_to != 0x41af49 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41af4b => {
  { let v: u16 = cpu.r16(0x4d6782u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d675au32, v); }
  { let v: u16 = cpu.r16(0x4d6784u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d675cu32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41af65 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041af67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41af67); }
 let mut pc: u32 = 0x41af67;
 loop { match pc {
 0x41af67 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.push(0x41af71); f_00430178(cpu); if cpu.returned_to != 0x41af71 { return; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6762u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6752u32, v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6764u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6754u32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5a2361u32), 0x0u8); }
  if cpu.zf { pc = 0x41afc0; continue; }
  pc = 0x41afab; continue;
 }
 0x41afab => {
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x5a2361u32), 0x1u8); }
  if cpu.zf { pc = 0x41afc0; continue; }
  pc = 0x41afba; continue;
 }
 0x41afba => {
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x41afc0; continue;
 }
 0x41afc0 => {
  { let v = cpu.r32(0x4d6a00u32); cpu.push(v); }
  { let v = cpu.r32(0x4d6a04u32); cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d677au32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d677cu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x41afdf); f_00418459(cpu); if cpu.returned_to != 0x41afdf { return; }
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d677au32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d677cu32, v); }
  { let v: u32 = 0x4u32; cpu.esi = v; }
  { let v: u32 = 0xfffffffcu32; cpu.edi = v; }
  { let v: u32 = 0xfffcu32; cpu.ebp = v; }
  { cpu.sub16(cpu.r16(0x4d67b6u32), 0x0u16); }
  if cpu.zf { pc = 0x41b021; continue; }
  pc = 0x41b012; continue;
 }
 0x41b012 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffu32; cpu.ebp = v; }
  pc = 0x41b021; continue;
 }
 0x41b021 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x41b04d; continue; }
  pc = 0x41b028; continue;
 }
 0x41b028 => {
  { let v: u16 = cpu.sar16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x41b038; continue; }
  pc = 0x41b02e; continue;
 }
 0x41b02e => {
  { cpu.sub16((cpu.eax as u16), (cpu.edi as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41b040; continue; }
  pc = 0x41b033; continue;
 }
 0x41b033 => {
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x41b040; continue;
 }
 0x41b038 => {
  { cpu.sub16((cpu.eax as u16), (cpu.esi as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x41b040; continue; }
  pc = 0x41b03d; continue;
 }
 0x41b03d => {
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x41b040; continue;
 }
 0x41b040 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & (cpu.ebp as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6a00u32, v); }
  pc = 0x41b04d; continue;
 }
 0x41b04d => {
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x41b077; continue; }
  pc = 0x41b052; continue;
 }
 0x41b052 => {
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 2u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x41b062; continue; }
  pc = 0x41b058; continue;
 }
 0x41b058 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.edi as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41b06a; continue; }
  pc = 0x41b05d; continue;
 }
 0x41b05d => {
  { let v: u16 = (cpu.edi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x41b06a; continue;
 }
 0x41b062 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.esi as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x41b06a; continue; }
  pc = 0x41b067; continue;
 }
 0x41b067 => {
  { let v: u16 = (cpu.esi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x41b06a; continue;
 }
 0x41b06a => {
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6a04u32, v); }
  pc = 0x41b077; continue;
 }
 0x41b077 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 3u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.edx as u16), 3u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x2u8); }
  if !cpu.zf { pc = 0x41b099; continue; }
  pc = 0x41b093; continue;
 }
 0x41b093 => {
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x41b099; continue;
 }
 0x41b099 => {
  cpu.push(0x41b09e); f_0041c9df(cpu); if cpu.returned_to != 0x41b09e { return; }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d675au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b0f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b0f5); }
 let mut pc: u32 = 0x41b0f5;
 loop { match pc {
 0x41b0f5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b18d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b18d); }
 let mut pc: u32 = 0x41b18d;
 loop { match pc {
 0x41b18d => {
  { let v: u32 = 0x4d7238u32; cpu.edi = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bfu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b1d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b1d6); }
 let mut pc: u32 = 0x41b1d6;
 loop { match pc {
 0x41b1d6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1e46u32, v); }
  { cpu.sub8(cpu.r8(0x4e1559u32), 0x0u8); }
  if cpu.zf { pc = 0x41b1ed; continue; }
  pc = 0x41b1e6; continue;
 }
 0x41b1e6 => {
  { let v: u8 = 0x78u8; cpu.w8(0x4e1e46u32, v); }
  pc = 0x41b1ed; continue;
 }
 0x41b1ed => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b1ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b1ef); }
 let mut pc: u32 = 0x41b1ef;
 loop { match pc {
 0x41b1ef => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b224(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b224); }
 let mut pc: u32 = 0x41b224;
 loop { match pc {
 0x41b224 => {
  { cpu.sub16(cpu.r16(0x4d6772u32), 0xff0u16); }
  if !cpu.cf { pc = 0x41b23d; continue; }
  pc = 0x41b22f; continue;
 }
 0x41b22f => {
  { let v: u16 = cpu.dec16(cpu.r16(0x4d6772u32)); cpu.w16(0x4d6772u32, v); }
  if !cpu.sf { pc = 0x41b23d; continue; }
  pc = 0x41b238; continue;
 }
 0x41b238 => {
  cpu.push(0x41b23d); f_0041b1ef(cpu); if cpu.returned_to != 0x41b23d { return; }
  pc = 0x41b23d; continue;
 }
 0x41b23d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b24f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b24f); }
 let mut pc: u32 = 0x41b24f;
 loop { match pc {
 0x41b24f => {
  { cpu.sub16(cpu.r16(0x4d6758u32), 0x0u16); }
  if cpu.zf { pc = 0x41b27e; continue; }
  pc = 0x41b259; continue;
 }
 0x41b259 => {
  cpu.push(0x41b25e); f_0041879e(cpu); if cpu.returned_to != 0x41b25e { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x4e1fbfu32, v); }
  cpu.push(0x41b26a); f_0041b1ef(cpu); if cpu.returned_to != 0x41b26a { return; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c10u32, v); }
  cpu.push(0x41b27e); f_00447c27(cpu); if cpu.returned_to != 0x41b27e { return; }
  pc = 0x41b27e; continue;
 }
 0x41b27e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b2d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b2d4); }
 let mut pc: u32 = 0x41b2d4;
 loop { match pc {
 0x41b2d4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b2d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b2d5); }
 let mut pc: u32 = 0x41b2d5;
 loop { match pc {
 0x41b2d5 => {
  { let v: u32 = 0x58c4a5u32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0x4ac414u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41b33a; continue; }
  pc = 0x41b330; continue;
 }
 0x41b330 => {
  cpu.push(0x41b335); f_0041795c(cpu); if cpu.returned_to != 0x41b335 { return; }
  { let v: u32 = 0x4acac4u32; cpu.esi = v; }
  pc = 0x41b33a; continue;
 }
 0x41b33a => {
  cpu.push(0x41b33f); f_0041b447(cpu); if cpu.returned_to != 0x41b33f { return; }
  cpu.push(0x41b344); f_0041b345(cpu); if cpu.returned_to != 0x41b344 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b345(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b345); }
 let mut pc: u32 = 0x41b345;
 loop { match pc {
 0x41b345 => {
  { let v: u32 = 0x557680u32; cpu.edi = v; }
  { let v: u32 = 0x4ac6d4u32; cpu.esi = v; }
  { cpu.and32(cpu.r32(0x58c87au32), 0x200u32); }
  if !cpu.zf { pc = 0x41b360; continue; }
  pc = 0x41b35b; continue;
 }
 0x41b35b => {
  { let v: u32 = 0x4ac8ccu32; cpu.esi = v; }
  pc = 0x41b360; continue;
 }
 0x41b360 => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x41b389; continue; }
  pc = 0x41b368; continue;
 }
 0x41b368 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xfffffffeu32); }
  if !cpu.zf { pc = 0x41b37c; continue; }
  pc = 0x41b36d; continue;
 }
 0x41b36d => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41b37c; continue; }
  pc = 0x41b379; continue;
 }
 0x41b379 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi))); cpu.esi = v; }
  pc = 0x41b37c; continue;
 }
 0x41b37c => {
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi; cpu.w32(0x58c4a1u32, v); }
  cpu.poll(); pc = 0x41b360; continue;
 }
 0x41b389 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b38a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b38a); }
 let mut pc: u32 = 0x41b38a;
 loop { match pc {
 0x41b38a => {
  { let v: u32 = 0x58c4a5u32; cpu.esi = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x41b394; continue;
 }
 0x41b394 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x41b3a7; continue; }
  pc = 0x41b39c; continue;
 }
 0x41b39c => {
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x41b394; continue; }
  pc = 0x41b3a2; continue;
 }
 0x41b3a2 => {
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  pc = 0x41b3a7; continue;
 }
 0x41b3a7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b3a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b3a8); }
 let mut pc: u32 = 0x41b3a8;
 loop { match pc {
 0x41b3a8 => {
  { let v: u32 = 0x58c4c1u32; cpu.esi = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x41b3b2; continue;
 }
 0x41b3b2 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x41b3c5; continue; }
  pc = 0x41b3ba; continue;
 }
 0x41b3ba => {
  { let v: u32 = cpu.sub32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x41b3b2; continue; }
  pc = 0x41b3c0; continue;
 }
 0x41b3c0 => {
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  pc = 0x41b3c5; continue;
 }
 0x41b3c5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b3c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b3c6); }
 let mut pc: u32 = 0x41b3c6;
 loop { match pc {
 0x41b3c6 => {
  cpu.push(0x41b3cb); f_0041b38a(cpu); if cpu.returned_to != 0x41b3cb { return; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x41b3d6; continue; }
  pc = 0x41b3d0; continue;
 }
 0x41b3d0 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41b3d6 => {
  { let v: u32 = cpu.r32(0x58c4a1u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x41b3d0; continue; }
  pc = 0x41b3e1; continue;
 }
 0x41b3e1 => {
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  pc = 0x41b3e5; continue;
 }
 0x41b3e5 => {
  { cpu.movs(4, false); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x41b3e5; continue; }
  pc = 0x41b3ec; continue;
 }
 0x41b3ec => {
  { let v: u32 = cpu.edi; cpu.w32(0x58c4a1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b41d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b41d); }
 let mut pc: u32 = 0x41b41d;
 loop { match pc {
 0x41b41d => {
  cpu.push(0x41b422); f_0041b3a8(cpu); if cpu.returned_to != 0x41b422 { return; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x41b42d; continue; }
  pc = 0x41b427; continue;
 }
 0x41b427 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41b42d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x58c4a1u32, v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b447(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b447); }
 let mut pc: u32 = 0x41b447;
 loop { match pc {
 0x41b447 => {
  { let v: u32 = 0x557680u32; cpu.edi = v; }
  pc = 0x41b44c; continue;
 }
 0x41b44c => {
  { cpu.movs(4, false); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x41b44c; continue; }
  pc = 0x41b453; continue;
 }
 0x41b453 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b454(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b454); }
 let mut pc: u32 = 0x41b454;
 loop { match pc {
 0x41b454 => {
  { let v: u32 = 0x557680u32; cpu.esi = v; }
  { let v: u32 = 0x58c88cu32; cpu.edi = v; }
  { let v: u32 = 0x20d0u32; cpu.ecx = v; }
  pc = 0x41b463; continue;
 }
 0x41b463 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x41b463; continue; }
  pc = 0x41b46c; continue;
 }
 0x41b46c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b46d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b46d); }
 let mut pc: u32 = 0x41b46d;
 loop { match pc {
 0x41b46d => {
  { let v: u32 = 0x58c88cu32; cpu.esi = v; }
  { let v: u32 = 0x557680u32; cpu.edi = v; }
  { let v: u32 = 0x20d0u32; cpu.ecx = v; }
  pc = 0x41b47c; continue;
 }
 0x41b47c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x41b47c; continue; }
  pc = 0x41b485; continue;
 }
 0x41b485 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b486(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b486); }
 let mut pc: u32 = 0x41b486;
 loop { match pc {
 0x41b486 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b487(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b487); }
 let mut pc: u32 = 0x41b487;
 loop { match pc {
 0x41b487 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b488(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b488); }
 let mut pc: u32 = 0x41b488;
 loop { match pc {
 0x41b488 => {
  { let v: u8 = cpu.r8(0x4e15a5u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e15a4u32, v); }
  { let v: u8 = cpu.r8(0x58c432u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c431u32, v); }
  { let v: u8 = cpu.r8(0x58c432u32) & 0xfcu8; cpu.w8(0x58c432u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c498u32, v); }
  { cpu.sub8(cpu.r8(0x575536u32), 0x0u8); }
  if !cpu.zf { pc = 0x41b4bd; continue; }
  pc = 0x41b4b3; continue;
 }
 0x41b4b3 => {
  cpu.push(0x41b4b8); f_00415156(cpu); if cpu.returned_to != 0x41b4b8 { return; }
  cpu.push(0x41b4bd); f_00434ac7(cpu); if cpu.returned_to != 0x41b4bd { return; }
  pc = 0x41b4bd; continue;
 }
 0x41b4bd => {
  { let v: u8 = 0x0u8; cpu.w8(0x5928d0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x575536u32, v); }
  { let v: u32 = 0x557680u32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x41b4e1; continue;
 }
 0x41b4e1 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x41b5cc; continue; }
  pc = 0x41b4ed; continue;
 }
 0x41b4ed => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x41b4fd; continue; }
  pc = 0x41b4f5; continue;
 }
 0x41b4f5 => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  pc = 0x41b5bf; continue;
 }
 0x41b4fd => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.ebp as u16), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x41b50c; continue; }
  pc = 0x41b504; continue;
 }
 0x41b504 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x41b5bf; continue;
 }
 0x41b50c => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x41b51b; continue; }
  pc = 0x41b513; continue;
 }
 0x41b513 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x41b5bf; continue;
 }
 0x41b51b => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.ebp as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x41b527; continue; }
  pc = 0x41b522; continue;
 }
 0x41b522 => {
  pc = 0x41b5bf; continue;
 }
 0x41b527 => {
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x419c7fu32); }
  if !cpu.zf { pc = 0x41b538; continue; }
  pc = 0x41b531; continue;
 }
 0x41b531 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c498u32, v); }
  pc = 0x41b538; continue;
 }
 0x41b538 => {
  { cpu.and8(cpu.r8(0x4e15a4u32), 0x1u8); }
  if cpu.zf { pc = 0x41b558; continue; }
  pc = 0x41b541; continue;
 }
 0x41b541 => {
  { cpu.sub8(cpu.r8(0x4e15bau32), 0x0u8); }
  if !cpu.zf { pc = 0x41b558; continue; }
  pc = 0x41b54a; continue;
 }
 0x41b54a => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15bau32, v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.push(0x41b555); cpu.call(t); if cpu.returned_to != 0x41b555 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x41b5bf; continue;
 }
 0x41b558 => {
  { cpu.and8(cpu.r8(0x4e15a4u32), 0x2u8); }
  if cpu.zf { pc = 0x41b58d; continue; }
  pc = 0x41b561; continue;
 }
 0x41b561 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x419c87u32); }
  if cpu.zf { pc = 0x41b573; continue; }
  pc = 0x41b56a; continue;
 }
 0x41b56a => {
  { cpu.sub8(cpu.r8(0x4e15bbu32), 0x0u8); }
  if !cpu.zf { pc = 0x41b58d; continue; }
  pc = 0x41b573; continue;
 }
 0x41b573 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e15bbu32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0xbu32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x41b587; continue; }
  pc = 0x41b581; continue;
 }
 0x41b581 => {
  let t = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.push(0x41b584); cpu.call(t); if cpu.returned_to != 0x41b584 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x41b5bf; continue;
 }
 0x41b587 => {
  let t = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.push(0x41b58a); cpu.call(t); if cpu.returned_to != 0x41b58a { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x41b5bf; continue;
 }
 0x41b58d => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x41b5b9; continue; }
  pc = 0x41b599; continue;
 }
 0x41b599 => {
  { cpu.and8(cpu.r8(0x58c431u32), 0x1u8); }
  if cpu.zf { pc = 0x41b5a9; continue; }
  pc = 0x41b5a2; continue;
 }
 0x41b5a2 => {
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.push(0x41b5a6); cpu.call(t); if cpu.returned_to != 0x41b5a6 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x41b5bf; continue;
 }
 0x41b5a9 => {
  { cpu.and8(cpu.r8(0x58c431u32), 0x2u8); }
  if cpu.zf { pc = 0x41b5b9; continue; }
  pc = 0x41b5b2; continue;
 }
 0x41b5b2 => {
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.push(0x41b5b6); cpu.call(t); if cpu.returned_to != 0x41b5b6 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x41b5bf; continue;
 }
 0x41b5b9 => {
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.push(0x41b5bc); cpu.call(t); if cpu.returned_to != 0x41b5bc { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x41b5bf; continue;
 }
 0x41b5bf => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1au32); cpu.esi = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x41b4e1; continue;
 }
 0x41b5cc => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x41b5d7; continue; }
  pc = 0x41b5d0; continue;
 }
 0x41b5d0 => {
  { let v: u8 = 0xffu8; cpu.w8(0x4e15b6u32, v); }
  pc = 0x41b5d7; continue;
 }
 0x41b5d7 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x41b5ef; continue; }
  pc = 0x41b5e3; continue;
 }
 0x41b5e3 => {
  { let v: u32 = cpu.r32(0x4d6752u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x58c435u32, v); }
  pc = 0x41b5ef; continue;
 }
 0x41b5ef => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b5f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b5f0); }
 let mut pc: u32 = 0x41b5f0;
 loop { match pc {
 0x41b5f0 => {
  { let v: u8 = 0xffu8; cpu.w8(0x4e15b6u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b5f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b5f8); }
 let mut pc: u32 = 0x41b5f8;
 loop { match pc {
 0x41b5f8 => {
  { cpu.sub32(cpu.r32(0x58c49du32), 0x0u32); }
  if cpu.zf { pc = 0x41b612; continue; }
  pc = 0x41b601; continue;
 }
 0x41b601 => {
  { cpu.sub8(cpu.r8(0x58c633u32), 0x0u8); }
  if cpu.zf { pc = 0x41b612; continue; }
  pc = 0x41b60a; continue;
 }
 0x41b60a => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41b612 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x41b627; continue; }
  pc = 0x41b61e; continue;
 }
 0x41b61e => {
  { cpu.sub8(cpu.r8(0x58c434u32), 0x0u8); }
  if !cpu.zf { pc = 0x41b642; continue; }
  pc = 0x41b627; continue;
 }
 0x41b627 => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x41b642; continue; }
  pc = 0x41b630; continue;
 }
 0x41b630 => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x41b642; continue; }
  pc = 0x41b639; continue;
 }
 0x41b639 => {
  { cpu.sub8(cpu.r8(0x4b929eu32), 0x0u8); }
  if cpu.zf { pc = 0x41b649; continue; }
  pc = 0x41b642; continue;
 }
 0x41b642 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  pc = 0x41b649; continue;
 }
 0x41b649 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b64a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b64a); }
 let mut pc: u32 = 0x41b64a;
 loop { match pc {
 0x41b64a => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bbu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b65f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b65f); }
 let mut pc: u32 = 0x41b65f;
 loop { match pc {
 0x41b65f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b66a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b66a); }
 let mut pc: u32 = 0x41b66a;
 loop { match pc {
 0x41b66a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b66b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b66b); }
 let mut pc: u32 = 0x41b66b;
 loop { match pc {
 0x41b66b => {
  { let v: u32 = 0x2000u32; cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b671(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b671); }
 let mut pc: u32 = 0x41b671;
 loop { match pc {
 0x41b671 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4e1fe6u32); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  pc = 0x41b690; continue;
 }
 0x41b690 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x41b690; continue; }
  pc = 0x41b69c; continue;
 }
 0x41b69c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x4e1fe6u32; cpu.esi = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b6f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b6f1); }
 let mut pc: u32 = 0x41b6f1;
 loop { match pc {
 0x41b6f1 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = 0x4e1fe6u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u16 = 0xau16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x41b70a; continue;
 }
 0x41b70a => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x41b70a; continue; }
  pc = 0x41b718; continue;
 }
 0x41b718 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b71a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b71a); }
 let mut pc: u32 = 0x41b71a;
 loop { match pc {
 0x41b71a => {
  { cpu.pushad(); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = 0x4e1fe6u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x41b733; continue;
 }
 0x41b733 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.sub8((cpu.edx as u8), 0x9u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41b741; continue; }
  pc = 0x41b73c; continue;
 }
 0x41b73c => {
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x41u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x41b744; continue;
 }
 0x41b741 => {
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x41b744; continue;
 }
 0x41b744 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x41b733; continue; }
  pc = 0x41b749; continue;
 }
 0x41b749 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b74b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b74b); }
 let mut pc: u32 = 0x41b74b;
 loop { match pc {
 0x41b74b => {
  { cpu.pushad(); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = 0x4e1fe6u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5928afu32, v); }
  { cpu.sub32(cpu.edx, 0x0u32); }
  if !cpu.sf { pc = 0x41b771; continue; }
  pc = 0x41b767; continue;
 }
 0x41b767 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5928afu32, v); }
  pc = 0x41b771; continue;
 }
 0x41b771 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  pc = 0x41b77a; continue;
 }
 0x41b77a => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x41b77a; continue; }
  pc = 0x41b786; continue;
 }
 0x41b786 => {
  { cpu.sub8(cpu.r8(0x5928afu32), 0x0u8); }
  if cpu.zf { pc = 0x41b792; continue; }
  pc = 0x41b78f; continue;
 }
 0x41b78f => {
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x41b792; continue;
 }
 0x41b792 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b794(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b794); }
 let mut pc: u32 = 0x41b794;
 loop { match pc {
 0x41b794 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xac6u32; cpu.eax = v; }
  { cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x41b7a9; continue; }
  pc = 0x41b7a4; continue;
 }
 0x41b7a4 => {
  { let v: u32 = 0xad6u32; cpu.eax = v; }
  pc = 0x41b7a9; continue;
 }
 0x41b7a9 => {
  cpu.push(0x41b7ae); f_0041b7b0(cpu); if cpu.returned_to != 0x41b7ae { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b7b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b7b0); }
 let mut pc: u32 = 0x41b7b0;
 loop { match pc {
 0x41b7b0 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xa3u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x41b7ef; continue; }
  pc = 0x41b7c3; continue;
 }
 0x41b7c3 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x13u32); cpu.esi = v; }
  { let v: u8 = 0x15u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x41b7d7; continue;
 }
 0x41b7d7 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x41b7ea; continue; }
  pc = 0x41b7dc; continue;
 }
 0x41b7dc => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x509333u32.wrapping_add(cpu.eax), v); }
  pc = 0x41b7ea; continue;
 }
 0x41b7ea => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x41b7d7; continue; }
  pc = 0x41b7ef; continue;
 }
 0x41b7ef => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b8fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b8fe); }
 let mut pc: u32 = 0x41b8fe;
 loop { match pc {
 0x41b8fe => {
  { cpu.pushad(); }
  { cpu.pushad(); }
  { let v: u32 = 0x6u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x41b90c); f_0040d400(cpu); if cpu.returned_to != 0x41b90c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x41b920; continue; }
  pc = 0x41b914; continue;
 }
 0x41b914 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x41b91d); f_00403120(cpu); if cpu.returned_to != 0x41b91d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  pc = 0x41b920; continue;
 }
 0x41b920 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b99b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b99b); }
 let mut pc: u32 = 0x41b99b;
 loop { match pc {
 0x41b99b => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6ac8u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x41b9d9; continue; }
  pc = 0x41b9a5; continue;
 }
 0x41b9a5 => {
  { let v: u32 = cpu.r32(0x588830u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6ac8u32); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x58882fu32), 0x0u8); }
  if cpu.zf { pc = 0x41b9c0; continue; }
  pc = 0x41b9ba; continue;
 }
 0x41b9ba => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x41b9c0); f_0041bf5e(cpu); if cpu.returned_to != 0x41b9c0 { return; }
  pc = 0x41b9c0; continue;
 }
 0x41b9c0 => {
  { let v: u16 = cpu.r16(0x4d6988u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d698au32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  cpu.push(0x41b9d3); f_0042119b(cpu); if cpu.returned_to != 0x41b9d3 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x588846u32)); cpu.w32(0x588846u32, v); }
  pc = 0x41b9d9; continue;
 }
 0x41b9d9 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4d6ac8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58882fu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x588830u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041b9f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41b9f6); }
 let mut pc: u32 = 0x41b9f6;
 loop { match pc {
 0x41b9f6 => {
  { let v: u32 = 0x0u32; cpu.w32(0x581b55u32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41ba2f; continue; }
  pc = 0x41ba09; continue;
 }
 0x41ba09 => {
  { cpu.sub32(cpu.r32(0x581b51u32), 0x0u32); }
  if !cpu.zf { pc = 0x41ba1c; continue; }
  pc = 0x41ba12; continue;
 }
 0x41ba12 => {
  { let v: u32 = 0xd9u32; cpu.w32(0x581b55u32, v); }
  pc = 0x41ba1c; continue;
 }
 0x41ba1c => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41ba2f; continue; }
  pc = 0x41ba25; continue;
 }
 0x41ba25 => {
  { let v: u32 = cpu.r32(0x581b55u32).wrapping_add(0x3d0u32); cpu.w32(0x581b55u32, v); }
  pc = 0x41ba2f; continue;
 }
 0x41ba2f => {
  cpu.push(0x41ba34); f_0041b66b(cpu); if cpu.returned_to != 0x41ba34 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x56d22bu32, v); }
  { let v: u32 = 0xceu32; cpu.w32(0x56d22fu32, v); }
  { let v: u32 = 0x2bcu32; cpu.w32(0x56d191u32, v); }
  { let v: u32 = 0xa72u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x57584au32)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x581b55u32)); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41ba70; continue; }
  pc = 0x41ba68; continue;
 }
 0x41ba68 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x81u32); cpu.ebx = v; }
  pc = 0x41ba71; continue;
 }
 0x41ba70 => {
  {  }
  pc = 0x41ba71; continue;
 }
 0x41ba71 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  if cpu.sf { pc = 0x41baa3; continue; }
  pc = 0x41ba77; continue;
 }
 0x41ba77 => {
  { cpu.sub32(cpu.eax, 0x2cau32); }
  if (cpu.sf != cpu.of) { pc = 0x41ba91; continue; }
  pc = 0x41ba7e; continue;
 }
 0x41ba7e => {
  { let v: u32 = 0x586u32; cpu.w32(0x56d191u32, v); }
  { let v: u32 = cpu.sub32(cpu.eax, 0x2cau32); cpu.eax = v; }
  if cpu.sf { pc = 0x41baa3; continue; }
  pc = 0x41ba8f; continue;
 }
 0x41ba8f => {
  pc = 0x41ba97; continue;
 }
 0x41ba91 => {
  { let v: u32 = cpu.add32(cpu.r32(0x56d191u32), cpu.eax); cpu.w32(0x56d191u32, v); }
  pc = 0x41ba97; continue;
 }
 0x41ba97 => {
  { let v: u32 = 0x5dcu32; cpu.w32(0x4d6b00u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41baa3 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041bbbd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41bbbd); }
 let mut pc: u32 = 0x41bbbd;
 loop { match pc {
 0x41bbbd => {
  cpu.push(0x41bbc2); f_0041bc15(cpu); if cpu.returned_to != 0x41bbc2 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6ab8u32, v); }
  { let v: u32 = cpu.r32(0x4d6b00u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 10u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bfcu32, v); }
  { let v: u32 = cpu.r32(0x56d191u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 10u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x56d18du32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bbf8; continue; }
  pc = 0x41bbf1; continue;
 }
 0x41bbf1 => {
  cpu.push(0x41bbf6); f_0041bca9(cpu); if cpu.returned_to != 0x41bbf6 { return; }
  pc = 0x41bc0f; continue;
 }
 0x41bbf8 => {
  { cpu.sub32(cpu.r32(0x581b51u32), 0x0u32); }
  if !cpu.zf { pc = 0x41bc08; continue; }
  pc = 0x41bc01; continue;
 }
 0x41bc01 => {
  cpu.push(0x41bc06); f_0041be0d(cpu); if cpu.returned_to != 0x41bc06 { return; }
  pc = 0x41bc0f; continue;
 }
 0x41bc08 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x41bc0e); f_0041be0d(cpu); if cpu.returned_to != 0x41bc0e { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x41bc0f; continue;
 }
 0x41bc0f => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6abcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041bc15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41bc15); }
 let mut pc: u32 = 0x41bc15;
 loop { match pc {
 0x41bc15 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b3cu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a38u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x61b0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b44u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x9c50u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a30u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4090u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5824dbu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b4cu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0xd630u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6accu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x800u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6adcu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x500u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6afcu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x2710u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b0cu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b10u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b18u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x640u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5cf8f3u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0xd630u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bca8; continue; }
  pc = 0x41bc9e; continue;
 }
 0x41bc9e => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c24u32, v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x20000u32); cpu.eax = v; }
  pc = 0x41bca8; continue;
 }
 0x41bca8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041bca9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41bca9); }
 let mut pc: u32 = 0x41bca9;
 loop { match pc {
 0x41bca9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5ba95cu32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bcbc; continue; }
  pc = 0x41bcb7; continue;
 }
 0x41bcb7 => {
  { let v: u32 = cpu.eax.wrapping_add(0x42cc0u32); cpu.eax = v; }
  pc = 0x41bcbc; continue;
 }
 0x41bcbc => {
  { let v: u32 = cpu.eax.wrapping_add(0x16440u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ba974u32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bcd4; continue; }
  pc = 0x41bccf; continue;
 }
 0x41bccf => {
  { let v: u32 = cpu.eax.wrapping_add(0x16440u32); cpu.eax = v; }
  pc = 0x41bcd4; continue;
 }
 0x41bcd4 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5bc430u32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bce7; continue; }
  pc = 0x41bce2; continue;
 }
 0x41bce2 => {
  { let v: u32 = cpu.eax.wrapping_add(0x87c0u32); cpu.eax = v; }
  pc = 0x41bce7; continue;
 }
 0x41bce7 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2d40u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5bc428u32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bcff; continue; }
  pc = 0x41bcfa; continue;
 }
 0x41bcfa => {
  { let v: u32 = cpu.eax.wrapping_add(0x2d40u32); cpu.eax = v; }
  pc = 0x41bcff; continue;
 }
 0x41bcff => {
  { let v: u32 = cpu.eax; cpu.w32(0x5c4fc8u32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bd17; continue; }
  pc = 0x41bd0d; continue;
 }
 0x41bd0d => {
  { let v: u32 = cpu.eax.wrapping_add(0x32130u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1e10u32); cpu.eax = v; }
  pc = 0x41bd17; continue;
 }
 0x41bd17 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10b10u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5c4fccu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bd34; continue; }
  pc = 0x41bd2f; continue;
 }
 0x41bd2f => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x41bd34; continue;
 }
 0x41bd34 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5c4fd0u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bd4c; continue; }
  pc = 0x41bd47; continue;
 }
 0x41bd47 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x41bd4c; continue;
 }
 0x41bd4c => {
  { let v: u32 = cpu.eax; cpu.w32(0x5c4fd4u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bd64; continue; }
  pc = 0x41bd5f; continue;
 }
 0x41bd5f => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x41bd64; continue;
 }
 0x41bd64 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5c4fd8u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bd7c; continue; }
  pc = 0x41bd77; continue;
 }
 0x41bd77 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x41bd7c; continue;
 }
 0x41bd7c => {
  { let v: u32 = cpu.eax; cpu.w32(0x5c4fdcu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bd94; continue; }
  pc = 0x41bd8f; continue;
 }
 0x41bd8f => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x41bd94; continue;
 }
 0x41bd94 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5c4fe0u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bdac; continue; }
  pc = 0x41bda7; continue;
 }
 0x41bda7 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x41bdac; continue;
 }
 0x41bdac => {
  { let v: u32 = cpu.eax; cpu.w32(0x5c4fe4u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bdc4; continue; }
  pc = 0x41bdbf; continue;
 }
 0x41bdbf => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x41bdc4; continue;
 }
 0x41bdc4 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5c4fe8u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bddc; continue; }
  pc = 0x41bdd7; continue;
 }
 0x41bdd7 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x41bddc; continue;
 }
 0x41bddc => {
  { let v: u32 = cpu.eax; cpu.w32(0x5c4fecu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8c0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bdf4; continue; }
  pc = 0x41bdef; continue;
 }
 0x41bdef => {
  { let v: u32 = cpu.eax.wrapping_add(0x1a40u32); cpu.eax = v; }
  pc = 0x41bdf4; continue;
 }
 0x41bdf4 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5ba954u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x7800u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41be0c; continue; }
  pc = 0x41be07; continue;
 }
 0x41be07 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x16800u32); cpu.eax = v; }
  pc = 0x41be0c; continue;
 }
 0x41be0c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041be0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41be0d); }
 let mut pc: u32 = 0x41be0d;
 loop { match pc {
 0x41be0d => {
  { let v: u32 = 0x3635u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5824b4u32, v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x5824b4u32), 4u32); cpu.w32(0x5824b4u32, v); }
  cpu.push(0x41be24); f_00421137(cpu); if cpu.returned_to != 0x41be24 { return; }
  if cpu.cf { pc = 0x41c694; continue; }
  pc = 0x41be2a; continue;
 }
 0x41be2a => {
  { let v: u32 = cpu.eax; cpu.w32(0x5824b8u32, v); }
  cpu.push(0x41be34); f_0041bca9(cpu); if cpu.returned_to != 0x41be34 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6c14u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41c694 => {
  { let v: u32 = cpu.r32(0x5addecu32); cpu.edx = v; }
  cpu.push(0x41c69f); f_004211b6(cpu); if cpu.returned_to != 0x41c69f { return; }
  { let v: u32 = cpu.r32(0x4d6ad8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041be3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41be3a); }
 let mut pc: u32 = 0x41be3a;
 loop { match pc {
 0x41be3a => {
  cpu.push(0x41be3f); f_00421170(cpu); if cpu.returned_to != 0x41be3f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041be7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41be7c); }
 let mut pc: u32 = 0x41be7c;
 loop { match pc {
 0x41be7c => {
  { let v: u16 = cpu.r16(0x5824c4u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5824c6u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.esi as u16), (cpu.edi as u16)); }
  if cpu.zf { pc = 0x41be94; continue; }
  pc = 0x41be8f; continue;
 }
 0x41be8f => {
  cpu.push(0x41be94); f_0042119b(cpu); if cpu.returned_to != 0x41be94 { return; }
  pc = 0x41be94; continue;
 }
 0x41be94 => {
  { let v: u16 = cpu.r16(0x4d698eu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d698cu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  cpu.push(0x41bea7); f_0042119b(cpu); if cpu.returned_to != 0x41bea7 { return; }
  cpu.push(0x41beac); f_0041be3a(cpu); if cpu.returned_to != 0x41beac { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041bef7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41bef7); }
 let mut pc: u32 = 0x41bef7;
 loop { match pc {
 0x41bef7 => {
  { let v: u16 = 0x2bu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x581a4eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f8du32, v); }
  cpu.push(0x41bf0c); f_0041baa5(cpu); if cpu.returned_to != 0x41bf0c { return; }
  cpu.push(0x41bf11); f_0041bfd2(cpu); if cpu.returned_to != 0x41bf11 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x41bf24); f_0041c1a0(cpu); if cpu.returned_to != 0x41bf24 { return; }
  cpu.push(0x41bf29); f_0041bfd4(cpu); if cpu.returned_to != 0x41bf29 { return; }
  cpu.push(0x41bf2e); f_004478a1(cpu); if cpu.returned_to != 0x41bf2e { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x592bd1u32, v); }
  { let v: u32 = cpu.r32(0x5824b8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5824b4u32); cpu.ecx = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x41bf4b; continue; }
  pc = 0x41bf46; continue;
 }
 0x41bf46 => {
  cpu.push(0x41bf4b); f_0041bf5d(cpu); if cpu.returned_to != 0x41bf4b { return; }
  pc = 0x41bf4b; continue;
 }
 0x41bf4b => {
  { let v: u32 = cpu.r32(0x5824c0u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5824bcu32); cpu.ecx = v; }
  cpu.push(0x41bf5c); f_0041bf5d(cpu); if cpu.returned_to != 0x41bf5c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041bf5d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41bf5d); }
 let mut pc: u32 = 0x41bf5d;
 loop { match pc {
 0x41bf5d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041bf5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41bf5e); }
 let mut pc: u32 = 0x41bf5e;
 loop { match pc {
 0x41bf5e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041bf5f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41bf5f); }
 let mut pc: u32 = 0x41bf5f;
 loop { match pc {
 0x41bf5f => {
  { cpu.sub8(cpu.r8(0x592bd0u32), 0x0u8); }
  if !cpu.zf { pc = 0x41bfd1; continue; }
  pc = 0x41bf68; continue;
 }
 0x41bf68 => {
  { cpu.sub8(cpu.r8(0x592bd1u32), 0x0u8); }
  if cpu.zf { pc = 0x41bfd1; continue; }
  pc = 0x41bf71; continue;
 }
 0x41bf71 => {
  cpu.push(0x41bf76); f_004010e0(cpu); if cpu.returned_to != 0x41bf76 { return; }
  { let v: u32 = cpu.r32(0x5824b8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5824b4u32); cpu.ecx = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x41bf8c; continue; }
  pc = 0x41bf87; continue;
 }
 0x41bf87 => {
  cpu.push(0x41bf8c); f_0041bf5e(cpu); if cpu.returned_to != 0x41bf8c { return; }
  pc = 0x41bf8c; continue;
 }
 0x41bf8c => {
  { let v: u32 = cpu.r32(0x5824c0u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5824bcu32); cpu.ecx = v; }
  cpu.push(0x41bf9d); f_0041bf5e(cpu); if cpu.returned_to != 0x41bf9d { return; }
  cpu.push(0x41bfa2); f_0044787e(cpu); if cpu.returned_to != 0x41bfa2 { return; }
  cpu.push(0x41bfa7); f_0041bfe1(cpu); if cpu.returned_to != 0x41bfa7 { return; }
  cpu.push(0x41bfac); f_0041c266(cpu); if cpu.returned_to != 0x41bfac { return; }
  cpu.push(0x41bfb1); f_00417c69(cpu); if cpu.returned_to != 0x41bfb1 { return; }
  cpu.push(0x41bfb6); f_004030c0(cpu); if cpu.returned_to != 0x41bfb6 { return; }
  cpu.push(0x41bfbb); f_0041be7c(cpu); if cpu.returned_to != 0x41bfbb { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x41bfc5); f_00414ada(cpu); if cpu.returned_to != 0x41bfc5 { return; }
  cpu.push(0x41bfca); f_0041bfd3(cpu); if cpu.returned_to != 0x41bfca { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x592bd0u32, v); }
  pc = 0x41bfd1; continue;
 }
 0x41bfd1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041bfd2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41bfd2); }
 let mut pc: u32 = 0x41bfd2;
 loop { match pc {
 0x41bfd2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041bfd3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41bfd3); }
 let mut pc: u32 = 0x41bfd3;
 loop { match pc {
 0x41bfd3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041bfd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41bfd4); }
 let mut pc: u32 = 0x41bfd4;
 loop { match pc {
 0x41bfd4 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x41bfe0); f_00421286(cpu); if cpu.returned_to != 0x41bfe0 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041bfe1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41bfe1); }
 let mut pc: u32 = 0x41bfe1;
 loop { match pc {
 0x41bfe1 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x41bfea); f_00421286(cpu); if cpu.returned_to != 0x41bfea { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041bfed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41bfed); }
 let mut pc: u32 = 0x41bfed;
 loop { match pc {
 0x41bfed => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6828u32); cpu.ebx = v; }
  cpu.push(0x41bff9); f_004211d4(cpu); if cpu.returned_to != 0x41bff9 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x581b46u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c00e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c00e); }
 let mut pc: u32 = 0x41c00e;
 loop { match pc {
 0x41c00e => {
  { cpu.df = false; }
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x56d394u32, v); }
  { cpu.sub8(cpu.r8(0x5820fbu32), 0x0u8); }
  if !cpu.zf { pc = 0x41c020; continue; }
  pc = 0x41c020; continue;
 }
 0x41c020 => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if !cpu.zf { pc = 0x41c032; continue; }
  pc = 0x41c029; continue;
 }
 0x41c029 => {
  { cpu.sub8(cpu.r8(0x575531u32), 0x0u8); }
  if cpu.zf { pc = 0x41c032; continue; }
  pc = 0x41c032; continue;
 }
 0x41c032 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x56d38fu32)); cpu.w32(0x56d38fu32, v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x4e15a7u32)); cpu.w8(0x4e15a7u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5c4e5bu32)); cpu.w32(0x5c4e5bu32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5c4e9cu32)); cpu.w32(0x5c4e9cu32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x4d69b2u32)); cpu.w32(0x4d69b2u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x56c6e6u32)); cpu.w32(0x56c6e6u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x581a38u32)); cpu.w32(0x581a38u32, v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x575530u32)); cpu.w8(0x575530u32, v); }
  { let v: u8 = cpu.r8(0x575530u32) & 0x1fu8; cpu.w8(0x575530u32, v); }
  { cpu.sub32(cpu.r32(0x56c737u32), 0x0u32); }
  if cpu.zf { pc = 0x41c078; continue; }
  pc = 0x41c072; continue;
 }
 0x41c072 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x56c737u32)); cpu.w32(0x56c737u32, v); }
  pc = 0x41c078; continue;
 }
 0x41c078 => {
  { cpu.sub32(cpu.r32(0x5ddbccu32), 0x0u32); }
  if !cpu.zf { pc = 0x41c0b6; continue; }
  pc = 0x41c081; continue;
 }
 0x41c081 => {
  { cpu.sub32(cpu.r32(0x581b4du32), 0x0u32); }
  if !cpu.zf { pc = 0x41c0b6; continue; }
  pc = 0x41c08a; continue;
 }
 0x41c08a => {
  { let v: u16 = cpu.inc16(cpu.r16(0x581b4bu32)); cpu.w16(0x581b4bu32, v); }
  { cpu.sub16(cpu.r16(0x581b4bu32), 0x46u16); }
  if !cpu.zf { pc = 0x41c0b6; continue; }
  pc = 0x41c09b; continue;
 }
 0x41c09b => {
  { cpu.sub32(cpu.r32(0x4b94d0u32), 0x8c9fu32); }
  if (cpu.sf == cpu.of) { pc = 0x41c0ad; continue; }
  pc = 0x41c0a7; continue;
 }
 0x41c0a7 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4b94d0u32)); cpu.w32(0x4b94d0u32, v); }
  pc = 0x41c0ad; continue;
 }
 0x41c0ad => {
  { let v: u16 = 0x0u16; cpu.w16(0x581b4bu32, v); }
  pc = 0x41c0b6; continue;
 }
 0x41c0b6 => {
  { cpu.sub16(cpu.r16(0x4d699cu32), 0x0u16); }
  if !cpu.zf { pc = 0x41c0c9; continue; }
  pc = 0x41c0c0; continue;
 }
 0x41c0c0 => {
  { cpu.sub8(cpu.r8(0x4e1f92u32), 0x0u8); }
  if cpu.zf { pc = 0x41c0ed; continue; }
  pc = 0x41c0c9; continue;
 }
 0x41c0c9 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1f91u32)); cpu.w8(0x4e1f91u32, v); }
  if !cpu.zf { pc = 0x41c0e4; continue; }
  pc = 0x41c0d1; continue;
 }
 0x41c0d1 => {
  { let v: u8 = cpu.r8(0x581a74u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.xor8(cpu.r8(0x581a74u32), 0x7u8); cpu.w8(0x581a74u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f91u32, v); }
  pc = 0x41c0ed; continue;
 }
 0x41c0e4 => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f90u32, v); }
  pc = 0x41c0ed; continue;
 }
 0x41c0ed => {
  { let v: u8 = 0x0u8; cpu.w8(0x56d394u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c0f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c0f6); }
 let mut pc: u32 = 0x41c0f6;
 loop { match pc {
 0x41c0f6 => {
  cpu.push(0x41c0fb); f_0041c2ad(cpu); if cpu.returned_to != 0x41c0fb { return; }
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x7u8); }
  if !cpu.zf { cpu.poll(); pc = 0x41c0f6; continue; }
  pc = 0x41c104; continue;
 }
 0x41c104 => {
  cpu.push(0x41c109); f_0041c2ad(cpu); if cpu.returned_to != 0x41c109 { return; }
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x7u8); }
  if !cpu.zf { pc = 0x41c119; continue; }
  pc = 0x41c112; continue;
 }
 0x41c112 => {
  cpu.push(0x41c117); f_0041c122(cpu); if cpu.returned_to != 0x41c117 { return; }
  if cpu.cf { pc = 0x41c119; continue; }
  pc = 0x41c119; continue;
 }
 0x41c119 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c11a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c11a); }
 let mut pc: u32 = 0x41c11a;
 loop { match pc {
 0x41c11a => {
  cpu.push(0x41c11f); f_0041c122(cpu); if cpu.returned_to != 0x41c11f { return; }
  if cpu.cf { pc = 0x41c121; continue; }
  pc = 0x41c121; continue;
 }
 0x41c121 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c162(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c162); }
 let mut pc: u32 = 0x41c162;
 loop { match pc {
 0x41c162 => {
  { let v = (cpu.ebx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.edx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  pc = 0x41c168; continue;
 }
 0x41c168 => {
  cpu.push(0x41c16d); f_0042125a(cpu); if cpu.returned_to != 0x41c16d { return; }
  if cpu.zf { pc = 0x41c176; continue; }
  pc = 0x41c16f; continue;
 }
 0x41c16f => {
  cpu.push(0x41c174); f_00421264(cpu); if cpu.returned_to != 0x41c174 { return; }
  cpu.poll(); pc = 0x41c168; continue;
 }
 0x41c176 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c17d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c17d); }
 let mut pc: u32 = 0x41c17d;
 loop { match pc {
 0x41c17d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x4d681cu32); cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6814u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d681cu32, v); }
  cpu.push(0x41c193); f_004173ce(cpu); if cpu.returned_to != 0x41c193 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d681cu32, v); }
  cpu.push(0x41c19e); f_004173ce(cpu); if cpu.returned_to != 0x41c19e { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c1a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c1a0); }
 let mut pc: u32 = 0x41c1a0;
 loop { match pc {
 0x41c1a0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fa4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5ce8d2u32, v); }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x41c1bf); f_0041c9df(cpu); if cpu.returned_to != 0x41c1bf { return; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0x9f8u32; cpu.edx = v; }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  cpu.push(0x41c1d5); f_0041c9df(cpu); if cpu.returned_to != 0x41c1d5 { return; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0x638u32; cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x41c1eb); f_0041c9df(cpu); if cpu.returned_to != 0x41c1eb { return; }
  { let v: u32 = 0x98u32; cpu.eax = v; }
  { let v: u32 = 0x5cu32; cpu.ebx = v; }
  cpu.push(0x41c1fa); f_0041c26e(cpu); if cpu.returned_to != 0x41c1fa { return; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = 0xfu32; cpu.eax = v; }
  cpu.push(0x41c20e); f_0041c9df(cpu); if cpu.returned_to != 0x41c20e { return; }
  { let v: u32 = 0x13u32; cpu.eax = v; }
  { let v: u32 = 0x5000u32; cpu.edx = v; }
  cpu.push(0x41c21d); f_0041c9df(cpu); if cpu.returned_to != 0x41c21d { return; }
  { cpu.sub8(cpu.r8(0x581a72u32), 0x0u8); }
  if !cpu.zf { pc = 0x41c256; continue; }
  pc = 0x41c226; continue;
 }
 0x41c226 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  { let v: u32 = 0x7fu32; cpu.ecx = v; }
  cpu.push(0x41c235); f_0041c9df(cpu); if cpu.returned_to != 0x41c235 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c64u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d69dcu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fc4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x581a72u32, v); }
  pc = 0x41c256; continue;
 }
 0x41c256 => {
  { let v: u8 = 0x0u8; cpu.w8(0x56d22au32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b5u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c266(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c266); }
 let mut pc: u32 = 0x41c266;
 loop { match pc {
 0x41c266 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b5u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c26e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c26e); }
 let mut pc: u32 = 0x41c26e;
 loop { match pc {
 0x41c26e => {
  { cpu.sub8(cpu.r8(0x4e15b5u32), 0x0u8); }
  if cpu.zf { pc = 0x41c2ac; continue; }
  pc = 0x41c277; continue;
 }
 0x41c277 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6752u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6754u32, v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 3u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.edx as u16), 3u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x2u8); }
  if !cpu.zf { pc = 0x41c2a7; continue; }
  pc = 0x41c2a1; continue;
 }
 0x41c2a1 => {
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x41c2a7; continue;
 }
 0x41c2a7 => {
  cpu.push(0x41c2ac); f_0041c9df(cpu); if cpu.returned_to != 0x41c2ac { return; }
  pc = 0x41c2ac; continue;
 }
 0x41c2ac => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c2ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c2ad); }
 let mut pc: u32 = 0x41c2ad;
 loop { match pc {
 0x41c2ad => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.push(0x41c2b7); f_0041c9df(cpu); if cpu.returned_to != 0x41c2b7 { return; }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x2u8); }
  if !cpu.zf { pc = 0x41c2cf; continue; }
  pc = 0x41c2c0; continue;
 }
 0x41c2c0 => {
  { cpu.sub8(cpu.r8(0x4e1fa4u32), 0x0u8); }
  if !cpu.zf { pc = 0x41c2eb; continue; }
  pc = 0x41c2c9; continue;
 }
 0x41c2c9 => {
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x41c2cf; continue;
 }
 0x41c2cf => {
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 3u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 3u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 3u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6752u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6754u32, v); }
  pc = 0x41c2f9; continue;
 }
 0x41c2eb => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6752u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d6754u32, v); }
  pc = 0x41c2f9; continue;
 }
 0x41c2f9 => {
  cpu.push(0x41c2fe); f_0041c31d(cpu); if cpu.returned_to != 0x41c2fe { return; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4e15a5u32, v); }
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x41c310; continue; }
  pc = 0x41c309; continue;
 }
 0x41c309 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bau32, v); }
  pc = 0x41c310; continue;
 }
 0x41c310 => {
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x41c31c; continue; }
  pc = 0x41c315; continue;
 }
 0x41c315 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15bbu32, v); }
  pc = 0x41c31c; continue;
 }
 0x41c31c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c31d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c31d); }
 let mut pc: u32 = 0x41c31d;
 loop { match pc {
 0x41c31d => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x4u32); }
  if cpu.zf { pc = 0x41c39e; continue; }
  pc = 0x41c329; continue;
 }
 0x41c329 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4e15a5u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x58c433u32, v); }
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x41c344; continue; }
  pc = 0x41c339; continue;
 }
 0x41c339 => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x41c344; continue; }
  pc = 0x41c33d; continue;
 }
 0x41c33d => {
  { let v: u8 = cpu.r8(0x58c432u32) | 0x1u8; cpu.w8(0x58c432u32, v); }
  pc = 0x41c344; continue;
 }
 0x41c344 => {
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x41c35d; continue; }
  pc = 0x41c349; continue;
 }
 0x41c349 => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x41c39d; continue; }
  pc = 0x41c34d; continue;
 }
 0x41c34d => {
  { let v: u8 = cpu.or8(cpu.r8(0x58c432u32), 0x2u8); cpu.w8(0x58c432u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c463u32, v); }
  pc = 0x41c39d; continue;
 }
 0x41c35d => {
  { cpu.sub8(cpu.r8(0x58c463u32), 0x0u8); }
  if !cpu.zf { pc = 0x41c39d; continue; }
  pc = 0x41c366; continue;
 }
 0x41c366 => {
  { let v: u8 = 0x1u8; cpu.w8(0x58c463u32, v); }
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x58c43bu32, v); }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x58c43du32, v); }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x58c496u32, v); }
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x58c494u32, v); }
  pc = 0x41c39d; continue;
 }
 0x41c39d => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x41c39e; continue;
 }
 0x41c39e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c458(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c458); }
 let mut pc: u32 = 0x41c458;
 loop { match pc {
 0x41c458 => {
  { cpu.sub8(cpu.r8(0x4e1fcbu32), 0x0u8); }
  if !cpu.zf { pc = 0x41c462; continue; }
  pc = 0x41c461; continue;
 }
 0x41c461 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41c462 => {
  { let v: u16 = 0x10eu16; cpu.w16(0x56d42bu32, v); }
  { let v: u16 = 0xb6u16; cpu.w16(0x56d42du32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41c495; continue; }
  pc = 0x41c47d; continue;
 }
 0x41c47d => {
  { let v: u16 = 0x21cu16; cpu.w16(0x56d42bu32, v); }
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x24u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x56d42du32, v); }
  pc = 0x41c495; continue;
 }
 0x41c495 => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x56d42bu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41c4a5; continue; }
  pc = 0x41c49e; continue;
 }
 0x41c49e => {
  { let v: u16 = cpu.r16(0x56d42bu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x41c4a5; continue;
 }
 0x41c4a5 => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x56d42du32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41c4b5; continue; }
  pc = 0x41c4ae; continue;
 }
 0x41c4ae => {
  { let v: u16 = cpu.r16(0x56d42du32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x41c4b5; continue;
 }
 0x41c4b5 => {
  { cpu.sub16((cpu.ecx as u16), 0x2u16); }
  if (cpu.sf == cpu.of) { pc = 0x41c4bf; continue; }
  pc = 0x41c4bb; continue;
 }
 0x41c4bb => {
  { let v: u16 = 0x2u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x41c4bf; continue;
 }
 0x41c4bf => {
  { cpu.sub16((cpu.edx as u16), 0x2u16); }
  if (cpu.sf == cpu.of) { pc = 0x41c4c9; continue; }
  pc = 0x41c4c5; continue;
 }
 0x41c4c5 => {
  { let v: u16 = 0x2u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x41c4c9; continue;
 }
 0x41c4c9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c4ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c4ca); }
 let mut pc: u32 = 0x41c4ca;
 loop { match pc {
 0x41c4ca => {
  { cpu.sub8(cpu.r8(0x5b3855u32), 0x0u8); }
  if !cpu.zf { pc = 0x41c4d4; continue; }
  pc = 0x41c4d3; continue;
 }
 0x41c4d3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41c4d4 => {
  { cpu.sub32(cpu.r32(0x5b3588u32), 0x0u32); }
  if !cpu.zf { pc = 0x41c4de; continue; }
  pc = 0x41c4dd; continue;
 }
 0x41c4dd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41c4de => {
  { let v: u16 = 0x2u16; cpu.w16(0x57553au32, v); }
  { let v: u16 = 0x2u16; cpu.w16(0x57553cu32, v); }
  { let v: u16 = 0x10eu16; cpu.w16(0x56d42bu32, v); }
  { let v: u16 = 0xb6u16; cpu.w16(0x56d42du32, v); }
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x41c523; continue; }
  pc = 0x41c50b; continue;
 }
 0x41c50b => {
  { let v: u16 = 0x21cu16; cpu.w16(0x56d42bu32, v); }
  { let v: u32 = cpu.r32(0x4d6bccu32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x24u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x56d42du32, v); }
  pc = 0x41c523; continue;
 }
 0x41c523 => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x56d42bu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41c533; continue; }
  pc = 0x41c52c; continue;
 }
 0x41c52c => {
  { let v: u16 = cpu.r16(0x56d42bu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x41c533; continue;
 }
 0x41c533 => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x56d42du32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41c543; continue; }
  pc = 0x41c53c; continue;
 }
 0x41c53c => {
  { let v: u16 = cpu.r16(0x56d42du32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x41c543; continue;
 }
 0x41c543 => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x57553au32)); }
  if (cpu.sf == cpu.of) { pc = 0x41c553; continue; }
  pc = 0x41c54c; continue;
 }
 0x41c54c => {
  { let v: u16 = cpu.r16(0x57553au32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x41c553; continue;
 }
 0x41c553 => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x57553cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x41c563; continue; }
  pc = 0x41c55c; continue;
 }
 0x41c55c => {
  { let v: u16 = cpu.r16(0x57553cu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x41c563; continue;
 }
 0x41c563 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c564(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c564); }
 let mut pc: u32 = 0x41c564;
 loop { match pc {
 0x41c564 => {
  { cpu.pushad(); }
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x41c597; continue; }
  pc = 0x41c56a; continue;
 }
 0x41c56a => {
  { cpu.and8(cpu.r8(0x581a73u32), 0x1u8); }
  if cpu.zf { pc = 0x41c608; continue; }
  pc = 0x41c577; continue;
 }
 0x41c577 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x581a6au32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x581a6cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x581a38u32, v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x581a73u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41c597 => {
  { cpu.and8(cpu.r8(0x581a73u32), 0x1u8); }
  if !cpu.zf { pc = 0x41c608; continue; }
  pc = 0x41c5a0; continue;
 }
 0x41c5a0 => {
  { cpu.sub32(cpu.r32(0x581a38u32), 0x14u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x41c5ef; continue; }
  pc = 0x41c5a9; continue;
 }
 0x41c5a9 => {
  { let v: u16 = cpu.sub16((cpu.ecx as u16), cpu.r16(0x581a6au32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x41c5b5; continue; }
  pc = 0x41c5b2; continue;
 }
 0x41c5b2 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x41c5b5; continue;
 }
 0x41c5b5 => {
  { cpu.sub16((cpu.ecx as u16), 0x8u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41c5ef; continue; }
  pc = 0x41c5bb; continue;
 }
 0x41c5bb => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), cpu.r16(0x581a6cu32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x41c5c7; continue; }
  pc = 0x41c5c4; continue;
 }
 0x41c5c4 => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x41c5c7; continue;
 }
 0x41c5c7 => {
  { cpu.sub16((cpu.edx as u16), 0x8u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x41c5ef; continue; }
  pc = 0x41c5cd; continue;
 }
 0x41c5cd => {
  { let v: u16 = cpu.r16(0x4d6752u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x581a48u32, v); }
  { let v: u16 = cpu.r16(0x4d6754u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x581a4au32, v); }
  { let v: u32 = cpu.r32(0x4d6a18u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x581a44u32, v); }
  pc = 0x41c5ef; continue;
 }
 0x41c5ef => {
  { let v: u8 = 0x1u8; cpu.w8(0x581a73u32, v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x581a6au32, v); }
  { let v: u16 = 0xfc18u16; cpu.w16(0x581a6cu32, v); }
  pc = 0x41c608; continue;
 }
 0x41c608 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c60a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c60a); }
 let mut pc: u32 = 0x41c60a;
 loop { match pc {
 0x41c60a => {
  { let v: u32 = 0x4ea988u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4d6accu32); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x41c620); f_00447961(cpu); if cpu.returned_to != 0x41c620 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c622(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c622); }
 let mut pc: u32 = 0x41c622;
 loop { match pc {
 0x41c622 => {
  { let v: u32 = 0x4ea9a1u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4d6adcu32); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x41c638); f_00447961(cpu); if cpu.returned_to != 0x41c638 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4d6adcu32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0x280u32; cpu.ecx = v; }
  pc = 0x41c646; continue;
 }
 0x41c646 => {
  { cpu.lods(2, false); }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x41c646; continue; }
  pc = 0x41c652; continue;
 }
 0x41c652 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c6a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c6a6); }
 let mut pc: u32 = 0x41c6a6;
 loop { match pc {
 0x41c6a6 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x581b42u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x592bd4u32); cpu.edi = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.push(v); }
  { let v = cpu.r32(0x581b42u32); cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x581b4au32, v); }
  { let v = cpu.r16(0x4d6756u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = cpu.r16(0x4d6962u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6756u32, v); }
  cpu.push(0x41c6db); f_0041c743(cpu); if cpu.returned_to != 0x41c6db { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6962u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6756u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x581b4au32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x581b42u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c743(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c743); }
 let mut pc: u32 = 0x41c743;
 loop { match pc {
 0x41c743 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x581b42u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x592bd4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4d6a18u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d301u32), 0x0u8); }
  if cpu.zf { pc = 0x41c765; continue; }
  pc = 0x41c760; continue;
 }
 0x41c760 => {
  { let v: u32 = cpu.eax | 0x80000000u32; cpu.eax = v; }
  pc = 0x41c765; continue;
 }
 0x41c765 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x40cf50u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x40cf50u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4e1570u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x59d8d8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x581b42u32).wrapping_add(0x10u32); cpu.w32(0x581b42u32, v); }
  { let v: u32 = cpu.r32(0x581b42u32) & 0x3ffu32; cpu.w32(0x581b42u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4d6756u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  { let v: u8 = cpu.rcr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.xor8((cpu.ecx as u8), (cpu.eax as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.xor8((cpu.ecx as u8), (cpu.eax as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.xor8((cpu.edx as u8), (cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcr8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcl8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6756u32, v); }
  { cpu.sub8(cpu.r8(0x581b4au32), 0x0u8); }
  if !cpu.zf { pc = 0x41c805; continue; }
  pc = 0x41c7da; continue;
 }
 0x41c7da => {
  { cpu.sub8(cpu.r8(0x54acd1u32), 0x1u8); }
  if !cpu.zf { pc = 0x41c805; continue; }
  pc = 0x41c7e3; continue;
 }
 0x41c7e3 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x581b42u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x592bd4u32); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x581b42u32).wrapping_add(0x2u32); cpu.w32(0x581b42u32, v); }
  { let v: u32 = cpu.and32(cpu.r32(0x581b42u32), 0x3ffu32); cpu.w32(0x581b42u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x41c805; continue;
 }
 0x41c805 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c808(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c808); }
 let mut pc: u32 = 0x41c808;
 loop { match pc {
 0x41c808 => {
  cpu.push(0x41c80d); f_0041c743(cpu); if cpu.returned_to != 0x41c80d { return; }
  f_0041c80d(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c80d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c80d); }
 let mut pc: u32 = 0x41c80d;
 loop { match pc {
 0x41c80d => {
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.zf { pc = 0x41c816; continue; }
  pc = 0x41c814; continue;
 }
 0x41c814 => {
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x41c816; continue;
 }
 0x41c816 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.ecx as u16)); }
  if cpu.cf { pc = 0x41c81e; continue; }
  pc = 0x41c81b; continue;
 }
 0x41c81b => {
  { let a = (cpu.ecx as u16); let b = (cpu.ebx as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } }
  pc = 0x41c81e; continue;
 }
 0x41c81e => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x1u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c845(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c845); }
 let mut pc: u32 = 0x41c845;
 loop { match pc {
 0x41c845 => {
  cpu.push(0x41c84a); f_0041c6a6(cpu); if cpu.returned_to != 0x41c84a { return; }
  cpu.push(0x41c84f); f_0041c80d(cpu); if cpu.returned_to != 0x41c84f { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c901(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c901); }
 let mut pc: u32 = 0x41c901;
 loop { match pc {
 0x41c901 => {
  { cpu.pushad(); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.r32(0x4d6824u32); cpu.push(v); }
  cpu.push(0x41c90f); f_0040d620(cpu); if cpu.returned_to != 0x41c90f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a94u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c919(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c919); }
 let mut pc: u32 = 0x41c919;
 loop { match pc {
 0x41c919 => {
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x41c922); f_0040d370(cpu); if cpu.returned_to != 0x41c922 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6824u32, v); }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x58c6f3u32), 0x0u8); }
  if !cpu.zf { pc = 0x41c964; continue; }
  pc = 0x41c934; continue;
 }
 0x41c934 => {
  { cpu.sub32(cpu.eax, 0x4ea994u32); }
  if !cpu.zf { pc = 0x41c964; continue; }
  pc = 0x41c93b; continue;
 }
 0x41c93b => {
  { let v: u32 = cpu.r32(0x4d6824u32); cpu.eax = v; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { f_0041c653(cpu); return; }
  pc = 0x41c949; continue;
 }
 0x41c949 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x4d6824u32); cpu.eax = v; }
  cpu.push(0x41c956); f_0046c6a3(cpu); if cpu.returned_to != 0x41c956 { return; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x41c95f); f_0040d9b0(cpu); if cpu.returned_to != 0x41c95f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  pc = 0x41c983; continue;
 }
 0x41c964 => {
  { let v: u32 = cpu.r32(0x4d6824u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { f_0041c653(cpu); return; }
  pc = 0x41c971; continue;
 }
 0x41c971 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.r32(0x4d6824u32); cpu.push(v); }
  cpu.push(0x41c980); f_0040d4e0(cpu); if cpu.returned_to != 0x41c980 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x41c983; continue;
 }
 0x41c983 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a94u32, v); }
  { cpu.popad(); }
  cpu.push(0x41c98e); f_0041c997(cpu); if cpu.returned_to != 0x41c98e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x58c6f3u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c997(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c997); }
 let mut pc: u32 = 0x41c997;
 loop { match pc {
 0x41c997 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x4d6824u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x41c9b9; continue; }
  pc = 0x41c9a1; continue;
 }
 0x41c9a1 => {
  { let v = cpu.r32(0x4d6824u32); cpu.push(v); }
  cpu.push(0x41c9ac); f_0040d640(cpu); if cpu.returned_to != 0x41c9ac { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6824u32, v); }
  pc = 0x41c9b9; continue;
 }
 0x41c9b9 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c9df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c9df); }
 let mut pc: u32 = 0x41c9df;
 loop { match pc {
 0x41c9df => {
  cpu.push(0x41c9e4); f_004212a1(cpu); if cpu.returned_to != 0x41c9e4 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041c9e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41c9e5); }
 let mut pc: u32 = 0x41c9e5;
 loop { match pc {
 0x41c9e5 => {
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x4b929fu32), 0x0u8); }
  if !cpu.zf { pc = 0x41ca56; continue; }
  pc = 0x41c9ef; continue;
 }
 0x41c9ef => {
  { cpu.sub8(cpu.r8(0x56d2f9u32), 0x0u8); }
  if !cpu.zf { pc = 0x41ca56; continue; }
  pc = 0x41c9f8; continue;
 }
 0x41c9f8 => {
  { cpu.sub8(cpu.r8(0x56d2fau32), 0x0u8); }
  if !cpu.zf { pc = 0x41ca56; continue; }
  pc = 0x41ca01; continue;
 }
 0x41ca01 => {
  { cpu.sub8(cpu.r8(0x575959u32), 0x0u8); }
  if !cpu.zf { pc = 0x41ca0a; continue; }
  pc = 0x41ca0a; continue;
 }
 0x41ca0a => {
  { cpu.pushad(); }
  cpu.push(0x41ca10); f_00403050(cpu); if cpu.returned_to != 0x41ca10 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x41ca56; continue; }
  pc = 0x41ca15; continue;
 }
 0x41ca15 => {
  { let v: u32 = cpu.r32(0x5ddaa0u32); cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x77u8); }
  if cpu.zf { pc = 0x41ca63; continue; }
  pc = 0x41ca1e; continue;
 }
 0x41ca1e => {
  { cpu.sub8((cpu.eax as u8), 0x57u8); }
  if cpu.zf { pc = 0x41ca63; continue; }
  pc = 0x41ca22; continue;
 }
 0x41ca22 => {
  { cpu.sub8((cpu.eax as u8), 0x48u8); }
  if cpu.zf { pc = 0x41ca2a; continue; }
  pc = 0x41ca26; continue;
 }
 0x41ca26 => {
  { cpu.sub8((cpu.eax as u8), 0x68u8); }
  if !cpu.zf { pc = 0x41ca3c; continue; }
  pc = 0x41ca2a; continue;
 }
 0x41ca2a => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x41ca56; continue; }
  pc = 0x41ca33; continue;
 }
 0x41ca33 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4b929fu32, v); }
  pc = 0x41ca60; continue;
 }
 0x41ca3c => {
  { cpu.sub8(cpu.r8(0x5820fbu32), 0x0u8); }
  if cpu.zf { pc = 0x41ca4e; continue; }
  pc = 0x41ca45; continue;
 }
 0x41ca45 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x41ca56; continue; }
  pc = 0x41ca4e; continue;
 }
 0x41ca4e => {
  { cpu.sub8((cpu.eax as u8), 0x71u8); }
  if cpu.zf { pc = 0x41ca59; continue; }
  pc = 0x41ca52; continue;
 }
 0x41ca52 => {
  { cpu.sub8((cpu.eax as u8), 0x51u8); }
  if cpu.zf { pc = 0x41ca59; continue; }
  pc = 0x41ca56; continue;
 }
 0x41ca56 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41ca59 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56d2f9u32, v); }
  pc = 0x41ca60; continue;
 }
 0x41ca60 => {
  { cpu.cf = true; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41ca63 => {
  { let v: u8 = 0x1u8; cpu.w8(0x576485u32, v); }
  { let v: u8 = !cpu.r8(0x4e15abu32); cpu.w8(0x4e15abu32, v); }
  { let v: u8 = !cpu.r8(0x56d23bu32); cpu.w8(0x56d23bu32, v); }
  { let v: u8 = !cpu.r8(0x575f2bu32); cpu.w8(0x575f2bu32, v); }
  cpu.poll(); pc = 0x41ca56; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041caf5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41caf5); }
 let mut pc: u32 = 0x41caf5;
 loop { match pc {
 0x41caf5 => {
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4d6b3cu32); cpu.edi = v; }
  pc = 0x41cafd; continue;
 }
 0x41cafd => {
  { let v: u32 = 0x4dd54cu32.wrapping_add(cpu.ebp.wrapping_mul(2)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x4d9548u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xfff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4d954au32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ebx as u8), 3u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x41cb2a; continue;
 }
 0x41cb2a => {
  { cpu.lods(1, false); }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.lods(1, false); }
  { let v: u16 = (cpu.eax as u16) & 0x7f7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x7eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0x7f7fu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x18u8); }
  if cpu.zf { pc = 0x41cba9; continue; }
  pc = 0x41cb43; continue;
 }
 0x41cb43 => {
  { cpu.sub8((cpu.eax as u8), 0x19u8); }
  if cpu.zf { pc = 0x41cba9; continue; }
  pc = 0x41cb4b; continue;
 }
 0x41cb4b => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x18u8); }
  if cpu.zf { pc = 0x41cb9f; continue; }
  pc = 0x41cb54; continue;
 }
 0x41cb54 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x19u8); }
  if cpu.zf { pc = 0x41cb9f; continue; }
  pc = 0x41cb5d; continue;
 }
 0x41cb5d => {
  { cpu.sub8((cpu.ebx as u8), 0x18u8); }
  if cpu.zf { pc = 0x41cba7; continue; }
  pc = 0x41cb66; continue;
 }
 0x41cb66 => {
  { cpu.sub8((cpu.ebx as u8), 0x19u8); }
  if cpu.zf { pc = 0x41cba7; continue; }
  pc = 0x41cb6f; continue;
 }
 0x41cb6f => {
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x18u8); }
  if cpu.zf { pc = 0x41cba3; continue; }
  pc = 0x41cb74; continue;
 }
 0x41cb74 => {
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x19u8); }
  if cpu.zf { pc = 0x41cba3; continue; }
  pc = 0x41cb79; continue;
 }
 0x41cb79 => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if cpu.zf { pc = 0x41cba9; continue; }
  pc = 0x41cb7d; continue;
 }
 0x41cb7d => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xbu8); }
  if cpu.zf { pc = 0x41cb9f; continue; }
  pc = 0x41cb82; continue;
 }
 0x41cb82 => {
  { cpu.sub8((cpu.ebx as u8), 0xbu8); }
  if cpu.zf { pc = 0x41cba7; continue; }
  pc = 0x41cb87; continue;
 }
 0x41cb87 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x41cba9; continue; }
  pc = 0x41cb8b; continue;
 }
 0x41cb8b => {
  { cpu.sub8((cpu.eax as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x41cba9; continue; }
  pc = 0x41cb8f; continue;
 }
 0x41cb8f => {
  { cpu.sub8((cpu.eax as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x41cba9; continue; }
  pc = 0x41cb93; continue;
 }
 0x41cb93 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x41cb9f; continue; }
  pc = 0x41cb97; continue;
 }
 0x41cb97 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x41cb9f; continue; }
  pc = 0x41cb9b; continue;
 }
 0x41cb9b => {
  { cpu.sub8((cpu.ebx as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x41cba7; continue; }
  pc = 0x41cb9f; continue;
 }
 0x41cb9f => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x41cba9; continue;
 }
 0x41cba3 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x41cba9; continue;
 }
 0x41cba7 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x41cba9; continue;
 }
 0x41cba9 => {
  { let v: u32 = 0x4ea0e9u32; cpu.ebx = v; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.dec8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x41cbc1; continue; }
  pc = 0x41cbb4; continue;
 }
 0x41cbb4 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(8)); cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x41cbc1; continue;
 }
 0x41cbc1 => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x41cb2a; continue; }
  pc = 0x41cbcc; continue;
 }
 0x41cbcc => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x4u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x41cafd; continue; }
  pc = 0x41cbd6; continue;
 }
 0x41cbd6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041cbd7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41cbd7); }
 let mut pc: u32 = 0x41cbd7;
 loop { match pc {
 0x41cbd7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041cbd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41cbd8); }
 let mut pc: u32 = 0x41cbd8;
 loop { match pc {
 0x41cbd8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041cbd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41cbd9); }
 let mut pc: u32 = 0x41cbd9;
 loop { match pc {
 0x41cbd9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041cbda(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41cbda); }
 let mut pc: u32 = 0x41cbda;
 loop { match pc {
 0x41cbda => {
  { let v: u16 = 0x215u16; cpu.w16(0x4d68f4u32, v); }
  cpu.push(0x41cbe8); f_0041cbea(cpu); if cpu.returned_to != 0x41cbe8 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041cbea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41cbea); }
 let mut pc: u32 = 0x41cbea;
 loop { match pc {
 0x41cbea => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x41cc21; continue; }
  pc = 0x41cbf3; continue;
 }
 0x41cbf3 => {
  if cpu.zf { pc = 0x41cc21; continue; }
  pc = 0x41cbf5; continue;
 }
 0x41cbf5 => {
  { cpu.sub8((cpu.edx as u8), 0x5au8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41cbfd; continue; }
  pc = 0x41cbfa; continue;
 }
 0x41cbfa => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x41cbfd; continue;
 }
 0x41cbfd => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4d68f4u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x41cc10); cpu.call(t); if cpu.returned_to != 0x41cc10 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x592febu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x41cbea; continue;
 }
 0x41cc21 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041cc22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41cc22); }
 let mut pc: u32 = 0x41cc22;
 loop { match pc {
 0x41cc22 => {
  { let v: u32 = 0x127du32; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.push(0x41cc31); f_0041cf66(cpu); if cpu.returned_to != 0x41cc31 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041cce3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41cce3); }
 let mut pc: u32 = 0x41cce3;
 loop { match pc {
 0x41cce3 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = 0x127du16; cpu.w16(0x4d68f4u32, v); }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { cpu.sub16((cpu.ecx as u16), 0x93u16); }
  if !cpu.zf { pc = 0x41cd01; continue; }
  pc = 0x41ccfb; continue;
 }
 0x41ccfb => {
  { let v: u16 = 0x6fu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x41cd1b; continue;
 }
 0x41cd01 => {
  { cpu.sub16((cpu.ecx as u16), 0x9au16); }
  if !cpu.zf { pc = 0x41cd0e; continue; }
  pc = 0x41cd08; continue;
 }
 0x41cd08 => {
  { let v: u16 = 0x75u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x41cd1b; continue;
 }
 0x41cd0e => {
  { cpu.sub16((cpu.ecx as u16), 0x84u16); }
  if !cpu.zf { pc = 0x41cd1b; continue; }
  pc = 0x41cd15; continue;
 }
 0x41cd15 => {
  { let v: u16 = 0x61u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x41cd1b; continue;
 }
 0x41cd1b => {
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x20u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4d68f4u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x41cd2f); cpu.call(t); if cpu.returned_to != 0x41cd2f { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x593046u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041cdf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41cdf8); }
 let mut pc: u32 = 0x41cdf8;
 loop { match pc {
 0x41cdf8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x41cdff; continue;
 }
 0x41cdff => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0x93u8); }
  if !cpu.zf { pc = 0x41ce08; continue; }
  pc = 0x41ce04; continue;
 }
 0x41ce04 => {
  { let v: u8 = 0x6fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x41ce18; continue;
 }
 0x41ce08 => {
  { cpu.sub8((cpu.eax as u8), 0x9au8); }
  if !cpu.zf { pc = 0x41ce10; continue; }
  pc = 0x41ce0c; continue;
 }
 0x41ce0c => {
  { let v: u8 = 0x75u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x41ce18; continue;
 }
 0x41ce10 => {
  { cpu.sub8((cpu.eax as u8), 0x84u8); }
  if !cpu.zf { pc = 0x41ce18; continue; }
  pc = 0x41ce14; continue;
 }
 0x41ce14 => {
  { let v: u8 = 0x61u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x41ce18; continue;
 }
 0x41ce18 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x41ce31; continue; }
  pc = 0x41ce1c; continue;
 }
 0x41ce1c => {
  if cpu.sf { pc = 0x41ce31; continue; }
  pc = 0x41ce1e; continue;
 }
 0x41ce1e => {
  { cpu.sub8((cpu.eax as u8), 0x5au8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41ce24; continue; }
  pc = 0x41ce22; continue;
 }
 0x41ce22 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x41ce24; continue;
 }
 0x41ce24 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x593026u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.eax as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x41cdff; continue;
 }
 0x41ce31 => {
  { let v: u16 = (cpu.ebp as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041ce38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41ce38); }
 let mut pc: u32 = 0x41ce38;
 loop { match pc {
 0x41ce38 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x20u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x592ff0u32, v); }
  { cpu.sub8(cpu.r8(0x581b67u32), 0x0u8); }
  if cpu.zf { pc = 0x41ce9d; continue; }
  pc = 0x41ce4e; continue;
 }
 0x41ce4e => {
  { cpu.sub32(cpu.ecx, 0x125du32); }
  if !cpu.zf { pc = 0x41ce84; continue; }
  pc = 0x41ce56; continue;
 }
 0x41ce56 => {
  { let v: u32 = 0x1b46u32; cpu.w32(0x592ff0u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x581b67u32, v); }
  pc = 0x41ce9d; continue;
 }
 0x41ce84 => {
  { cpu.sub32(cpu.ecx, 0x134eu32); }
  if !cpu.zf { pc = 0x41ce9d; continue; }
  pc = 0x41ce8c; continue;
 }
 0x41ce8c => {
  { let v: u32 = 0x1b9bu32; cpu.w32(0x592ff0u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x581b67u32, v); }
  pc = 0x41ce9d; continue;
 }
 0x41ce9d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x41cea4; continue;
 }
 0x41cea4 => {
  { cpu.lods(1, false); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x41cf0b; continue; }
  pc = 0x41cead; continue;
 }
 0x41cead => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x592ff0u32), 0x12f9u32); }
  if cpu.zf { pc = 0x41cef1; continue; }
  pc = 0x41cebb; continue;
 }
 0x41cebb => {
  { cpu.sub32(cpu.r32(0x592ff0u32), 0x1b46u32); }
  if cpu.zf { pc = 0x41cee8; continue; }
  pc = 0x41cec7; continue;
 }
 0x41cec7 => {
  { cpu.sub32(cpu.r32(0x592ff0u32), 0x134eu32); }
  if cpu.zf { pc = 0x41cef8; continue; }
  pc = 0x41ced3; continue;
 }
 0x41ced3 => {
  { cpu.sub32(cpu.r32(0x592ff0u32), 0x125du32); }
  if !cpu.zf { pc = 0x41cef8; continue; }
  pc = 0x41cedf; continue;
 }
 0x41cedf => {
  cpu.push(0x41cee4); f_0041d70b(cpu); if cpu.returned_to != 0x41cee4 { return; }
  if cpu.cf { pc = 0x41cf12; continue; }
  pc = 0x41cee6; continue;
 }
 0x41cee6 => {
  pc = 0x41cef8; continue;
 }
 0x41cee8 => {
  cpu.push(0x41ceed); f_0041f8f6(cpu); if cpu.returned_to != 0x41ceed { return; }
  if cpu.cf { pc = 0x41cf12; continue; }
  pc = 0x41ceef; continue;
 }
 0x41ceef => {
  pc = 0x41cef8; continue;
 }
 0x41cef1 => {
  cpu.push(0x41cef6); f_0041d0bc(cpu); if cpu.returned_to != 0x41cef6 { return; }
  if cpu.cf { pc = 0x41cf12; continue; }
  pc = 0x41cef8; continue;
 }
 0x41cef8 => {
  { cpu.sub8((cpu.eax as u8), 0x5au8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41cefe; continue; }
  pc = 0x41cefc; continue;
 }
 0x41cefc => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x41cefe; continue;
 }
 0x41cefe => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x593026u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x41cf06; continue;
 }
 0x41cf06 => {
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.eax as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x41cea4; continue;
 }
 0x41cf0b => {
  { let v: u16 = (cpu.ebp as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41cf12 => {
  { let v: u32 = cpu.r32(0x581a04u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x581b67u32), 0x2u8); }
  if !cpu.zf { pc = 0x41cf26; continue; }
  pc = 0x41cf20; continue;
 }
 0x41cf20 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x581a04u32)); cpu.eax = v; }
  pc = 0x41cf26; continue;
 }
 0x41cf26 => {
  cpu.poll(); pc = 0x41cf06; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041cf28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41cf28); }
 let mut pc: u32 = 0x41cf28;
 loop { match pc {
 0x41cf28 => {
  { let v: u32 = 0x125du32; cpu.w32(0x592ff0u32, v); }
  pc = 0x41cf32; continue;
 }
 0x41cf32 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x41cf65; continue; }
  pc = 0x41cf3b; continue;
 }
 0x41cf3b => {
  { cpu.sub8((cpu.edx as u8), 0x5au8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41cf43; continue; }
  pc = 0x41cf40; continue;
 }
 0x41cf40 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x6u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x41cf43; continue;
 }
 0x41cf43 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x592ff0u32)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x41cf55); cpu.call(t); if cpu.returned_to != 0x41cf55 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x593026u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.poll(); pc = 0x41cf32; continue;
 }
 0x41cf65 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041cf66(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41cf66); }
 let mut pc: u32 = 0x41cf66;
 loop { match pc {
 0x41cf66 => {
  { let v: u8 = 0x0u8; cpu.w8(0x593009u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(0x20u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x592ff0u32, v); }
  { cpu.sub8(cpu.r8(0x581b67u32), 0x0u8); }
  if cpu.zf { pc = 0x41cfd2; continue; }
  pc = 0x41cf83; continue;
 }
 0x41cf83 => {
  { cpu.sub32(cpu.ecx, 0x125du32); }
  if !cpu.zf { pc = 0x41cfb9; continue; }
  pc = 0x41cf8b; continue;
 }
 0x41cf8b => {
  { let v: u32 = 0x1b46u32; cpu.w32(0x592ff0u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x581b67u32, v); }
  pc = 0x41cfd2; continue;
 }
 0x41cfb9 => {
  { cpu.sub32(cpu.ecx, 0x134eu32); }
  if !cpu.zf { pc = 0x41cfd2; continue; }
  pc = 0x41cfc1; continue;
 }
 0x41cfc1 => {
  { let v: u32 = 0x1b9bu32; cpu.w32(0x592ff0u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x581b67u32, v); }
  pc = 0x41cfd2; continue;
 }
 0x41cfd2 => {
  { let v: u32 = cpu.edx; cpu.w32(0x592ff4u32, v); }
  { let v: u32 = cpu.r32(0x592ff4u32).wrapping_add(cpu.eax); cpu.w32(0x592ff4u32, v); }
  pc = 0x41cfde; continue;
 }
 0x41cfde => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x41d08f; continue; }
  pc = 0x41cfeb; continue;
 }
 0x41cfeb => {
  { cpu.sub32(cpu.r32(0x592ff0u32), 0x12f9u32); }
  if cpu.zf { pc = 0x41d035; continue; }
  pc = 0x41cff7; continue;
 }
 0x41cff7 => {
  { cpu.sub32(cpu.r32(0x592ff0u32), 0x1b46u32); }
  if cpu.zf { pc = 0x41d028; continue; }
  pc = 0x41d003; continue;
 }
 0x41d003 => {
  { cpu.sub32(cpu.r32(0x592ff0u32), 0x134eu32); }
  if cpu.zf { pc = 0x41d03c; continue; }
  pc = 0x41d00f; continue;
 }
 0x41d00f => {
  { cpu.sub32(cpu.r32(0x592ff0u32), 0x125du32); }
  if !cpu.zf { pc = 0x41d03c; continue; }
  pc = 0x41d01b; continue;
 }
 0x41d01b => {
  cpu.push(0x41d020); f_0041d70b(cpu); if cpu.returned_to != 0x41d020 { return; }
  if cpu.cf { pc = 0x41d090; continue; }
  pc = 0x41d026; continue;
 }
 0x41d026 => {
  pc = 0x41d03c; continue;
 }
 0x41d028 => {
  cpu.push(0x41d02d); f_0041f8f6(cpu); if cpu.returned_to != 0x41d02d { return; }
  if cpu.cf { pc = 0x41d090; continue; }
  pc = 0x41d033; continue;
 }
 0x41d033 => {
  pc = 0x41d03c; continue;
 }
 0x41d035 => {
  cpu.push(0x41d03a); f_0041d0bc(cpu); if cpu.returned_to != 0x41d03a { return; }
  if cpu.cf { pc = 0x41d090; continue; }
  pc = 0x41d03c; continue;
 }
 0x41d03c => {
  { cpu.sub8((cpu.edx as u8), 0x5au8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x41d044; continue; }
  pc = 0x41d041; continue;
 }
 0x41d041 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x6u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x41d044; continue;
 }
 0x41d044 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x592ff0u32)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x41d056); cpu.call(t); if cpu.returned_to != 0x41d056 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.inc16(cpu.r16(0x593009u32)); cpu.w16(0x593009u32, v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x593026u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x581b67u32), 0x2u8); }
  if !cpu.zf { pc = 0x41d076; continue; }
  pc = 0x41d074; continue;
 }
 0x41d074 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x41d076; continue;
 }
 0x41d076 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x593005u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x593007u32, v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x592ff4u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x41cfde; continue; }
  pc = 0x41d08f; continue;
 }
 0x41d08f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41d090 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x578d6bu32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x41d09d); cpu.call(t); if cpu.returned_to != 0x41d09d { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x581a04u32)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x581b67u32), 0x2u8); }
  if !cpu.zf { pc = 0x41d0b3; continue; }
  pc = 0x41d0ad; continue;
 }
 0x41d0ad => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x581a04u32)); cpu.eax = v; }
  pc = 0x41d0b3; continue;
 }
 0x41d0b3 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x593009u32)); cpu.w16(0x593009u32, v); }
  cpu.poll(); pc = 0x41d076; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041d0bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41d0bc); }
 let mut pc: u32 = 0x41d0bc;
 loop { match pc {
 0x41d0bc => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x581a04u32, v); }
  { cpu.sub8((cpu.edx as u8), 0x84u8); }
  if !cpu.zf { pc = 0x41d0f0; continue; }
  pc = 0x41d0d5; continue;
 }
 0x41d0d5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1401u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d0f0 => {
  { cpu.sub8((cpu.edx as u8), 0x94u8); }
  if !cpu.zf { pc = 0x41d110; continue; }
  pc = 0x41d0f5; continue;
 }
 0x41d0f5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1402u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d110 => {
  { cpu.sub8((cpu.edx as u8), 0x81u8); }
  if !cpu.zf { pc = 0x41d130; continue; }
  pc = 0x41d115; continue;
 }
 0x41d115 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1403u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d130 => {
  { cpu.sub8((cpu.edx as u8), 0x85u8); }
  if !cpu.zf { pc = 0x41d150; continue; }
  pc = 0x41d135; continue;
 }
 0x41d135 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1411u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d150 => {
  { cpu.sub8((cpu.edx as u8), 0x83u8); }
  if !cpu.zf { pc = 0x41d170; continue; }
  pc = 0x41d155; continue;
 }
 0x41d155 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1412u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d170 => {
  { cpu.sub8((cpu.edx as u8), 0x87u8); }
  if !cpu.zf { pc = 0x41d190; continue; }
  pc = 0x41d175; continue;
 }
 0x41d175 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1413u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d190 => {
  { cpu.sub8((cpu.edx as u8), 0x82u8); }
  if !cpu.zf { pc = 0x41d1b0; continue; }
  pc = 0x41d195; continue;
 }
 0x41d195 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1414u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d1b0 => {
  { cpu.sub8((cpu.edx as u8), 0x8au8); }
  if !cpu.zf { pc = 0x41d1d0; continue; }
  pc = 0x41d1b5; continue;
 }
 0x41d1b5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1415u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d1d0 => {
  { cpu.sub8((cpu.edx as u8), 0x88u8); }
  if !cpu.zf { pc = 0x41d1f0; continue; }
  pc = 0x41d1d5; continue;
 }
 0x41d1d5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1416u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d1f0 => {
  { cpu.sub8((cpu.edx as u8), 0x8bu8); }
  if !cpu.zf { pc = 0x41d210; continue; }
  pc = 0x41d1f5; continue;
 }
 0x41d1f5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1417u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d210 => {
  { cpu.sub8((cpu.edx as u8), 0x8cu8); }
  if !cpu.zf { pc = 0x41d230; continue; }
  pc = 0x41d215; continue;
 }
 0x41d215 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1418u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d230 => {
  { cpu.sub8((cpu.edx as u8), 0x93u8); }
  if !cpu.zf { pc = 0x41d250; continue; }
  pc = 0x41d235; continue;
 }
 0x41d235 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1419u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d250 => {
  { cpu.sub8((cpu.edx as u8), 0x96u8); }
  if !cpu.zf { pc = 0x41d270; continue; }
  pc = 0x41d255; continue;
 }
 0x41d255 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x141au32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d270 => {
  { cpu.sub8((cpu.edx as u8), 0xa3u8); }
  if !cpu.zf { pc = 0x41d290; continue; }
  pc = 0x41d275; continue;
 }
 0x41d275 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x141bu32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d290 => {
  { cpu.sub8((cpu.edx as u8), 0x91u8); }
  if !cpu.zf { pc = 0x41d2b0; continue; }
  pc = 0x41d295; continue;
 }
 0x41d295 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x141cu32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d2b0 => {
  { cpu.sub8((cpu.edx as u8), 0xa0u8); }
  if !cpu.zf { pc = 0x41d2d0; continue; }
  pc = 0x41d2b5; continue;
 }
 0x41d2b5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1423u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d2d0 => {
  { cpu.sub8((cpu.edx as u8), 0xa1u8); }
  if !cpu.zf { pc = 0x41d2f0; continue; }
  pc = 0x41d2d5; continue;
 }
 0x41d2d5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x3u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1424u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d2f0 => {
  { cpu.sub8((cpu.edx as u8), 0xa2u8); }
  if !cpu.zf { pc = 0x41d310; continue; }
  pc = 0x41d2f5; continue;
 }
 0x41d2f5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1425u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d310 => {
  { cpu.sub8((cpu.edx as u8), 0x95u8); }
  if !cpu.zf { pc = 0x41d330; continue; }
  pc = 0x41d315; continue;
 }
 0x41d315 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x142au32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d330 => {
  { cpu.sub8((cpu.edx as u8), 0x97u8); }
  if !cpu.zf { pc = 0x41d350; continue; }
  pc = 0x41d335; continue;
 }
 0x41d335 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x142bu32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d350 => {
  { cpu.sub8((cpu.edx as u8), 0x92u8); }
  if !cpu.zf { pc = 0x41d370; continue; }
  pc = 0x41d355; continue;
 }
 0x41d355 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1410u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d370 => {
  { cpu.sub8((cpu.edx as u8), 0x8bu8); }
  if !cpu.zf { pc = 0x41d390; continue; }
  pc = 0x41d375; continue;
 }
 0x41d375 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x140bu32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d390 => {
  { cpu.sub8((cpu.edx as u8), 0x8cu8); }
  if !cpu.zf { pc = 0x41d3b0; continue; }
  pc = 0x41d395; continue;
 }
 0x41d395 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x140cu32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d3b0 => {
  { cpu.sub8((cpu.edx as u8), 0x93u8); }
  if !cpu.zf { pc = 0x41d3d0; continue; }
  pc = 0x41d3b5; continue;
 }
 0x41d3b5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x140du32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d3d0 => {
  { cpu.sub8((cpu.edx as u8), 0x96u8); }
  if !cpu.zf { pc = 0x41d3f0; continue; }
  pc = 0x41d3d5; continue;
 }
 0x41d3d5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x140eu32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d3f0 => {
  { cpu.sub8((cpu.edx as u8), 0xa3u8); }
  if !cpu.zf { pc = 0x41d410; continue; }
  pc = 0x41d3f5; continue;
 }
 0x41d3f5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x140fu32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d410 => {
  { cpu.sub8((cpu.edx as u8), 0x7fu8); }
  if !cpu.zf { pc = 0x41d430; continue; }
  pc = 0x41d415; continue;
 }
 0x41d415 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1405u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d430 => {
  { cpu.sub8((cpu.edx as u8), 0x83u8); }
  if !cpu.zf { pc = 0x41d450; continue; }
  pc = 0x41d435; continue;
 }
 0x41d435 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1406u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d450 => {
  { cpu.sub8((cpu.edx as u8), 0x80u8); }
  if !cpu.zf { pc = 0x41d470; continue; }
  pc = 0x41d455; continue;
 }
 0x41d455 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1407u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d470 => {
  { cpu.sub8((cpu.edx as u8), 0x82u8); }
  if !cpu.zf { pc = 0x41d490; continue; }
  pc = 0x41d475; continue;
 }
 0x41d475 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1408u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d490 => {
  { cpu.sub8((cpu.edx as u8), 0x8au8); }
  if !cpu.zf { pc = 0x41d4b0; continue; }
  pc = 0x41d495; continue;
 }
 0x41d495 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1409u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d4b0 => {
  { cpu.sub8((cpu.edx as u8), 0x88u8); }
  if !cpu.zf { pc = 0x41d4d0; continue; }
  pc = 0x41d4b5; continue;
 }
 0x41d4b5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x140au32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d4d0 => {
  { cpu.sub8((cpu.edx as u8), 0x8fu8); }
  if !cpu.zf { pc = 0x41d4f0; continue; }
  pc = 0x41d4d5; continue;
 }
 0x41d4d5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1420u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d4f0 => {
  { cpu.sub8((cpu.edx as u8), 0xa1u8); }
  if !cpu.zf { pc = 0x41d510; continue; }
  pc = 0x41d4f5; continue;
 }
 0x41d4f5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1421u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d510 => {
  { cpu.sub8((cpu.edx as u8), 0xa2u8); }
  if !cpu.zf { pc = 0x41d530; continue; }
  pc = 0x41d515; continue;
 }
 0x41d515 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1422u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d530 => {
  { cpu.sub8((cpu.edx as u8), 0x95u8); }
  if !cpu.zf { pc = 0x41d550; continue; }
  pc = 0x41d535; continue;
 }
 0x41d535 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1428u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d550 => {
  { cpu.sub8((cpu.edx as u8), 0x97u8); }
  if !cpu.zf { pc = 0x41d570; continue; }
  pc = 0x41d555; continue;
 }
 0x41d555 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1429u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d570 => {
  { cpu.sub8((cpu.edx as u8), 0x8eu8); }
  if !cpu.zf { pc = 0x41d590; continue; }
  pc = 0x41d575; continue;
 }
 0x41d575 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x13feu32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d590 => {
  { cpu.sub8((cpu.edx as u8), 0x99u8); }
  if !cpu.zf { pc = 0x41d5b0; continue; }
  pc = 0x41d595; continue;
 }
 0x41d595 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x13ffu32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d5b0 => {
  { cpu.sub8((cpu.edx as u8), 0x9au8); }
  if !cpu.zf { pc = 0x41d5d0; continue; }
  pc = 0x41d5b5; continue;
 }
 0x41d5b5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1400u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d5d0 => {
  { cpu.sub8((cpu.edx as u8), 0xc6u8); }
  if !cpu.zf { pc = 0x41d5f0; continue; }
  pc = 0x41d5d5; continue;
 }
 0x41d5d5 => {
  { let v: u32 = 0x3u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1439u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d5f0 => {
  { cpu.sub8((cpu.edx as u8), 0xc7u8); }
  if !cpu.zf { pc = 0x41d610; continue; }
  pc = 0x41d5f5; continue;
 }
 0x41d5f5 => {
  { let v: u32 = 0x3u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1438u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d610 => {
  { cpu.sub8((cpu.edx as u8), 0xe4u8); }
  if !cpu.zf { pc = 0x41d630; continue; }
  pc = 0x41d615; continue;
 }
 0x41d615 => {
  { let v: u32 = 0x3u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x143bu32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d630 => {
  { cpu.sub8((cpu.edx as u8), 0xe5u8); }
  if !cpu.zf { pc = 0x41d650; continue; }
  pc = 0x41d635; continue;
 }
 0x41d635 => {
  { let v: u32 = 0x3u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x143au32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d650 => {
  { cpu.sub8((cpu.edx as u8), 0xf8u8); }
  if !cpu.zf { pc = 0x41d670; continue; }
  pc = 0x41d655; continue;
 }
 0x41d655 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x3u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x141du32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d670 => {
  { cpu.sub8((cpu.edx as u8), 0xe1u8); }
  if !cpu.zf { pc = 0x41d690; continue; }
  pc = 0x41d675; continue;
 }
 0x41d675 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1404u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d690 => {
  { cpu.sub8((cpu.edx as u8), 0xaeu8); }
  if !cpu.zf { pc = 0x41d6b0; continue; }
  pc = 0x41d695; continue;
 }
 0x41d695 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x141eu32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d6b0 => {
  { cpu.sub8((cpu.edx as u8), 0xafu8); }
  if !cpu.zf { pc = 0x41d6cd; continue; }
  pc = 0x41d6b5; continue;
 }
 0x41d6b5 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x141fu32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d6cd => {
  { cpu.sub8((cpu.edx as u8), 0xa8u8); }
  if !cpu.zf { pc = 0x41d6ea; continue; }
  pc = 0x41d6d2; continue;
 }
 0x41d6d2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1426u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d6ea => {
  { cpu.sub8((cpu.edx as u8), 0xadu8); }
  if !cpu.zf { pc = 0x41d707; continue; }
  pc = 0x41d6ef; continue;
 }
 0x41d6ef => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x2u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1427u32; cpu.ecx = v; }
  pc = 0x41d709; continue;
 }
 0x41d707 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41d709 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041d70b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41d70b); }
 let mut pc: u32 = 0x41d70b;
 loop { match pc {
 0x41d70b => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x581a04u32, v); }
  { cpu.sub8((cpu.edx as u8), 0x84u8); }
  if !cpu.zf { pc = 0x41d73f; continue; }
  pc = 0x41d724; continue;
 }
 0x41d724 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12e8u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d73f => {
  { cpu.sub8((cpu.edx as u8), 0x94u8); }
  if !cpu.zf { pc = 0x41d75f; continue; }
  pc = 0x41d744; continue;
 }
 0x41d744 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12e9u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d75f => {
  { cpu.sub8((cpu.edx as u8), 0x81u8); }
  if !cpu.zf { pc = 0x41d77f; continue; }
  pc = 0x41d764; continue;
 }
 0x41d764 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12eau32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d77f => {
  { cpu.sub8((cpu.edx as u8), 0x85u8); }
  if !cpu.zf { pc = 0x41d79f; continue; }
  pc = 0x41d784; continue;
 }
 0x41d784 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12f8u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d79f => {
  { cpu.sub8((cpu.edx as u8), 0x83u8); }
  if !cpu.zf { pc = 0x41d7bf; continue; }
  pc = 0x41d7a4; continue;
 }
 0x41d7a4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12f9u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d7bf => {
  { cpu.sub8((cpu.edx as u8), 0x87u8); }
  if !cpu.zf { pc = 0x41d7df; continue; }
  pc = 0x41d7c4; continue;
 }
 0x41d7c4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12fau32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d7df => {
  { cpu.sub8((cpu.edx as u8), 0x82u8); }
  if !cpu.zf { pc = 0x41d7ff; continue; }
  pc = 0x41d7e4; continue;
 }
 0x41d7e4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12fbu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d7ff => {
  { cpu.sub8((cpu.edx as u8), 0x8au8); }
  if !cpu.zf { pc = 0x41d81f; continue; }
  pc = 0x41d804; continue;
 }
 0x41d804 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12fcu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d81f => {
  { cpu.sub8((cpu.edx as u8), 0x88u8); }
  if !cpu.zf { pc = 0x41d83f; continue; }
  pc = 0x41d824; continue;
 }
 0x41d824 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12fdu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d83f => {
  { cpu.sub8((cpu.edx as u8), 0x8bu8); }
  if !cpu.zf { pc = 0x41d85f; continue; }
  pc = 0x41d844; continue;
 }
 0x41d844 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12feu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d85f => {
  { cpu.sub8((cpu.edx as u8), 0x8cu8); }
  if !cpu.zf { pc = 0x41d87f; continue; }
  pc = 0x41d864; continue;
 }
 0x41d864 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12ffu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d87f => {
  { cpu.sub8((cpu.edx as u8), 0x93u8); }
  if !cpu.zf { pc = 0x41d89f; continue; }
  pc = 0x41d884; continue;
 }
 0x41d884 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1300u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d89f => {
  { cpu.sub8((cpu.edx as u8), 0x96u8); }
  if !cpu.zf { pc = 0x41d8bf; continue; }
  pc = 0x41d8a4; continue;
 }
 0x41d8a4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1301u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d8bf => {
  { cpu.sub8((cpu.edx as u8), 0xa3u8); }
  if !cpu.zf { pc = 0x41d8df; continue; }
  pc = 0x41d8c4; continue;
 }
 0x41d8c4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1302u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d8df => {
  { cpu.sub8((cpu.edx as u8), 0x91u8); }
  if !cpu.zf { pc = 0x41d8ff; continue; }
  pc = 0x41d8e4; continue;
 }
 0x41d8e4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1303u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d8ff => {
  { cpu.sub8((cpu.edx as u8), 0xa0u8); }
  if !cpu.zf { pc = 0x41d91f; continue; }
  pc = 0x41d904; continue;
 }
 0x41d904 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x130au32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d91f => {
  { cpu.sub8((cpu.edx as u8), 0xa1u8); }
  if !cpu.zf { pc = 0x41d93f; continue; }
  pc = 0x41d924; continue;
 }
 0x41d924 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x3u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x130bu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d93f => {
  { cpu.sub8((cpu.edx as u8), 0xa2u8); }
  if !cpu.zf { pc = 0x41d95f; continue; }
  pc = 0x41d944; continue;
 }
 0x41d944 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x130cu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d95f => {
  { cpu.sub8((cpu.edx as u8), 0x95u8); }
  if !cpu.zf { pc = 0x41d97f; continue; }
  pc = 0x41d964; continue;
 }
 0x41d964 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1311u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d97f => {
  { cpu.sub8((cpu.edx as u8), 0x97u8); }
  if !cpu.zf { pc = 0x41d99f; continue; }
  pc = 0x41d984; continue;
 }
 0x41d984 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1312u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d99f => {
  { cpu.sub8((cpu.edx as u8), 0x92u8); }
  if !cpu.zf { pc = 0x41d9bf; continue; }
  pc = 0x41d9a4; continue;
 }
 0x41d9a4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12f7u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d9bf => {
  { cpu.sub8((cpu.edx as u8), 0x8bu8); }
  if !cpu.zf { pc = 0x41d9df; continue; }
  pc = 0x41d9c4; continue;
 }
 0x41d9c4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12f2u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d9df => {
  { cpu.sub8((cpu.edx as u8), 0x8cu8); }
  if !cpu.zf { pc = 0x41d9ff; continue; }
  pc = 0x41d9e4; continue;
 }
 0x41d9e4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12f3u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41d9ff => {
  { cpu.sub8((cpu.edx as u8), 0x93u8); }
  if !cpu.zf { pc = 0x41da1f; continue; }
  pc = 0x41da04; continue;
 }
 0x41da04 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12f4u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41da1f => {
  { cpu.sub8((cpu.edx as u8), 0x96u8); }
  if !cpu.zf { pc = 0x41da3f; continue; }
  pc = 0x41da24; continue;
 }
 0x41da24 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12f5u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41da3f => {
  { cpu.sub8((cpu.edx as u8), 0xa3u8); }
  if !cpu.zf { pc = 0x41da5f; continue; }
  pc = 0x41da44; continue;
 }
 0x41da44 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12f6u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41da5f => {
  { cpu.sub8((cpu.edx as u8), 0x7fu8); }
  if !cpu.zf { pc = 0x41da7f; continue; }
  pc = 0x41da64; continue;
 }
 0x41da64 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12ecu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41da7f => {
  { cpu.sub8((cpu.edx as u8), 0x83u8); }
  if !cpu.zf { pc = 0x41da9f; continue; }
  pc = 0x41da84; continue;
 }
 0x41da84 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12edu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41da9f => {
  { cpu.sub8((cpu.edx as u8), 0x80u8); }
  if !cpu.zf { pc = 0x41dabf; continue; }
  pc = 0x41daa4; continue;
 }
 0x41daa4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12eeu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dabf => {
  { cpu.sub8((cpu.edx as u8), 0x82u8); }
  if !cpu.zf { pc = 0x41dadf; continue; }
  pc = 0x41dac4; continue;
 }
 0x41dac4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12efu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dadf => {
  { cpu.sub8((cpu.edx as u8), 0x8au8); }
  if !cpu.zf { pc = 0x41daff; continue; }
  pc = 0x41dae4; continue;
 }
 0x41dae4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12f0u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41daff => {
  { cpu.sub8((cpu.edx as u8), 0x88u8); }
  if !cpu.zf { pc = 0x41db1f; continue; }
  pc = 0x41db04; continue;
 }
 0x41db04 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12f1u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41db1f => {
  { cpu.sub8((cpu.edx as u8), 0x8fu8); }
  if !cpu.zf { pc = 0x41db3f; continue; }
  pc = 0x41db24; continue;
 }
 0x41db24 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1307u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41db3f => {
  { cpu.sub8((cpu.edx as u8), 0xa1u8); }
  if !cpu.zf { pc = 0x41db5f; continue; }
  pc = 0x41db44; continue;
 }
 0x41db44 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1308u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41db5f => {
  { cpu.sub8((cpu.edx as u8), 0xa2u8); }
  if !cpu.zf { pc = 0x41db7f; continue; }
  pc = 0x41db64; continue;
 }
 0x41db64 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1309u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41db7f => {
  { cpu.sub8((cpu.edx as u8), 0x95u8); }
  if !cpu.zf { pc = 0x41db9f; continue; }
  pc = 0x41db84; continue;
 }
 0x41db84 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x130fu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41db9f => {
  { cpu.sub8((cpu.edx as u8), 0x97u8); }
  if !cpu.zf { pc = 0x41dbbf; continue; }
  pc = 0x41dba4; continue;
 }
 0x41dba4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1310u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dbbf => {
  { cpu.sub8((cpu.edx as u8), 0x8eu8); }
  if !cpu.zf { pc = 0x41dbdf; continue; }
  pc = 0x41dbc4; continue;
 }
 0x41dbc4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12e5u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dbdf => {
  { cpu.sub8((cpu.edx as u8), 0x99u8); }
  if !cpu.zf { pc = 0x41dbff; continue; }
  pc = 0x41dbe4; continue;
 }
 0x41dbe4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12e6u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dbff => {
  { cpu.sub8((cpu.edx as u8), 0x9au8); }
  if !cpu.zf { pc = 0x41dc1f; continue; }
  pc = 0x41dc04; continue;
 }
 0x41dc04 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12e7u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dc1f => {
  { cpu.sub8((cpu.edx as u8), 0xf8u8); }
  if !cpu.zf { pc = 0x41dc3f; continue; }
  pc = 0x41dc24; continue;
 }
 0x41dc24 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x3u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1304u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dc3f => {
  { cpu.sub8((cpu.edx as u8), 0xe1u8); }
  if !cpu.zf { pc = 0x41dc5f; continue; }
  pc = 0x41dc44; continue;
 }
 0x41dc44 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x12ebu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dc5f => {
  { cpu.sub8((cpu.edx as u8), 0xaeu8); }
  if !cpu.zf { pc = 0x41dc7f; continue; }
  pc = 0x41dc64; continue;
 }
 0x41dc64 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1305u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dc7f => {
  { cpu.sub8((cpu.edx as u8), 0xafu8); }
  if !cpu.zf { pc = 0x41dc9f; continue; }
  pc = 0x41dc84; continue;
 }
 0x41dc84 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1306u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dc9f => {
  { cpu.sub8((cpu.edx as u8), 0xa8u8); }
  if !cpu.zf { pc = 0x41dcbf; continue; }
  pc = 0x41dca4; continue;
 }
 0x41dca4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x130du32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dcbf => {
  { cpu.sub8((cpu.edx as u8), 0xadu8); }
  if !cpu.zf { pc = 0x41dcdf; continue; }
  pc = 0x41dcc4; continue;
 }
 0x41dcc4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x2u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x130eu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dcdf => {
  { cpu.sub8((cpu.edx as u8), 0xc6u8); }
  if !cpu.zf { pc = 0x41dcff; continue; }
  pc = 0x41dce4; continue;
 }
 0x41dce4 => {
  { let v: u32 = 0x3u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1316u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dcff => {
  { cpu.sub8((cpu.edx as u8), 0xc7u8); }
  if !cpu.zf { pc = 0x41dd1f; continue; }
  pc = 0x41dd04; continue;
 }
 0x41dd04 => {
  { let v: u32 = 0x3u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1315u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dd1f => {
  { cpu.sub8((cpu.edx as u8), 0xe4u8); }
  if !cpu.zf { pc = 0x41dd3f; continue; }
  pc = 0x41dd24; continue;
 }
 0x41dd24 => {
  { let v: u32 = 0x3u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1318u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dd3f => {
  { cpu.sub8((cpu.edx as u8), 0xe5u8); }
  if !cpu.zf { pc = 0x41dd5f; continue; }
  pc = 0x41dd44; continue;
 }
 0x41dd44 => {
  { let v: u32 = 0x3u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1317u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dd5f => {
  { cpu.sub8((cpu.edx as u8), 0xa5u8); }
  if !cpu.zf { pc = 0x41dd7f; continue; }
  pc = 0x41dd64; continue;
 }
 0x41dd64 => {
  { let v: u32 = 0x2u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x142cu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dd7f => {
  { cpu.sub8((cpu.edx as u8), 0xa4u8); }
  if !cpu.zf { pc = 0x41dd9f; continue; }
  pc = 0x41dd84; continue;
 }
 0x41dd84 => {
  { let v: u32 = 0x3u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x142du32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dd9f => {
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x41ddbf; continue; }
  pc = 0x41dda4; continue;
 }
 0x41dda4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x8u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x142eu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41ddbf => {
  { cpu.sub8((cpu.edx as u8), 0x23u8); }
  if !cpu.zf { pc = 0x41dddf; continue; }
  pc = 0x41ddc4; continue;
 }
 0x41ddc4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x142fu32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41dddf => {
  { cpu.sub8((cpu.edx as u8), 0x24u8); }
  if !cpu.zf { pc = 0x41ddff; continue; }
  pc = 0x41dde4; continue;
 }
 0x41dde4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1430u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41ddff => {
  { cpu.sub8((cpu.edx as u8), 0x5fu8); }
  if !cpu.zf { pc = 0x41de1f; continue; }
  pc = 0x41de04; continue;
 }
 0x41de04 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1431u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41de1f => {
  { cpu.sub8((cpu.edx as u8), 0x26u8); }
  if !cpu.zf { pc = 0x41de3f; continue; }
  pc = 0x41de24; continue;
 }
 0x41de24 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x8u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1432u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41de3f => {
  { cpu.sub8((cpu.edx as u8), 0x7eu8); }
  if !cpu.zf { pc = 0x41de5f; continue; }
  pc = 0x41de44; continue;
 }
 0x41de44 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1433u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41de5f => {
  { cpu.sub8((cpu.edx as u8), 0x5eu8); }
  if !cpu.zf { pc = 0x41de7f; continue; }
  pc = 0x41de64; continue;
 }
 0x41de64 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1434u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41de7f => {
  { cpu.sub8((cpu.edx as u8), 0x5cu8); }
  if !cpu.zf { pc = 0x41de9c; continue; }
  pc = 0x41de84; continue;
 }
 0x41de84 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1435u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41de9c => {
  { cpu.sub8((cpu.edx as u8), 0x7cu8); }
  if !cpu.zf { pc = 0x41deb9; continue; }
  pc = 0x41dea1; continue;
 }
 0x41dea1 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x2u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1436u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41deb9 => {
  { cpu.sub8((cpu.edx as u8), 0x9cu8); }
  if !cpu.zf { pc = 0x41ded6; continue; }
  pc = 0x41debe; continue;
 }
 0x41debe => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x8u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1437u32; cpu.ecx = v; }
  pc = 0x41ded8; continue;
 }
 0x41ded6 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41ded8 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041deda(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41deda); }
 let mut pc: u32 = 0x41deda;
 loop { match pc {
 0x41deda => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x581a04u32, v); }
  { cpu.sub8((cpu.edx as u8), 0x84u8); }
  if !cpu.zf { pc = 0x41df0e; continue; }
  pc = 0x41def3; continue;
 }
 0x41def3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x41bu32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41df0e => {
  { cpu.sub8((cpu.edx as u8), 0x94u8); }
  if !cpu.zf { pc = 0x41df2e; continue; }
  pc = 0x41df13; continue;
 }
 0x41df13 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x41cu32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41df2e => {
  { cpu.sub8((cpu.edx as u8), 0x81u8); }
  if !cpu.zf { pc = 0x41df4e; continue; }
  pc = 0x41df33; continue;
 }
 0x41df33 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x41du32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41df4e => {
  { cpu.sub8((cpu.edx as u8), 0x85u8); }
  if !cpu.zf { pc = 0x41df6e; continue; }
  pc = 0x41df53; continue;
 }
 0x41df53 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x42bu32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41df6e => {
  { cpu.sub8((cpu.edx as u8), 0x83u8); }
  if !cpu.zf { pc = 0x41df8e; continue; }
  pc = 0x41df73; continue;
 }
 0x41df73 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x42cu32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41df8e => {
  { cpu.sub8((cpu.edx as u8), 0x87u8); }
  if !cpu.zf { pc = 0x41dfae; continue; }
  pc = 0x41df93; continue;
 }
 0x41df93 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x42du32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41dfae => {
  { cpu.sub8((cpu.edx as u8), 0x82u8); }
  if !cpu.zf { pc = 0x41dfce; continue; }
  pc = 0x41dfb3; continue;
 }
 0x41dfb3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x42eu32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41dfce => {
  { cpu.sub8((cpu.edx as u8), 0x8au8); }
  if !cpu.zf { pc = 0x41dfee; continue; }
  pc = 0x41dfd3; continue;
 }
 0x41dfd3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x42fu32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41dfee => {
  { cpu.sub8((cpu.edx as u8), 0x88u8); }
  if !cpu.zf { pc = 0x41e00e; continue; }
  pc = 0x41dff3; continue;
 }
 0x41dff3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x430u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e00e => {
  { cpu.sub8((cpu.edx as u8), 0x8bu8); }
  if !cpu.zf { pc = 0x41e02e; continue; }
  pc = 0x41e013; continue;
 }
 0x41e013 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x431u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e02e => {
  { cpu.sub8((cpu.edx as u8), 0x8cu8); }
  if !cpu.zf { pc = 0x41e04e; continue; }
  pc = 0x41e033; continue;
 }
 0x41e033 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x432u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e04e => {
  { cpu.sub8((cpu.edx as u8), 0x93u8); }
  if !cpu.zf { pc = 0x41e06e; continue; }
  pc = 0x41e053; continue;
 }
 0x41e053 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x433u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e06e => {
  { cpu.sub8((cpu.edx as u8), 0x96u8); }
  if !cpu.zf { pc = 0x41e08e; continue; }
  pc = 0x41e073; continue;
 }
 0x41e073 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x434u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e08e => {
  { cpu.sub8((cpu.edx as u8), 0xa3u8); }
  if !cpu.zf { pc = 0x41e0ae; continue; }
  pc = 0x41e093; continue;
 }
 0x41e093 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x435u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e0ae => {
  { cpu.sub8((cpu.edx as u8), 0x91u8); }
  if !cpu.zf { pc = 0x41e0ce; continue; }
  pc = 0x41e0b3; continue;
 }
 0x41e0b3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x436u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e0ce => {
  { cpu.sub8((cpu.edx as u8), 0xa0u8); }
  if !cpu.zf { pc = 0x41e0ee; continue; }
  pc = 0x41e0d3; continue;
 }
 0x41e0d3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x43du32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e0ee => {
  { cpu.sub8((cpu.edx as u8), 0xa1u8); }
  if !cpu.zf { pc = 0x41e10e; continue; }
  pc = 0x41e0f3; continue;
 }
 0x41e0f3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x3u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x43eu32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e10e => {
  { cpu.sub8((cpu.edx as u8), 0xa2u8); }
  if !cpu.zf { pc = 0x41e12e; continue; }
  pc = 0x41e113; continue;
 }
 0x41e113 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x43fu32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e12e => {
  { cpu.sub8((cpu.edx as u8), 0x95u8); }
  if !cpu.zf { pc = 0x41e14e; continue; }
  pc = 0x41e133; continue;
 }
 0x41e133 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x444u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e14e => {
  { cpu.sub8((cpu.edx as u8), 0x97u8); }
  if !cpu.zf { pc = 0x41e16e; continue; }
  pc = 0x41e153; continue;
 }
 0x41e153 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x445u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e16e => {
  { cpu.sub8((cpu.edx as u8), 0x92u8); }
  if !cpu.zf { pc = 0x41e18e; continue; }
  pc = 0x41e173; continue;
 }
 0x41e173 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x42au32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e18e => {
  { cpu.sub8((cpu.edx as u8), 0x8bu8); }
  if !cpu.zf { pc = 0x41e1ae; continue; }
  pc = 0x41e193; continue;
 }
 0x41e193 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x425u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e1ae => {
  { cpu.sub8((cpu.edx as u8), 0x8cu8); }
  if !cpu.zf { pc = 0x41e1ce; continue; }
  pc = 0x41e1b3; continue;
 }
 0x41e1b3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x426u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e1ce => {
  { cpu.sub8((cpu.edx as u8), 0x93u8); }
  if !cpu.zf { pc = 0x41e1ee; continue; }
  pc = 0x41e1d3; continue;
 }
 0x41e1d3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x427u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e1ee => {
  { cpu.sub8((cpu.edx as u8), 0x96u8); }
  if !cpu.zf { pc = 0x41e20e; continue; }
  pc = 0x41e1f3; continue;
 }
 0x41e1f3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x428u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e20e => {
  { cpu.sub8((cpu.edx as u8), 0xa3u8); }
  if !cpu.zf { pc = 0x41e22e; continue; }
  pc = 0x41e213; continue;
 }
 0x41e213 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x429u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e22e => {
  { cpu.sub8((cpu.edx as u8), 0x7fu8); }
  if !cpu.zf { pc = 0x41e24e; continue; }
  pc = 0x41e233; continue;
 }
 0x41e233 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x41fu32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e24e => {
  { cpu.sub8((cpu.edx as u8), 0x83u8); }
  if !cpu.zf { pc = 0x41e26e; continue; }
  pc = 0x41e253; continue;
 }
 0x41e253 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x420u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e26e => {
  { cpu.sub8((cpu.edx as u8), 0x80u8); }
  if !cpu.zf { pc = 0x41e28e; continue; }
  pc = 0x41e273; continue;
 }
 0x41e273 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x421u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e28e => {
  { cpu.sub8((cpu.edx as u8), 0x82u8); }
  if !cpu.zf { pc = 0x41e2ae; continue; }
  pc = 0x41e293; continue;
 }
 0x41e293 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x422u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e2ae => {
  { cpu.sub8((cpu.edx as u8), 0x8au8); }
  if !cpu.zf { pc = 0x41e2ce; continue; }
  pc = 0x41e2b3; continue;
 }
 0x41e2b3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x423u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e2ce => {
  { cpu.sub8((cpu.edx as u8), 0x88u8); }
  if !cpu.zf { pc = 0x41e2ee; continue; }
  pc = 0x41e2d3; continue;
 }
 0x41e2d3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x424u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e2ee => {
  { cpu.sub8((cpu.edx as u8), 0x8fu8); }
  if !cpu.zf { pc = 0x41e30e; continue; }
  pc = 0x41e2f3; continue;
 }
 0x41e2f3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x43au32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e30e => {
  { cpu.sub8((cpu.edx as u8), 0xa1u8); }
  if !cpu.zf { pc = 0x41e32e; continue; }
  pc = 0x41e313; continue;
 }
 0x41e313 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x43bu32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e32e => {
  { cpu.sub8((cpu.edx as u8), 0xa2u8); }
  if !cpu.zf { pc = 0x41e34e; continue; }
  pc = 0x41e333; continue;
 }
 0x41e333 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x43cu32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e34e => {
  { cpu.sub8((cpu.edx as u8), 0x95u8); }
  if !cpu.zf { pc = 0x41e36e; continue; }
  pc = 0x41e353; continue;
 }
 0x41e353 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x442u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e36e => {
  { cpu.sub8((cpu.edx as u8), 0x97u8); }
  if !cpu.zf { pc = 0x41e38e; continue; }
  pc = 0x41e373; continue;
 }
 0x41e373 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x443u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e38e => {
  { cpu.sub8((cpu.edx as u8), 0x8eu8); }
  if !cpu.zf { pc = 0x41e3ae; continue; }
  pc = 0x41e393; continue;
 }
 0x41e393 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x418u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e3ae => {
  { cpu.sub8((cpu.edx as u8), 0x99u8); }
  if !cpu.zf { pc = 0x41e3ce; continue; }
  pc = 0x41e3b3; continue;
 }
 0x41e3b3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x419u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e3ce => {
  { cpu.sub8((cpu.edx as u8), 0x9au8); }
  if !cpu.zf { pc = 0x41e3ee; continue; }
  pc = 0x41e3d3; continue;
 }
 0x41e3d3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x41au32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e3ee => {
  { cpu.sub8((cpu.edx as u8), 0xe1u8); }
  if !cpu.zf { pc = 0x41e40e; continue; }
  pc = 0x41e3f3; continue;
 }
 0x41e3f3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x3u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x41eu32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e40e => {
  { cpu.sub8((cpu.edx as u8), 0xf8u8); }
  if !cpu.zf { pc = 0x41e42e; continue; }
  pc = 0x41e413; continue;
 }
 0x41e413 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x437u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e42e => {
  { cpu.sub8((cpu.edx as u8), 0xaeu8); }
  if !cpu.zf { pc = 0x41e44e; continue; }
  pc = 0x41e433; continue;
 }
 0x41e433 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x438u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e44e => {
  { cpu.sub8((cpu.edx as u8), 0xafu8); }
  if !cpu.zf { pc = 0x41e46e; continue; }
  pc = 0x41e453; continue;
 }
 0x41e453 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x439u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e46e => {
  { cpu.sub8((cpu.edx as u8), 0xa8u8); }
  if !cpu.zf { pc = 0x41e48e; continue; }
  pc = 0x41e473; continue;
 }
 0x41e473 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x440u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e48e => {
  { cpu.sub8((cpu.edx as u8), 0xadu8); }
  if !cpu.zf { pc = 0x41e4ae; continue; }
  pc = 0x41e493; continue;
 }
 0x41e493 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x2u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x441u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e4ae => {
  { cpu.sub8((cpu.edx as u8), 0xc6u8); }
  if !cpu.zf { pc = 0x41e4ce; continue; }
  pc = 0x41e4b3; continue;
 }
 0x41e4b3 => {
  { let v: u32 = 0x3u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x464u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e4ce => {
  { cpu.sub8((cpu.edx as u8), 0xc7u8); }
  if !cpu.zf { pc = 0x41e4ee; continue; }
  pc = 0x41e4d3; continue;
 }
 0x41e4d3 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x463u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e4ee => {
  { cpu.sub8((cpu.edx as u8), 0xe4u8); }
  if !cpu.zf { pc = 0x41e50e; continue; }
  pc = 0x41e4f3; continue;
 }
 0x41e4f3 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x466u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e50e => {
  { cpu.sub8((cpu.edx as u8), 0xe5u8); }
  if !cpu.zf { pc = 0x41e52e; continue; }
  pc = 0x41e513; continue;
 }
 0x41e513 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x465u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e52e => {
  { cpu.sub8((cpu.edx as u8), 0xa5u8); }
  if !cpu.zf { pc = 0x41e54e; continue; }
  pc = 0x41e533; continue;
 }
 0x41e533 => {
  { let v: u32 = 0x3u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x452u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e54e => {
  { cpu.sub8((cpu.edx as u8), 0xa4u8); }
  if !cpu.zf { pc = 0x41e56e; continue; }
  pc = 0x41e553; continue;
 }
 0x41e553 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x453u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e56e => {
  { cpu.sub8((cpu.edx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x41e58e; continue; }
  pc = 0x41e573; continue;
 }
 0x41e573 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x8u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x454u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e58e => {
  { cpu.sub8((cpu.edx as u8), 0x23u8); }
  if !cpu.zf { pc = 0x41e5ae; continue; }
  pc = 0x41e593; continue;
 }
 0x41e593 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x455u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e5ae => {
  { cpu.sub8((cpu.edx as u8), 0x24u8); }
  if !cpu.zf { pc = 0x41e5ce; continue; }
  pc = 0x41e5b3; continue;
 }
 0x41e5b3 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x456u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e5ce => {
  { cpu.sub8((cpu.edx as u8), 0x5fu8); }
  if !cpu.zf { pc = 0x41e5ee; continue; }
  pc = 0x41e5d3; continue;
 }
 0x41e5d3 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x457u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e5ee => {
  { cpu.sub8((cpu.edx as u8), 0x26u8); }
  if !cpu.zf { pc = 0x41e60e; continue; }
  pc = 0x41e5f3; continue;
 }
 0x41e5f3 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x8u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x458u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e60e => {
  { cpu.sub8((cpu.edx as u8), 0x7eu8); }
  if !cpu.zf { pc = 0x41e62e; continue; }
  pc = 0x41e613; continue;
 }
 0x41e613 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x459u32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e62e => {
  { cpu.sub8((cpu.edx as u8), 0x5eu8); }
  if !cpu.zf { pc = 0x41e64e; continue; }
  pc = 0x41e633; continue;
 }
 0x41e633 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x45au32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e64e => {
  { cpu.sub8((cpu.edx as u8), 0x5cu8); }
  if !cpu.zf { pc = 0x41e66b; continue; }
  pc = 0x41e653; continue;
 }
 0x41e653 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x45bu32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e66b => {
  { cpu.sub8((cpu.edx as u8), 0x7cu8); }
  if !cpu.zf { pc = 0x41e688; continue; }
  pc = 0x41e670; continue;
 }
 0x41e670 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x2u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x45cu32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e688 => {
  { cpu.sub8((cpu.edx as u8), 0x9cu8); }
  if !cpu.zf { pc = 0x41e6a5; continue; }
  pc = 0x41e68d; continue;
 }
 0x41e68d => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x8u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x45du32; cpu.ecx = v; }
  pc = 0x41e6a7; continue;
 }
 0x41e6a5 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41e6a7 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041e6a9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41e6a9); } r_0041e6a9(cpu, 0x41e6a9); }
pub fn f_0041e800(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41e800); } r_0041e6a9(cpu, 0x41e800); }
pub fn f_0041e831(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41e831); } r_0041e6a9(cpu, 0x41e831); }
fn r_0041e6a9(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x41e6a9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x581a04u32, v); }
  { cpu.sub8((cpu.edx as u8), 0x84u8); }
  if !cpu.zf { pc = 0x41e6dd; continue; }
  pc = 0x41e6c2; continue;
 }
 0x41e6c2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3edu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e6dd => {
  { cpu.sub8((cpu.edx as u8), 0x94u8); }
  if !cpu.zf { pc = 0x41e6fd; continue; }
  pc = 0x41e6e2; continue;
 }
 0x41e6e2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3eeu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e6fd => {
  { cpu.sub8((cpu.edx as u8), 0x81u8); }
  if !cpu.zf { pc = 0x41e71d; continue; }
  pc = 0x41e702; continue;
 }
 0x41e702 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3efu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e71d => {
  { cpu.sub8((cpu.edx as u8), 0x85u8); }
  if !cpu.zf { pc = 0x41e73d; continue; }
  pc = 0x41e722; continue;
 }
 0x41e722 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3fdu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e73d => {
  { cpu.sub8((cpu.edx as u8), 0x83u8); }
  if !cpu.zf { pc = 0x41e75d; continue; }
  pc = 0x41e742; continue;
 }
 0x41e742 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3feu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e75d => {
  { cpu.sub8((cpu.edx as u8), 0x87u8); }
  if !cpu.zf { pc = 0x41e77d; continue; }
  pc = 0x41e762; continue;
 }
 0x41e762 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3ffu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e77d => {
  { cpu.sub8((cpu.edx as u8), 0x82u8); }
  if !cpu.zf { pc = 0x41e79d; continue; }
  pc = 0x41e782; continue;
 }
 0x41e782 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e79d => {
  { cpu.sub8((cpu.edx as u8), 0x8au8); }
  if !cpu.zf { pc = 0x41e7bd; continue; }
  pc = 0x41e7a2; continue;
 }
 0x41e7a2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x401u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e7bd => {
  { cpu.sub8((cpu.edx as u8), 0x88u8); }
  if !cpu.zf { pc = 0x41e7dd; continue; }
  pc = 0x41e7c2; continue;
 }
 0x41e7c2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x402u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e7dd => {
  { cpu.sub8((cpu.edx as u8), 0x8bu8); }
  if !cpu.zf { pc = 0x41e7fd; continue; }
  pc = 0x41e7e2; continue;
 }
 0x41e7e2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x403u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e7fd => {
  { cpu.sub8((cpu.edx as u8), 0x8cu8); }
  pc = 0x41e800; continue;
 }
 0x41e800 => {
  if !cpu.zf { pc = 0x41e81d; continue; }
  pc = 0x41e802; continue;
 }
 0x41e802 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x404u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e81d => {
  { cpu.sub8((cpu.edx as u8), 0x93u8); }
  if !cpu.zf { pc = 0x41e83d; continue; }
  pc = 0x41e822; continue;
 }
 0x41e822 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x405u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e831 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(4))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(4)), v); }
  { let v: u32 = cpu.eax.wrapping_add(0xe9000004u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.eax); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x41e83d; continue;
 }
 0x41e83d => {
  { cpu.sub8((cpu.edx as u8), 0x96u8); }
  if !cpu.zf { pc = 0x41e85d; continue; }
  pc = 0x41e842; continue;
 }
 0x41e842 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x406u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e85d => {
  { cpu.sub8((cpu.edx as u8), 0xa3u8); }
  if !cpu.zf { pc = 0x41e87d; continue; }
  pc = 0x41e862; continue;
 }
 0x41e862 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x407u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e87d => {
  { cpu.sub8((cpu.edx as u8), 0x91u8); }
  if !cpu.zf { pc = 0x41e89d; continue; }
  pc = 0x41e882; continue;
 }
 0x41e882 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x408u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e89d => {
  { cpu.sub8((cpu.edx as u8), 0xa0u8); }
  if !cpu.zf { pc = 0x41e8bd; continue; }
  pc = 0x41e8a2; continue;
 }
 0x41e8a2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x40fu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e8bd => {
  { cpu.sub8((cpu.edx as u8), 0xa1u8); }
  if !cpu.zf { pc = 0x41e8dd; continue; }
  pc = 0x41e8c2; continue;
 }
 0x41e8c2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x3u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x410u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e8dd => {
  { cpu.sub8((cpu.edx as u8), 0xa2u8); }
  if !cpu.zf { pc = 0x41e8fd; continue; }
  pc = 0x41e8e2; continue;
 }
 0x41e8e2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x411u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e8fd => {
  { cpu.sub8((cpu.edx as u8), 0x95u8); }
  if !cpu.zf { pc = 0x41e91d; continue; }
  pc = 0x41e902; continue;
 }
 0x41e902 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x416u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e91d => {
  { cpu.sub8((cpu.edx as u8), 0x97u8); }
  if !cpu.zf { pc = 0x41e93d; continue; }
  pc = 0x41e922; continue;
 }
 0x41e922 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x417u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e93d => {
  { cpu.sub8((cpu.edx as u8), 0x92u8); }
  if !cpu.zf { pc = 0x41e95d; continue; }
  pc = 0x41e942; continue;
 }
 0x41e942 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3fcu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e95d => {
  { cpu.sub8((cpu.edx as u8), 0x8bu8); }
  if !cpu.zf { pc = 0x41e97d; continue; }
  pc = 0x41e962; continue;
 }
 0x41e962 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3f7u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e97d => {
  { cpu.sub8((cpu.edx as u8), 0x8cu8); }
  if !cpu.zf { pc = 0x41e99d; continue; }
  pc = 0x41e982; continue;
 }
 0x41e982 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3f8u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e99d => {
  { cpu.sub8((cpu.edx as u8), 0x93u8); }
  if !cpu.zf { pc = 0x41e9bd; continue; }
  pc = 0x41e9a2; continue;
 }
 0x41e9a2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3f9u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e9bd => {
  { cpu.sub8((cpu.edx as u8), 0x96u8); }
  if !cpu.zf { pc = 0x41e9dd; continue; }
  pc = 0x41e9c2; continue;
 }
 0x41e9c2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3fau32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e9dd => {
  { cpu.sub8((cpu.edx as u8), 0xa3u8); }
  if !cpu.zf { pc = 0x41e9fd; continue; }
  pc = 0x41e9e2; continue;
 }
 0x41e9e2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3fbu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41e9fd => {
  { cpu.sub8((cpu.edx as u8), 0x7fu8); }
  if !cpu.zf { pc = 0x41ea1d; continue; }
  pc = 0x41ea02; continue;
 }
 0x41ea02 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3f1u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ea1d => {
  { cpu.sub8((cpu.edx as u8), 0x83u8); }
  if !cpu.zf { pc = 0x41ea3d; continue; }
  pc = 0x41ea22; continue;
 }
 0x41ea22 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3f2u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ea3d => {
  { cpu.sub8((cpu.edx as u8), 0x80u8); }
  if !cpu.zf { pc = 0x41ea5d; continue; }
  pc = 0x41ea42; continue;
 }
 0x41ea42 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3f3u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ea5d => {
  { cpu.sub8((cpu.edx as u8), 0x82u8); }
  if !cpu.zf { pc = 0x41ea7d; continue; }
  pc = 0x41ea62; continue;
 }
 0x41ea62 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3f4u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ea7d => {
  { cpu.sub8((cpu.edx as u8), 0x8au8); }
  if !cpu.zf { pc = 0x41ea9d; continue; }
  pc = 0x41ea82; continue;
 }
 0x41ea82 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3f5u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ea9d => {
  { cpu.sub8((cpu.edx as u8), 0x88u8); }
  if !cpu.zf { pc = 0x41eabd; continue; }
  pc = 0x41eaa2; continue;
 }
 0x41eaa2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3f6u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41eabd => {
  { cpu.sub8((cpu.edx as u8), 0x8fu8); }
  if !cpu.zf { pc = 0x41eadd; continue; }
  pc = 0x41eac2; continue;
 }
 0x41eac2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x40cu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41eadd => {
  { cpu.sub8((cpu.edx as u8), 0xa1u8); }
  if !cpu.zf { pc = 0x41eafd; continue; }
  pc = 0x41eae2; continue;
 }
 0x41eae2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x40du32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41eafd => {
  { cpu.sub8((cpu.edx as u8), 0xa2u8); }
  if !cpu.zf { pc = 0x41eb1d; continue; }
  pc = 0x41eb02; continue;
 }
 0x41eb02 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x40eu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41eb1d => {
  { cpu.sub8((cpu.edx as u8), 0x95u8); }
  if !cpu.zf { pc = 0x41eb3d; continue; }
  pc = 0x41eb22; continue;
 }
 0x41eb22 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x414u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41eb3d => {
  { cpu.sub8((cpu.edx as u8), 0x97u8); }
  if !cpu.zf { pc = 0x41eb5d; continue; }
  pc = 0x41eb42; continue;
 }
 0x41eb42 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x415u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41eb5d => {
  { cpu.sub8((cpu.edx as u8), 0x8eu8); }
  if !cpu.zf { pc = 0x41eb7d; continue; }
  pc = 0x41eb62; continue;
 }
 0x41eb62 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3eau32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41eb7d => {
  { cpu.sub8((cpu.edx as u8), 0x99u8); }
  if !cpu.zf { pc = 0x41eb9d; continue; }
  pc = 0x41eb82; continue;
 }
 0x41eb82 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3ebu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41eb9d => {
  { cpu.sub8((cpu.edx as u8), 0x9au8); }
  if !cpu.zf { pc = 0x41ebbd; continue; }
  pc = 0x41eba2; continue;
 }
 0x41eba2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3ecu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ebbd => {
  { cpu.sub8((cpu.edx as u8), 0xe1u8); }
  if !cpu.zf { pc = 0x41ebdd; continue; }
  pc = 0x41ebc2; continue;
 }
 0x41ebc2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x3u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x3f0u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ebdd => {
  { cpu.sub8((cpu.edx as u8), 0xf8u8); }
  if !cpu.zf { pc = 0x41ebfd; continue; }
  pc = 0x41ebe2; continue;
 }
 0x41ebe2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x409u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ebfd => {
  { cpu.sub8((cpu.edx as u8), 0xaeu8); }
  if !cpu.zf { pc = 0x41ec1d; continue; }
  pc = 0x41ec02; continue;
 }
 0x41ec02 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x40au32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ec1d => {
  { cpu.sub8((cpu.edx as u8), 0xafu8); }
  if !cpu.zf { pc = 0x41ec3d; continue; }
  pc = 0x41ec22; continue;
 }
 0x41ec22 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x40bu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ec3d => {
  { cpu.sub8((cpu.edx as u8), 0xa8u8); }
  if !cpu.zf { pc = 0x41ec5d; continue; }
  pc = 0x41ec42; continue;
 }
 0x41ec42 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x412u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ec5d => {
  { cpu.sub8((cpu.edx as u8), 0xadu8); }
  if !cpu.zf { pc = 0x41ec7d; continue; }
  pc = 0x41ec62; continue;
 }
 0x41ec62 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x2u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x413u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ec7d => {
  { cpu.sub8((cpu.edx as u8), 0xc6u8); }
  if !cpu.zf { pc = 0x41ec9d; continue; }
  pc = 0x41ec82; continue;
 }
 0x41ec82 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x460u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ec9d => {
  { cpu.sub8((cpu.edx as u8), 0xc7u8); }
  if !cpu.zf { pc = 0x41ecbd; continue; }
  pc = 0x41eca2; continue;
 }
 0x41eca2 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x45fu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ecbd => {
  { cpu.sub8((cpu.edx as u8), 0xe4u8); }
  if !cpu.zf { pc = 0x41ecdd; continue; }
  pc = 0x41ecc2; continue;
 }
 0x41ecc2 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x462u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ecdd => {
  { cpu.sub8((cpu.edx as u8), 0xe5u8); }
  if !cpu.zf { pc = 0x41ecfd; continue; }
  pc = 0x41ece2; continue;
 }
 0x41ece2 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x461u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ecfd => {
  { cpu.sub8((cpu.edx as u8), 0xa5u8); }
  if !cpu.zf { pc = 0x41ed1d; continue; }
  pc = 0x41ed02; continue;
 }
 0x41ed02 => {
  { let v: u32 = 0x3u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x446u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ed1d => {
  { cpu.sub8((cpu.edx as u8), 0xa4u8); }
  if !cpu.zf { pc = 0x41ed3d; continue; }
  pc = 0x41ed22; continue;
 }
 0x41ed22 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x447u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ed3d => {
  { cpu.sub8((cpu.edx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x41ed5d; continue; }
  pc = 0x41ed42; continue;
 }
 0x41ed42 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x8u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x448u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ed5d => {
  { cpu.sub8((cpu.edx as u8), 0x23u8); }
  if !cpu.zf { pc = 0x41ed7d; continue; }
  pc = 0x41ed62; continue;
 }
 0x41ed62 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x449u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ed7d => {
  { cpu.sub8((cpu.edx as u8), 0x24u8); }
  if !cpu.zf { pc = 0x41ed9d; continue; }
  pc = 0x41ed82; continue;
 }
 0x41ed82 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x44au32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ed9d => {
  { cpu.sub8((cpu.edx as u8), 0x5fu8); }
  if !cpu.zf { pc = 0x41edbd; continue; }
  pc = 0x41eda2; continue;
 }
 0x41eda2 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x44bu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41edbd => {
  { cpu.sub8((cpu.edx as u8), 0x26u8); }
  if !cpu.zf { pc = 0x41eddd; continue; }
  pc = 0x41edc2; continue;
 }
 0x41edc2 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x8u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x44cu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41eddd => {
  { cpu.sub8((cpu.edx as u8), 0x7eu8); }
  if !cpu.zf { pc = 0x41edfd; continue; }
  pc = 0x41ede2; continue;
 }
 0x41ede2 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x44du32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41edfd => {
  { cpu.sub8((cpu.edx as u8), 0x5eu8); }
  if !cpu.zf { pc = 0x41ee1d; continue; }
  pc = 0x41ee02; continue;
 }
 0x41ee02 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x44eu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ee1d => {
  { cpu.sub8((cpu.edx as u8), 0x5cu8); }
  if !cpu.zf { pc = 0x41ee3a; continue; }
  pc = 0x41ee22; continue;
 }
 0x41ee22 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x44fu32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ee3a => {
  { cpu.sub8((cpu.edx as u8), 0x7cu8); }
  if !cpu.zf { pc = 0x41ee57; continue; }
  pc = 0x41ee3f; continue;
 }
 0x41ee3f => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x2u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x450u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ee57 => {
  { cpu.sub8((cpu.edx as u8), 0x9cu8); }
  if !cpu.zf { pc = 0x41ee74; continue; }
  pc = 0x41ee5c; continue;
 }
 0x41ee5c => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x8u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x451u32; cpu.ecx = v; }
  pc = 0x41ee76; continue;
 }
 0x41ee74 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41ee76 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041ee78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41ee78); }
 let mut pc: u32 = 0x41ee78;
 loop { match pc {
 0x41ee78 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = 0x3u32; cpu.w32(0x581a04u32, v); }
  { cpu.sub8((cpu.edx as u8), 0x84u8); }
  if !cpu.zf { pc = 0x41eeac; continue; }
  pc = 0x41ee91; continue;
 }
 0x41ee91 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x13u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x195au32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41eeac => {
  { cpu.sub8((cpu.edx as u8), 0x94u8); }
  if !cpu.zf { pc = 0x41eecc; continue; }
  pc = 0x41eeb1; continue;
 }
 0x41eeb1 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x12u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x195bu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41eecc => {
  { cpu.sub8((cpu.edx as u8), 0x81u8); }
  if !cpu.zf { pc = 0x41eeec; continue; }
  pc = 0x41eed1; continue;
 }
 0x41eed1 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x14u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x195cu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41eeec => {
  { cpu.sub8((cpu.edx as u8), 0x85u8); }
  if !cpu.zf { pc = 0x41ef0c; continue; }
  pc = 0x41eef1; continue;
 }
 0x41eef1 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x13u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x195du32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41ef0c => {
  { cpu.sub8((cpu.edx as u8), 0x83u8); }
  if !cpu.zf { pc = 0x41ef2c; continue; }
  pc = 0x41ef11; continue;
 }
 0x41ef11 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x12u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x196au32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41ef2c => {
  { cpu.sub8((cpu.edx as u8), 0x82u8); }
  if !cpu.zf { pc = 0x41ef4c; continue; }
  pc = 0x41ef31; continue;
 }
 0x41ef31 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xfu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x195fu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41ef4c => {
  { cpu.sub8((cpu.edx as u8), 0x8au8); }
  if !cpu.zf { pc = 0x41ef6c; continue; }
  pc = 0x41ef51; continue;
 }
 0x41ef51 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xfu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x195eu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41ef6c => {
  { cpu.sub8((cpu.edx as u8), 0x88u8); }
  if !cpu.zf { pc = 0x41ef8c; continue; }
  pc = 0x41ef71; continue;
 }
 0x41ef71 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xeu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1960u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41ef8c => {
  { cpu.sub8((cpu.edx as u8), 0x8bu8); }
  if !cpu.zf { pc = 0x41efac; continue; }
  pc = 0x41ef91; continue;
 }
 0x41ef91 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xeu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1961u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41efac => {
  { cpu.sub8((cpu.edx as u8), 0x8cu8); }
  if !cpu.zf { pc = 0x41efcc; continue; }
  pc = 0x41efb1; continue;
 }
 0x41efb1 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xau32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x196bu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41efcc => {
  { cpu.sub8((cpu.edx as u8), 0x93u8); }
  if !cpu.zf { pc = 0x41efec; continue; }
  pc = 0x41efd1; continue;
 }
 0x41efd1 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x11u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1966u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41efec => {
  { cpu.sub8((cpu.edx as u8), 0x96u8); }
  if !cpu.zf { pc = 0x41f00c; continue; }
  pc = 0x41eff1; continue;
 }
 0x41eff1 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x14u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1965u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f00c => {
  { cpu.sub8((cpu.edx as u8), 0xa3u8); }
  if !cpu.zf { pc = 0x41f02c; continue; }
  pc = 0x41f011; continue;
 }
 0x41f011 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x13u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1964u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f02c => {
  { cpu.sub8((cpu.edx as u8), 0xa0u8); }
  if !cpu.zf { pc = 0x41f04c; continue; }
  pc = 0x41f031; continue;
 }
 0x41f031 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x13u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1968u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f04c => {
  { cpu.sub8((cpu.edx as u8), 0xa1u8); }
  if !cpu.zf { pc = 0x41f06c; continue; }
  pc = 0x41f051; continue;
 }
 0x41f051 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x9u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1962u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f06c => {
  { cpu.sub8((cpu.edx as u8), 0xa2u8); }
  if !cpu.zf { pc = 0x41f08c; continue; }
  pc = 0x41f071; continue;
 }
 0x41f071 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x12u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1967u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f08c => {
  { cpu.sub8((cpu.edx as u8), 0x95u8); }
  if !cpu.zf { pc = 0x41f0ac; continue; }
  pc = 0x41f091; continue;
 }
 0x41f091 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x12u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1963u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f0ac => {
  { cpu.sub8((cpu.edx as u8), 0x97u8); }
  if !cpu.zf { pc = 0x41f0cc; continue; }
  pc = 0x41f0b1; continue;
 }
 0x41f0b1 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x14u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x196cu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f0cc => {
  { cpu.sub8((cpu.edx as u8), 0x8eu8); }
  if !cpu.zf { pc = 0x41f0ec; continue; }
  pc = 0x41f0d1; continue;
 }
 0x41f0d1 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x13u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x195au32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f0ec => {
  { cpu.sub8((cpu.edx as u8), 0x99u8); }
  if !cpu.zf { pc = 0x41f10c; continue; }
  pc = 0x41f0f1; continue;
 }
 0x41f0f1 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x12u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x195bu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f10c => {
  { cpu.sub8((cpu.edx as u8), 0x9au8); }
  if !cpu.zf { pc = 0x41f12c; continue; }
  pc = 0x41f111; continue;
 }
 0x41f111 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x14u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x195cu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f12c => {
  { cpu.sub8((cpu.edx as u8), 0x7fu8); }
  if !cpu.zf { pc = 0x41f14c; continue; }
  pc = 0x41f131; continue;
 }
 0x41f131 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x13u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x195du32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f14c => {
  { cpu.sub8((cpu.edx as u8), 0x83u8); }
  if !cpu.zf { pc = 0x41f16c; continue; }
  pc = 0x41f151; continue;
 }
 0x41f151 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x12u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x196au32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f16c => {
  { cpu.sub8((cpu.edx as u8), 0x80u8); }
  if !cpu.zf { pc = 0x41f18c; continue; }
  pc = 0x41f171; continue;
 }
 0x41f171 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x10u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1969u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f18c => {
  { cpu.sub8((cpu.edx as u8), 0x87u8); }
  if !cpu.zf { pc = 0x41f1ac; continue; }
  pc = 0x41f191; continue;
 }
 0x41f191 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x10u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1969u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f1ac => {
  { cpu.sub8((cpu.edx as u8), 0x82u8); }
  if !cpu.zf { pc = 0x41f1cc; continue; }
  pc = 0x41f1b1; continue;
 }
 0x41f1b1 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xfu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x195fu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f1cc => {
  { cpu.sub8((cpu.edx as u8), 0x8au8); }
  if !cpu.zf { pc = 0x41f1ec; continue; }
  pc = 0x41f1d1; continue;
 }
 0x41f1d1 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xfu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x195eu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f1ec => {
  { cpu.sub8((cpu.edx as u8), 0x88u8); }
  if !cpu.zf { pc = 0x41f20c; continue; }
  pc = 0x41f1f1; continue;
 }
 0x41f1f1 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xeu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1960u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f20c => {
  { cpu.sub8((cpu.edx as u8), 0x8bu8); }
  if !cpu.zf { pc = 0x41f22c; continue; }
  pc = 0x41f211; continue;
 }
 0x41f211 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xeu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1961u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f22c => {
  { cpu.sub8((cpu.edx as u8), 0x8cu8); }
  if !cpu.zf { pc = 0x41f24c; continue; }
  pc = 0x41f231; continue;
 }
 0x41f231 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xau32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x196bu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f24c => {
  { cpu.sub8((cpu.edx as u8), 0x93u8); }
  if !cpu.zf { pc = 0x41f26c; continue; }
  pc = 0x41f251; continue;
 }
 0x41f251 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x11u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1966u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f26c => {
  { cpu.sub8((cpu.edx as u8), 0x96u8); }
  if !cpu.zf { pc = 0x41f28c; continue; }
  pc = 0x41f271; continue;
 }
 0x41f271 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x14u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1965u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f28c => {
  { cpu.sub8((cpu.edx as u8), 0xa3u8); }
  if !cpu.zf { pc = 0x41f2ac; continue; }
  pc = 0x41f291; continue;
 }
 0x41f291 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x13u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1964u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f2ac => {
  { cpu.sub8((cpu.edx as u8), 0x8fu8); }
  if !cpu.zf { pc = 0x41f2cc; continue; }
  pc = 0x41f2b1; continue;
 }
 0x41f2b1 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x13u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1968u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f2cc => {
  { cpu.sub8((cpu.edx as u8), 0xa1u8); }
  if !cpu.zf { pc = 0x41f2ec; continue; }
  pc = 0x41f2d1; continue;
 }
 0x41f2d1 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x9u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1962u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f2ec => {
  { cpu.sub8((cpu.edx as u8), 0xa2u8); }
  if !cpu.zf { pc = 0x41f30c; continue; }
  pc = 0x41f2f1; continue;
 }
 0x41f2f1 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x12u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1967u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f30c => {
  { cpu.sub8((cpu.edx as u8), 0x95u8); }
  if !cpu.zf { pc = 0x41f32c; continue; }
  pc = 0x41f311; continue;
 }
 0x41f311 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x12u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1963u32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f32c => {
  { cpu.sub8((cpu.edx as u8), 0x97u8); }
  if !cpu.zf { pc = 0x41f34c; continue; }
  pc = 0x41f331; continue;
 }
 0x41f331 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x14u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x196cu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f34c => {
  { cpu.sub8((cpu.edx as u8), 0xc7u8); }
  if !cpu.zf { pc = 0x41f36c; continue; }
  pc = 0x41f351; continue;
 }
 0x41f351 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x13u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x196eu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f36c => {
  { cpu.sub8((cpu.edx as u8), 0xe5u8); }
  if !cpu.zf { pc = 0x41f38c; continue; }
  pc = 0x41f371; continue;
 }
 0x41f371 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x13u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x196fu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f38c => {
  { cpu.sub8((cpu.edx as u8), 0xc6u8); }
  if !cpu.zf { pc = 0x41f3a9; continue; }
  pc = 0x41f391; continue;
 }
 0x41f391 => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x13u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x196eu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f3a9 => {
  { cpu.sub8((cpu.edx as u8), 0xe4u8); }
  if !cpu.zf { pc = 0x41f3c6; continue; }
  pc = 0x41f3ae; continue;
 }
 0x41f3ae => {
  { let v: u32 = 0x5u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x13u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x196fu32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f3c6 => {
  { cpu.sub8((cpu.edx as u8), 0x26u8); }
  if !cpu.zf { pc = 0x41f3e3; continue; }
  pc = 0x41f3cb; continue;
 }
 0x41f3cb => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x13u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x196du32; cpu.ecx = v; }
  pc = 0x41f3e5; continue;
 }
 0x41f3e3 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41f3e5 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041f3e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x41f3e7); }
 let mut pc: u32 = 0x41f3e7;
 loop { match pc {
 0x41f3e7 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = 0x3u32; cpu.w32(0x581a04u32, v); }
  { cpu.sub8((cpu.edx as u8), 0x84u8); }
  if !cpu.zf { pc = 0x41f41b; continue; }
  pc = 0x41f400; continue;
 }
 0x41f400 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xdu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19a3u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f41b => {
  { cpu.sub8((cpu.edx as u8), 0x94u8); }
  if !cpu.zf { pc = 0x41f43b; continue; }
  pc = 0x41f420; continue;
 }
 0x41f420 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xbu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19a4u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f43b => {
  { cpu.sub8((cpu.edx as u8), 0x81u8); }
  if !cpu.zf { pc = 0x41f45b; continue; }
  pc = 0x41f440; continue;
 }
 0x41f440 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xcu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19a5u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f45b => {
  { cpu.sub8((cpu.edx as u8), 0x85u8); }
  if !cpu.zf { pc = 0x41f47b; continue; }
  pc = 0x41f460; continue;
 }
 0x41f460 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xdu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19a6u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f47b => {
  { cpu.sub8((cpu.edx as u8), 0x83u8); }
  if !cpu.zf { pc = 0x41f49b; continue; }
  pc = 0x41f480; continue;
 }
 0x41f480 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xdu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19b3u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f49b => {
  { cpu.sub8((cpu.edx as u8), 0x87u8); }
  if !cpu.zf { pc = 0x41f4bb; continue; }
  pc = 0x41f4a0; continue;
 }
 0x41f4a0 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xbu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19b2u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f4bb => {
  { cpu.sub8((cpu.edx as u8), 0x82u8); }
  if !cpu.zf { pc = 0x41f4db; continue; }
  pc = 0x41f4c0; continue;
 }
 0x41f4c0 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xbu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19a8u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f4db => {
  { cpu.sub8((cpu.edx as u8), 0x8au8); }
  if !cpu.zf { pc = 0x41f4fb; continue; }
  pc = 0x41f4e0; continue;
 }
 0x41f4e0 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xbu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19a7u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f4fb => {
  { cpu.sub8((cpu.edx as u8), 0x88u8); }
  if !cpu.zf { pc = 0x41f51b; continue; }
  pc = 0x41f500; continue;
 }
 0x41f500 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xbu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19a9u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f51b => {
  { cpu.sub8((cpu.edx as u8), 0x8bu8); }
  if !cpu.zf { pc = 0x41f53b; continue; }
  pc = 0x41f520; continue;
 }
 0x41f520 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x8u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19aau32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f53b => {
  { cpu.sub8((cpu.edx as u8), 0x8cu8); }
  if !cpu.zf { pc = 0x41f55b; continue; }
  pc = 0x41f540; continue;
 }
 0x41f540 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19b4u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f55b => {
  { cpu.sub8((cpu.edx as u8), 0x93u8); }
  if !cpu.zf { pc = 0x41f57b; continue; }
  pc = 0x41f560; continue;
 }
 0x41f560 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xcu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19afu32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f57b => {
  { cpu.sub8((cpu.edx as u8), 0x96u8); }
  if !cpu.zf { pc = 0x41f59b; continue; }
  pc = 0x41f580; continue;
 }
 0x41f580 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xcu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19aeu32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f59b => {
  { cpu.sub8((cpu.edx as u8), 0xa3u8); }
  if !cpu.zf { pc = 0x41f5bb; continue; }
  pc = 0x41f5a0; continue;
 }
 0x41f5a0 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xcu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19adu32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f5bb => {
  { cpu.sub8((cpu.edx as u8), 0xa0u8); }
  if !cpu.zf { pc = 0x41f5db; continue; }
  pc = 0x41f5c0; continue;
 }
 0x41f5c0 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xdu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19b1u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f5db => {
  { cpu.sub8((cpu.edx as u8), 0xa1u8); }
  if !cpu.zf { pc = 0x41f5fb; continue; }
  pc = 0x41f5e0; continue;
 }
 0x41f5e0 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19abu32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f5fb => {
  { cpu.sub8((cpu.edx as u8), 0xa2u8); }
  if !cpu.zf { pc = 0x41f61b; continue; }
  pc = 0x41f600; continue;
 }
 0x41f600 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xcu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19b0u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f61b => {
  { cpu.sub8((cpu.edx as u8), 0x95u8); }
  if !cpu.zf { pc = 0x41f63b; continue; }
  pc = 0x41f620; continue;
 }
 0x41f620 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xcu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19acu32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f63b => {
  { cpu.sub8((cpu.edx as u8), 0x8eu8); }
  if !cpu.zf { pc = 0x41f65b; continue; }
  pc = 0x41f640; continue;
 }
 0x41f640 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xdu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19a3u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f65b => {
  { cpu.sub8((cpu.edx as u8), 0x99u8); }
  if !cpu.zf { pc = 0x41f67b; continue; }
  pc = 0x41f660; continue;
 }
 0x41f660 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xbu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19a4u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f67b => {
  { cpu.sub8((cpu.edx as u8), 0x9au8); }
  if !cpu.zf { pc = 0x41f69b; continue; }
  pc = 0x41f680; continue;
 }
 0x41f680 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xcu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19a5u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f69b => {
  { cpu.sub8((cpu.edx as u8), 0x7fu8); }
  if !cpu.zf { pc = 0x41f6bb; continue; }
  pc = 0x41f6a0; continue;
 }
 0x41f6a0 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xdu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19a6u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f6bb => {
  { cpu.sub8((cpu.edx as u8), 0x83u8); }
  if !cpu.zf { pc = 0x41f6db; continue; }
  pc = 0x41f6c0; continue;
 }
 0x41f6c0 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xdu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19b3u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f6db => {
  { cpu.sub8((cpu.edx as u8), 0x80u8); }
  if !cpu.zf { pc = 0x41f6fb; continue; }
  pc = 0x41f6e0; continue;
 }
 0x41f6e0 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xbu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19b2u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f6fb => {
  { cpu.sub8((cpu.edx as u8), 0x87u8); }
  if !cpu.zf { pc = 0x41f71b; continue; }
  pc = 0x41f700; continue;
 }
 0x41f700 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x10u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1969u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f71b => {
  { cpu.sub8((cpu.edx as u8), 0x82u8); }
  if !cpu.zf { pc = 0x41f73b; continue; }
  pc = 0x41f720; continue;
 }
 0x41f720 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xbu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19a8u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f73b => {
  { cpu.sub8((cpu.edx as u8), 0x8au8); }
  if !cpu.zf { pc = 0x41f75b; continue; }
  pc = 0x41f740; continue;
 }
 0x41f740 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xbu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19a7u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f75b => {
  { cpu.sub8((cpu.edx as u8), 0x88u8); }
  if !cpu.zf { pc = 0x41f77b; continue; }
  pc = 0x41f760; continue;
 }
 0x41f760 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19a9u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f77b => {
  { cpu.sub8((cpu.edx as u8), 0x8bu8); }
  if !cpu.zf { pc = 0x41f79b; continue; }
  pc = 0x41f780; continue;
 }
 0x41f780 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19aau32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f79b => {
  { cpu.sub8((cpu.edx as u8), 0x93u8); }
  if !cpu.zf { pc = 0x41f7bb; continue; }
  pc = 0x41f7a0; continue;
 }
 0x41f7a0 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xcu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19afu32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f7bb => {
  { cpu.sub8((cpu.edx as u8), 0x96u8); }
  if !cpu.zf { pc = 0x41f7db; continue; }
  pc = 0x41f7c0; continue;
 }
 0x41f7c0 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xcu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19aeu32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f7db => {
  { cpu.sub8((cpu.edx as u8), 0xa3u8); }
  if !cpu.zf { pc = 0x41f7fb; continue; }
  pc = 0x41f7e0; continue;
 }
 0x41f7e0 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xcu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19adu32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f7fb => {
  { cpu.sub8((cpu.edx as u8), 0x8fu8); }
  if !cpu.zf { pc = 0x41f81b; continue; }
  pc = 0x41f800; continue;
 }
 0x41f800 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xdu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19b1u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f81b => {
  { cpu.sub8((cpu.edx as u8), 0xa1u8); }
  if !cpu.zf { pc = 0x41f83b; continue; }
  pc = 0x41f820; continue;
 }
 0x41f820 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19abu32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f83b => {
  { cpu.sub8((cpu.edx as u8), 0xa2u8); }
  if !cpu.zf { pc = 0x41f85b; continue; }
  pc = 0x41f840; continue;
 }
 0x41f840 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xcu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19b0u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f85b => {
  { cpu.sub8((cpu.edx as u8), 0x95u8); }
  if !cpu.zf { pc = 0x41f87b; continue; }
  pc = 0x41f860; continue;
 }
 0x41f860 => {
  { let v: u32 = 0x4u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xcu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19acu32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f87b => {
  { cpu.sub8((cpu.edx as u8), 0xc7u8); }
  if !cpu.zf { pc = 0x41f89b; continue; }
  pc = 0x41f880; continue;
 }
 0x41f880 => {
  { let v: u32 = 0x3u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xdu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19b5u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f89b => {
  { cpu.sub8((cpu.edx as u8), 0xe5u8); }
  if !cpu.zf { pc = 0x41f8b8; continue; }
  pc = 0x41f8a0; continue;
 }
 0x41f8a0 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xdu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19b6u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f8b8 => {
  { cpu.sub8((cpu.edx as u8), 0xc6u8); }
  if !cpu.zf { pc = 0x41f8d5; continue; }
  pc = 0x41f8bd; continue;
 }
 0x41f8bd => {
  { let v: u32 = 0x3u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xdu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19b5u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f8d5 => {
  { cpu.sub8((cpu.edx as u8), 0xe4u8); }
  if !cpu.zf { pc = 0x41f8f2; continue; }
  pc = 0x41f8da; continue;
 }
 0x41f8da => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0xdu32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x19b6u32; cpu.ecx = v; }
  pc = 0x41f8f4; continue;
 }
 0x41f8f2 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x41f8f4 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0041f8f6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x41f8f6); } r_0041f8f6(cpu, 0x41f8f6); }
pub fn f_00420000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x420000); } r_0041f8f6(cpu, 0x420000); }
pub fn f_00420001(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x420001); } r_0041f8f6(cpu, 0x420001); }
pub fn f_00420002(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x420002); } r_0041f8f6(cpu, 0x420002); }
pub fn f_00420003(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x420003); } r_0041f8f6(cpu, 0x420003); }
pub fn f_0042000f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42000f); } r_0041f8f6(cpu, 0x42000f); }
pub fn f_00420016(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x420016); } r_0041f8f6(cpu, 0x420016); }
pub fn f_00420020(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x420020); } r_0041f8f6(cpu, 0x420020); }
pub fn f_00420041(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x420041); } r_0041f8f6(cpu, 0x420041); }
pub fn f_00420042(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x420042); } r_0041f8f6(cpu, 0x420042); }
pub fn f_00420048(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x420048); } r_0041f8f6(cpu, 0x420048); }
pub fn f_00420053(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x420053); } r_0041f8f6(cpu, 0x420053); }
pub fn f_00420056(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x420056); } r_0041f8f6(cpu, 0x420056); }
pub fn f_00420057(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x420057); } r_0041f8f6(cpu, 0x420057); }
pub fn f_0042005b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42005b); } r_0041f8f6(cpu, 0x42005b); }
pub fn f_0042007b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x42007b); } r_0041f8f6(cpu, 0x42007b); }
pub fn f_00420080(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x420080); } r_0041f8f6(cpu, 0x420080); }
fn r_0041f8f6(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x41f8f6 => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x581a04u32, v); }
  { cpu.sub8((cpu.edx as u8), 0x84u8); }
  if !cpu.zf { pc = 0x41f92a; continue; }
  pc = 0x41f90f; continue;
 }
 0x41f90f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c5au32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41f92a => {
  { cpu.sub8((cpu.edx as u8), 0x94u8); }
  if !cpu.zf { pc = 0x41f94a; continue; }
  pc = 0x41f92f; continue;
 }
 0x41f92f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c5bu32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41f94a => {
  { cpu.sub8((cpu.edx as u8), 0x81u8); }
  if !cpu.zf { pc = 0x41f96a; continue; }
  pc = 0x41f94f; continue;
 }
 0x41f94f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c5cu32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41f96a => {
  { cpu.sub8((cpu.edx as u8), 0x85u8); }
  if !cpu.zf { pc = 0x41f98a; continue; }
  pc = 0x41f96f; continue;
 }
 0x41f96f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c6au32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41f98a => {
  { cpu.sub8((cpu.edx as u8), 0x83u8); }
  if !cpu.zf { pc = 0x41f9aa; continue; }
  pc = 0x41f98f; continue;
 }
 0x41f98f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c6bu32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41f9aa => {
  { cpu.sub8((cpu.edx as u8), 0x87u8); }
  if !cpu.zf { pc = 0x41f9ca; continue; }
  pc = 0x41f9af; continue;
 }
 0x41f9af => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c6cu32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41f9ca => {
  { cpu.sub8((cpu.edx as u8), 0x82u8); }
  if !cpu.zf { pc = 0x41f9ea; continue; }
  pc = 0x41f9cf; continue;
 }
 0x41f9cf => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c6du32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41f9ea => {
  { cpu.sub8((cpu.edx as u8), 0x8au8); }
  if !cpu.zf { pc = 0x41fa0a; continue; }
  pc = 0x41f9ef; continue;
 }
 0x41f9ef => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c6eu32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fa0a => {
  { cpu.sub8((cpu.edx as u8), 0x88u8); }
  if !cpu.zf { pc = 0x41fa2a; continue; }
  pc = 0x41fa0f; continue;
 }
 0x41fa0f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c6fu32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fa2a => {
  { cpu.sub8((cpu.edx as u8), 0x8bu8); }
  if !cpu.zf { pc = 0x41fa4a; continue; }
  pc = 0x41fa2f; continue;
 }
 0x41fa2f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c70u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fa4a => {
  { cpu.sub8((cpu.edx as u8), 0x8cu8); }
  if !cpu.zf { pc = 0x41fa6a; continue; }
  pc = 0x41fa4f; continue;
 }
 0x41fa4f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c71u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fa6a => {
  { cpu.sub8((cpu.edx as u8), 0x93u8); }
  if !cpu.zf { pc = 0x41fa8a; continue; }
  pc = 0x41fa6f; continue;
 }
 0x41fa6f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c72u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fa8a => {
  { cpu.sub8((cpu.edx as u8), 0x96u8); }
  if !cpu.zf { pc = 0x41faaa; continue; }
  pc = 0x41fa8f; continue;
 }
 0x41fa8f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c73u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41faaa => {
  { cpu.sub8((cpu.edx as u8), 0xa3u8); }
  if !cpu.zf { pc = 0x41faca; continue; }
  pc = 0x41faaf; continue;
 }
 0x41faaf => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c74u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41faca => {
  { cpu.sub8((cpu.edx as u8), 0xa0u8); }
  if !cpu.zf { pc = 0x41faea; continue; }
  pc = 0x41facf; continue;
 }
 0x41facf => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c7cu32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41faea => {
  { cpu.sub8((cpu.edx as u8), 0xa1u8); }
  if !cpu.zf { pc = 0x41fb0a; continue; }
  pc = 0x41faef; continue;
 }
 0x41faef => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x3u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c7du32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fb0a => {
  { cpu.sub8((cpu.edx as u8), 0xa2u8); }
  if !cpu.zf { pc = 0x41fb2a; continue; }
  pc = 0x41fb0f; continue;
 }
 0x41fb0f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c7eu32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fb2a => {
  { cpu.sub8((cpu.edx as u8), 0x95u8); }
  if !cpu.zf { pc = 0x41fb4a; continue; }
  pc = 0x41fb2f; continue;
 }
 0x41fb2f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c83u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fb4a => {
  { cpu.sub8((cpu.edx as u8), 0x97u8); }
  if !cpu.zf { pc = 0x41fb6a; continue; }
  pc = 0x41fb4f; continue;
 }
 0x41fb4f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c84u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fb6a => {
  { cpu.sub8((cpu.edx as u8), 0x91u8); }
  if !cpu.zf { pc = 0x41fb8a; continue; }
  pc = 0x41fb6f; continue;
 }
 0x41fb6f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c69u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fb8a => {
  { cpu.sub8((cpu.edx as u8), 0x8eu8); }
  if !cpu.zf { pc = 0x41fbaa; continue; }
  pc = 0x41fb8f; continue;
 }
 0x41fb8f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c57u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fbaa => {
  { cpu.sub8((cpu.edx as u8), 0x99u8); }
  if !cpu.zf { pc = 0x41fbca; continue; }
  pc = 0x41fbaf; continue;
 }
 0x41fbaf => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c58u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fbca => {
  { cpu.sub8((cpu.edx as u8), 0x9au8); }
  if !cpu.zf { pc = 0x41fbea; continue; }
  pc = 0x41fbcf; continue;
 }
 0x41fbcf => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c59u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fbea => {
  { cpu.sub8((cpu.edx as u8), 0x7fu8); }
  if !cpu.zf { pc = 0x41fc0a; continue; }
  pc = 0x41fbef; continue;
 }
 0x41fbef => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c5eu32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fc0a => {
  { cpu.sub8((cpu.edx as u8), 0x83u8); }
  if !cpu.zf { pc = 0x41fc2a; continue; }
  pc = 0x41fc0f; continue;
 }
 0x41fc0f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c5fu32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fc2a => {
  { cpu.sub8((cpu.edx as u8), 0x80u8); }
  if !cpu.zf { pc = 0x41fc4a; continue; }
  pc = 0x41fc2f; continue;
 }
 0x41fc2f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c60u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fc4a => {
  { cpu.sub8((cpu.edx as u8), 0x82u8); }
  if !cpu.zf { pc = 0x41fc6a; continue; }
  pc = 0x41fc4f; continue;
 }
 0x41fc4f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c61u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fc6a => {
  { cpu.sub8((cpu.edx as u8), 0x8au8); }
  if !cpu.zf { pc = 0x41fc8a; continue; }
  pc = 0x41fc6f; continue;
 }
 0x41fc6f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c62u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fc8a => {
  { cpu.sub8((cpu.edx as u8), 0x88u8); }
  if !cpu.zf { pc = 0x41fcaa; continue; }
  pc = 0x41fc8f; continue;
 }
 0x41fc8f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c63u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fcaa => {
  { cpu.sub8((cpu.edx as u8), 0x8bu8); }
  if !cpu.zf { pc = 0x41fcca; continue; }
  pc = 0x41fcaf; continue;
 }
 0x41fcaf => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c70u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fcca => {
  { cpu.sub8((cpu.edx as u8), 0x93u8); }
  if !cpu.zf { pc = 0x41fcea; continue; }
  pc = 0x41fccf; continue;
 }
 0x41fccf => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c66u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fcea => {
  { cpu.sub8((cpu.edx as u8), 0x96u8); }
  if !cpu.zf { pc = 0x41fd0a; continue; }
  pc = 0x41fcef; continue;
 }
 0x41fcef => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c67u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fd0a => {
  { cpu.sub8((cpu.edx as u8), 0xa3u8); }
  if !cpu.zf { pc = 0x41fd2a; continue; }
  pc = 0x41fd0f; continue;
 }
 0x41fd0f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c68u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fd2a => {
  { cpu.sub8((cpu.edx as u8), 0x8fu8); }
  if !cpu.zf { pc = 0x41fd4a; continue; }
  pc = 0x41fd2f; continue;
 }
 0x41fd2f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c79u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fd4a => {
  { cpu.sub8((cpu.edx as u8), 0xa1u8); }
  if !cpu.zf { pc = 0x41fd6a; continue; }
  pc = 0x41fd4f; continue;
 }
 0x41fd4f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c7au32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fd6a => {
  { cpu.sub8((cpu.edx as u8), 0xa2u8); }
  if !cpu.zf { pc = 0x41fd8a; continue; }
  pc = 0x41fd6f; continue;
 }
 0x41fd6f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c7bu32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fd8a => {
  { cpu.sub8((cpu.edx as u8), 0x95u8); }
  if !cpu.zf { pc = 0x41fdaa; continue; }
  pc = 0x41fd8f; continue;
 }
 0x41fd8f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c81u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fdaa => {
  { cpu.sub8((cpu.edx as u8), 0x97u8); }
  if !cpu.zf { pc = 0x41fdca; continue; }
  pc = 0x41fdaf; continue;
 }
 0x41fdaf => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c82u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fdca => {
  { cpu.sub8((cpu.edx as u8), 0x92u8); }
  if !cpu.zf { pc = 0x41fdea; continue; }
  pc = 0x41fdcf; continue;
 }
 0x41fdcf => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c75u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fdea => {
  { cpu.sub8((cpu.edx as u8), 0xf8u8); }
  if !cpu.zf { pc = 0x41fe0a; continue; }
  pc = 0x41fdef; continue;
 }
 0x41fdef => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c76u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fe0a => {
  { cpu.sub8((cpu.edx as u8), 0xe1u8); }
  if !cpu.zf { pc = 0x41fe2a; continue; }
  pc = 0x41fe0f; continue;
 }
 0x41fe0f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c5du32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fe2a => {
  { cpu.sub8((cpu.edx as u8), 0xaeu8); }
  if !cpu.zf { pc = 0x41fe4a; continue; }
  pc = 0x41fe2f; continue;
 }
 0x41fe2f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c77u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fe4a => {
  { cpu.sub8((cpu.edx as u8), 0xafu8); }
  if !cpu.zf { pc = 0x41fe6a; continue; }
  pc = 0x41fe4f; continue;
 }
 0x41fe4f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c78u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fe6a => {
  { cpu.sub8((cpu.edx as u8), 0xa8u8); }
  if !cpu.zf { pc = 0x41fe8a; continue; }
  pc = 0x41fe6f; continue;
 }
 0x41fe6f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c7fu32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41fe8a => {
  { cpu.sub8((cpu.edx as u8), 0xadu8); }
  if !cpu.zf { pc = 0x41feaa; continue; }
  pc = 0x41fe8f; continue;
 }
 0x41fe8f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x2u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c80u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41feaa => {
  { cpu.sub8((cpu.edx as u8), 0xc7u8); }
  if !cpu.zf { pc = 0x41feca; continue; }
  pc = 0x41feaf; continue;
 }
 0x41feaf => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c85u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41feca => {
  { cpu.sub8((cpu.edx as u8), 0xe5u8); }
  if !cpu.zf { pc = 0x41feea; continue; }
  pc = 0x41fecf; continue;
 }
 0x41fecf => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c86u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41feea => {
  { cpu.sub8((cpu.edx as u8), 0xc6u8); }
  if !cpu.zf { pc = 0x41ff0a; continue; }
  pc = 0x41feef; continue;
 }
 0x41feef => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c87u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41ff0a => {
  { cpu.sub8((cpu.edx as u8), 0xe4u8); }
  if !cpu.zf { pc = 0x41ff2a; continue; }
  pc = 0x41ff0f; continue;
 }
 0x41ff0f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x5u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c88u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41ff2a => {
  { cpu.sub8((cpu.edx as u8), 0xa5u8); }
  if !cpu.zf { pc = 0x41ff4a; continue; }
  pc = 0x41ff2f; continue;
 }
 0x41ff2f => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x4u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c10u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41ff4a => {
  { cpu.sub8((cpu.edx as u8), 0xa4u8); }
  if !cpu.zf { pc = 0x41ff6a; continue; }
  pc = 0x41ff4f; continue;
 }
 0x41ff4f => {
  { let v: u32 = 0x0u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x3u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c11u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41ff6a => {
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x41ff8a; continue; }
  pc = 0x41ff6f; continue;
 }
 0x41ff6f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x8u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c12u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41ff8a => {
  { cpu.sub8((cpu.edx as u8), 0x23u8); }
  if !cpu.zf { pc = 0x41ffaa; continue; }
  pc = 0x41ff8f; continue;
 }
 0x41ff8f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c13u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41ffaa => {
  { cpu.sub8((cpu.edx as u8), 0x24u8); }
  if !cpu.zf { pc = 0x41ffca; continue; }
  pc = 0x41ffaf; continue;
 }
 0x41ffaf => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c14u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41ffca => {
  { cpu.sub8((cpu.edx as u8), 0x5fu8); }
  if !cpu.zf { pc = 0x41ffea; continue; }
  pc = 0x41ffcf; continue;
 }
 0x41ffcf => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c15u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x41ffea => {
  { cpu.sub8((cpu.edx as u8), 0x26u8); }
  if !cpu.zf { pc = 0x42000a; continue; }
  pc = 0x41ffef; continue;
 }
 0x41ffef => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x8u32); cpu.w32(0x581a04u32, v); }
  pc = 0x420000; continue;
 }
 0x420000 => {
  { let v: u32 = 0x1c16u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x420001 => {
  { cpu.push(0x2b); }
  pc = 0x420002; continue;
 }
 0x420002 => {
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x1b757efau32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x1b757efau32.wrapping_add(cpu.eax), v); }
  pc = 0x42000f; continue;
 }
 0x420003 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4200a3; continue;
 }
 0x42000a => {
  { cpu.sub8((cpu.edx as u8), 0x7eu8); }
  if !cpu.zf { pc = 0x42002a; continue; }
  pc = 0x42000f; continue;
 }
 0x42000f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  pc = 0x420020; continue;
 }
 0x420016 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x581a0405u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x581a0405u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x420020; continue;
 }
 0x420020 => {
  { let v: u32 = 0x1c17u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x42002a => {
  { cpu.sub8((cpu.edx as u8), 0x5eu8); }
  if !cpu.zf { pc = 0x42004a; continue; }
  pc = 0x42002f; continue;
 }
 0x42002f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x6u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c18u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x420041 => {
  { let v: u8 = cpu.sbb8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))), (cpu.ebx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x18755cfau32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x18755cfau32.wrapping_add(cpu.eax), v); }
  pc = 0x42004f; continue;
 }
 0x420042 => {
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x18755cfau32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x18755cfau32.wrapping_add(cpu.eax), v); }
  pc = 0x42004f; continue;
 }
 0x420048 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x42004a; continue;
 }
 0x42004a => {
  { cpu.sub8((cpu.edx as u8), 0x5cu8); }
  if !cpu.zf { pc = 0x420067; continue; }
  pc = 0x42004f; continue;
 }
 0x42004f => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c19u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x420053 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x420056; continue;
 }
 0x420056 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x581a0405u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x581a0405u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1c19u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x420057 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x7u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c19u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x42005b => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1c19u32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x420067 => {
  { cpu.sub8((cpu.edx as u8), 0x7cu8); }
  if !cpu.zf { pc = 0x420084; continue; }
  pc = 0x42006c; continue;
 }
 0x42006c => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x2u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c1au32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x42007b => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1c1au32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x420080 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4200a3; continue;
 }
 0x420084 => {
  { cpu.sub8((cpu.edx as u8), 0x9cu8); }
  if !cpu.zf { pc = 0x4200a1; continue; }
  pc = 0x420089; continue;
 }
 0x420089 => {
  { let v: u32 = 0x1u32; cpu.w32(0x578d6bu32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x581a04u32), 0x8u32); cpu.w32(0x581a04u32, v); }
  { let v: u32 = 0x1c1bu32; cpu.ecx = v; }
  pc = 0x4200a3; continue;
 }
 0x4200a1 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4200a3 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
