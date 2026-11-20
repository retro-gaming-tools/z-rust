#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_01087c64(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1087c64); } r_01087c64(cpu, 0x1087c64); }
pub fn f_01087d74(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1087d74); } r_01087c64(cpu, 0x1087d74); }
pub fn f_01087d80(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1087d80); } r_01087c64(cpu, 0x1087d80); }
fn r_01087c64(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1087c64 => {
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x1087d31; continue; }
  pc = 0x1087c6f; continue;
 }
 0x1087c6f => {
  cpu.push(0x1087c74); f_01087bd7(cpu); if cpu.returned_to != 0x1087c74 { return; }
  if cpu.cf { pc = 0x1087cd0; continue; }
  pc = 0x1087c76; continue;
 }
 0x1087c76 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1087c81); f_01087f84(cpu); if cpu.returned_to != 0x1087c81 { return; }
  if cpu.cf { pc = 0x1087cd9; continue; }
  pc = 0x1087c83; continue;
 }
 0x1087c83 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1087c8a); f_01087f84(cpu); if cpu.returned_to != 0x1087c8a { return; }
  if cpu.cf { pc = 0x1087cd9; continue; }
  pc = 0x1087c8c; continue;
 }
 0x1087c8c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.push(0x1087c94); f_0108bb7a(cpu); if cpu.returned_to != 0x1087c94 { return; }
  { let v: u8 = cpu.r8(0x1133820u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1087cce; continue; }
  pc = 0x1087c9e; continue;
 }
 0x1087c9e => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x11eb140u32; cpu.esi = v; }
  pc = 0x1087ca8; continue;
 }
 0x1087ca8 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x1087cc7; continue; }
  pc = 0x1087cac; continue;
 }
 0x1087cac => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1087cb9); f_0108a681(cpu); if cpu.returned_to != 0x1087cb9 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.push(0x1087cc3); f_0108834c(cpu); if cpu.returned_to != 0x1087cc3 { return; }
  { cpu.popad(); }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x1087cce; continue;
 }
 0x1087cc7 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1087ca8; continue; }
  pc = 0x1087cce; continue;
 }
 0x1087cce => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1087cd0 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x1087cd8); f_01087c1d(cpu); if cpu.returned_to != 0x1087cd8 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1087cd9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.push(0x1087ce1); f_0108bb7a(cpu); if cpu.returned_to != 0x1087ce1 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x1133820u32), 0x0u32); }
  if cpu.zf { pc = 0x1087d04; continue; }
  pc = 0x1087ceb; continue;
 }
 0x1087ceb => {
  { let v: u8 = cpu.r8(0x1133820u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x11eb140u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  pc = 0x1087cf9; continue;
 }
 0x1087cf9 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.zf { pc = 0x1087d05; continue; }
  pc = 0x1087cfd; continue;
 }
 0x1087cfd => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x4u32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1087cf9; continue; }
  pc = 0x1087d04; continue;
 }
 0x1087d04 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1087d05 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1087d0d); f_0108a681(cpu); if cpu.returned_to != 0x1087d0d { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x1087d1f); f_0108b988(cpu); if cpu.returned_to != 0x1087d1f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1087d31 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1087d04; continue; }
  pc = 0x1087d37; continue;
 }
 0x1087d37 => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.push(0x1087d41); f_01087f84(cpu); if cpu.returned_to != 0x1087d41 { return; }
  if cpu.cf { cpu.poll(); pc = 0x1087d04; continue; }
  pc = 0x1087d43; continue;
 }
 0x1087d43 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1087d4c); f_0108a681(cpu); if cpu.returned_to != 0x1087d4c { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.pushad(); }
  cpu.push(0x1087d5a); f_010884f1(cpu); if cpu.returned_to != 0x1087d5a { return; }
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x1087d67); f_0108b988(cpu); if cpu.returned_to != 0x1087d67 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1087d74 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1087e00; continue; }
  pc = 0x1087d7e; continue;
 }
 0x1087d7e => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x1087e00; continue; }
  pc = 0x1087d84; continue;
 }
 0x1087d80 => {
  { let v: u32 = cpu.adc32(cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.esi.wrapping_mul(2))), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.esi.wrapping_mul(2)), v); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1087d05; continue; }
  pc = 0x1087d85; continue;
 }
 0x1087d84 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x1087e00; continue; }
  pc = 0x1087d8a; continue;
 }
 0x1087d85 => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1087d98; continue; }
  pc = 0x1087d87; continue;
 }
 0x1087d87 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x7e807674u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x1087da8; continue; }
  pc = 0x1087d90; continue;
 }
 0x1087d8a => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1087da8; continue; }
  pc = 0x1087d90; continue;
 }
 0x1087d90 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.zf { pc = 0x1087da8; continue; }
  pc = 0x1087d96; continue;
 }
 0x1087d96 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.zf { pc = 0x1087da8; continue; }
  pc = 0x1087d9c; continue;
 }
 0x1087d98 => {
  { let v: u32 = cpu.adc32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x1087da8; continue; }
  pc = 0x1087d9c; continue;
 }
 0x1087d9c => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x1087da8; continue; }
  pc = 0x1087da2; continue;
 }
 0x1087da2 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x1087e00; continue; }
  pc = 0x1087da8; continue;
 }
 0x1087da8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1087e00; continue; }
  pc = 0x1087db6; continue;
 }
 0x1087db6 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x1087e00; continue; }
  pc = 0x1087dbb; continue;
 }
 0x1087dbb => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if cpu.zf { pc = 0x1087e01; continue; }
  pc = 0x1087dc1; continue;
 }
 0x1087dc1 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x1087e00; continue; }
  pc = 0x1087dc5; continue;
 }
 0x1087dc5 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.push(0x1087dcf); f_01087fc5(cpu); if cpu.returned_to != 0x1087dcf { return; }
  if cpu.cf { pc = 0x1087e00; continue; }
  pc = 0x1087dd1; continue;
 }
 0x1087dd1 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1087dd8); f_0108a6b9(cpu); if cpu.returned_to != 0x1087dd8 { return; }
  { cpu.sub8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1087e00; continue; }
  pc = 0x1087dde; continue;
 }
 0x1087dde => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1087de5); f_0108a681(cpu); if cpu.returned_to != 0x1087de5 { return; }
  if cpu.cf { pc = 0x1087e00; continue; }
  pc = 0x1087de7; continue;
 }
 0x1087de7 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.esi = v; }
  cpu.push(0x1087df5); f_010881a9(cpu); if cpu.returned_to != 0x1087df5 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x1087e00; continue;
 }
 0x1087e00 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1087e01 => {
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1087e11); f_0108a681(cpu); if cpu.returned_to != 0x1087e11 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x1087e19); f_01088a90(cpu); if cpu.returned_to != 0x1087e19 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1087e20); f_01087e6c(cpu); if cpu.returned_to != 0x1087e20 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01087e21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1087e21); }
 let mut pc: u32 = 0x1087e21;
 loop { match pc {
 0x1087e21 => {
  { let v: u32 = 0x116e734u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1087e28; continue;
 }
 0x1087e28 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1087e57; continue; }
  pc = 0x1087e2e; continue;
 }
 0x1087e2e => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x1087e57; continue; }
  pc = 0x1087e34; continue;
 }
 0x1087e34 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x1087e57; continue; }
  pc = 0x1087e3a; continue;
 }
 0x1087e3a => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { pc = 0x1087e57; continue; }
  pc = 0x1087e40; continue;
 }
 0x1087e40 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if !cpu.zf { pc = 0x1087e5f; continue; }
  pc = 0x1087e49; continue;
 }
 0x1087e49 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x1087e50); f_01087f41(cpu); if cpu.returned_to != 0x1087e50 { return; }
  { cpu.sub32(cpu.eax, 0x1f4u32); }
  if (cpu.sf != cpu.of) { pc = 0x1087e5f; continue; }
  pc = 0x1087e57; continue;
 }
 0x1087e57 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1087e28; continue; }
  pc = 0x1087e5e; continue;
 }
 0x1087e5e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1087e5f => {
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x1087e57; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01087e6c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1087e6c); } r_01087e6c(cpu, 0x1087e6c); }
pub fn f_01087e80(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1087e80); } r_01087e6c(cpu, 0x1087e80); }
fn r_01087e6c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1087e6c => {
  { cpu.sub32(cpu.r32(0x19u32.wrapping_add(cpu.eax)), 0x1f4u32); }
  if cpu.cf { pc = 0x1087ed1; continue; }
  pc = 0x1087e75; continue;
 }
 0x1087e75 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x1087e7c); f_01087f41(cpu); if cpu.returned_to != 0x1087e7c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1087ed1; continue; }
  pc = 0x1087e80; continue;
 }
 0x1087e80 => {
  cpu.push(0x1087e85); f_01087ed2(cpu); if cpu.returned_to != 0x1087e85 { return; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x1087ed1; continue; }
  pc = 0x1087e89; continue;
 }
 0x1087e89 => {
  { let v: u32 = 0x116e734u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1087e92; continue;
 }
 0x1087e92 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1087eca; continue; }
  pc = 0x1087e98; continue;
 }
 0x1087e98 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x1087eca; continue; }
  pc = 0x1087e9c; continue;
 }
 0x1087e9c => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x1087eca; continue; }
  pc = 0x1087ea2; continue;
 }
 0x1087ea2 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x1087eca; continue; }
  pc = 0x1087ea8; continue;
 }
 0x1087ea8 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { pc = 0x1087eca; continue; }
  pc = 0x1087eae; continue;
 }
 0x1087eae => {
  { cpu.and8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if cpu.zf { pc = 0x1087ec0; continue; }
  pc = 0x1087eb2; continue;
 }
 0x1087eb2 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.edx = v; }
  if !cpu.sf { pc = 0x1087ec0; continue; }
  pc = 0x1087eb9; continue;
 }
 0x1087eb9 => {
  cpu.push(0x1087ebe); f_01087f1e(cpu); if cpu.returned_to != 0x1087ebe { return; }
  pc = 0x1087ed1; continue;
 }
 0x1087ec0 => {
  cpu.push(0x1087ec5); f_01087f1e(cpu); if cpu.returned_to != 0x1087ec5 { return; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  pc = 0x1087eca; continue;
 }
 0x1087eca => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1087e92; continue; }
  pc = 0x1087ed1; continue;
 }
 0x1087ed1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01087ed2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1087ed2); }
 let mut pc: u32 = 0x1087ed2;
 loop { match pc {
 0x1087ed2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x116e734u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1087ee1; continue;
 }
 0x1087ee1 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1087f10; continue; }
  pc = 0x1087ee7; continue;
 }
 0x1087ee7 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1087eee); f_0108a681(cpu); if cpu.returned_to != 0x1087eee { return; }
  if cpu.cf { pc = 0x1087f10; continue; }
  pc = 0x1087ef0; continue;
 }
 0x1087ef0 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x2u8); }
  if cpu.zf { pc = 0x1087f0d; continue; }
  pc = 0x1087ef6; continue;
 }
 0x1087ef6 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x1087f10; continue; }
  pc = 0x1087efc; continue;
 }
 0x1087efc => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x1087f10; continue; }
  pc = 0x1087f02; continue;
 }
 0x1087f02 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { pc = 0x1087f10; continue; }
  pc = 0x1087f08; continue;
 }
 0x1087f08 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x1087f10; continue; }
  pc = 0x1087f0d; continue;
 }
 0x1087f0d => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.edx = v; }
  pc = 0x1087f10; continue;
 }
 0x1087f10 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1087ee1; continue; }
  pc = 0x1087f17; continue;
 }
 0x1087f17 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01087f1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1087f1e); }
 let mut pc: u32 = 0x1087f1e;
 loop { match pc {
 0x1087f1e => {
  { let v: u8 = 0x4u8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1087f34); f_0108a681(cpu); if cpu.returned_to != 0x1087f34 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x1087f3f); f_0108b988(cpu); if cpu.returned_to != 0x1087f3f { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01087f41(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1087f41); } r_01087f41(cpu, 0x1087f41); }
pub fn f_01087f80(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1087f80); } r_01087f41(cpu, 0x1087f80); }
fn r_01087f41(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1087f41 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133844u32); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x64u32); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.r32(0x1133844u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x113383cu32)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if !cpu.zf { pc = 0x1087f67; continue; }
  pc = 0x1087f62; continue;
 }
 0x1087f62 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x1087f67; continue;
 }
 0x1087f67 => {
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = 0x64u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x19u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1087f80; continue;
 }
 0x1087f80 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01087f84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1087f84); }
 let mut pc: u32 = 0x1087f84;
 loop { match pc {
 0x1087f84 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x116e734u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1087f90; continue;
 }
 0x1087f90 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x1087fba; continue; }
  pc = 0x1087f96; continue;
 }
 0x1087f96 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0x8u8); }
  if cpu.zf { pc = 0x1087fba; continue; }
  pc = 0x1087f9c; continue;
 }
 0x1087f9c => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0x10u8); }
  if cpu.zf { pc = 0x1087fba; continue; }
  pc = 0x1087fa2; continue;
 }
 0x1087fa2 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x11u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x1087fba; continue; }
  pc = 0x1087fa7; continue;
 }
 0x1087fa7 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x1087fba; continue; }
  pc = 0x1087fab; continue;
 }
 0x1087fab => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edi)), cpu.ebx); }
  if !cpu.zf { pc = 0x1087fba; continue; }
  pc = 0x1087fb0; continue;
 }
 0x1087fb0 => {
  { let v: u32 = cpu.edi; cpu.w32(0x11eb128u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1087fba => {
  { let v: u32 = cpu.edi.wrapping_add(0x16u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1087f90; continue; }
  pc = 0x1087fc1; continue;
 }
 0x1087fc1 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01087fc5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1087fc5); }
 let mut pc: u32 = 0x1087fc5;
 loop { match pc {
 0x1087fc5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x116e734u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1087fd1; continue;
 }
 0x1087fd1 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1087ff8; continue; }
  pc = 0x1087fd7; continue;
 }
 0x1087fd7 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x1087ff8; continue; }
  pc = 0x1087fdd; continue;
 }
 0x1087fdd => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x1087ff8; continue; }
  pc = 0x1087fe3; continue;
 }
 0x1087fe3 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1087ff8; continue; }
  pc = 0x1087fe8; continue;
 }
 0x1087fe8 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x1087ff2; continue; }
  pc = 0x1087fed; continue;
 }
 0x1087fed => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), cpu.ebx); }
  if !cpu.zf { pc = 0x1087ff8; continue; }
  pc = 0x1087ff2; continue;
 }
 0x1087ff2 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1087ff8 => {
  { let v: u32 = cpu.esi.wrapping_add(0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1087fd1; continue; }
  pc = 0x1087fff; continue;
 }
 0x1087fff => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088003(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088003); }
 let mut pc: u32 = 0x1088003;
 loop { match pc {
 0x1088003 => {
  { cpu.pushad(); }
  pc = 0x1088004; continue;
 }
 0x1088004 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x108800e); f_01087fc5(cpu); if cpu.returned_to != 0x108800e { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x1088017; continue; }
  pc = 0x1088011; continue;
 }
 0x1088011 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x1088004; continue;
 }
 0x1088017 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088019(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088019); }
 let mut pc: u32 = 0x1088019;
 loop { match pc {
 0x1088019 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1088128; continue; }
  pc = 0x1088023; continue;
 }
 0x1088023 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if !cpu.zf { pc = 0x108811f; continue; }
  pc = 0x108802f; continue;
 }
 0x108802f => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x1088128; continue; }
  pc = 0x1088039; continue;
 }
 0x1088039 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1088128; continue; }
  pc = 0x1088043; continue;
 }
 0x1088043 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x1088128; continue; }
  pc = 0x108804d; continue;
 }
 0x108804d => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x1088128; continue; }
  pc = 0x1088057; continue;
 }
 0x1088057 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x1088072; continue; }
  pc = 0x108805d; continue;
 }
 0x108805d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1088067); f_01087b3a(cpu); if cpu.returned_to != 0x1088067 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x1088128; continue; }
  pc = 0x1088072; continue;
 }
 0x1088072 => {
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1088081; continue; }
  pc = 0x1088079; continue;
 }
 0x1088079 => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x1088126; continue; }
  pc = 0x1088081; continue;
 }
 0x1088081 => {
  { cpu.pushad(); }
  { let v: u32 = 0x116e734u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x108808c; continue;
 }
 0x108808c => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x10880a6; continue; }
  pc = 0x1088092; continue;
 }
 0x1088092 => {
  { cpu.sub32(cpu.edi, cpu.esi); }
  if cpu.zf { pc = 0x10880a6; continue; }
  pc = 0x1088096; continue;
 }
 0x1088096 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x10880a6; continue; }
  pc = 0x108809a; continue;
 }
 0x108809a => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x10880a6; continue; }
  pc = 0x10880a0; continue;
 }
 0x10880a0 => {
  { cpu.popad(); }
  pc = 0x1088126; continue;
 }
 0x10880a6 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x16u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108808c; continue; }
  pc = 0x10880ac; continue;
 }
 0x10880ac => {
  { cpu.popad(); }
  cpu.push(0x10880b2); f_01088171(cpu); if cpu.returned_to != 0x10880b2 { return; }
  if cpu.cf { pc = 0x1088126; continue; }
  pc = 0x10880b4; continue;
 }
 0x10880b4 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebx)), 0x20u8); }
  if cpu.zf { pc = 0x10880c9; continue; }
  pc = 0x10880ba; continue;
 }
 0x10880ba => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10880c2); f_0108a681(cpu); if cpu.returned_to != 0x10880c2 { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x1u8); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x1088126; continue; }
  pc = 0x10880c9; continue;
 }
 0x10880c9 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.ebx)), 0x40u8); }
  if cpu.zf { pc = 0x1088100; continue; }
  pc = 0x10880cf; continue;
 }
 0x10880cf => {
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10880d7); f_0108a681(cpu); if cpu.returned_to != 0x10880d7 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x10880e9; continue; }
  pc = 0x10880e0; continue;
 }
 0x10880e0 => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x1088126; continue; }
  pc = 0x10880e9; continue;
 }
 0x10880e9 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x1088126; continue; }
  pc = 0x10880ef; continue;
 }
 0x10880ef => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x1088100; continue; }
  pc = 0x10880f5; continue;
 }
 0x10880f5 => {
  { cpu.pushad(); }
  cpu.push(0x10880fb); f_01068d2a(cpu); if cpu.returned_to != 0x10880fb { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x1088126; continue; }
  pc = 0x1088100; continue;
 }
 0x1088100 => {
  cpu.push(0x1088105); f_0108812a(cpu); if cpu.returned_to != 0x1088105 { return; }
  if cpu.cf { pc = 0x1088126; continue; }
  pc = 0x1088107; continue;
 }
 0x1088107 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1088110); f_0108a681(cpu); if cpu.returned_to != 0x1088110 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  cpu.push(0x108811c); f_01088219(cpu); if cpu.returned_to != 0x108811c { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1088128; continue;
 }
 0x108811f => {
  cpu.push(0x1088124); f_01087625(cpu); if cpu.returned_to != 0x1088124 { return; }
  if !cpu.cf { cpu.poll(); pc = 0x1088107; continue; }
  pc = 0x1088126; continue;
 }
 0x1088126 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1088128 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108812a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108812a); }
 let mut pc: u32 = 0x108812a;
 loop { match pc {
 0x108812a => {
  { cpu.pushad(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1088134); f_0108a681(cpu); if cpu.returned_to != 0x1088134 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x6400u32); }
  { cpu.popad(); }
  { cpu.cf = !cpu.cf; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088171(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088171); }
 let mut pc: u32 = 0x1088171;
 loop { match pc {
 0x1088171 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x108817c); f_0108a6b9(cpu); if cpu.returned_to != 0x108817c { return; }
  { let v: u32 = 0x13u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = 0x15u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1088183; continue;
 }
 0x1088183 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0xffu8); }
  if cpu.zf { pc = 0x108819e; continue; }
  pc = 0x1088188; continue;
 }
 0x1088188 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { cpu.and8(cpu.r8(0x1152edbu32.wrapping_add(cpu.eax)), 0x4u8); }
  if cpu.zf { pc = 0x108819e; continue; }
  pc = 0x1088198; continue;
 }
 0x1088198 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108819e => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1088183; continue; }
  pc = 0x10881a3; continue;
 }
 0x10881a3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010881a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10881a9); }
 let mut pc: u32 = 0x10881a9;
 loop { match pc {
 0x10881a9 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x10881cd; continue; }
  pc = 0x10881c6; continue;
 }
 0x10881c6 => {
  cpu.push(0x10881cb); f_01088219(cpu); if cpu.returned_to != 0x10881cb { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10881cd => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11336c4u32, v); }
  cpu.push(0x10881fc); f_01036a75(cpu); if cpu.returned_to != 0x10881fc { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11336c4u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088205(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088205); }
 let mut pc: u32 = 0x1088205;
 loop { match pc {
 0x1088205 => {
  cpu.push(0x108820a); f_0106a6aa(cpu); if cpu.returned_to != 0x108820a { return; }
  if cpu.cf { pc = 0x1088217; continue; }
  pc = 0x108820c; continue;
 }
 0x108820c => {
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x1088217; continue; }
  pc = 0x1088215; continue;
 }
 0x1088215 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1088217 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088219(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088219); }
 let mut pc: u32 = 0x1088219;
 loop { match pc {
 0x1088219 => {
  { cpu.pushad(); }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x1088227; continue; }
  pc = 0x1088220; continue;
 }
 0x1088220 => {
  cpu.push(0x1088225); f_010664e4(cpu); if cpu.returned_to != 0x1088225 { return; }
  if !cpu.cf { pc = 0x1088229; continue; }
  pc = 0x1088227; continue;
 }
 0x1088227 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x1088229; continue;
 }
 0x1088229 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x1088230); f_01088205(cpu); if cpu.returned_to != 0x1088230 { return; }
  if !cpu.cf { pc = 0x1088234; continue; }
  pc = 0x1088232; continue;
 }
 0x1088232 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1088234 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x1088263; continue; }
  pc = 0x1088250; continue;
 }
 0x1088250 => {
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x48u32); cpu.ebx = v; }
  pc = 0x1088263; continue;
 }
 0x1088263 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x1088297; continue; }
  pc = 0x1088269; continue;
 }
 0x1088269 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1088271); f_010507b9(cpu); if cpu.returned_to != 0x1088271 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x1088283; continue; }
  pc = 0x108827a; continue;
 }
 0x108827a => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x78u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1088283); f_01050829(cpu); if cpu.returned_to != 0x1088283 { return; }
  pc = 0x1088283; continue;
 }
 0x1088283 => {
  cpu.push(0x1088288); f_0106a2d3(cpu); if cpu.returned_to != 0x1088288 { return; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1088297; continue; }
  pc = 0x108828e; continue;
 }
 0x108828e => {
  { let v: u8 = cpu.or8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x4u8); cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1088297 => {
  cpu.push(0x108829c); f_0108a641(cpu); if cpu.returned_to != 0x108829c { return; }
  if cpu.cf { pc = 0x10882a7; continue; }
  pc = 0x108829e; continue;
 }
 0x108829e => {
  { let v: u8 = 0x1u8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x10882a7; continue;
 }
 0x10882a7 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010882a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10882a9); }
 let mut pc: u32 = 0x10882a9;
 loop { match pc {
 0x10882a9 => {
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x10882b3; continue;
 }
 0x10882b3 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x10882e9; continue; }
  pc = 0x10882bc; continue;
 }
 0x10882bc => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x10882e9; continue; }
  pc = 0x10882c2; continue;
 }
 0x10882c2 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10882d1); f_0108c1c5(cpu); if cpu.returned_to != 0x10882d1 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x10882e9; continue; }
  pc = 0x10882d9; continue;
 }
 0x10882d9 => {
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10882e7); f_010882f3(cpu); if cpu.returned_to != 0x10882e7 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x10882e9; continue;
 }
 0x10882e9 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10882b3; continue; }
  pc = 0x10882f2; continue;
 }
 0x10882f2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010882f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10882f3); }
 let mut pc: u32 = 0x10882f3;
 loop { match pc {
 0x10882f3 => {
  cpu.push(0x10882f8); f_01088205(cpu); if cpu.returned_to != 0x10882f8 { return; }
  if cpu.cf { pc = 0x108834b; continue; }
  pc = 0x10882fa; continue;
 }
 0x10882fa => {
  cpu.push(0x10882ff); f_0106a326(cpu); if cpu.returned_to != 0x10882ff { return; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x108834b; continue; }
  pc = 0x1088305; continue;
 }
 0x1088305 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f84u32, v); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x9u32.wrapping_add(cpu.eax)), 0x1u8); cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1088326); f_010673d5(cpu); if cpu.returned_to != 0x1088326 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x9u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x108834b; continue; }
  pc = 0x1088336; continue;
 }
 0x1088336 => {
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  cpu.push(0x1088346); f_010507b9(cpu); if cpu.returned_to != 0x1088346 { return; }
  cpu.push(0x108834b); f_0106a2d3(cpu); if cpu.returned_to != 0x108834b { return; }
  pc = 0x108834b; continue;
 }
 0x108834b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108834c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108834c); }
 let mut pc: u32 = 0x108834c;
 loop { match pc {
 0x108834c => {
  { cpu.pushad(); }
  cpu.push(0x1088352); f_01088205(cpu); if cpu.returned_to != 0x1088352 { return; }
  if cpu.cf { pc = 0x1088399; continue; }
  pc = 0x1088354; continue;
 }
 0x1088354 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x1088383; continue; }
  pc = 0x1088370; continue;
 }
 0x1088370 => {
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x48u32); cpu.ebx = v; }
  pc = 0x1088383; continue;
 }
 0x1088383 => {
  { cpu.pushad(); }
  cpu.push(0x1088389); f_01086768(cpu); if cpu.returned_to != 0x1088389 { return; }
  cpu.push(0x108838e); f_01086779(cpu); if cpu.returned_to != 0x108838e { return; }
  { cpu.popad(); }
  cpu.push(0x1088394); f_010507b9(cpu); if cpu.returned_to != 0x1088394 { return; }
  cpu.push(0x1088399); f_0106a2d3(cpu); if cpu.returned_to != 0x1088399 { return; }
  pc = 0x1088399; continue;
 }
 0x1088399 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108839b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108839b); }
 let mut pc: u32 = 0x108839b;
 loop { match pc {
 0x108839b => {
  { let v: u32 = 0x11ea774u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11eb0f4u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10883bb; continue; }
  pc = 0x10883aa; continue;
 }
 0x10883aa => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.push(0x10883b3); f_010883bc(cpu); if cpu.returned_to != 0x10883b3 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10883aa; continue; }
  pc = 0x10883bb; continue;
 }
 0x10883bb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010883bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10883bc); }
 let mut pc: u32 = 0x10883bc;
 loop { match pc {
 0x10883bc => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10883c2); f_01088682(cpu); if cpu.returned_to != 0x10883c2 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x10883d5; continue; }
  pc = 0x10883c5; continue;
 }
 0x10883c5 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10883cb); f_010885dc(cpu); if cpu.returned_to != 0x10883cb { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x10883d1); f_010883d7(cpu); if cpu.returned_to != 0x10883d1 { return; }
  if cpu.cf { pc = 0x10883d5; continue; }
  pc = 0x10883d3; continue;
 }
 0x10883d3 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10883d5 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010883d7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10883d7); } r_010883d7(cpu, 0x10883d7); }
pub fn f_01088422(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1088422); } r_010883d7(cpu, 0x1088422); }
fn r_010883d7(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10883d7 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10883df); f_010885b0(cpu); if cpu.returned_to != 0x10883df { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11eb118u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10883eb); f_01088583(cpu); if cpu.returned_to != 0x10883eb { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x10883f7; continue; }
  pc = 0x10883ee; continue;
 }
 0x10883ee => {
  cpu.push(0x10883f3); f_010884a2(cpu); if cpu.returned_to != 0x10883f3 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10883f5 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10883f7 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10883fd); f_010878db(cpu); if cpu.returned_to != 0x10883fd { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x1088420; continue; }
  pc = 0x1088400; continue;
 }
 0x1088400 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x19u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x19u32.wrapping_add(cpu.esi))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1088420; continue; }
  pc = 0x108840b; continue;
 }
 0x108840b => {
  cpu.push(0x1088410); f_01088483(cpu); if cpu.returned_to != 0x1088410 { return; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1088416); f_01088583(cpu); if cpu.returned_to != 0x1088416 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x1088420; continue; }
  pc = 0x1088419; continue;
 }
 0x1088419 => {
  cpu.push(0x108841e); f_010884a2(cpu); if cpu.returned_to != 0x108841e { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1088420 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1088422 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.zf { cpu.poll(); pc = 0x10883f5; continue; }
  pc = 0x1088428; continue;
 }
 0x1088428 => {
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1088445); f_010877ce(cpu); if cpu.returned_to != 0x1088445 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { cpu.poll(); pc = 0x10883f5; continue; }
  pc = 0x1088448; continue;
 }
 0x1088448 => {
  { let v: u8 = cpu.or8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x18u8); cpu.w8(0x10u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1088452); f_01088483(cpu); if cpu.returned_to != 0x1088452 { return; }
  cpu.push(0x1088457); f_01088468(cpu); if cpu.returned_to != 0x1088457 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x108845e); f_01088583(cpu); if cpu.returned_to != 0x108845e { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { cpu.poll(); pc = 0x1088420; continue; }
  pc = 0x1088461; continue;
 }
 0x1088461 => {
  cpu.push(0x1088466); f_010884a2(cpu); if cpu.returned_to != 0x1088466 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088468(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088468); }
 let mut pc: u32 = 0x1088468;
 loop { match pc {
 0x1088468 => {
  { cpu.pushad(); }
  cpu.push(0x108846e); f_0108a681(cpu); if cpu.returned_to != 0x108846e { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0xau16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1088481); f_01066c93(cpu); if cpu.returned_to != 0x1088481 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088483(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088483); }
 let mut pc: u32 = 0x1088483;
 loop { match pc {
 0x1088483 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v = cpu.r8(0x11eb11cu32); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x11ea974u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11eb11cu32)); cpu.w32(0x11eb11cu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010884a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10884a2); }
 let mut pc: u32 = 0x10884a2;
 loop { match pc {
 0x10884a2 => {
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { let v: u32 = 0x11ea974u32; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x10884af; continue;
 }
 0x10884af => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10884ca); f_0108a681(cpu); if cpu.returned_to != 0x10884ca { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  { cpu.pushad(); }
  cpu.push(0x10884db); f_0108b988(cpu); if cpu.returned_to != 0x10884db { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10884af; continue; }
  pc = 0x10884e5; continue;
 }
 0x10884e5 => {
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  cpu.push(0x10884f0); f_010884f1(cpu); if cpu.returned_to != 0x10884f0 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010884f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10884f1); }
 let mut pc: u32 = 0x10884f1;
 loop { match pc {
 0x10884f1 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x1088582; continue; }
  pc = 0x10884fb; continue;
 }
 0x10884fb => {
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  cpu.push(0x1088506); f_0108bb7a(cpu); if cpu.returned_to != 0x1088506 { return; }
  { let v: u32 = cpu.r32(0x1133820u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1088582; continue; }
  pc = 0x1088510; continue;
 }
 0x1088510 => {
  { let v: u32 = 0x11eb140u32; cpu.esi = v; }
  pc = 0x1088515; continue;
 }
 0x1088515 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = 0x116e734u32; cpu.ebp = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1088521; continue;
 }
 0x1088521 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x108856f; continue; }
  pc = 0x1088527; continue;
 }
 0x1088527 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x108856f; continue; }
  pc = 0x108852d; continue;
 }
 0x108852d => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x108856f; continue; }
  pc = 0x1088533; continue;
 }
 0x1088533 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x0u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x108856f; continue; }
  pc = 0x1088538; continue;
 }
 0x1088538 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x1088548; continue; }
  pc = 0x108853e; continue;
 }
 0x108853e => {
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x108856f; continue; }
  pc = 0x1088548; continue;
 }
 0x1088548 => {
  { cpu.pushad(); }
  { let v: u8 = 0x3u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x108855f); f_0108a681(cpu); if cpu.returned_to != 0x108855f { return; }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x108856e); f_0108b988(cpu); if cpu.returned_to != 0x108856e { return; }
  { cpu.popad(); }
  pc = 0x108856f; continue;
 }
 0x108856f => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x16u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x1088521; continue; }
  pc = 0x108857a; continue;
 }
 0x108857a => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1088515; continue; }
  pc = 0x1088582; continue;
 }
 0x1088582 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088583(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088583); }
 let mut pc: u32 = 0x1088583;
 loop { match pc {
 0x1088583 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x11ea974u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11eb11cu32); cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x1088593; continue;
 }
 0x1088593 => {
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.eax))); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x11eb118u32)); }
  if !cpu.cf { pc = 0x10885ab; continue; }
  pc = 0x10885a2; continue;
 }
 0x10885a2 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1088593; continue; }
  pc = 0x10885a8; continue;
 }
 0x10885a8 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10885ab => {
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010885b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10885b0); }
 let mut pc: u32 = 0x10885b0;
 loop { match pc {
 0x10885b0 => {
  cpu.push(0x10885b5); f_010885cd(cpu); if cpu.returned_to != 0x10885b5 { return; }
  if cpu.cf { pc = 0x10885c7; continue; }
  pc = 0x10885b7; continue;
 }
 0x10885b7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x19u32.wrapping_add(cpu.eax))); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10885c7 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010885cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10885cd); }
 let mut pc: u32 = 0x10885cd;
 loop { match pc {
 0x10885cd => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.eax)), 0x0u16); }
  if !cpu.zf { pc = 0x10885d9; continue; }
  pc = 0x10885d6; continue;
 }
 0x10885d6 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10885d9 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010885dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10885dc); }
 let mut pc: u32 = 0x10885dc;
 loop { match pc {
 0x10885dc => {
  { cpu.sub32(cpu.r32(0x11eb11cu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x108861d; continue; }
  pc = 0x10885e5; continue;
 }
 0x10885e5 => {
  cpu.push(0x10885ea); f_0108861e(cpu); if cpu.returned_to != 0x10885ea { return; }
  { let v: u8 = cpu.r8(0x11eb11cu32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10885f0; continue;
 }
 0x10885f0 => {
  { let v: u8 = cpu.r8(0x11eb11cu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x11ea974u32; cpu.esi = v; }
  pc = 0x10885fd; continue;
 }
 0x10885fd => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (cpu.cf || cpu.zf) { pc = 0x1088612; continue; }
  pc = 0x1088605; continue;
 }
 0x1088605 => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1088612; continue;
 }
 0x1088612 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10885fd; continue; }
  pc = 0x1088619; continue;
 }
 0x1088619 => {
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x10885f0; continue; }
  pc = 0x108861d; continue;
 }
 0x108861d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108861e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108861e); }
 let mut pc: u32 = 0x108861e;
 loop { match pc {
 0x108861e => {
  { let v: u32 = 0x11ea974u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x11eb11cu32); cpu.ecx = v; }
  pc = 0x1088629; continue;
 }
 0x1088629 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x108863a); f_0108a681(cpu); if cpu.returned_to != 0x108863a { return; }
  if cpu.cf { pc = 0x1088679; continue; }
  pc = 0x108863c; continue;
 }
 0x108863c => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x1088679; continue;
 }
 0x1088679 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1088629; continue; }
  pc = 0x1088681; continue;
 }
 0x1088681 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088682(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088682); }
 let mut pc: u32 = 0x1088682;
 loop { match pc {
 0x1088682 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x11ea974u32; cpu.edi = v; }
  { let v: u32 = 0x116e734u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11eb11cu32, v); }
  pc = 0x108869d; continue;
 }
 0x108869d => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1088722; continue; }
  pc = 0x10886a3; continue;
 }
 0x10886a3 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x1088722; continue; }
  pc = 0x10886a9; continue;
 }
 0x10886a9 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x1088722; continue; }
  pc = 0x10886af; continue;
 }
 0x10886af => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x1088722; continue; }
  pc = 0x10886b5; continue;
 }
 0x10886b5 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x1088722; continue; }
  pc = 0x10886bb; continue;
 }
 0x10886bb => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { pc = 0x1088722; continue; }
  pc = 0x10886c1; continue;
 }
 0x10886c1 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.edx)), 0x20u8); }
  if cpu.zf { pc = 0x10886d6; continue; }
  pc = 0x10886c7; continue;
 }
 0x10886c7 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10886ce); f_0108a681(cpu); if cpu.returned_to != 0x10886ce { return; }
  if cpu.cf { pc = 0x1088722; continue; }
  pc = 0x10886d0; continue;
 }
 0x10886d0 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x1088722; continue; }
  pc = 0x10886d6; continue;
 }
 0x10886d6 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.edx)), 0x40u8); }
  if cpu.zf { pc = 0x1088717; continue; }
  pc = 0x10886dc; continue;
 }
 0x10886dc => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10886e3); f_0108a681(cpu); if cpu.returned_to != 0x10886e3 { return; }
  if cpu.cf { pc = 0x1088722; continue; }
  pc = 0x10886e5; continue;
 }
 0x10886e5 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x20u8); }
  if !cpu.zf { pc = 0x10886f4; continue; }
  pc = 0x10886eb; continue;
 }
 0x10886eb => {
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.eax)), 0x3u8); }
  if !cpu.zf { pc = 0x1088722; continue; }
  pc = 0x10886f4; continue;
 }
 0x10886f4 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x22u8); }
  if cpu.zf { pc = 0x1088722; continue; }
  pc = 0x10886fa; continue;
 }
 0x10886fa => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x10u8); }
  if cpu.zf { pc = 0x108870c; continue; }
  pc = 0x1088700; continue;
 }
 0x1088700 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0xfu8); }
  if cpu.zf { pc = 0x108870c; continue; }
  pc = 0x1088706; continue;
 }
 0x1088706 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1088717; continue; }
  pc = 0x108870c; continue;
 }
 0x108870c => {
  { cpu.pushad(); }
  cpu.push(0x1088712); f_01068d2a(cpu); if cpu.returned_to != 0x1088712 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x1088722; continue; }
  pc = 0x1088717; continue;
 }
 0x1088717 => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11eb11cu32)); cpu.w32(0x11eb11cu32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1088722; continue;
 }
 0x1088722 => {
  { let v: u32 = cpu.esi.wrapping_add(0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108869d; continue; }
  pc = 0x108872c; continue;
 }
 0x108872c => {
  { cpu.sub32(cpu.r32(0x11eb11cu32), 0x0u32); }
  if cpu.zf { pc = 0x1088737; continue; }
  pc = 0x1088735; continue;
 }
 0x1088735 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1088737 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088739(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088739); }
 let mut pc: u32 = 0x1088739;
 loop { match pc {
 0x1088739 => {
  cpu.push(0x108873e); f_01088898(cpu); if cpu.returned_to != 0x108873e { return; }
  cpu.push(0x1088743); f_010887f8(cpu); if cpu.returned_to != 0x1088743 { return; }
  cpu.push(0x1088748); f_010887bf(cpu); if cpu.returned_to != 0x1088748 { return; }
  cpu.push(0x108874d); f_010887a1(cpu); if cpu.returned_to != 0x108874d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108874e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108874e); }
 let mut pc: u32 = 0x108874e;
 loop { match pc {
 0x108874e => {
  { let v: u32 = 0x11ea774u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11eb0f4u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1088777; continue; }
  pc = 0x108875d; continue;
 }
 0x108875d => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x1088777; continue; }
  pc = 0x108876b; continue;
 }
 0x108876b => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x1088777; continue; }
  pc = 0x1088770; continue;
 }
 0x1088770 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108876b; continue; }
  pc = 0x1088777; continue;
 }
 0x1088777 => {
  { cpu.sub8((cpu.ebx as u8), 0x1u8); }
  if cpu.zf { pc = 0x108878f; continue; }
  pc = 0x108877c; continue;
 }
 0x108877c => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x1088788); f_0105f2ec(cpu); if cpu.returned_to != 0x1088788 { return; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x1088791; continue;
 }
 0x108878f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1088791; continue;
 }
 0x1088791 => {
  { let v: u8 = 0x8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.r32(0x11ea774u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11eb0f8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010887a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10887a1); }
 let mut pc: u32 = 0x10887a1;
 loop { match pc {
 0x10887a1 => {
  { let v: u32 = 0x11ea774u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11eb0f4u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10887be; continue; }
  pc = 0x10887b0; continue;
 }
 0x10887b0 => {
  { let v: u32 = 0xffffffc0u32; cpu.eax = v; }
  pc = 0x10887b5; continue;
 }
 0x10887b5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)) & cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10887b5; continue; }
  pc = 0x10887be; continue;
 }
 0x10887be => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010887bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10887bf); }
 let mut pc: u32 = 0x10887bf;
 loop { match pc {
 0x10887bf => {
  { let v: u8 = cpu.r8(0x11eb0f4u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x1u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10887f7; continue; }
  pc = 0x10887ca; continue;
 }
 0x10887ca => {
  { let v: u32 = 0x11ea774u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x11eb0f4u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x10887d7; continue;
 }
 0x10887d7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if !cpu.cf { pc = 0x10887ec; continue; }
  pc = 0x10887df; continue;
 }
 0x10887df => {
  { let a = cpu.r32(0xcu32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x10887ec; continue;
 }
 0x10887ec => {
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x10887d7; continue; }
  pc = 0x10887f3; continue;
 }
 0x10887f3 => {
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x10887ca; continue; }
  pc = 0x10887f7; continue;
 }
 0x10887f7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010887f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10887f8); }
 let mut pc: u32 = 0x10887f8;
 loop { match pc {
 0x10887f8 => {
  { cpu.sub8(cpu.r8(0x11eb12eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1088897; continue; }
  pc = 0x1088805; continue;
 }
 0x1088805 => {
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x108880f; continue;
 }
 0x108880f => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1088833; continue; }
  pc = 0x1088816; continue;
 }
 0x1088816 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x1088833; continue; }
  pc = 0x108881c; continue;
 }
 0x108881c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  pc = 0x1088839; continue;
 }
 0x1088833 => {
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108880f; continue; }
  pc = 0x1088839; continue;
 }
 0x1088839 => {
  { let v: u32 = 0x11ea774u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11eb0f4u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x1088897; continue; }
  pc = 0x1088848; continue;
 }
 0x1088848 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x10e4138u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sar8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x10e413cu32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sar8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.eax as u8); cpu.imul8(v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x3ffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x108887b; continue; }
  pc = 0x1088876; continue;
 }
 0x1088876 => {
  { let v: u32 = 0x3ffu32; cpu.ebx = v; }
  pc = 0x108887b; continue;
 }
 0x108887b => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e4028u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1088848; continue; }
  pc = 0x1088897; continue;
 }
 0x1088897 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088898(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088898); }
 let mut pc: u32 = 0x1088898;
 loop { match pc {
 0x1088898 => {
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u32 = 0x11ea774u32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11eb0f4u32, v); }
  pc = 0x10888b3; continue;
 }
 0x10888b3 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x10888df; continue; }
  pc = 0x10888ba; continue;
 }
 0x10888ba => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x10888df; continue; }
  pc = 0x10888c0; continue;
 }
 0x10888c0 => {
  cpu.push(0x10888c5); f_010888e6(cpu); if cpu.returned_to != 0x10888c5 { return; }
  if cpu.cf { pc = 0x10888df; continue; }
  pc = 0x10888c7; continue;
 }
 0x10888c7 => {
  { let v: u32 = cpu.r32(0x19u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.sf { pc = 0x10888d1; continue; }
  pc = 0x10888cf; continue;
 }
 0x10888cf => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10888d1; continue;
 }
 0x10888d1 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11eb0f4u32)); cpu.w32(0x11eb0f4u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x10888df; continue;
 }
 0x10888df => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10888b3; continue; }
  pc = 0x10888e5; continue;
 }
 0x10888e5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010888e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10888e6); }
 let mut pc: u32 = 0x10888e6;
 loop { match pc {
 0x10888e6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10888f8); f_01087f84(cpu); if cpu.returned_to != 0x10888f8 { return; }
  if cpu.cf { pc = 0x108891a; continue; }
  pc = 0x10888fa; continue;
 }
 0x10888fa => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1088901); f_01087f84(cpu); if cpu.returned_to != 0x1088901 { return; }
  if cpu.cf { pc = 0x108891a; continue; }
  pc = 0x1088903; continue;
 }
 0x1088903 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x108890a); f_01087f84(cpu); if cpu.returned_to != 0x108890a { return; }
  if cpu.cf { pc = 0x108891a; continue; }
  pc = 0x108890c; continue;
 }
 0x108890c => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1088913); f_01087f84(cpu); if cpu.returned_to != 0x1088913 { return; }
  if cpu.cf { pc = 0x108891a; continue; }
  pc = 0x1088915; continue;
 }
 0x1088915 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108891a => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108891f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108891f); }
 let mut pc: u32 = 0x108891f;
 loop { match pc {
 0x108891f => {
  { let v: u32 = 0x116e734u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x11757a4u32; cpu.edx = v; }
  pc = 0x108892b; continue;
 }
 0x108892b => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1088984; continue; }
  pc = 0x1088931; continue;
 }
 0x1088931 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x1088984; continue; }
  pc = 0x1088937; continue;
 }
 0x1088937 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x1088984; continue; }
  pc = 0x108893d; continue;
 }
 0x108893d => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.edx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1088954); f_0108b344(cpu); if cpu.returned_to != 0x1088954 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x1088984; continue; }
  pc = 0x108895a; continue;
 }
 0x108895a => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x1088984; continue; }
  pc = 0x1088960; continue;
 }
 0x1088960 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x108896c); f_01088a90(cpu); if cpu.returned_to != 0x108896c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x1088981; continue; }
  pc = 0x1088972; continue;
 }
 0x1088972 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x1088981; continue; }
  pc = 0x108897c; continue;
 }
 0x108897c => {
  cpu.push(0x1088981); f_01088f55(cpu); if cpu.returned_to != 0x1088981 { return; }
  pc = 0x1088981; continue;
 }
 0x1088981 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1088984; continue;
 }
 0x1088984 => {
  { let v: u32 = cpu.esi.wrapping_add(0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x48u32); cpu.edx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108892b; continue; }
  pc = 0x108898e; continue;
 }
 0x108898e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088a90(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1088a90); } r_01088a90(cpu, 0x1088a90); }
pub fn f_01088c09(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1088c09); } r_01088a90(cpu, 0x1088c09); }
fn r_01088a90(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1088a90 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1088c1f; continue; }
  pc = 0x1088a9a; continue;
 }
 0x1088a9a => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1088c1f; continue; }
  pc = 0x1088aa4; continue;
 }
 0x1088aa4 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x1088c1f; continue; }
  pc = 0x1088aae; continue;
 }
 0x1088aae => {
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.push(0x1088ab7); f_01087fc5(cpu); if cpu.returned_to != 0x1088ab7 { return; }
  if !cpu.cf { pc = 0x1088aea; continue; }
  pc = 0x1088ab9; continue;
 }
 0x1088ab9 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x1088ac0); f_0108a681(cpu); if cpu.returned_to != 0x1088ac0 { return; }
  if cpu.cf { pc = 0x1088c1f; continue; }
  pc = 0x1088ac6; continue;
 }
 0x1088ac6 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x2u8); }
  if cpu.zf { pc = 0x1088ad6; continue; }
  pc = 0x1088acc; continue;
 }
 0x1088acc => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x1088c1f; continue; }
  pc = 0x1088ad6; continue;
 }
 0x1088ad6 => {
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x1088aae; continue;
 }
 0x1088aea => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x1088b0d; continue; }
  pc = 0x1088af0; continue;
 }
 0x1088af0 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1088b0d; continue; }
  pc = 0x1088af6; continue;
 }
 0x1088af6 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1088afd); f_0108a6b9(cpu); if cpu.returned_to != 0x1088afd { return; }
  if cpu.cf { pc = 0x1088c1f; continue; }
  pc = 0x1088b03; continue;
 }
 0x1088b03 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.eax)), 0x1u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1088c1f; continue; }
  pc = 0x1088b0d; continue;
 }
 0x1088b0d => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { pc = 0x1088c1f; continue; }
  pc = 0x1088b17; continue;
 }
 0x1088b17 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x1088c1f; continue; }
  pc = 0x1088b21; continue;
 }
 0x1088b21 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x1088c1f; continue; }
  pc = 0x1088b2f; continue;
 }
 0x1088b2f => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x1088c1f; continue; }
  pc = 0x1088b39; continue;
 }
 0x1088b39 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1088b41); f_0108bfd5(cpu); if cpu.returned_to != 0x1088b41 { return; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x1088c1f; continue; }
  pc = 0x1088b4c; continue;
 }
 0x1088b4c => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x1088bf9; continue; }
  pc = 0x1088b54; continue;
 }
 0x1088b54 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1088c1f; continue; }
  pc = 0x1088b5e; continue;
 }
 0x1088b5e => {
  { cpu.and8(cpu.r8(0x1au32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x1088b77; continue; }
  pc = 0x1088b64; continue;
 }
 0x1088b64 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x1088b77; continue; }
  pc = 0x1088b6a; continue;
 }
 0x1088b6a => {
  cpu.push(0x1088b6f); f_01068d2a(cpu); if cpu.returned_to != 0x1088b6f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1088c1f; continue; }
  pc = 0x1088b77; continue;
 }
 0x1088b77 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x11757a4u32); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1088b91); f_01065bdb(cpu); if cpu.returned_to != 0x1088b91 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if !cpu.cf { pc = 0x1088b9b; continue; }
  pc = 0x1088b99; continue;
 }
 0x1088b99 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x1088b9b; continue;
 }
 0x1088b9b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  { cpu.pushad(); }
  cpu.push(0x1088bce); f_01036a75(cpu); if cpu.returned_to != 0x1088bce { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  cpu.push(0x1088bd5); f_0108a641(cpu); if cpu.returned_to != 0x1088bd5 { return; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)) | 0x4u8; cpu.w8(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x5u8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1088bec); f_0108c1c5(cpu); if cpu.returned_to != 0x1088bec { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x18u8); cpu.w8(0x10u32.wrapping_add(cpu.eax), v); }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1088bf9 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1088c1f; continue; }
  pc = 0x1088bff; continue;
 }
 0x1088bff => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1088b54; continue; }
  pc = 0x1088c07; continue;
 }
 0x1088c07 => {
  pc = 0x1088c1f; continue;
 }
 0x1088c09 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x1133690u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.push(0x1088c16); cpu.call(t); if cpu.returned_to != 0x1088c16 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if cpu.cf { pc = 0x1088c1f; continue; }
  pc = 0x1088c19; continue;
 }
 0x1088c19 => {
  { let v: u8 = cpu.or8(cpu.r8(0x10u32.wrapping_add(cpu.edx)), 0x4u8); cpu.w8(0x10u32.wrapping_add(cpu.edx), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1088c1f => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088c21(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1088c21); } r_01088c21(cpu, 0x1088c21); }
pub fn f_01088c3a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1088c3a); } r_01088c21(cpu, 0x1088c3a); }
pub fn f_01088c53(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1088c53); } r_01088c21(cpu, 0x1088c53); }
pub fn f_01088c85(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1088c85); } r_01088c21(cpu, 0x1088c85); }
pub fn f_01088cb7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1088cb7); } r_01088c21(cpu, 0x1088cb7); }
fn r_01088c21(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1088c21 => {
  { let v: u32 = 0x1088f0au32; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = 0x1088d12u32; cpu.w32(0x10e413cu32, v); }
  pc = 0x1088ce9; continue;
 }
 0x1088c3a => {
  { let v: u32 = 0x1088d12u32; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = 0x1088f0au32; cpu.w32(0x10e413cu32, v); }
  pc = 0x1088ce9; continue;
 }
 0x1088c53 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1088c6f; continue; }
  pc = 0x1088c59; continue;
 }
 0x1088c59 => {
  { let v: u32 = 0x1088f0au32; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = 0x1088d12u32; cpu.w32(0x10e413cu32, v); }
  pc = 0x1088ce9; continue;
 }
 0x1088c6f => {
  { let v: u32 = 0x1088f0au32; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = 0x1088d10u32; cpu.w32(0x10e413cu32, v); }
  pc = 0x1088ce9; continue;
 }
 0x1088c85 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1088ca1; continue; }
  pc = 0x1088c8b; continue;
 }
 0x1088c8b => {
  { let v: u32 = 0x1088d12u32; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = 0x1088f0au32; cpu.w32(0x10e413cu32, v); }
  pc = 0x1088ce9; continue;
 }
 0x1088ca1 => {
  { let v: u32 = 0x1088f0au32; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = 0x1088d10u32; cpu.w32(0x10e413cu32, v); }
  pc = 0x1088ce9; continue;
 }
 0x1088cb7 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1088cd3; continue; }
  pc = 0x1088cbd; continue;
 }
 0x1088cbd => {
  { let v: u32 = 0x1088d12u32; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = 0x1088f0au32; cpu.w32(0x10e413cu32, v); }
  pc = 0x1088ce9; continue;
 }
 0x1088cd3 => {
  { let v: u32 = 0x1088f0au32; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = 0x1088d10u32; cpu.w32(0x10e413cu32, v); }
  pc = 0x1088ce9; continue;
 }
 0x1088ce9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1088cf4); f_0105f215(cpu); if cpu.returned_to != 0x1088cf4 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x11e94b0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1088d0a; continue; }
  pc = 0x1088d04; continue;
 }
 0x1088d04 => {
  cpu.poll(); pc = cpu.r32(0x10e4138u32); continue;
 }
 0x1088d0a => {
  cpu.poll(); pc = cpu.r32(0x10e413cu32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088d10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088d10); }
 let mut pc: u32 = 0x1088d10;
 loop { match pc {
 0x1088d10 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088d12(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088d12); }
 let mut pc: u32 = 0x1088d12;
 loop { match pc {
 0x1088d12 => {
  { let v: u32 = 0x4au32; cpu.eax = v; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = 0x10c6120u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { cpu.pushad(); }
  cpu.push(0x1088d5a); f_01036a75(cpu); if cpu.returned_to != 0x1088d5a { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088d5d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088d5d); }
 let mut pc: u32 = 0x1088d5d;
 loop { match pc {
 0x1088d5d => {
  { let v: u32 = 0x116e734u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1088d67; continue;
 }
 0x1088d67 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1088eff; continue; }
  pc = 0x1088d71; continue;
 }
 0x1088d71 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x1088eff; continue; }
  pc = 0x1088d7b; continue;
 }
 0x1088d7b => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x1088eff; continue; }
  pc = 0x1088d85; continue;
 }
 0x1088d85 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x1088d95; continue; }
  pc = 0x1088d8b; continue;
 }
 0x1088d8b => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x1088eff; continue; }
  pc = 0x1088d95; continue;
 }
 0x1088d95 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x1088eff; continue; }
  pc = 0x1088da0; continue;
 }
 0x1088da0 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.eax)), 0x3u8); }
  if !cpu.zf { pc = 0x1088eff; continue; }
  pc = 0x1088daa; continue;
 }
 0x1088daa => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1088db1); f_0108a681(cpu); if cpu.returned_to != 0x1088db1 { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1088df8; continue; }
  pc = 0x1088db7; continue;
 }
 0x1088db7 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x10u8); }
  if cpu.zf { pc = 0x1088df8; continue; }
  pc = 0x1088dbd; continue;
 }
 0x1088dbd => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0xfu8); }
  if cpu.zf { pc = 0x1088e0b; continue; }
  pc = 0x1088dc3; continue;
 }
 0x1088dc3 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x3u8); }
  if cpu.zf { pc = 0x1088e0b; continue; }
  pc = 0x1088dc9; continue;
 }
 0x1088dc9 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x1eu8); }
  if cpu.zf { pc = 0x1088e0b; continue; }
  pc = 0x1088dcf; continue;
 }
 0x1088dcf => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x6u8); }
  if cpu.zf { pc = 0x1088e0b; continue; }
  pc = 0x1088dd5; continue;
 }
 0x1088dd5 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x1au8); }
  if cpu.zf { pc = 0x1088e0b; continue; }
  pc = 0x1088ddb; continue;
 }
 0x1088ddb => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x19u8); }
  if cpu.zf { pc = 0x1088e0b; continue; }
  pc = 0x1088de1; continue;
 }
 0x1088de1 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x1eu8); }
  if cpu.zf { pc = 0x1088e0b; continue; }
  pc = 0x1088de7; continue;
 }
 0x1088de7 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x1bu8); }
  if cpu.zf { pc = 0x1088e0b; continue; }
  pc = 0x1088ded; continue;
 }
 0x1088ded => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x18u8); }
  if cpu.zf { pc = 0x1088e0b; continue; }
  pc = 0x1088df3; continue;
 }
 0x1088df3 => {
  pc = 0x1088eff; continue;
 }
 0x1088df8 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1088e01); f_01068d2a(cpu); if cpu.returned_to != 0x1088e01 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1088eff; continue; }
  pc = 0x1088e0b; continue;
 }
 0x1088e0b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1088eff; continue; }
  pc = 0x1088e17; continue;
 }
 0x1088e17 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.eax)), 0x0u16); }
  if cpu.zf { pc = 0x1088eff; continue; }
  pc = 0x1088e23; continue;
 }
 0x1088e23 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r8(0x12u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x40u32); cpu.ebx = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10e4144u32, v); }
  cpu.push(0x1088e62); f_0105381d(cpu); if cpu.returned_to != 0x1088e62 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = 0x11a0f95u32; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8) ^ (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1088e6b; continue;
 }
 0x1088e6b => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0xffu8); }
  if cpu.zf { pc = 0x1088ebe; continue; }
  pc = 0x1088e70; continue;
 }
 0x1088e70 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x1088ebb; continue; }
  pc = 0x1088e87; continue;
 }
 0x1088e87 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x10e4138u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1088eb9; continue; }
  pc = 0x1088e97; continue;
 }
 0x1088e97 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x10e4140u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1088eb9; continue; }
  pc = 0x1088e9f; continue;
 }
 0x1088e9f => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x10e413cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x1088eb9; continue; }
  pc = 0x1088eab; continue;
 }
 0x1088eab => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x10e4144u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1088eb9; continue; }
  pc = 0x1088eb3; continue;
 }
 0x1088eb3 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1088ebb; continue;
 }
 0x1088eb9 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x1088ebb; continue;
 }
 0x1088ebb => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x1088e6b; continue;
 }
 0x1088ebe => {
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x1088eff; continue; }
  pc = 0x1088ec2; continue;
 }
 0x1088ec2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1088ecb; continue;
 }
 0x1088ecb => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.sf { pc = 0x1088ef8; continue; }
  pc = 0x1088ed3; continue;
 }
 0x1088ed3 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.sf { pc = 0x1088ef8; continue; }
  pc = 0x1088ed9; continue;
 }
 0x1088ed9 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x1eu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x1088ef8; continue; }
  pc = 0x1088ede; continue;
 }
 0x1088ede => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1088ee9); f_0108a681(cpu); if cpu.returned_to != 0x1088ee9 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x1088ef0); f_01088f0a(cpu); if cpu.returned_to != 0x1088ef0 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x6u8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  pc = 0x1088eff; continue;
 }
 0x1088ef8 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1088ecb; continue; }
  pc = 0x1088eff; continue;
 }
 0x1088eff => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1088d67; continue; }
  pc = 0x1088f09; continue;
 }
 0x1088f09 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088f0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088f0a); }
 let mut pc: u32 = 0x1088f0a;
 loop { match pc {
 0x1088f0a => {
  { let v: u32 = 0x4au32; cpu.eax = v; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = 0x10c6120u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = 0xd80u16; cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e13u32, v); }
  { cpu.pushad(); }
  cpu.push(0x1088f52); f_01036a75(cpu); if cpu.returned_to != 0x1088f52 { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01088f55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1088f55); }
 let mut pc: u32 = 0x1088f55;
 loop { match pc {
 0x1088f55 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x108904b; continue; }
  pc = 0x1088f5f; continue;
 }
 0x1088f5f => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { pc = 0x108904b; continue; }
  pc = 0x1088f69; continue;
 }
 0x1088f69 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x108904b; continue; }
  pc = 0x1088f73; continue;
 }
 0x1088f73 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.push(0x1088f7a); f_0108bb7a(cpu); if cpu.returned_to != 0x1088f7a { return; }
  { cpu.sub32(cpu.r32(0x1133820u32), 0x0u32); }
  if cpu.zf { pc = 0x108904b; continue; }
  pc = 0x1088f87; continue;
 }
 0x1088f87 => {
  { let v: u8 = cpu.r8(0x1133820u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x11eb140u32; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0xfff0bdc0u32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  pc = 0x1088f9d; continue;
 }
 0x1088f9d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { pc = 0x1088fae; continue; }
  pc = 0x1088fa5; continue;
 }
 0x1088fa5 => {
  { let v: u32 = 0xffffd8f0u32; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  pc = 0x1088fc8; continue;
 }
 0x1088fae => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x1088fc1; continue; }
  pc = 0x1088fb4; continue;
 }
 0x1088fb4 => {
  cpu.push(0x1088fb9); f_0108910a(cpu); if cpu.returned_to != 0x1088fb9 { return; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x1088fc1; continue; }
  pc = 0x1088fbd; continue;
 }
 0x1088fbd => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  pc = 0x1088fc1; continue;
 }
 0x1088fc1 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1088f9d; continue; }
  pc = 0x1088fc8; continue;
 }
 0x1088fc8 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1088fd7; continue; }
  pc = 0x1088fd0; continue;
 }
 0x1088fd0 => {
  cpu.push(0x1088fd5); f_0108904d(cpu); if cpu.returned_to != 0x1088fd5 { return; }
  if cpu.cf { pc = 0x108904b; continue; }
  pc = 0x1088fd7; continue;
 }
 0x1088fd7 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1088fdd); f_0108b344(cpu); if cpu.returned_to != 0x1088fdd { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x1088fea; continue; }
  pc = 0x1088fe0; continue;
 }
 0x1088fe0 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x108904b; continue; }
  pc = 0x1088fe4; continue;
 }
 0x1088fe4 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x108904b; continue; }
  pc = 0x1088fea; continue;
 }
 0x1088fea => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x1089019; continue; }
  pc = 0x1089006; continue;
 }
 0x1089006 => {
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x50u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x48u32); cpu.ebx = v; }
  pc = 0x1089019; continue;
 }
 0x1089019 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { cpu.pushad(); }
  cpu.push(0x1089042); f_01036a75(cpu); if cpu.returned_to != 0x1089042 { return; }
  { cpu.popad(); }
  { let v: u16 = 0x3fcu16; cpu.w16(0x16u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108904b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108904d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108904d); }
 let mut pc: u32 = 0x108904d;
 loop { match pc {
 0x108904d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1089061); f_0108c1c5(cpu); if cpu.returned_to != 0x1089061 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1089108; continue; }
  pc = 0x108906a; continue;
 }
 0x108906a => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u8 = 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x10e4144u32, v); }
  pc = 0x1089096; continue;
 }
 0x1089096 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x10e4134u32)); }
  if cpu.zf { pc = 0x10890f1; continue; }
  pc = 0x108909e; continue;
 }
 0x108909e => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x10890f1; continue; }
  pc = 0x10890a5; continue;
 }
 0x10890a5 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x10890f1; continue; }
  pc = 0x10890ab; continue;
 }
 0x10890ab => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e4138u32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e413cu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x10e4140u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10890f1; continue; }
  pc = 0x10890e5; continue;
 }
 0x10890e5 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4144u32, v); }
  pc = 0x10890f1; continue;
 }
 0x10890f1 => {
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1089096; continue; }
  pc = 0x10890f8; continue;
 }
 0x10890f8 => {
  { cpu.sub32(cpu.r32(0x10e4140u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x1089108; continue; }
  pc = 0x1089101; continue;
 }
 0x1089101 => {
  { let v: u32 = cpu.r32(0x10e4144u32); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1089108 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108910a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108910a); }
 let mut pc: u32 = 0x108910a;
 loop { match pc {
 0x108910a => {
  { let v: u32 = cpu.r32(0x19u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01089740(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089740); } r_01089740(cpu, 0x1089740); }
pub fn f_01089749(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089749); } r_01089740(cpu, 0x1089749); }
pub fn f_010897a3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10897a3); } r_01089740(cpu, 0x10897a3); }
pub fn f_010897cd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10897cd); } r_01089740(cpu, 0x10897cd); }
pub fn f_010897f3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10897f3); } r_01089740(cpu, 0x10897f3); }
pub fn f_01089826(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089826); } r_01089740(cpu, 0x1089826); }
pub fn f_01089867(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089867); } r_01089740(cpu, 0x1089867); }
pub fn f_010898a8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10898a8); } r_01089740(cpu, 0x10898a8); }
pub fn f_010898dd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10898dd); } r_01089740(cpu, 0x10898dd); }
pub fn f_0108990c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108990c); } r_01089740(cpu, 0x108990c); }
pub fn f_01089939(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089939); } r_01089740(cpu, 0x1089939); }
pub fn f_0108995a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108995a); } r_01089740(cpu, 0x108995a); }
pub fn f_01089983(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089983); } r_01089740(cpu, 0x1089983); }
pub fn f_010899b1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10899b1); } r_01089740(cpu, 0x10899b1); }
pub fn f_010899dd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10899dd); } r_01089740(cpu, 0x10899dd); }
pub fn f_01089a09(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089a09); } r_01089740(cpu, 0x1089a09); }
pub fn f_01089a3f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089a3f); } r_01089740(cpu, 0x1089a3f); }
pub fn f_01089a71(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089a71); } r_01089740(cpu, 0x1089a71); }
pub fn f_01089aa1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089aa1); } r_01089740(cpu, 0x1089aa1); }
pub fn f_01089acc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089acc); } r_01089740(cpu, 0x1089acc); }
pub fn f_01089af8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089af8); } r_01089740(cpu, 0x1089af8); }
pub fn f_01089b1f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089b1f); } r_01089740(cpu, 0x1089b1f); }
pub fn f_01089b52(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089b52); } r_01089740(cpu, 0x1089b52); }
pub fn f_01089b7b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089b7b); } r_01089740(cpu, 0x1089b7b); }
pub fn f_01089baa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089baa); } r_01089740(cpu, 0x1089baa); }
pub fn f_01089bd5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1089bd5); } r_01089740(cpu, 0x1089bd5); }
fn r_01089740(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1089740 => {
  cpu.flat_ds(0x1089740); cpu.push(0x1089745); f_01086824(cpu); if cpu.returned_to != 0x1089745 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089766; continue;
 }
 0x1089749 => {
  if cpu.of { pc = 0x10897bd; continue; }
  pc = 0x108974b; continue;
 }
 0x108974b => {
  cpu.unimplemented(0x108974b, "Outsd");
  cpu.unimplemented(0x108974c, "Arpl");
  if !cpu.cf { pc = 0x10897b0; continue; }
  pc = 0x1089751; continue;
 }
 0x1089751 => {
  cpu.unimplemented(0x1089751, "Arpl");
  if cpu.zf { pc = 0x10897cb; continue; }
  pc = 0x1089756; continue;
 }
 0x1089756 => {
  if cpu.cf { pc = 0x10897bd; continue; }
  pc = 0x1089758; continue;
 }
 0x1089758 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x10897c1; continue; }
  pc = 0x108975c; continue;
 }
 0x108975c => {
  if cpu.cf { pc = 0x10897d0; continue; }
  pc = 0x108975e; continue;
 }
 0x108975e => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2)).wrapping_add(cpu.bases[3])), 0x736569u32); cpu.esi = v; }
  pc = 0x1089766; continue;
 }
 0x1089766 => {
  { let v: u32 = 0x1089749u32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089772); cpu.push(0x1089777); f_0108b09c(cpu); if cpu.returned_to != 0x1089777 { return; }
  { cpu.sub8(cpu.r8(0x10ee91eu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x1089bd4; continue; }
  pc = 0x1089784; continue;
 }
 0x1089784 => {
  { cpu.sub16(cpu.r16(0x10e3f39u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { pc = 0x1089bd4; continue; }
  pc = 0x1089792; continue;
 }
 0x1089792 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x1089bd4; continue; }
  pc = 0x108979f; continue;
 }
 0x108979f => {
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x10897b8; continue;
 }
 0x10897a3 => {
  if !cpu.cf { pc = 0x108980a; continue; }
  pc = 0x10897a5; continue;
 }
 0x10897a5 => {
  if cpu.zf { pc = 0x1089806; continue; }
  pc = 0x10897a7; continue;
 }
 0x10897a7 => {
  if !cpu.zf { pc = 0x1089817; continue; }
  pc = 0x10897a9; continue;
 }
 0x10897a9 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x74u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(2)).wrapping_add(cpu.bases[3])), 0x69727265u32); cpu.esi = v; }
  if cpu.zf { pc = 0x1089822; continue; }
  pc = 0x10897b3; continue;
 }
 0x10897b0 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2)).wrapping_add(cpu.bases[3])), 0x736569u32); cpu.esi = v; }
  pc = 0x10897b8; continue;
 }
 0x10897b3 => {
  if cpu.cf { pc = 0x108981e; continue; }
  pc = 0x10897b5; continue;
 }
 0x10897b5 => {
  if !cpu.cf { pc = 0x10897b8; continue; }
  pc = 0x10897b8; continue;
 }
 0x10897b8 => {
  { let v: u32 = 0x10897a3u32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x10897c4); cpu.push(0x10897c9); f_0108bb0f(cpu); if cpu.returned_to != 0x10897c9 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x10897cb; continue;
 }
 0x10897bd => {
  { let v: u32 = cpu.r32(0x61010897u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.esp); cpu.w32(0x61010897u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x10897c4); cpu.push(0x10897c9); f_0108bb0f(cpu); if cpu.returned_to != 0x10897c9 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x10897cb; continue;
 }
 0x10897c1 => {
  { let v: u32 = cpu.add32(cpu.r32(0xffffff9du32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), cpu.esp); cpu.w32(0xffffff9du32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x10897c4); cpu.push(0x10897c9); f_0108bb0f(cpu); if cpu.returned_to != 0x10897c9 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x10897cb; continue;
 }
 0x10897cb => {
  pc = 0x10897de; continue;
 }
 0x10897cd => {
  { cpu.popad(); }
  if cpu.zf { pc = 0x1089844; continue; }
  pc = 0x10897d0; continue;
 }
 0x10897d0 => {
  { cpu.popad(); }
  cpu.unimplemented(0x10897d1, "Arpl");
  if cpu.zf { pc = 0x108983e; continue; }
  pc = 0x10897d6; continue;
 }
 0x10897d6 => {
  { cpu.popad(); }
  if cpu.zf { pc = 0x1089838; continue; }
  pc = 0x10897d9; continue;
 }
 0x10897d9 => {
  cpu.unimplemented(0x10897d9, "Outsw");
  if cpu.cf { cpu.call(0x1089851); return; }
  pc = 0x10897dd; continue;
 }
 0x10897dd => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x1057193u32); cpu.eax = v; }
  { cpu.interrupt(0x97); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x10897ea); cpu.push(0x10897ef); f_010875e2(cpu); if cpu.returned_to != 0x10897ef { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089808; continue;
 }
 0x10897de => {
  { let v: u32 = 0x10897cdu32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x10897ea); cpu.push(0x10897ef); f_010875e2(cpu); if cpu.returned_to != 0x10897ef { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089808; continue;
 }
 0x10897f3 => {
  if !cpu.cf { pc = 0x108985a; continue; }
  pc = 0x10897f5; continue;
 }
 0x10897f5 => {
  cpu.unimplemented(0x10897f5, "Outsb");
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.cf { pc = 0x1089869; continue; }
  pc = 0x10897fa; continue;
 }
 0x10897fa => {
  cpu.unimplemented(0x10897fa, "Bound");
  if !cpu.cf { pc = 0x108985e; continue; }
  pc = 0x10897ff; continue;
 }
 0x10897ff => {
  if cpu.zf { pc = 0x1089870; continue; }
  pc = 0x1089801; continue;
 }
 0x1089801 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x1089802, "Insb");
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1089808; continue; }
  pc = 0x1089808; continue;
 }
 0x1089806 => {
  if !cpu.cf { pc = 0x1089808; continue; }
  pc = 0x1089808; continue;
 }
 0x1089808 => {
  { let v: u32 = 0x10897f3u32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1089812; continue;
 }
 0x108980a => {
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  if !cpu.of { pc = 0x1089812; continue; }
  pc = 0x108980d; continue;
 }
 0x108980d => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let a = cpu.edi; let b = cpu.eax; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1089812; continue;
 }
 0x1089812 => {
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089814); cpu.push(0x1089819); f_01089d72(cpu); if cpu.returned_to != 0x1089819 { return; }
  { cpu.sub8(cpu.r8(0x11337ceu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x1089857; continue; }
  pc = 0x1089822; continue;
 }
 0x1089817 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x11337ceu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x1089857; continue; }
  pc = 0x1089822; continue;
 }
 0x108981e => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  if cpu.zf { pc = 0x1089857; continue; }
  pc = 0x1089822; continue;
 }
 0x1089822 => {
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x108983c; continue;
 }
 0x1089826 => {
  if cpu.zf { pc = 0x108988d; continue; }
  pc = 0x1089828; continue;
 }
 0x1089828 => {
  if cpu.cf { pc = 0x108989c; continue; }
  pc = 0x108982a; continue;
 }
 0x108982a => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2)).wrapping_add(cpu.bases[3])), 0x61725f79u32); cpu.esi = v; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x1089895; continue; }
  pc = 0x1089836; continue;
 }
 0x1089836 => {
  cpu.unimplemented(0x1089836, "Arpl");
  cpu.unimplemented(0x1089839, "Arpl");
  pc = 0x108983c; continue;
 }
 0x1089838 => {
  cpu.unimplemented(0x1089838, "Arpl");
  pc = 0x108983c; continue;
 }
 0x108983c => {
  { let v: u32 = 0x1089826u32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1089846; continue;
 }
 0x108983e => {
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  if !cpu.of { pc = 0x1089846; continue; }
  pc = 0x1089841; continue;
 }
 0x1089841 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])).wrapping_add(cpu.esp); cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  pc = 0x1089844; continue;
 }
 0x1089844 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1089846; continue;
 }
 0x1089846 => {
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089848); cpu.push(0x108984d); f_01089f59(cpu); if cpu.returned_to != 0x108984d { return; }
  cpu.flat_ds(0x108984d); cpu.push(0x1089852); f_01086c38(cpu); if cpu.returned_to != 0x1089852 { return; }
  cpu.flat_ds(0x1089852); cpu.push(0x1089857); f_01086d00(cpu); if cpu.returned_to != 0x1089857 { return; }
  pc = 0x1089857; continue;
 }
 0x1089857 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x113382cu32.wrapping_add(cpu.bases[3]))); cpu.w32(0x113382cu32.wrapping_add(cpu.bases[3]), v); }
  if !cpu.sf { pc = 0x1089bd4; continue; }
  pc = 0x1089863; continue;
 }
 0x108985a => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), (cpu.edx as u8)); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x108985e; continue;
 }
 0x108985e => {
  { let v: u32 = cpu.esi; cpu.w32(0x3u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1089863; continue;
 }
 0x1089863 => {
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089879; continue;
 }
 0x1089867 => {
  if cpu.cf { pc = 0x10898ce; continue; }
  pc = 0x1089869; continue;
 }
 0x1089869 => {
  if !cpu.cf { pc = 0x10898d0; continue; }
  pc = 0x108986b; continue;
 }
 0x108986b => {
  if cpu.zf { pc = 0x10898cc; continue; }
  pc = 0x108986d; continue;
 }
 0x108986d => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x74u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x645f6c65u32); cpu.ebp = v; }
  cpu.unimplemented(0x1089874, "Insb");
  { cpu.popad(); }
  if !cpu.sf { pc = 0x1089879; continue; }
  pc = 0x1089879; continue;
 }
 0x1089870 => {
  cpu.unimplemented(0x1089870, "Insb");
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x1089873, "Insb");
  { cpu.popad(); }
  if !cpu.sf { pc = 0x1089879; continue; }
  pc = 0x1089879; continue;
 }
 0x1089879 => {
  { let v: u32 = 0x1089867u32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089885); cpu.push(0x108988a); f_0108bc5c(cpu); if cpu.returned_to != 0x108988a { return; }
  { let v: u8 = cpu.r8(0x1133b86u32.wrapping_add(cpu.bases[3])) ^ 0x1u8; cpu.w8(0x1133b86u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11e94acu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x11337ceu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x10898d9; continue; }
  pc = 0x10898a4; continue;
 }
 0x108988d => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11e94acu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x11337ceu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x10898d9; continue; }
  pc = 0x10898a4; continue;
 }
 0x1089895 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x1337ce3du32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x1337ce3du32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  if !cpu.zf { pc = 0x10898d9; continue; }
  pc = 0x10898a4; continue;
 }
 0x108989c => {
  { cpu.sub32(cpu.eax, 0x11337ceu32); }
  { let v: u8 = cpu.add8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), ((cpu.edx >> 8) as u8)); cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10898a4; continue;
 }
 0x10898a4 => {
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x10898be; continue;
 }
 0x10898a8 => {
  if cpu.zf { pc = 0x108990f; continue; }
  pc = 0x10898aa; continue;
 }
 0x10898aa => {
  if cpu.cf { pc = 0x108991e; continue; }
  pc = 0x10898ac; continue;
 }
 0x10898ac => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2)).wrapping_add(cpu.bases[3])), 0x61725f79u32); cpu.esi = v; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x1089917; continue; }
  pc = 0x10898b8; continue;
 }
 0x10898b8 => {
  cpu.unimplemented(0x10898b8, "Arpl");
  cpu.unimplemented(0x10898bb, "Arpl");
  pc = 0x10898be; continue;
 }
 0x10898be => {
  { let v: u32 = 0x10898a8u32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x10898ca); cpu.push(0x10898cf); f_01089f59(cpu); if cpu.returned_to != 0x10898cf { return; }
  cpu.flat_ds(0x10898cf); cpu.push(0x10898d4); f_01086c38(cpu); if cpu.returned_to != 0x10898d4 { return; }
  cpu.flat_ds(0x10898d4); cpu.push(0x10898d9); f_01086d00(cpu); if cpu.returned_to != 0x10898d9 { return; }
  pc = 0x10898d9; continue;
 }
 0x10898cc => {
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x10898cf); cpu.push(0x10898d4); f_01086c38(cpu); if cpu.returned_to != 0x10898d4 { return; }
  cpu.flat_ds(0x10898d4); cpu.push(0x10898d9); f_01086d00(cpu); if cpu.returned_to != 0x10898d9 { return; }
  pc = 0x10898d9; continue;
 }
 0x10898ce => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10898d0; continue;
 }
 0x10898d0 => {
  { let v: u32 = cpu.sar32(cpu.edi, (cpu.ecx & 0xff)); cpu.edi = v; }
  cpu.call(0x10898d3); return;
 }
 0x10898d9 => {
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x10898f7; continue;
 }
 0x10898dd => {
  cpu.unimplemented(0x10898dd, "Insd");
  cpu.unimplemented(0x10898de, "Outsd");
  cpu.unimplemented(0x10898df, "Outsb");
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2)).wrapping_add(cpu.bases[3])), 0x6d6f685fu32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x1089951; continue; }
  pc = 0x10898ec; continue;
 }
 0x10898ec => {
  if cpu.cf { pc = 0x1089960; continue; }
  pc = 0x10898ee; continue;
 }
 0x10898ee => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2)).wrapping_add(cpu.bases[3])), 0x20736569u32); cpu.esi = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.eax, 0x1057193u32); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf64(0x9d610108u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); cpu.fpop(); }
  cpu.flat_ds(0x1089903); cpu.push(0x1089908); f_01086509(cpu); if cpu.returned_to != 0x1089908 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089924; continue;
 }
 0x10898f7 => {
  { let v: u32 = 0x10898ddu32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089903); cpu.push(0x1089908); f_01086509(cpu); if cpu.returned_to != 0x1089908 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089924; continue;
 }
 0x108990c => {
  { cpu.popad(); }
  { let v = 0x75u32; cpu.push(v); }
  if !cpu.cf { pc = 0x1089986; continue; }
  pc = 0x1089912; continue;
 }
 0x108990f => {
  if !cpu.zf { pc = 0x1089984; continue; }
  pc = 0x1089911; continue;
 }
 0x1089911 => {
  if cpu.zf { pc = 0x1089972; continue; }
  pc = 0x1089913; continue;
 }
 0x1089912 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x1089913; continue;
 }
 0x1089913 => {
  cpu.unimplemented(0x1089913, "Outsw");
  if cpu.cf { pc = 0x1089976; continue; }
  pc = 0x1089917; continue;
 }
 0x1089917 => {
  if !cpu.zf { pc = 0x1089987; continue; }
  pc = 0x1089919; continue;
 }
 0x1089919 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x62u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(2)).wrapping_add(cpu.bases[3])), 0x6e616c61u32); cpu.esi = v; }
  cpu.unimplemented(0x1089921, "Arpl");
  pc = 0x1089924; continue;
 }
 0x108991e => {
  cpu.unimplemented(0x108991e, "Insb");
  { cpu.popad(); }
  cpu.unimplemented(0x1089920, "Outsb");
  cpu.unimplemented(0x1089921, "Arpl");
  pc = 0x1089924; continue;
 }
 0x1089924 => {
  { let v: u32 = 0x108990cu32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089930); cpu.push(0x1089935); f_010864ac(cpu); if cpu.returned_to != 0x1089935 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089945; continue;
 }
 0x1089939 => {
  if !cpu.cf { pc = 0x10899ab; continue; }
  pc = 0x108993b; continue;
 }
 0x108993b => {
  cpu.unimplemented(0x108993b, "Insb");
  { let v: u32 = cpu.imul2_32(cpu.r32(0x75u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(2)).wrapping_add(cpu.bases[3])), 0x7374696eu32); cpu.esi = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x1057193u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x9d610108u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), cpu.ebx); }
  pc = 0x1089951; continue;
 }
 0x1089945 => {
  { let v: u32 = 0x1089939u32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  pc = 0x1089951; continue;
 }
 0x1089951 => {
  cpu.flat_ds(0x1089951); cpu.push(0x1089956); f_01089bd5(cpu); if cpu.returned_to != 0x1089956 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x108996e; continue;
 }
 0x108995a => {
  { cpu.popad(); }
  if cpu.cf { pc = 0x10899c4; continue; }
  pc = 0x108995f; continue;
 }
 0x108995f => {
  if !cpu.cf { pc = 0x10899d4; continue; }
  pc = 0x1089961; continue;
 }
 0x1089960 => {
  if !cpu.cf { pc = 0x10899cb; continue; }
  pc = 0x1089962; continue;
 }
 0x1089961 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x65u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x70706f5fu32); cpu.esi = v; }
  pc = 0x1089968; continue;
 }
 0x1089962 => {
  if (cpu.cf || cpu.zf) { pc = 0x10899c9; continue; }
  pc = 0x1089964; continue;
 }
 0x1089964 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x1089965, "Outsd");
  if cpu.of { pc = 0x10899d8; continue; }
  pc = 0x1089968; continue;
 }
 0x1089968 => {
  cpu.unimplemented(0x1089968, "Outsd");
  cpu.unimplemented(0x1089969, "Outsb");
  cpu.unimplemented(0x108996a, "Outsb");
  if cpu.zf { pc = 0x108996e; continue; }
  pc = 0x108996e; continue;
 }
 0x108996e => {
  { let v: u32 = 0x108995au32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x108997a); cpu.push(0x108997f); f_010872b4(cpu); if cpu.returned_to != 0x108997f { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x108999c; continue;
 }
 0x1089972 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8995a01u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0xffffff9du32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), cpu.esp); cpu.w32(0xffffff9du32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x108997a); cpu.push(0x108997f); f_010872b4(cpu); if cpu.returned_to != 0x108997f { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x108999c; continue;
 }
 0x1089976 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x108997a); cpu.push(0x108997f); f_010872b4(cpu); if cpu.returned_to != 0x108997f { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x108999c; continue;
 }
 0x1089983 => {
  cpu.unimplemented(0x1089983, "Arpl");
  pc = 0x1089986; continue;
 }
 0x1089984 => {
  { let v = 0x5f6b6365u32; cpu.push(v); }
  cpu.unimplemented(0x1089989, "Outsw");
  if cpu.cf { pc = 0x10899ec; continue; }
  pc = 0x108998d; continue;
 }
 0x1089986 => {
  cpu.unimplemented(0x1089986, "Arpl");
  cpu.unimplemented(0x1089989, "Outsw");
  if cpu.cf { pc = 0x10899ec; continue; }
  pc = 0x108998d; continue;
 }
 0x1089987 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x66u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x6fu32); cpu.ebx = v; }
  if cpu.cf { pc = 0x10899ec; continue; }
  pc = 0x108998d; continue;
 }
 0x108998d => {
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x1089992, "Bound");
  cpu.unimplemented(0x1089995, "Insb");
  { let v: u32 = cpu.imul2_32(cpu.r32(0x67u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[4])), 0x5c70073u32); cpu.ebp = v; }
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  if !cpu.of { pc = 0x10899a6; continue; }
  pc = 0x10899a1; continue;
 }
 0x108999c => {
  { let v: u32 = 0x1089983u32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x10899a6; continue;
 }
 0x10899a1 => {
  { let v: u32 = cpu.r32(0x61010899u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x61010899u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x10899a8); cpu.push(0x10899ad); f_0108aae9(cpu); if cpu.returned_to != 0x10899ad { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x10899c8; continue;
 }
 0x10899a6 => {
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x10899a8); cpu.push(0x10899ad); f_0108aae9(cpu); if cpu.returned_to != 0x10899ad { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x10899c8; continue;
 }
 0x10899ab => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x10899c8; continue;
 }
 0x10899b1 => {
  cpu.unimplemented(0x10899b1, "Arpl");
  cpu.unimplemented(0x10899b4, "Arpl");
  cpu.unimplemented(0x10899b7, "Outsw");
  if cpu.cf { pc = 0x1089a1a; continue; }
  pc = 0x10899bb; continue;
 }
 0x10899bb => {
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x10899c0, "Bound");
  if !cpu.cf { pc = 0x10899c8; continue; }
  pc = 0x10899c8; continue;
 }
 0x10899c4 => {
  if !cpu.cf { pc = 0x10899c8; continue; }
  pc = 0x10899c8; continue;
 }
 0x10899c8 => {
  { let v: u32 = 0x10899b1u32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x10899d2; continue;
 }
 0x10899c9 => {
  { let v: u32 = cpu.eax.wrapping_add(0x1057193u32); cpu.eax = v; }
  { let v: u8 = 0x99u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10899d2; continue;
 }
 0x10899cb => {
  if !cpu.of { pc = 0x10899d2; continue; }
  pc = 0x10899cd; continue;
 }
 0x10899cd => {
  { let v: u32 = cpu.r32(0x61010899u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.esi); cpu.w32(0x61010899u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  pc = 0x10899d4; continue;
 }
 0x10899d2 => {
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  pc = 0x10899d4; continue;
 }
 0x10899d4 => {
  cpu.flat_ds(0x10899d4); cpu.push(0x10899d9); f_0108abe7(cpu); if cpu.returned_to != 0x10899d9 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x10899f4; continue;
 }
 0x10899d8 => {
  { let v: u8 = cpu.add8(cpu.r8(0x686317ebu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.ebx as u8)); cpu.w8(0x686317ebu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.unimplemented(0x10899df, "Arpl");
  cpu.unimplemented(0x10899e3, "Outsw");
  if cpu.cf { pc = 0x1089a46; continue; }
  pc = 0x10899e7; continue;
 }
 0x10899dd => {
  cpu.unimplemented(0x10899dd, "Arpl");
  cpu.unimplemented(0x10899e0, "Arpl");
  cpu.unimplemented(0x10899e3, "Outsw");
  if cpu.cf { pc = 0x1089a46; continue; }
  pc = 0x10899e7; continue;
 }
 0x10899e7 => {
  cpu.unimplemented(0x10899e7, "Bound");
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x10899ed, "Arpl");
  cpu.unimplemented(0x10899f0, "Outsb");
  if !cpu.cf { pc = 0x10899f4; continue; }
  pc = 0x10899f4; continue;
 }
 0x10899ec => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x10899ed, "Arpl");
  cpu.unimplemented(0x10899f0, "Outsb");
  if !cpu.cf { pc = 0x10899f4; continue; }
  pc = 0x10899f4; continue;
 }
 0x10899f4 => {
  { let v: u32 = 0x10899ddu32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089a00); cpu.push(0x1089a05); f_0108aa4e(cpu); if cpu.returned_to != 0x1089a05 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089a24; continue;
 }
 0x1089a09 => {
  cpu.unimplemented(0x1089a09, "Arpl");
  cpu.unimplemented(0x1089a0c, "Arpl");
  if cpu.of { pc = 0x1089a83; continue; }
  pc = 0x1089a11; continue;
 }
 0x1089a11 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x5f797469u32); cpu.ebp = v; }
  if cpu.zf { pc = 0x1089a7f; continue; }
  pc = 0x1089a1a; continue;
 }
 0x1089a1a => {
  if cpu.cf { pc = 0x1089a8e; continue; }
  pc = 0x1089a1c; continue;
 }
 0x1089a1c => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2)).wrapping_add(cpu.bases[3])), 0x736569u32); cpu.esi = v; }
  pc = 0x1089a24; continue;
 }
 0x1089a24 => {
  { let v: u32 = 0x1089a09u32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089a30); cpu.push(0x1089a35); f_0108a372(cpu); if cpu.returned_to != 0x1089a35 { return; }
  if cpu.cf { pc = 0x1089bd4; continue; }
  pc = 0x1089a3b; continue;
 }
 0x1089a3b => {
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089a5c; continue;
 }
 0x1089a3f => {
  cpu.unimplemented(0x1089a3f, "Arpl");
  cpu.unimplemented(0x1089a42, "Arpl");
  cpu.unimplemented(0x1089a45, "Bound");
  pc = 0x1089a48; continue;
 }
 0x1089a46 => {
  if !cpu.zf { pc = 0x1089ab1; continue; }
  pc = 0x1089a48; continue;
 }
 0x1089a48 => {
  cpu.unimplemented(0x1089a48, "Insb");
  { let v: u32 = cpu.imul2_32(cpu.r32(0x67u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[4])), 0x6e755f73u32); cpu.ebp = v; }
  if cpu.cf { pc = 0x1089ab4; continue; }
  pc = 0x1089a55; continue;
 }
 0x1089a55 => {
  { cpu.popad(); }
  if cpu.zf { pc = 0x1089acc; continue; }
  pc = 0x1089a58; continue;
 }
 0x1089a58 => {
  { cpu.popad(); }
  cpu.unimplemented(0x1089a59, "Arpl");
  pc = 0x1089a5c; continue;
 }
 0x1089a5c => {
  { let v: u32 = 0x1089a3fu32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089a68); cpu.push(0x1089a6d); f_0108a4ec(cpu); if cpu.returned_to != 0x1089a6d { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089a8c; continue;
 }
 0x1089a71 => {
  if cpu.cf { pc = 0x1089ad8; continue; }
  pc = 0x1089a73; continue;
 }
 0x1089a73 => {
  cpu.unimplemented(0x1089a73, "Arpl");
  if cpu.zf { pc = 0x1089aed; continue; }
  pc = 0x1089a78; continue;
 }
 0x1089a78 => {
  if cpu.cf { pc = 0x1089adf; continue; }
  pc = 0x1089a7a; continue;
 }
 0x1089a7a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x1089a7b, "Insb");
  cpu.unimplemented(0x1089a7c, "Outsd");
  if !cpu.cf { pc = 0x1089af3; continue; }
  pc = 0x1089a7f; continue;
 }
 0x1089a7f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x1089ae7; continue; }
  pc = 0x1089a82; continue;
 }
 0x1089a82 => {
  if cpu.cf { pc = 0x1089af6; continue; }
  pc = 0x1089a84; continue;
 }
 0x1089a83 => {
  if cpu.cf { pc = 0x1089aee; continue; }
  pc = 0x1089a85; continue;
 }
 0x1089a84 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2)).wrapping_add(cpu.bases[3])), 0x736569u32); cpu.esi = v; }
  pc = 0x1089a8c; continue;
 }
 0x1089a85 => {
  if cpu.zf { pc = 0x1089af6; continue; }
  pc = 0x1089a87; continue;
 }
 0x1089a87 => {
  if cpu.cf { pc = 0x1089af2; continue; }
  pc = 0x1089a89; continue;
 }
 0x1089a89 => {
  if !cpu.cf { pc = 0x1089a8c; continue; }
  pc = 0x1089a8c; continue;
 }
 0x1089a8c => {
  { let v: u32 = 0x1089a71u32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1089a96; continue;
 }
 0x1089a8e => {
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  if !cpu.of { pc = 0x1089a96; continue; }
  pc = 0x1089a91; continue;
 }
 0x1089a91 => {
  { let v: u32 = cpu.r32(0xffffff9au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.esi); cpu.w32(0xffffff9au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1089a96; continue;
 }
 0x1089a96 => {
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089a98); cpu.push(0x1089a9d); f_0108a6d0(cpu); if cpu.returned_to != 0x1089a9d { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089ab7; continue;
 }
 0x1089aa1 => {
  cpu.unimplemented(0x1089aa1, "Arpl");
  if cpu.cf { pc = 0x1089b06; continue; }
  pc = 0x1089aa7; continue;
 }
 0x1089aa7 => {
  if cpu.cf { pc = 0x1089b0e; continue; }
  pc = 0x1089aa9; continue;
 }
 0x1089aa9 => {
  cpu.unimplemented(0x1089aa9, "Arpl");
  if cpu.zf { pc = 0x1089b23; continue; }
  pc = 0x1089aae; continue;
 }
 0x1089aae => {
  if cpu.cf { pc = 0x1089b15; continue; }
  pc = 0x1089ab0; continue;
 }
 0x1089ab0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x1089ab1; continue;
 }
 0x1089ab1 => {
  cpu.unimplemented(0x1089ab1, "Insb");
  { cpu.popad(); }
  pc = 0x1089ab4; continue;
 }
 0x1089ab4 => {
  if !cpu.cf { pc = 0x1089ab7; continue; }
  pc = 0x1089ab7; continue;
 }
 0x1089ab7 => {
  { let v: u32 = 0x1089aa1u32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089ac3); cpu.push(0x1089ac8); f_0108b010(cpu); if cpu.returned_to != 0x1089ac8 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089ae3; continue;
 }
 0x1089acc => {
  cpu.unimplemented(0x1089acc, "Arpl");
  cpu.unimplemented(0x1089acf, "Arpl");
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.of { pc = 0x1089b44; continue; }
  pc = 0x1089ad8; continue;
 }
 0x1089ad8 => {
  { cpu.popad(); }
  if !cpu.sf { pc = 0x1089b40; continue; }
  pc = 0x1089adb; continue;
 }
 0x1089adb => {
  if cpu.cf { pc = 0x1089b3c; continue; }
  pc = 0x1089add; continue;
 }
 0x1089add => {
  if !cpu.zf { pc = 0x1089b4d; continue; }
  pc = 0x1089adf; continue;
 }
 0x1089adf => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(2)).wrapping_add(cpu.bases[3])), 0x719305c7u32); cpu.esi = v; }
  pc = 0x1089ae7; continue;
 }
 0x1089ae3 => {
  { let v: u32 = 0x1089accu32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1089aed; continue;
 }
 0x1089ae7 => {
  { let v: u32 = cpu.eax.wrapping_add(0x89acc01u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0xffffff9du32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), cpu.esp); cpu.w32(0xffffff9du32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x1089aef); cpu.push(0x1089af4); f_0108b1df(cpu); if cpu.returned_to != 0x1089af4 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089af6; continue;
 }
 0x1089aed => {
  { cpu.popad(); }
  pc = 0x1089aee; continue;
 }
 0x1089aee => {
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089aef); cpu.push(0x1089af4); f_0108b1df(cpu); if cpu.returned_to != 0x1089af4 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089af6; continue;
 }
 0x1089af2 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089af6; continue;
 }
 0x1089af3 => {
  { let v: u8 = cpu.r8(0x686312ebu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.ebx as u8)); cpu.w8(0x686312ebu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.unimplemented(0x1089afa, "Arpl");
  if !cpu.zf { pc = 0x1089b6e; continue; }
  pc = 0x1089b00; continue;
 }
 0x1089af6 => {
  pc = 0x1089b0a; continue;
 }
 0x1089af8 => {
  cpu.unimplemented(0x1089af8, "Arpl");
  cpu.unimplemented(0x1089afb, "Arpl");
  if !cpu.zf { pc = 0x1089b6e; continue; }
  pc = 0x1089b00; continue;
 }
 0x1089b00 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x5fu32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(2)).wrapping_add(cpu.bases[3])), 0x656d6f68u32); cpu.esi = v; }
  if !cpu.cf { pc = 0x1089b0a; continue; }
  pc = 0x1089b0a; continue;
 }
 0x1089b06 => {
  cpu.unimplemented(0x1089b06, "Insd");
  if !cpu.cf { pc = 0x1089b0a; continue; }
  pc = 0x1089b0a; continue;
 }
 0x1089b0a => {
  { let v: u32 = 0x1089af8u32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  pc = 0x1089b15; continue;
 }
 0x1089b0e => {
  { let v: u32 = cpu.eax.wrapping_add(0x89af801u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0xffffff9du32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), cpu.esp); cpu.w32(0xffffff9du32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x1089b16); cpu.push(0x1089b1b); f_0108b0ba(cpu); if cpu.returned_to != 0x1089b1b { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089b3d; continue;
 }
 0x1089b15 => {
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089b16); cpu.push(0x1089b1b); f_0108b0ba(cpu); if cpu.returned_to != 0x1089b1b { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089b3d; continue;
 }
 0x1089b1f => {
  cpu.unimplemented(0x1089b1f, "Arpl");
  cpu.unimplemented(0x1089b22, "Arpl");
  cpu.unimplemented(0x1089b25, "Outsw");
  if cpu.cf { pc = 0x1089b88; continue; }
  pc = 0x1089b29; continue;
 }
 0x1089b23 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x66u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x6fu32); cpu.ebx = v; }
  if cpu.cf { pc = 0x1089b88; continue; }
  pc = 0x1089b29; continue;
 }
 0x1089b29 => {
  if cpu.cf { pc = 0x1089b90; continue; }
  pc = 0x1089b2b; continue;
 }
 0x1089b2b => {
  if cpu.of { pc = 0x1089b8e; continue; }
  pc = 0x1089b2d; continue;
 }
 0x1089b2d => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x61u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5f656c62u32); cpu.esi = v; }
  if (cpu.cf || cpu.zf) { pc = 0x1089b9b; continue; }
  pc = 0x1089b36; continue;
 }
 0x1089b36 => {
  { let v = 0x656c6369u32; cpu.push(v); }
  if !cpu.cf { pc = 0x1089b3d; continue; }
  pc = 0x1089b3d; continue;
 }
 0x1089b38 => {
  cpu.unimplemented(0x1089b38, "Arpl");
  pc = 0x1089b3c; continue;
 }
 0x1089b3c => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x1057193u32); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x1089b44; continue;
 }
 0x1089b3d => {
  { let v: u32 = 0x1089b1fu32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1089b47; continue;
 }
 0x1089b40 => {
  if !cpu.of { pc = 0x1089b47; continue; }
  pc = 0x1089b42; continue;
 }
 0x1089b42 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])).wrapping_add(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1089b44; continue;
 }
 0x1089b44 => {
  {  }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1089b47; continue;
 }
 0x1089b47 => {
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089b49); cpu.push(0x1089b4e); f_0108a7ce(cpu); if cpu.returned_to != 0x1089b4e { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089b66; continue;
 }
 0x1089b4d => {
  { let v: u8 = cpu.r8(0x686314ebu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.ebx as u8)); cpu.w8(0x686314ebu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.unimplemented(0x1089b54, "Arpl");
  if !cpu.zf { pc = 0x1089bc8; continue; }
  pc = 0x1089b5a; continue;
 }
 0x1089b52 => {
  cpu.unimplemented(0x1089b52, "Arpl");
  cpu.unimplemented(0x1089b55, "Arpl");
  if !cpu.zf { pc = 0x1089bc8; continue; }
  pc = 0x1089b5a; continue;
 }
 0x1089b5a => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x5fu32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(2)).wrapping_add(cpu.bases[3])), 0x67726174u32); cpu.esi = v; }
  if cpu.zf { pc = 0x1089bd8; continue; }
  pc = 0x1089b65; continue;
 }
 0x1089b65 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x1057193u32); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  {  }
  pc = 0x1089b6e; continue;
 }
 0x1089b66 => {
  { let v: u32 = 0x1089b52u32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089b72); cpu.push(0x1089b77); f_0108a101(cpu); if cpu.returned_to != 0x1089b77 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089b95; continue;
 }
 0x1089b6e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089b72); cpu.push(0x1089b77); f_0108a101(cpu); if cpu.returned_to != 0x1089b77 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089b95; continue;
 }
 0x1089b7b => {
  cpu.unimplemented(0x1089b7b, "Arpl");
  cpu.unimplemented(0x1089b7e, "Arpl");
  if cpu.cf { pc = 0x1089be8; continue; }
  pc = 0x1089b83; continue;
 }
 0x1089b83 => {
  if cpu.of { pc = 0x1089be6; continue; }
  pc = 0x1089b85; continue;
 }
 0x1089b85 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x5fu32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x6564726fu32); cpu.esi = v; }
  if cpu.cf { pc = 0x1089c01; continue; }
  pc = 0x1089b8e; continue;
 }
 0x1089b88 => {
  cpu.unimplemented(0x1089b88, "Outsd");
  if cpu.cf { cpu.call(0x1089bef); return; }
  pc = 0x1089b8b; continue;
 }
 0x1089b8b => {
  if cpu.cf { pc = 0x1089c01; continue; }
  pc = 0x1089b8e; continue;
 }
 0x1089b8e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.cf || cpu.zf) { pc = 0x1089bf2; continue; }
  pc = 0x1089b91; continue;
 }
 0x1089b90 => {
  { cpu.popad(); }
  pc = 0x1089b91; continue;
 }
 0x1089b91 => {
  cpu.unimplemented(0x1089b91, "Insb");
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffc7u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])), 0x5719305u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffff9bu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.edi); cpu.w32(0xffffff9bu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1089b9d; continue;
 }
 0x1089b95 => {
  { let v: u32 = 0x1089b7bu32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089ba1); cpu.push(0x1089ba6); f_0108a164(cpu); if cpu.returned_to != 0x1089ba6 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089bc3; continue;
 }
 0x1089b9b => {
  if !cpu.pf() { cpu.poll(); pc = 0x1089b38; continue; }
  pc = 0x1089b9d; continue;
 }
 0x1089b9d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089ba1); cpu.push(0x1089ba6); f_0108a164(cpu); if cpu.returned_to != 0x1089ba6 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x1089bc3; continue;
 }
 0x1089baa => {
  cpu.unimplemented(0x1089baa, "Arpl");
  cpu.unimplemented(0x1089bad, "Arpl");
  cpu.unimplemented(0x1089bb0, "Outsb");
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x1089c1e; continue; }
  pc = 0x1089bb9; continue;
 }
 0x1089bb9 => {
  if cpu.cf { pc = 0x1089c2d; continue; }
  pc = 0x1089bbb; continue;
 }
 0x1089bbb => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2)).wrapping_add(cpu.bases[3])), 0x736569u32); cpu.esi = v; }
  pc = 0x1089bc3; continue;
 }
 0x1089bc3 => {
  { let v: u32 = 0x1089baau32; cpu.w32(0x1057193u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089bcf); cpu.push(0x1089bd4); f_0108b671(cpu); if cpu.returned_to != 0x1089bd4 { return; }
  pc = 0x1089bd4; continue;
 }
 0x1089bc8 => {
  { let v: u32 = cpu.r32(0x6101089bu32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.ebp); cpu.w32(0x6101089bu32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.flat_ds(0x1089bcf); cpu.push(0x1089bd4); f_0108b671(cpu); if cpu.returned_to != 0x1089bd4 { return; }
  pc = 0x1089bd4; continue;
 }
 0x1089bd4 => {
  cpu.flat_ds(0x1089bd4); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1089bd5 => {
  { let v: u32 = cpu.r32(0x1133b72u32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = 0x11eb414u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1133b76u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  pc = 0x1089be6; continue;
 }
 0x1089bd8 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  { let v: u32 = cpu.r32(0x11eb414u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])).wrapping_add(cpu.edi); cpu.w32(0x11eb414u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x1133b76u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  pc = 0x1089be6; continue;
 }
 0x1089be6 => {
  { let v: u32 = 0x116e734u32; cpu.ebp = v; }
  pc = 0x1089beb; continue;
 }
 0x1089be8 => {
  { cpu.port_out(0x16u16, 4, cpu.eax as u32); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.call(0x1089bed); return;
 }
 0x1089beb => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x1089bf2; continue;
 }
 0x1089bf2 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1089c62; continue; }
  pc = 0x1089bf8; continue;
 }
 0x1089bf8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.flat_ds(0x1089bfa); cpu.push(0x1089bff); f_010868d2(cpu); if cpu.returned_to != 0x1089bff { return; }
  cpu.flat_ds(0x1089bff); cpu.push(0x1089c04); f_01086867(cpu); if cpu.returned_to != 0x1089c04 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0xffu8); }
  if cpu.zf { pc = 0x1089c62; continue; }
  pc = 0x1089c0c; continue;
 }
 0x1089c01 => {
  cpu.unimplemented(0x1089c01, "Int3");
  cpu.call(0x1089c02); return;
 }
 0x1089c0c => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.mul8(v); }
  { let v: u8 = cpu.r8(0x1152e85u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  pc = 0x1089c1e; continue;
 }
 0x1089c1e => {
  if !cpu.zf { pc = 0x1089c62; continue; }
  pc = 0x1089c20; continue;
 }
 0x1089c20 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  pc = 0x1089c2d; continue;
 }
 0x1089c2d => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x26u8); }
  if cpu.zf { pc = 0x1089c5c; continue; }
  pc = 0x1089c33; continue;
 }
 0x1089c33 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x24u8); }
  if cpu.zf { pc = 0x1089c5c; continue; }
  pc = 0x1089c39; continue;
 }
 0x1089c39 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x2u8); }
  if !cpu.zf { pc = 0x1089c62; continue; }
  pc = 0x1089c3f; continue;
 }
 0x1089c3f => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.flat_ds(0x1089c4a); cpu.push(0x1089c4f); f_0108c1c5(cpu); if cpu.returned_to != 0x1089c4f { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x1089c62; continue; }
  pc = 0x1089c56; continue;
 }
 0x1089c56 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x1089c62; continue; }
  pc = 0x1089c5c; continue;
 }
 0x1089c5c => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1089c62; continue;
 }
 0x1089c62 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x16u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x48u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1089beb; continue; }
  pc = 0x1089c74; continue;
 }
 0x1089c74 => {
  { let v: u32 = 0x11eb414u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1089c80; continue;
 }
 0x1089c80 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0xffffffffu32); }
  if cpu.zf { pc = 0x1089c8a; continue; }
  pc = 0x1089c88; continue;
 }
 0x1089c88 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  pc = 0x1089c8a; continue;
 }
 0x1089c8a => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1089c80; continue; }
  pc = 0x1089c91; continue;
 }
 0x1089c91 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1133838u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x113383cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1133840u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(cpu.r8(0x116e609u32.wrapping_add(cpu.bases[3]))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1133378u32.wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x113384cu32.wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  if !cpu.sf { pc = 0x1089cc4; continue; }
  pc = 0x1089cc2; continue;
 }
 0x1089cc2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1089cc4; continue;
 }
 0x1089cc4 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x1133838u32.wrapping_add(cpu.bases[3]))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1089cd1; continue; }
  pc = 0x1089ccc; continue;
 }
 0x1089ccc => {
  { let v: u32 = cpu.r32(0x1133838u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x1089cd1; continue;
 }
 0x1089cd1 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1133844u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x113383cu32.wrapping_add(cpu.bases[3])).wrapping_sub(cpu.eax); cpu.w32(0x113383cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x1133830u32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.r32(0x113383cu32.wrapping_add(cpu.bases[3]))); cpu.esi = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1089cfb; continue; }
  pc = 0x1089cea; continue;
 }
 0x1089cea => {
  { let v: u32 = cpu.r32(0x113383cu32.wrapping_add(cpu.bases[3])).wrapping_add(cpu.esi); cpu.w32(0x113383cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.esi); cpu.eax = v; }
  if !cpu.sf { pc = 0x1089cf6; continue; }
  pc = 0x1089cf4; continue;
 }
 0x1089cf4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1089cf6; continue;
 }
 0x1089cf6 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1133844u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x1089cfb; continue;
 }
 0x1089cfb => {
  { cpu.sub32(cpu.eax, cpu.r32(0x1133b87u32.wrapping_add(cpu.bases[3]))); }
  if cpu.zf { pc = 0x1089d04; continue; }
  pc = 0x1089d03; continue;
 }
 0x1089d03 => {
  {  }
  pc = 0x1089d04; continue;
 }
 0x1089d04 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1133b87u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x116e734u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x11eb414u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1089d1a; continue;
 }
 0x1089d1a => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x1089d5b; continue; }
  pc = 0x1089d20; continue;
 }
 0x1089d20 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x10u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.w8(0x11eb7d7u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) & 0xefu8; cpu.w8(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x1089d47; continue; }
  pc = 0x1089d37; continue;
 }
 0x1089d37 => {
  { cpu.sub8(cpu.r8(0x11eb7d7u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x1089d5b; continue; }
  pc = 0x1089d40; continue;
 }
 0x1089d40 => {
  cpu.flat_ds(0x1089d40); cpu.push(0x1089d45); f_01089efd(cpu); if cpu.returned_to != 0x1089d45 { return; }
  pc = 0x1089d5b; continue;
 }
 0x1089d47 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0xffffffffu32); }
  if cpu.zf { pc = 0x1089d5b; continue; }
  pc = 0x1089d4f; continue;
 }
 0x1089d4f => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x8u8); }
  if cpu.zf { pc = 0x1089d5b; continue; }
  pc = 0x1089d55; continue;
 }
 0x1089d55 => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) | 0x10u8; cpu.w8(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1089d5b; continue;
 }
 0x1089d5b => {
  { let v: u32 = cpu.esi.wrapping_add(0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1089d1a; continue; }
  pc = 0x1089d65; continue;
 }
 0x1089d65 => {
  { let v: u32 = cpu.edx; cpu.w32(0x1133848u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.sub32(cpu.r32(0x1133840u32.wrapping_add(cpu.bases[3])), cpu.edx); cpu.w32(0x1133840u32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x1089d71); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01089d72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1089d72); }
 let mut pc: u32 = 0x1089d72;
 loop { match pc {
 0x1089d72 => {
  { let v: u32 = 0x116e734u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1089d7c; continue;
 }
 0x1089d7c => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1089e3a; continue; }
  pc = 0x1089d86; continue;
 }
 0x1089d86 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x13u32.wrapping_add(cpu.esi))); cpu.w8(0x13u32.wrapping_add(cpu.esi), v); }
  if !cpu.sf { pc = 0x1089e3a; continue; }
  pc = 0x1089d8f; continue;
 }
 0x1089d8f => {
  { let v: u8 = cpu.inc8(cpu.r8(0x13u32.wrapping_add(cpu.esi))); cpu.w8(0x13u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x1089e3a; continue; }
  pc = 0x1089d9e; continue;
 }
 0x1089d9e => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.eax)), 0x3u8); }
  if !cpu.zf { pc = 0x1089e3a; continue; }
  pc = 0x1089da8; continue;
 }
 0x1089da8 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1089db1); f_0108a681(cpu); if cpu.returned_to != 0x1089db1 { return; }
  if cpu.cf { pc = 0x1089e3a; continue; }
  pc = 0x1089db7; continue;
 }
 0x1089db7 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1089e3a; continue; }
  pc = 0x1089dbf; continue;
 }
 0x1089dbf => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1089e3a; continue; }
  pc = 0x1089dc5; continue;
 }
 0x1089dc5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x113381au32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x113381cu32, v); }
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
  if !cpu.cf { pc = 0x1089e3a; continue; }
  pc = 0x1089e18; continue;
 }
 0x1089e18 => {
  { let v: u16 = cpu.r16(0x113381au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x113381cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1089e34); f_0106a04e(cpu); if cpu.returned_to != 0x1089e34 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x64u8; cpu.w8(0x13u32.wrapping_add(cpu.esi), v); }
  pc = 0x1089e3a; continue;
 }
 0x1089e3a => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1089d7c; continue; }
  pc = 0x1089e44; continue;
 }
 0x1089e44 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01089e45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1089e45); }
 let mut pc: u32 = 0x1089e45;
 loop { match pc {
 0x1089e45 => {
  { cpu.sub8(cpu.r8(0x10e3d63u32), 0x12u8); }
  if cpu.zf { pc = 0x1089efc; continue; }
  pc = 0x1089e52; continue;
 }
 0x1089e52 => {
  { let v: u32 = 0x11a188cu32; cpu.ebp = v; }
  { let v: u8 = 0xc8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1089e59; continue;
 }
 0x1089e59 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1089ef1; continue; }
  pc = 0x1089e66; continue;
 }
 0x1089e66 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x1089ef1; continue; }
  pc = 0x1089e71; continue;
 }
 0x1089e71 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x10c5974u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xc8u32); }
  if cpu.zf { pc = 0x1089ed4; continue; }
  pc = 0x1089e7f; continue;
 }
 0x1089e7f => {
  { cpu.sub32(cpu.ebx, 0x271u32); }
  if cpu.zf { pc = 0x1089ed4; continue; }
  pc = 0x1089e87; continue;
 }
 0x1089e87 => {
  { cpu.sub32(cpu.ebx, 0x2d5u32); }
  if cpu.zf { pc = 0x1089ed4; continue; }
  pc = 0x1089e8f; continue;
 }
 0x1089e8f => {
  { cpu.sub32(cpu.ebx, 0x307u32); }
  if cpu.zf { pc = 0x1089ed4; continue; }
  pc = 0x1089e97; continue;
 }
 0x1089e97 => {
  { cpu.sub32(cpu.ebx, 0x20du32); }
  if cpu.zf { pc = 0x1089ed4; continue; }
  pc = 0x1089e9f; continue;
 }
 0x1089e9f => {
  { cpu.sub32(cpu.ebx, 0x226u32); }
  if cpu.zf { pc = 0x1089ed4; continue; }
  pc = 0x1089ea7; continue;
 }
 0x1089ea7 => {
  { cpu.sub32(cpu.ebx, 0x23fu32); }
  if cpu.zf { pc = 0x1089ed4; continue; }
  pc = 0x1089eaf; continue;
 }
 0x1089eaf => {
  { cpu.sub32(cpu.ebx, 0x258u32); }
  if cpu.zf { pc = 0x1089ed4; continue; }
  pc = 0x1089eb7; continue;
 }
 0x1089eb7 => {
  { cpu.sub32(cpu.ebx, 0x339u32); }
  if cpu.zf { pc = 0x1089ed4; continue; }
  pc = 0x1089ebf; continue;
 }
 0x1089ebf => {
  { cpu.sub32(cpu.ebx, 0x36bu32); }
  if cpu.zf { pc = 0x1089ed4; continue; }
  pc = 0x1089ec7; continue;
 }
 0x1089ec7 => {
  { cpu.sub32(cpu.ebx, 0x39du32); }
  if cpu.zf { pc = 0x1089ed4; continue; }
  pc = 0x1089ecf; continue;
 }
 0x1089ecf => {
  { cpu.sub32(cpu.ebx, 0x7du32); }
  if !cpu.zf { pc = 0x1089ef1; continue; }
  pc = 0x1089ed4; continue;
 }
 0x1089ed4 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1089ee1); f_0108c1c5(cpu); if cpu.returned_to != 0x1089ee1 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x1089ef1; continue; }
  pc = 0x1089ee9; continue;
 }
 0x1089ee9 => {
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.zf { pc = 0x1089ef1; continue; }
  pc = 0x1089eed; continue;
 }
 0x1089eed => {
  { let v: u8 = cpu.r8(0x1au32.wrapping_add(cpu.ebp)) & 0xfeu8; cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1089ef1; continue;
 }
 0x1089ef1 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x28u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1089e59; continue; }
  pc = 0x1089efc; continue;
 }
 0x1089efc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01089efd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1089efd); }
 let mut pc: u32 = 0x1089efd;
 loop { match pc {
 0x1089efd => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1089f50; continue; }
  pc = 0x1089f04; continue;
 }
 0x1089f04 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.zf { pc = 0x1089f24; continue; }
  pc = 0x1089f0a; continue;
 }
 0x1089f0a => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1089f24; continue; }
  pc = 0x1089f10; continue;
 }
 0x1089f10 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x1089f24; continue; }
  pc = 0x1089f16; continue;
 }
 0x1089f16 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x1089f38; continue; }
  pc = 0x1089f1c; continue;
 }
 0x1089f1c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.eax)), 0x2u8); }
  if !cpu.zf { pc = 0x1089f53; continue; }
  pc = 0x1089f24; continue;
 }
 0x1089f24 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x1089f38; continue; }
  pc = 0x1089f2c; continue;
 }
 0x1089f2c => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1089f38; continue; }
  pc = 0x1089f31; continue;
 }
 0x1089f31 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1089f37); f_01087af9(cpu); if cpu.returned_to != 0x1089f37 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1089f38; continue;
 }
 0x1089f38 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1089f3f); f_0108a681(cpu); if cpu.returned_to != 0x1089f3f { return; }
  if cpu.cf { pc = 0x1089f50; continue; }
  pc = 0x1089f41; continue;
 }
 0x1089f41 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1133850u32); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x1089f50); f_0108b988(cpu); if cpu.returned_to != 0x1089f50 { return; }
  pc = 0x1089f50; continue;
 }
 0x1089f50 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1089f53 => {
  { let v: u8 = cpu.or8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); cpu.w8(0x10u32.wrapping_add(cpu.esi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01089f59(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1089f59); }
 let mut pc: u32 = 0x1089f59;
 loop { match pc {
 0x1089f59 => {
  cpu.push(0x1089f5e); f_0108a033(cpu); if cpu.returned_to != 0x1089f5e { return; }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x1089f68; continue;
 }
 0x1089f68 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x116c0d8u32); cpu.eax = v; }
  if cpu.zf { pc = 0x1089f7c; continue; }
  pc = 0x1089f71; continue;
 }
 0x1089f71 => {
  { let v: u32 = 0x50u32; cpu.edx = v; }
  { let v = (cpu.edx as u8); cpu.div8(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x1089fce; continue; }
  pc = 0x1089f7c; continue;
 }
 0x1089f7c => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.sf { pc = 0x1089fce; continue; }
  pc = 0x1089f84; continue;
 }
 0x1089f84 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x1089fce; continue; }
  pc = 0x1089f8a; continue;
 }
 0x1089f8a => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.edx)), 0x1u8); }
  if !cpu.zf { pc = 0x1089fce; continue; }
  pc = 0x1089f93; continue;
 }
 0x1089f93 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1089f9a); f_01031c81(cpu); if cpu.returned_to != 0x1089f9a { return; }
  { cpu.sub8((cpu.eax as u8), 0x1au8); }
  if cpu.cf { pc = 0x1089fa0; continue; }
  pc = 0x1089f9e; continue;
 }
 0x1089f9e => {
  { let v: u8 = 0x19u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1089fa0; continue;
 }
 0x1089fa0 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x50u32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10e4144u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e413cu32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1089fcc); f_01089fd5(cpu); if cpu.returned_to != 0x1089fcc { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1089fce; continue;
 }
 0x1089fce => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x1eu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1089f68; continue; }
  pc = 0x1089fd4; continue;
 }
 0x1089fd4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01089fd5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1089fd5); }
 let mut pc: u32 = 0x1089fd5;
 loop { match pc {
 0x1089fd5 => {
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x1089fdf; continue;
 }
 0x1089fdf => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.sf { pc = 0x108a02c; continue; }
  pc = 0x1089fe6; continue;
 }
 0x1089fe6 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x108a028; continue; }
  pc = 0x1089fec; continue;
 }
 0x1089fec => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e4138u32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e413cu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x10e4144u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x108a02c; continue; }
  pc = 0x108a028; continue;
 }
 0x108a028 => {
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.esi)) | 0x1u8; cpu.w8(0x1du32.wrapping_add(cpu.esi), v); }
  pc = 0x108a02c; continue;
 }
 0x108a02c => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1089fdf; continue; }
  pc = 0x108a032; continue;
 }
 0x108a032 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a033(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a033); }
 let mut pc: u32 = 0x108a033;
 loop { match pc {
 0x108a033 => {
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x108a03d; continue;
 }
 0x108a03d => {
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.esi)) & 0xfeu8; cpu.w8(0x1du32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108a03d; continue; }
  pc = 0x108a047; continue;
 }
 0x108a047 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a048(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a048); }
 let mut pc: u32 = 0x108a048;
 loop { match pc {
 0x108a048 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x108a06a); f_0108c1c5(cpu); if cpu.returned_to != 0x108a06a { return; }
  cpu.push(0x108a06f); f_0108bb7a(cpu); if cpu.returned_to != 0x108a06f { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x1133820u32), 0x0u32); }
  if cpu.zf { pc = 0x108a0f8; continue; }
  pc = 0x108a079; continue;
 }
 0x108a079 => {
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0x11eb140u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x1133820u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x108a090; continue;
 }
 0x108a090 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x108a0e0; continue; }
  pc = 0x108a09b; continue;
 }
 0x108a09b => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.cf { pc = 0x108a0a6; continue; }
  pc = 0x108a0a1; continue;
 }
 0x108a0a1 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), ((cpu.ecx >> 8) as u8)); }
  if !cpu.zf { pc = 0x108a0e0; continue; }
  pc = 0x108a0a6; continue;
 }
 0x108a0a6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e4138u32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e413cu32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.cf { pc = 0x108a0e0; continue; }
  pc = 0x108a0dc; continue;
 }
 0x108a0dc => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x108a0e0; continue;
 }
 0x108a0e0 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108a090; continue; }
  pc = 0x108a0e9; continue;
 }
 0x108a0e9 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x108a0f8; continue; }
  pc = 0x108a0ee; continue;
 }
 0x108a0ee => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108a0f8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a100(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a100); }
 let mut pc: u32 = 0x108a100;
 loop { match pc {
 0x108a100 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a101(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a101); }
 let mut pc: u32 = 0x108a101;
 loop { match pc {
 0x108a101 => {
  { let v: u32 = 0x11757a4u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x116e734u32; cpu.ebp = v; }
  pc = 0x108a10d; continue;
 }
 0x108a10d => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x108a159; continue; }
  pc = 0x108a113; continue;
 }
 0x108a113 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x108a159; continue; }
  pc = 0x108a119; continue;
 }
 0x108a119 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x108a159; continue; }
  pc = 0x108a11f; continue;
 }
 0x108a11f => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x108a159; continue; }
  pc = 0x108a125; continue;
 }
 0x108a125 => {
  cpu.push(0x108a12a); f_0108b3a5(cpu); if cpu.returned_to != 0x108a12a { return; }
  if cpu.cf { pc = 0x108a159; continue; }
  pc = 0x108a12c; continue;
 }
 0x108a12c => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x108a159; continue; }
  pc = 0x108a134; continue;
 }
 0x108a134 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.edi)), 0x0u16); }
  if !cpu.zf { pc = 0x108a159; continue; }
  pc = 0x108a13c; continue;
 }
 0x108a13c => {
  { cpu.pushad(); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  cpu.push(0x108a158); f_0108b988(cpu); if cpu.returned_to != 0x108a158 { return; }
  { cpu.popad(); }
  pc = 0x108a159; continue;
 }
 0x108a159 => {
  { let v: u32 = cpu.esi.wrapping_add(0x48u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x16u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108a10d; continue; }
  pc = 0x108a163; continue;
 }
 0x108a163 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a164(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a164); }
 let mut pc: u32 = 0x108a164;
 loop { match pc {
 0x108a164 => {
  { let v: u32 = 0x11757a4u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x116e734u32; cpu.ebp = v; }
  pc = 0x108a170; continue;
 }
 0x108a170 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x108a338; continue; }
  pc = 0x108a17c; continue;
 }
 0x108a17c => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x108a338; continue; }
  pc = 0x108a186; continue;
 }
 0x108a186 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x108a338; continue; }
  pc = 0x108a190; continue;
 }
 0x108a190 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x108a1a0; continue; }
  pc = 0x108a196; continue;
 }
 0x108a196 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x9u8); }
  if !cpu.zf { pc = 0x108a338; continue; }
  pc = 0x108a1a0; continue;
 }
 0x108a1a0 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x108a1a7); f_0108a681(cpu); if cpu.returned_to != 0x108a1a7 { return; }
  if cpu.cf { pc = 0x108a338; continue; }
  pc = 0x108a1ad; continue;
 }
 0x108a1ad => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.eax)), 0x2084u32); }
  if !cpu.zf { pc = 0x108a1bb; continue; }
  pc = 0x108a1b6; continue;
 }
 0x108a1b6 => {
  pc = 0x108a338; continue;
 }
 0x108a1bb => {
  { let v: u32 = cpu.eax; cpu.w32(0x10e4154u32, v); }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.eax)), 0x40u8); }
  if cpu.zf { pc = 0x108a1e5; continue; }
  pc = 0x108a1c9; continue;
 }
 0x108a1c9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x108a1d9); f_0108c1c5(cpu); if cpu.returned_to != 0x108a1d9 { return; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.zf { pc = 0x108a1f5; continue; }
  pc = 0x108a1e0; continue;
 }
 0x108a1e0 => {
  pc = 0x108a279; continue;
 }
 0x108a1e5 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x108a279; continue; }
  pc = 0x108a1f5; continue;
 }
 0x108a1f5 => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x108a225; continue; }
  pc = 0x108a200; continue;
 }
 0x108a200 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.eax)), 0x40u8); }
  if cpu.zf { pc = 0x108a21a; continue; }
  pc = 0x108a209; continue;
 }
 0x108a209 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.eax)) & 0xfbu8; cpu.w8(0xfu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  pc = 0x108a225; continue;
 }
 0x108a21a => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.eax)) & 0xfbu8; cpu.w8(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x37u32.wrapping_add(cpu.eax), v); }
  pc = 0x108a225; continue;
 }
 0x108a225 => {
  { cpu.pushad(); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xc8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  cpu.push(0x108a26f); f_01036a75(cpu); if cpu.returned_to != 0x108a26f { return; }
  { cpu.popad(); }
  { let v: u8 = 0x8u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  pc = 0x108a338; continue;
 }
 0x108a279 => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if cpu.zf { pc = 0x108a315; continue; }
  pc = 0x108a288; continue;
 }
 0x108a288 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x9u8); }
  if !cpu.zf { pc = 0x108a338; continue; }
  pc = 0x108a292; continue;
 }
 0x108a292 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.eax)), 0x40u8); }
  if cpu.zf { pc = 0x108a2d3; continue; }
  pc = 0x108a29b; continue;
 }
 0x108a29b => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x108a2ad; continue; }
  pc = 0x108a2a4; continue;
 }
 0x108a2a4 => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x108a2b8; continue; }
  pc = 0x108a2ad; continue;
 }
 0x108a2ad => {
  { cpu.sub8(cpu.r8(0x83u32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x108a2b8; continue; }
  pc = 0x108a2b6; continue;
 }
 0x108a2b6 => {
  pc = 0x108a2ba; continue;
 }
 0x108a2b8 => {
  pc = 0x108a338; continue;
 }
 0x108a2ba => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.eax)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x108a1f5; continue; }
  pc = 0x108a2c7; continue;
 }
 0x108a2c7 => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.eax)), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108a1f5; continue; }
  pc = 0x108a2d1; continue;
 }
 0x108a2d1 => {
  pc = 0x108a338; continue;
 }
 0x108a2d3 => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x108a2e5; continue; }
  pc = 0x108a2dc; continue;
 }
 0x108a2dc => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x108a2f0; continue; }
  pc = 0x108a2e5; continue;
 }
 0x108a2e5 => {
  { cpu.sub8(cpu.r8(0x83u32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x108a2f0; continue; }
  pc = 0x108a2ee; continue;
 }
 0x108a2ee => {
  pc = 0x108a2f2; continue;
 }
 0x108a2f0 => {
  pc = 0x108a338; continue;
 }
 0x108a2f2 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.eax)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x108a1f5; continue; }
  pc = 0x108a2ff; continue;
 }
 0x108a2ff => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.eax)), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108a1f5; continue; }
  pc = 0x108a309; continue;
 }
 0x108a309 => {
  { cpu.sub8(cpu.r8(0x1cu32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108a1f5; continue; }
  pc = 0x108a313; continue;
 }
 0x108a313 => {
  pc = 0x108a338; continue;
 }
 0x108a315 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.edi)), 0x0u16); }
  if cpu.zf { pc = 0x108a323; continue; }
  pc = 0x108a321; continue;
 }
 0x108a321 => {
  if !cpu.sf { pc = 0x108a329; continue; }
  pc = 0x108a323; continue;
 }
 0x108a323 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.poll(); pc = 0x108a1f5; continue;
 }
 0x108a329 => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1cu32.wrapping_add(cpu.edi))); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x108a1f5; continue; }
  pc = 0x108a338; continue;
 }
 0x108a338 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x48u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x16u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108a170; continue; }
  pc = 0x108a348; continue;
 }
 0x108a348 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a349(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a349); }
 let mut pc: u32 = 0x108a349;
 loop { match pc {
 0x108a349 => {
  { cpu.pushad(); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x108a36f; continue; }
  pc = 0x108a35d; continue;
 }
 0x108a35d => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax & 0x303u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x108a36f; continue; }
  pc = 0x108a36c; continue;
 }
 0x108a36c => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108a36f => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a372(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a372); }
 let mut pc: u32 = 0x108a372;
 loop { match pc {
 0x108a372 => {
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x108a37c; continue;
 }
 0x108a37c => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x108a38e; continue; }
  pc = 0x108a382; continue;
 }
 0x108a382 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x108a395; continue; }
  pc = 0x108a388; continue;
 }
 0x108a388 => {
  { cpu.sub8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x20u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x108a395; continue; }
  pc = 0x108a38e; continue;
 }
 0x108a38e => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108a37c; continue; }
  pc = 0x108a393; continue;
 }
 0x108a393 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108a395 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { cpu.poll(); pc = 0x108a393; continue; }
  pc = 0x108a39d; continue;
 }
 0x108a39d => {
  { let v: u32 = 0x1152e54u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x108a3a7; continue;
 }
 0x108a3a7 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x108a3d3; continue; }
  pc = 0x108a3b0; continue;
 }
 0x108a3b0 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x108a3d3; continue; }
  pc = 0x108a3b6; continue;
 }
 0x108a3b6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x108a3c6); f_0108c1c5(cpu); if cpu.returned_to != 0x108a3c6 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x108a3d3; continue; }
  pc = 0x108a3cf; continue;
 }
 0x108a3cf => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x108a3dd; continue;
 }
 0x108a3d3 => {
  { let v: u32 = cpu.add32(cpu.edi, 0xb8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108a3a7; continue; }
  pc = 0x108a3db; continue;
 }
 0x108a3db => {
  cpu.poll(); pc = 0x108a393; continue;
 }
 0x108a3dd => {
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x108a3e7; continue;
 }
 0x108a3e7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x108a4db; continue; }
  pc = 0x108a3f3; continue;
 }
 0x108a3f3 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if !cpu.zf { pc = 0x108a4db; continue; }
  pc = 0x108a400; continue;
 }
 0x108a400 => {
  cpu.push(0x108a405); f_0108a641(cpu); if cpu.returned_to != 0x108a405 { return; }
  if cpu.cf { pc = 0x108a4db; continue; }
  pc = 0x108a40b; continue;
 }
 0x108a40b => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x10u8); }
  if !cpu.zf { pc = 0x108a4db; continue; }
  pc = 0x108a415; continue;
 }
 0x108a415 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x108a4db; continue; }
  pc = 0x108a41f; continue;
 }
 0x108a41f => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x108a4db; continue; }
  pc = 0x108a429; continue;
 }
 0x108a429 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x108a4db; continue; }
  pc = 0x108a433; continue;
 }
 0x108a433 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x108a4db; continue; }
  pc = 0x108a43d; continue;
 }
 0x108a43d => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x108a4db; continue; }
  pc = 0x108a447; continue;
 }
 0x108a447 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x108a46c; continue; }
  pc = 0x108a44d; continue;
 }
 0x108a44d => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x108a459; continue; }
  pc = 0x108a453; continue;
 }
 0x108a453 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xfbu8); }
  if !cpu.zf { pc = 0x108a4b5; continue; }
  pc = 0x108a459; continue;
 }
 0x108a459 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x108a460); f_01063324(cpu); if cpu.returned_to != 0x108a460 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x108a4db; continue; }
  pc = 0x108a466; continue;
 }
 0x108a466 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x108a4db; continue; }
  pc = 0x108a46a; continue;
 }
 0x108a46a => {
  pc = 0x108a4b5; continue;
 }
 0x108a46c => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x108a4b5; continue; }
  pc = 0x108a472; continue;
 }
 0x108a472 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0xeu32.wrapping_add(cpu.ebx)), 0xfb2eu16); }
  if cpu.zf { pc = 0x108a494; continue; }
  pc = 0x108a47d; continue;
 }
 0x108a47d => {
  { cpu.sub16(cpu.r16(0xeu32.wrapping_add(cpu.ebx)), 0xef1fu16); }
  if !cpu.zf { pc = 0x108a4b5; continue; }
  pc = 0x108a485; continue;
 }
 0x108a485 => {
  { let v: u32 = cpu.r16(0x10u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.zf { pc = 0x108a4b5; continue; }
  pc = 0x108a492; continue;
 }
 0x108a492 => {
  pc = 0x108a4db; continue;
 }
 0x108a494 => {
  { let v: u32 = cpu.r16(0x10u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x108a4b5; continue; }
  pc = 0x108a4a4; continue;
 }
 0x108a4a4 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x108a4db; continue; }
  pc = 0x108a4b5; continue;
 }
 0x108a4b5 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v = cpu.r32(0x11e94acu32); cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11e94acu32, v); }
  cpu.push(0x108a4cf); f_0108b5e0(cpu); if cpu.returned_to != 0x108a4cf { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x11e94acu32, v); }
  { let v: u32 = cpu.r32(0x11e94acu32).wrapping_add(cpu.eax); cpu.w32(0x11e94acu32, v); }
  pc = 0x108a4db; continue;
 }
 0x108a4db => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108a3e7; continue; }
  pc = 0x108a4ea; continue;
 }
 0x108a4ea => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a4ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a4ec); }
 let mut pc: u32 = 0x108a4ec;
 loop { match pc {
 0x108a4ec => {
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x108a4f6; continue;
 }
 0x108a4f6 => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x108a502; continue; }
  pc = 0x108a4fc; continue;
 }
 0x108a4fc => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { pc = 0x108a509; continue; }
  pc = 0x108a502; continue;
 }
 0x108a502 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108a4f6; continue; }
  pc = 0x108a507; continue;
 }
 0x108a507 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108a509 => {
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { pc = 0x108a52a; continue; }
  pc = 0x108a526; continue;
 }
 0x108a526 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x108a52c; continue;
 }
 0x108a52a => {
  cpu.poll(); pc = 0x108a502; continue;
 }
 0x108a52c => {
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x108a536; continue;
 }
 0x108a536 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.sf == cpu.of) { pc = 0x108a632; continue; }
  pc = 0x108a540; continue;
 }
 0x108a540 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if !cpu.zf { pc = 0x108a632; continue; }
  pc = 0x108a54d; continue;
 }
 0x108a54d => {
  cpu.push(0x108a552); f_0108a641(cpu); if cpu.returned_to != 0x108a552 { return; }
  if cpu.cf { pc = 0x108a632; continue; }
  pc = 0x108a558; continue;
 }
 0x108a558 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.eax))); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.zf { pc = 0x108a632; continue; }
  pc = 0x108a568; continue;
 }
 0x108a568 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x108a632; continue; }
  pc = 0x108a572; continue;
 }
 0x108a572 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x108a632; continue; }
  pc = 0x108a57c; continue;
 }
 0x108a57c => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x108a632; continue; }
  pc = 0x108a586; continue;
 }
 0x108a586 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x108a632; continue; }
  pc = 0x108a590; continue;
 }
 0x108a590 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x108a632; continue; }
  pc = 0x108a59a; continue;
 }
 0x108a59a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x108a632; continue; }
  pc = 0x108a5a4; continue;
 }
 0x108a5a4 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x108a632; continue; }
  pc = 0x108a5ae; continue;
 }
 0x108a5ae => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x108a5d3; continue; }
  pc = 0x108a5b4; continue;
 }
 0x108a5b4 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x108a5c0; continue; }
  pc = 0x108a5ba; continue;
 }
 0x108a5ba => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xfbu8); }
  if !cpu.zf { pc = 0x108a61c; continue; }
  pc = 0x108a5c0; continue;
 }
 0x108a5c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x108a5c7); f_01063324(cpu); if cpu.returned_to != 0x108a5c7 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x108a632; continue; }
  pc = 0x108a5cd; continue;
 }
 0x108a5cd => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x108a632; continue; }
  pc = 0x108a5d1; continue;
 }
 0x108a5d1 => {
  pc = 0x108a61c; continue;
 }
 0x108a5d3 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x108a61c; continue; }
  pc = 0x108a5d9; continue;
 }
 0x108a5d9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0xeu32.wrapping_add(cpu.ebx)), 0xfb2eu16); }
  if cpu.zf { pc = 0x108a5fb; continue; }
  pc = 0x108a5e4; continue;
 }
 0x108a5e4 => {
  { cpu.sub16(cpu.r16(0xeu32.wrapping_add(cpu.ebx)), 0xef1fu16); }
  if !cpu.zf { pc = 0x108a61c; continue; }
  pc = 0x108a5ec; continue;
 }
 0x108a5ec => {
  { let v: u32 = cpu.r16(0x10u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.zf { pc = 0x108a61c; continue; }
  pc = 0x108a5f9; continue;
 }
 0x108a5f9 => {
  pc = 0x108a632; continue;
 }
 0x108a5fb => {
  { let v: u32 = cpu.r16(0x10u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x108a61c; continue; }
  pc = 0x108a60b; continue;
 }
 0x108a60b => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x108a632; continue; }
  pc = 0x108a61c; continue;
 }
 0x108a61c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x108a626); f_0108b5e0(cpu); if cpu.returned_to != 0x108a626 { return; }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x15u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108a632 => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108a536; continue; }
  pc = 0x108a63f; continue;
 }
 0x108a63f => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a641(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a641); }
 let mut pc: u32 = 0x108a641;
 loop { match pc {
 0x108a641 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x108a67a; continue; }
  pc = 0x108a64a; continue;
 }
 0x108a64a => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x108a67a; continue; }
  pc = 0x108a650; continue;
 }
 0x108a650 => {
  { let v: u32 = 0x16u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x116e734u32); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x15u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if !cpu.zf { pc = 0x108a67a; continue; }
  pc = 0x108a66e; continue;
 }
 0x108a66e => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x108a67a; continue; }
  pc = 0x108a674; continue;
 }
 0x108a674 => {
  { let v: u8 = 0x1u8; cpu.w8(0x15u32.wrapping_add(cpu.eax), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108a67a => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a681(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a681); }
 let mut pc: u32 = 0x108a681;
 loop { match pc {
 0x108a681 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x116e734u32); cpu.eax = v; }
  { let v: u8 = 0x16u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = 0x48u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x108a6b6; continue; }
  pc = 0x108a6a0; continue;
 }
 0x108a6a0 => {
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x108a6b6; continue; }
  pc = 0x108a6b3; continue;
 }
 0x108a6b3 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108a6b6 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a6b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a6b9); }
 let mut pc: u32 = 0x108a6b9;
 loop { match pc {
 0x108a6b9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x116e734u32); cpu.eax = v; }
  { let v: u8 = 0x16u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = 0x48u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a6d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a6d0); }
 let mut pc: u32 = 0x108a6d0;
 loop { match pc {
 0x108a6d0 => {
  { cpu.sub8(cpu.r8(0x10ef368u32), 0x0u8); }
  if !cpu.zf { pc = 0x108a70b; continue; }
  pc = 0x108a6d9; continue;
 }
 0x108a6d9 => {
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x108a6e3; continue;
 }
 0x108a6e3 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x108a706; continue; }
  pc = 0x108a6eb; continue;
 }
 0x108a6eb => {
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x108a706; continue; }
  pc = 0x108a6f3; continue;
 }
 0x108a6f3 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x108a706; continue; }
  pc = 0x108a6fb; continue;
 }
 0x108a6fb => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x108a706; continue; }
  pc = 0x108a701; continue;
 }
 0x108a701 => {
  cpu.push(0x108a706); f_0108a70c(cpu); if cpu.returned_to != 0x108a706 { return; }
  pc = 0x108a706; continue;
 }
 0x108a706 => {
  { let v: u32 = 0x1eu32.wrapping_add(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108a6e3; continue; }
  pc = 0x108a70b; continue;
 }
 0x108a70b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a70c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a70c); }
 let mut pc: u32 = 0x108a70c;
 loop { match pc {
 0x108a70c => {
  { cpu.pushad(); }
  { let v: u32 = 0x116e734u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x108a719; continue;
 }
 0x108a719 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x108a7c0; continue; }
  pc = 0x108a723; continue;
 }
 0x108a723 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x108a7c0; continue; }
  pc = 0x108a72d; continue;
 }
 0x108a72d => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x108a7c0; continue; }
  pc = 0x108a737; continue;
 }
 0x108a737 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x108a7c0; continue; }
  pc = 0x108a740; continue;
 }
 0x108a740 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x108a7cc; continue; }
  pc = 0x108a74a; continue;
 }
 0x108a74a => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x108a7cc; continue; }
  pc = 0x108a750; continue;
 }
 0x108a750 => {
  { cpu.pushad(); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.edi)), 0xffu8); }
  if cpu.zf { pc = 0x108a7bf; continue; }
  pc = 0x108a764; continue;
 }
 0x108a764 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x108a7bf; continue; }
  pc = 0x108a778; continue;
 }
 0x108a778 => {
  { let v: u32 = cpu.edi; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u32 = cpu.r8(0x12u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x13u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x108a7b6); f_01036a75(cpu); if cpu.returned_to != 0x108a7b6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x2u8); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108a7bf => {
  { cpu.popad(); }
  pc = 0x108a7c0; continue;
 }
 0x108a7c0 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x16u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108a719; continue; }
  pc = 0x108a7cc; continue;
 }
 0x108a7cc => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a7ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a7ce); }
 let mut pc: u32 = 0x108a7ce;
 loop { match pc {
 0x108a7ce => {
  cpu.push(0x108a7d3); f_0108a8f7(cpu); if cpu.returned_to != 0x108a7d3 { return; }
  cpu.push(0x108a7d8); f_0108a988(cpu); if cpu.returned_to != 0x108a7d8 { return; }
  { cpu.sub32(cpu.r32(0x113386eu32), 0x0u32); }
  if cpu.zf { pc = 0x108a8f6; continue; }
  pc = 0x108a7e5; continue;
 }
 0x108a7e5 => {
  { cpu.sub32(cpu.r32(0x113386au32), 0x0u32); }
  if cpu.zf { pc = 0x108a8f6; continue; }
  pc = 0x108a7f2; continue;
 }
 0x108a7f2 => {
  { let v: u32 = 0x1133972u32; cpu.ebx = v; }
  pc = 0x108a7f7; continue;
 }
 0x108a7f7 => {
  { cpu.sub32(cpu.r32(0x113386eu32), 0x0u32); }
  if cpu.zf { pc = 0x108a8f6; continue; }
  pc = 0x108a804; continue;
 }
 0x108a804 => {
  if cpu.sf { pc = 0x108a8f6; continue; }
  pc = 0x108a80a; continue;
 }
 0x108a80a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e99u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1133862u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1133866u32, v); }
  { let v: u32 = 0x1133a72u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x113386au32); cpu.ecx = v; }
  pc = 0x108a83f; continue;
 }
 0x108a83f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x108a8b9; continue; }
  pc = 0x108a84b; continue;
 }
 0x108a84b => {
  { cpu.sub32(cpu.esi, 0x0u32); }
  if cpu.zf { pc = 0x108a8b9; continue; }
  pc = 0x108a850; continue;
 }
 0x108a850 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x10c6120u32.wrapping_add(cpu.eax); cpu.edi = v; }
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
  { let v: u16 = cpu.r16(0x10e3e97u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e99u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x108a896); f_01035ca1(cpu); if cpu.returned_to != 0x108a896 { return; }
  { cpu.sub32(cpu.eax, 0x10000u32); }
  if (cpu.sf == cpu.of) { pc = 0x108a8b9; continue; }
  pc = 0x108a89d; continue;
 }
 0x108a89d => {
  { cpu.sub32(cpu.r32(0x1133866u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x108a8ae; continue; }
  pc = 0x108a8a6; continue;
 }
 0x108a8a6 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x1133866u32)); }
  if (cpu.sf == cpu.of) { pc = 0x108a8b9; continue; }
  pc = 0x108a8ae; continue;
 }
 0x108a8ae => {
  { let v: u32 = cpu.eax; cpu.w32(0x1133866u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1133862u32, v); }
  pc = 0x108a8b9; continue;
 }
 0x108a8b9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108a83f; continue; }
  pc = 0x108a8c8; continue;
 }
 0x108a8c8 => {
  { cpu.sub32(cpu.r32(0x1133862u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x108a8f6; continue; }
  pc = 0x108a8d1; continue;
 }
 0x108a8d1 => {
  { let v: u32 = cpu.r32(0x1133862u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x108a8e8; continue; }
  pc = 0x108a8de; continue;
 }
 0x108a8de => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x108a8e8; continue; }
  pc = 0x108a8e3; continue;
 }
 0x108a8e3 => {
  cpu.push(0x108a8e8); f_0108a9e1(cpu); if cpu.returned_to != 0x108a8e8 { return; }
  pc = 0x108a8e8; continue;
 }
 0x108a8e8 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x113386eu32)); cpu.w32(0x113386eu32, v); }
  cpu.poll(); pc = 0x108a7f7; continue;
 }
 0x108a8f6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a8f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a8f7); }
 let mut pc: u32 = 0x108a8f7;
 loop { match pc {
 0x108a8f7 => {
  { cpu.pushad(); }
  { let v: u32 = 0x1133972u32; cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x0u32; cpu.w32(0x113386eu32, v); }
  { let v: u32 = 0x1133972u32; cpu.esi = v; }
  { let v: u32 = 0x116e734u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x108a924; continue;
 }
 0x108a924 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x108a97e; continue; }
  pc = 0x108a92a; continue;
 }
 0x108a92a => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x108a97e; continue; }
  pc = 0x108a930; continue;
 }
 0x108a930 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x108a97e; continue; }
  pc = 0x108a936; continue;
 }
 0x108a936 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x108a97e; continue; }
  pc = 0x108a93c; continue;
 }
 0x108a93c => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x108a943); f_0108a681(cpu); if cpu.returned_to != 0x108a943 { return; }
  if cpu.cf { pc = 0x108a97e; continue; }
  pc = 0x108a945; continue;
 }
 0x108a945 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x108a97e; continue; }
  pc = 0x108a94b; continue;
 }
 0x108a94b => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x108a97e; continue; }
  pc = 0x108a951; continue;
 }
 0x108a951 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.sf == cpu.of) { pc = 0x108a97e; continue; }
  pc = 0x108a963; continue;
 }
 0x108a963 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x113386eu32)); cpu.w32(0x113386eu32, v); }
  pc = 0x108a97e; continue;
 }
 0x108a97e => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x16u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108a924; continue; }
  pc = 0x108a986; continue;
 }
 0x108a986 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a988(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a988); }
 let mut pc: u32 = 0x108a988;
 loop { match pc {
 0x108a988 => {
  { let v: u32 = 0x1133a72u32; cpu.esi = v; }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x113386au32, v); }
  pc = 0x108a9a1; continue;
 }
 0x108a9a1 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x108a9da; continue; }
  pc = 0x108a9a8; continue;
 }
 0x108a9a8 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x108a9da; continue; }
  pc = 0x108a9ae; continue;
 }
 0x108a9ae => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x108a9da; continue; }
  pc = 0x108a9b6; continue;
 }
 0x108a9b6 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x108a9da; continue; }
  pc = 0x108a9bf; continue;
 }
 0x108a9bf => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x113386au32)); cpu.w32(0x113386au32, v); }
  pc = 0x108a9da; continue;
 }
 0x108a9da => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108a9a1; continue; }
  pc = 0x108a9e0; continue;
 }
 0x108a9e0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108a9e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108a9e1); }
 let mut pc: u32 = 0x108a9e1;
 loop { match pc {
 0x108a9e1 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x108a9e9); f_0108a681(cpu); if cpu.returned_to != 0x108a9e9 { return; }
  if cpu.cf { pc = 0x108aa4b; continue; }
  pc = 0x108a9eb; continue;
 }
 0x108a9eb => {
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
  { let v: u32 = 0x10c6120u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x34u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x35u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  cpu.push(0x108aa3f); f_01036a75(cpu); if cpu.returned_to != 0x108aa3f { return; }
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
 0x108aa4b => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108aa4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108aa4e); }
 let mut pc: u32 = 0x108aa4e;
 loop { match pc {
 0x108aa4e => {
  { let v: u32 = 0x11757a4u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x116e734u32; cpu.ebp = v; }
  pc = 0x108aa5a; continue;
 }
 0x108aa5a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x108aad6; continue; }
  pc = 0x108aa62; continue;
 }
 0x108aa62 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x108aad6; continue; }
  pc = 0x108aa68; continue;
 }
 0x108aa68 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x108aad6; continue; }
  pc = 0x108aa6e; continue;
 }
 0x108aa6e => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x108aad6; continue; }
  pc = 0x108aa74; continue;
 }
 0x108aa74 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x108aad6; continue; }
  pc = 0x108aa7d; continue;
 }
 0x108aa7d => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x108aa84); f_0108a681(cpu); if cpu.returned_to != 0x108aa84 { return; }
  if cpu.cf { pc = 0x108aad6; continue; }
  pc = 0x108aa86; continue;
 }
 0x108aa86 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x24u8); }
  if cpu.zf { pc = 0x108aa94; continue; }
  pc = 0x108aa8c; continue;
 }
 0x108aa8c => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x26u8); }
  if cpu.zf { pc = 0x108aa94; continue; }
  pc = 0x108aa92; continue;
 }
 0x108aa92 => {
  pc = 0x108aad6; continue;
 }
 0x108aa94 => {
  { let v: u8 = 0x8u8; cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xc8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { cpu.pushad(); }
  cpu.push(0x108aad5); f_01036a75(cpu); if cpu.returned_to != 0x108aad5 { return; }
  { cpu.popad(); }
  pc = 0x108aad6; continue;
 }
 0x108aad6 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x48u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x16u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108aa5a; continue; }
  pc = 0x108aae6; continue;
 }
 0x108aae6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108aae9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108aae9); }
 let mut pc: u32 = 0x108aae9;
 loop { match pc {
 0x108aae9 => {
  cpu.push(0x108aaee); f_0108addc(cpu); if cpu.returned_to != 0x108aaee { return; }
  { let v: u32 = 0x11338f2u32; cpu.ebx = v; }
  pc = 0x108aaf3; continue;
 }
 0x108aaf3 => {
  { cpu.sub8(cpu.r8(0x1133860u32), 0x0u8); }
  if cpu.zf { pc = 0x108abe6; continue; }
  pc = 0x108ab00; continue;
 }
 0x108ab00 => {
  if cpu.sf { pc = 0x108abe6; continue; }
  pc = 0x108ab06; continue;
 }
 0x108ab06 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e99u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1133862u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1133866u32, v); }
  { let v: u32 = 0x11dbce8u32; cpu.edi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x108ab3a; continue;
 }
 0x108ab3a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x108abae; continue; }
  pc = 0x108ab46; continue;
 }
 0x108ab46 => {
  { cpu.sub8(cpu.r8(0x37u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x108abae; continue; }
  pc = 0x108ab4c; continue;
 }
 0x108ab4c => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x10c6120u32.wrapping_add(cpu.eax); cpu.edi = v; }
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
  { let v: u16 = cpu.r16(0x10e3e97u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e99u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x108ab92); f_01035ca1(cpu); if cpu.returned_to != 0x108ab92 { return; }
  { cpu.sub32(cpu.r32(0x1133866u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x108aba3; continue; }
  pc = 0x108ab9b; continue;
 }
 0x108ab9b => {
  { cpu.sub32(cpu.eax, cpu.r32(0x1133866u32)); }
  if (cpu.sf == cpu.of) { pc = 0x108abae; continue; }
  pc = 0x108aba3; continue;
 }
 0x108aba3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1133866u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1133862u32, v); }
  pc = 0x108abae; continue;
 }
 0x108abae => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108ab3a; continue; }
  pc = 0x108abb8; continue;
 }
 0x108abb8 => {
  { cpu.sub32(cpu.r32(0x1133862u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x108abe6; continue; }
  pc = 0x108abc1; continue;
 }
 0x108abc1 => {
  { let v: u32 = cpu.r32(0x1133862u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x108abd8; continue; }
  pc = 0x108abce; continue;
 }
 0x108abce => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x108abd8; continue; }
  pc = 0x108abd3; continue;
 }
 0x108abd3 => {
  cpu.push(0x108abd8); f_0108ae52(cpu); if cpu.returned_to != 0x108abd8 { return; }
  pc = 0x108abd8; continue;
 }
 0x108abd8 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x1133860u32)); cpu.w8(0x1133860u32, v); }
  cpu.poll(); pc = 0x108aaf3; continue;
 }
 0x108abe6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108abe7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108abe7); }
 let mut pc: u32 = 0x108abe7;
 loop { match pc {
 0x108abe7 => {
  cpu.push(0x108abec); f_0108addc(cpu); if cpu.returned_to != 0x108abec { return; }
  { let v: u32 = 0x11338f2u32; cpu.ebx = v; }
  pc = 0x108abf1; continue;
 }
 0x108abf1 => {
  { cpu.sub8(cpu.r8(0x1133860u32), 0x0u8); }
  if cpu.zf { pc = 0x108addb; continue; }
  pc = 0x108abfe; continue;
 }
 0x108abfe => {
  if cpu.sf { pc = 0x108addb; continue; }
  pc = 0x108ac04; continue;
 }
 0x108ac04 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e99u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1133862u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1133866u32, v); }
  { let v: u32 = 0x11d58bcu32; cpu.edi = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  pc = 0x108ac38; continue;
 }
 0x108ac38 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x108ad38; continue; }
  pc = 0x108ac48; continue;
 }
 0x108ac48 => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), 0x8u8); }
  if !cpu.zf { pc = 0x108ad38; continue; }
  pc = 0x108ac52; continue;
 }
 0x108ac52 => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x108acd2; continue; }
  pc = 0x108ac58; continue;
 }
 0x108ac58 => {
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
  if cpu.zf { pc = 0x108ac86; continue; }
  pc = 0x108ac82; continue;
 }
 0x108ac82 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x108ac86; continue;
 }
 0x108ac86 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x10e3e97u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e99u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x108ac9f); f_01035ca1(cpu); if cpu.returned_to != 0x108ac9f { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133b7au32, v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x11u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x108acc4); f_01035ca1(cpu); if cpu.returned_to != 0x108acc4 { return; }
  { cpu.sub32(cpu.eax, cpu.r32(0x1133b7au32)); }
  if (cpu.sf == cpu.of) { pc = 0x108accf; continue; }
  pc = 0x108accc; continue;
 }
 0x108accc => {
  { cpu.popad(); }
  pc = 0x108ad38; continue;
 }
 0x108accf => {
  { cpu.popad(); }
  pc = 0x108acd8; continue;
 }
 0x108acd2 => {
  { cpu.sub8(cpu.r8(0x19u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x108ad38; continue; }
  pc = 0x108acd8; continue;
 }
 0x108acd8 => {
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
  if cpu.zf { pc = 0x108ad05; continue; }
  pc = 0x108ad01; continue;
 }
 0x108ad01 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x108ad05; continue;
 }
 0x108ad05 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x10e3e97u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e99u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x108ad1c); f_01035ca1(cpu); if cpu.returned_to != 0x108ad1c { return; }
  { cpu.sub32(cpu.r32(0x1133866u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x108ad2d; continue; }
  pc = 0x108ad25; continue;
 }
 0x108ad25 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x1133866u32)); }
  if (cpu.sf == cpu.of) { pc = 0x108ad38; continue; }
  pc = 0x108ad2d; continue;
 }
 0x108ad2d => {
  { let v: u32 = cpu.eax; cpu.w32(0x1133866u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1133862u32, v); }
  pc = 0x108ad38; continue;
 }
 0x108ad38 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108ac38; continue; }
  pc = 0x108ad47; continue;
 }
 0x108ad47 => {
  { cpu.sub32(cpu.r32(0x1133862u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x108addb; continue; }
  pc = 0x108ad54; continue;
 }
 0x108ad54 => {
  { let v: u32 = cpu.r32(0x1133862u32); cpu.esi = v; }
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x108adbc; continue; }
  pc = 0x108ad60; continue;
 }
 0x108ad60 => {
  { cpu.pushad(); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x11u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x108ad76); f_0108a641(cpu); if cpu.returned_to != 0x108ad76 { return; }
  { let v: u8 = 0x8u8; cpu.w8(0x11u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x108ad7f); f_0108a6b9(cpu); if cpu.returned_to != 0x108ad7f { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xc8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  cpu.push(0x108adbb); f_01036a75(cpu); if cpu.returned_to != 0x108adbb { return; }
  { cpu.popad(); }
  pc = 0x108adbc; continue;
 }
 0x108adbc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x108adcd; continue; }
  pc = 0x108adc3; continue;
 }
 0x108adc3 => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x108adcd; continue; }
  pc = 0x108adc8; continue;
 }
 0x108adc8 => {
  cpu.push(0x108adcd); f_0108af0a(cpu); if cpu.returned_to != 0x108adcd { return; }
  pc = 0x108adcd; continue;
 }
 0x108adcd => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x1133860u32)); cpu.w8(0x1133860u32, v); }
  cpu.poll(); pc = 0x108abf1; continue;
 }
 0x108addb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108addc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108addc); }
 let mut pc: u32 = 0x108addc;
 loop { match pc {
 0x108addc => {
  { cpu.pushad(); }
  { let v: u32 = 0x11338f2u32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u8 = 0x0u8; cpu.w8(0x1133860u32, v); }
  { let v: u32 = 0x11338f2u32; cpu.esi = v; }
  { let v: u32 = 0x116e734u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x108ae06; continue;
 }
 0x108ae06 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x108ae48; continue; }
  pc = 0x108ae0c; continue;
 }
 0x108ae0c => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x108ae48; continue; }
  pc = 0x108ae12; continue;
 }
 0x108ae12 => {
  { cpu.sub8(cpu.r8(0x11u32.wrapping_add(cpu.ebp)), 0x9u8); }
  if cpu.zf { pc = 0x108ae48; continue; }
  pc = 0x108ae18; continue;
 }
 0x108ae18 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x108ae1f); f_0108a681(cpu); if cpu.returned_to != 0x108ae1f { return; }
  if cpu.cf { pc = 0x108ae48; continue; }
  pc = 0x108ae21; continue;
 }
 0x108ae21 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x24u8); }
  if !cpu.zf { pc = 0x108ae48; continue; }
  pc = 0x108ae27; continue;
 }
 0x108ae27 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x108ae48; continue; }
  pc = 0x108ae2d; continue;
 }
 0x108ae2d => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x1133860u32)); cpu.w8(0x1133860u32, v); }
  pc = 0x108ae48; continue;
 }
 0x108ae48 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ebp, 0x16u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108ae06; continue; }
  pc = 0x108ae50; continue;
 }
 0x108ae50 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ae52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ae52); }
 let mut pc: u32 = 0x108ae52;
 loop { match pc {
 0x108ae52 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x108ae5a); f_0108a681(cpu); if cpu.returned_to != 0x108ae5a { return; }
  if cpu.cf { pc = 0x108af07; continue; }
  pc = 0x108ae60; continue;
 }
 0x108ae60 => {
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
  { let v: u32 = cpu.r8(0x6u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x108ae81; continue; }
  pc = 0x108ae7f; continue;
 }
 0x108ae7f => {
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x108ae81; continue;
 }
 0x108ae81 => {
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
  { let v: u32 = 0x10c6120u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r8(0x34u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x35u32.wrapping_add(cpu.esi)) as i8 as i16 as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  cpu.push(0x108aefb); f_01036a75(cpu); if cpu.returned_to != 0x108aefb { return; }
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
 0x108af07 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108af0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108af0a); }
 let mut pc: u32 = 0x108af0a;
 loop { match pc {
 0x108af0a => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x108af12); f_0108a681(cpu); if cpu.returned_to != 0x108af12 { return; }
  if cpu.cf { pc = 0x108b00d; continue; }
  pc = 0x108af18; continue;
 }
 0x108af18 => {
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
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
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
  if !cpu.zf { pc = 0x108af79; continue; }
  pc = 0x108af6d; continue;
 }
 0x108af6d => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x108af79; continue;
 }
 0x108af79 => {
  { cpu.pushad(); }
  cpu.push(0x108af7f); f_0104f36d(cpu); if cpu.returned_to != 0x108af7f { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  if cpu.zf { pc = 0x108afd7; continue; }
  pc = 0x108af8b; continue;
 }
 0x108af8b => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x108af96); f_0104f36d(cpu); if cpu.returned_to != 0x108af96 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  if cpu.zf { pc = 0x108afd7; continue; }
  pc = 0x108afa2; continue;
 }
 0x108afa2 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x108afad); f_0104f36d(cpu); if cpu.returned_to != 0x108afad { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  if cpu.zf { pc = 0x108afd7; continue; }
  pc = 0x108afb9; continue;
 }
 0x108afb9 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x108afc8); f_0104f36d(cpu); if cpu.returned_to != 0x108afc8 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  if cpu.zf { pc = 0x108afd7; continue; }
  pc = 0x108afd4; continue;
 }
 0x108afd4 => {
  { cpu.popad(); }
  pc = 0x108afd9; continue;
 }
 0x108afd7 => {
  { cpu.popad(); }
  {  }
  pc = 0x108afd9; continue;
 }
 0x108afd9 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  cpu.push(0x108b001); f_01036a75(cpu); if cpu.returned_to != 0x108b001 { return; }
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
 0x108b00d => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108b010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b010); }
 let mut pc: u32 = 0x108b010;
 loop { match pc {
 0x108b010 => {
  { let v: u32 = 0x116e734u32; cpu.edi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x11757a4u32; cpu.ebx = v; }
  pc = 0x108b01c; continue;
 }
 0x108b01c => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x108b06d; continue; }
  pc = 0x108b022; continue;
 }
 0x108b022 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x108b06d; continue; }
  pc = 0x108b028; continue;
 }
 0x108b028 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0x4u8); }
  if cpu.zf { pc = 0x108b041; continue; }
  pc = 0x108b03c; continue;
 }
 0x108b03c => {
  cpu.push(0x108b041); f_0108b078(cpu); if cpu.returned_to != 0x108b041 { return; }
  pc = 0x108b041; continue;
 }
 0x108b041 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { pc = 0x108b06d; continue; }
  pc = 0x108b047; continue;
 }
 0x108b047 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x108b06d; continue; }
  pc = 0x108b04e; continue;
 }
 0x108b04e => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x108b06d; continue; }
  pc = 0x108b054; continue;
 }
 0x108b054 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x108b05e); f_0108b344(cpu); if cpu.returned_to != 0x108b05e { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x108b06d; continue; }
  pc = 0x108b065; continue;
 }
 0x108b065 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x108b06d; continue; }
  pc = 0x108b069; continue;
 }
 0x108b069 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) & 0xfdu8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x108b06d; continue;
 }
 0x108b06d => {
  { let v: u32 = cpu.edi.wrapping_add(0x16u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x48u32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108b01c; continue; }
  pc = 0x108b077; continue;
 }
 0x108b077 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108b078(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b078); }
 let mut pc: u32 = 0x108b078;
 loop { match pc {
 0x108b078 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x108b090; continue; }
  pc = 0x108b07f; continue;
 }
 0x108b07f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x108b089); f_0108b344(cpu); if cpu.returned_to != 0x108b089 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x108b091; continue; }
  pc = 0x108b090; continue;
 }
 0x108b090 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108b091 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x108b09b; continue; }
  pc = 0x108b097; continue;
 }
 0x108b097 => {
  { let v: u8 = cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0xfbu8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x108b09b; continue;
 }
 0x108b09b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108b09c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b09c); }
 let mut pc: u32 = 0x108b09c;
 loop { match pc {
 0x108b09c => {
  { let v: u32 = 0x116de88u32; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x108b0a8; continue;
 }
 0x108b0a8 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.edi)), 0x0u16); }
  if cpu.zf { pc = 0x108b0b4; continue; }
  pc = 0x108b0b0; continue;
 }
 0x108b0b0 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x16u32.wrapping_add(cpu.edi))); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x108b0b4; continue;
 }
 0x108b0b4 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x1eu32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108b0a8; continue; }
  pc = 0x108b0b9; continue;
 }
 0x108b0b9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108b1df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b1df); }
 let mut pc: u32 = 0x108b1df;
 loop { match pc {
 0x108b1df => {
  { let v: u32 = cpu.r32(0x10e3f6cu32); cpu.ebp = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x108b1ea; continue;
 }
 0x108b1ea => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x108b313; continue; }
  pc = 0x108b1f4; continue;
 }
 0x108b1f4 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.sf == cpu.of) { pc = 0x108b313; continue; }
  pc = 0x108b1fe; continue;
 }
 0x108b1fe => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x108b313; continue; }
  pc = 0x108b208; continue;
 }
 0x108b208 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x108b313; continue; }
  pc = 0x108b212; continue;
 }
 0x108b212 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x108b313; continue; }
  pc = 0x108b21c; continue;
 }
 0x108b21c => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x108b313; continue; }
  pc = 0x108b226; continue;
 }
 0x108b226 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x108b313; continue; }
  pc = 0x108b230; continue;
 }
 0x108b230 => {
  cpu.push(0x108b235); f_0108a641(cpu); if cpu.returned_to != 0x108b235 { return; }
  if cpu.cf { pc = 0x108b313; continue; }
  pc = 0x108b23b; continue;
 }
 0x108b23b => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x10u8); }
  if !cpu.zf { pc = 0x108b313; continue; }
  pc = 0x108b245; continue;
 }
 0x108b245 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x108b253); f_0108c1c5(cpu); if cpu.returned_to != 0x108b253 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x108b313; continue; }
  pc = 0x108b25a; continue;
 }
 0x108b25a => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x8u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x108b313; continue; }
  pc = 0x108b268; continue;
 }
 0x108b268 => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x108b291; continue; }
  pc = 0x108b270; continue;
 }
 0x108b270 => {
  { let v: u32 = 0x16u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x116e734u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x2u8); }
  if !cpu.zf { pc = 0x108b313; continue; }
  pc = 0x108b287; continue;
 }
 0x108b287 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x8u8); }
  if cpu.zf { pc = 0x108b313; continue; }
  pc = 0x108b291; continue;
 }
 0x108b291 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x108b2bb; continue; }
  pc = 0x108b299; continue;
 }
 0x108b299 => {
  { let v: u32 = cpu.r8(0x116e727u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x9u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0xcu32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0xau32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x108b313; continue; }
  pc = 0x108b2bb; continue;
 }
 0x108b2bb => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x108b2ce); f_0108c1c5(cpu); if cpu.returned_to != 0x108b2ce { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x108b2d6); f_0108b48f(cpu); if cpu.returned_to != 0x108b2d6 { return; }
  cpu.push(0x108b2db); f_0108b344(cpu); if cpu.returned_to != 0x108b2db { return; }
  if cpu.cf { pc = 0x108b2e1; continue; }
  pc = 0x108b2dd; continue;
 }
 0x108b2dd => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x108b2f1; continue; }
  pc = 0x108b2e1; continue;
 }
 0x108b2e1 => {
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x108b2f1); f_0108b988(cpu); if cpu.returned_to != 0x108b2f1 { return; }
  pc = 0x108b2f1; continue;
 }
 0x108b2f1 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x3bu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x108b313; continue; }
  pc = 0x108b30a; continue;
 }
 0x108b30a => {
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  pc = 0x108b313; continue;
 }
 0x108b313 => {
  { let v: u32 = 0xb8u32.wrapping_add(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108b1ea; continue; }
  pc = 0x108b320; continue;
 }
 0x108b320 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e3f6cu32, v); }
  { let v: u32 = cpu.sub32(cpu.r32(0x10e3f70u32), 0x14u32); cpu.w32(0x10e3f70u32, v); }
  if !cpu.zf { pc = 0x108b343; continue; }
  pc = 0x108b32f; continue;
 }
 0x108b32f => {
  { let v: u32 = 0xf0u32; cpu.w32(0x10e3f70u32, v); }
  { let v: u32 = 0x1152e54u32; cpu.w32(0x10e3f6cu32, v); }
  pc = 0x108b343; continue;
 }
 0x108b343 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108b344(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b344); }
 let mut pc: u32 = 0x108b344;
 loop { match pc {
 0x108b344 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x108b39d; continue; }
  pc = 0x108b350; continue;
 }
 0x108b350 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x108b36b; continue; }
  pc = 0x108b363; continue;
 }
 0x108b363 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x108b39d; continue; }
  pc = 0x108b369; continue;
 }
 0x108b369 => {
  pc = 0x108b377; continue;
 }
 0x108b36b => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x108b39d; continue; }
  pc = 0x108b371; continue;
 }
 0x108b371 => {
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x108b39d; continue; }
  pc = 0x108b377; continue;
 }
 0x108b377 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1133810u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1133812u32, v); }
  cpu.push(0x108b391); f_0108c1c5(cpu); if cpu.returned_to != 0x108b391 { return; }
  if cpu.cf { pc = 0x108b39d; continue; }
  pc = 0x108b393; continue;
 }
 0x108b393 => {
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
 0x108b39d => {
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
pub fn f_0108b3a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b3a5); }
 let mut pc: u32 = 0x108b3a5;
 loop { match pc {
 0x108b3a5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x108b487; continue; }
  pc = 0x108b3b5; continue;
 }
 0x108b3b5 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x108b432; continue; }
  pc = 0x108b3c8; continue;
 }
 0x108b3c8 => {
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x108b487; continue; }
  pc = 0x108b3d2; continue;
 }
 0x108b3d2 => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.esi)), 0xd80u16); }
  if cpu.zf { pc = 0x108b41b; continue; }
  pc = 0x108b3da; continue;
 }
 0x108b3da => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x108b404; continue; }
  pc = 0x108b3e3; continue;
 }
 0x108b3e3 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x108b487; continue; }
  pc = 0x108b404; continue;
 }
 0x108b404 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x108b423; continue;
 }
 0x108b41b => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x108b423; continue;
 }
 0x108b423 => {
  cpu.push(0x108b428); f_0108c1c5(cpu); if cpu.returned_to != 0x108b428 { return; }
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
 0x108b432 => {
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x108b487; continue; }
  pc = 0x108b438; continue;
 }
 0x108b438 => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.esi)), 0xfb2eu16); }
  if cpu.zf { pc = 0x108b452; continue; }
  pc = 0x108b440; continue;
 }
 0x108b440 => {
  { cpu.sub16(cpu.r16(0x2cu32.wrapping_add(cpu.esi)), 0xef1fu16); }
  if cpu.zf { pc = 0x108b47f; continue; }
  pc = 0x108b448; continue;
 }
 0x108b448 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x108b423; continue;
 }
 0x108b452 => {
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x11757a4u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x108b487; continue; }
  pc = 0x108b462; continue;
 }
 0x108b462 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x108b487; continue; }
  pc = 0x108b468; continue;
 }
 0x108b468 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  pc = 0x108b470; continue;
 }
 0x108b470 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x1152e54u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x108b423; continue;
 }
 0x108b47f => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x108b470; continue;
 }
 0x108b487 => {
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
pub fn f_0108b48f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b48f); }
 let mut pc: u32 = 0x108b48f;
 loop { match pc {
 0x108b48f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  pc = 0x108b4bb; continue;
 }
 0x108b4bb => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x8u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x108b506; continue; }
  pc = 0x108b4c0; continue;
 }
 0x108b4c0 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r8(0x12u32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3e93u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u16 = cpu.r8(0x13u32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3e95u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.cf { pc = 0x108b506; continue; }
  pc = 0x108b4fa; continue;
 }
 0x108b4fa => {
  { cpu.sub32(cpu.esi, cpu.r32(0x10e4134u32)); }
  if cpu.zf { pc = 0x108b506; continue; }
  pc = 0x108b502; continue;
 }
 0x108b502 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x108b506; continue;
 }
 0x108b506 => {
  { let v: u32 = 0x1eu32.wrapping_add(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108b4bb; continue; }
  pc = 0x108b50b; continue;
 }
 0x108b50b => {
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
pub fn f_0108b513(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b513); }
 let mut pc: u32 = 0x108b513;
 loop { match pc {
 0x108b513 => {
  { cpu.pushad(); }
  { cpu.sub16(cpu.r16(0x10e3f39u32), 0x0u16); }
  if cpu.zf { pc = 0x108b5de; continue; }
  pc = 0x108b522; continue;
 }
 0x108b522 => {
  { let a = cpu.esi; let b = cpu.ebp; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.ebp = v; } }
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x108b5de; continue; }
  pc = 0x108b52e; continue;
 }
 0x108b52e => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x108b542); f_0108c1c5(cpu); if cpu.returned_to != 0x108b542 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4134u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  pc = 0x108b559; continue;
 }
 0x108b559 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if (cpu.sf == cpu.of) { pc = 0x108b5d1; continue; }
  pc = 0x108b55f; continue;
 }
 0x108b55f => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x108b5d1; continue; }
  pc = 0x108b565; continue;
 }
 0x108b565 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.ebx as u8), ((cpu.ebx >> 8) as u8)); }
  if !cpu.zf { pc = 0x108b5d1; continue; }
  pc = 0x108b56f; continue;
 }
 0x108b56f => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x108b5d1; continue; }
  pc = 0x108b575; continue;
 }
 0x108b575 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x10e4154u32)); }
  if cpu.zf { pc = 0x108b5d1; continue; }
  pc = 0x108b57d; continue;
 }
 0x108b57d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x108b58e); f_0108c1c5(cpu); if cpu.returned_to != 0x108b58e { return; }
  { cpu.sub32(cpu.esi, cpu.r32(0x10e4134u32)); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.zf { pc = 0x108b5d1; continue; }
  pc = 0x108b59a; continue;
 }
 0x108b59a => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x108b5d1; continue; }
  pc = 0x108b5a0; continue;
 }
 0x108b5a0 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x108b5d1; continue; }
  pc = 0x108b5a6; continue;
 }
 0x108b5a6 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x108b5d1; continue; }
  pc = 0x108b5ac; continue;
 }
 0x108b5ac => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x108b5d1; continue; }
  pc = 0x108b5b2; continue;
 }
 0x108b5b2 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x108b5d1; continue; }
  pc = 0x108b5b8; continue;
 }
 0x108b5b8 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x108b5d1; continue; }
  pc = 0x108b5be; continue;
 }
 0x108b5be => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x108b5d1; continue; }
  pc = 0x108b5c4; continue;
 }
 0x108b5c4 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x108b5d1; continue; }
  pc = 0x108b5ca; continue;
 }
 0x108b5ca => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x108b5d1); f_0108b5e0(cpu); if cpu.returned_to != 0x108b5d1 { return; }
  pc = 0x108b5d1; continue;
 }
 0x108b5d1 => {
  { let v: u32 = 0xb8u32.wrapping_add(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108b559; continue; }
  pc = 0x108b5de; continue;
 }
 0x108b5de => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108b5e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b5e0); }
 let mut pc: u32 = 0x108b5e0;
 loop { match pc {
 0x108b5e0 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x108b63d; continue; }
  pc = 0x108b5ec; continue;
 }
 0x108b5ec => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x108b63d; continue; }
  pc = 0x108b5f2; continue;
 }
 0x108b5f2 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x108b604; continue; }
  pc = 0x108b5f8; continue;
 }
 0x108b5f8 => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x108b611; continue;
 }
 0x108b604 => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x108b611; continue;
 }
 0x108b611 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x5u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e13u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x108b636); f_01036a75(cpu); if cpu.returned_to != 0x108b636 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108b63a => {
  {  }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108b63d => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { cpu.poll(); pc = 0x108b63a; continue; }
  pc = 0x108b643; continue;
 }
 0x108b643 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e13u32, v); }
  { let v: u16 = 0xfb2eu16; cpu.w16(0x10e3e0fu32, v); }
  cpu.push(0x108b657); f_01062fda(cpu); if cpu.returned_to != 0x108b657 { return; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e11u32, v); }
  cpu.push(0x108b66f); f_01036a75(cpu); if cpu.returned_to != 0x108b66f { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108b671(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b671); }
 let mut pc: u32 = 0x108b671;
 loop { match pc {
 0x108b671 => {
  { cpu.sub32(cpu.r32(0x11eb134u32), 0x0u32); }
  if cpu.zf { pc = 0x108b6c1; continue; }
  pc = 0x108b67a; continue;
 }
 0x108b67a => {
  { let v: u32 = 0x11eb3b0u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11eb134u32); cpu.ecx = v; }
  pc = 0x108b685; continue;
 }
 0x108b685 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x108b69f); f_0108c1c5(cpu); if cpu.returned_to != 0x108b69f { return; }
  if cpu.cf { pc = 0x108b6be; continue; }
  pc = 0x108b6a1; continue;
 }
 0x108b6a1 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x8u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x108b6be; continue; }
  pc = 0x108b6ab; continue;
 }
 0x108b6ab => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0xcu32.wrapping_add(cpu.esi))); }
  if cpu.cf { pc = 0x108b6be; continue; }
  pc = 0x108b6b5; continue;
 }
 0x108b6b5 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x108b6bc); f_0108b6c2(cpu); if cpu.returned_to != 0x108b6bc { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x108b6be; continue;
 }
 0x108b6be => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108b685; continue; }
  pc = 0x108b6c1; continue;
 }
 0x108b6c1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108b6c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b6c2); }
 let mut pc: u32 = 0x108b6c2;
 loop { match pc {
 0x108b6c2 => {
  cpu.push(0x108b6c7); f_0108bb7a(cpu); if cpu.returned_to != 0x108b6c7 { return; }
  { let v: u32 = cpu.r32(0x1133820u32); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x108b713; continue; }
  pc = 0x108b6d1; continue;
 }
 0x108b6d1 => {
  { let v: u32 = 0x11eb140u32; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0xau32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x108b6e3; continue;
 }
 0x108b6e3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x108b703; continue; }
  pc = 0x108b6ef; continue;
 }
 0x108b6ef => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0xau32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x108b703; continue; }
  pc = 0x108b6f8; continue;
 }
 0x108b6f8 => {
  cpu.push(0x108b6fd); f_0108b88c(cpu); if cpu.returned_to != 0x108b6fd { return; }
  if cpu.cf { pc = 0x108b703; continue; }
  pc = 0x108b6ff; continue;
 }
 0x108b6ff => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x108b703; continue;
 }
 0x108b703 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108b6e3; continue; }
  pc = 0x108b708; continue;
 }
 0x108b708 => {
  { cpu.sub32(cpu.edx, cpu.esi); }
  if cpu.zf { pc = 0x108b714; continue; }
  pc = 0x108b70c; continue;
 }
 0x108b70c => {
  cpu.push(0x108b711); f_0108b798(cpu); if cpu.returned_to != 0x108b711 { return; }
  if cpu.cf { pc = 0x108b714; continue; }
  pc = 0x108b713; continue;
 }
 0x108b713 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108b714 => {
  cpu.push(0x108b719); f_0108b721(cpu); if cpu.returned_to != 0x108b719 { return; }
  if !cpu.cf { cpu.poll(); pc = 0x108b713; continue; }
  pc = 0x108b71b; continue;
 }
 0x108b71b => {
  cpu.push(0x108b720); f_0108b7d9(cpu); if cpu.returned_to != 0x108b720 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108b721(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b721); }
 let mut pc: u32 = 0x108b721;
 loop { match pc {
 0x108b721 => {
  { cpu.pushad(); }
  { let v: u32 = 0x116e734u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0xau32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x108b73c; continue;
 }
 0x108b73c => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x108b781; continue; }
  pc = 0x108b742; continue;
 }
 0x108b742 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x108b781; continue; }
  pc = 0x108b747; continue;
 }
 0x108b747 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if cpu.zf { pc = 0x108b781; continue; }
  pc = 0x108b750; continue;
 }
 0x108b750 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x108b781; continue; }
  pc = 0x108b756; continue;
 }
 0x108b756 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x108b781; continue; }
  pc = 0x108b75c; continue;
 }
 0x108b75c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x9u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x108b781; continue; }
  pc = 0x108b767; continue;
 }
 0x108b767 => {
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0xau32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x108b781; continue; }
  pc = 0x108b771; continue;
 }
 0x108b771 => {
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x108b781; continue; }
  pc = 0x108b776; continue;
 }
 0x108b776 => {
  cpu.push(0x108b77b); f_0108b88c(cpu); if cpu.returned_to != 0x108b77b { return; }
  if cpu.cf { pc = 0x108b781; continue; }
  pc = 0x108b77d; continue;
 }
 0x108b77d => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x108b781; continue;
 }
 0x108b781 => {
  { let v: u32 = cpu.edi.wrapping_add(0x16u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108b73c; continue; }
  pc = 0x108b786; continue;
 }
 0x108b786 => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x108b795; continue; }
  pc = 0x108b78b; continue;
 }
 0x108b78b => {
  cpu.push(0x108b790); f_0108b798(cpu); if cpu.returned_to != 0x108b790 { return; }
  if cpu.cf { pc = 0x108b795; continue; }
  pc = 0x108b792; continue;
 }
 0x108b792 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108b795 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108b798(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b798); }
 let mut pc: u32 = 0x108b798;
 loop { match pc {
 0x108b798 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x108b7b4; continue; }
  pc = 0x108b7b1; continue;
 }
 0x108b7b1 => {
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  pc = 0x108b7b4; continue;
 }
 0x108b7b4 => {
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xffu8); }
  if cpu.zf { pc = 0x108b7d6; continue; }
  pc = 0x108b7bc; continue;
 }
 0x108b7bc => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x108b7d3); f_0108b988(cpu); if cpu.returned_to != 0x108b7d3 { return; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108b7d6 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108b7d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b7d9); }
 let mut pc: u32 = 0x108b7d9;
 loop { match pc {
 0x108b7d9 => {
  { let v: u32 = 0x116e734u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x108b7e5; continue;
 }
 0x108b7e5 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x108b7fb; continue; }
  pc = 0x108b7eb; continue;
 }
 0x108b7eb => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0x8u8); }
  if cpu.zf { pc = 0x108b7fb; continue; }
  pc = 0x108b7f1; continue;
 }
 0x108b7f1 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0x10u8); }
  if !cpu.zf { pc = 0x108b7fb; continue; }
  pc = 0x108b7f7; continue;
 }
 0x108b7f7 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if cpu.zf { pc = 0x108b803; continue; }
  pc = 0x108b7fb; continue;
 }
 0x108b7fb => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edi, 0x16u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108b7e5; continue; }
  pc = 0x108b802; continue;
 }
 0x108b802 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108b803 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x108b7fb; continue; }
  pc = 0x108b815; continue;
 }
 0x108b815 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0xffffffffu32); }
  if cpu.zf { pc = 0x108b84a; continue; }
  pc = 0x108b81e; continue;
 }
 0x108b81e => {
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x108b84a; continue; }
  pc = 0x108b823; continue;
 }
 0x108b823 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x108b847; continue; }
  pc = 0x108b83d; continue;
 }
 0x108b83d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x108b847); f_0108b988(cpu); if cpu.returned_to != 0x108b847 { return; }
  pc = 0x108b847; continue;
 }
 0x108b847 => {
  { cpu.popad(); }
  cpu.poll(); pc = 0x108b7fb; continue;
 }
 0x108b84a => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.edx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x108b7fb; continue; }
  pc = 0x108b860; continue;
 }
 0x108b860 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x108b871); f_0108c1c5(cpu); if cpu.returned_to != 0x108b871 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x108b879); f_0108b48f(cpu); if cpu.returned_to != 0x108b879 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x108b886); f_0108b988(cpu); if cpu.returned_to != 0x108b886 { return; }
  { cpu.popad(); }
  cpu.poll(); pc = 0x108b7fb; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108b88c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b88c); }
 let mut pc: u32 = 0x108b88c;
 loop { match pc {
 0x108b88c => {
  { cpu.pushad(); }
  { let v: u32 = 0x116e734u32; cpu.edx = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1133824u32, v); }
  pc = 0x108b8a7; continue;
 }
 0x108b8a7 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x108b8c0; continue; }
  pc = 0x108b8ad; continue;
 }
 0x108b8ad => {
  { cpu.sub32(cpu.edi, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x108b8c0; continue; }
  pc = 0x108b8b1; continue;
 }
 0x108b8b1 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x108b8ba); f_0108b8ec(cpu); if cpu.returned_to != 0x108b8ba { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if !cpu.cf { pc = 0x108b8d5; continue; }
  pc = 0x108b8c0; continue;
 }
 0x108b8c0 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x16u32); cpu.edx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108b8a7; continue; }
  pc = 0x108b8c7; continue;
 }
 0x108b8c7 => {
  { cpu.popad(); }
  { cpu.sub32(cpu.r32(0x1133824u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x108b8d3; continue; }
  pc = 0x108b8d1; continue;
 }
 0x108b8d1 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108b8d3 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108b8d5 => {
  { let v: u32 = cpu.r32(0x11eb130u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x1133824u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x108b8c0; continue; }
  pc = 0x108b8e2; continue;
 }
 0x108b8e2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x1133824u32, v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  cpu.poll(); pc = 0x108b8c0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108b8ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b8ec); }
 let mut pc: u32 = 0x108b8ec;
 loop { match pc {
 0x108b8ec => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x11757b7u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11eb130u32, v); }
  pc = 0x108b908; continue;
 }
 0x108b908 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x108b933; continue; }
  pc = 0x108b90d; continue;
 }
 0x108b90d => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x108b933; continue; }
  pc = 0x108b920; continue;
 }
 0x108b920 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x108b929); f_0108693c(cpu); if cpu.returned_to != 0x108b929 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11eb130u32).wrapping_add(cpu.eax); cpu.w32(0x11eb130u32, v); }
  pc = 0x108b933; continue;
 }
 0x108b933 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108b908; continue; }
  pc = 0x108b936; continue;
 }
 0x108b936 => {
  { cpu.sub32(cpu.r32(0x11eb130u32), 0x0u32); }
  if cpu.zf { pc = 0x108b986; continue; }
  pc = 0x108b93f; continue;
 }
 0x108b93f => {
  { cpu.sub32(cpu.r32(0x11eb130u32), 0xffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x108b955; continue; }
  pc = 0x108b94b; continue;
 }
 0x108b94b => {
  { let v: u32 = 0xffffu32; cpu.w32(0x11eb130u32, v); }
  pc = 0x108b955; continue;
 }
 0x108b955 => {
  { let v: u32 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x11eb130u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x108b986; continue; }
  pc = 0x108b969; continue;
 }
 0x108b969 => {
  { let v: u32 = cpu.r8(0x116e727u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x9u32.wrapping_add(cpu.edi)) as u32; cpu.ecx = v; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x108b986; continue; }
  pc = 0x108b97f; continue;
 }
 0x108b97f => {
  { cpu.sub16((cpu.ebx as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x108b986; continue; }
  pc = 0x108b984; continue;
 }
 0x108b984 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108b986 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108b988(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108b988); }
 let mut pc: u32 = 0x108b988;
 loop { match pc {
 0x108b988 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x108ba1d; continue; }
  pc = 0x108b992; continue;
 }
 0x108b992 => {
  cpu.push(0x108b997); f_01087ac6(cpu); if cpu.returned_to != 0x108b997 { return; }
  if cpu.cf { pc = 0x108ba1e; continue; }
  pc = 0x108b99d; continue;
 }
 0x108b99d => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e13u32, v); }
  { let v: u32 = cpu.r8(0x12u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x13u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x108b9da; continue; }
  pc = 0x108b9c4; continue;
 }
 0x108b9c4 => {
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xdcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x108b9da; continue;
 }
 0x108b9da => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x108b9f5; continue; }
  pc = 0x108b9e0; continue;
 }
 0x108b9e0 => {
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x50u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x50u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x108b9f5; continue;
 }
 0x108b9f5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  cpu.push(0x108ba07); f_0108ba50(cpu); if cpu.returned_to != 0x108ba07 { return; }
  if cpu.cf { pc = 0x108ba1d; continue; }
  pc = 0x108ba09; continue;
 }
 0x108ba09 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e11u32, v); }
  { cpu.pushad(); }
  cpu.push(0x108ba1c); f_01036a75(cpu); if cpu.returned_to != 0x108ba1c { return; }
  { cpu.popad(); }
  pc = 0x108ba1d; continue;
 }
 0x108ba1d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108ba1e => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef210u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e13u32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e0fu32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e11u32, v); }
  { cpu.pushad(); }
  cpu.push(0x108ba4e); f_01036a75(cpu); if cpu.returned_to != 0x108ba4e { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ba50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ba50); }
 let mut pc: u32 = 0x108ba50;
 loop { match pc {
 0x108ba50 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x108ba57); f_0105f215(cpu); if cpu.returned_to != 0x108ba57 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x108ba61; continue;
 }
 0x108ba61 => {
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x11337f0u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x11337f2u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x108ba83); f_01058feb(cpu); if cpu.returned_to != 0x108ba83 { return; }
  if cpu.cf { pc = 0x108ba8c; continue; }
  pc = 0x108ba85; continue;
 }
 0x108ba85 => {
  cpu.push(0x108ba8a); f_0108ba9a(cpu); if cpu.returned_to != 0x108ba8a { return; }
  if !cpu.cf { pc = 0x108ba96; continue; }
  pc = 0x108ba8c; continue;
 }
 0x108ba8c => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.and32(cpu.esi, 0x7u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108ba61; continue; }
  pc = 0x108ba92; continue;
 }
 0x108ba92 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108ba96 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ba9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ba9a); }
 let mut pc: u32 = 0x108ba9a;
 loop { match pc {
 0x108ba9a => {
  { cpu.pushad(); }
  { let v: u32 = 0x11757a4u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x116e734u32; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  pc = 0x108bab5; continue;
 }
 0x108bab5 => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x108bb03; continue; }
  pc = 0x108babc; continue;
 }
 0x108babc => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x108bb03; continue; }
  pc = 0x108bac2; continue;
 }
 0x108bac2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  cpu.push(0x108bad8); f_0108b344(cpu); if cpu.returned_to != 0x108bad8 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x108bb03; continue; }
  pc = 0x108badd; continue;
 }
 0x108badd => {
  { let v: u16 = cpu.r16(0x1133810u32) & 0xfff0u16; cpu.w16(0x1133810u32, v); }
  { let v: u16 = cpu.r16(0x1133812u32) & 0xfff0u16; cpu.w16(0x1133812u32, v); }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1133810u32)); }
  if !cpu.zf { pc = 0x108bb03; continue; }
  pc = 0x108baf6; continue;
 }
 0x108baf6 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x1133812u32)); }
  if !cpu.zf { pc = 0x108bb03; continue; }
  pc = 0x108baff; continue;
 }
 0x108baff => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108bb03 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x16u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108bab5; continue; }
  pc = 0x108bb0c; continue;
 }
 0x108bb0c => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108bb0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108bb0f); }
 let mut pc: u32 = 0x108bb0f;
 loop { match pc {
 0x108bb0f => {
  { cpu.pushad(); }
  { let v: u32 = 0x116e734u32; cpu.edi = v; }
  { let v: u32 = 0x11757a4u32; cpu.edx = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x108bb1f; continue;
 }
 0x108bb1f => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x108bb70; continue; }
  pc = 0x108bb25; continue;
 }
 0x108bb25 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) & 0xf7u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.edx)), 0xffu8); }
  if cpu.zf { pc = 0x108bb6b; continue; }
  pc = 0x108bb37; continue;
 }
 0x108bb37 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.edx)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x108bb6b; continue; }
  pc = 0x108bb4c; continue;
 }
 0x108bb4c => {
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x108bb6b; continue; }
  pc = 0x108bb52; continue;
 }
 0x108bb52 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x108bb5c; continue; }
  pc = 0x108bb58; continue;
 }
 0x108bb58 => {
  { let v: u8 = cpu.or8(cpu.r8(0x10u32.wrapping_add(cpu.edi)), 0x8u8); cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x108bb5c; continue;
 }
 0x108bb5c => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x108bb69); f_0108c1c5(cpu); if cpu.returned_to != 0x108bb69 { return; }
  if cpu.cf { pc = 0x108bb6d; continue; }
  pc = 0x108bb6b; continue;
 }
 0x108bb6b => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x108bb6d; continue;
 }
 0x108bb6d => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x108bb70; continue;
 }
 0x108bb70 => {
  { let v: u32 = cpu.edx.wrapping_add(0x48u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x16u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108bb1f; continue; }
  pc = 0x108bb78; continue;
 }
 0x108bb78 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108bb7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108bb7a); }
 let mut pc: u32 = 0x108bb7a;
 loop { match pc {
 0x108bb7a => {
  { cpu.pushad(); }
  { let v: u32 = 0x0u32; cpu.w32(0x1133820u32, v); }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x108bbf7; continue; }
  pc = 0x108bb8a; continue;
 }
 0x108bb8a => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x108bbf7; continue; }
  pc = 0x108bb91; continue;
 }
 0x108bb91 => {
  { let v: u32 = 0x11eb140u32; cpu.edi = v; }
  { let v: u8 = 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x116de88u32; cpu.ebp = v; }
  pc = 0x108bb9d; continue;
 }
 0x108bb9d => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x108bbf0; continue; }
  pc = 0x108bba5; continue;
 }
 0x108bba5 => {
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if cpu.zf { pc = 0x108bbf0; continue; }
  pc = 0x108bba9; continue;
 }
 0x108bba9 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x108bc46; continue; }
  pc = 0x108bbb6; continue;
 }
 0x108bbb6 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x108bc46; continue; }
  pc = 0x108bbc4; continue;
 }
 0x108bbc4 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x108bc30; continue; }
  pc = 0x108bbca; continue;
 }
 0x108bbca => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x108bc30; continue; }
  pc = 0x108bbd0; continue;
 }
 0x108bbd0 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x108bc1b; continue; }
  pc = 0x108bbda; continue;
 }
 0x108bbda => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x6u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x108bc1b; continue; }
  pc = 0x108bbe4; continue;
 }
 0x108bbe4 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x108bc06; continue; }
  pc = 0x108bbea; continue;
 }
 0x108bbea => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x6u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x108bc06; continue; }
  pc = 0x108bbf0; continue;
 }
 0x108bbf0 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x1eu32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108bb9d; continue; }
  pc = 0x108bbf7; continue;
 }
 0x108bbf7 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108bbf9 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x1133820u32)); cpu.w32(0x1133820u32, v); }
  cpu.poll(); pc = 0x108bbf0; continue;
 }
 0x108bc06 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x108bbf0; continue; }
  pc = 0x108bc10; continue;
 }
 0x108bc10 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x108bbf0; continue; }
  pc = 0x108bc19; continue;
 }
 0x108bc19 => {
  cpu.poll(); pc = 0x108bbf9; continue;
 }
 0x108bc1b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x108bbf9; continue; }
  pc = 0x108bc24; continue;
 }
 0x108bc24 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x108bbf9; continue; }
  pc = 0x108bc2e; continue;
 }
 0x108bc2e => {
  cpu.poll(); pc = 0x108bbf0; continue;
 }
 0x108bc30 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x108bbf0; continue; }
  pc = 0x108bc3a; continue;
 }
 0x108bc3a => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x6u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x108bbf0; continue; }
  pc = 0x108bc44; continue;
 }
 0x108bc44 => {
  cpu.poll(); pc = 0x108bbf9; continue;
 }
 0x108bc46 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x6u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x108bbf9; continue; }
  pc = 0x108bc50; continue;
 }
 0x108bc50 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x108bbf9; continue; }
  pc = 0x108bc5a; continue;
 }
 0x108bc5a => {
  cpu.poll(); pc = 0x108bbf0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108bc5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108bc5c); }
 let mut pc: u32 = 0x108bc5c;
 loop { match pc {
 0x108bc5c => {
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x11e94b0u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x113382cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108bc6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108bc6f); }
 let mut pc: u32 = 0x108bc6f;
 loop { match pc {
 0x108bc6f => {
  cpu.push(0x108bc74); f_0108bce4(cpu); if cpu.returned_to != 0x108bc74 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11eb7d6u32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x108bc82); f_0108bcfc(cpu); if cpu.returned_to != 0x108bc82 { return; }
  cpu.push(0x108bc87); f_0108bb7a(cpu); if cpu.returned_to != 0x108bc87 { return; }
  { cpu.sub32(cpu.r32(0x1133820u32), 0x0u32); }
  if cpu.zf { pc = 0x108bce3; continue; }
  pc = 0x108bc90; continue;
 }
 0x108bc90 => {
  { let v: u8 = cpu.r8(0x1133820u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x11eb140u32; cpu.esi = v; }
  pc = 0x108bc9b; continue;
 }
 0x108bc9b => {
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108bc9b; continue; }
  pc = 0x108bca4; continue;
 }
 0x108bca4 => {
  { let v: u32 = 0x11eb290u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x1133820u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x108bcaf; continue;
 }
 0x108bcaf => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x108bcb5); f_0108bb7a(cpu); if cpu.returned_to != 0x108bcb5 { return; }
  { let v: u8 = cpu.r8(0x1133820u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if cpu.zf { pc = 0x108bcdf; continue; }
  pc = 0x108bcbf; continue;
 }
 0x108bcbf => {
  { let v: u32 = 0x11eb140u32; cpu.esi = v; }
  pc = 0x108bcc4; continue;
 }
 0x108bcc4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  cpu.push(0x108bcce); f_0108bcfc(cpu); if cpu.returned_to != 0x108bcce { return; }
  if cpu.cf { pc = 0x108bcdb; continue; }
  pc = 0x108bcd0; continue;
 }
 0x108bcd0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x11eb7d6u32)); cpu.w8(0x11eb7d6u32, v); }
  pc = 0x108bcdb; continue;
 }
 0x108bcdb => {
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x108bcc4; continue; }
  pc = 0x108bcdf; continue;
 }
 0x108bcdf => {
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108bcaf; continue; }
  pc = 0x108bce3; continue;
 }
 0x108bce3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108bce4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108bce4); }
 let mut pc: u32 = 0x108bce4;
 loop { match pc {
 0x108bce4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x11eb190u32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108bcfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108bcfc); }
 let mut pc: u32 = 0x108bcfc;
 loop { match pc {
 0x108bcfc => {
  cpu.push(0x108bd01); f_0108bd1d(cpu); if cpu.returned_to != 0x108bd01 { return; }
  if cpu.cf { pc = 0x108bd1b; continue; }
  pc = 0x108bd03; continue;
 }
 0x108bd03 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x11eb190u32; cpu.esi = v; }
  pc = 0x108bd09; continue;
 }
 0x108bd09 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x108bd16; continue; }
  pc = 0x108bd11; continue;
 }
 0x108bd11 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x108bd09; continue;
 }
 0x108bd16 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108bd1b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108bd1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108bd1d); }
 let mut pc: u32 = 0x108bd1d;
 loop { match pc {
 0x108bd1d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x11eb190u32; cpu.esi = v; }
  pc = 0x108bd23; continue;
 }
 0x108bd23 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x108bd37; continue; }
  pc = 0x108bd2b; continue;
 }
 0x108bd2b => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x108bd34; continue; }
  pc = 0x108bd2f; continue;
 }
 0x108bd2f => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x108bd23; continue;
 }
 0x108bd34 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108bd37 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108bd3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108bd3a); }
 let mut pc: u32 = 0x108bd3a;
 loop { match pc {
 0x108bd3a => {
  { let v: u32 = 0x0u32; cpu.w32(0x11e94acu32, v); }
  cpu.push(0x108bd49); f_01086824(cpu); if cpu.returned_to != 0x108bd49 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x1133b86u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116e728u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d79u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116e729u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d7bu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116e72bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d7du32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116e72au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d81u32, v); }
  cpu.push(0x108bd89); f_0108be03(cpu); if cpu.returned_to != 0x108bd89 { return; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x108bdc1; continue; }
  pc = 0x108bd92; continue;
 }
 0x108bd92 => {
  cpu.push(0x108bd97); f_0108bc5c(cpu); if cpu.returned_to != 0x108bd97 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11eb12eu32, v); }
  cpu.push(0x108bda3); f_0108bee1(cpu); if cpu.returned_to != 0x108bda3 { return; }
  cpu.push(0x108bda8); f_0108becb(cpu); if cpu.returned_to != 0x108bda8 { return; }
  cpu.push(0x108bdad); f_0108bf23(cpu); if cpu.returned_to != 0x108bdad { return; }
  cpu.push(0x108bdb2); f_01089bd5(cpu); if cpu.returned_to != 0x108bdb2 { return; }
  cpu.push(0x108bdb7); f_0108bdc2(cpu); if cpu.returned_to != 0x108bdb7 { return; }
  cpu.push(0x108bdbc); f_01086c38(cpu); if cpu.returned_to != 0x108bdbc { return; }
  cpu.push(0x108bdc1); f_01086d00(cpu); if cpu.returned_to != 0x108bdc1 { return; }
  pc = 0x108bdc1; continue;
 }
 0x108bdc1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108bdc2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108bdc2); }
 let mut pc: u32 = 0x108bdc2;
 loop { match pc {
 0x108bdc2 => {
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x108bdcc; continue;
 }
 0x108bdcc => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x108bdfc; continue; }
  pc = 0x108bdd3; continue;
 }
 0x108bdd3 => {
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.esi)) & 0xbfu8; cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0xfu8); }
  if !cpu.zf { pc = 0x108bdfc; continue; }
  pc = 0x108bddd; continue;
 }
 0x108bddd => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  cpu.push(0x108bdf4); f_0108c206(cpu); if cpu.returned_to != 0x108bdf4 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if !cpu.cf { pc = 0x108bdfc; continue; }
  pc = 0x108bdf8; continue;
 }
 0x108bdf8 => {
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.esi)) | 0x40u8; cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x108bdfc; continue;
 }
 0x108bdfc => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x108bdcc; continue; }
  pc = 0x108be02; continue;
 }
 0x108be02 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108be03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108be03); }
 let mut pc: u32 = 0x108be03;
 loop { match pc {
 0x108be03 => {
  { let v: u32 = 0x116de88u32; cpu.edi = v; }
  { let v: u8 = 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x108be0a; continue;
 }
 0x108be0a => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.edi)), 0xe0u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x108be26); f_0108c1c5(cpu); if cpu.returned_to != 0x108be26 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.cf { pc = 0x108be38; continue; }
  pc = 0x108be2a; continue;
 }
 0x108be2a => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x108be38; continue; }
  pc = 0x108be2e; continue;
 }
 0x108be2e => {
  { let v: u8 = cpu.or8(cpu.r8(0x18u32.wrapping_add(cpu.edi)), 0x1u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1133850u32, v); }
  pc = 0x108be38; continue;
 }
 0x108be38 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x108be4c); f_0108c1c5(cpu); if cpu.returned_to != 0x108be4c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.cf { pc = 0x108be5e; continue; }
  pc = 0x108be50; continue;
 }
 0x108be50 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x108be5e; continue; }
  pc = 0x108be54; continue;
 }
 0x108be54 => {
  { let v: u8 = cpu.or8(cpu.r8(0x18u32.wrapping_add(cpu.edi)), 0x2u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1133854u32, v); }
  pc = 0x108be5e; continue;
 }
 0x108be5e => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x108be72); f_0108c1c5(cpu); if cpu.returned_to != 0x108be72 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.cf { pc = 0x108be84; continue; }
  pc = 0x108be76; continue;
 }
 0x108be76 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x108be84; continue; }
  pc = 0x108be7a; continue;
 }
 0x108be7a => {
  { let v: u8 = cpu.or8(cpu.r8(0x18u32.wrapping_add(cpu.edi)), 0x4u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1133854u32, v); }
  pc = 0x108be84; continue;
 }
 0x108be84 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x108be98); f_0108c1c5(cpu); if cpu.returned_to != 0x108be98 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.cf { pc = 0x108beaa; continue; }
  pc = 0x108be9c; continue;
 }
 0x108be9c => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x108beaa; continue; }
  pc = 0x108bea0; continue;
 }
 0x108bea0 => {
  { let v: u8 = cpu.or8(cpu.r8(0x18u32.wrapping_add(cpu.edi)), 0x8u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1133854u32, v); }
  pc = 0x108beaa; continue;
 }
 0x108beaa => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x108beb5); f_0108bfd5(cpu); if cpu.returned_to != 0x108beb5 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if !cpu.cf { pc = 0x108bebd; continue; }
  pc = 0x108beb9; continue;
 }
 0x108beb9 => {
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.edi)) | 0x10u8; cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  pc = 0x108bebd; continue;
 }
 0x108bebd => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x1eu32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108be0a; continue; }
  pc = 0x108beca; continue;
 }
 0x108beca => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108becb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108becb); }
 let mut pc: u32 = 0x108becb;
 loop { match pc {
 0x108becb => {
  { let v: u32 = 0x116de88u32; cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x108bed7; continue;
 }
 0x108bed7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1eu32.wrapping_add(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108bed7; continue; }
  pc = 0x108bee0; continue;
 }
 0x108bee0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108bee1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108bee1); }
 let mut pc: u32 = 0x108bee1;
 loop { match pc {
 0x108bee1 => {
  { let v: u32 = 0x10eea03u32; cpu.edi = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x116e61au32; cpu.esi = v; }
  { let v: u32 = 0x1au32; cpu.ecx = v; }
  { let v: u32 = 0x10eea03u32; cpu.edi = v; }
  pc = 0x108befe; continue;
 }
 0x108befe => {
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x108bf08; continue; }
  pc = 0x108bf06; continue;
 }
 0x108bf06 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x108bf08; continue;
 }
 0x108bf08 => {
  { cpu.stos(1, false); }
  { let v: u32 = cpu.add32(cpu.esi, 0xau32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108befe; continue; }
  pc = 0x108bf0e; continue;
 }
 0x108bf0e => {
  { let v: u32 = 0x1152e54u32; cpu.w32(0x10e3f6cu32, v); }
  { let v: u32 = 0xf0u32; cpu.w32(0x10e3f70u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108bf23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108bf23); }
 let mut pc: u32 = 0x108bf23;
 loop { match pc {
 0x108bf23 => {
  { let v: u32 = 0x116e734u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x116e61au32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.sub32(cpu.edi, 0xau32); cpu.edi = v; }
  pc = 0x108bf34; continue;
 }
 0x108bf34 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x108bf3b); f_0108a681(cpu); if cpu.returned_to != 0x108bf3b { return; }
  if !cpu.cf { pc = 0x108bf43; continue; }
  pc = 0x108bf3d; continue;
 }
 0x108bf3d => {
  { let v: u8 = 0x0u8; cpu.w8(0x15u32.wrapping_add(cpu.esi), v); }
  pc = 0x108bfa3; continue;
 }
 0x108bf43 => {
  { let v: u8 = 0x1u8; cpu.w8(0x15u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x13u32.wrapping_add(cpu.esi), v); }
  pc = 0x108bf4f; continue;
 }
 0x108bf4f => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x1au8); }
  if (cpu.sf == cpu.of) { pc = 0x108bfa3; continue; }
  pc = 0x108bf54; continue;
 }
 0x108bf54 => {
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0xau32); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x108bf4f; continue; }
  pc = 0x108bf5f; continue;
 }
 0x108bf5f => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x108bfa3; continue; }
  pc = 0x108bf65; continue;
 }
 0x108bf65 => {
  { cpu.sub16(cpu.r16(0x10e3f39u32), 0x0u16); }
  if cpu.zf { pc = 0x108bfa3; continue; }
  pc = 0x108bf6f; continue;
 }
 0x108bf6f => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)) | 0x18u8; cpu.w8(0x10u32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x7u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x1eu32); cpu.eax = v; }
  { let v: u32 = 0x116de88u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x108bf96); f_0108a681(cpu); if cpu.returned_to != 0x108bf96 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x108bfa2; continue; }
  pc = 0x108bf99; continue;
 }
 0x108bf99 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x108bfa2); f_0108b988(cpu); if cpu.returned_to != 0x108bfa2 { return; }
  pc = 0x108bfa2; continue;
 }
 0x108bfa2 => {
  { cpu.popad(); }
  pc = 0x108bfa3; continue;
 }
 0x108bfa3 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108bf34; continue; }
  pc = 0x108bfaa; continue;
 }
 0x108bfaa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108bfab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108bfab); }
 let mut pc: u32 = 0x108bfab;
 loop { match pc {
 0x108bfab => {
  cpu.push(0x108bfb0); f_0108a641(cpu); if cpu.returned_to != 0x108bfb0 { return; }
  if cpu.cf { pc = 0x108bfd4; continue; }
  pc = 0x108bfb2; continue;
 }
 0x108bfb2 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x108bfc1); f_0108c1c5(cpu); if cpu.returned_to != 0x108bfc1 { return; }
  if cpu.cf { pc = 0x108bfc5; continue; }
  pc = 0x108bfc3; continue;
 }
 0x108bfc3 => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x108bfc5; continue;
 }
 0x108bfc5 => {
  { let v: u8 = 0x8u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x11u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x108bfd4; continue;
 }
 0x108bfd4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108bfd5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108bfd5); } r_0108bfd5(cpu, 0x108bfd5); }
pub fn f_0108c083(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108c083); } r_0108bfd5(cpu, 0x108c083); }
fn r_0108bfd5(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x108bfd5 => {
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.w8(0x11eb7d4u32, v); }
  pc = 0x108bfe5; continue;
 }
 0x108bfe5 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x108c02c; continue; }
  pc = 0x108bfec; continue;
 }
 0x108bfec => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x108c02c; continue; }
  pc = 0x108bff4; continue;
 }
 0x108bff4 => {
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x108c02c; continue; }
  pc = 0x108bffa; continue;
 }
 0x108bffa => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x108c00b); f_0108c1c5(cpu); if cpu.returned_to != 0x108c00b { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x108c02c; continue; }
  pc = 0x108c013; continue;
 }
 0x108c013 => {
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.zf { pc = 0x108c02c; continue; }
  pc = 0x108c017; continue;
 }
 0x108c017 => {
  { let v: u8 = cpu.r8(0x11eb7d4u32) | 0x1u8; cpu.w8(0x11eb7d4u32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x11eb7d5u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11eb13cu32, v); }
  pc = 0x108c035; continue;
 }
 0x108c02c => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108bfe5; continue; }
  pc = 0x108c035; continue;
 }
 0x108c035 => {
  { let v: u32 = 0x11a188cu32; cpu.ebp = v; }
  { let v: u8 = 0xc8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x108c03e; continue;
 }
 0x108c03e => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x108c0e7; continue; }
  pc = 0x108c04b; continue;
 }
 0x108c04b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x108c0e7; continue; }
  pc = 0x108c056; continue;
 }
 0x108c056 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x10c5974u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xc8u32); }
  if cpu.zf { pc = 0x108c0b9; continue; }
  pc = 0x108c064; continue;
 }
 0x108c064 => {
  { cpu.sub32(cpu.ebx, 0x271u32); }
  if cpu.zf { pc = 0x108c0b9; continue; }
  pc = 0x108c06c; continue;
 }
 0x108c06c => {
  { cpu.sub32(cpu.ebx, 0x2d5u32); }
  if cpu.zf { pc = 0x108c0b9; continue; }
  pc = 0x108c074; continue;
 }
 0x108c074 => {
  { cpu.sub32(cpu.ebx, 0x307u32); }
  if cpu.zf { pc = 0x108c0b9; continue; }
  pc = 0x108c07c; continue;
 }
 0x108c07c => {
  { cpu.sub32(cpu.ebx, 0x20du32); }
  if cpu.zf { pc = 0x108c0b9; continue; }
  pc = 0x108c084; continue;
 }
 0x108c083 => {
  { let v: u32 = cpu.eax ^ 0x226fb81u32; cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x108c0b9; continue; }
  pc = 0x108c08c; continue;
 }
 0x108c084 => {
  { cpu.sub32(cpu.ebx, 0x226u32); }
  if cpu.zf { pc = 0x108c0b9; continue; }
  pc = 0x108c08c; continue;
 }
 0x108c08c => {
  { cpu.sub32(cpu.ebx, 0x23fu32); }
  if cpu.zf { pc = 0x108c0b9; continue; }
  pc = 0x108c094; continue;
 }
 0x108c094 => {
  { cpu.sub32(cpu.ebx, 0x258u32); }
  if cpu.zf { pc = 0x108c0b9; continue; }
  pc = 0x108c09c; continue;
 }
 0x108c09c => {
  { cpu.sub32(cpu.ebx, 0x339u32); }
  if cpu.zf { pc = 0x108c0b9; continue; }
  pc = 0x108c0a4; continue;
 }
 0x108c0a4 => {
  { cpu.sub32(cpu.ebx, 0x36bu32); }
  if cpu.zf { pc = 0x108c0b9; continue; }
  pc = 0x108c0ac; continue;
 }
 0x108c0ac => {
  { cpu.sub32(cpu.ebx, 0x39du32); }
  if cpu.zf { pc = 0x108c0b9; continue; }
  pc = 0x108c0b4; continue;
 }
 0x108c0b4 => {
  { cpu.sub32(cpu.ebx, 0x7du32); }
  if !cpu.zf { pc = 0x108c0e7; continue; }
  pc = 0x108c0b9; continue;
 }
 0x108c0b9 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x108c0c6); f_0108c1c5(cpu); if cpu.returned_to != 0x108c0c6 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x108c0e7; continue; }
  pc = 0x108c0ce; continue;
 }
 0x108c0ce => {
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.zf { pc = 0x108c0e7; continue; }
  pc = 0x108c0d2; continue;
 }
 0x108c0d2 => {
  { let v: u8 = cpu.r8(0x11eb7d4u32) | 0x2u8; cpu.w8(0x11eb7d4u32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x11eb7d5u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11eb13cu32, v); }
  pc = 0x108c0f4; continue;
 }
 0x108c0e7 => {
  { let v: u8 = cpu.inc8(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp.wrapping_add(0x28u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108c03e; continue; }
  pc = 0x108c0f4; continue;
 }
 0x108c0f4 => {
  { let v: u8 = cpu.r8(0x11eb7d4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x11eb7d5u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x11eb13cu32); cpu.ebp = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x108c10b; continue; }
  pc = 0x108c109; continue;
 }
 0x108c109 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108c10b => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c10d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c10d); }
 let mut pc: u32 = 0x108c10d;
 loop { match pc {
 0x108c10d => {
  { let v: u32 = 0x11757a4u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x116e734u32; cpu.edx = v; }
  pc = 0x108c11c; continue;
 }
 0x108c11c => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x108c153; continue; }
  pc = 0x108c122; continue;
 }
 0x108c122 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.edi)), 0xffu8); }
  if cpu.zf { pc = 0x108c153; continue; }
  pc = 0x108c128; continue;
 }
 0x108c128 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x108c146); f_0108c1c5(cpu); if cpu.returned_to != 0x108c146 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if !cpu.cf { pc = 0x108c150; continue; }
  pc = 0x108c14b; continue;
 }
 0x108c14b => {
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  pc = 0x108c150; continue;
 }
 0x108c150 => {
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x108c153; continue;
 }
 0x108c153 => {
  { let v: u32 = cpu.add32(cpu.edx, 0x16u32); cpu.edx = v; }
  { let v: u32 = 0x48u32.wrapping_add(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108c11c; continue; }
  pc = 0x108c15b; continue;
 }
 0x108c15b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c15c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c15c); }
 let mut pc: u32 = 0x108c15c;
 loop { match pc {
 0x108c15c => {
  { let v: u32 = 0x116e734u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x108c166; continue;
 }
 0x108c166 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)) & 0xfeu8; cpu.w8(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x108c166; continue; }
  pc = 0x108c16f; continue;
 }
 0x108c16f => {
  { let v: u8 = 0xffu8; cpu.w8(0x11eb3b0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11eb134u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c181(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c181); }
 let mut pc: u32 = 0x108c181;
 loop { match pc {
 0x108c181 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x108c1c4; continue; }
  pc = 0x108c187; continue;
 }
 0x108c187 => {
  { cpu.pushad(); }
  cpu.push(0x108c18d); f_0108a641(cpu); if cpu.returned_to != 0x108c18d { return; }
  if cpu.cf { pc = 0x108c1c3; continue; }
  pc = 0x108c18f; continue;
 }
 0x108c18f => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x10u8); }
  if !cpu.zf { pc = 0x108c1c3; continue; }
  pc = 0x108c195; continue;
 }
 0x108c195 => {
  { let v: u32 = 0x11eb3b0u32; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0xffu8); }
  if cpu.zf { pc = 0x108c1b1; continue; }
  pc = 0x108c19f; continue;
 }
 0x108c19f => {
  { let v: u32 = cpu.r32(0x11eb134u32); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x108c1b1; continue; }
  pc = 0x108c1a9; continue;
 }
 0x108c1a9 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x108c1c3; continue; }
  pc = 0x108c1b1; continue;
 }
 0x108c1b1 => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x11eb134u32)); cpu.edi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11eb134u32)); cpu.w32(0x11eb134u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x108c1c3; continue;
 }
 0x108c1c3 => {
  { cpu.popad(); }
  pc = 0x108c1c4; continue;
 }
 0x108c1c4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c1c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c1c5); }
 let mut pc: u32 = 0x108c1c5;
 loop { match pc {
 0x108c1c5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x116de88u32; cpu.esi = v; }
  { let v: u8 = 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x108c1d7; continue;
 }
 0x108c1d7 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if !cpu.zf { pc = 0x108c1ea; continue; }
  pc = 0x108c1de; continue;
 }
 0x108c1de => {
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108c1d7; continue; }
  pc = 0x108c1e5; continue;
 }
 0x108c1e5 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108c1ea => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.cf { cpu.poll(); pc = 0x108c1de; continue; }
  pc = 0x108c1ef; continue;
 }
 0x108c1ef => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4u32.wrapping_add(cpu.esi))); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x108c1de; continue; }
  pc = 0x108c1f5; continue;
 }
 0x108c1f5 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if cpu.cf { cpu.poll(); pc = 0x108c1de; continue; }
  pc = 0x108c1fb; continue;
 }
 0x108c1fb => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x6u32.wrapping_add(cpu.esi))); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x108c1de; continue; }
  pc = 0x108c201; continue;
 }
 0x108c201 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c206(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c206); }
 let mut pc: u32 = 0x108c206;
 loop { match pc {
 0x108c206 => {
  { let v: u32 = cpu.esp; cpu.w32(0x11eb138u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  cpu.push(0x108c21c); f_0104f36d(cpu); if cpu.returned_to != 0x108c21c { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x400u32; cpu.ebx = v; }
  cpu.push(0x108c227); f_0105dce8(cpu); if cpu.returned_to != 0x108c227 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x108c325; continue; }
  pc = 0x108c22e; continue;
 }
 0x108c22e => {
  { let v: u32 = cpu.r32(0x10e4024u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x1000u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.esi.wrapping_sub(0x10e6910u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x108c24b; continue;
 }
 0x108c24b => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x108c318; continue; }
  pc = 0x108c253; continue;
 }
 0x108c253 => {
  { cpu.sub32(cpu.ecx, 0x3e8u32); }
  if !cpu.cf { pc = 0x108c325; continue; }
  pc = 0x108c25f; continue;
 }
 0x108c25f => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0x100u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x108c325; continue; }
  pc = 0x108c26d; continue;
 }
 0x108c26d => {
  { cpu.sub32(cpu.esi, 0x3f00u32); }
  if !cpu.cf { pc = 0x108c325; continue; }
  pc = 0x108c279; continue;
 }
 0x108c279 => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x10ea910u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if !cpu.zf { pc = 0x108c294; continue; }
  pc = 0x108c289; continue;
 }
 0x108c289 => {
  cpu.push(0x108c28e); f_0108c332(cpu); if cpu.returned_to != 0x108c28e { return; }
  if cpu.cf { pc = 0x108c325; continue; }
  pc = 0x108c294; continue;
 }
 0x108c294 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x0u8); }
  if !cpu.zf { pc = 0x108c2b1; continue; }
  pc = 0x108c29b; continue;
 }
 0x108c29b => {
  { let v: u8 = cpu.r8(0x10ea911u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10f6391u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x108c2b1; continue; }
  pc = 0x108c2ac; continue;
 }
 0x108c2ac => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x108c2b1; continue;
 }
 0x108c2b1 => {
  { cpu.sub8(cpu.r8(0xffffff80u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x0u8); }
  if !cpu.zf { pc = 0x108c2d2; continue; }
  pc = 0x108c2b8; continue;
 }
 0x108c2b8 => {
  { let v: u8 = cpu.r8(0x10ea890u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10f6391u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x108c2d2; continue; }
  pc = 0x108c2c9; continue;
 }
 0x108c2c9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)).wrapping_sub(0x80u32); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x108c2d2; continue;
 }
 0x108c2d2 => {
  { cpu.sub8(cpu.r8(0x80u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x0u8); }
  if !cpu.zf { pc = 0x108c2f6; continue; }
  pc = 0x108c2dc; continue;
 }
 0x108c2dc => {
  { let v: u8 = cpu.r8(0x10ea990u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10f6391u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x108c2f6; continue; }
  pc = 0x108c2ed; continue;
 }
 0x108c2ed => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)).wrapping_add(0x80u32); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x108c2f6; continue;
 }
 0x108c2f6 => {
  { cpu.sub8(cpu.r8(0xffffffffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))), 0x0u8); }
  if !cpu.zf { pc = 0x108c313; continue; }
  pc = 0x108c2fd; continue;
 }
 0x108c2fd => {
  { let v: u8 = cpu.r8(0x10ea90fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10f6391u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x108c313; continue; }
  pc = 0x108c30e; continue;
 }
 0x108c30e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x108c313; continue;
 }
 0x108c313 => {
  cpu.poll(); pc = 0x108c24b; continue;
 }
 0x108c318 => {
  cpu.push(0x108c31d); f_0105dd62(cpu); if cpu.returned_to != 0x108c31d { return; }
  { let v: u32 = cpu.r32(0x11eb138u32); cpu.esp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108c325 => {
  cpu.push(0x108c32a); f_0105dd62(cpu); if cpu.returned_to != 0x108c32a { return; }
  { let v: u32 = cpu.r32(0x11eb138u32); cpu.esp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c332(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c332); }
 let mut pc: u32 = 0x108c332;
 loop { match pc {
 0x108c332 => {
  { cpu.pushad(); }
  { let v: u32 = 0x10ea910u32.wrapping_add(cpu.esi); cpu.eax = v; }
  cpu.push(0x108c33e); f_01008d26(cpu); if cpu.returned_to != 0x108c33e { return; }
  cpu.push(0x108c343); f_0108c1c5(cpu); if cpu.returned_to != 0x108c343 { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.ebx = v; }
  cpu.push(0x108c354); f_0108c1c5(cpu); if cpu.returned_to != 0x108c354 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  { cpu.popad(); }
  if cpu.zf { pc = 0x108c35c; continue; }
  pc = 0x108c35a; continue;
 }
 0x108c35a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108c35c => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c35e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c35e); }
 let mut pc: u32 = 0x108c35e;
 loop { match pc {
 0x108c35e => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x108c3a0; continue; }
  pc = 0x108c368; continue;
 }
 0x108c368 => {
  { let v: u32 = 0x16u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x108c3a0; continue; }
  pc = 0x108c376; continue;
 }
 0x108c376 => {
  { cpu.and8(cpu.r8(0x116e744u32.wrapping_add(cpu.eax)), 0x8u8); }
  if cpu.zf { pc = 0x108c3a0; continue; }
  pc = 0x108c37f; continue;
 }
 0x108c37f => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x108c38c); f_0108c1c5(cpu); if cpu.returned_to != 0x108c38c { return; }
  if cpu.cf { pc = 0x108c3a0; continue; }
  pc = 0x108c38e; continue;
 }
 0x108c38e => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x108c3a0; continue; }
  pc = 0x108c394; continue;
 }
 0x108c394 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x108c3a3; continue; }
  pc = 0x108c3a0; continue;
 }
 0x108c3a0 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108c3a3 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c3a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c3a6); }
 let mut pc: u32 = 0x108c3a6;
 loop { match pc {
 0x108c3a6 => {
  { cpu.sub8(cpu.r8(0x11eb7d8u32), 0x0u8); }
  if !cpu.zf { pc = 0x108c3b5; continue; }
  pc = 0x108c3af; continue;
 }
 0x108c3af => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x108c3c4; continue; }
  pc = 0x108c3b5; continue;
 }
 0x108c3b5 => {
  { let v: u8 = cpu.or8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x2u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x49u32; cpu.ebx = v; }
  cpu.push(0x108c3c3); f_0108c7d1(cpu); if cpu.returned_to != 0x108c3c3 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108c3c4 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x108c3cd); f_01087f41(cpu); if cpu.returned_to != 0x108c3cd { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0x1f4u32); }
  if cpu.cf { cpu.poll(); pc = 0x108c3b5; continue; }
  pc = 0x108c3d5; continue;
 }
 0x108c3d5 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x108c3e0); f_01017779(cpu); if cpu.returned_to != 0x108c3e0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x108c3b5; continue; }
  pc = 0x108c3e5; continue;
 }
 0x108c3e5 => {
  cpu.push(0x108c3ea); f_01017748(cpu); if cpu.returned_to != 0x108c3ea { return; }
  if cpu.cf { cpu.poll(); pc = 0x108c3b5; continue; }
  pc = 0x108c3ec; continue;
 }
 0x108c3ec => {
  { let v: u32 = 0x49u32; cpu.ebx = v; }
  cpu.push(0x108c3f6); f_0108c791(cpu); if cpu.returned_to != 0x108c3f6 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c3f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c3f7); }
 let mut pc: u32 = 0x108c3f7;
 loop { match pc {
 0x108c3f7 => {
  { cpu.sub8(cpu.r8(0x11eb7d8u32), 0x0u8); }
  if !cpu.zf { pc = 0x108c406; continue; }
  pc = 0x108c400; continue;
 }
 0x108c400 => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x108c415; continue; }
  pc = 0x108c406; continue;
 }
 0x108c406 => {
  { let v: u8 = cpu.or8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x2u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4au32; cpu.ebx = v; }
  cpu.push(0x108c414); f_0108c811(cpu); if cpu.returned_to != 0x108c414 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108c415 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x3e8u32); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x108c406; continue; }
  pc = 0x108c41e; continue;
 }
 0x108c41e => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x108c427); f_01087f41(cpu); if cpu.returned_to != 0x108c427 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0x1f4u32); }
  if cpu.cf { cpu.poll(); pc = 0x108c406; continue; }
  pc = 0x108c42f; continue;
 }
 0x108c42f => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x108c43a); f_01017779(cpu); if cpu.returned_to != 0x108c43a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x108c406; continue; }
  pc = 0x108c43f; continue;
 }
 0x108c43f => {
  cpu.push(0x108c444); f_01017748(cpu); if cpu.returned_to != 0x108c444 { return; }
  if cpu.cf { cpu.poll(); pc = 0x108c406; continue; }
  pc = 0x108c446; continue;
 }
 0x108c446 => {
  { let v: u32 = 0x4au32; cpu.ebx = v; }
  cpu.push(0x108c450); f_0108c791(cpu); if cpu.returned_to != 0x108c450 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c451(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c451); }
 let mut pc: u32 = 0x108c451;
 loop { match pc {
 0x108c451 => {
  { cpu.sub8(cpu.r8(0x11eb7d8u32), 0x0u8); }
  if !cpu.zf { pc = 0x108c460; continue; }
  pc = 0x108c45a; continue;
 }
 0x108c45a => {
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x108c46f; continue; }
  pc = 0x108c460; continue;
 }
 0x108c460 => {
  { let v: u8 = cpu.or8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x2u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  cpu.push(0x108c46e); f_0108c851(cpu); if cpu.returned_to != 0x108c46e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108c46f => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x3e8u32); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x108c460; continue; }
  pc = 0x108c478; continue;
 }
 0x108c478 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x108c481); f_01087f41(cpu); if cpu.returned_to != 0x108c481 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0x1f4u32); }
  if cpu.cf { cpu.poll(); pc = 0x108c460; continue; }
  pc = 0x108c489; continue;
 }
 0x108c489 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x108c494); f_01017779(cpu); if cpu.returned_to != 0x108c494 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x108c460; continue; }
  pc = 0x108c499; continue;
 }
 0x108c499 => {
  cpu.push(0x108c49e); f_01017748(cpu); if cpu.returned_to != 0x108c49e { return; }
  if cpu.cf { cpu.poll(); pc = 0x108c460; continue; }
  pc = 0x108c4a0; continue;
 }
 0x108c4a0 => {
  { let v: u32 = 0x4au32; cpu.ebx = v; }
  cpu.push(0x108c4aa); f_0108c791(cpu); if cpu.returned_to != 0x108c4aa { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c4ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c4ab); }
 let mut pc: u32 = 0x108c4ab;
 loop { match pc {
 0x108c4ab => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e4138u32, v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.edi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4144u32, v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.edi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x108c56d; continue; }
  pc = 0x108c4e2; continue;
 }
 0x108c4e2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x108c4ec; continue;
 }
 0x108c4ec => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10fd1a3u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x41u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10fd299u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x41u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1125244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1125246u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3d1bu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3d1du32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0xcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x108c540); f_0103dda6(cpu); if cpu.returned_to != 0x108c540 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x108c561; continue; }
  pc = 0x108c547; continue;
 }
 0x108c547 => {
  { let v: u32 = cpu.edi; cpu.w32(0x110eac4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11337cfu32, v); }
  cpu.push(0x108c559); f_0108c653(cpu); if cpu.returned_to != 0x108c559 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108c561 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x6u32); }
  if !cpu.zf { cpu.poll(); pc = 0x108c4ec; continue; }
  pc = 0x108c568; continue;
 }
 0x108c568 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x108c56d; continue;
 }
 0x108c56d => {
  { let v: u16 = 0xffffu16; cpu.w16(0x1133814u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x1133816u32, v); }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11e94b0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1133818u32, v); }
  { let v: u8 = 0x2du8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x108c595); f_0108c707(cpu); if cpu.returned_to != 0x108c595 { return; }
  if cpu.cf { pc = 0x108c5bb; continue; }
  pc = 0x108c597; continue;
 }
 0x108c597 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1133814u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1133816u32, v); }
  cpu.push(0x108c5a9); f_0105f215(cpu); if cpu.returned_to != 0x108c5a9 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1133818u32)); }
  if (cpu.sf == cpu.of) { pc = 0x108c5bb; continue; }
  pc = 0x108c5b6; continue;
 }
 0x108c5b6 => {
  pc = 0x108c646; continue;
 }
 0x108c5bb => {
  { let v: u8 = 0x35u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x108c5c2); f_0108c707(cpu); if cpu.returned_to != 0x108c5c2 { return; }
  if cpu.cf { pc = 0x108c5e5; continue; }
  pc = 0x108c5c4; continue;
 }
 0x108c5c4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1133814u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1133816u32, v); }
  cpu.push(0x108c5d6); f_0105f215(cpu); if cpu.returned_to != 0x108c5d6 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1133818u32)); }
  if (cpu.sf == cpu.of) { pc = 0x108c5e5; continue; }
  pc = 0x108c5e3; continue;
 }
 0x108c5e3 => {
  pc = 0x108c646; continue;
 }
 0x108c5e5 => {
  { let v: u8 = 0x36u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x108c5ec); f_0108c707(cpu); if cpu.returned_to != 0x108c5ec { return; }
  if cpu.cf { pc = 0x108c60f; continue; }
  pc = 0x108c5ee; continue;
 }
 0x108c5ee => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1133814u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1133816u32, v); }
  cpu.push(0x108c600); f_0105f215(cpu); if cpu.returned_to != 0x108c600 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1133818u32)); }
  if (cpu.sf == cpu.of) { pc = 0x108c60f; continue; }
  pc = 0x108c60d; continue;
 }
 0x108c60d => {
  pc = 0x108c646; continue;
 }
 0x108c60f => {
  { let v: u8 = 0x37u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x108c616); f_0108c707(cpu); if cpu.returned_to != 0x108c616 { return; }
  if cpu.cf { pc = 0x108c639; continue; }
  pc = 0x108c618; continue;
 }
 0x108c618 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1133814u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1133816u32, v); }
  cpu.push(0x108c62a); f_0105f215(cpu); if cpu.returned_to != 0x108c62a { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1133818u32)); }
  if (cpu.sf == cpu.of) { pc = 0x108c639; continue; }
  pc = 0x108c637; continue;
 }
 0x108c637 => {
  pc = 0x108c646; continue;
 }
 0x108c639 => {
  { cpu.sub16(cpu.r16(0x1133814u32), 0xffffu16); }
  if !cpu.zf { pc = 0x108c646; continue; }
  pc = 0x108c644; continue;
 }
 0x108c644 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108c646 => {
  { let v: u16 = cpu.r16(0x1133814u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1133816u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  f_0108c653(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c653(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c653); }
 let mut pc: u32 = 0x108c653;
 loop { match pc {
 0x108c653 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x10ef37du32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x52u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0xcu32); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x108c6a9; continue; }
  pc = 0x108c6a3; continue;
 }
 0x108c6a3 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  pc = 0x108c6a9; continue;
 }
 0x108c6a9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3f43u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3f45u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef20au32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ef37du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10eea7bu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef37bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11296e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef209u32, v); }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.push(0x108c6e6); f_0103fb02(cpu); if cpu.returned_to != 0x108c6e6 { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x112635cu32); cpu.ebp = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x81u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11296e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef37bu32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c707(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108c707); } r_0108c707(cpu, 0x108c707); }
pub fn f_0108c781(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108c781); } r_0108c707(cpu, 0x108c781); }
fn r_0108c707(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x108c707 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x11337d0u32, v); }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.edi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4140u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e4144u32, v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.edi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  pc = 0x108c733; continue;
 }
 0x108c733 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x108c73a); f_0104f36d(cpu); if cpu.returned_to != 0x108c73a { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x11337d0u32)); }
  if !cpu.zf { pc = 0x108c773; continue; }
  pc = 0x108c74d; continue;
 }
 0x108c74d => {
  { let v: u32 = cpu.edi; cpu.w32(0x110eac4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11337cfu32, v); }
  { cpu.pushad(); }
  cpu.push(0x108c760); f_0103db9f(cpu); if cpu.returned_to != 0x108c760 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x11337cfu32, v); }
  if cpu.cf { pc = 0x108c773; continue; }
  pc = 0x108c76a; continue;
 }
 0x108c76a => {
  { let v: u8 = cpu.and8(cpu.r8(0x4000u32.wrapping_add(cpu.esi)), 0x80u8); cpu.w8(0x4000u32.wrapping_add(cpu.esi), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108c773 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4140u32)); }
  if cpu.cf { cpu.poll(); pc = 0x108c733; continue; }
  pc = 0x108c77e; continue;
 }
 0x108c77e => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  pc = 0x108c781; continue;
 }
 0x108c781 => {
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x10e4144u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x108c733; continue; }
  pc = 0x108c78f; continue;
 }
 0x108c78f => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c791(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c791); }
 let mut pc: u32 = 0x108c791;
 loop { match pc {
 0x108c791 => {
  { cpu.pushad(); }
  cpu.push(0x108c797); f_01086c38(cpu); if cpu.returned_to != 0x108c797 { return; }
  cpu.push(0x108c79c); f_010864ac(cpu); if cpu.returned_to != 0x108c79c { return; }
  { cpu.popad(); }
  { let v: u32 = 0x11337eau32; cpu.esi = v; }
  cpu.push(0x108c7a7); f_0108c896(cpu); if cpu.returned_to != 0x108c7a7 { return; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10c6a60u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10c6a64u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x108c7cc; continue; }
  pc = 0x108c7c9; continue;
 }
 0x108c7c9 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x108c7cc; continue;
 }
 0x108c7cc => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c7d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c7d1); }
 let mut pc: u32 = 0x108c7d1;
 loop { match pc {
 0x108c7d1 => {
  { cpu.pushad(); }
  cpu.push(0x108c7d7); f_01086c38(cpu); if cpu.returned_to != 0x108c7d7 { return; }
  cpu.push(0x108c7dc); f_010864ac(cpu); if cpu.returned_to != 0x108c7dc { return; }
  { cpu.popad(); }
  { let v: u32 = 0x11337d8u32; cpu.esi = v; }
  cpu.push(0x108c7e7); f_0108c896(cpu); if cpu.returned_to != 0x108c7e7 { return; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10c6a60u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10c6a64u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x108c80c; continue; }
  pc = 0x108c809; continue;
 }
 0x108c809 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x108c80c; continue;
 }
 0x108c80c => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c811(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c811); }
 let mut pc: u32 = 0x108c811;
 loop { match pc {
 0x108c811 => {
  { cpu.pushad(); }
  cpu.push(0x108c817); f_01086c38(cpu); if cpu.returned_to != 0x108c817 { return; }
  cpu.push(0x108c81c); f_010864ac(cpu); if cpu.returned_to != 0x108c81c { return; }
  { cpu.popad(); }
  { let v: u32 = 0x11337d1u32; cpu.esi = v; }
  cpu.push(0x108c827); f_0108c896(cpu); if cpu.returned_to != 0x108c827 { return; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10c6a60u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10c6a64u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x108c84c; continue; }
  pc = 0x108c849; continue;
 }
 0x108c849 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x108c84c; continue;
 }
 0x108c84c => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c851(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c851); }
 let mut pc: u32 = 0x108c851;
 loop { match pc {
 0x108c851 => {
  { cpu.pushad(); }
  cpu.push(0x108c857); f_01086c38(cpu); if cpu.returned_to != 0x108c857 { return; }
  cpu.push(0x108c85c); f_010864ac(cpu); if cpu.returned_to != 0x108c85c { return; }
  { cpu.popad(); }
  { let v: u32 = 0x11337deu32; cpu.esi = v; }
  cpu.push(0x108c867); f_0108c896(cpu); if cpu.returned_to != 0x108c867 { return; }
  cpu.push(0x108c86c); f_0108c991(cpu); if cpu.returned_to != 0x108c86c { return; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10c6a60u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10c6a64u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x108c891; continue; }
  pc = 0x108c88e; continue;
 }
 0x108c88e => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x108c891; continue;
 }
 0x108c891 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))) as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c896(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108c896); } r_0108c896(cpu, 0x108c896); }
pub fn f_0108c908(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108c908); } r_0108c896(cpu, 0x108c908); }
fn r_0108c896(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x108c896 => {
  { let v: u32 = 0x11eb390u32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = 0x3e8u32; cpu.edx = v; }
  pc = 0x108c8aa; continue;
 }
 0x108c8aa => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x108c8af; continue;
 }
 0x108c8af => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = ((cpu.ecx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))) as u32; cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x108c91e; continue; }
  pc = 0x108c8cb; continue;
 }
 0x108c8cb => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x108c91e; continue; }
  pc = 0x108c8d5; continue;
 }
 0x108c8d5 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x108c91e; continue; }
  pc = 0x108c8da; continue;
 }
 0x108c8da => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x108c91e; continue; }
  pc = 0x108c8e2; continue;
 }
 0x108c8e2 => {
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x108c90e); f_0108c9f9(cpu); if cpu.returned_to != 0x108c90e { return; }
  if cpu.cf { pc = 0x108c91e; continue; }
  pc = 0x108c910; continue;
 }
 0x108c908 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  pc = 0x108c90c; continue;
 }
 0x108c90c => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if cpu.cf { pc = 0x108c91e; continue; }
  pc = 0x108c910; continue;
 }
 0x108c910 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x10e4138u32)); cpu.w8(0x10e4138u32, v); }
  cpu.push(0x108c91e); f_0108c940(cpu); if cpu.returned_to != 0x108c91e { return; }
  pc = 0x108c91e; continue;
 }
 0x108c91e => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108c8af; continue; }
  pc = 0x108c925; continue;
 }
 0x108c925 => {
  { let v: u32 = cpu.edx.wrapping_add(0x3e8u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x108c8aa; continue; }
  pc = 0x108c934; continue;
 }
 0x108c934 => {
  { let v: u32 = 0x11eb390u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c940(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c940); }
 let mut pc: u32 = 0x108c940;
 loop { match pc {
 0x108c940 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10e4138u32), 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x108c98f; continue; }
  pc = 0x108c94a; continue;
 }
 0x108c94a => {
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0xffffffffu32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x108c963); f_01087028(cpu); if cpu.returned_to != 0x108c963 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0xfffffffeu32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x108c980); f_01087028(cpu); if cpu.returned_to != 0x108c980 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x108c98f; continue; }
  pc = 0x108c986; continue;
 }
 0x108c986 => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0xfffffffeu32.wrapping_add(cpu.edi)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0xfffffffeu32.wrapping_add(cpu.edi), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  pc = 0x108c98f; continue;
 }
 0x108c98f => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c991(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c991); }
 let mut pc: u32 = 0x108c991;
 loop { match pc {
 0x108c991 => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x108c9f8; continue; }
  pc = 0x108c996; continue;
 }
 0x108c996 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = 0x1194208u32; cpu.edi = v; }
  pc = 0x108c99f; continue;
 }
 0x108c99f => {
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x108c9b9); f_01087028(cpu); if cpu.returned_to != 0x108c9b9 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108c99f; continue; }
  pc = 0x108c9c6; continue;
 }
 0x108c9c6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x108c9ce; continue;
 }
 0x108c9ce => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1194208u32; cpu.esi = v; }
  pc = 0x108c9d5; continue;
 }
 0x108c9d5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if !cpu.cf { pc = 0x108c9e8; continue; }
  pc = 0x108c9dc; continue;
 }
 0x108c9dc => {
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.esi)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x1u32.wrapping_add(cpu.edi)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x108c9e8; continue;
 }
 0x108c9e8 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108c9d5; continue; }
  pc = 0x108c9f0; continue;
 }
 0x108c9f0 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x108c9ce; continue; }
  pc = 0x108c9f6; continue;
 }
 0x108c9f6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x108c9f8; continue;
 }
 0x108c9f8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108c9f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108c9f9); }
 let mut pc: u32 = 0x108c9f9;
 loop { match pc {
 0x108c9f9 => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x108c9fa; continue;
 }
 0x108c9fa => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x108ca09; continue; }
  pc = 0x108c9ff; continue;
 }
 0x108c9ff => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x108ca06; continue; }
  pc = 0x108ca03; continue;
 }
 0x108ca03 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x108c9fa; continue;
 }
 0x108ca06 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108ca09 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ca0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ca0c); }
 let mut pc: u32 = 0x108ca0c;
 loop { match pc {
 0x108ca0c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x11eb7dcu32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x11ec3dcu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x108ca31; continue;
 }
 0x108ca31 => {
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x100u16); }
  if (cpu.sf != cpu.of) { pc = 0x108ca43; continue; }
  pc = 0x108ca39; continue;
 }
 0x108ca39 => {
  pc = 0x108ca81; continue;
 }
 0x108ca3b => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x108ca31; continue;
 }
 0x108ca43 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(0xcu32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x108ca3b; continue;
 }
 0x108ca81 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_0108ca8a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ca8a); }
 let mut pc: u32 = 0x108ca8a;
 loop { match pc {
 0x108ca8a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11eb7dcu32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x11ec3dcu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x108cab1; continue;
 }
 0x108cab1 => {
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x100u16); }
  if (cpu.sf != cpu.of) { pc = 0x108cac3; continue; }
  pc = 0x108cab9; continue;
 }
 0x108cab9 => {
  pc = 0x108cb22; continue;
 }
 0x108cabb => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x108cab1; continue;
 }
 0x108cac3 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 7u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 7u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 7u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(0xcu32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(0x3u32); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x108cabb; continue;
 }
 0x108cb22 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108cb2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108cb2a); }
 let mut pc: u32 = 0x108cb2a;
 loop { match pc {
 0x108cb2a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x2u16; cpu.w16(0x11ec3e0u32, v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ec3e2u32, v); }
  { let v: u32 = 0x11eb7dcu32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x2u16); }
  if cpu.cf { pc = 0x108cb7f; continue; }
  pc = 0x108cb64; continue;
 }
 0x108cb64 => {
  { cpu.sub16(cpu.r16(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x2u16); }
  if (cpu.cf || cpu.zf) { pc = 0x108cc46; continue; }
  pc = 0x108cb6f; continue;
 }
 0x108cb6f => {
  { cpu.sub16(cpu.r16(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x3u16); }
  if cpu.zf { pc = 0x108cd11; continue; }
  pc = 0x108cb7a; continue;
 }
 0x108cb7a => {
  pc = 0x108cdf0; continue;
 }
 0x108cb7f => {
  { cpu.sub16(cpu.r16(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x1u16); }
  if !cpu.zf { pc = 0x108cdf0; continue; }
  pc = 0x108cb8a; continue;
 }
 0x108cb8a => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ec3dcu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x108cb9a; continue;
 }
 0x108cb9a => {
  { cpu.sub16(cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x100u16); }
  if (cpu.sf != cpu.of) { pc = 0x108cbaf; continue; }
  pc = 0x108cba2; continue;
 }
 0x108cba2 => {
  pc = 0x108cc41; continue;
 }
 0x108cba7 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x108cb9a; continue;
 }
 0x108cbaf => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 7u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x2u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)).wrapping_add(0xcu32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)).wrapping_add(0x3u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x108cba7; continue;
 }
 0x108cc41 => {
  pc = 0x108cdf0; continue;
 }
 0x108cc46 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ec3dcu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x108cc56; continue;
 }
 0x108cc56 => {
  { cpu.sub16(cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x100u16); }
  if (cpu.sf != cpu.of) { pc = 0x108cc6b; continue; }
  pc = 0x108cc5e; continue;
 }
 0x108cc5e => {
  pc = 0x108cd0c; continue;
 }
 0x108cc63 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x108cc56; continue;
 }
 0x108cc6b => {
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x108cc63; continue;
 }
 0x108cd0c => {
  pc = 0x108cdf0; continue;
 }
 0x108cd11 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ec3dcu32, v); }
  { let v: u16 = cpu.neg16(cpu.r16(0x11ec3dcu32)); cpu.w16(0x11ec3dcu32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x11ec3deu32, v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x108cd4c; continue;
 }
 0x108cd4c => {
  { cpu.sub16(cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x100u16); }
  if (cpu.sf != cpu.of) { pc = 0x108cd61; continue; }
  pc = 0x108cd54; continue;
 }
 0x108cd54 => {
  pc = 0x108cdf0; continue;
 }
 0x108cd59 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x108cd4c; continue;
 }
 0x108cd61 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)).wrapping_add(0xcu32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x2u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.edx, 7u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x4u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edx), v); }
  cpu.poll(); pc = 0x108cd59; continue;
 }
 0x108cdf0 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108cdf7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108cdf7); }
 let mut pc: u32 = 0x108cdf7;
 loop { match pc {
 0x108cdf7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0x11ec3dcu32), 0x0u16); }
  if !cpu.zf { pc = 0x108ce21; continue; }
  pc = 0x108ce16; continue;
 }
 0x108ce16 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x108cf0f; continue;
 }
 0x108ce21 => {
  { let v: u32 = 0x11eb7dcu32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0x11ec3e0u32), 0x0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x108ce45; continue; }
  pc = 0x108ce32; continue;
 }
 0x108ce32 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x11ec3e0u32)); cpu.w16(0x11ec3e0u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x108cf09; continue;
 }
 0x108ce45 => {
  { cpu.sub16(cpu.r16(0x11ec3dcu32), 0x0u16); }
  if cpu.zf { pc = 0x108cf09; continue; }
  pc = 0x108ce53; continue;
 }
 0x108ce53 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0x11ec3dcu32), 0x0u16); }
  if (cpu.sf == cpu.of) { pc = 0x108ceb7; continue; }
  pc = 0x108ce64; continue;
 }
 0x108ce64 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x11ec3dcu32)); cpu.w16(0x11ec3dcu32, v); }
  { cpu.sub16(cpu.r16(0x11ec3dcu32), 0x0u16); }
  if !cpu.zf { pc = 0x108ceb7; continue; }
  pc = 0x108ce75; continue;
 }
 0x108ce75 => {
  { let v: u16 = cpu.neg16(cpu.r16(0x11ec3e0u32)); cpu.w16(0x11ec3e0u32, v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11ec3e2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11ec3dcu32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x108ce94); f_0108cb2a(cpu); if cpu.returned_to != 0x108ce94 { return; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x108cf0f; continue;
 }
 0x108ceb7 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x11ec3dcu32)); cpu.w16(0x11ec3dcu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x108cec5; continue;
 }
 0x108cec5 => {
  { cpu.sub16(cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x100u16); }
  if (cpu.sf != cpu.of) { pc = 0x108ced7; continue; }
  pc = 0x108cecd; continue;
 }
 0x108cecd => {
  pc = 0x108cf09; continue;
 }
 0x108cecf => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x108cec5; continue;
 }
 0x108ced7 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u16)); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u16)); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)).wrapping_add(0xcu32); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x108cecf; continue;
 }
 0x108cf09 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x108cf0f; continue;
 }
 0x108cf0f => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_0108cf1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108cf1c); }
 let mut pc: u32 = 0x108cf1c;
 loop { match pc {
 0x108cf1c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108cf1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108cf1d); }
 let mut pc: u32 = 0x108cf1d;
 loop { match pc {
 0x108cf1d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v = cpu.r16(0x1133e5cu32); cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x81u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108cf6b; continue; }
  pc = 0x108cf42; continue;
 }
 0x108cf42 => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if (cpu.sf == cpu.of) { pc = 0x108cf4e; continue; }
  pc = 0x108cf47; continue;
 }
 0x108cf47 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  pc = 0x108cf6b; continue;
 }
 0x108cf4e => {
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x108cf55; continue; }
  pc = 0x108cf52; continue;
 }
 0x108cf52 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.ebx = v; }
  pc = 0x108cf55; continue;
 }
 0x108cf55 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x108cf64; continue; }
  pc = 0x108cf5b; continue;
 }
 0x108cf5b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0])); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x108cf55; continue;
 }
 0x108cf64 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.esi); cpu.ebx = v; }
  pc = 0x108cf6b; continue;
 }
 0x108cf6b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108cf72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108cf72); }
 let mut pc: u32 = 0x108cf72;
 loop { match pc {
 0x108cf72 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x7fffffffu32; cpu.edx = v; }
  cpu.push(0x108cf80); f_0108cf1d(cpu); if cpu.returned_to != 0x108cf80 { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108cf85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108cf85); }
 let mut pc: u32 = 0x108cf85;
 loop { match pc {
 0x108cf85 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1133c52u32; cpu.eax = v; }
  cpu.push(0x108cfa1); f_01096bcc(cpu); if cpu.returned_to != 0x108cfa1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108cfa7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108cfa7); }
 let mut pc: u32 = 0x108cfa7;
 loop { match pc {
 0x108cfa7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1133e90u32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x108cfff; continue; }
  pc = 0x108cfb9; continue;
 }
 0x108cfb9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108cfff; continue; }
  pc = 0x108cfbd; continue;
 }
 0x108cfbd => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  pc = 0x108cff9; continue;
 }
 0x108cfd3 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x108cfde); f_0108f6fc(cpu); if cpu.returned_to != 0x108cfde { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108cff6; continue; }
  pc = 0x108cfe2; continue;
 }
 0x108cfe2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3du32); }
  if !cpu.zf { pc = 0x108cff6; continue; }
  pc = 0x108cfef; continue;
 }
 0x108cfef => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  pc = 0x108d001; continue;
 }
 0x108cff6 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x108cff9; continue;
 }
 0x108cff9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x108cfd3; continue; }
  pc = 0x108cfff; continue;
 }
 0x108cfff => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x108d001; continue;
 }
 0x108d001 => {
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
pub fn f_0108d008(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108d008); }
 let mut pc: u32 = 0x108d008;
 loop { match pc {
 0x108d008 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x108d00f; continue;
 }
 0x108d00f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x108d027; continue; }
  pc = 0x108d015; continue;
 }
 0x108d015 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x61u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x19u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x108d024; continue; }
  pc = 0x108d020; continue;
 }
 0x108d020 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x41u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x108d024; continue;
 }
 0x108d024 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x108d00f; continue;
 }
 0x108d027 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108d02d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108d02d); }
 let mut pc: u32 = 0x108d02d;
 loop { match pc {
 0x108d02d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x108d045; continue; }
  pc = 0x108d03e; continue;
 }
 0x108d03e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x108d0e2; continue;
 }
 0x108d045 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x108d067; continue; }
  pc = 0x108d04b; continue;
 }
 0x108d04b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x108d04d; continue;
 }
 0x108d04d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x108d065; continue; }
  pc = 0x108d053; continue;
 }
 0x108d053 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108d063; continue; }
  pc = 0x108d057; continue;
 }
 0x108d057 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x108d065; continue; }
  pc = 0x108d05e; continue;
 }
 0x108d05e => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108d04d; continue; }
  pc = 0x108d063; continue;
 }
 0x108d063 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x108d065; continue;
 }
 0x108d065 => {
  cpu.poll(); pc = 0x108d03e; continue;
 }
 0x108d067 => {
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[0] as u32); }
  if cpu.ecx == 0 { pc = 0x108d07e; continue; }
  pc = 0x108d073; continue;
 }
 0x108d073 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { cpu.scas(1, 2); }
  if !cpu.zf { pc = 0x108d07e; continue; }
  pc = 0x108d07b; continue;
 }
 0x108d07b => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.and16((cpu.eax as u16), 0xcf89u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x108d098; continue;
 }
 0x108d07e => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x108d098; continue;
 }
 0x108d098 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if cpu.cf { pc = 0x108d0e0; continue; }
  pc = 0x108d0a1; continue;
 }
 0x108d0a1 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[0] as u32); }
  if cpu.ecx == 0 { pc = 0x108d0b3; continue; }
  pc = 0x108d0a8; continue;
 }
 0x108d0a8 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { cpu.scas(1, 2); }
  if !cpu.zf { pc = 0x108d0b3; continue; }
  pc = 0x108d0b0; continue;
 }
 0x108d0b0 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.and16((cpu.eax as u16), 0xcf89u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x108d0e0; continue; }
  pc = 0x108d0bc; continue;
 }
 0x108d0b3 => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x108d0e0; continue; }
  pc = 0x108d0bc; continue;
 }
 0x108d0bc => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.cmps(1, 1); }
  if cpu.zf { pc = 0x108d0d2; continue; }
  pc = 0x108d0cd; continue;
 }
 0x108d0cd => {
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x108d0d2; continue;
 }
 0x108d0d2 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108d0db; continue; }
  pc = 0x108d0d7; continue;
 }
 0x108d0d7 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x108d0e2; continue;
 }
 0x108d0db => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.esi = v; }
  cpu.poll(); pc = 0x108d098; continue;
 }
 0x108d0e0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108d0e2; continue;
 }
 0x108d0e2 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108d0f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108d0f0); }
 let mut pc: u32 = 0x108d0f0;
 loop { match pc {
 0x108d0f0 => {
  { let v = 0x1137474u32; cpu.push(v); }
  cpu.push(0x108d0fa); f_010971b0(cpu); if cpu.returned_to != 0x108d0fa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x11374c0u32; cpu.push(v); }
  cpu.push(0x108d107); f_010971b0(cpu); if cpu.returned_to != 0x108d107 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x113750cu32; cpu.push(v); }
  cpu.push(0x108d114); f_010971b0(cpu); if cpu.returned_to != 0x108d114 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x1137558u32; cpu.push(v); }
  cpu.push(0x108d121); f_010971b0(cpu); if cpu.returned_to != 0x108d121 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x1137580u32; cpu.push(v); }
  cpu.push(0x108d12e); f_010971b0(cpu); if cpu.returned_to != 0x108d12e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x11375ccu32; cpu.push(v); }
  cpu.push(0x108d13b); f_010971b0(cpu); if cpu.returned_to != 0x108d13b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x113761cu32; cpu.push(v); }
  cpu.push(0x108d148); f_010971b0(cpu); if cpu.returned_to != 0x108d148 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x1137668u32; cpu.push(v); }
  cpu.push(0x108d155); f_010971b0(cpu); if cpu.returned_to != 0x108d155 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x11376b4u32; cpu.push(v); }
  cpu.push(0x108d162); f_010971b0(cpu); if cpu.returned_to != 0x108d162 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  f_0108dd4b(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108d170(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108d170); }
 let mut pc: u32 = 0x108d170;
 loop { match pc {
 0x108d170 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x108d17c); f_0108f186(cpu); if cpu.returned_to != 0x108d17c { return; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x108d18c); f_0108f08a(cpu); if cpu.returned_to != 0x108d18c { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x108d193); f_0108f186(cpu); if cpu.returned_to != 0x108d193 { return; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x108d1a0); f_0108f08a(cpu); if cpu.returned_to != 0x108d1a0 { return; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
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
pub fn f_0108d1a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108d1a8); }
 let mut pc: u32 = 0x108d1a8;
 loop { match pc {
 0x108d1a8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x11ec43cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133b98u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1133b94u32); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x11ec438u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.esi), v); }
  cpu.push(0x108d1dd); f_010982ad(cpu); if cpu.returned_to != 0x108d1dd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x108d1ec); f_01098a48(cpu); if cpu.returned_to != 0x108d1ec { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133b98u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133b94u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x108d204); f_01098282(cpu); if cpu.returned_to != 0x108d204 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108d210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108d210); }
 let mut pc: u32 = 0x108d210;
 loop { match pc {
 0x108d210 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x268u32); cpu.esp = v; }
  { let v: u32 = 0x4f00u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x250u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1137700u32; cpu.edx = v; }
  cpu.push(0x108d238); f_01098c3e(cpu); if cpu.returned_to != 0x108d238 { return; }
  { let v: u32 = 0x200u32; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = 0x250u32.wrapping_add(cpu.esp); cpu.eax = v; }
  cpu.push(0x108d24b); f_0108d1a8(cpu); if cpu.returned_to != 0x108d24b { return; }
  { cpu.sub16(cpu.r16(0x250u32.wrapping_add(cpu.esp)), 0x4fu16); }
  if cpu.zf { pc = 0x108d25d; continue; }
  pc = 0x108d256; continue;
 }
 0x108d256 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108d38c; continue;
 }
 0x108d25d => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x1137708u32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x108d26e); f_0108f5f5(cpu); if cpu.returned_to != 0x108d26e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108d27f; continue; }
  pc = 0x108d272; continue;
 }
 0x108d272 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x268u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108d27f => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x268u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x270u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108d2a6); f_01098888(cpu); if cpu.returned_to != 0x108d2a6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x50u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x264u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x26cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x20cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x12u32; cpu.ebx = v; }
  cpu.push(0x108d2cd); f_01098282(cpu); if cpu.returned_to != 0x108d2cd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1137710u32; cpu.edx = v; }
  { let v: u32 = 0x200u32.wrapping_add(cpu.esp); cpu.eax = v; }
  cpu.push(0x108d2e1); f_0108f5f5(cpu); if cpu.returned_to != 0x108d2e1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108d2f2; continue; }
  pc = 0x108d2e5; continue;
 }
 0x108d2e5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x268u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108d2f2 => {
  { let v: u32 = 0x250u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x4f0fu32; cpu.ecx = v; }
  { let v: u32 = 0x1234u32; cpu.esi = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x25cu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x25eu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x108d31c); f_0109895c(cpu); if cpu.returned_to != 0x108d31c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x250u32.wrapping_add(cpu.esp)), 0x4fu16); }
  if cpu.zf { pc = 0x108d337; continue; }
  pc = 0x108d32a; continue;
 }
 0x108d32a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x268u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108d337 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x252u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x25au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x268u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x270u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108d362); f_01098888(cpu); if cpu.returned_to != 0x108d362 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x124u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x264u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x26cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x11ec4acu32; cpu.push(v); }
  cpu.push(0x108d384); f_01098282(cpu); if cpu.returned_to != 0x108d384 { return; }
  { let v: u32 = 0x11ec4acu32; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x108d38c; continue;
 }
 0x108d38c => {
  { let v: u32 = cpu.add32(cpu.esp, 0x268u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108d398(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108d398); }
 let mut pc: u32 = 0x108d398;
 loop { match pc {
 0x108d398 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x218u32); cpu.esp = v; }
  { let v: u32 = 0x4f00u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x200u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1137700u32; cpu.edx = v; }
  cpu.push(0x108d3bf); f_01098c3e(cpu); if cpu.returned_to != 0x108d3bf { return; }
  { let v: u32 = 0x200u32; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = 0x200u32.wrapping_add(cpu.esp); cpu.eax = v; }
  cpu.push(0x108d3d2); f_0108d1a8(cpu); if cpu.returned_to != 0x108d3d2 { return; }
  { cpu.sub16(cpu.r16(0x200u32.wrapping_add(cpu.esp)), 0x4fu16); }
  if cpu.zf { pc = 0x108d3e4; continue; }
  pc = 0x108d3dd; continue;
 }
 0x108d3dd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108d465; continue;
 }
 0x108d3e4 => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x1137708u32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x108d3f5); f_0108f5f5(cpu); if cpu.returned_to != 0x108d3f5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108d405; continue; }
  pc = 0x108d3f9; continue;
 }
 0x108d3f9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x218u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108d405 => {
  { cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.esp)), 0x200u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x108d45e; continue; }
  pc = 0x108d40e; continue;
 }
 0x108d40e => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x218u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x220u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108d435); f_01098888(cpu); if cpu.returned_to != 0x108d435 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x210u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x218u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x108d44d); f_01098233(cpu); if cpu.returned_to != 0x108d44d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x54524f56u32); }
  if !cpu.zf { pc = 0x108d45e; continue; }
  pc = 0x108d457; continue;
 }
 0x108d457 => {
  { let v: u16 = 0x102u16; cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x108d45e; continue;
 }
 0x108d45e => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  pc = 0x108d465; continue;
 }
 0x108d465 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x218u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108d470(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108d470); }
 let mut pc: u32 = 0x108d470;
 loop { match pc {
 0x108d470 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x108d479); f_010982f8(cpu); if cpu.returned_to != 0x108d479 { return; }
  { let v = 0x6cu32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108d488); f_01098233(cpu); if cpu.returned_to != 0x108d488 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1137724u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x108d497); f_010971b0(cpu); if cpu.returned_to != 0x108d497 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x108d49f); f_0108dca0(cpu); if cpu.returned_to != 0x108d49f { return; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108d4a5); f_0108dc98(cpu); if cpu.returned_to != 0x108d4a5 { return; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108d4ab); f_0108dc90(cpu); if cpu.returned_to != 0x108d4ab { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1137768u32; cpu.push(v); }
  cpu.push(0x108d4b6); f_010971b0(cpu); if cpu.returned_to != 0x108d4b6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  cpu.push(0x108d4be); f_0108dc88(cpu); if cpu.returned_to != 0x108d4be { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1137794u32; cpu.push(v); }
  cpu.push(0x108d4c9); f_010971b0(cpu); if cpu.returned_to != 0x108d4c9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1137798u32; cpu.push(v); }
  cpu.push(0x108d4d6); f_010971b0(cpu); if cpu.returned_to != 0x108d4d6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x108d4de); f_0108dd34(cpu); if cpu.returned_to != 0x108d4de { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x113779cu32; cpu.push(v); }
  cpu.push(0x108d4e9); f_010971b0(cpu); if cpu.returned_to != 0x108d4e9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  cpu.push(0x108d4f1); f_0108dd3c(cpu); if cpu.returned_to != 0x108d4f1 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x11377acu32; cpu.push(v); }
  cpu.push(0x108d4fc); f_010971b0(cpu); if cpu.returned_to != 0x108d4fc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  cpu.push(0x108d504); f_0108dd44(cpu); if cpu.returned_to != 0x108d504 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x11377bcu32; cpu.push(v); }
  cpu.push(0x108d50f); f_010971b0(cpu); if cpu.returned_to != 0x108d50f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x108d512; continue;
 }
 0x108d512 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = 0x6cu32; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108d51f); f_01098233(cpu); if cpu.returned_to != 0x108d51f { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x23u32); }
  if cpu.cf { cpu.poll(); pc = 0x108d512; continue; }
  pc = 0x108d529; continue;
 }
 0x108d529 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108d530(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108d530); }
 let mut pc: u32 = 0x108d530;
 loop { match pc {
 0x108d530 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x120u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x11377ccu32; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108d54c); f_0108dece(cpu); if cpu.returned_to != 0x108d54c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x108d57c; continue; }
  pc = 0x108d553; continue;
 }
 0x108d553 => {
  { let v: u32 = 0x11377d8u32; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x108d563; continue;
 }
 0x108d563 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108d57b; continue; }
  pc = 0x108d56b; continue;
 }
 0x108d56b => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108d563; continue; }
  pc = 0x108d57b; continue;
 }
 0x108d57b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x108d57c; continue;
 }
 0x108d57c => {
  { let v: u32 = 0x90u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x108d586; continue;
 }
 0x108d586 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108d59e; continue; }
  pc = 0x108d58e; continue;
 }
 0x108d58e => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108d586; continue; }
  pc = 0x108d59e; continue;
 }
 0x108d59e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11377e4u32; cpu.esi = v; }
  { let v: u32 = 0x90u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x108d5b4; continue;
 }
 0x108d5b4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108d5cc; continue; }
  pc = 0x108d5bc; continue;
 }
 0x108d5bc => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108d5b4; continue; }
  pc = 0x108d5cc; continue;
 }
 0x108d5cc => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x98u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x108d5e4); f_0108eb78(cpu); if cpu.returned_to != 0x108d5e4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108d5f2; continue; }
  pc = 0x108d5eb; continue;
 }
 0x108d5eb => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x108d623; continue;
 }
 0x108d5f2 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x11377e4u32; cpu.push(v); }
  { let v = 0x11377e4u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x108d60a); f_0108eb78(cpu); if cpu.returned_to != 0x108d60a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108d621; continue; }
  pc = 0x108d611; continue;
 }
 0x108d611 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x120u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108d621 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108d623; continue;
 }
 0x108d623 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x120u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0108d630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108d630); }
 let mut pc: u32 = 0x108d630;
 loop { match pc {
 0x108d630 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14cu32); cpu.esp = v; }
  { let v = 0x11ec438u32; cpu.push(v); }
  { let v = 0x11ec43cu32; cpu.push(v); }
  { let v = 0x1133b98u32; cpu.push(v); }
  { let v = 0x1133b94u32; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = 0x200u32; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x158u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x108d661); f_010988c4(cpu); if cpu.returned_to != 0x108d661 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108d672; continue; }
  pc = 0x108d668; continue;
 }
 0x108d668 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  pc = 0x108dbea; continue;
 }
 0x108d672 => {
  cpu.push(0x108d677); f_0108d398(cpu); if cpu.returned_to != 0x108d677 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x13cu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x108d685); f_0108d210(cpu); if cpu.returned_to != 0x108d685 { return; }
  { let v: u32 = cpu.r32(0x1133b98u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133b94u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x140u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x108d69f); f_01098934(cpu); if cpu.returned_to != 0x108d69f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x164u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x108d6c4; continue; }
  pc = 0x108d6ac; continue;
 }
 0x108d6ac => {
  { cpu.sub32(cpu.ebx, 0x200u32); }
  if (cpu.sf != cpu.of) { pc = 0x108d6c4; continue; }
  pc = 0x108d6b4; continue;
 }
 0x108d6b4 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108d6c4 => {
  { cpu.sub32(cpu.r32(0x144u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x108d6da; continue; }
  pc = 0x108d6ce; continue;
 }
 0x108d6ce => {
  { let v: u32 = cpu.r32(0x140u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x108d6da); f_01094b7d(cpu); if cpu.returned_to != 0x108d6da { return; }
  pc = 0x108d6da; continue;
 }
 0x108d6da => {
  { let v: u32 = cpu.r32(0x160u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x90u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x108d6e9; continue;
 }
 0x108d6e9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108d701; continue; }
  pc = 0x108d6f1; continue;
 }
 0x108d6f1 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108d6e9; continue; }
  pc = 0x108d701; continue;
 }
 0x108d701 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x90u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x108d728; continue; }
  pc = 0x108d717; continue;
 }
 0x108d717 => {
  { let v: u32 = 0x90u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0x11377f4u32; cpu.esi = v; }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  pc = 0x108d743; continue;
 }
 0x108d728 => {
  { cpu.sub8(cpu.r8(0x8fu32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))), 0x5cu8); }
  if cpu.zf { pc = 0x108d743; continue; }
  pc = 0x108d732; continue;
 }
 0x108d732 => {
  { let v: u8 = 0x5cu8; cpu.w8(0x90u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x91u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x108d743; continue;
 }
 0x108d743 => {
  { let v: u32 = 0x90u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x108d74d; continue;
 }
 0x108d74d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108d765; continue; }
  pc = 0x108d755; continue;
 }
 0x108d755 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108d74d; continue; }
  pc = 0x108d765; continue;
 }
 0x108d765 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11377f8u32; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = 0x1137804u32; cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x108d77b; continue;
 }
 0x108d77b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108d793; continue; }
  pc = 0x108d783; continue;
 }
 0x108d783 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108d77b; continue; }
  pc = 0x108d793; continue;
 }
 0x108d793 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x108d79b); f_0108e8a5(cpu); if cpu.returned_to != 0x108d79b { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108d7ea; continue; }
  pc = 0x108d7a1; continue;
 }
 0x108d7a1 => {
  { let v: u32 = cpu.r32(0x168u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x90u32.wrapping_add(cpu.esp); cpu.eax = v; }
  cpu.push(0x108d7b4); f_0108d530(cpu); if cpu.returned_to != 0x108d7b4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108d7c8; continue; }
  pc = 0x108d7b8; continue;
 }
 0x108d7b8 => {
  { let v: u32 = 0xau32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108d7c8 => {
  { let v: u32 = 0x1137804u32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x108d7d4); f_0108e8a5(cpu); if cpu.returned_to != 0x108d7d4 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108d7ea; continue; }
  pc = 0x108d7da; continue;
 }
 0x108d7da => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108d7ea => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x108d7f1); f_0108d170(cpu); if cpu.returned_to != 0x108d7f1 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x148u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x108d7ff); f_01096ee0(cpu); if cpu.returned_to != 0x108d7ff { return; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  cpu.push(0x108d807); f_01094a9a(cpu); if cpu.returned_to != 0x108d807 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x140u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108d824; continue; }
  pc = 0x108d814; continue;
 }
 0x108d814 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108d824 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x108d832); f_0108f30b(cpu); if cpu.returned_to != 0x108d832 { return; }
  { let v: u32 = 0x12u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x1137710u32; cpu.edx = v; }
  cpu.push(0x108d843); f_0108f1c8(cpu); if cpu.returned_to != 0x108d843 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x108d84a); f_0108f5f5(cpu); if cpu.returned_to != 0x108d84a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108d858; continue; }
  pc = 0x108d84e; continue;
 }
 0x108d84e => {
  { let v: u32 = 0x6u32; cpu.edx = v; }
  pc = 0x108dbdc; continue;
 }
 0x108d858 => {
  cpu.push(0x108d85d); f_0108dc98(cpu); if cpu.returned_to != 0x108d85d { return; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108d863); f_0108dc90(cpu); if cpu.returned_to != 0x108d863 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1137808u32; cpu.push(v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108d873); f_0108dece(cpu); if cpu.returned_to != 0x108d873 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  cpu.push(0x108d87b); f_0108dc90(cpu); if cpu.returned_to != 0x108d87b { return; }
  { let v: u8 = cpu.r8(0x25cu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); }
  if cpu.cf { pc = 0x108d8a3; continue; }
  pc = 0x108d885; continue;
 }
 0x108d885 => {
  cpu.push(0x108d88a); f_0108dc90(cpu); if cpu.returned_to != 0x108d88a { return; }
  { let v: u8 = cpu.r8(0x25cu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x108d8ad; continue; }
  pc = 0x108d894; continue;
 }
 0x108d894 => {
  cpu.push(0x108d899); f_0108dc98(cpu); if cpu.returned_to != 0x108d899 { return; }
  { let v: u8 = cpu.r8(0x25du32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); }
  if !cpu.cf { pc = 0x108d8ad; continue; }
  pc = 0x108d8a3; continue;
 }
 0x108d8a3 => {
  { let v: u32 = 0x7u32; cpu.edx = v; }
  pc = 0x108dbdc; continue;
 }
 0x108d8ad => {
  { cpu.sub16(cpu.r16(0x95u32.wrapping_add(cpu.ebp)), 0x26u16); }
  if !cpu.zf { pc = 0x108d8c1; continue; }
  pc = 0x108d8b7; continue;
 }
 0x108d8b7 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x108dbdc; continue;
 }
 0x108d8c1 => {
  { let v: u32 = cpu.r32(0x138u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108d914; continue; }
  pc = 0x108d8cc; continue;
 }
 0x108d8cc => {
  { let v: u8 = cpu.r8(0x107u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x25cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.ecx >> 8) as u8)); }
  if (cpu.cf || cpu.zf) { pc = 0x108d8e6; continue; }
  pc = 0x108d8dc; continue;
 }
 0x108d8dc => {
  { let v: u32 = 0x3u32; cpu.edx = v; }
  pc = 0x108dbdc; continue;
 }
 0x108d8e6 => {
  { let v: u32 = cpu.r32(0x138u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), cpu.r8(0x107u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x108d914; continue; }
  pc = 0x108d8f5; continue;
 }
 0x108d8f5 => {
  { let v: u32 = cpu.r32(0x138u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x25du32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x108u32.wrapping_add(cpu.eax))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x108d914; continue; }
  pc = 0x108d90a; continue;
 }
 0x108d90a => {
  { let v: u32 = 0x3u32; cpu.edx = v; }
  pc = 0x108dbdc; continue;
 }
 0x108d914 => {
  { let v = 0x11ec440u32; cpu.push(v); }
  { let v = 0x11ec444u32; cpu.push(v); }
  { let v = 0x1133b90u32; cpu.push(v); }
  { let v = 0x1133b8cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x158u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x108d935); f_010988c4(cpu); if cpu.returned_to != 0x108d935 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108d946; continue; }
  pc = 0x108d93c; continue;
 }
 0x108d93c => {
  { let v: u32 = 0x9u32; cpu.edx = v; }
  pc = 0x108dbdc; continue;
 }
 0x108d946 => {
  { let v = 0x1133b9cu32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x108d952); f_010983e4(cpu); if cpu.returned_to != 0x108d952 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1133ba0u32; cpu.push(v); }
  { let v = 0x2fu32; cpu.push(v); }
  cpu.push(0x108d961); f_010983e4(cpu); if cpu.returned_to != 0x108d961 { return; }
  { let v: u32 = cpu.r32(0x1133b9cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1133ba0u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x99u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x6bu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 6u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x81u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x148u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x11ec444u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x91u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133b90u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133b8cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x108d9ba); f_010982ad(cpu); if cpu.returned_to != 0x108d9ba { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x130u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x124u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5fu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec444u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x108d9e0); f_010986c0(cpu); if cpu.returned_to != 0x108d9e0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x120u32.wrapping_add(cpu.esp)), 0x1u16); }
  if cpu.zf { pc = 0x108dad3; continue; }
  pc = 0x108d9f2; continue;
 }
 0x108d9f2 => {
  { let v: u32 = cpu.r32(0x1133b90u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133b8cu32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x108da05); f_01098934(cpu); if cpu.returned_to != 0x108da05 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x144u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x108da1c; continue; }
  pc = 0x108da12; continue;
 }
 0x108da12 => {
  { let v: u32 = 0xbu32; cpu.edx = v; }
  pc = 0x108dbdc; continue;
 }
 0x108da1c => {
  { let v: u32 = 0x90u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x108da26; continue;
 }
 0x108da26 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108da3e; continue; }
  pc = 0x108da2e; continue;
 }
 0x108da2e => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108da26; continue; }
  pc = 0x108da3e; continue;
 }
 0x108da3e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x1137810u32; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x108da4f; continue;
 }
 0x108da4f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108da67; continue; }
  pc = 0x108da57; continue;
 }
 0x108da57 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108da4f; continue; }
  pc = 0x108da67; continue;
 }
 0x108da67 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = 0x90u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  cpu.push(0x108da78); f_01098c63(cpu); if cpu.returned_to != 0x108da78 { return; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x108da79; continue;
 }
 0x108da79 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108da91; continue; }
  pc = 0x108da81; continue;
 }
 0x108da81 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108da79; continue; }
  pc = 0x108da91; continue;
 }
 0x108da91 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11377f8u32; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x108daa7; continue;
 }
 0x108daa7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108dabf; continue; }
  pc = 0x108daaf; continue;
 }
 0x108daaf => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108daa7; continue; }
  pc = 0x108dabf; continue;
 }
 0x108dabf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x144u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x108dace); f_01098c63(cpu); if cpu.returned_to != 0x108dace { return; }
  cpu.poll(); pc = 0x108d6c4; continue;
 }
 0x108dad3 => {
  { cpu.sub16(cpu.r16(0x93u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x108daec; continue; }
  pc = 0x108dadd; continue;
 }
 0x108dadd => {
  { cpu.sub32(cpu.r32(0x13cu32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x108daec; continue; }
  pc = 0x108dae7; continue;
 }
 0x108dae7 => {
  cpu.push(0x108daec); f_0108d0f0(cpu); if cpu.returned_to != 0x108daec { return; }
  pc = 0x108daec; continue;
 }
 0x108daec => {
  { let v: u32 = 0x11ec3e8u32; cpu.edi = v; }
  { let v: u32 = 0x155u32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x108daf8; continue;
 }
 0x108daf8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108db10; continue; }
  pc = 0x108db00; continue;
 }
 0x108db00 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108daf8; continue; }
  pc = 0x108db10; continue;
 }
 0x108db10 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11ec448u32; cpu.edi = v; }
  { let v: u32 = 0x1a5u32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x108db1d; continue;
 }
 0x108db1d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108db35; continue; }
  pc = 0x108db25; continue;
 }
 0x108db25 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108db1d; continue; }
  pc = 0x108db35; continue;
 }
 0x108db35 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11ec47au32; cpu.edi = v; }
  { let v: u32 = 0x1d7u32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x108db42; continue;
 }
 0x108db42 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108db5a; continue; }
  pc = 0x108db4a; continue;
 }
 0x108db4a => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108db42; continue; }
  pc = 0x108db5a; continue;
 }
 0x108db5a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x1133ba4u32), 0x4321u32); }
  if cpu.zf { pc = 0x108db6c; continue; }
  pc = 0x108db67; continue;
 }
 0x108db67 => {
  cpu.push(0x108db6c); f_0108d470(cpu); if cpu.returned_to != 0x108db6c { return; }
  pc = 0x108db6c; continue;
 }
 0x108db6c => {
  { let v: u32 = cpu.r32(0x148u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133b90u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133b8cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x108db88); f_010982ad(cpu); if cpu.returned_to != 0x108db88 { return; }
  { let v: u32 = cpu.r32(0x11ec444u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x63u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x108dba4); f_0109842c(cpu); if cpu.returned_to != 0x108dba4 { return; }
  { let v: u32 = cpu.r32(0x11ec444u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x65u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2fu32; cpu.push(v); }
  cpu.push(0x108dbc0); f_0109842c(cpu); if cpu.returned_to != 0x108dbc0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x140u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x108dbcf); f_01094b7d(cpu); if cpu.returned_to != 0x108dbcf { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108dbdc => {
  { let v: u32 = cpu.r32(0x140u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x108dbe8); f_01094b7d(cpu); if cpu.returned_to != 0x108dbe8 { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x108dbea; continue;
 }
 0x108dbea => {
  { let v: u32 = cpu.add32(cpu.esp, 0x14cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dbf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dbf8); }
 let mut pc: u32 = 0x108dbf8;
 loop { match pc {
 0x108dbf8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1133b9cu32), 0x0u32); }
  if cpu.zf { pc = 0x108dc45; continue; }
  pc = 0x108dc05; continue;
 }
 0x108dc05 => {
  { let v: u32 = cpu.r32(0x1133ba0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x2fu32; cpu.push(v); }
  cpu.push(0x108dc13); f_0109842c(cpu); if cpu.returned_to != 0x108dc13 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1133b9cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x108dc24); f_0109842c(cpu); if cpu.returned_to != 0x108dc24 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1133b90u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1133b8cu32); cpu.ebp = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1133b9cu32, v); }
  cpu.push(0x108dc42); f_01098934(cpu); if cpu.returned_to != 0x108dc42 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x108dc45; continue;
 }
 0x108dc45 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dc4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dc4c); }
 let mut pc: u32 = 0x108dc4c;
 loop { match pc {
 0x108dc4c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x5cu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x108dc63); f_01098c84(cpu); if cpu.returned_to != 0x108dc63 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108dc7f; continue; }
  pc = 0x108dc67; continue;
 }
 0x108dc67 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  cpu.push(0x108dc77); f_01098c3e(cpu); if cpu.returned_to != 0x108dc77 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108dc7f => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dc88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dc88); }
 let mut pc: u32 = 0x108dc88;
 loop { match pc {
 0x108dc88 => {
  { let v: u32 = cpu.r32(0x1133ba8u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dc90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dc90); }
 let mut pc: u32 = 0x108dc90;
 loop { match pc {
 0x108dc90 => {
  { let v: u32 = cpu.r32(0x1133bacu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dc98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dc98); }
 let mut pc: u32 = 0x108dc98;
 loop { match pc {
 0x108dc98 => {
  { let v: u32 = cpu.r32(0x1133bb0u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dca0); }
 let mut pc: u32 = 0x108dca0;
 loop { match pc {
 0x108dca0 => {
  { let v: u32 = cpu.r32(0x1133bb4u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dca8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dca8); }
 let mut pc: u32 = 0x108dca8;
 loop { match pc {
 0x108dca8 => {
  cpu.call(0xf009e589); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dcad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dcad); }
 let mut pc: u32 = 0x108dcad;
 loop { match pc {
 0x108dcad => {
  { let v = cpu.st(0) * cpu.rf64(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf64(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v = cpu.st(0) * cpu.rf64(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.unimplemented(0x108dcb9, "Fisttp");
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.unimplemented(0x108dcbd, "Fisttp");
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.unimplemented(0x108dcc1, "Fisttp");
  { let v: u32 = cpu.r32(0x130108ddu32).wrapping_add(cpu.ecx); cpu.w32(0x130108ddu32, v); }
  cpu.unimplemented(0x108dcc9, "Fisttp");
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.unimplemented(0x108dccd, "Fisttp");
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.unimplemented(0x108dcd1, "Fisttp");
  { let v: u32 = cpu.r32(0x8b0108ddu32).wrapping_add(cpu.esp); cpu.w32(0x8b0108ddu32, v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8) & 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0xbu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x108dd2b; continue; }
  pc = 0x108dce1; continue;
 }
 0x108dce1 => {
  cpu.poll(); pc = cpu.r32(0x108dca8u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x108dce9 => {
  { let v: u32 = 0x1137820u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108dcef => {
  { let v: u32 = 0x1137824u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108dcf5 => {
  { let v: u32 = 0x1137844u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108dcfb => {
  { let v: u32 = 0x1137870u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108dd01 => {
  { let v: u32 = 0x113789cu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108dd07 => {
  { let v: u32 = 0x11378c4u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108dd0d => {
  { let v: u32 = 0x11378ecu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108dd13 => {
  { let v: u32 = 0x1137918u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108dd19 => {
  { let v: u32 = 0x1137944u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108dd1f => {
  { let v: u32 = 0x113796cu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108dd25 => {
  { let v: u32 = 0x11379b0u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108dd2b => {
  { let v: u32 = 0x11379ecu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dd34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dd34); }
 let mut pc: u32 = 0x108dd34;
 loop { match pc {
 0x108dd34 => {
  { let v: u32 = 0x11ec3e8u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dd3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dd3c); }
 let mut pc: u32 = 0x108dd3c;
 loop { match pc {
 0x108dd3c => {
  { let v: u32 = 0x11ec448u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dd44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dd44); }
 let mut pc: u32 = 0x108dd44;
 loop { match pc {
 0x108dd44 => {
  { let v: u32 = 0x11ec47au32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dd4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dd4a); }
 let mut pc: u32 = 0x108dd4a;
 loop { match pc {
 0x108dd4a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dd4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dd4b); }
 let mut pc: u32 = 0x108dd4b;
 loop { match pc {
 0x108dd4b => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x1133bb8u32); cpu.push(0x108dd54); cpu.call(t); if cpu.returned_to != 0x108dd54 { return; }
  let t = cpu.r32(0x1133bbcu32); cpu.push(0x108dd5a); cpu.call(t); if cpu.returned_to != 0x108dd5a { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x108dd61); f_0108dd63(cpu); if cpu.returned_to != 0x108dd61 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dd63(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108dd63); } r_0108dd63(cpu, 0x108dd63); }
pub fn f_0108ebb0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108ebb0); } r_0108dd63(cpu, 0x108ebb0); }
pub fn f_0108ec24(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108ec24); } r_0108dd63(cpu, 0x108ec24); }
pub fn f_0108ed08(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108ed08); } r_0108dd63(cpu, 0x108ed08); }
pub fn f_0108ee41(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108ee41); } r_0108dd63(cpu, 0x108ee41); }
fn r_0108dd63(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x108dd63 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x1133bbcu32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x108dd66); cpu.push(0x108dd6c); cpu.call(t); if cpu.returned_to != 0x108dd6c { return; }
  let t = cpu.r32(0x1133bc0u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x108dd6c); cpu.push(0x108dd72); cpu.call(t); if cpu.returned_to != 0x108dd72 { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x108ee3f; continue;
 }
 0x108ebb0 => {
  pc = 0x108ec28; continue;
 }
 0x108ec24 => {
  cpu.unimplemented(0x108ec24, "Arpl");
  { let v: u8 = (cpu.ebx as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp & 0xfffffffcu32; cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133e64u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133e50u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x24u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1133e5cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x50484152u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1133e87u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1133e88u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x81u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x4458u16); }
  if !cpu.zf { pc = 0x108ecaa; continue; }
  pc = 0x108ec6c; continue;
 }
 0x108ec28 => {
  { cpu.interrupts_off = false; }
  { let v: u32 = cpu.esp & 0xfffffffcu32; cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133e64u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133e50u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x24u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1133e5cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x50484152u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1133e87u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1133e88u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x81u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x4458u16); }
  if !cpu.zf { pc = 0x108ecaa; continue; }
  pc = 0x108ec6c; continue;
 }
 0x108ec6c => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x30u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r16(0x1133e5cu32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.bases[0])); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xfffu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfffff000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133e50u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.shr32(cpu.ebx, 12u32); cpu.ebx = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2cu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x108ece2; continue;
 }
 0x108ecaa => {
  { cpu.sub16((cpu.eax as u16), 0x4243u16); }
  if !cpu.zf { pc = 0x108ece4; continue; }
  pc = 0x108ecb0; continue;
 }
 0x108ecb0 => {
  { let v: u32 = cpu.edx; cpu.w32(0x1133bf8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x1133e5cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x108ece2; continue;
 }
 0x108ece2 => {
  pc = 0x108ed3d; continue;
 }
 0x108ece4 => {
  { let v: u16 = 0x78u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xff00u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108ed26; continue; }
  pc = 0x108ecf2; continue;
 }
 0x108ecf2 => {
  { let v: u16 = (cpu.segments[5] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.zf { pc = 0x108ed01; continue; }
  pc = 0x108ecfb; continue;
 }
 0x108ecfb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1133bfcu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x108ed01; continue;
 }
 0x108ed01 => {
  { let v: u16 = 0x6u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x108ed08; continue;
 }
 0x108ed08 => {
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x108ed15; continue; }
  pc = 0x108ed13; continue;
 }
 0x108ed13 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x108ed15; continue;
 }
 0x108ed15 => {
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x1133e5cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.bases[0])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x108ed3d; continue;
 }
 0x108ed26 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x24u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2cu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x17u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x108ed3d; continue;
 }
 0x108ed3d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1133e7eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1133e7fu32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ebx; cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x108ee85u32.wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1133e81u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1133e85u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.r16(0x1133e5cu32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x11ef628u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xfu32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xf0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0])); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.df = false; }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 1); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u16 = (cpu.segments[0] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.ebx; cpu.set_segment(3, v as u16); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  if cpu.zf { pc = 0x108ed92; continue; }
  pc = 0x108ed8f; continue;
 }
 0x108ed8f => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.movs(1, true); }
  pc = 0x108ed92; continue;
 }
 0x108ed92 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.r16(0x1133e85u32.wrapping_add(cpu.bases[0])); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  pc = 0x108eda4; continue;
 }
 0x108eda4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x20202020u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x37386f6eu32); }
  if !cpu.zf { pc = 0x108edb9; continue; }
  pc = 0x108edb2; continue;
 }
 0x108edb2 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x3du8); }
  if !cpu.zf { pc = 0x108edb9; continue; }
  pc = 0x108edb8; continue;
 }
 0x108edb8 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x108edb9; continue;
 }
 0x108edb9 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  { cpu.lods(1, false); }
  if !cpu.zf { cpu.poll(); pc = 0x108edb9; continue; }
  pc = 0x108edbf; continue;
 }
 0x108edbf => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108eda4; continue; }
  pc = 0x108edc4; continue;
 }
 0x108edc4 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x108edc7; continue;
 }
 0x108edc7 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  { cpu.movs(1, false); }
  if !cpu.zf { cpu.poll(); pc = 0x108edc7; continue; }
  pc = 0x108edcd; continue;
 }
 0x108edcd => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x1133e7cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1133e60u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1133e4cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x11ef628u32; cpu.ecx = v; }
  { let v: u32 = 0x1139648u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edi); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1133e7eu32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x108ee08; continue; }
  pc = 0x108edfb; continue;
 }
 0x108edfb => {
  { cpu.sub32(cpu.ecx, 0x1000u32); }
  if (cpu.cf || cpu.zf) { pc = 0x108ee08; continue; }
  pc = 0x108ee03; continue;
 }
 0x108ee03 => {
  { let v: u32 = 0x1000u32; cpu.ecx = v; }
  pc = 0x108ee08; continue;
 }
 0x108ee08 => {
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x11ef628u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfu32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x1133e54u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1133e58u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  cpu.flat_ds(0x108ee33); cpu.push(0x108ee38); f_0109a9f0(cpu); if cpu.returned_to != 0x108ee38 { return; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  cpu.flat_ds(0x108ee3a); cpu.push(0x108ee3f); f_0109a9a0(cpu); if cpu.returned_to != 0x108ee3f { return; }
  pc = 0x108ee3f; continue;
 }
 0x108ee3f => {
  pc = 0x108ee64; continue;
 }
 0x108ee41 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x108ec24u32; cpu.edx = v; }
  { let v: u16 = 0x3d01u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.df = false; }
  pc = 0x108ee55; continue;
 }
 0x108ee55 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108ee55; continue; }
  pc = 0x108ee5a; continue;
 }
 0x108ee5a => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x108ee64; continue;
 }
 0x108ee64 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.flat_ds(0x108ee6f); cpu.push(0x108ee74); f_0109aa3b(cpu); if cpu.returned_to != 0x108ee74 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x4cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x108ee79); f_0108ee7c(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dd79(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dd79); }
 let mut pc: u32 = 0x108dd79;
 loop { match pc {
 0x108dd79 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x108dd83); f_0108de27(cpu); if cpu.returned_to != 0x108dd83 { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.ebx))); cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dd88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dd88); }
 let mut pc: u32 = 0x108dd88;
 loop { match pc {
 0x108dd88 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x108dd79u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x108dd9a); f_01098c9e(cpu); if cpu.returned_to != 0x108dd9a { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dd9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dd9d); }
 let mut pc: u32 = 0x108dd9d;
 loop { match pc {
 0x108dd9d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x108dd79u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x108ddbc); f_01098c9e(cpu); if cpu.returned_to != 0x108ddbc { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ddc3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ddc3); }
 let mut pc: u32 = 0x108ddc3;
 loop { match pc {
 0x108ddc3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x108ddc7; continue;
 }
 0x108ddc7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x108dddb; continue; }
  pc = 0x108ddd4; continue;
 }
 0x108ddd4 => {
  { cpu.sub32(cpu.ecx, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x108dddb; continue; }
  pc = 0x108ddd9; continue;
 }
 0x108ddd9 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x108dddb; continue;
 }
 0x108dddb => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x108ddec; continue; }
  pc = 0x108dde4; continue;
 }
 0x108dde4 => {
  { cpu.sub32(cpu.ecx, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x108ddec; continue; }
  pc = 0x108dde9; continue;
 }
 0x108dde9 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x108ddec; continue;
 }
 0x108ddec => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x108ddf8; continue; }
  pc = 0x108ddf0; continue;
 }
 0x108ddf0 => {
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x108ddf8; continue; }
  pc = 0x108ddf4; continue;
 }
 0x108ddf4 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x108ddc7; continue;
 }
 0x108ddf8 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108de0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108de0a); }
 let mut pc: u32 = 0x108de0a;
 loop { match pc {
 0x108de0a => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1133e70u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1133e70u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108de25; continue; }
  pc = 0x108de1c; continue;
 }
 0x108de1c => {
  { let v: u8 = 0x8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  pc = 0x108de25; continue;
 }
 0x108de25 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108de27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108de27); }
 let mut pc: u32 = 0x108de27;
 loop { match pc {
 0x108de27 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108de36(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108de36); }
 let mut pc: u32 = 0x108de36;
 loop { match pc {
 0x108de36 => {
  { cpu.and8(cpu.r8(0xbu32.wrapping_add(cpu.esp)), 0x80u8); }
  if cpu.zf { pc = 0x108de5d; continue; }
  pc = 0x108de3d; continue;
 }
 0x108de3d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if cpu.zf { pc = 0x108de5d; continue; }
  pc = 0x108de4c; continue;
 }
 0x108de4c => {
  { let v = 0x3u32; cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x108de5b); f_0109987c(cpu); if cpu.returned_to != 0x108de5b { return; }
  pc = 0x108de63; continue;
 }
 0x108de5d => {
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  pc = 0x108de63; continue;
 }
 0x108de63 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108de9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108de9e); }
 let mut pc: u32 = 0x108de9e;
 loop { match pc {
 0x108de9e => {
  { let v = cpu.eax; cpu.push(v); }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  {  }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v: u8 = 0x1fu8; cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x0u32.wrapping_add(cpu.esp)); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { cpu.fpu_control = cpu.r16(0x4u32.wrapping_add(cpu.esp)); }
  {  }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108debb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108debb); }
 let mut pc: u32 = 0x108debb;
 loop { match pc {
 0x108debb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.eax))); cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108dece(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108dece); }
 let mut pc: u32 = 0x108dece;
 loop { match pc {
 0x108dece => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x108debbu32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x108deef); f_01098c9e(cpu); if cpu.returned_to != 0x108deef { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108defe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108defe); }
 let mut pc: u32 = 0x108defe;
 loop { match pc {
 0x108defe => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x108df1c); f_0108df22(cpu); if cpu.returned_to != 0x108df1c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108df22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108df22); }
 let mut pc: u32 = 0x108df22;
 loop { match pc {
 0x108df22 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x108df2e; continue;
 }
 0x108df2e => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x20u32); }
  if !cpu.zf { pc = 0x108df39; continue; }
  pc = 0x108df36; continue;
 }
 0x108df36 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x108df2e; continue;
 }
 0x108df39 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x83u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x3du8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x108df6c; continue; }
  pc = 0x108df66; continue;
 }
 0x108df66 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x108df6c; continue;
 }
 0x108df6c => {
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), 0x3u8); }
  if cpu.zf { pc = 0x108dfef; continue; }
  pc = 0x108df79; continue;
 }
 0x108df79 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x108dfef; continue; }
  pc = 0x108df82; continue;
 }
 0x108df82 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x108df89); f_010998f7(cpu); if cpu.returned_to != 0x108df89 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108dfef; continue; }
  pc = 0x108df8d; continue;
 }
 0x108df8d => {
  { cpu.and8(cpu.r8(0x29u32.wrapping_add(cpu.esp)), 0x4u8); }
  if cpu.zf { pc = 0x108dfb7; continue; }
  pc = 0x108df94; continue;
 }
 0x108df94 => {
  { cpu.and8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x108dfb7; continue; }
  pc = 0x108df99; continue;
 }
 0x108df99 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  cpu.push(0x108dfad); f_01099923(cpu); if cpu.returned_to != 0x108dfad { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x108e102; continue;
 }
 0x108dfb7 => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x40u8); }
  if cpu.zf { pc = 0x108dfef; continue; }
  pc = 0x108dfbe; continue;
 }
 0x108dfbe => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x108dfef; continue; }
  pc = 0x108dfd5; continue;
 }
 0x108dfd5 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x108dfdf; continue;
 }
 0x108dfdf => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x108dfea); f_0109995d(cpu); if cpu.returned_to != 0x108dfea { return; }
  pc = 0x108e102; continue;
 }
 0x108dfef => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x108e091; continue; }
  pc = 0x108dff8; continue;
 }
 0x108dff8 => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x20u8); }
  if cpu.zf { cpu.poll(); pc = 0x108dfdf; continue; }
  pc = 0x108dfff; continue;
 }
 0x108dfff => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { cpu.poll(); pc = 0x108dfdf; continue; }
  pc = 0x108e00a; continue;
 }
 0x108e00a => {
  { let v: u32 = 0x30u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108e01d; continue; }
  pc = 0x108e018; continue;
 }
 0x108e018 => {
  { let v: u32 = 0x180u32; cpu.eax = v; }
  pc = 0x108e01d; continue;
 }
 0x108e01d => {
  { let v: u32 = cpu.r32(0x1133ef4u32); cpu.edx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax & cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if !cpu.zf { pc = 0x108e032; continue; }
  pc = 0x108e02d; continue;
 }
 0x108e02d => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x108e032; continue;
 }
 0x108e032 => {
  { let v: u32 = cpu.and32(cpu.ecx, 0xffu32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = 0x3cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x108dfdf; continue; }
  pc = 0x108e049; continue;
 }
 0x108e049 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x108e091; continue; }
  pc = 0x108e057; continue;
 }
 0x108e057 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x108dfdf; continue; }
  pc = 0x108e06c; continue;
 }
 0x108e06c => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), ((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3du8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x108dfdf; continue; }
  pc = 0x108e08b; continue;
 }
 0x108e08b => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x108e091; continue;
 }
 0x108e091 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x108e098); f_010999ce(cpu); if cpu.returned_to != 0x108e098 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3cu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x108e0a3); f_010998f7(cpu); if cpu.returned_to != 0x108e0a3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108e0aa; continue; }
  pc = 0x108e0a7; continue;
 }
 0x108e0a7 => {
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x20u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x108e0aa; continue;
 }
 0x108e0aa => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)) & 0x7fu8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x2u32); }
  if !cpu.zf { pc = 0x108e0b9; continue; }
  pc = 0x108e0b6; continue;
 }
 0x108e0b6 => {
  { let v: u8 = (cpu.edx as u8) | 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x108e0b9; continue;
 }
 0x108e0b9 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x108e0c3; continue; }
  pc = 0x108e0c0; continue;
 }
 0x108e0c0 => {
  { let v: u8 = (cpu.edx as u8) | 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x108e0c3; continue;
 }
 0x108e0c3 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x1u32); }
  if !cpu.zf { pc = 0x108e0cd; continue; }
  pc = 0x108e0ca; continue;
 }
 0x108e0ca => {
  { let v: u8 = (cpu.edx as u8) | 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x108e0cd; continue;
 }
 0x108e0cd => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x10u8); }
  if cpu.zf { pc = 0x108e0d7; continue; }
  pc = 0x108e0d4; continue;
 }
 0x108e0d4 => {
  { let v: u8 = (cpu.edx as u8) | 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x108e0d7; continue;
 }
 0x108e0d7 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x3u8); }
  if cpu.zf { pc = 0x108e0eb; continue; }
  pc = 0x108e0e4; continue;
 }
 0x108e0e4 => {
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if cpu.zf { pc = 0x108e0f9; continue; }
  pc = 0x108e0e9; continue;
 }
 0x108e0e9 => {
  pc = 0x108e0f7; continue;
 }
 0x108e0eb => {
  { cpu.sub32(cpu.r32(0x1133e41u32), 0x200u32); }
  if !cpu.zf { pc = 0x108e0f9; continue; }
  pc = 0x108e0f7; continue;
 }
 0x108e0f7 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x108e0f9; continue;
 }
 0x108e0f9 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x108e100); f_01099a23(cpu); if cpu.returned_to != 0x108e100 { return; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x108e102; continue;
 }
 0x108e102 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
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
pub fn f_0108e10b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e10b); }
 let mut pc: u32 = 0x108e10b;
 loop { match pc {
 0x108e10b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x108e119); f_010999ce(cpu); if cpu.returned_to != 0x108e119 { return; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x108e126); f_01099a23(cpu); if cpu.returned_to != 0x108e126 { return; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x20u8); }
  if cpu.zf { pc = 0x108e166; continue; }
  pc = 0x108e12b; continue;
 }
 0x108e12b => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x108e14b; continue; }
  pc = 0x108e13e; continue;
 }
 0x108e13e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x108e149); f_0109995d(cpu); if cpu.returned_to != 0x108e149 { return; }
  pc = 0x108e168; continue;
 }
 0x108e14b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), 0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x108e13e; continue; }
  pc = 0x108e166; continue;
 }
 0x108e166 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108e168; continue;
 }
 0x108e168 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e170(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e170); }
 let mut pc: u32 = 0x108e170;
 loop { match pc {
 0x108e170 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  cpu.push(0x108e17f); f_010999ce(cpu); if cpu.returned_to != 0x108e17f { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108e19b; continue; }
  pc = 0x108e187; continue;
 }
 0x108e187 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x108e18c; continue;
 }
 0x108e18c => {
  cpu.push(0x108e191); f_01099923(cpu); if cpu.returned_to != 0x108e191 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x108e263; continue;
 }
 0x108e19b => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x108e1a6; continue; }
  pc = 0x108e19f; continue;
 }
 0x108e19f => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.poll(); pc = 0x108e18c; continue;
 }
 0x108e1a6 => {
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if cpu.zf { pc = 0x108e1d2; continue; }
  pc = 0x108e1aa; continue;
 }
 0x108e1aa => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x108e261; continue; }
  pc = 0x108e1c3; continue;
 }
 0x108e1c3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x108e1c8; continue;
 }
 0x108e1c8 => {
  cpu.push(0x108e1cd); f_0109995d(cpu); if cpu.returned_to != 0x108e1cd { return; }
  pc = 0x108e263; continue;
 }
 0x108e1d2 => {
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  pc = 0x108e1d8; continue;
 }
 0x108e1d8 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x108e1fc; continue; }
  pc = 0x108e1f5; continue;
 }
 0x108e1f5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x108e1c8; continue;
 }
 0x108e1fc => {
  if cpu.zf { pc = 0x108e261; continue; }
  pc = 0x108e1fe; continue;
 }
 0x108e1fe => {
  { let v: u32 = cpu.edi ^ cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x108e23e; continue;
 }
 0x108e20d => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x1au32); }
  if !cpu.zf { pc = 0x108e22e; continue; }
  pc = 0x108e216; continue;
 }
 0x108e216 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.push(0x108e22c); f_01094bff(cpu); if cpu.returned_to != 0x108e22c { return; }
  pc = 0x108e261; continue;
 }
 0x108e22e => {
  { cpu.sub32(cpu.ebx, 0xdu32); }
  if cpu.zf { pc = 0x108e23c; continue; }
  pc = 0x108e233; continue;
 }
 0x108e233 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x108e23c; continue;
 }
 0x108e23c => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x108e23e; continue;
 }
 0x108e23e => {
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.esp))); }
  if cpu.cf { cpu.poll(); pc = 0x108e20d; continue; }
  pc = 0x108e244; continue;
 }
 0x108e244 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x20u8); }
  if !cpu.zf { pc = 0x108e261; continue; }
  pc = 0x108e259; continue;
 }
 0x108e259 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x108e1d8; continue; }
  pc = 0x108e261; continue;
 }
 0x108e261 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x108e263; continue;
 }
 0x108e263 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e26b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e26b); }
 let mut pc: u32 = 0x108e26b;
 loop { match pc {
 0x108e26b => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x108e27b; continue; }
  pc = 0x108e26f; continue;
 }
 0x108e26f => {
  { cpu.sub32(cpu.eax, cpu.r32(0x1133ef8u32)); }
  if (cpu.cf || cpu.zf) { f_01099a38(cpu); return; }
  pc = 0x108e27b; continue;
 }
 0x108e27b => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x108e285); f_01099923(cpu); if cpu.returned_to != 0x108e285 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e28c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e28c); }
 let mut pc: u32 = 0x108e28c;
 loop { match pc {
 0x108e28c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u8 = 0x1au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = 0x4eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  cpu.push(0x108e2a0); f_01099a6e(cpu); if cpu.returned_to != 0x108e2a0 { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x108e2a7); f_0108e2c8(cpu); if cpu.returned_to != 0x108e2a7 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e2a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e2a9); }
 let mut pc: u32 = 0x108e2a9;
 loop { match pc {
 0x108e2a9 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x1au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  cpu.push(0x108e2b5); f_0108e2eb(cpu); if cpu.returned_to != 0x108e2b5 { return; }
  { let v: u8 = 0x4fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  cpu.push(0x108e2be); f_01099a6e(cpu); if cpu.returned_to != 0x108e2be { return; }
  cpu.push(0x108e2c3); f_0108e2c8(cpu); if cpu.returned_to != 0x108e2c3 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e2c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e2c8); }
 let mut pc: u32 = 0x108e2c8;
 loop { match pc {
 0x108e2c8 => {
  { cpu.sub8(cpu.r8(0x1133e7eu32), 0x9u8); }
  if !cpu.zf { pc = 0x108e2ea; continue; }
  pc = 0x108e2d1; continue;
 }
 0x108e2d1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = 0x2fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0x2bu32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x108e2ea; continue;
 }
 0x108e2ea => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e2eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e2eb); }
 let mut pc: u32 = 0x108e2eb;
 loop { match pc {
 0x108e2eb => {
  { cpu.sub8(cpu.r8(0x1133e7eu32), 0x9u8); }
  if !cpu.zf { pc = 0x108e30d; continue; }
  pc = 0x108e2f4; continue;
 }
 0x108e2f4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = 0x2fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = 0x2bu32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x108e30d; continue;
 }
 0x108e30d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e30e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e30e); }
 let mut pc: u32 = 0x108e30e;
 loop { match pc {
 0x108e30e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x108e310; continue;
 }
 0x108e310 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x108e32a; continue; }
  pc = 0x108e319; continue;
 }
 0x108e319 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108e310; continue; }
  pc = 0x108e32a; continue;
 }
 0x108e32a => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e32d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e32d); }
 let mut pc: u32 = 0x108e32d;
 loop { match pc {
 0x108e32d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[0] as u32); }
  if cpu.ecx == 0 { pc = 0x108e34c; continue; }
  pc = 0x108e341; continue;
 }
 0x108e341 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { cpu.scas(1, 2); }
  if !cpu.zf { pc = 0x108e34c; continue; }
  pc = 0x108e349; continue;
 }
 0x108e349 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.and16((cpu.eax as u16), 0xcf89u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x108e351; continue;
 }
 0x108e34c => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x108e351; continue;
 }
 0x108e351 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x108e362; continue; }
  pc = 0x108e355; continue;
 }
 0x108e355 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x108e362; continue; }
  pc = 0x108e35d; continue;
 }
 0x108e35d => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x108e351; continue;
 }
 0x108e362 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e36c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e36c); }
 let mut pc: u32 = 0x108e36c;
 loop { match pc {
 0x108e36c => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x39u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x108e389; continue; }
  pc = 0x108e37d; continue;
 }
 0x108e37d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x108e387); f_0109995d(cpu); if cpu.returned_to != 0x108e387 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108e389 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e38d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e38d); }
 let mut pc: u32 = 0x108e38d;
 loop { match pc {
 0x108e38d => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x3bu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x108e3aa; continue; }
  pc = 0x108e39e; continue;
 }
 0x108e39e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x108e3a8); f_0109995d(cpu); if cpu.returned_to != 0x108e3a8 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108e3aa => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e3ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e3ae); }
 let mut pc: u32 = 0x108e3ae;
 loop { match pc {
 0x108e3ae => {
  { cpu.sub32(cpu.eax, 0x61u32); }
  if (cpu.sf != cpu.of) { pc = 0x108e3bb; continue; }
  pc = 0x108e3b3; continue;
 }
 0x108e3b3 => {
  { cpu.sub32(cpu.eax, 0x7au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x108e3bb; continue; }
  pc = 0x108e3b8; continue;
 }
 0x108e3b8 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  pc = 0x108e3bb; continue;
 }
 0x108e3bb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e3bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e3bc); }
 let mut pc: u32 = 0x108e3bc;
 loop { match pc {
 0x108e3bc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e3d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e3d0); }
 let mut pc: u32 = 0x108e3d0;
 loop { match pc {
 0x108e3d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0xeu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e3e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e3e4); }
 let mut pc: u32 = 0x108e3e4;
 loop { match pc {
 0x108e3e4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0x3u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u8 = cpu.xor8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x47u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x108e413; continue; }
  pc = 0x108e405; continue;
 }
 0x108e405 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x108e40a; continue;
 }
 0x108e40a => {
  cpu.push(0x108e40f); f_01099923(cpu); if cpu.returned_to != 0x108e40f { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108e460; continue;
 }
 0x108e413 => {
  { let v: u8 = 0x19u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x5cu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x41u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x3au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x108e457; continue; }
  pc = 0x108e441; continue;
 }
 0x108e441 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x108e448); f_01094a9a(cpu); if cpu.returned_to != 0x108e448 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108e455; continue; }
  pc = 0x108e44e; continue;
 }
 0x108e44e => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.poll(); pc = 0x108e40a; continue;
 }
 0x108e455 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x108e457; continue;
 }
 0x108e457 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x108e460); f_01098c3e(cpu); if cpu.returned_to != 0x108e460 { return; }
  pc = 0x108e460; continue;
 }
 0x108e460 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e46c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e46c); }
 let mut pc: u32 = 0x108e46c;
 loop { match pc {
 0x108e46c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e485(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e485); }
 let mut pc: u32 = 0x108e485;
 loop { match pc {
 0x108e485 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  f_0108e579(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e48f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e48f); }
 let mut pc: u32 = 0x108e48f;
 loop { match pc {
 0x108e48f => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1133bccu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x108e4be; continue; }
  pc = 0x108e4a2; continue;
 }
 0x108e4a2 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x108e4be; continue; }
  pc = 0x108e4a7; continue;
 }
 0x108e4a7 => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x108e4be; continue; }
  pc = 0x108e4ac; continue;
 }
 0x108e4ac => {
  { let v: u32 = 0x2u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1133bccu32, v); }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(0x108e4be); cpu.call(t); if cpu.returned_to != 0x108e4be { return; }
  pc = 0x108e4be; continue;
 }
 0x108e4be => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e4c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e4c4); }
 let mut pc: u32 = 0x108e4c4;
 loop { match pc {
 0x108e4c4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf != cpu.of) { pc = 0x108e4d1; continue; }
  pc = 0x108e4cc; continue;
 }
 0x108e4cc => {
  { cpu.sub32(cpu.eax, 0xcu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x108e4e2; continue; }
  pc = 0x108e4d1; continue;
 }
 0x108e4d1 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x108e4db); f_01099923(cpu); if cpu.returned_to != 0x108e4db { return; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108e4e2 => {
  { let v: u32 = 0x108e485u32; cpu.w32(0x1133f50u32, v); }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x108e509; continue; }
  pc = 0x108e4f1; continue;
 }
 0x108e4f1 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if !cpu.zf { pc = 0x108e4fd; continue; }
  pc = 0x108e4f6; continue;
 }
 0x108e4f6 => {
  cpu.push(0x108e4fb); f_01099c3a(cpu); if cpu.returned_to != 0x108e4fb { return; }
  pc = 0x108e539; continue;
 }
 0x108e4fd => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x108e539; continue; }
  pc = 0x108e502; continue;
 }
 0x108e502 => {
  cpu.push(0x108e507); f_01099e28(cpu); if cpu.returned_to != 0x108e507 { return; }
  pc = 0x108e539; continue;
 }
 0x108e509 => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if !cpu.zf { pc = 0x108e526; continue; }
  pc = 0x108e50e; continue;
 }
 0x108e50e => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if !cpu.zf { pc = 0x108e51a; continue; }
  pc = 0x108e513; continue;
 }
 0x108e513 => {
  cpu.push(0x108e518); f_01099df0(cpu); if cpu.returned_to != 0x108e518 { return; }
  pc = 0x108e539; continue;
 }
 0x108e51a => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x108e539; continue; }
  pc = 0x108e51f; continue;
 }
 0x108e51f => {
  cpu.push(0x108e524); f_01099f43(cpu); if cpu.returned_to != 0x108e524 { return; }
  pc = 0x108e539; continue;
 }
 0x108e526 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x108e539; continue; }
  pc = 0x108e52b; continue;
 }
 0x108e52b => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.zf { pc = 0x108e539; continue; }
  pc = 0x108e52f; continue;
 }
 0x108e52f => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x108e539; continue; }
  pc = 0x108e534; continue;
 }
 0x108e534 => {
  cpu.push(0x108e539); f_0109a05e(cpu); if cpu.returned_to != 0x108e539 { return; }
  pc = 0x108e539; continue;
 }
 0x108e539 => {
  { let v: u32 = cpu.r32(0x1133bc4u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1133bc4u32.wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e549(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108e549); } r_0108e549(cpu, 0x108e549); }
pub fn f_0108e579(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108e579); } r_0108e549(cpu, 0x108e579); }
fn r_0108e549(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x108e549 => {
  { let v: u32 = 0xa30108e5u32; cpu.eax = v; }
  { let v: u32 = cpu.port_in(0x8u16, 4) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.port_out(0x8u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.port_in(0x8u16, 4) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.port_out(0x8u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.port_out(0x8u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.esp.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.port_in(0x8u16, 4) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.port_out(0x8u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.port_out(0x8u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.port_out(0x8u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.port_out(0x8u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.port_out(0x8u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.r32(0x51u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.edx); cpu.w32(0x51u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1133bc4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xbu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x108e631; continue; }
  pc = 0x108e59b; continue;
 }
 0x108e579 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1133bc4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xbu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x108e631; continue; }
  pc = 0x108e59b; continue;
 }
 0x108e59b => {
  cpu.poll(); pc = cpu.r32(0x108e549u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x108e5a3 => {
  { let v: u32 = 0x8cu32; cpu.eax = v; }
  cpu.push(0x108e5ad); f_0108e48f(cpu); if cpu.returned_to != 0x108e5ad { return; }
  pc = 0x108e5ad; continue;
 }
 0x108e5ad => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108e5af; continue;
 }
 0x108e5af => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108e5b8 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x2u32); }
  if !cpu.zf { pc = 0x108e5c3; continue; }
  pc = 0x108e5be; continue;
 }
 0x108e5be => {
  cpu.push(0x108e5c3); f_01099a87(cpu); if cpu.returned_to != 0x108e5c3 { return; }
  pc = 0x108e5c3; continue;
 }
 0x108e5c3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x108e5ad; continue; }
  pc = 0x108e5cb; continue;
 }
 0x108e5cb => {
  { cpu.sub32(cpu.edi, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x108e5ad; continue; }
  pc = 0x108e5d0; continue;
 }
 0x108e5d0 => {
  { cpu.sub32(cpu.edi, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x108e5ad; continue; }
  pc = 0x108e5d5; continue;
 }
 0x108e5d5 => {
  { let v: u32 = 0x2u32; cpu.w32(0x1133bc4u32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  cpu.push(0x108e5e5); f_01099c3a(cpu); if cpu.returned_to != 0x108e5e5 { return; }
  pc = 0x108e5e5; continue;
 }
 0x108e5e5 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x108e5e7; continue;
 }
 0x108e5e7 => {
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(0x108e5ea); cpu.call(t); if cpu.returned_to != 0x108e5ea { return; }
  cpu.poll(); pc = 0x108e5ad; continue;
 }
 0x108e5ec => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x108e5ad; continue; }
  pc = 0x108e5f4; continue;
 }
 0x108e5f4 => {
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x108e5ad; continue; }
  pc = 0x108e5f9; continue;
 }
 0x108e5f9 => {
  { cpu.sub32(cpu.ebx, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x108e5ad; continue; }
  pc = 0x108e5fe; continue;
 }
 0x108e5fe => {
  { let v: u32 = 0x2u32; cpu.w32(0x1133bc4u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x108e60d); f_01099df0(cpu); if cpu.returned_to != 0x108e60d { return; }
  cpu.poll(); pc = 0x108e5e5; continue;
 }
 0x108e60f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x108e5ad; continue; }
  pc = 0x108e617; continue;
 }
 0x108e617 => {
  { cpu.sub32(cpu.ebp, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x108e5ad; continue; }
  pc = 0x108e61c; continue;
 }
 0x108e61c => {
  { cpu.sub32(cpu.ebp, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x108e5ad; continue; }
  pc = 0x108e621; continue;
 }
 0x108e621 => {
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1133bc4u32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  cpu.poll(); pc = 0x108e5e7; continue;
 }
 0x108e631 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x108e5af; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e63c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e63c); }
 let mut pc: u32 = 0x108e63c;
 loop { match pc {
 0x108e63c => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.push(v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x22u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.w16(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.segments[4] as u16); cpu.w16(0x26u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.segments[5] as u16); cpu.w16(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x2au32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e6d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e6d2); }
 let mut pc: u32 = 0x108e6d2;
 loop { match pc {
 0x108e6d2 => {
  pc = 0x109a0ca; continue;
 }
 0x109a0ca => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x109a0d2); f_0109a064(cpu); if cpu.returned_to != 0x109a0d2 { return; }
  { let v: u32 = cpu.r32(0x11ee14cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.cf { pc = 0x109a0e4; continue; }
  pc = 0x109a0de; continue;
 }
 0x109a0de => {
  { let v: u32 = 0x83d600u32.wrapping_add(cpu.eax); cpu.edx = v; }
  pc = 0x109a0e4; continue;
 }
 0x109a0e4 => {
  { let v: u32 = cpu.r32(0x11ee14cu32); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e6d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e6d7); }
 let mut pc: u32 = 0x108e6d7;
 loop { match pc {
 0x108e6d7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x108e6eb); f_0109a0f2(cpu); if cpu.returned_to != 0x108e6eb { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x72u32); }
  if cpu.zf { pc = 0x108e713; continue; }
  pc = 0x108e6f8; continue;
 }
 0x108e6f8 => {
  { cpu.sub32(cpu.eax, 0x77u32); }
  if cpu.zf { pc = 0x108e713; continue; }
  pc = 0x108e6fd; continue;
 }
 0x108e6fd => {
  { cpu.sub32(cpu.eax, 0x61u32); }
  if cpu.zf { pc = 0x108e713; continue; }
  pc = 0x108e702; continue;
 }
 0x108e702 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x108e70c); f_01099923(cpu); if cpu.returned_to != 0x108e70c { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108e7a0; continue;
 }
 0x108e713 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.esi as u16) | 0x3u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x2bu32); }
  if !cpu.zf { pc = 0x108e748; continue; }
  pc = 0x108e723; continue;
 }
 0x108e723 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.esi as u16) | 0x40u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0x62u32); }
  if cpu.zf { pc = 0x108e744; continue; }
  pc = 0x108e733; continue;
 }
 0x108e733 => {
  { cpu.sub32(cpu.eax, 0x74u32); }
  if cpu.zf { pc = 0x108e782; continue; }
  pc = 0x108e738; continue;
 }
 0x108e738 => {
  { cpu.sub32(cpu.r32(0x1133e41u32), 0x200u32); }
  pc = 0x108e742; continue;
 }
 0x108e742 => {
  if !cpu.zf { pc = 0x108e782; continue; }
  pc = 0x108e744; continue;
 }
 0x108e744 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  pc = 0x108e782; continue;
 }
 0x108e748 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x62u32); }
  if !cpu.zf { pc = 0x108e765; continue; }
  pc = 0x108e751; continue;
 }
 0x108e751 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x2bu32); }
  if !cpu.zf { pc = 0x108e782; continue; }
  pc = 0x108e761; continue;
 }
 0x108e761 => {
  { let v: u8 = (cpu.eax as u8) | 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x108e780; continue;
 }
 0x108e765 => {
  { cpu.sub32(cpu.ecx, 0x74u32); }
  if !cpu.zf { pc = 0x108e774; continue; }
  pc = 0x108e76a; continue;
 }
 0x108e76a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2bu32); }
  cpu.poll(); pc = 0x108e742; continue;
 }
 0x108e774 => {
  { cpu.sub32(cpu.r32(0x1133e41u32), 0x200u32); }
  if !cpu.zf { pc = 0x108e782; continue; }
  pc = 0x108e780; continue;
 }
 0x108e780 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x108e782; continue;
 }
 0x108e782 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x77u32); }
  if !cpu.zf { pc = 0x108e791; continue; }
  pc = 0x108e78c; continue;
 }
 0x108e78c => {
  { let v: u8 = (cpu.edx as u8) | 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x108e79e; continue;
 }
 0x108e791 => {
  { cpu.sub32(cpu.eax, 0x61u32); }
  if !cpu.zf { pc = 0x108e79b; continue; }
  pc = 0x108e796; continue;
 }
 0x108e796 => {
  { let v: u8 = (cpu.edx as u8) | 0x82u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x108e79e; continue;
 }
 0x108e79b => {
  { let v: u8 = (cpu.edx as u8) | 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x108e79e; continue;
 }
 0x108e79e => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x108e7a0; continue;
 }
 0x108e7a0 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e7a8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108e7a8); } r_0108e7a8(cpu, 0x108e7a8); }
pub fn f_0108e800(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108e800); } r_0108e7a8(cpu, 0x108e800); }
fn r_0108e7a8(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x108e7a8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)) & 0xfcu8; cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)) | cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  cpu.push(0x108e7c4); f_0109a0f2(cpu); if cpu.returned_to != 0x108e7c4 { return; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x72u32); }
  if !cpu.zf { pc = 0x108e7eb; continue; }
  pc = 0x108e7ce; continue;
 }
 0x108e7ce => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if cpu.zf { pc = 0x108e7da; continue; }
  pc = 0x108e7d5; continue;
 }
 0x108e7d5 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x108e7da; continue;
 }
 0x108e7da => {
  { cpu.and8((cpu.ebx as u8), 0x40u8); }
  if cpu.zf { pc = 0x108e7e4; continue; }
  pc = 0x108e7df; continue;
 }
 0x108e7df => {
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x108e7e7; continue;
 }
 0x108e7e4 => {
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x108e7e7; continue;
 }
 0x108e7e7 => {
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x108e816; continue;
 }
 0x108e7eb => {
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x21u32); cpu.eax = v; }
  { cpu.and8((cpu.ebx as u8), 0x80u8); }
  if cpu.zf { pc = 0x108e802; continue; }
  pc = 0x108e7fe; continue;
 }
 0x108e7fe => {
  { let v: u8 = (cpu.eax as u8) | 0x10u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x108e800; continue;
 }
 0x108e800 => {
  pc = 0x108e804; continue;
 }
 0x108e802 => {
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x108e804; continue;
 }
 0x108e804 => {
  { cpu.and8((cpu.ebx as u8), 0x40u8); }
  if cpu.zf { pc = 0x108e80e; continue; }
  pc = 0x108e809; continue;
 }
 0x108e809 => {
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x108e811; continue;
 }
 0x108e80e => {
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x108e811; continue;
 }
 0x108e811 => {
  { let v = 0x180u32; cpu.push(v); }
  pc = 0x108e816; continue;
 }
 0x108e816 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x108e81e); f_0108df22(cpu); if cpu.returned_to != 0x108e81e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ecx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x108e837; continue; }
  pc = 0x108e82a; continue;
 }
 0x108e82a => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x108e831); f_0109a1a6(cpu); if cpu.returned_to != 0x108e831 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x108e837 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { cpu.and8((cpu.ebx as u8), 0x80u8); }
  if cpu.zf { pc = 0x108e85f; continue; }
  pc = 0x108e851; continue;
 }
 0x108e851 => {
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x108e85f); f_0108f08a(cpu); if cpu.returned_to != 0x108e85f { return; }
  pc = 0x108e85f; continue;
 }
 0x108e85f => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x108e866); f_0109a1fd(cpu); if cpu.returned_to != 0x108e866 { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e86c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e86c); }
 let mut pc: u32 = 0x108e86c;
 loop { match pc {
 0x108e86c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x108e87a); f_0108e6d7(cpu); if cpu.returned_to != 0x108e87a { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108e8a1; continue; }
  pc = 0x108e880; continue;
 }
 0x108e880 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x108e887); f_0109a100(cpu); if cpu.returned_to != 0x108e887 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108e89f; continue; }
  pc = 0x108e88d; continue;
 }
 0x108e88d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x108e89d); f_0108e7a8(cpu); if cpu.returned_to != 0x108e89d { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x108e89f; continue;
 }
 0x108e89f => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x108e8a1; continue;
 }
 0x108e8a1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e8a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e8a5); }
 let mut pc: u32 = 0x108e8a5;
 loop { match pc {
 0x108e8a5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  cpu.push(0x108e8ad); f_0108e86c(cpu); if cpu.returned_to != 0x108e8ad { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e8af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e8af); }
 let mut pc: u32 = 0x108e8af;
 loop { match pc {
 0x108e8af => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11ee150u32); cpu.eax = v; }
  pc = 0x108e8b9; continue;
 }
 0x108e8b9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108e8e0; continue; }
  pc = 0x108e8bd; continue;
 }
 0x108e8bd => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if !cpu.zf { pc = 0x108e8dc; continue; }
  pc = 0x108e8c4; continue;
 }
 0x108e8c4 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x3u8); }
  if cpu.zf { pc = 0x108e8d6; continue; }
  pc = 0x108e8ca; continue;
 }
 0x108e8ca => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x108e8d6); f_0108f27b(cpu); if cpu.returned_to != 0x108e8d6 { return; }
  pc = 0x108e8d6; continue;
 }
 0x108e8d6 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108e8dc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.poll(); pc = 0x108e8b9; continue;
 }
 0x108e8e0 => {
  { let v: u32 = 0x11eca18u32; cpu.edx = v; }
  pc = 0x108e8e5; continue;
 }
 0x108e8e5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108e907; continue; }
  pc = 0x108e8eb; continue;
 }
 0x108e8eb => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x108e903; continue; }
  pc = 0x108e8f0; continue;
 }
 0x108e8f0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x11ee150u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ee150u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x108e8d6; continue;
 }
 0x108e903 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.poll(); pc = 0x108e8e5; continue;
 }
 0x108e907 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x108e911); f_01099923(cpu); if cpu.returned_to != 0x108e911 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108e959(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e959); }
 let mut pc: u32 = 0x108e959;
 loop { match pc {
 0x108e959 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x108e96c); f_01094bff(cpu); if cpu.returned_to != 0x108e96c { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x108e990; continue; }
  pc = 0x108e973; continue;
 }
 0x108e973 => {
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x108e981); f_01094bff(cpu); if cpu.returned_to != 0x108e981 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x108e98e); f_01094bff(cpu); if cpu.returned_to != 0x108e98e { return; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x108e990; continue;
 }
 0x108e990 => {
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
pub fn f_0108e996(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108e996); }
 let mut pc: u32 = 0x108e996;
 loop { match pc {
 0x108e996 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x108e9a7); f_010999ce(cpu); if cpu.returned_to != 0x108e9a7 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108e9c2; continue; }
  pc = 0x108e9ae; continue;
 }
 0x108e9ae => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x108e9b3; continue;
 }
 0x108e9b3 => {
  cpu.push(0x108e9b8); f_01099923(cpu); if cpu.returned_to != 0x108e9b8 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x108eb64; continue;
 }
 0x108e9c2 => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x108e9cd; continue; }
  pc = 0x108e9c6; continue;
 }
 0x108e9c6 => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.poll(); pc = 0x108e9b3; continue;
 }
 0x108e9cd => {
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x108ea04; continue; }
  pc = 0x108e9d1; continue;
 }
 0x108e9d1 => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0x42u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  { let v: u16 = cpu.rcl16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.ror16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x108ea04; continue; }
  pc = 0x108e9f4; continue;
 }
 0x108e9f4 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x108e9ff); f_0109995d(cpu); if cpu.returned_to != 0x108e9ff { return; }
  pc = 0x108eb64; continue;
 }
 0x108ea04 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x108ea3f; continue; }
  pc = 0x108ea0a; continue;
 }
 0x108ea0a => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x108e9f4; continue; }
  pc = 0x108ea24; continue;
 }
 0x108ea24 => {
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x108eb61; continue; }
  pc = 0x108ea2d; continue;
 }
 0x108ea2d => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x108ea37); f_01099923(cpu); if cpu.returned_to != 0x108ea37 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x108eb61; continue;
 }
 0x108ea3f => {
  cpu.push(0x108ea44); f_0109a22e(cpu); if cpu.returned_to != 0x108ea44 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if !cpu.cf { pc = 0x108ea52; continue; }
  pc = 0x108ea4d; continue;
 }
 0x108ea4d => {
  cpu.push(0x108ea52); f_0108eec1(cpu); if cpu.returned_to != 0x108ea52 { return; }
  pc = 0x108ea52; continue;
 }
 0x108ea52 => {
  { let v: u32 = 0x200u32; cpu.edi = v; }
  { cpu.sub32(cpu.edx, 0x230u32); }
  if !cpu.cf { pc = 0x108ea64; continue; }
  pc = 0x108ea5f; continue;
 }
 0x108ea5f => {
  { let v: u32 = 0x80u32; cpu.edi = v; }
  pc = 0x108ea64; continue;
 }
 0x108ea64 => {
  { let v: u32 = 0x3u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x108ea80; continue;
 }
 0x108ea80 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x108eb2b; continue; }
  pc = 0x108ea8c; continue;
 }
 0x108ea8c => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if !cpu.zf { pc = 0x108eae0; continue; }
  pc = 0x108ea98; continue;
 }
 0x108ea98 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x108eae0; continue; }
  pc = 0x108eaa4; continue;
 }
 0x108eaa4 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x108e9f4; continue; }
  pc = 0x108eac0; continue;
 }
 0x108eac0 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x108ead8; continue; }
  pc = 0x108eac4; continue;
 }
 0x108eac4 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x108eace); f_01099923(cpu); if cpu.returned_to != 0x108eace { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esi); cpu.eax = v; }
  pc = 0x108eb64; continue;
 }
 0x108ead8 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x108eae0; continue;
 }
 0x108eae0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x108ea80; continue; }
  pc = 0x108eafe; continue;
 }
 0x108eafe => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x108e9f4; continue; }
  pc = 0x108eb1a; continue;
 }
 0x108eb1a => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x108eac4; continue; }
  pc = 0x108eb1e; continue;
 }
 0x108eb1e => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x108ea80; continue;
 }
 0x108eb2b => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x108eb61; continue; }
  pc = 0x108eb2f; continue;
 }
 0x108eb2f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x108e9f4; continue; }
  pc = 0x108eb4c; continue;
 }
 0x108eb4c => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x108eb61; continue; }
  pc = 0x108eb50; continue;
 }
 0x108eb50 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x108eb5a); f_01099923(cpu); if cpu.returned_to != 0x108eb5a { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edi); cpu.eax = v; }
  pc = 0x108eb64; continue;
 }
 0x108eb61 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x108eb64; continue;
 }
 0x108eb64 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108eb6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108eb6b); }
 let mut pc: u32 = 0x108eb6b;
 loop { match pc {
 0x108eb6b => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x62u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108eb74); f_0108defe(cpu); if cpu.returned_to != 0x108eb74 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108eb78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108eb78); }
 let mut pc: u32 = 0x108eb78;
 loop { match pc {
 0x108eb78 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1133e90u32); cpu.ecx = v; }
  cpu.push(0x108eb92); f_0109a2af(cpu); if cpu.returned_to != 0x108eb92 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108eb96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108eb96); }
 let mut pc: u32 = 0x108eb96;
 loop { match pc {
 0x108eb96 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x108eba8); f_0109a5c2(cpu); if cpu.returned_to != 0x108eba8 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ec81(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ec81); }
 let mut pc: u32 = 0x108ec81;
 loop { match pc {
 0x108ec81 => {
  { let v: u8 = cpu.r8(0xfffc3u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0xfffc3u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0xfff000e3u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0xfff000e3u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x133e501du32.wrapping_add(cpu.ecx))); cpu.w32(0x133e501du32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  cpu.call(0x108eca3); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ee7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ee7c); }
 let mut pc: u32 = 0x108ee7c;
 loop { match pc {
 0x108ee7c => {
  { let v = cpu.r16(0x108ee85u32); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x108ee84); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ee85(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108ee85); } r_0108ee85(cpu, 0x108ee85); }
pub fn f_0108ee87(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108ee87); } r_0108ee85(cpu, 0x108ee87); }
pub fn f_0108ee88(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108ee88); } r_0108ee85(cpu, 0x108ee88); }
fn r_0108ee85(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x108ee85 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x108ee87; continue;
 }
 0x108ee87 => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffff8cu32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff8cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x1133c00u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108ee88 => {
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x1133c00u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ee90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ee90); }
 let mut pc: u32 = 0x108ee90;
 loop { match pc {
 0x108ee90 => {
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.esp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); } { let v: u32 = a; cpu.eax = v; } }
  cpu.push(0x108ee99); f_0108eea3(cpu); if cpu.returned_to != 0x108ee99 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108eea3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108eea3); }
 let mut pc: u32 = 0x108eea3;
 loop { match pc {
 0x108eea3 => {
  { cpu.sub32(cpu.eax, cpu.esp); }
  if !cpu.cf { pc = 0x108eeb4; continue; }
  pc = 0x108eea7; continue;
 }
 0x108eea7 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x1133e60u32)); }
  if (cpu.cf || cpu.zf) { pc = 0x108eeb4; continue; }
  pc = 0x108eeb3; continue;
 }
 0x108eeb3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108eeb4 => {
  { let v: u16 = (cpu.segments[2] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1133c00u32)); }
  if cpu.zf { f_0108eec1(cpu); return; }
  pc = 0x108eec0; continue;
 }
 0x108eec0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108eec1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108eec1); }
 let mut pc: u32 = 0x108eec1;
 loop { match pc {
 0x108eec1 => {
  { let v: u32 = 0x1133c02u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  cpu.push(0x108eed0); f_0109aa8f(cpu); if cpu.returned_to != 0x108eed0 { return; }
  f_0108eed0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108eed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108eed0); }
 let mut pc: u32 = 0x108eed0;
 loop { match pc {
 0x108eed0 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x2au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108eeeb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108eeeb); }
 let mut pc: u32 = 0x108eeeb;
 loop { match pc {
 0x108eeeb => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x2cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x3u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ef05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ef05); }
 let mut pc: u32 = 0x108ef05;
 loop { match pc {
 0x108ef05 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x3u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ef2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ef2a); }
 let mut pc: u32 = 0x108ef2a;
 loop { match pc {
 0x108ef2a => {
  { cpu.fpush(1.0); }
  { cpu.and8(cpu.r8(0x1133f74u32), 0x1u8); }
  if !cpu.zf { pc = 0x108ef39; continue; }
  pc = 0x108ef35; continue;
 }
 0x108ef35 => {
  { let v = cpu.st(1).atan2(cpu.st(0)); cpu.fpop(); cpu.set_st(0, v); }
  pc = 0x108ef3e; continue;
 }
 0x108ef39 => {
  cpu.push(0x108ef3e); f_0109aaac(cpu); if cpu.returned_to != 0x108ef3e { return; }
  pc = 0x108ef3e; continue;
 }
 0x108ef3e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ef3f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ef3f); }
 let mut pc: u32 = 0x108ef3f;
 loop { match pc {
 0x108ef3f => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.and8(cpu.r8(0x1133f74u32), 0x1u8); }
  if !cpu.zf { pc = 0x108ef4e; continue; }
  pc = 0x108ef4a; continue;
 }
 0x108ef4a => {
  { let v = cpu.st(1).atan2(cpu.st(0)); cpu.fpop(); cpu.set_st(0, v); }
  pc = 0x108ef53; continue;
 }
 0x108ef4e => {
  cpu.push(0x108ef53); f_0109aaac(cpu); if cpu.returned_to != 0x108ef53 { return; }
  pc = 0x108ef53; continue;
 }
 0x108ef53 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ef54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ef54); }
 let mut pc: u32 = 0x108ef54;
 loop { match pc {
 0x108ef54 => {
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x108ef5d); f_0108ef2a(cpu); if cpu.returned_to != 0x108ef5d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ef60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ef60); }
 let mut pc: u32 = 0x108ef60;
 loop { match pc {
 0x108ef60 => {
  { let v = cpu.rf64(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x108ef6d); f_0108ef3f(cpu); if cpu.returned_to != 0x108ef6d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ef70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ef70); }
 let mut pc: u32 = 0x108ef70;
 loop { match pc {
 0x108ef70 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  cpu.push(0x108ef80); f_0109aae0(cpu); if cpu.returned_to != 0x108ef80 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x108ef87); f_0108eb96(cpu); if cpu.returned_to != 0x108ef87 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108ef8d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108ef8d); }
 let mut pc: u32 = 0x108ef8d;
 loop { match pc {
 0x108ef8d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x108efa0; continue;
 }
 0x108efa0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x108efb8; continue; }
  pc = 0x108efa8; continue;
 }
 0x108efa8 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x108efa0; continue; }
  pc = 0x108efb8; continue;
 }
 0x108efb8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.flat_ds(0x108efbf); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108efc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108efc0); }
 let mut pc: u32 = 0x108efc0;
 loop { match pc {
 0x108efc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { cpu.cf = false; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x108f005); f_01099959(cpu); if cpu.returned_to != 0x108f005 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f00e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f00e); }
 let mut pc: u32 = 0x108f00e;
 loop { match pc {
 0x108f00e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x108f021); f_0109ab08(cpu); if cpu.returned_to != 0x108f021 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f024(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f024); }
 let mut pc: u32 = 0x108f024;
 loop { match pc {
 0x108f024 => {
  pc = 0x109ab32; continue;
 }
 0x109ab32 => {
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.fpu_control; cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x3u8); }
  if cpu.zf { pc = 0x109ab43; continue; }
  pc = 0x109ab42; continue;
 }
 0x109ab42 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x109ab43; continue;
 }
 0x109ab43 => {
  { let v: u32 = cpu.or32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if cpu.zf { pc = 0x109ab4c; continue; }
  pc = 0x109ab47; continue;
 }
 0x109ab47 => {
  cpu.push(0x109ab4c); f_0109ab62(cpu); if cpu.returned_to != 0x109ab4c { return; }
  pc = 0x109ab4c; continue;
 }
 0x109ab4c => {
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpu_control = cpu.r16(0x1134ed0u32); }
  { cpu.fpush(0.0); }
  { cpu.fpush(0.0); }
  { cpu.fpush(0.0); }
  { cpu.fpush(0.0); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f029(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f029); }
 let mut pc: u32 = 0x108f029;
 loop { match pc {
 0x108f029 => {
  pc = 0x109acb7; continue;
 }
 0x109acb7 => {
  { cpu.sub8(cpu.r8(0x1133fc0u32), 0x1u8); }
  if cpu.zf { pc = 0x109acc1; continue; }
  pc = 0x109acc0; continue;
 }
 0x109acc0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x109acc1 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x1133e7eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x1133e80u32), 0x0u8); }
  if cpu.zf { pc = 0x109acd9; continue; }
  pc = 0x109acd2; continue;
 }
 0x109acd2 => {
  cpu.push(0x109acd7); f_0109ad6c(cpu); if cpu.returned_to != 0x109acd7 { return; }
  pc = 0x109ad26; continue;
 }
 0x109acd9 => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if !cpu.zf { pc = 0x109acf2; continue; }
  pc = 0x109acdd; continue;
 }
 0x109acdd => {
  { let v: u32 = 0xe02u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1133bf8u32); cpu.ecx = v; }
  let t = cpu.r32(0x30u32.wrapping_add(cpu.ecx)); cpu.push(0x109acf0); cpu.call(t); if cpu.returned_to != 0x109acf0 { return; }
  pc = 0x109ad26; continue;
 }
 0x109acf2 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x109ad02; continue; }
  pc = 0x109acf6; continue;
 }
 0x109acf6 => {
  { let v: u16 = cpu.r16(0x1133fbeu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xf3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x109ad26; continue;
 }
 0x109ad02 => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x109ad16; continue; }
  pc = 0x109ad06; continue;
 }
 0x109ad06 => {
  { let v: u16 = cpu.r16(0x1133bfcu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x109ad14); f_010aa542(cpu); if cpu.returned_to != 0x109ad14 { return; }
  pc = 0x109ad26; continue;
 }
 0x109ad16 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  cpu.push(0x109ad1d); f_010aa542(cpu); if cpu.returned_to != 0x109ad1d { return; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x109ad26; continue; }
  pc = 0x109ad21; continue;
 }
 0x109ad21 => {
  cpu.push(0x109ad26); f_0109ad31(cpu); if cpu.returned_to != 0x109ad26 { return; }
  pc = 0x109ad26; continue;
 }
 0x109ad26 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1133fc0u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f030); }
 let mut pc: u32 = 0x108f030;
 loop { match pc {
 0x108f030 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  cpu.push(0x108f045); f_0109ae20(cpu); if cpu.returned_to != 0x108f045 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f048(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f048); }
 let mut pc: u32 = 0x108f048;
 loop { match pc {
 0x108f048 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.edx)) & 0xefu8; cpu.w8(0xcu32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x108f074; continue; }
  pc = 0x108f055; continue;
 }
 0x108f055 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x108f074; continue; }
  pc = 0x108f060; continue;
 }
 0x108f060 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108f074 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f08a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f08a); }
 let mut pc: u32 = 0x108f08a;
 loop { match pc {
 0x108f08a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.eax)), 0x6u8); }
  if cpu.zf { pc = 0x108f0f2; continue; }
  pc = 0x108f097; continue;
 }
 0x108f097 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ecx)), 0x10u8); }
  if cpu.zf { pc = 0x108f0c3; continue; }
  pc = 0x108f09d; continue;
 }
 0x108f09d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x108f0a4); f_0109aec3(cpu); if cpu.returned_to != 0x108f0a4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108f0d7; continue; }
  pc = 0x108f0a8; continue;
 }
 0x108f0a8 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x108f0ba; continue; }
  pc = 0x108f0ac; continue;
 }
 0x108f0ac => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x108f0ba; continue; }
  pc = 0x108f0b0; continue;
 }
 0x108f0b0 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x108f0ba); f_01099923(cpu); if cpu.returned_to != 0x108f0ba { return; }
  pc = 0x108f0ba; continue;
 }
 0x108f0ba => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108f0c3 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if !cpu.zf { pc = 0x108f0cb; continue; }
  pc = 0x108f0c8; continue;
 }
 0x108f0c8 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.ecx))); cpu.esi = v; }
  pc = 0x108f0cb; continue;
 }
 0x108f0cb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x108f0d7; continue;
 }
 0x108f0d7 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)) & 0xebu8; cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x108f0e5); f_01094bff(cpu); if cpu.returned_to != 0x108f0e5 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x108f180; continue; }
  pc = 0x108f0ee; continue;
 }
 0x108f0ee => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108f0f2 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.cf { pc = 0x108f100; continue; }
  pc = 0x108f0f7; continue;
 }
 0x108f0f7 => {
  if (cpu.cf || cpu.zf) { pc = 0x108f106; continue; }
  pc = 0x108f0f9; continue;
 }
 0x108f0f9 => {
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if cpu.zf { pc = 0x108f15c; continue; }
  pc = 0x108f0fe; continue;
 }
 0x108f0fe => {
  cpu.poll(); pc = 0x108f0b0; continue;
 }
 0x108f100 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x108f12b; continue; }
  pc = 0x108f104; continue;
 }
 0x108f104 => {
  cpu.poll(); pc = 0x108f0b0; continue;
 }
 0x108f106 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  cpu.push(0x108f112); f_0108f048(cpu); if cpu.returned_to != 0x108f112 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108f180; continue; }
  pc = 0x108f116; continue;
 }
 0x108f116 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edi); cpu.edx = v; }
  cpu.push(0x108f122); f_01094bff(cpu); if cpu.returned_to != 0x108f122 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x108f180; continue; }
  pc = 0x108f127; continue;
 }
 0x108f127 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108f12b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x108f133); f_0109af9c(cpu); if cpu.returned_to != 0x108f133 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x108f145); f_0108f048(cpu); if cpu.returned_to != 0x108f145 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108f180; continue; }
  pc = 0x108f149; continue;
 }
 0x108f149 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x108f153); f_01094bff(cpu); if cpu.returned_to != 0x108f153 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x108f180; continue; }
  pc = 0x108f158; continue;
 }
 0x108f158 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108f15c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.edx as u8) & 0xefu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x108f17b); f_01094bff(cpu); if cpu.returned_to != 0x108f17b { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x108f182; continue; }
  pc = 0x108f180; continue;
 }
 0x108f180 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x108f182; continue;
 }
 0x108f182 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f186(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f186); }
 let mut pc: u32 = 0x108f186;
 loop { match pc {
 0x108f186 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x108f192); f_0109af9c(cpu); if cpu.returned_to != 0x108f192 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x108f1c3; continue; }
  pc = 0x108f199; continue;
 }
 0x108f199 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x80u8); }
  if cpu.zf { pc = 0x108f1ac; continue; }
  pc = 0x108f19f; continue;
 }
 0x108f19f => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edx)), 0x10u8); }
  if cpu.zf { pc = 0x108f1ac; continue; }
  pc = 0x108f1a5; continue;
 }
 0x108f1a5 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x108f1ac); f_0109afad(cpu); if cpu.returned_to != 0x108f1ac { return; }
  pc = 0x108f1ac; continue;
 }
 0x108f1ac => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if cpu.zf { pc = 0x108f1c3; continue; }
  pc = 0x108f1b2; continue;
 }
 0x108f1b2 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edx)), 0x10u8); }
  if cpu.zf { pc = 0x108f1c0; continue; }
  pc = 0x108f1b8; continue;
 }
 0x108f1b8 => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108f1c0 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  pc = 0x108f1c3; continue;
 }
 0x108f1c3 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f1c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f1c8); }
 let mut pc: u32 = 0x108f1c8;
 loop { match pc {
 0x108f1c8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11ee150u32); cpu.eax = v; }
  pc = 0x108f1d1; continue;
 }
 0x108f1d1 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108f1dd; continue; }
  pc = 0x108f1d5; continue;
 }
 0x108f1d5 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108f1dd => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x108f1e6; continue; }
  pc = 0x108f1e2; continue;
 }
 0x108f1e2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.poll(); pc = 0x108f1d1; continue;
 }
 0x108f1e6 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x108f1f2); f_0108f1f5(cpu); if cpu.returned_to != 0x108f1f2 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f1f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f1f5); }
 let mut pc: u32 = 0x108f1f5;
 loop { match pc {
 0x108f1f5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x108f1fd); f_0108f27b(cpu); if cpu.returned_to != 0x108f1fd { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x108f206); f_0109a1a6(cpu); if cpu.returned_to != 0x108f206 { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f20a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f20a); }
 let mut pc: u32 = 0x108f20a;
 loop { match pc {
 0x108f20a => {
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x108f215; continue; }
  pc = 0x108f212; continue;
 }
 0x108f212 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x27u32); cpu.eax = v; }
  pc = 0x108f215; continue;
 }
 0x108f215 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f216(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f216); }
 let mut pc: u32 = 0x108f216;
 loop { match pc {
 0x108f216 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  cpu.push(0x108f223); f_0109afbd(cpu); if cpu.returned_to != 0x108f223 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u8 = 0x74u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  pc = 0x108f22d; continue;
 }
 0x108f22d => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  cpu.push(0x108f237); f_0108f20a(cpu); if cpu.returned_to != 0x108f237 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x108f22d; continue; }
  pc = 0x108f241; continue;
 }
 0x108f241 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u8 = 0x5fu8; cpu.w8(0x5u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x108f252); f_0108f20a(cpu); if cpu.returned_to != 0x108f252 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xfu32); cpu.eax = v; }
  cpu.push(0x108f25f); f_0108f20a(cpu); if cpu.returned_to != 0x108f25f { return; }
  { let v: u8 = 0x2eu8; cpu.w8(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x74u8; cpu.w8(0x9u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x6du8; cpu.w8(0xau32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x70u8; cpu.w8(0xbu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f27b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f27b); }
 let mut pc: u32 = 0x108f27b;
 loop { match pc {
 0x108f27b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x108f297; continue; }
  pc = 0x108f28d; continue;
 }
 0x108f28d => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x108f302; continue;
 }
 0x108f297 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x108f2aa; continue; }
  pc = 0x108f2a1; continue;
 }
 0x108f2a1 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x108f2a8); f_0109aec3(cpu); if cpu.returned_to != 0x108f2a8 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x108f2aa; continue;
 }
 0x108f2aa => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x108f2b1); f_0108f186(cpu); if cpu.returned_to != 0x108f2b1 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x108f2c4; continue; }
  pc = 0x108f2b6; continue;
 }
 0x108f2b6 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x108f2c4); f_01094bff(cpu); if cpu.returned_to != 0x108f2c4 { return; }
  pc = 0x108f2c4; continue;
 }
 0x108f2c4 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x108f2d2; continue; }
  pc = 0x108f2c8; continue;
 }
 0x108f2c8 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x108f2d0); f_01099a38(cpu); if cpu.returned_to != 0x108f2d0 { return; }
  { let v: u32 = cpu.esi | cpu.eax; cpu.esi = v; }
  pc = 0x108f2d2; continue;
 }
 0x108f2d2 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x8u8); }
  if cpu.zf { pc = 0x108f2e7; continue; }
  pc = 0x108f2d8; continue;
 }
 0x108f2d8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x108f2e0); f_01094b7d(cpu); if cpu.returned_to != 0x108f2e0 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x108f2e7; continue;
 }
 0x108f2e7 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ecx)), 0x8u8); }
  if cpu.zf { pc = 0x108f300; continue; }
  pc = 0x108f2ed; continue;
 }
 0x108f2ed => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x108f2f9); f_0108f216(cpu); if cpu.returned_to != 0x108f2f9 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x108f300); f_0109afc3(cpu); if cpu.returned_to != 0x108f300 { return; }
  pc = 0x108f300; continue;
 }
 0x108f300 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x108f302; continue;
 }
 0x108f302 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f30b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f30b); }
 let mut pc: u32 = 0x108f30b;
 loop { match pc {
 0x108f30b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x1u8); }
  if !cpu.zf { pc = 0x108f335; continue; }
  pc = 0x108f31b; continue;
 }
 0x108f31b => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x108f325); f_01099923(cpu); if cpu.returned_to != 0x108f325 { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x20u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x108f4df; continue;
 }
 0x108f335 => {
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x108f343; continue; }
  pc = 0x108f33c; continue;
 }
 0x108f33c => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x108f4df; continue;
 }
 0x108f343 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x108f350; continue; }
  pc = 0x108f349; continue;
 }
 0x108f349 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x108f350); f_0109afc8(cpu); if cpu.returned_to != 0x108f350 { return; }
  pc = 0x108f350; continue;
 }
 0x108f350 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and8((cpu.edx as u8), 0x40u8); }
  if cpu.zf { pc = 0x108f460; continue; }
  pc = 0x108f362; continue;
 }
 0x108f362 => {
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x108f366; continue;
 }
 0x108f366 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x108f3c1; continue; }
  pc = 0x108f36d; continue;
 }
 0x108f36d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x108f377; continue; }
  pc = 0x108f375; continue;
 }
 0x108f375 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  pc = 0x108f377; continue;
 }
 0x108f377 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x108f3c1; continue;
 }
 0x108f3c1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x108f4d4; continue; }
  pc = 0x108f3cd; continue;
 }
 0x108f3cd => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x14u32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x108f3dc; continue; }
  pc = 0x108f3d2; continue;
 }
 0x108f3d2 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x108f44c; continue; }
  pc = 0x108f3dc; continue;
 }
 0x108f3dc => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if !cpu.zf { pc = 0x108f402; continue; }
  pc = 0x108f3f5; continue;
 }
 0x108f3f5 => {
  { cpu.sub32(cpu.ebx, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x108f402; continue; }
  pc = 0x108f3fd; continue;
 }
 0x108f3fd => {
  { let v: u8 = (cpu.ebx as u8) ^ (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0xfeu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x108f402; continue;
 }
 0x108f402 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x108f40e); f_0109b03f(cpu); if cpu.returned_to != 0x108f40e { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x108f41c; continue; }
  pc = 0x108f413; continue;
 }
 0x108f413 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x108f4d4; continue;
 }
 0x108f41c => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108f429; continue; }
  pc = 0x108f420; continue;
 }
 0x108f420 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)) | 0x10u8; cpu.w8(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x108f4d4; continue;
 }
 0x108f429 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x108f366; continue;
 }
 0x108f44c => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x108f453); f_0109b116(cpu); if cpu.returned_to != 0x108f453 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108f4d4; continue; }
  pc = 0x108f45b; continue;
 }
 0x108f45b => {
  cpu.poll(); pc = 0x108f366; continue;
 }
 0x108f460 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  pc = 0x108f466; continue;
 }
 0x108f466 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x108f477; continue; }
  pc = 0x108f46c; continue;
 }
 0x108f46c => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x108f473); f_0109b116(cpu); if cpu.returned_to != 0x108f473 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108f4d4; continue; }
  pc = 0x108f477; continue;
 }
 0x108f477 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xdu32); }
  if !cpu.zf { pc = 0x108f4bb; continue; }
  pc = 0x108f493; continue;
 }
 0x108f493 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x108f4a4; continue; }
  pc = 0x108f499; continue;
 }
 0x108f499 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x108f4a0); f_0109b116(cpu); if cpu.returned_to != 0x108f4a0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108f4d4; continue; }
  pc = 0x108f4a4; continue;
 }
 0x108f4a4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x108f4bb; continue;
 }
 0x108f4bb => {
  { cpu.sub32(cpu.eax, 0x1au32); }
  if cpu.zf { cpu.poll(); pc = 0x108f420; continue; }
  pc = 0x108f4c4; continue;
 }
 0x108f4c4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x108f466; continue; }
  pc = 0x108f4d4; continue;
 }
 0x108f4d4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.esi; cpu.div32(v); }
  pc = 0x108f4df; continue;
 }
 0x108f4df => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f4e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f4e6); }
 let mut pc: u32 = 0x108f4e6;
 loop { match pc {
 0x108f4e6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108f519; continue; }
  pc = 0x108f4f2; continue;
 }
 0x108f4f2 => {
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x108f4f8; continue; }
  pc = 0x108f4f6; continue;
 }
 0x108f4f6 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  pc = 0x108f4f8; continue;
 }
 0x108f4f8 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x108f4ff); f_0109b1c2(cpu); if cpu.returned_to != 0x108f4ff { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x108f50c); f_0109b48f(cpu); if cpu.returned_to != 0x108f50c { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x108f515); f_0109b200(cpu); if cpu.returned_to != 0x108f515 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x108f519; continue;
 }
 0x108f519 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f51c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f51c); }
 let mut pc: u32 = 0x108f51c;
 loop { match pc {
 0x108f51c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x108f54f; continue; }
  pc = 0x108f530; continue;
 }
 0x108f530 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3au32); }
  if !cpu.zf { pc = 0x108f54f; continue; }
  pc = 0x108f53a; continue;
 }
 0x108f53a => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x108f54a; continue; }
  pc = 0x108f53e; continue;
 }
 0x108f53e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3au8; cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x108f54a; continue;
 }
 0x108f54a => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x108f556; continue;
 }
 0x108f54f => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x108f556; continue; }
  pc = 0x108f553; continue;
 }
 0x108f553 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x108f556; continue;
 }
 0x108f556 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  pc = 0x108f55c; continue;
 }
 0x108f55c => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x108f55e; continue;
 }
 0x108f55e => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x108f5a5; continue; }
  pc = 0x108f563; continue;
 }
 0x108f563 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x108f56a); f_0109b562(cpu); if cpu.returned_to != 0x108f56a { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x108f576; continue; }
  pc = 0x108f571; continue;
 }
 0x108f571 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x108f55e; continue;
 }
 0x108f576 => {
  { cpu.sub32(cpu.r32(0x1133fc4u32), 0x0u32); }
  if cpu.zf { pc = 0x108f592; continue; }
  pc = 0x108f57f; continue;
 }
 0x108f57f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1133fc9u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x108f594; continue;
 }
 0x108f592 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108f594; continue;
 }
 0x108f594 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0x5cu32); }
  if cpu.zf { pc = 0x108f5a1; continue; }
  pc = 0x108f59c; continue;
 }
 0x108f59c => {
  { cpu.sub32(cpu.edx, 0x2fu32); }
  if !cpu.zf { cpu.poll(); pc = 0x108f55e; continue; }
  pc = 0x108f5a1; continue;
 }
 0x108f5a1 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.poll(); pc = 0x108f55c; continue;
 }
 0x108f5a5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x81u32; cpu.ecx = v; }
  cpu.push(0x108f5be); f_0108f4e6(cpu); if cpu.returned_to != 0x108f5be { return; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x108f5c4; continue; }
  pc = 0x108f5c2; continue;
 }
 0x108f5c2 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x108f5c4; continue;
 }
 0x108f5c4 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebp); cpu.ebx = v; }
  cpu.push(0x108f5d8); f_0108f4e6(cpu); if cpu.returned_to != 0x108f5d8 { return; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  cpu.push(0x108f5ec); f_0108f4e6(cpu); if cpu.returned_to != 0x108f5ec { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f5f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f5f5); }
 let mut pc: u32 = 0x108f5f5;
 loop { match pc {
 0x108f5f5 => {
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x108f5f6; continue;
 }
 0x108f5f6 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x108f5fe; continue; }
  pc = 0x108f5fa; continue;
 }
 0x108f5fa => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108f5fe => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.ecx as u8), ((cpu.ecx >> 8) as u8)); }
  if cpu.zf { pc = 0x108f614; continue; }
  pc = 0x108f606; continue;
 }
 0x108f606 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108f614 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x108f5fa; continue; }
  pc = 0x108f619; continue;
 }
 0x108f619 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x108f5f6; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f61e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f61e); }
 let mut pc: u32 = 0x108f61e;
 loop { match pc {
 0x108f61e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xcfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi & 0x30u32; cpu.edi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebx), v); }
  pc = 0x108f63a; continue;
 }
 0x108f63a => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x108f65e; continue; }
  pc = 0x108f63f; continue;
 }
 0x108f63f => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x108f646); f_0109b05e(cpu); if cpu.returned_to != 0x108f646 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x108f65e; continue; }
  pc = 0x108f64e; continue;
 }
 0x108f64e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if !cpu.zf { cpu.poll(); pc = 0x108f63a; continue; }
  pc = 0x108f65e; continue;
 }
 0x108f65e => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x108f672; continue; }
  pc = 0x108f664; continue;
 }
 0x108f664 => {
  { cpu.sub32(cpu.edx, cpu.esi); }
  if cpu.zf { pc = 0x108f66e; continue; }
  pc = 0x108f668; continue;
 }
 0x108f668 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ebx)), 0x20u8); }
  if cpu.zf { pc = 0x108f672; continue; }
  pc = 0x108f66e; continue;
 }
 0x108f66e => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x108f675; continue;
 }
 0x108f672 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x108f675; continue;
 }
 0x108f675 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f687(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f687); }
 let mut pc: u32 = 0x108f687;
 loop { match pc {
 0x108f687 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x108f69d; continue; }
  pc = 0x108f694; continue;
 }
 0x108f694 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108f69d => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), 0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f6b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f6b0); }
 let mut pc: u32 = 0x108f6b0;
 loop { match pc {
 0x108f6b0 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x8u32.wrapping_add(cpu.edx))); cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f6b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f6b4); }
 let mut pc: u32 = 0x108f6b4;
 loop { match pc {
 0x108f6b4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = 0x108f687u32; cpu.ecx = v; }
  { let v: u32 = 0x108f6b0u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x108f6d5); f_0109b738(cpu); if cpu.returned_to != 0x108f6d5 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f6db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f6db); }
 let mut pc: u32 = 0x108f6db;
 loop { match pc {
 0x108f6db => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x108f6f6); f_0108f6b4(cpu); if cpu.returned_to != 0x108f6f6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f6fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f6fc); }
 let mut pc: u32 = 0x108f6fc;
 loop { match pc {
 0x108f6fc => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  pc = 0x108f702; continue;
 }
 0x108f702 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x108f70b; continue; }
  pc = 0x108f706; continue;
 }
 0x108f706 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108f70b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x108f71f; continue; }
  pc = 0x108f718; continue;
 }
 0x108f718 => {
  { cpu.sub32(cpu.ebx, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x108f71f; continue; }
  pc = 0x108f71d; continue;
 }
 0x108f71d => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x108f71f; continue;
 }
 0x108f71f => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x108f730; continue; }
  pc = 0x108f728; continue;
 }
 0x108f728 => {
  { cpu.sub32(cpu.ebx, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x108f730; continue; }
  pc = 0x108f72d; continue;
 }
 0x108f72d => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x108f730; continue;
 }
 0x108f730 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x108f746; continue; }
  pc = 0x108f734; continue;
 }
 0x108f734 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x108f746 => {
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x108f706; continue; }
  pc = 0x108f74a; continue;
 }
 0x108f74a => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x108f702; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f74f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f74f); }
 let mut pc: u32 = 0x108f74f;
 loop { match pc {
 0x108f74f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x1133c18u32), 0x0u32); }
  if !cpu.zf { pc = 0x108f7f3; continue; }
  pc = 0x108f75f; continue;
 }
 0x108f75f => {
  { let v = 0x1094a70u32; cpu.push(v); }
  { let v = 0x108f74fu32; cpu.push(v); }
  cpu.push(0x108f76e); f_01013d5e(cpu); if cpu.returned_to != 0x108f76e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ec5d4u32; cpu.push(v); }
  cpu.push(0x108f77d); f_01013dec(cpu); if cpu.returned_to != 0x108f77d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ec5d8u32; cpu.push(v); }
  cpu.push(0x108f78c); f_01013dec(cpu); if cpu.returned_to != 0x108f78c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ec5d0u32; cpu.push(v); }
  cpu.push(0x108f79b); f_01013dec(cpu); if cpu.returned_to != 0x108f79b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ec5dcu32; cpu.push(v); }
  cpu.push(0x108f7aa); f_01013dec(cpu); if cpu.returned_to != 0x108f7aa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ec5ecu32; cpu.push(v); }
  cpu.push(0x108f7b9); f_01013dec(cpu); if cpu.returned_to != 0x108f7b9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ec5e8u32; cpu.push(v); }
  cpu.push(0x108f7c8); f_01013dec(cpu); if cpu.returned_to != 0x108f7c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ec5f0u32; cpu.push(v); }
  cpu.push(0x108f7d7); f_01013dec(cpu); if cpu.returned_to != 0x108f7d7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x11ec5e0u32; cpu.push(v); }
  cpu.push(0x108f7e6); f_01013dec(cpu); if cpu.returned_to != 0x108f7e6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x1133c18u32, v); }
  pc = 0x108f7f3; continue;
 }
 0x108f7f3 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f7f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f7f7); }
 let mut pc: u32 = 0x108f7f7;
 loop { match pc {
 0x108f7f7 => {
  { let v: u32 = cpu.r32(0x11ec5dcu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x63u32); }
  if !cpu.zf { pc = 0x108f81d; continue; }
  pc = 0x108f802; continue;
 }
 0x108f802 => {
  { let v: u32 = cpu.r32(0x11ec5ecu32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x3bu32); }
  if !cpu.zf { pc = 0x108f81d; continue; }
  pc = 0x108f80d; continue;
 }
 0x108f80d => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x11ec5e8u32)); }
  if !cpu.zf { pc = 0x108f81d; continue; }
  pc = 0x108f815; continue;
 }
 0x108f815 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5f0u32)); }
  if cpu.zf { pc = 0x108f862; continue; }
  pc = 0x108f81d; continue;
 }
 0x108f81d => {
  { let v: u32 = cpu.inc32(cpu.r32(0x11ec5f0u32)); cpu.w32(0x11ec5f0u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5f0u32), 0x64u32); }
  if !cpu.zf { pc = 0x108f862; continue; }
  pc = 0x108f82c; continue;
 }
 0x108f82c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ec5f0u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11ec5e8u32)); cpu.w32(0x11ec5e8u32, v); }
  { let v: u32 = cpu.r32(0x11ec5e8u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x3cu32); }
  if !cpu.zf { pc = 0x108f862; continue; }
  pc = 0x108f844; continue;
 }
 0x108f844 => {
  { let v: u32 = cpu.eax; cpu.w32(0x11ec5e8u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11ec5ecu32)); cpu.w32(0x11ec5ecu32, v); }
  { cpu.sub32(cpu.ebx, cpu.r32(0x11ec5ecu32)); }
  if !cpu.zf { pc = 0x108f862; continue; }
  pc = 0x108f857; continue;
 }
 0x108f857 => {
  { let v: u32 = cpu.eax; cpu.w32(0x11ec5ecu32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11ec5dcu32)); cpu.w32(0x11ec5dcu32, v); }
  pc = 0x108f862; continue;
 }
 0x108f862 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108f863(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f863); }
 let mut pc: u32 = 0x108f863;
 loop { match pc {
 0x108f863 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  cpu.push(0x108f871); f_0109c35c(cpu); if cpu.returned_to != 0x108f871 { return; }
  { let v: u32 = cpu.r32(0x11ec5dcu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x11ec5ecu32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x11ec5e8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x11ec5f0u32); cpu.esi = v; }
  cpu.push(0x108f891); f_0109c369(cpu); if cpu.returned_to != 0x108f891 { return; }
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if !cpu.zf { pc = 0x108f8b8; continue; }
  pc = 0x108f89a; continue;
 }
 0x108f89a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x1137a10u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x108f8b3); f_01095199(cpu); if cpu.returned_to != 0x108f8b3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  pc = 0x108f8d6; continue;
 }
 0x108f8b8 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x108f8d1; continue;
 }
 0x108f8bc => {
  { let v = 0x1137a2bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x108f8cd); f_01095199(cpu); if cpu.returned_to != 0x108f8cd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x108f8d1; continue;
 }
 0x108f8d1 => {
  { cpu.sub32(cpu.edx, 0xeu32); }
  if cpu.cf { cpu.poll(); pc = 0x108f8bc; continue; }
  pc = 0x108f8d6; continue;
 }
 0x108f8d6 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x108f8f2; continue;
 }
 0x108f8dd => {
  { let v = 0x1137a2du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x108f8ee); f_01095199(cpu); if cpu.returned_to != 0x108f8ee { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x108f8f2; continue;
 }
 0x108f8f2 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x11ec5e4u32)); }
  if cpu.cf { cpu.poll(); pc = 0x108f8dd; continue; }
  pc = 0x108f8fa; continue;
 }
 0x108f8fa => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
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
pub fn f_0108f909(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x108f909); }
 let mut pc: u32 = 0x108f909;
 loop { match pc {
 0x108f909 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  cpu.push(0x108f914); f_0108f74f(cpu); if cpu.returned_to != 0x108f914 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5d4u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5d8u32, v); }
  { let v: u32 = 0x1137a2fu32; cpu.eax = v; }
  cpu.push(0x108f92c); f_0108cfa7(cpu); if cpu.returned_to != 0x108f92c { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108f93c; continue; }
  pc = 0x108f932; continue;
 }
 0x108f932 => {
  cpu.push(0x108f937); f_0109d4ef(cpu); if cpu.returned_to != 0x108f937 { return; }
  pc = 0x108fa7e; continue;
 }
 0x108f93c => {
  { let v: u32 = 0x1137a39u32; cpu.eax = v; }
  cpu.push(0x108f946); f_0108cfa7(cpu); if cpu.returned_to != 0x108f946 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108f954; continue; }
  pc = 0x108f94a; continue;
 }
 0x108f94a => {
  { let v: u32 = 0x1u32; cpu.w32(0x11ec5d8u32, v); }
  pc = 0x108f954; continue;
 }
 0x108f954 => {
  { let v: u32 = 0x1137a47u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x108f960); f_0108e8a5(cpu); if cpu.returned_to != 0x108f960 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ec5d0u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x108f932; continue; }
  pc = 0x108f969; continue;
 }
 0x108f969 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x108f971); f_010998f7(cpu); if cpu.returned_to != 0x108f971 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108f981; continue; }
  pc = 0x108f975; continue;
 }
 0x108f975 => {
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x108f981); f_0109d6b5(cpu); if cpu.returned_to != 0x108f981 { return; }
  pc = 0x108f981; continue;
 }
 0x108f981 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x108f988); f_0109d6d2(cpu); if cpu.returned_to != 0x108f988 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x108f98f); f_0109d752(cpu); if cpu.returned_to != 0x108f98f { return; }
  cpu.push(0x108f994); f_0109d89d(cpu); if cpu.returned_to != 0x108f994 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x11ec5f4u32; cpu.eax = v; }
  cpu.push(0x108f9a0); f_0108e30e(cpu); if cpu.returned_to != 0x108f9a0 { return; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x11ec60cu32, v); }
  { let v = 0x1137a4bu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x108f9b9); f_01095199(cpu); if cpu.returned_to != 0x108f9b9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1137a9cu32; cpu.push(v); }
  { let v = 0x1137aa1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x108f9d2); f_01095199(cpu); if cpu.returned_to != 0x108f9d2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x11ec5f4u32; cpu.push(v); }
  { let v = 0x1137ae8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x108f9eb); f_01095199(cpu); if cpu.returned_to != 0x108f9eb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x1137af8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108f9fe); f_01095199(cpu); if cpu.returned_to != 0x108f9fe { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  cpu.push(0x108fa06); f_0109d4ef(cpu); if cpu.returned_to != 0x108fa06 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5dcu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5ecu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e8u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5f0u32, v); }
  { let v = 0x108f7f7u32; cpu.push(v); }
  cpu.push(0x108fa2a); f_0109cb1e(cpu); if cpu.returned_to != 0x108fa2a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11ec5e0u32, v); }
  { let v = 0x64u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x108fa3a); f_0109ccb0(cpu); if cpu.returned_to != 0x108fa3a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11ec5e0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x108fa49); f_0109cbd8(cpu); if cpu.returned_to != 0x108fa49 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x11ec5d4u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11ec5e4u32, v); }
  cpu.push(0x108fa62); f_0108f863(cpu); if cpu.returned_to != 0x108fa62 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { let v = 0x1137b4au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x108fa7b); f_01095199(cpu); if cpu.returned_to != 0x108fa7b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x108fa7e; continue;
 }
 0x108fa7e => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0108fa85(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x108fa85); } r_0108fa85(cpu, 0x108fa85); }
pub fn f_010907fd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10907fd); } r_0108fa85(cpu, 0x10907fd); }
pub fn f_0109099b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x109099b); } r_0108fa85(cpu, 0x109099b); }
fn r_0108fa85(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x108fa85 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x108fac3; continue; }
  pc = 0x108fa9c; continue;
 }
 0x108fa9c => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x108faaa; continue; }
  pc = 0x108faa1; continue;
 }
 0x108faa1 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x108fac3; continue; }
  pc = 0x108faaa; continue;
 }
 0x108faaa => {
  cpu.push(0x108faaf); f_0109cb12(cpu); if cpu.returned_to != 0x108faaf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108fac3; continue; }
  pc = 0x108fab3; continue;
 }
 0x108fab3 => {
  cpu.push(0x108fab8); f_0108f863(cpu); if cpu.returned_to != 0x108fab8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108fac3; continue; }
  pc = 0x108fabc; continue;
 }
 0x108fabc => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x108fac5; continue;
 }
 0x108fac3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x108fac5; continue;
 }
 0x108fac5 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x108fadd; continue; }
  pc = 0x108fac9; continue;
 }
 0x108fac9 => {
  { let v = 0x1137b59u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x108fada); f_01095199(cpu); if cpu.returned_to != 0x108fada { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x108fadd; continue;
 }
 0x108fadd => {
  cpu.push(0x108fae2); f_0109d601(cpu); if cpu.returned_to != 0x108fae2 { return; }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x108fb10; continue; }
  pc = 0x108faeb; continue;
 }
 0x108faeb => {
  { cpu.sub32(cpu.r32(0x11ec5e4u32), 0x1u32); }
  if cpu.zf { pc = 0x108fafd; continue; }
  pc = 0x108faf4; continue;
 }
 0x108faf4 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x108fb10; continue; }
  pc = 0x108fafd; continue;
 }
 0x108fafd => {
  cpu.push(0x108fb02); f_0109cb12(cpu); if cpu.returned_to != 0x108fb02 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x108fb10; continue; }
  pc = 0x108fb06; continue;
 }
 0x108fb06 => {
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.eax = v; }
  cpu.push(0x108fb10); f_0108f1c8(cpu); if cpu.returned_to != 0x108fb10 { return; }
  pc = 0x108fb10; continue;
 }
 0x108fb10 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11ec5e4u32)); cpu.w32(0x11ec5e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10907fd => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x109083b; continue; }
  pc = 0x1090814; continue;
 }
 0x1090814 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x1090822; continue; }
  pc = 0x1090819; continue;
 }
 0x1090819 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x109083b; continue; }
  pc = 0x1090822; continue;
 }
 0x1090822 => {
  cpu.push(0x1090827); f_0109cb12(cpu); if cpu.returned_to != 0x1090827 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x109083b; continue; }
  pc = 0x109082b; continue;
 }
 0x109082b => {
  cpu.push(0x1090830); f_0108f863(cpu); if cpu.returned_to != 0x1090830 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x109083b; continue; }
  pc = 0x1090834; continue;
 }
 0x1090834 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x109083d; continue;
 }
 0x109083b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x109083d; continue;
 }
 0x109083d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1090855; continue; }
  pc = 0x1090841; continue;
 }
 0x1090841 => {
  { let v = 0x1137d7au32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1090852); f_01095199(cpu); if cpu.returned_to != 0x1090852 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x1090855; continue;
 }
 0x1090855 => {
  cpu.push(0x109085a); f_0109cc08(cpu); if cpu.returned_to != 0x109085a { return; }
  cpu.poll(); pc = 0x108fb10; continue;
 }
 0x109099b => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11ec5e4u32, v); }
  { cpu.sub32(cpu.r32(0x11ec5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10909d9; continue; }
  pc = 0x10909b2; continue;
 }
 0x10909b2 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10909c0; continue; }
  pc = 0x10909b7; continue;
 }
 0x10909b7 => {
  { cpu.sub32(cpu.r32(0x11ec5d8u32), 0x0u32); }
  if cpu.zf { pc = 0x10909d9; continue; }
  pc = 0x10909c0; continue;
 }
 0x10909c0 => {
  cpu.push(0x10909c5); f_0109cb12(cpu); if cpu.returned_to != 0x10909c5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10909d9; continue; }
  pc = 0x10909c9; continue;
 }
 0x10909c9 => {
  cpu.push(0x10909ce); f_0108f863(cpu); if cpu.returned_to != 0x10909ce { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10909d9; continue; }
  pc = 0x10909d2; continue;
 }
 0x10909d2 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10909db; continue;
 }
 0x10909d9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10909db; continue;
 }
 0x10909db => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10909f3; continue; }
  pc = 0x10909df; continue;
 }
 0x10909df => {
  { let v = 0x1137ddbu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11ec5d0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10909f0); f_01095199(cpu); if cpu.returned_to != 0x10909f0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x10909f3; continue;
 }
 0x10909f3 => {
  cpu.push(0x10909f8); f_0109cbb4(cpu); if cpu.returned_to != 0x10909f8 { return; }
  cpu.poll(); pc = 0x108fb10; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
