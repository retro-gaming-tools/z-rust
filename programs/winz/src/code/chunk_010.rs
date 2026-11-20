#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_0046c82b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46c82b); }
 let mut pc: u32 = 0x46c82b;
 loop { match pc {
 0x46c82b => {
  { cpu.movs(4, false); }
  { cpu.sub32(cpu.edi, 0x4e154cu32); }
  if cpu.cf { cpu.poll(); pc = 0x46c82b; continue; }
  pc = 0x46c834; continue;
 }
 0x46c834 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046c838(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46c838); }
 let mut pc: u32 = 0x46c838;
 loop { match pc {
 0x46c838 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046cc53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46cc53); }
 let mut pc: u32 = 0x46cc53;
 loop { match pc {
 0x46cc53 => {
  { let v: u8 = 0x0u8; cpu.w8(0x58c432u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c433u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b8u32, v); }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x46cc72); f_0041c9df(cpu); if cpu.returned_to != 0x46cc72 { return; }
  { cpu.sub8(cpu.r8(0x5ce8d2u32), 0x1u8); }
  if cpu.zf { pc = 0x46ccff; continue; }
  pc = 0x46cc7f; continue;
 }
 0x46cc7f => {
  { let v: u32 = 0xau32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0xffffu32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x46cc98); f_0041c9df(cpu); if cpu.returned_to != 0x46cc98 { return; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v: u32 = 0xc8u32; cpu.edx = v; }
  cpu.push(0x46ccac); f_0041c9df(cpu); if cpu.returned_to != 0x46ccac { return; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  { let v: u32 = 0xfu32; cpu.eax = v; }
  cpu.push(0x46ccc0); f_0041c9df(cpu); if cpu.returned_to != 0x46ccc0 { return; }
  { let v: u32 = 0x13u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x46cccf); f_0041c9df(cpu); if cpu.returned_to != 0x46cccf { return; }
  { let v: u32 = 0x40d660u32; cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x46cce1); f_00402f20(cpu); if cpu.returned_to != 0x46cce1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c64u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d69dcu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x581a72u32, v); }
  pc = 0x46ccff; continue;
 }
 0x46ccff => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0x278u32; cpu.edx = v; }
  cpu.push(0x46cd13); f_0041c9df(cpu); if cpu.returned_to != 0x46cd13 { return; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0xfffffff8u32; cpu.ecx = v; }
  { let v: u32 = 0x188u32; cpu.edx = v; }
  cpu.push(0x46cd27); f_0041c9df(cpu); if cpu.returned_to != 0x46cd27 { return; }
  cpu.push(0x46cd2c); f_0046d178(cpu); if cpu.returned_to != 0x46cd2c { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1fa4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5ce8d2u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046cd7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46cd7f); }
 let mut pc: u32 = 0x46cd7f;
 loop { match pc {
 0x46cd7f => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x46cd88); f_00401080(cpu); if cpu.returned_to != 0x46cd88 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x46cd90); f_004010e0(cpu); if cpu.returned_to != 0x46cd90 { return; }
  { cpu.popad(); }
  { let v: u8 = 0x5u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x28u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x46u32; cpu.ecx = v; }
  { let v: u16 = 0x7u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x46cda5); f_0046d22a(cpu); if cpu.returned_to != 0x46cda5 { return; }
  { let v: u8 = 0x7u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x2au8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x59d9f8u32; cpu.esi = v; }
  { let v: u8 = 0xau8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46cdb7); f_0046d05f(cpu); if cpu.returned_to != 0x46cdb7 { return; }
  { let v: u8 = 0x7u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x2bu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x59da2du32; cpu.esi = v; }
  { let v: u8 = 0xau8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46cdc9); f_0046d05f(cpu); if cpu.returned_to != 0x46cdc9 { return; }
  { let v: u8 = 0x14u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x2au8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x5ce8d3u32; cpu.esi = v; }
  { let v: u8 = 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46cddb); f_0046d05f(cpu); if cpu.returned_to != 0x46cddb { return; }
  { let v: u8 = 0x14u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x2bu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x5ce90fu32); cpu.esi = v; }
  { let v: u8 = 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46cdee); f_0046d05f(cpu); if cpu.returned_to != 0x46cdee { return; }
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xfu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u16 = 0x14u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xc0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x46ce02); f_0046d22a(cpu); if cpu.returned_to != 0x46ce02 { return; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x4e1f7cu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5add98u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u8 = 0x18u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x11u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46ce1c); f_0046d05f(cpu); if cpu.returned_to != 0x46ce1c { return; }
  { let v: u32 = 0x5cca28u32; cpu.esi = v; }
  { let v: u32 = 0x7u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5cd0a8u32); cpu.eax = v; }
  cpu.push(0x46ce2e); f_0046d019(cpu); if cpu.returned_to != 0x46ce2e { return; }
  { let v: u32 = 0x15u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5cd0a8u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x40cf50u32); cpu.eax = v; }
  cpu.push(0x46ce40); f_0046d019(cpu); if cpu.returned_to != 0x46ce40 { return; }
  { let v: u8 = 0x1au8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x13u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46ce49); f_0046d05f(cpu); if cpu.returned_to != 0x46ce49 { return; }
  cpu.push(0x46ce4e); f_0046cf0f(cpu); if cpu.returned_to != 0x46ce4e { return; }
  { let v: u8 = 0xeu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x15u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x5cce7au32; cpu.esi = v; }
  { let v: u8 = 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46ce60); f_0046d05f(cpu); if cpu.returned_to != 0x46ce60 { return; }
  { let v: u8 = 0xeu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x16u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x5cceaeu32; cpu.esi = v; }
  { let v: u8 = 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46ce72); f_0046d05f(cpu); if cpu.returned_to != 0x46ce72 { return; }
  { let v: u8 = 0xeu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x17u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x5ccee2u32; cpu.esi = v; }
  { let v: u8 = 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46ce84); f_0046d05f(cpu); if cpu.returned_to != 0x46ce84 { return; }
  { let v: u8 = 0xeu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x18u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x5cceffu32; cpu.esi = v; }
  { let v: u8 = 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46ce96); f_0046d05f(cpu); if cpu.returned_to != 0x46ce96 { return; }
  { let v: u8 = 0xeu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x5ccf2eu32; cpu.esi = v; }
  { let v: u8 = 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46cea8); f_0046d05f(cpu); if cpu.returned_to != 0x46cea8 { return; }
  { let v: u8 = 0xeu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1au8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x5ccf5du32; cpu.esi = v; }
  { let v: u8 = 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46ceba); f_0046d05f(cpu); if cpu.returned_to != 0x46ceba { return; }
  { let v: u8 = 0xeu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x5ccf8cu32; cpu.esi = v; }
  { let v: u8 = 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46cecc); f_0046d05f(cpu); if cpu.returned_to != 0x46cecc { return; }
  { let v: u8 = 0xeu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x5ccfbbu32; cpu.esi = v; }
  { let v: u8 = 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46cede); f_0046d05f(cpu); if cpu.returned_to != 0x46cede { return; }
  { let v: u8 = 0xeu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x5ccfeau32; cpu.esi = v; }
  { let v: u8 = 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46cef0); f_0046d05f(cpu); if cpu.returned_to != 0x46cef0 { return; }
  { let v: u8 = 0xeu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x5cd019u32; cpu.esi = v; }
  { let v: u8 = 0xeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x46cf02); f_0046d05f(cpu); if cpu.returned_to != 0x46cf02 { return; }
  cpu.push(0x46cf07); f_0041c0f6(cpu); if cpu.returned_to != 0x46cf07 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046cf0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46cf0f); }
 let mut pc: u32 = 0x46cf0f;
 loop { match pc {
 0x46cf0f => {
  { let v: u32 = 0x5cce7eu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5ce91cu32); cpu.eax = v; }
  cpu.push(0x46cf1e); f_0046d019(cpu); if cpu.returned_to != 0x46cf1e { return; }
  { let v: u32 = 0x5cce8bu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5ce920u32); cpu.eax = v; }
  cpu.push(0x46cf2d); f_0046d019(cpu); if cpu.returned_to != 0x46cf2d { return; }
  { let v: u32 = 0x5cce98u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5ce924u32); cpu.eax = v; }
  cpu.push(0x46cf3c); f_0046d019(cpu); if cpu.returned_to != 0x46cf3c { return; }
  { let v: u32 = 0x5ccea5u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5ce928u32); cpu.eax = v; }
  cpu.push(0x46cf4b); f_0046d019(cpu); if cpu.returned_to != 0x46cf4b { return; }
  { let v: u32 = 0x5cceb2u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5ce92cu32); cpu.eax = v; }
  cpu.push(0x46cf5a); f_0046d019(cpu); if cpu.returned_to != 0x46cf5a { return; }
  { let v: u32 = 0x5ccebfu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5ce930u32); cpu.eax = v; }
  cpu.push(0x46cf69); f_0046d019(cpu); if cpu.returned_to != 0x46cf69 { return; }
  { let v: u32 = 0x5cceccu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5ce934u32); cpu.eax = v; }
  cpu.push(0x46cf78); f_0046d019(cpu); if cpu.returned_to != 0x46cf78 { return; }
  { let v: u32 = 0x5cced9u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5ce938u32); cpu.eax = v; }
  cpu.push(0x46cf87); f_0046d019(cpu); if cpu.returned_to != 0x46cf87 { return; }
  { let v: u32 = 0x5ccee7u32; cpu.edi = v; }
  { let v: u32 = 0x40cf50u32; cpu.eax = v; }
  cpu.push(0x46cf96); f_0046d019(cpu); if cpu.returned_to != 0x46cf96 { return; }
  { let v: u32 = 0x5ccef5u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  cpu.push(0x46cfa5); f_0046d019(cpu); if cpu.returned_to != 0x46cfa5 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d019(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d019); }
 let mut pc: u32 = 0x46d019;
 loop { match pc {
 0x46d019 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x7u32); cpu.edi = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x46d026; continue;
 }
 0x46d026 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.sub32(cpu.edx, 0x9u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x46d034; continue; }
  pc = 0x46d02f; continue;
 }
 0x46d02f => {
  { let v: u32 = cpu.add32(cpu.edx, 0x37u32); cpu.edx = v; }
  pc = 0x46d037; continue;
 }
 0x46d034 => {
  { let v: u32 = cpu.add32(cpu.edx, 0x30u32); cpu.edx = v; }
  pc = 0x46d037; continue;
 }
 0x46d037 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x46d026; continue; }
  pc = 0x46d03c; continue;
 }
 0x46d03c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d03d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d03d); }
 let mut pc: u32 = 0x46d03d;
 loop { match pc {
 0x46d03d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4b928cu32); cpu.ecx = v; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d05f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d05f); }
 let mut pc: u32 = 0x46d05f;
 loop { match pc {
 0x46d05f => {
  cpu.push(0x46d064); f_0046d03d(cpu); if cpu.returned_to != 0x46d064 { return; }
  cpu.push(0x46d069); f_0046d0b2(cpu); if cpu.returned_to != 0x46d069 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d0b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d0b2); }
 let mut pc: u32 = 0x46d0b2;
 loop { match pc {
 0x46d0b2 => {
  cpu.push(0x46d0b7); f_0046d1a8(cpu); if cpu.returned_to != 0x46d0b7 { return; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4b9290u32)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x5ce913u32)); cpu.edi = v; }
  pc = 0x46d0c6; continue;
 }
 0x46d0c6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x46d0fb; continue; }
  pc = 0x46d0cd; continue;
 }
 0x46d0cd => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x46d0fb; continue; }
  pc = 0x46d0d1; continue;
 }
 0x46d0d1 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x46d11f; continue; }
  pc = 0x46d0d5; continue;
 }
 0x46d0d5 => {
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.zf { cpu.poll(); pc = 0x46d0c6; continue; }
  pc = 0x46d0d9; continue;
 }
 0x46d0d9 => {
  { cpu.sub8((cpu.eax as u8), 0x24u8); }
  if cpu.zf { pc = 0x46d0fb; continue; }
  pc = 0x46d0dd; continue;
 }
 0x46d0dd => {
  { cpu.sub8((cpu.eax as u8), 0x80u8); }
  if cpu.cf { pc = 0x46d0e5; continue; }
  pc = 0x46d0e1; continue;
 }
 0x46d0e1 => {
  { cpu.sub8((cpu.eax as u8), 0xa0u8); }
  if (cpu.cf || cpu.zf) { pc = 0x46d155; continue; }
  pc = 0x46d0e5; continue;
 }
 0x46d0e5 => {
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x46d10f; continue; }
  pc = 0x46d0e9; continue;
 }
 0x46d0e9 => {
  { cpu.sub8((cpu.eax as u8), 0x11u8); }
  if !cpu.zf { pc = 0x46d0ef; continue; }
  pc = 0x46d0ed; continue;
 }
 0x46d0ed => {
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x46d0ef; continue;
 }
 0x46d0ef => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x46d0fb; continue; }
  pc = 0x46d0f9; continue;
 }
 0x46d0f9 => {
  cpu.poll(); pc = 0x46d0c6; continue;
 }
 0x46d0fb => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x4b9290u32)); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.r32(0x5ce913u32)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x46d10d); f_0046d178(cpu); if cpu.returned_to != 0x46d10d { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x46d10f => {
  { cpu.sub16((cpu.ecx as u16), 0x2u16); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x46d0fb; continue; }
  pc = 0x46d115; continue;
 }
 0x46d115 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x2u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x46d0c6; continue;
 }
 0x46d11f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4b9290u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5ce913u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4b928cu32); cpu.ebx = v; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4b9290u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5ce913u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.poll(); pc = 0x46d0c6; continue;
 }
 0x46d155 => {
  { cpu.sub8((cpu.eax as u8), 0x90u8); }
  if cpu.cf { pc = 0x46d16a; continue; }
  pc = 0x46d159; continue;
 }
 0x46d159 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x90u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) | (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.poll(); pc = 0x46d0c6; continue;
 }
 0x46d16a => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x80u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xf0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) | (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.poll(); pc = 0x46d0c6; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d178(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d178); }
 let mut pc: u32 = 0x46d178;
 loop { match pc {
 0x46d178 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x4e15b5u32), 0x0u8); }
  if cpu.zf { pc = 0x46d1a6; continue; }
  pc = 0x46d182; continue;
 }
 0x46d182 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x46d18c); f_0041c9df(cpu); if cpu.returned_to != 0x46d18c { return; }
  { cpu.sub8(cpu.r8(0x4e1fc4u32), 0x0u8); }
  if !cpu.zf { pc = 0x46d1a6; continue; }
  pc = 0x46d195; continue;
 }
 0x46d195 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c64u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e1fc4u32, v); }
  pc = 0x46d1a6; continue;
 }
 0x46d1a6 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d1a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d1a8); }
 let mut pc: u32 = 0x46d1a8;
 loop { match pc {
 0x46d1a8 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x4e15b5u32), 0x0u8); }
  if cpu.zf { pc = 0x46d1e0; continue; }
  pc = 0x46d1b2; continue;
 }
 0x46d1b2 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x46d1bc); f_0041c9df(cpu); if cpu.returned_to != 0x46d1bc { return; }
  { let v: u32 = cpu.r32(0x4d6c64u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x46d1e0; continue; }
  pc = 0x46d1c6; continue;
 }
 0x46d1c6 => {
  { let v: u16 = cpu.r16(0x4d69dcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6c64u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1fc4u32, v); }
  pc = 0x46d1e0; continue;
 }
 0x46d1e0 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d22a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d22a); }
 let mut pc: u32 = 0x46d22a;
 loop { match pc {
 0x46d22a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x46d233); f_0046d1a8(cpu); if cpu.returned_to != 0x46d233 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x46d239); f_0046d03d(cpu); if cpu.returned_to != 0x46d239 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x5ce913u32)); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4b928cu32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.stos(2, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0x2u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x46d25c; continue;
 }
 0x46d25c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x2u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { cpu.stos(2, true); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46d25c; continue; }
  pc = 0x46d278; continue;
 }
 0x46d278 => {
  { cpu.stos(2, true); }
  cpu.push(0x46d280); f_0046d178(cpu); if cpu.returned_to != 0x46d280 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d281(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d281); }
 let mut pc: u32 = 0x46d281;
 loop { match pc {
 0x46d281 => {
  cpu.push(0x46d286); f_0046d1a8(cpu); if cpu.returned_to != 0x46d286 { return; }
  { let v: u32 = cpu.r32(0x5ce913u32); cpu.edi = v; }
  { let v: u16 = 0x720u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xfa0u32; cpu.ecx = v; }
  { cpu.stos(2, true); }
  { let v: u32 = cpu.r32(0x5ce913u32); cpu.edi = v; }
  { let v: u16 = 0x1720u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { cpu.stos(2, true); }
  { let v: u32 = cpu.r32(0x5ce913u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1540u32); cpu.edi = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { cpu.stos(2, true); }
  { let v: u32 = cpu.r32(0x5ce913u32); cpu.edi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4b928cu32); cpu.ebx = v; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x46d2d4; continue;
 }
 0x46d2d4 => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x46d2d4; continue; }
  pc = 0x46d2da; continue;
 }
 0x46d2da => {
  { let v: u32 = cpu.r32(0x5ce913u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x9eu32); cpu.edi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x46d2eb; continue;
 }
 0x46d2eb => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x46d2eb; continue; }
  pc = 0x46d2f1; continue;
 }
 0x46d2f1 => {
  { let v: u32 = cpu.r32(0x5ce913u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4eu32); cpu.edi = v; }
  { let v: u32 = 0x22u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x46d303; continue;
 }
 0x46d303 => {
  { cpu.stos(2, false); }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x46d303; continue; }
  pc = 0x46d30b; continue;
 }
 0x46d30b => {
  { let v: u16 = 0x4720u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x5ce913u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1680u32); cpu.edi = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { cpu.stos(2, true); }
  { let v: u32 = cpu.r32(0x5ce913u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1ea0u32); cpu.edi = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { cpu.stos(2, true); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ce913u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x17beu32); cpu.edi = v; }
  pc = 0x46d34c; continue;
 }
 0x46d34c => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x46d34c; continue; }
  pc = 0x46d352; continue;
 }
 0x46d352 => {
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ce913u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x176eu32); cpu.edi = v; }
  pc = 0x46d363; continue;
 }
 0x46d363 => {
  { cpu.stos(2, false); }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edi.wrapping_sub(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x46d363; continue; }
  pc = 0x46d36e; continue;
 }
 0x46d36e => {
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5ce913u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1720u32); cpu.edi = v; }
  pc = 0x46d37f; continue;
 }
 0x46d37f => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x46d37f; continue; }
  pc = 0x46d385; continue;
 }
 0x46d385 => {
  cpu.push(0x46d38a); f_0046d178(cpu); if cpu.returned_to != 0x46d38a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d38b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d38b); }
 let mut pc: u32 = 0x46d38b;
 loop { match pc {
 0x46d38b => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x46d398); f_00402d70(cpu); if cpu.returned_to != 0x46d398 { return; }
  cpu.push(0x46d39d); f_00405320(cpu); if cpu.returned_to != 0x46d39d { return; }
  { cpu.popad(); }
  cpu.push(0x46d3a3); f_004478a1(cpu); if cpu.returned_to != 0x46d3a3 { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.push(0x46d3ad); f_0047ae6f(cpu); if cpu.returned_to != 0x46d3ad { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x46d3ba); f_00402d60(cpu); if cpu.returned_to != 0x46d3ba { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x46d3c4); f_00401080(cpu); if cpu.returned_to != 0x46d3c4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.push(0x46d3cc); f_004010e0(cpu); if cpu.returned_to != 0x46d3cc { return; }
  { cpu.popad(); }
  cpu.push(0x46d3d2); f_0046d3d8(cpu); if cpu.returned_to != 0x46d3d2 { return; }
  cpu.push(0x46d3d7); f_0046cc53(cpu); if cpu.returned_to != 0x46d3d7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d3d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d3d8); }
 let mut pc: u32 = 0x46d3d8;
 loop { match pc {
 0x46d3d8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d525(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d525); }
 let mut pc: u32 = 0x46d525;
 loop { match pc {
 0x46d525 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bc03u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x15u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x46d53c; continue;
 }
 0x46d53c => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x46d551; continue; }
  pc = 0x46d541; continue;
 }
 0x46d541 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x509325u32.wrapping_add(cpu.eax)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  pc = 0x46d551; continue;
 }
 0x46d551 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46d53c; continue; }
  pc = 0x46d556; continue;
 }
 0x46d556 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d55d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d55d); }
 let mut pc: u32 = 0x46d55d;
 loop { match pc {
 0x46d55d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4e155cu32); cpu.eax = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x46d572; continue; }
  pc = 0x46d56f; continue;
 }
 0x46d56f => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x46d572; continue;
 }
 0x46d572 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d574(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d574); }
 let mut pc: u32 = 0x46d574;
 loop { match pc {
 0x46d574 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x38u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d5b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d5b9); }
 let mut pc: u32 = 0x46d5b9;
 loop { match pc {
 0x46d5b9 => {
  { let v: u32 = 0x5ce994u32; cpu.w32(0x4d6af0u32, v); }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x46d5e1; continue; }
  pc = 0x46d5d8; continue;
 }
 0x46d5d8 => {
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  f_0046d630(cpu); return;
 }
 0x46d5e1 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d630); }
 let mut pc: u32 = 0x46d630;
 loop { match pc {
 0x46d630 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.and32(cpu.r32(0x58c87au32), 0x8u32); }
  if cpu.zf { pc = 0x46d66c; continue; }
  pc = 0x46d640; continue;
 }
 0x46d640 => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x46d66c; continue; }
  pc = 0x46d649; continue;
 }
 0x46d649 => {
  { cpu.sub8(cpu.r8(0x58c889u32), 0x0u8); }
  if cpu.zf { pc = 0x46d66c; continue; }
  pc = 0x46d652; continue;
 }
 0x46d652 => {
  { let v: u32 = cpu.r32(0x4d6a24u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  if cpu.sf { pc = 0x46d66c; continue; }
  pc = 0x46d65d; continue;
 }
 0x46d65d => {
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_add((cpu.ecx as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.ecx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x46d66c; continue;
 }
 0x46d66c => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0xeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_sub(0x2u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_sub(0x2u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x46d683; continue;
 }
 0x46d683 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x46d6d2; continue; }
  pc = 0x46d68a; continue;
 }
 0x46d68a => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x46d6d2; continue; }
  pc = 0x46d691; continue;
 }
 0x46d691 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x46d6d2; continue; }
  pc = 0x46d69b; continue;
 }
 0x46d69b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x46d6ab; continue; }
  pc = 0x46d6a6; continue;
 }
 0x46d6a6 => {
  cpu.push(0x46d6ab); f_0046d7be(cpu); if cpu.returned_to != 0x46d6ab { return; }
  pc = 0x46d6ab; continue;
 }
 0x46d6ab => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x46d6ce; continue; }
  pc = 0x46d6b5; continue;
 }
 0x46d6b5 => {
  { cpu.sub16((cpu.esi as u16), (cpu.edx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x46d6ce; continue; }
  pc = 0x46d6ba; continue;
 }
 0x46d6ba => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x46d6ce; continue; }
  pc = 0x46d6c1; continue;
 }
 0x46d6c1 => {
  { cpu.sub16((cpu.edi as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x46d6ce; continue; }
  pc = 0x46d6c8; continue;
 }
 0x46d6c8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x46d6ce => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x46d6d2; continue;
 }
 0x46d6d2 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x46d683; continue; }
  pc = 0x46d6da; continue;
 }
 0x46d6da => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d6dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d6dc); }
 let mut pc: u32 = 0x46d6dc;
 loop { match pc {
 0x46d6dc => {
  cpu.push(0x46d6e1); f_00418bd9(cpu); if cpu.returned_to != 0x46d6e1 { return; }
  { let v: u32 = 0x5ce994u32; cpu.w32(0x4d6af0u32, v); }
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x46d7bc; continue; }
  pc = 0x46d704; continue;
 }
 0x46d704 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.and32(cpu.r32(0x58c87au32), 0x8u32); }
  if cpu.zf { pc = 0x46d747; continue; }
  pc = 0x46d71b; continue;
 }
 0x46d71b => {
  { cpu.sub8(cpu.r8(0x4e1fa1u32), 0x0u8); }
  if cpu.zf { pc = 0x46d747; continue; }
  pc = 0x46d724; continue;
 }
 0x46d724 => {
  { cpu.sub8(cpu.r8(0x58c889u32), 0x0u8); }
  if cpu.zf { pc = 0x46d747; continue; }
  pc = 0x46d72d; continue;
 }
 0x46d72d => {
  { let v: u32 = cpu.r32(0x4d6a24u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  if cpu.sf { pc = 0x46d747; continue; }
  pc = 0x46d738; continue;
 }
 0x46d738 => {
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_add((cpu.ecx as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.ecx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x46d747; continue;
 }
 0x46d747 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0xeu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_sub(0x2u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_sub(0x2u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x46d75e; continue;
 }
 0x46d75e => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x46d7b4; continue; }
  pc = 0x46d765; continue;
 }
 0x46d765 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x46d7b4; continue; }
  pc = 0x46d76c; continue;
 }
 0x46d76c => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x46d7b4; continue; }
  pc = 0x46d773; continue;
 }
 0x46d773 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x46d7b4; continue; }
  pc = 0x46d77d; continue;
 }
 0x46d77d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x46d78d; continue; }
  pc = 0x46d788; continue;
 }
 0x46d788 => {
  cpu.push(0x46d78d); f_0046d7be(cpu); if cpu.returned_to != 0x46d78d { return; }
  pc = 0x46d78d; continue;
 }
 0x46d78d => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x46d7b0; continue; }
  pc = 0x46d797; continue;
 }
 0x46d797 => {
  { cpu.sub16((cpu.esi as u16), (cpu.edx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x46d7b0; continue; }
  pc = 0x46d79c; continue;
 }
 0x46d79c => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x46d7b0; continue; }
  pc = 0x46d7a3; continue;
 }
 0x46d7a3 => {
  { cpu.sub16((cpu.edi as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x46d7b0; continue; }
  pc = 0x46d7aa; continue;
 }
 0x46d7aa => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x46d7b0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x46d7b4; continue;
 }
 0x46d7b4 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x46d75e; continue; }
  pc = 0x46d7bc; continue;
 }
 0x46d7bc => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d7be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d7be); }
 let mut pc: u32 = 0x46d7be;
 loop { match pc {
 0x46d7be => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x5b9ff4u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.ebx as u8), 2u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let a = cpu.ebx; let b = cpu.esi; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4d6af0u32)); cpu.esi = v; }
  { cpu.lods(1, false); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.lods(1, false); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.lods(1, false); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.lods(1, false); }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u16 = cpu.add16((cpu.edi as u16), (cpu.eax as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d834(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d834); }
 let mut pc: u32 = 0x46d834;
 loop { match pc {
 0x46d834 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0xcu32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x46d848; continue; }
  pc = 0x46d83c; continue;
 }
 0x46d83c => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x46d86a; continue;
 }
 0x46d848 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x46d86a; continue;
 }
 0x46d86a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d86b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d86b); }
 let mut pc: u32 = 0x46d86b;
 loop { match pc {
 0x46d86b => {
  { cpu.sub32(cpu.esi, 0x2bu32); }
  if !cpu.zf { pc = 0x46d91a; continue; }
  pc = 0x46d874; continue;
 }
 0x46d874 => {
  { let v: u32 = 0x4e1ff4u32; cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x46d87e; continue;
 }
 0x46d87e => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.edi))); }
  if cpu.cf { pc = 0x46d910; continue; }
  pc = 0x46d887; continue;
 }
 0x46d887 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi))); }
  if cpu.cf { pc = 0x46d910; continue; }
  pc = 0x46d891; continue;
 }
 0x46d891 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x46d910; continue; }
  pc = 0x46d8a1; continue;
 }
 0x46d8a1 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x46d910; continue; }
  pc = 0x46d8ae; continue;
 }
 0x46d8ae => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x46d8d4; continue; }
  pc = 0x46d8b5; continue;
 }
 0x46d8b5 => {
  { cpu.sub8(cpu.r8(0xcu32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x46d8c7; continue; }
  pc = 0x46d8bb; continue;
 }
 0x46d8bb => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x46d8e9; continue;
 }
 0x46d8c7 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x46d8e9; continue;
 }
 0x46d8d4 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x46d8e9; continue;
 }
 0x46d8e9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4e1594u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4e1596u32, v); }
  { let v: u16 = cpu.r16(0x4e1594u32).wrapping_sub(0x8u16); cpu.w16(0x4e1594u32, v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x4e1596u32), 0x8u16); cpu.w16(0x4e1596u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e159eu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x46d910 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x1au32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x46d87e; continue; }
  pc = 0x46d91a; continue;
 }
 0x46d91a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d91b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d91b); }
 let mut pc: u32 = 0x46d91b;
 loop { match pc {
 0x46d91b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x46d922; continue;
 }
 0x46d922 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x17u8); }
  if cpu.zf { cpu.poll(); pc = 0x46d922; continue; }
  pc = 0x46d930; continue;
 }
 0x46d930 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x46d931; continue;
 }
 0x46d931 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x80u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x17u8); }
  if cpu.zf { cpu.poll(); pc = 0x46d931; continue; }
  pc = 0x46d944; continue;
 }
 0x46d944 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046d947(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46d947); }
 let mut pc: u32 = 0x46d947;
 loop { match pc {
 0x46d947 => {
  cpu.push(0x46d94c); f_0046dad0(cpu); if cpu.returned_to != 0x46d94c { return; }
  { let v: u32 = 0x4dd54cu32; cpu.esi = v; }
  { let v: u32 = 0x4000u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x46d95c; continue;
 }
 0x46d95c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if !cpu.zf { pc = 0x46da19; continue; }
  pc = 0x46d969; continue;
 }
 0x46d969 => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if cpu.zf { pc = 0x46da19; continue; }
  pc = 0x46d976; continue;
 }
 0x46d976 => {
  { let v: u8 = cpu.r8(0xffffff80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if cpu.zf { pc = 0x46da19; continue; }
  pc = 0x46d983; continue;
 }
 0x46d983 => {
  { let v: u8 = cpu.r8(0xffffff7fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  if !cpu.zf { pc = 0x46da19; continue; }
  pc = 0x46d993; continue;
 }
 0x46d993 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(0x10u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x10u32); cpu.edx = v; }
  cpu.push(0x46d9a0); f_0046da8a(cpu); if cpu.returned_to != 0x46d9a0 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd4u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x56c9d5u32), 0x0u8); }
  if !cpu.zf { pc = 0x46d9cf; continue; }
  pc = 0x46d9b8; continue;
 }
 0x46d9b8 => {
  if cpu.cf { pc = 0x46da19; continue; }
  pc = 0x46d9ba; continue;
 }
 0x46d9ba => {
  { let v: u16 = 0x0u16; cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  if !cpu.zf { pc = 0x46d9cf; continue; }
  pc = 0x46d9c9; continue;
 }
 0x46d9c9 => {
  { let v: u16 = 0x10u16; cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x46d9cf; continue;
 }
 0x46d9cf => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x46d9d6); f_0046d91b(cpu); if cpu.returned_to != 0x46d9d6 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x56c9d5u32), 0x0u8); }
  if !cpu.zf { pc = 0x46d9f3; continue; }
  pc = 0x46d9ef; continue;
 }
 0x46d9ef => {
  { let v: u8 = 0x0u8; cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  pc = 0x46d9f3; continue;
 }
 0x46d9f3 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x56c9d5u32), 0x0u8); }
  if !cpu.zf { pc = 0x46da18; continue; }
  pc = 0x46d9ff; continue;
 }
 0x46d9ff => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x46da18; continue; }
  pc = 0x46da0c; continue;
 }
 0x46da0c => {
  { let v: u16 = 0x258u16; cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x46da18; continue;
 }
 0x46da18 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x46da19; continue;
 }
 0x46da19 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub16((cpu.ebx as u16), 0x800u16); }
  if (cpu.sf != cpu.of) { pc = 0x46da2d; continue; }
  pc = 0x46da26; continue;
 }
 0x46da26 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), 0x10u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x46da2d; continue;
 }
 0x46da2d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x46d95c; continue; }
  pc = 0x46da34; continue;
 }
 0x46da34 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046da35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46da35); }
 let mut pc: u32 = 0x46da35;
 loop { match pc {
 0x46da35 => {
  { let v: u32 = 0x4e1ff4u32; cpu.esi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x46da3f; continue;
 }
 0x46da3f => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x46da82; continue; }
  pc = 0x46da46; continue;
 }
 0x46da46 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x46da7a; continue; }
  pc = 0x46da4e; continue;
 }
 0x46da4e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = 0x258u16; cpu.w16(0xau32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.esi)) | 0x2u8; cpu.w8(0xfu32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x46da6c); f_0046e3dc(cpu); if cpu.returned_to != 0x46da6c { return; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xeu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x46da74); f_0046e787(cpu); if cpu.returned_to != 0x46da74 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.inc8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x46da82; continue;
 }
 0x46da7a => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x46da82; continue;
 }
 0x46da82 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1au32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46da3f; continue; }
  pc = 0x46da89; continue;
 }
 0x46da89 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046da8a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46da8a); }
 let mut pc: u32 = 0x46da8a;
 loop { match pc {
 0x46da8a => {
  { let v: u8 = 0x0u8; cpu.w8(0x56c9d5u32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4e1ff4u32; cpu.edi = v; }
  { let v: u8 = 0x14u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x46da99; continue;
 }
 0x46da99 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x46daae; continue; }
  pc = 0x46da9e; continue;
 }
 0x46da9e => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x46daae; continue; }
  pc = 0x46daa4; continue;
 }
 0x46daa4 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x56c9d5u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x46daae => {
  { let v: u32 = cpu.edi.wrapping_add(0x1au32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46da99; continue; }
  pc = 0x46dab5; continue;
 }
 0x46dab5 => {
  { let v: u32 = 0x4e1ff4u32; cpu.edi = v; }
  { let v: u8 = 0x14u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x46dabc; continue;
 }
 0x46dabc => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if !cpu.zf { pc = 0x46dac6; continue; }
  pc = 0x46dac3; continue;
 }
 0x46dac3 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x46dac6 => {
  { let v: u32 = cpu.edi.wrapping_add(0x1au32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46dabc; continue; }
  pc = 0x46dacd; continue;
 }
 0x46dacd => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046dad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46dad0); }
 let mut pc: u32 = 0x46dad0;
 loop { match pc {
 0x46dad0 => {
  { let v: u32 = 0x4e1ff4u32; cpu.ebp = v; }
  { let v: u8 = 0x14u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x46dad7; continue;
 }
 0x46dad7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0xfu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x46daf4); f_00420989(cpu); if cpu.returned_to != 0x46daf4 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4001u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if !cpu.zf { pc = 0x46db95; continue; }
  pc = 0x46db06; continue;
 }
 0x46db06 => {
  { let v: u8 = cpu.r8(0xffffff7fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  if !cpu.zf { pc = 0x46db95; continue; }
  pc = 0x46db16; continue;
 }
 0x46db16 => {
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x46db5c; continue; }
  pc = 0x46db1f; continue;
 }
 0x46db1f => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  if !cpu.zf { pc = 0x46db95; continue; }
  pc = 0x46db2c; continue;
 }
 0x46db2c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x46db3d; continue; }
  pc = 0x46db36; continue;
 }
 0x46db36 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x46db53; continue;
 }
 0x46db3d => {
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16) | (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x46db53; continue;
 }
 0x46db53 => {
  { let v: u32 = cpu.edx; cpu.w32(0x210u32.wrapping_add(cpu.ebp), v); }
  pc = 0x46db9d; continue;
 }
 0x46db5c => {
  { let v: u8 = cpu.r8(0xffffff80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2bu8); }
  if !cpu.zf { pc = 0x46db95; continue; }
  pc = 0x46db65; continue;
 }
 0x46db65 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x46db76; continue; }
  pc = 0x46db6f; continue;
 }
 0x46db6f => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x46db8c; continue;
 }
 0x46db76 => {
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16) | (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x46db8c; continue;
 }
 0x46db8c => {
  { let v: u32 = cpu.edx; cpu.w32(0x210u32.wrapping_add(cpu.ebp), v); }
  pc = 0x46db9d; continue;
 }
 0x46db95 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x46db9d; continue;
 }
 0x46db9d => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x1au32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46dad7; continue; }
  pc = 0x46dba8; continue;
 }
 0x46dba8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046dba9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46dba9); }
 let mut pc: u32 = 0x46dba9;
 loop { match pc {
 0x46dba9 => {
  { cpu.sub8(cpu.r8(0x575f18u32), 0x0u8); }
  if !cpu.zf { pc = 0x46dbb3; continue; }
  pc = 0x46dbb2; continue;
 }
 0x46dbb2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x46dbb3 => {
  if cpu.zf { pc = 0x46dcb8; continue; }
  pc = 0x46dbb9; continue;
 }
 0x46dbb9 => {
  { cpu.sub32(cpu.r32(0x210u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x46dcb8; continue; }
  pc = 0x46dbc9; continue;
 }
 0x46dbc9 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x210u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.sf { pc = 0x46dcb7; continue; }
  pc = 0x46dbd9; continue;
 }
 0x46dbd9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x575f1du32), 0x4u8); }
  if cpu.zf { pc = 0x46dbf2; continue; }
  pc = 0x46dbe9; continue;
 }
 0x46dbe9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x575f1du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x46dbf2; continue;
 }
 0x46dbf2 => {
  { let v: u8 = cpu.r8(0x58c887u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x46dc07; continue; }
  pc = 0x46dc01; continue;
 }
 0x46dc01 => {
  { let v: u8 = cpu.r8(0x58c888u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x46dc07; continue;
 }
 0x46dc07 => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x46dc1f; continue; }
  pc = 0x46dc0c; continue;
 }
 0x46dc0c => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.sub8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x46dc1d; continue; }
  pc = 0x46dc15; continue;
 }
 0x46dc15 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  if !cpu.sf { pc = 0x46dc1f; continue; }
  pc = 0x46dc19; continue;
 }
 0x46dc19 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x46dc1f; continue;
 }
 0x46dc1d => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x46dc1f; continue;
 }
 0x46dc1f => {
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
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0xffu32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x46dc83; continue; }
  pc = 0x46dc5f; continue;
 }
 0x46dc5f => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if cpu.sf { pc = 0x46dc83; continue; }
  pc = 0x46dc65; continue;
 }
 0x46dc65 => {
  { cpu.sub16((cpu.eax as u16), 0x100u16); }
  if (cpu.sf == cpu.of) { pc = 0x46dc83; continue; }
  pc = 0x46dc6b; continue;
 }
 0x46dc6b => {
  { cpu.sub16((cpu.ebx as u16), 0xaau16); }
  if (cpu.sf == cpu.of) { pc = 0x46dc83; continue; }
  pc = 0x46dc72; continue;
 }
 0x46dc72 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x46dc81); f_0041b671(cpu); if cpu.returned_to != 0x46dc81 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x46dc83; continue;
 }
 0x46dc83 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7fu32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = 0xffu32; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x18u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x46dcb7; continue; }
  pc = 0x46dc9f; continue;
 }
 0x46dc9f => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if cpu.sf { pc = 0x46dcb7; continue; }
  pc = 0x46dca5; continue;
 }
 0x46dca5 => {
  { cpu.sub16((cpu.eax as u16), 0x100u16); }
  if (cpu.sf == cpu.of) { pc = 0x46dcb7; continue; }
  pc = 0x46dcab; continue;
 }
 0x46dcab => {
  { cpu.sub16((cpu.ebx as u16), 0xaau16); }
  if (cpu.sf == cpu.of) { pc = 0x46dcb7; continue; }
  pc = 0x46dcb2; continue;
 }
 0x46dcb2 => {
  cpu.push(0x46dcb7); f_0041b671(cpu); if cpu.returned_to != 0x46dcb7 { return; }
  pc = 0x46dcb7; continue;
 }
 0x46dcb7 => {
  { cpu.popad(); }
  pc = 0x46dcb8; continue;
 }
 0x46dcb8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046dcb9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46dcb9); }
 let mut pc: u32 = 0x46dcb9;
 loop { match pc {
 0x46dcb9 => {
  cpu.push(0x46dcbe); f_0046e8bb(cpu); if cpu.returned_to != 0x46dcbe { return; }
  { let v: u32 = 0x4e1ff4u32; cpu.esi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x46dcc8; continue;
 }
 0x46dcc8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x46de50; continue; }
  pc = 0x46dcd4; continue;
 }
 0x46dcd4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x46dce1); f_0047a2d9(cpu); if cpu.returned_to != 0x46dce1 { return; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x575f1du32, v); }
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x46de36; continue; }
  pc = 0x46dcf7; continue;
 }
 0x46dcf7 => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { pc = 0x46dd7d; continue; }
  pc = 0x46dd01; continue;
 }
 0x46dd01 => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x46dd7d; continue; }
  pc = 0x46dd0b; continue;
 }
 0x46dd0b => {
  { cpu.sub32(cpu.r32(0x210u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x46dd7d; continue; }
  pc = 0x46dd1b; continue;
 }
 0x46dd1b => {
  { cpu.sub32(cpu.r32(0x210u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.sf { pc = 0x46dd7d; continue; }
  pc = 0x46dd28; continue;
 }
 0x46dd28 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x575f1du32), 0x4u8); }
  if cpu.zf { pc = 0x46dd41; continue; }
  pc = 0x46dd38; continue;
 }
 0x46dd38 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x575f1du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x46dd41; continue;
 }
 0x46dd41 => {
  { let v: u8 = cpu.r8(0x58c887u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x46dd56; continue; }
  pc = 0x46dd50; continue;
 }
 0x46dd50 => {
  { let v: u8 = cpu.r8(0x58c888u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x46dd56; continue;
 }
 0x46dd56 => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x46dd6e; continue; }
  pc = 0x46dd5b; continue;
 }
 0x46dd5b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.sub8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x46dd6c; continue; }
  pc = 0x46dd64; continue;
 }
 0x46dd64 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  if !cpu.sf { pc = 0x46dd6e; continue; }
  pc = 0x46dd68; continue;
 }
 0x46dd68 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x46dd6e; continue;
 }
 0x46dd6c => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x46dd6e; continue;
 }
 0x46dd6e => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x210u32.wrapping_add(cpu.esi)), cpu.ebx); cpu.w32(0x210u32.wrapping_add(cpu.esi), v); }
  if !cpu.sf { pc = 0x46dd7d; continue; }
  pc = 0x46dd78; continue;
 }
 0x46dd78 => {
  cpu.push(0x46dd7d); f_0046df2e(cpu); if cpu.returned_to != 0x46dd7d { return; }
  pc = 0x46dd7d; continue;
 }
 0x46dd7d => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x46dda6; continue; }
  pc = 0x46dd86; continue;
 }
 0x46dd86 => {
  { let v: u8 = cpu.r8(0x575f1du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x46dda6; continue; }
  pc = 0x46dd91; continue;
 }
 0x46dd91 => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x46dda6; continue; }
  pc = 0x46dd97; continue;
 }
 0x46dd97 => {
  { let v: u32 = cpu.r32(0x5cea2cu32); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x5cea2cu32).wrapping_add(0x4u32); cpu.w32(0x5cea2cu32, v); }
  pc = 0x46dda6; continue;
 }
 0x46dda6 => {
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x46ddbe; continue; }
  pc = 0x46ddae; continue;
 }
 0x46ddae => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x46ddcc; continue;
 }
 0x46ddbe => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  pc = 0x46ddcc; continue;
 }
 0x46ddcc => {
  { let v: u8 = cpu.r8(0xeu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x46ddd1; continue;
 }
 0x46ddd1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffc0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x46de1b; continue; }
  pc = 0x46dddf; continue;
 }
 0x46dddf => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x46de1b; continue; }
  pc = 0x46dde7; continue;
 }
 0x46dde7 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffc0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x46de1b; continue; }
  pc = 0x46ddf2; continue;
 }
 0x46ddf2 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x46de1b; continue; }
  pc = 0x46ddfa; continue;
 }
 0x46ddfa => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4eab3au32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x46de12; continue; }
  pc = 0x46de0a; continue;
 }
 0x46de0a => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x46de10); cpu.call(t); if cpu.returned_to != 0x46de10 { return; }
  pc = 0x46de19; continue;
 }
 0x46de12 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  let t = cpu.r32(0x4d6f2cu32); cpu.push(0x46de19); cpu.call(t); if cpu.returned_to != 0x46de19 { return; }
  pc = 0x46de19; continue;
 }
 0x46de19 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x46de1b; continue;
 }
 0x46de1b => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0xcu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x46de2d; continue; }
  pc = 0x46de24; continue;
 }
 0x46de24 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46ddd1; continue; }
  pc = 0x46de2b; continue;
 }
 0x46de2b => {
  pc = 0x46de50; continue;
 }
 0x46de2d => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46ddd1; continue; }
  pc = 0x46de34; continue;
 }
 0x46de34 => {
  pc = 0x46de50; continue;
 }
 0x46de36 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x46de50; continue; }
  pc = 0x46de45; continue;
 }
 0x46de45 => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x46de50; continue; }
  pc = 0x46de4b; continue;
 }
 0x46de4b => {
  cpu.push(0x46de50); f_0046de62(cpu); if cpu.returned_to != 0x46de50 { return; }
  pc = 0x46de50; continue;
 }
 0x46de50 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x46de57); f_0046dba9(cpu); if cpu.returned_to != 0x46de57 { return; }
  { let v: u32 = cpu.add32(cpu.esi, 0x1au32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x46dcc8; continue; }
  pc = 0x46de61; continue;
 }
 0x46de61 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046de62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46de62); }
 let mut pc: u32 = 0x46de62;
 loop { match pc {
 0x46de62 => {
  { let v: u16 = 0x0u16; cpu.w16(0x576509u32, v); }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x46de86; continue; }
  pc = 0x46de73; continue;
 }
 0x46de73 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  pc = 0x46de97; continue;
 }
 0x46de86 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  pc = 0x46de97; continue;
 }
 0x46de97 => {
  cpu.push(0x46de9c); f_0046df10(cpu); if cpu.returned_to != 0x46de9c { return; }
  pc = 0x46de9c; continue;
 }
 0x46de9c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6a00u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffc0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x46deee; continue; }
  pc = 0x46deaa; continue;
 }
 0x46deaa => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4d6ba4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x46deee; continue; }
  pc = 0x46deb2; continue;
 }
 0x46deb2 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x4d6a04u32)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffc0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x46deee; continue; }
  pc = 0x46debd; continue;
 }
 0x46debd => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4d6ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x46deee; continue; }
  pc = 0x46dec5; continue;
 }
 0x46dec5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x576507u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x46dedd; continue; }
  pc = 0x46ded6; continue;
 }
 0x46ded6 => {
  { let v: u16 = cpu.r16(0x576505u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x46dedd; continue;
 }
 0x46dedd => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x576509u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x46deeb); cpu.call(t); if cpu.returned_to != 0x46deeb { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x46deee; continue;
 }
 0x46deee => {
  { let v: u16 = cpu.r16(0x576509u32) ^ 0x1u16; cpu.w16(0x576509u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0xcu32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x46df08; continue; }
  pc = 0x46deff; continue;
 }
 0x46deff => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x10u32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46de9c; continue; }
  pc = 0x46df06; continue;
 }
 0x46df06 => {
  pc = 0x46df0f; continue;
 }
 0x46df08 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x10u32); cpu.eax = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46de9c; continue; }
  pc = 0x46df0f; continue;
 }
 0x46df0f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046df10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46df10); }
 let mut pc: u32 = 0x46df10;
 loop { match pc {
 0x46df10 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x46df24; continue; }
  pc = 0x46df19; continue;
 }
 0x46df19 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x46df2d; continue;
 }
 0x46df24 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x46df2d; continue;
 }
 0x46df2d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046df2e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x46df2e); } r_0046df2e(cpu, 0x46df2e); }
pub fn f_0046e000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x46e000); } r_0046df2e(cpu, 0x46e000); }
fn r_0046df2e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x46df2e => {
  { cpu.pushad(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x576bcfu32, v); }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.edi)), 0x0u16); }
  if !cpu.zf { pc = 0x46dff4; continue; }
  pc = 0x46df45; continue;
 }
 0x46df45 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x2u32); cpu.ecx = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  cpu.push(0x46df6a); f_0046e4b3(cpu); if cpu.returned_to != 0x46df6a { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x46df76); f_0041c743(cpu); if cpu.returned_to != 0x46df76 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x46df7d; continue;
 }
 0x46df7d => {
  cpu.push(0x46df82); f_0048aadc(cpu); if cpu.returned_to != 0x46df82 { return; }
  if cpu.cf { pc = 0x46e09f; continue; }
  pc = 0x46df88; continue;
 }
 0x46df88 => {
  { let v: u8 = 0x1u8; cpu.w8(0x576bcfu32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.inc16(cpu.r16(0x4d6934u32)); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = cpu.r16(0x4d6934u32) & 0x3u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x56c5dcu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) | 0x80u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xau32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d6d48u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x46df7d; continue; }
  pc = 0x46dfef; continue;
 }
 0x46dfef => {
  pc = 0x46e09f; continue;
 }
 0x46dff4 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 4u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x2u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  cpu.push(0x46e01e); f_0046e4b3(cpu); if cpu.returned_to != 0x46e01e { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x46e02a); f_0041c743(cpu); if cpu.returned_to != 0x46e02a { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x46e031; continue;
 }
 0x46e000 => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 4u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x2u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  cpu.push(0x46e01e); f_0046e4b3(cpu); if cpu.returned_to != 0x46e01e { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x46e02a); f_0041c743(cpu); if cpu.returned_to != 0x46e02a { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x46e031; continue;
 }
 0x46e031 => {
  cpu.push(0x46e036); f_0048aadc(cpu); if cpu.returned_to != 0x46e036 { return; }
  if cpu.cf { pc = 0x46e09f; continue; }
  pc = 0x46e038; continue;
 }
 0x46e038 => {
  { let v: u8 = 0x1u8; cpu.w8(0x576bcfu32, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.inc16(cpu.r16(0x4d6934u32)); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = cpu.r16(0x4d6934u32) & 0x3u16; cpu.w16(0x4d6934u32, v); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x56c5ccu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) | 0x80u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xau32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d6d50u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x46e031; continue; }
  pc = 0x46e09f; continue;
 }
 0x46e09f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x576bcfu32), 0x0u8); }
  if !cpu.zf { pc = 0x46e0f2; continue; }
  pc = 0x46e0ae; continue;
 }
 0x46e0ae => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.esi)) & 0xd1u8; cpu.w8(0xfu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.zf { pc = 0x46e0c5; continue; }
  pc = 0x46e0c1; continue;
 }
 0x46e0c1 => {
  { let v: u16 = 0x12cu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x46e0c5; continue;
 }
 0x46e0c5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.mul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x210u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5cf454u32, v); }
  { cpu.popad(); }
  pc = 0x46e0f2; continue;
 }
 0x46e0f2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e0f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46e0f3); }
 let mut pc: u32 = 0x46e0f3;
 loop { match pc {
 0x46e0f3 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebp; cpu.w32(0x57553fu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4dd54cu32); cpu.eax = v; }
  { let v: u8 = 0x80u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x46e11e); f_0046e31e(cpu); if cpu.returned_to != 0x46e11e { return; }
  if !cpu.cf { pc = 0x46e23a; continue; }
  pc = 0x46e124; continue;
 }
 0x46e124 => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esi)), 0x0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x46e23a; continue; }
  pc = 0x46e130; continue;
 }
 0x46e130 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esi)), (cpu.ebp as u16)); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x46e203; continue; }
  pc = 0x46e141; continue;
 }
 0x46e141 => {
  { cpu.sub8(cpu.r8(0x5cc647u32), 0x0u8); }
  if !cpu.zf { pc = 0x46e169; continue; }
  pc = 0x46e14a; continue;
 }
 0x46e14a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x57553fu32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  cpu.push(0x46e166); f_004168c3(cpu); if cpu.returned_to != 0x46e166 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x46e169; continue;
 }
 0x46e169 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.esi)) | 0x2u8; cpu.w8(0xfu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x46e187; continue; }
  pc = 0x46e180; continue;
 }
 0x46e180 => {
  cpu.push(0x46e185); f_0046e2af(cpu); if cpu.returned_to != 0x46e185 { return; }
  pc = 0x46e18c; continue;
 }
 0x46e187 => {
  cpu.push(0x46e18c); f_0046e240(cpu); if cpu.returned_to != 0x46e18c { return; }
  pc = 0x46e18c; continue;
 }
 0x46e18c => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x46e198); f_00467458(cpu); if cpu.returned_to != 0x46e198 { return; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x46e1cf; continue; }
  pc = 0x46e1c4; continue;
 }
 0x46e1c4 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, 0x8u32); cpu.edx = v; }
  pc = 0x46e1db; continue;
 }
 0x46e1cf => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x8u32); cpu.ecx = v; }
  pc = 0x46e1db; continue;
 }
 0x46e1db => {
  { let v: u8 = 0x1u8; cpu.w8(0x57925du32, v); }
  cpu.push(0x46e1e7); f_0045bec8(cpu); if cpu.returned_to != 0x46e1e7 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x57925du32, v); }
  cpu.push(0x46e1f3); f_004480c3(cpu); if cpu.returned_to != 0x46e1f3 { return; }
  { cpu.popad(); }
  cpu.push(0x46e1f9); f_0046e3dc(cpu); if cpu.returned_to != 0x46e1f9 { return; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xeu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x46e201); f_0046e787(cpu); if cpu.returned_to != 0x46e201 { return; }
  pc = 0x46e23d; continue;
 }
 0x46e203 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x46e23a; continue; }
  pc = 0x46e210; continue;
 }
 0x46e210 => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), 0x20u8); }
  if !cpu.zf { pc = 0x46e23a; continue; }
  pc = 0x46e216; continue;
 }
 0x46e216 => {
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.esi)) | 0x20u8; cpu.w8(0xfu32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x46e235; continue; }
  pc = 0x46e22e; continue;
 }
 0x46e22e => {
  cpu.push(0x46e233); f_0046e2af(cpu); if cpu.returned_to != 0x46e233 { return; }
  pc = 0x46e23a; continue;
 }
 0x46e235 => {
  cpu.push(0x46e23a); f_0046e240(cpu); if cpu.returned_to != 0x46e23a { return; }
  pc = 0x46e23a; continue;
 }
 0x46e23a => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x46e23d => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e240(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46e240); }
 let mut pc: u32 = 0x46e240;
 loop { match pc {
 0x46e240 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u32 = 0x655u32; cpu.w32(0x5cea28u32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x2u32); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x46e271; continue;
 }
 0x46e271 => {
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x4d6938u32), 0x0u16); }
  if !cpu.zf { pc = 0x46e281; continue; }
  pc = 0x46e27c; continue;
 }
 0x46e27c => {
  cpu.push(0x46e281); f_0046e864(cpu); if cpu.returned_to != 0x46e281 { return; }
  pc = 0x46e281; continue;
 }
 0x46e281 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x46e28a); f_0046e864(cpu); if cpu.returned_to != 0x46e28a { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x46e293); f_0046e864(cpu); if cpu.returned_to != 0x46e293 { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x4d6938u32), 0x0u16); }
  if !cpu.zf { pc = 0x46e2a6; continue; }
  pc = 0x46e2a1; continue;
 }
 0x46e2a1 => {
  cpu.push(0x46e2a6); f_0046e864(cpu); if cpu.returned_to != 0x46e2a6 { return; }
  pc = 0x46e2a6; continue;
 }
 0x46e2a6 => {
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x46e271; continue; }
  pc = 0x46e2ae; continue;
 }
 0x46e2ae => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e2af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46e2af); }
 let mut pc: u32 = 0x46e2af;
 loop { match pc {
 0x46e2af => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u32 = 0x655u32; cpu.w32(0x5cea28u32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x2u32); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7au32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x46e2e0; continue;
 }
 0x46e2e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x4d6938u32), 0x0u16); }
  if !cpu.zf { pc = 0x46e2f0; continue; }
  pc = 0x46e2eb; continue;
 }
 0x46e2eb => {
  cpu.push(0x46e2f0); f_0046e864(cpu); if cpu.returned_to != 0x46e2f0 { return; }
  pc = 0x46e2f0; continue;
 }
 0x46e2f0 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x46e2f9); f_0046e864(cpu); if cpu.returned_to != 0x46e2f9 { return; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x46e302); f_0046e864(cpu); if cpu.returned_to != 0x46e302 { return; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x4d6938u32), 0x0u16); }
  if !cpu.zf { pc = 0x46e315; continue; }
  pc = 0x46e310; continue;
 }
 0x46e310 => {
  cpu.push(0x46e315); f_0046e864(cpu); if cpu.returned_to != 0x46e315 { return; }
  pc = 0x46e315; continue;
 }
 0x46e315 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x46e2e0; continue; }
  pc = 0x46e31d; continue;
 }
 0x46e31d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e31e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46e31e); }
 let mut pc: u32 = 0x46e31e;
 loop { match pc {
 0x46e31e => {
  { let v: u32 = 0x4e1ff4u32; cpu.esi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x46e328; continue;
 }
 0x46e328 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x46e359; continue; }
  pc = 0x46e32e; continue;
 }
 0x46e32e => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x46e359; continue; }
  pc = 0x46e334; continue;
 }
 0x46e334 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x46e359; continue; }
  pc = 0x46e33c; continue;
 }
 0x46e33c => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x4u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x46e359; continue; }
  pc = 0x46e345; continue;
 }
 0x46e345 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x46e359; continue; }
  pc = 0x46e34e; continue;
 }
 0x46e34e => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x46e359; continue; }
  pc = 0x46e357; continue;
 }
 0x46e357 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x46e359 => {
  { let v: u32 = cpu.esi.wrapping_add(0x1au32); cpu.esi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46e328; continue; }
  pc = 0x46e360; continue;
 }
 0x46e360 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e362(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46e362); }
 let mut pc: u32 = 0x46e362;
 loop { match pc {
 0x46e362 => {
  { let v: u32 = 0x4e1ff4u32; cpu.esi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x46e36c; continue;
 }
 0x46e36c => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x46e39d; continue; }
  pc = 0x46e372; continue;
 }
 0x46e372 => {
  { cpu.and8(cpu.r8(0xfu32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x46e39d; continue; }
  pc = 0x46e378; continue;
 }
 0x46e378 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x46e39d; continue; }
  pc = 0x46e380; continue;
 }
 0x46e380 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x4u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x46e39d; continue; }
  pc = 0x46e389; continue;
 }
 0x46e389 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x46e39d; continue; }
  pc = 0x46e392; continue;
 }
 0x46e392 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x46e39d; continue; }
  pc = 0x46e39b; continue;
 }
 0x46e39b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x46e39d => {
  { let v: u32 = cpu.esi.wrapping_add(0x1au32); cpu.esi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46e36c; continue; }
  pc = 0x46e3a4; continue;
 }
 0x46e3a4 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e3af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46e3af); }
 let mut pc: u32 = 0x46e3af;
 loop { match pc {
 0x46e3af => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x32u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e3b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46e3b8); }
 let mut pc: u32 = 0x46e3b8;
 loop { match pc {
 0x46e3b8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2fu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e3c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46e3c1); }
 let mut pc: u32 = 0x46e3c1;
 loop { match pc {
 0x46e3c1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e3ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46e3ca); }
 let mut pc: u32 = 0x46e3ca;
 loop { match pc {
 0x46e3ca => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x17u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e3d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46e3d3); }
 let mut pc: u32 = 0x46e3d3;
 loop { match pc {
 0x46e3d3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e3dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46e3dc); }
 let mut pc: u32 = 0x46e3dc;
 loop { match pc {
 0x46e3dc => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x46e3e8); f_00420989(cpu); if cpu.returned_to != 0x46e3e8 { return; }
  { let v: u32 = 0x4000u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x46e466; continue; }
  pc = 0x46e3fa; continue;
 }
 0x46e3fa => {
  { let v: u32 = cpu.ebx.wrapping_add(0x80u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x46e414); f_0046e3af(cpu); if cpu.returned_to != 0x46e414 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x80u32); cpu.ebx = v; }
  cpu.push(0x46e41f); f_0046e3af(cpu); if cpu.returned_to != 0x46e41f { return; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x80u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  pc = 0x46e42a; continue;
 }
 0x46e42a => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x46e447; continue; }
  pc = 0x46e42f; continue;
 }
 0x46e42f => {
  cpu.push(0x46e434); f_0046e3af(cpu); if cpu.returned_to != 0x46e434 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x80u32); cpu.ebx = v; }
  cpu.push(0x46e43f); f_0046e3af(cpu); if cpu.returned_to != 0x46e43f { return; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x80u32); cpu.ebx = v; }
  pc = 0x46e45d; continue;
 }
 0x46e447 => {
  cpu.push(0x46e44c); f_0046e3b8(cpu); if cpu.returned_to != 0x46e44c { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x80u32); cpu.ebx = v; }
  cpu.push(0x46e457); f_0046e3b8(cpu); if cpu.returned_to != 0x46e457 { return; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x80u32); cpu.ebx = v; }
  pc = 0x46e45d; continue;
 }
 0x46e45d => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46e42a; continue; }
  pc = 0x46e462; continue;
 }
 0x46e462 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x46e4ac; continue;
 }
 0x46e466 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  cpu.push(0x46e479); f_0046e3af(cpu); if cpu.returned_to != 0x46e479 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x46e47f); f_0046e3af(cpu); if cpu.returned_to != 0x46e47f { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x7fu32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  pc = 0x46e486; continue;
 }
 0x46e486 => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x46e498; continue; }
  pc = 0x46e48b; continue;
 }
 0x46e48b => {
  cpu.push(0x46e490); f_0046e3af(cpu); if cpu.returned_to != 0x46e490 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x46e496); f_0046e3af(cpu); if cpu.returned_to != 0x46e496 { return; }
  pc = 0x46e4a3; continue;
 }
 0x46e498 => {
  cpu.push(0x46e49d); f_0046e3b8(cpu); if cpu.returned_to != 0x46e49d { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x46e4a3); f_0046e3b8(cpu); if cpu.returned_to != 0x46e4a3 { return; }
  pc = 0x46e4a3; continue;
 }
 0x46e4a3 => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x7fu32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46e486; continue; }
  pc = 0x46e4aa; continue;
 }
 0x46e4aa => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x46e4ac; continue;
 }
 0x46e4ac => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e4b3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x46e4b3); } r_0046e4b3(cpu, 0x46e4b3); }
pub fn f_0046e5b8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x46e5b8); } r_0046e4b3(cpu, 0x46e5b8); }
pub fn f_0046e609(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x46e609); } r_0046e4b3(cpu, 0x46e609); }
pub fn f_0046e624(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x46e624); } r_0046e4b3(cpu, 0x46e624); }
pub fn f_0046e643(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x46e643); } r_0046e4b3(cpu, 0x46e643); }
pub fn f_0046e65f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x46e65f); } r_0046e4b3(cpu, 0x46e65f); }
pub fn f_0046e67e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x46e67e); } r_0046e4b3(cpu, 0x46e67e); }
pub fn f_0046e69a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x46e69a); } r_0046e4b3(cpu, 0x46e69a); }
pub fn f_0046e6b9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x46e6b9); } r_0046e4b3(cpu, 0x46e6b9); }
pub fn f_0046e6d5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x46e6d5); } r_0046e4b3(cpu, 0x46e6d5); }
fn r_0046e4b3(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x46e4b3 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 3u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4dd54cu32); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd8u32, v); }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x46e559; continue; }
  pc = 0x46e4e5; continue;
 }
 0x46e4e5 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  pc = 0x46e4f3; continue;
 }
 0x46e4f3 => {
  { cpu.sub8((cpu.ecx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x46e4fc; continue; }
  pc = 0x46e4f7; continue;
 }
 0x46e4f7 => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x46e524; continue; }
  pc = 0x46e4fc; continue;
 }
 0x46e4fc => {
  cpu.push(0x46e501); f_0046e3c1(cpu); if cpu.returned_to != 0x46e501 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x80u32); cpu.ebx = v; }
  cpu.push(0x46e50c); f_0046e3d3(cpu); if cpu.returned_to != 0x46e50c { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x80u32); cpu.ebx = v; }
  cpu.push(0x46e517); f_0046e3d3(cpu); if cpu.returned_to != 0x46e517 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x80u32); cpu.ebx = v; }
  cpu.push(0x46e522); f_0046e3c1(cpu); if cpu.returned_to != 0x46e522 { return; }
  pc = 0x46e54a; continue;
 }
 0x46e524 => {
  cpu.push(0x46e529); f_0046e3c1(cpu); if cpu.returned_to != 0x46e529 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x80u32); cpu.ebx = v; }
  cpu.push(0x46e534); f_0046e3ca(cpu); if cpu.returned_to != 0x46e534 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x80u32); cpu.ebx = v; }
  cpu.push(0x46e53f); f_0046e3ca(cpu); if cpu.returned_to != 0x46e53f { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x80u32); cpu.ebx = v; }
  cpu.push(0x46e54a); f_0046e3c1(cpu); if cpu.returned_to != 0x46e54a { return; }
  pc = 0x46e54a; continue;
 }
 0x46e54a => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x180u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46e4f3; continue; }
  pc = 0x46e555; continue;
 }
 0x46e555 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x46e5a8; continue;
 }
 0x46e559 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  pc = 0x46e566; continue;
 }
 0x46e566 => {
  { cpu.sub8((cpu.ecx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x46e56f; continue; }
  pc = 0x46e56a; continue;
 }
 0x46e56a => {
  { cpu.sub8((cpu.ecx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x46e588; continue; }
  pc = 0x46e56f; continue;
 }
 0x46e56f => {
  cpu.push(0x46e574); f_0046e3c1(cpu); if cpu.returned_to != 0x46e574 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x46e57a); f_0046e3d3(cpu); if cpu.returned_to != 0x46e57a { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x46e580); f_0046e3d3(cpu); if cpu.returned_to != 0x46e580 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x46e586); f_0046e3c1(cpu); if cpu.returned_to != 0x46e586 { return; }
  pc = 0x46e59f; continue;
 }
 0x46e588 => {
  cpu.push(0x46e58d); f_0046e3c1(cpu); if cpu.returned_to != 0x46e58d { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x46e593); f_0046e3ca(cpu); if cpu.returned_to != 0x46e593 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x46e599); f_0046e3ca(cpu); if cpu.returned_to != 0x46e599 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x46e59f); f_0046e3c1(cpu); if cpu.returned_to != 0x46e59f { return; }
  pc = 0x46e59f; continue;
 }
 0x46e59f => {
  { let v: u32 = cpu.ebx.wrapping_add(0x7du32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x46e566; continue; }
  pc = 0x46e5a6; continue;
 }
 0x46e5a6 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x46e5a8; continue;
 }
 0x46e5a8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.pushad(); }
  { let v: u32 = 0x46e5b8u32; cpu.edx = v; }
  pc = 0x46e5bd; continue;
 }
 0x46e5b8 => {
  if !cpu.cf { pc = 0x46e62d; continue; }
  pc = 0x46e5ba; continue;
 }
 0x46e5ba => {
  if !cpu.cf { pc = 0x46e62f; continue; }
  pc = 0x46e5bc; continue;
 }
 0x46e5bc => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x5ce90fu32); cpu.eax = v; }
  { let v: u32 = 0x610046e5u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4e1ff4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x46e6e5; continue; }
  pc = 0x46e5e2; continue;
 }
 0x46e5bd => {
  { let v: u32 = 0x46e5b8u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4e1ff4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x15u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x46e6e5; continue; }
  pc = 0x46e5e2; continue;
 }
 0x46e5e2 => {
  { cpu.pushad(); }
  { let v: u32 = 0x46e5eau32; cpu.edx = v; }
  pc = 0x46e5ef; continue;
 }
 0x46e5ef => {
  { let v: u32 = 0x46e5eau32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x208u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u32 = 0x46e609u32; cpu.edx = v; }
  pc = 0x46e60e; continue;
 }
 0x46e609 => {
  cpu.unimplemented(0x46e609, "Bound");
  cpu.unimplemented(0x46e60c, "Bound");
  pc = 0x46e60e; continue;
 }
 0x46e60e => {
  { let v: u32 = 0x46e609u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  { let v: u32 = 0x46e624u32; cpu.edx = v; }
  pc = 0x46e629; continue;
 }
 0x46e624 => {
  cpu.unimplemented(0x46e624, "Arpl");
  cpu.unimplemented(0x46e627, "Arpl");
  pc = 0x46e629; continue;
 }
 0x46e629 => {
  { let v: u32 = 0x46e624u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x20au32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u32 = 0x46e643u32; cpu.edx = v; }
  pc = 0x46e648; continue;
 }
 0x46e62d => {
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x66u32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0x66u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x20au32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x46e643u32; cpu.edx = v; }
  pc = 0x46e648; continue;
 }
 0x46e62f => {
  { let v: u8 = (cpu.eax as u8) & 0xe6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x66u32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0x66u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x20au32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x46e643u32; cpu.edx = v; }
  pc = 0x46e648; continue;
 }
 0x46e643 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x5ce90fu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.port_out(0x46u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.add8(cpu.r8(0x66u32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0x66u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x2u32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  { let v: u32 = 0x46e65fu32; cpu.edx = v; }
  pc = 0x46e664; continue;
 }
 0x46e648 => {
  { let v: u32 = 0x46e643u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  { let v: u32 = 0x46e65fu32; cpu.edx = v; }
  pc = 0x46e664; continue;
 }
 0x46e65f => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x5ce90fu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.port_out(0x46u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.add8(cpu.r8(0x66u32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0x66u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x20cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x46e67eu32; cpu.edx = v; }
  pc = 0x46e683; continue;
 }
 0x46e664 => {
  { let v: u32 = 0x46e65fu32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x20cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u32 = 0x46e67eu32; cpu.edx = v; }
  pc = 0x46e683; continue;
 }
 0x46e671 => {
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))); let b = (cpu.ecx as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); } { let v: u8 = a; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; } }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.pushad(); }
  { let v: u32 = 0x46e67eu32; cpu.edx = v; }
  pc = 0x46e683; continue;
 }
 0x46e67e => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.eax, 0x5ce90fu32); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x46e671; continue; }
  pc = 0x46e68b; continue;
 }
 0x46e683 => {
  { let v: u32 = 0x46e67eu32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  { let v: u32 = 0x46e69au32; cpu.edx = v; }
  pc = 0x46e69f; continue;
 }
 0x46e68b => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x66u32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0x66u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  { let v: u32 = 0x46e69au32; cpu.edx = v; }
  pc = 0x46e69f; continue;
 }
 0x46e69a => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x5ce90fu32); cpu.eax = v; }
  cpu.push(cpu.segments[1] as u32); cpu.push(0x46e6ac); cpu.call(0x610046e6); if cpu.returned_to != 0x46e6ac { return; }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.esi)); let b = (cpu.ecx as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); } { let v: u8 = a; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; } }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xffffffbau32.wrapping_add(cpu.eax)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffffbau32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xeb0046e6u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x68686868u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.eax, 0x5ce90fu32); cpu.eax = v; }
  { let v: u32 = 0x610046e6u32; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  { let v: u32 = 0x46e6d5u32; cpu.edx = v; }
  pc = 0x46e6da; continue;
 }
 0x46e69f => {
  { let v: u32 = 0x46e69au32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x20eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.pushad(); }
  { let v: u32 = 0x46e6b9u32; cpu.edx = v; }
  pc = 0x46e6be; continue;
 }
 0x46e6b9 => {
  { let v = 0x686868u32; cpu.push(v); }
  pc = 0x46e6be; continue;
 }
 0x46e6be => {
  { let v: u32 = 0x46e6b9u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  { let v: u32 = 0x46e6d5u32; cpu.edx = v; }
  pc = 0x46e6da; continue;
 }
 0x46e6d5 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x69u32.wrapping_add(cpu.ecx)), 0x5c70069u32); cpu.ebp = v; }
  cpu.unimplemented(0x46e6dc, "Psubsw");
  { cpu.port_out(0x46u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.r8(0xffffffc6u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffffc6u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5cf454u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x58u32.wrapping_add(cpu.edi)), cpu.ebx); cpu.w32(0x58u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x46e6da => {
  { let v: u32 = 0x46e6d5u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  pc = 0x46e6e5; continue;
 }
 0x46e6e5 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5cf454u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e5ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46e5ea); }
 let mut pc: u32 = 0x46e5ea;
 loop { match pc {
 0x46e5ea => {
  { cpu.popad(); }
  { cpu.popad(); }
  { cpu.popad(); }
  { cpu.popad(); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.eax, 0x5ce90fu32); cpu.eax = v; }
  cpu.call(0x610046e5); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e787(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x46e787); } r_0046e787(cpu, 0x46e787); }
pub fn f_0046e800(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x46e800); } r_0046e787(cpu, 0x46e800); }
fn r_0046e787(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x46e787 => {
  { cpu.pushad(); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x46e78e); f_00476a0e(cpu); if cpu.returned_to != 0x46e78e { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x46e852; continue; }
  pc = 0x46e795; continue;
 }
 0x46e795 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6b4cu32)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x10u32); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x46e7be); f_00420989(cpu); if cpu.returned_to != 0x46e7be { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x46e7ce; continue;
 }
 0x46e7ce => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x46e7d0; continue;
 }
 0x46e7d0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if cpu.zf { pc = 0x46e7e6; continue; }
  pc = 0x46e7dc; continue;
 }
 0x46e7dc => {
  { cpu.sub8((cpu.edx as u8), 0x33u8); }
  if cpu.zf { pc = 0x46e7e6; continue; }
  pc = 0x46e7e1; continue;
 }
 0x46e7e1 => {
  { cpu.sub8((cpu.edx as u8), 0x17u8); }
  if !cpu.zf { pc = 0x46e7eb; continue; }
  pc = 0x46e7e6; continue;
 }
 0x46e7e6 => {
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x46e7eb; continue;
 }
 0x46e7eb => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x46e7d0; continue; }
  pc = 0x46e7f3; continue;
 }
 0x46e7f3 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x80u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x6u32.wrapping_add(cpu.edi))); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x46e7ce; continue; }
  pc = 0x46e804; continue;
 }
 0x46e800 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.push(0x2b); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x46e7ce; continue; }
  pc = 0x46e804; continue;
 }
 0x46e804 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x208u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20au32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20cu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20eu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5cf454u32, v); }
  pc = 0x46e852; continue;
 }
 0x46e852 => {
  { cpu.sub32(cpu.r32(0x15u32.wrapping_add(cpu.edi)), 0xfffffffcu32); }
  if !cpu.zf { pc = 0x46e862; continue; }
  pc = 0x46e85b; continue;
 }
 0x46e85b => {
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.div16(v); }
  pc = 0x46e862; continue;
 }
 0x46e862 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e864(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46e864); }
 let mut pc: u32 = 0x46e864;
 loop { match pc {
 0x46e864 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x5cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d67beu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d67c0u32, v); }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.ebx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = cpu.r16(0x5a2b0eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6960u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xbu8; cpu.w8(0x4e1f8eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f7bu32, v); }
  { let v: u8 = 0xcu8; cpu.w8(0x4e1f79u32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x46e8af); f_00482ea4(cpu); if cpu.returned_to != 0x46e8af { return; }
  { let v: u32 = cpu.add32(cpu.r32(0x5a463du32), 0xau32); cpu.w32(0x5a463du32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e8bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46e8bb); }
 let mut pc: u32 = 0x46e8bb;
 loop { match pc {
 0x46e8bb => {
  { let v: u32 = 0x5ce9d8u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5cea2cu32, v); }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x46e8cb; continue;
 }
 0x46e8cb => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x46e8cb; continue; }
  pc = 0x46e8d6; continue;
 }
 0x46e8d6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046e8d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x46e8d8); }
 let mut pc: u32 = 0x46e8d8;
 loop { match pc {
 0x46e8d8 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x46e8e9; continue; }
  pc = 0x46e8dd; continue;
 }
 0x46e8dd => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x5ceb5cu32; cpu.edx = v; }
  pc = 0x46e8f5; continue;
 }
 0x46e8e9 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5cea4bu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  pc = 0x46e8f5; continue;
 }
 0x46e8f5 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x588841u32, v); }
  { let v: u32 = 0x9u32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5cea30u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x46e912); f_0040d5d0(cpu); if cpu.returned_to != 0x46e912 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46e91d; continue;
 }
 0x46e91d => {
  { let v: u32 = cpu.eax; cpu.w32(0x5cea30u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6828u32, v); }
  { let v: u32 = 0x5ceb64u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46e93d); f_0041bfed(cpu); if cpu.returned_to != 0x46e93d { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46e949; continue;
 }
 0x46e949 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46e95c); f_0041bfed(cpu); if cpu.returned_to != 0x46e95c { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46e968; continue;
 }
 0x46e968 => {
  { let v: u32 = 0x4d67dau32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46e97e); f_0041bfed(cpu); if cpu.returned_to != 0x46e97e { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46e98a; continue;
 }
 0x46e98a => {
  { let v: u32 = 0x42a237u32; cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46e9a0); f_0041bfed(cpu); if cpu.returned_to != 0x46e9a0 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46e9ac; continue;
 }
 0x46e9ac => {
  { let v: u32 = 0x4d6812u32; cpu.ecx = v; }
  { let v: u32 = 0x4d6752u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46e9c8); f_0041bfed(cpu); if cpu.returned_to != 0x46e9c8 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46e9d4; continue;
 }
 0x46e9d4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6a1cu32).wrapping_sub(cpu.eax); cpu.w32(0x4d6a1cu32, v); }
  { let v: u32 = cpu.r32(0x4d6a0cu32).wrapping_sub(cpu.eax); cpu.w32(0x4d6a0cu32, v); }
  { let v: u32 = cpu.r32(0x4d6a20u32).wrapping_sub(cpu.eax); cpu.w32(0x4d6a20u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x4d6a28u32; cpu.ecx = v; }
  { let v: u32 = 0x4d69ecu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ea09); f_0041bfed(cpu); if cpu.returned_to != 0x46ea09 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ea15; continue;
 }
 0x46ea15 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6a1cu32).wrapping_add(cpu.eax); cpu.w32(0x4d6a1cu32, v); }
  { let v: u32 = cpu.r32(0x4d6a0cu32).wrapping_add(cpu.eax); cpu.w32(0x4d6a0cu32, v); }
  { let v: u32 = cpu.r32(0x4d6a20u32).wrapping_add(cpu.eax); cpu.w32(0x4d6a20u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x4e1594u32; cpu.ecx = v; }
  { let v: u32 = 0x4e1553u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ea4a); f_0041bfed(cpu); if cpu.returned_to != 0x46ea4a { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ea56; continue;
 }
 0x46ea56 => {
  { let v: u32 = 0x4e1fa1u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ea6e); f_0041bfed(cpu); if cpu.returned_to != 0x46ea6e { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ea7a; continue;
 }
 0x46ea7a => {
  { let v: u32 = 0x4d952bu32; cpu.edx = v; }
  { let v: u32 = 0x8021u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ea90); f_0041bfed(cpu); if cpu.returned_to != 0x46ea90 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ea9c; continue;
 }
 0x46ea9c => {
  { let v: u32 = 0x4e240cu32; cpu.edx = v; }
  { let v: u32 = 0x6000u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46eab2); f_0041bfed(cpu); if cpu.returned_to != 0x46eab2 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46eabe; continue;
 }
 0x46eabe => {
  { let v: u32 = 0x4efa88u32; cpu.edx = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ead4); f_0041bfed(cpu); if cpu.returned_to != 0x46ead4 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46eae0; continue;
 }
 0x46eae0 => {
  { let v: u32 = 0x514860u32; cpu.edx = v; }
  { let v: u32 = 0xfa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46eaf6); f_0041bfed(cpu); if cpu.returned_to != 0x46eaf6 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46eb02; continue;
 }
 0x46eb02 => {
  { let v: u32 = 0x515800u32; cpu.edx = v; }
  { let v: u32 = 0x1f4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46eb18); f_0041bfed(cpu); if cpu.returned_to != 0x46eb18 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46eb24; continue;
 }
 0x46eb24 => {
  { let v: u32 = 0x5159f4u32; cpu.edx = v; }
  { let v: u32 = 0x8214u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46eb3a); f_0041bfed(cpu); if cpu.returned_to != 0x46eb3a { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46eb46; continue;
 }
 0x46eb46 => {
  { let v: u32 = 0x51e310u32; cpu.edx = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46eb5c); f_0041bfed(cpu); if cpu.returned_to != 0x46eb5c { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46eb68; continue;
 }
 0x46eb68 => {
  { let v: u32 = 0x51e3b0u32; cpu.edx = v; }
  { let v: u32 = 0x2940u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46eb7e); f_0041bfed(cpu); if cpu.returned_to != 0x46eb7e { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46eb8a; continue;
 }
 0x46eb8a => {
  { let v: u32 = 0x520cf0u32; cpu.edx = v; }
  { let v: u32 = 0x12cu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46eba0); f_0041bfed(cpu); if cpu.returned_to != 0x46eba0 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ebac; continue;
 }
 0x46ebac => {
  { let v: u32 = 0x520e1cu32; cpu.edx = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ebc2); f_0041bfed(cpu); if cpu.returned_to != 0x46ebc2 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ebce; continue;
 }
 0x46ebce => {
  { let v: u32 = 0x52195cu32; cpu.edx = v; }
  { let v: u32 = 0xbb8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ebe4); f_0041bfed(cpu); if cpu.returned_to != 0x46ebe4 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ebf0; continue;
 }
 0x46ebf0 => {
  { let v: u32 = 0x526020u32; cpu.edx = v; }
  { let v: u32 = 0x180u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ec06); f_0041bfed(cpu); if cpu.returned_to != 0x46ec06 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ec12; continue;
 }
 0x46ec12 => {
  { let v: u32 = 0x534c50u32; cpu.edx = v; }
  { let v: u32 = 0xe4c0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ec28); f_0041bfed(cpu); if cpu.returned_to != 0x46ec28 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ec34; continue;
 }
 0x46ec34 => {
  { let v: u32 = 0x52aca0u32; cpu.edx = v; }
  { let v: u32 = 0x1f4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ec4a); f_0041bfed(cpu); if cpu.returned_to != 0x46ec4a { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ec56; continue;
 }
 0x46ec56 => {
  { let v: u32 = 0x5261a0u32; cpu.edx = v; }
  { let v: u32 = 0x15e0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ec6c); f_0041bfed(cpu); if cpu.returned_to != 0x46ec6c { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ec78; continue;
 }
 0x46ec78 => {
  { let v: u32 = 0x52ff70u32; cpu.edx = v; }
  { let v: u32 = 0x4ce0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ec8e); f_0041bfed(cpu); if cpu.returned_to != 0x46ec8e { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ec9a; continue;
 }
 0x46ec9a => {
  { let v: u32 = 0x543110u32; cpu.edx = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ecb0); f_0041bfed(cpu); if cpu.returned_to != 0x46ecb0 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ecbc; continue;
 }
 0x46ecbc => {
  { let v: u32 = 0x543160u32; cpu.edx = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ecd2); f_0041bfed(cpu); if cpu.returned_to != 0x46ecd2 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ecde; continue;
 }
 0x46ecde => {
  { let v: u32 = 0x4eab34u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ecf4); f_0041bfed(cpu); if cpu.returned_to != 0x46ecf4 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ed00; continue;
 }
 0x46ed00 => {
  { let v: u32 = 0x4ea368u32; cpu.edx = v; }
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ed16); f_0041bfed(cpu); if cpu.returned_to != 0x46ed16 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ed22; continue;
 }
 0x46ed22 => {
  { let v: u32 = 0x4ea56cu32; cpu.edx = v; }
  { let v: u32 = 0x48u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ed38); f_0041bfed(cpu); if cpu.returned_to != 0x46ed38 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ed44; continue;
 }
 0x46ed44 => {
  { let v: u32 = 0x4b836cu32; cpu.edx = v; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ed5a); f_0041bfed(cpu); if cpu.returned_to != 0x46ed5a { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ed66; continue;
 }
 0x46ed66 => {
  { let v: u32 = 0x4ba3f4u32; cpu.edx = v; }
  { let v: u32 = 0xbeu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ed7c); f_0041bfed(cpu); if cpu.returned_to != 0x46ed7c { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ed88; continue;
 }
 0x46ed88 => {
  { let v: u32 = 0x4ba4b2u32; cpu.edx = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ed9e); f_0041bfed(cpu); if cpu.returned_to != 0x46ed9e { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46edaa; continue;
 }
 0x46edaa => {
  { let v: u32 = 0x4ba4d2u32; cpu.edx = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46edc0); f_0041bfed(cpu); if cpu.returned_to != 0x46edc0 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46edcc; continue;
 }
 0x46edcc => {
  { let v: u32 = 0x4acdb4u32; cpu.edx = v; }
  { let v: u32 = 0x2a8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ede2); f_0041bfed(cpu); if cpu.returned_to != 0x46ede2 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46edee; continue;
 }
 0x46edee => {
  { let v: u32 = 0x5a2e72u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ee04); f_0041bfed(cpu); if cpu.returned_to != 0x46ee04 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ee10; continue;
 }
 0x46ee10 => {
  { let v: u32 = 0x4b94ccu32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ee26); f_0041bfed(cpu); if cpu.returned_to != 0x46ee26 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ee32; continue;
 }
 0x46ee32 => {
  { let v: u32 = 0x4ba3e0u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ee48); f_0041bfed(cpu); if cpu.returned_to != 0x46ee48 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ee54; continue;
 }
 0x46ee54 => {
  { let v: u32 = 0x4b94c0u32; cpu.edx = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ee6a); f_0041bfed(cpu); if cpu.returned_to != 0x46ee6a { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ee76; continue;
 }
 0x46ee76 => {
  { let v: u32 = 0x4b94ceu32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ee8c); f_0041bfed(cpu); if cpu.returned_to != 0x46ee8c { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ee98; continue;
 }
 0x46ee98 => {
  { let v: u32 = 0x4b94d0u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46eeae); f_0041bfed(cpu); if cpu.returned_to != 0x46eeae { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46eeba; continue;
 }
 0x46eeba => {
  { let v: u32 = 0x575f24u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46eed0); f_0041bfed(cpu); if cpu.returned_to != 0x46eed0 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46eedc; continue;
 }
 0x46eedc => {
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0xd629u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46eef5); f_0041bfed(cpu); if cpu.returned_to != 0x46eef5 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ef01; continue;
 }
 0x46ef01 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b3980u32).wrapping_sub(cpu.eax); cpu.w32(0x5b3980u32, v); }
  { let v: u32 = cpu.r32(0x5b3984u32).wrapping_sub(cpu.eax); cpu.w32(0x5b3984u32, v); }
  { let v: u32 = cpu.r32(0x5b3988u32).wrapping_sub(cpu.eax); cpu.w32(0x5b3988u32, v); }
  { let v: u32 = cpu.r32(0x5b398cu32).wrapping_sub(cpu.eax); cpu.w32(0x5b398cu32, v); }
  { let v: u32 = cpu.r32(0x5b3598u32).wrapping_sub(cpu.eax); cpu.w32(0x5b3598u32, v); }
  { let v: u32 = cpu.r32(0x5b359cu32).wrapping_sub(cpu.eax); cpu.w32(0x5b359cu32, v); }
  { let v: u32 = cpu.r32(0x5b35a0u32).wrapping_sub(cpu.eax); cpu.w32(0x5b35a0u32, v); }
  { let v: u32 = cpu.r32(0x5b35a4u32).wrapping_sub(cpu.eax); cpu.w32(0x5b35a4u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x4e1fccu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ef4e); f_0041bfed(cpu); if cpu.returned_to != 0x46ef4e { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ef5a; continue;
 }
 0x46ef5a => {
  { let v: u32 = 0x5b3980u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ef70); f_0041bfed(cpu); if cpu.returned_to != 0x46ef70 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ef7c; continue;
 }
 0x46ef7c => {
  { let v: u32 = 0x5b3598u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46ef92); f_0041bfed(cpu); if cpu.returned_to != 0x46ef92 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46ef9e; continue;
 }
 0x46ef9e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b3980u32).wrapping_add(cpu.eax); cpu.w32(0x5b3980u32, v); }
  { let v: u32 = cpu.r32(0x5b3984u32).wrapping_add(cpu.eax); cpu.w32(0x5b3984u32, v); }
  { let v: u32 = cpu.r32(0x5b3988u32).wrapping_add(cpu.eax); cpu.w32(0x5b3988u32, v); }
  { let v: u32 = cpu.r32(0x5b398cu32).wrapping_add(cpu.eax); cpu.w32(0x5b398cu32, v); }
  { let v: u32 = cpu.r32(0x5b3598u32).wrapping_add(cpu.eax); cpu.w32(0x5b3598u32, v); }
  { let v: u32 = cpu.r32(0x5b359cu32).wrapping_add(cpu.eax); cpu.w32(0x5b359cu32, v); }
  { let v: u32 = cpu.r32(0x5b35a0u32).wrapping_add(cpu.eax); cpu.w32(0x5b35a0u32, v); }
  { let v: u32 = cpu.r32(0x5b35a4u32).wrapping_add(cpu.eax); cpu.w32(0x5b35a4u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x561fccu32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46efeb); f_0041bfed(cpu); if cpu.returned_to != 0x46efeb { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46eff7; continue;
 }
 0x46eff7 => {
  { let v: u32 = 0x5647ccu32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f00d); f_0041bfed(cpu); if cpu.returned_to != 0x46f00d { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f019; continue;
 }
 0x46f019 => {
  { let v: u32 = 0x566fccu32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f02f); f_0041bfed(cpu); if cpu.returned_to != 0x46f02f { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f03b; continue;
 }
 0x46f03b => {
  { let v: u32 = 0x5697ccu32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f051); f_0041bfed(cpu); if cpu.returned_to != 0x46f051 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f05d; continue;
 }
 0x46f05d => {
  { let v: u32 = 0x5d39ecu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f073); f_0041bfed(cpu); if cpu.returned_to != 0x46f073 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f07f; continue;
 }
 0x46f07f => {
  { let v: u32 = 0x5d39f0u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f095); f_0041bfed(cpu); if cpu.returned_to != 0x46f095 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f0a1; continue;
 }
 0x46f0a1 => {
  { let v: u32 = 0x5d39f4u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f0b7); f_0041bfed(cpu); if cpu.returned_to != 0x46f0b7 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f0c3; continue;
 }
 0x46f0c3 => {
  { let v: u32 = 0x5d39f8u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f0d9); f_0041bfed(cpu); if cpu.returned_to != 0x46f0d9 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f0e5; continue;
 }
 0x46f0e5 => {
  { let v: u32 = 0x5d39fcu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f0fb); f_0041bfed(cpu); if cpu.returned_to != 0x46f0fb { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f107; continue;
 }
 0x46f107 => {
  { let v: u32 = 0x5d3a00u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f11d); f_0041bfed(cpu); if cpu.returned_to != 0x46f11d { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f129; continue;
 }
 0x46f129 => {
  { let v: u32 = 0x5d3a0cu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f13f); f_0041bfed(cpu); if cpu.returned_to != 0x46f13f { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f14b; continue;
 }
 0x46f14b => {
  { let v: u32 = 0x5d3a10u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f161); f_0041bfed(cpu); if cpu.returned_to != 0x46f161 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f16d; continue;
 }
 0x46f16d => {
  { let v: u32 = 0x4d680eu32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f183); f_0041bfed(cpu); if cpu.returned_to != 0x46f183 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f18f; continue;
 }
 0x46f18f => {
  cpu.push(0x46f194); f_00470769(cpu); if cpu.returned_to != 0x46f194 { return; }
  { let v: u32 = 0x5242ccu32; cpu.edx = v; }
  { let v: u32 = 0x8a9u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f1aa); f_0041bfed(cpu); if cpu.returned_to != 0x46f1aa { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f1b6; continue;
 }
 0x46f1b6 => {
  { let v: u32 = 0x524b80u32; cpu.edx = v; }
  { let v: u32 = 0x14a0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f1cc); f_0041bfed(cpu); if cpu.returned_to != 0x46f1cc { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f1d8; continue;
 }
 0x46f1d8 => {
  cpu.push(0x46f1dd); f_004707cb(cpu); if cpu.returned_to != 0x46f1dd { return; }
  cpu.push(0x46f1e2); f_004706b7(cpu); if cpu.returned_to != 0x46f1e2 { return; }
  { let v: u32 = 0x4b94dcu32; cpu.edx = v; }
  { let v: u32 = 0xf00u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f1f8); f_0041bfed(cpu); if cpu.returned_to != 0x46f1f8 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f204; continue;
 }
 0x46f204 => {
  { let v: u32 = 0x4ad05cu32; cpu.edx = v; }
  { let v: u32 = 0xf00u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f21a); f_0041bfed(cpu); if cpu.returned_to != 0x46f21a { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f226; continue;
 }
 0x46f226 => {
  cpu.push(0x46f22b); f_00470710(cpu); if cpu.returned_to != 0x46f22b { return; }
  cpu.push(0x46f230); f_0047082d(cpu); if cpu.returned_to != 0x46f230 { return; }
  { let v: u32 = 0x522514u32; cpu.edx = v; }
  { let v: u32 = 0xaf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f246); f_0041bfed(cpu); if cpu.returned_to != 0x46f246 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f252; continue;
 }
 0x46f252 => {
  cpu.push(0x46f257); f_004708ae(cpu); if cpu.returned_to != 0x46f257 { return; }
  cpu.push(0x46f25c); f_0047092f(cpu); if cpu.returned_to != 0x46f25c { return; }
  { let v: u32 = 0x520e94u32; cpu.edx = v; }
  { let v: u32 = 0xac8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f272); f_0041bfed(cpu); if cpu.returned_to != 0x46f272 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f27e; continue;
 }
 0x46f27e => {
  { let v: u32 = cpu.r32(0x4d6b44u32); cpu.edx = v; }
  { let v: u32 = 0x9c40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f295); f_0041bfed(cpu); if cpu.returned_to != 0x46f295 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f2a1; continue;
 }
 0x46f2a1 => {
  cpu.push(0x46f2a6); f_0047095a(cpu); if cpu.returned_to != 0x46f2a6 { return; }
  cpu.push(0x46f2ab); f_00470cb7(cpu); if cpu.returned_to != 0x46f2ab { return; }
  { let v: u32 = 0x4efb50u32; cpu.edx = v; }
  { let v: u32 = 0x157c0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f2c1); f_0041bfed(cpu); if cpu.returned_to != 0x46f2c1 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f2cd; continue;
 }
 0x46f2cd => {
  cpu.push(0x46f2d2); f_00470d07(cpu); if cpu.returned_to != 0x46f2d2 { return; }
  cpu.push(0x46f2d7); f_00470c63(cpu); if cpu.returned_to != 0x46f2d7 { return; }
  { let v: u32 = 0x505310u32; cpu.edx = v; }
  { let v: u32 = 0x3e80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f2ed); f_0041bfed(cpu); if cpu.returned_to != 0x46f2ed { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f2f9; continue;
 }
 0x46f2f9 => {
  cpu.push(0x46f2fe); f_00470c8d(cpu); if cpu.returned_to != 0x46f2fe { return; }
  cpu.push(0x46f303); f_004709df(cpu); if cpu.returned_to != 0x46f303 { return; }
  { let v: u32 = 0x509290u32; cpu.edx = v; }
  { let v: u32 = 0xac80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f319); f_0041bfed(cpu); if cpu.returned_to != 0x46f319 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f325; continue;
 }
 0x46f325 => {
  cpu.push(0x46f32a); f_00470b21(cpu); if cpu.returned_to != 0x46f32a { return; }
  cpu.push(0x46f32f); f_00470985(cpu); if cpu.returned_to != 0x46f32f { return; }
  { let v: u32 = 0x513fc8u32; cpu.edx = v; }
  { let v: u32 = 0x898u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f345); f_0041bfed(cpu); if cpu.returned_to != 0x46f345 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f351; continue;
 }
 0x46f351 => {
  cpu.push(0x46f356); f_004709b2(cpu); if cpu.returned_to != 0x46f356 { return; }
  cpu.push(0x46f35b); f_00470d57(cpu); if cpu.returned_to != 0x46f35b { return; }
  { let v: u32 = 0x4ba552u32; cpu.edx = v; }
  { let v: u32 = 0x12c04u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f371); f_0041bfed(cpu); if cpu.returned_to != 0x46f371 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f37d; continue;
 }
 0x46f37d => {
  cpu.push(0x46f382); f_00470db0(cpu); if cpu.returned_to != 0x46f382 { return; }
  cpu.push(0x46f387); f_004705b7(cpu); if cpu.returned_to != 0x46f387 { return; }
  { let v: u32 = 0x5293a0u32; cpu.edx = v; }
  { let v: u32 = 0x1900u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f39d); f_0041bfed(cpu); if cpu.returned_to != 0x46f39d { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f3a9; continue;
 }
 0x46f3a9 => {
  cpu.push(0x46f3ae); f_004705cf(cpu); if cpu.returned_to != 0x46f3ae { return; }
  cpu.push(0x46f3b3); f_0047064f(cpu); if cpu.returned_to != 0x46f3b3 { return; }
  { let v: u32 = 0x52ae94u32; cpu.edx = v; }
  { let v: u32 = 0x780u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f3c9); f_0041bfed(cpu); if cpu.returned_to != 0x46f3c9 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f3d5; continue;
 }
 0x46f3d5 => {
  cpu.push(0x46f3da); f_00470683(cpu); if cpu.returned_to != 0x46f3da { return; }
  cpu.push(0x46f3df); f_004705e7(cpu); if cpu.returned_to != 0x46f3df { return; }
  { let v: u32 = 0x527780u32; cpu.edx = v; }
  { let v: u32 = 0x1c20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f3f5); f_0041bfed(cpu); if cpu.returned_to != 0x46f3f5 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f401; continue;
 }
 0x46f401 => {
  cpu.push(0x46f406); f_0047061b(cpu); if cpu.returned_to != 0x46f406 { return; }
  cpu.push(0x46f40b); f_00470544(cpu); if cpu.returned_to != 0x46f40b { return; }
  { let v: u32 = 0x52bbf0u32; cpu.edx = v; }
  { let v: u32 = 0x4380u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f421); f_0041bfed(cpu); if cpu.returned_to != 0x46f421 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f429; continue;
 }
 0x46f429 => {
  cpu.push(0x46f42e); f_0047057c(cpu); if cpu.returned_to != 0x46f42e { return; }
  cpu.push(0x46f433); f_004704b2(cpu); if cpu.returned_to != 0x46f433 { return; }
  { let v: u32 = 0x559754u32; cpu.edx = v; }
  { let v: u32 = 0x1f40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f449); f_0041bfed(cpu); if cpu.returned_to != 0x46f449 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f451; continue;
 }
 0x46f451 => {
  cpu.push(0x46f456); f_004704fb(cpu); if cpu.returned_to != 0x46f456 { return; }
  cpu.push(0x46f45b); f_00470458(cpu); if cpu.returned_to != 0x46f45b { return; }
  { let v: u32 = 0x4e1ff4u32; cpu.edx = v; }
  { let v: u32 = 0x410u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x46f471); f_0041bfed(cpu); if cpu.returned_to != 0x46f471 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f479; continue;
 }
 0x46f479 => {
  cpu.push(0x46f47e); f_00470485(cpu); if cpu.returned_to != 0x46f47e { return; }
  { let v = cpu.r32(0x5cea30u32); cpu.push(v); }
  cpu.push(0x46f489); f_0040d640(cpu); if cpu.returned_to != 0x46f489 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x46f492; continue; }
  pc = 0x46f490; continue;
 }
 0x46f490 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x46f492 => {
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x46f4a5; continue; }
  pc = 0x46f49c; continue;
 }
 0x46f49c => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x46f4a2); f_0040d640(cpu); if cpu.returned_to != 0x46f4a2 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x46f4a5; continue;
 }
 0x46f4a5 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0046f4a7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x46f4a7); } r_0046f4a7(cpu, 0x46f4a7); }
pub fn f_00470000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470000); } r_0046f4a7(cpu, 0x470000); }
pub fn f_00470009(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470009); } r_0046f4a7(cpu, 0x470009); }
pub fn f_00470010(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470010); } r_0046f4a7(cpu, 0x470010); }
pub fn f_0047001d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47001d); } r_0046f4a7(cpu, 0x47001d); }
pub fn f_00470022(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470022); } r_0046f4a7(cpu, 0x470022); }
pub fn f_00470023(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470023); } r_0046f4a7(cpu, 0x470023); }
pub fn f_00470026(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470026); } r_0046f4a7(cpu, 0x470026); }
pub fn f_00470027(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470027); } r_0046f4a7(cpu, 0x470027); }
pub fn f_0047002d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47002d); } r_0046f4a7(cpu, 0x47002d); }
pub fn f_0047003f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47003f); } r_0046f4a7(cpu, 0x47003f); }
pub fn f_00470046(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470046); } r_0046f4a7(cpu, 0x470046); }
pub fn f_00470047(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470047); } r_0046f4a7(cpu, 0x470047); }
pub fn f_00470059(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470059); } r_0046f4a7(cpu, 0x470059); }
pub fn f_00470060(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470060); } r_0046f4a7(cpu, 0x470060); }
pub fn f_00470061(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470061); } r_0046f4a7(cpu, 0x470061); }
pub fn f_0047006b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47006b); } r_0046f4a7(cpu, 0x47006b); }
pub fn f_00470070(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470070); } r_0046f4a7(cpu, 0x470070); }
pub fn f_00470077(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470077); } r_0046f4a7(cpu, 0x470077); }
pub fn f_00470078(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470078); } r_0046f4a7(cpu, 0x470078); }
pub fn f_00470079(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470079); } r_0046f4a7(cpu, 0x470079); }
pub fn f_0047007e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47007e); } r_0046f4a7(cpu, 0x47007e); }
pub fn f_00470080(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470080); } r_0046f4a7(cpu, 0x470080); }
pub fn f_00470082(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470082); } r_0046f4a7(cpu, 0x470082); }
pub fn f_00470088(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x470088); } r_0046f4a7(cpu, 0x470088); }
pub fn f_004700a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4700a4); } r_0046f4a7(cpu, 0x4700a4); }
fn r_0046f4a7(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x46f4a7 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x46f4b5; continue; }
  pc = 0x46f4b0; continue;
 }
 0x46f4b0 => {
  cpu.push(0x46f4b5); f_0043daab(cpu); if cpu.returned_to != 0x46f4b5 { return; }
  pc = 0x46f4b5; continue;
 }
 0x46f4b5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x46f4c4); f_00402d70(cpu); if cpu.returned_to != 0x46f4c4 { return; }
  cpu.push(0x46f4c9); f_00405320(cpu); if cpu.returned_to != 0x46f4c9 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cea73u32, v); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x46f4e6; continue; }
  pc = 0x46f4d8; continue;
 }
 0x46f4d8 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ceae0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  pc = 0x46f4f2; continue;
 }
 0x46f4e6 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5cea4bu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  pc = 0x46f4f2; continue;
 }
 0x46f4f2 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0x9u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5cea34u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x46f50b); f_0040d370(cpu); if cpu.returned_to != 0x46f50b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6824u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x4d6824u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f521; continue;
 }
 0x46f521 => {
  { let v: u32 = cpu.r32(0x4d6824u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5cea34u32, v); }
  { let v: u32 = 0x5ceb64u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f541); f_0041c901(cpu); if cpu.returned_to != 0x46f541 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f54d; continue;
 }
 0x46f54d => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f560); f_0041c901(cpu); if cpu.returned_to != 0x46f560 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f56c; continue;
 }
 0x46f56c => {
  { let v: u32 = 0x4d67dau32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f582); f_0041c901(cpu); if cpu.returned_to != 0x46f582 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f58e; continue;
 }
 0x46f58e => {
  { let v: u32 = 0x5cea74u32; cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f5a4); f_0041c901(cpu); if cpu.returned_to != 0x46f5a4 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f5b0; continue;
 }
 0x46f5b0 => {
  { let v: u32 = 0x4d6812u32; cpu.ecx = v; }
  { let v: u32 = 0x4d6752u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f5cc); f_0041c901(cpu); if cpu.returned_to != 0x46f5cc { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f5d8; continue;
 }
 0x46f5d8 => {
  { let v: u32 = 0x4d6a28u32; cpu.ecx = v; }
  { let v: u32 = 0x4d69ecu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f5f4); f_0041c901(cpu); if cpu.returned_to != 0x46f5f4 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f600; continue;
 }
 0x46f600 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6a1cu32).wrapping_add(cpu.eax); cpu.w32(0x4d6a1cu32, v); }
  { let v: u32 = cpu.r32(0x4d6a0cu32).wrapping_add(cpu.eax); cpu.w32(0x4d6a0cu32, v); }
  { let v: u32 = cpu.r32(0x4d6a20u32).wrapping_add(cpu.eax); cpu.w32(0x4d6a20u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x4e1594u32; cpu.ecx = v; }
  { let v: u32 = 0x4e1553u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f635); f_0041c901(cpu); if cpu.returned_to != 0x46f635 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f641; continue;
 }
 0x46f641 => {
  { let v: u32 = 0x576e8cu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f659); f_0041c901(cpu); if cpu.returned_to != 0x46f659 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f665; continue;
 }
 0x46f665 => {
  { let v: u32 = 0x4d952bu32; cpu.edx = v; }
  { let v: u32 = 0x8021u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f67b); f_0041c901(cpu); if cpu.returned_to != 0x46f67b { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f687; continue;
 }
 0x46f687 => {
  { let v: u32 = 0x4e240cu32; cpu.edx = v; }
  { let v: u32 = 0x6000u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f69d); f_0041c901(cpu); if cpu.returned_to != 0x46f69d { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f6a9; continue;
 }
 0x46f6a9 => {
  { let v: u32 = 0x4efa88u32; cpu.edx = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f6bf); f_0041c901(cpu); if cpu.returned_to != 0x46f6bf { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f6cb; continue;
 }
 0x46f6cb => {
  { let v: u32 = 0x514860u32; cpu.edx = v; }
  { let v: u32 = 0xfa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f6e1); f_0041c901(cpu); if cpu.returned_to != 0x46f6e1 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f6ed; continue;
 }
 0x46f6ed => {
  { let v: u32 = 0x515800u32; cpu.edx = v; }
  { let v: u32 = 0x1f4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f703); f_0041c901(cpu); if cpu.returned_to != 0x46f703 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f70f; continue;
 }
 0x46f70f => {
  { let v: u32 = 0x5159f4u32; cpu.edx = v; }
  { let v: u32 = 0x8214u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f725); f_0041c901(cpu); if cpu.returned_to != 0x46f725 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f731; continue;
 }
 0x46f731 => {
  { let v: u32 = 0x51e310u32; cpu.edx = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f747); f_0041c901(cpu); if cpu.returned_to != 0x46f747 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f753; continue;
 }
 0x46f753 => {
  { let v: u32 = 0x51e3b0u32; cpu.edx = v; }
  { let v: u32 = 0x2940u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f769); f_0041c901(cpu); if cpu.returned_to != 0x46f769 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f775; continue;
 }
 0x46f775 => {
  { let v: u32 = 0x520cf0u32; cpu.edx = v; }
  { let v: u32 = 0x12cu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f78b); f_0041c901(cpu); if cpu.returned_to != 0x46f78b { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f797; continue;
 }
 0x46f797 => {
  { let v: u32 = 0x520e1cu32; cpu.edx = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f7ad); f_0041c901(cpu); if cpu.returned_to != 0x46f7ad { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f7b9; continue;
 }
 0x46f7b9 => {
  { let v: u32 = 0x52195cu32; cpu.edx = v; }
  { let v: u32 = 0xbb8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f7cf); f_0041c901(cpu); if cpu.returned_to != 0x46f7cf { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f7db; continue;
 }
 0x46f7db => {
  { let v: u32 = 0x526020u32; cpu.edx = v; }
  { let v: u32 = 0x180u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f7f1); f_0041c901(cpu); if cpu.returned_to != 0x46f7f1 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f7fd; continue;
 }
 0x46f7fd => {
  { let v: u32 = 0x534c50u32; cpu.edx = v; }
  { let v: u32 = 0xe4c0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f813); f_0041c901(cpu); if cpu.returned_to != 0x46f813 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f81f; continue;
 }
 0x46f81f => {
  { let v: u32 = 0x52aca0u32; cpu.edx = v; }
  { let v: u32 = 0x1f4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f835); f_0041c901(cpu); if cpu.returned_to != 0x46f835 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f841; continue;
 }
 0x46f841 => {
  { let v: u32 = 0x5261a0u32; cpu.edx = v; }
  { let v: u32 = 0x15e0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f857); f_0041c901(cpu); if cpu.returned_to != 0x46f857 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f863; continue;
 }
 0x46f863 => {
  { let v: u32 = 0x52ff70u32; cpu.edx = v; }
  { let v: u32 = 0x4ce0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f879); f_0041c901(cpu); if cpu.returned_to != 0x46f879 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f885; continue;
 }
 0x46f885 => {
  { let v: u32 = 0x543110u32; cpu.edx = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f89b); f_0041c901(cpu); if cpu.returned_to != 0x46f89b { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f8a7; continue;
 }
 0x46f8a7 => {
  { let v: u32 = 0x543160u32; cpu.edx = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f8bd); f_0041c901(cpu); if cpu.returned_to != 0x46f8bd { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f8c9; continue;
 }
 0x46f8c9 => {
  { let v: u32 = 0x4eab34u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f8df); f_0041c901(cpu); if cpu.returned_to != 0x46f8df { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f8eb; continue;
 }
 0x46f8eb => {
  { let v: u32 = 0x4ea368u32; cpu.edx = v; }
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f901); f_0041c901(cpu); if cpu.returned_to != 0x46f901 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f90d; continue;
 }
 0x46f90d => {
  { let v: u32 = 0x4ea56cu32; cpu.edx = v; }
  { let v: u32 = 0x48u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f923); f_0041c901(cpu); if cpu.returned_to != 0x46f923 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f92f; continue;
 }
 0x46f92f => {
  { let v: u32 = 0x4b836cu32; cpu.edx = v; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f945); f_0041c901(cpu); if cpu.returned_to != 0x46f945 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f951; continue;
 }
 0x46f951 => {
  { let v: u32 = 0x4ba3f4u32; cpu.edx = v; }
  { let v: u32 = 0xbeu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f967); f_0041c901(cpu); if cpu.returned_to != 0x46f967 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f973; continue;
 }
 0x46f973 => {
  { let v: u32 = 0x4ba4b2u32; cpu.edx = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f989); f_0041c901(cpu); if cpu.returned_to != 0x46f989 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f995; continue;
 }
 0x46f995 => {
  { let v: u32 = 0x4ba4d2u32; cpu.edx = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f9ab); f_0041c901(cpu); if cpu.returned_to != 0x46f9ab { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f9b7; continue;
 }
 0x46f9b7 => {
  { let v: u32 = 0x4acdb4u32; cpu.edx = v; }
  { let v: u32 = 0x2a8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f9cd); f_0041c901(cpu); if cpu.returned_to != 0x46f9cd { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f9d9; continue;
 }
 0x46f9d9 => {
  { let v: u32 = 0x5a2e72u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46f9ef); f_0041c901(cpu); if cpu.returned_to != 0x46f9ef { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46f9fb; continue;
 }
 0x46f9fb => {
  { let v: u32 = 0x4b94ccu32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fa11); f_0041c901(cpu); if cpu.returned_to != 0x46fa11 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fa1d; continue;
 }
 0x46fa1d => {
  { let v: u32 = 0x4ba3e0u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fa33); f_0041c901(cpu); if cpu.returned_to != 0x46fa33 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fa3f; continue;
 }
 0x46fa3f => {
  { let v: u32 = 0x4b94c0u32; cpu.edx = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fa55); f_0041c901(cpu); if cpu.returned_to != 0x46fa55 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fa61; continue;
 }
 0x46fa61 => {
  { let v: u32 = 0x4b94ceu32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fa77); f_0041c901(cpu); if cpu.returned_to != 0x46fa77 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fa83; continue;
 }
 0x46fa83 => {
  { let v: u32 = 0x4b94d0u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fa99); f_0041c901(cpu); if cpu.returned_to != 0x46fa99 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46faa5; continue;
 }
 0x46faa5 => {
  { let v: u32 = 0x575f24u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fabb); f_0041c901(cpu); if cpu.returned_to != 0x46fabb { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fac7; continue;
 }
 0x46fac7 => {
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0xd629u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fae0); f_0041c901(cpu); if cpu.returned_to != 0x46fae0 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46faec; continue;
 }
 0x46faec => {
  { let v: u32 = 0x4e1fccu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fb02); f_0041c901(cpu); if cpu.returned_to != 0x46fb02 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fb0e; continue;
 }
 0x46fb0e => {
  { let v: u32 = 0x5b3980u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fb24); f_0041c901(cpu); if cpu.returned_to != 0x46fb24 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fb30; continue;
 }
 0x46fb30 => {
  { let v: u32 = 0x5b3598u32; cpu.edx = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fb46); f_0041c901(cpu); if cpu.returned_to != 0x46fb46 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fb52; continue;
 }
 0x46fb52 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5b3980u32).wrapping_add(cpu.eax); cpu.w32(0x5b3980u32, v); }
  { let v: u32 = cpu.r32(0x5b3984u32).wrapping_add(cpu.eax); cpu.w32(0x5b3984u32, v); }
  { let v: u32 = cpu.r32(0x5b3988u32).wrapping_add(cpu.eax); cpu.w32(0x5b3988u32, v); }
  { let v: u32 = cpu.r32(0x5b398cu32).wrapping_add(cpu.eax); cpu.w32(0x5b398cu32, v); }
  { let v: u32 = cpu.r32(0x5b3598u32).wrapping_add(cpu.eax); cpu.w32(0x5b3598u32, v); }
  { let v: u32 = cpu.r32(0x5b359cu32).wrapping_add(cpu.eax); cpu.w32(0x5b359cu32, v); }
  { let v: u32 = cpu.r32(0x5b35a0u32).wrapping_add(cpu.eax); cpu.w32(0x5b35a0u32, v); }
  { let v: u32 = cpu.r32(0x5b35a4u32).wrapping_add(cpu.eax); cpu.w32(0x5b35a4u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x561fccu32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fb9f); f_0041c901(cpu); if cpu.returned_to != 0x46fb9f { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fbab; continue;
 }
 0x46fbab => {
  { let v: u32 = 0x5647ccu32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fbc1); f_0041c901(cpu); if cpu.returned_to != 0x46fbc1 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fbcd; continue;
 }
 0x46fbcd => {
  { let v: u32 = 0x566fccu32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fbe3); f_0041c901(cpu); if cpu.returned_to != 0x46fbe3 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fbef; continue;
 }
 0x46fbef => {
  { let v: u32 = 0x5697ccu32; cpu.edx = v; }
  { let v: u32 = 0x2800u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fc05); f_0041c901(cpu); if cpu.returned_to != 0x46fc05 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fc11; continue;
 }
 0x46fc11 => {
  { let v: u32 = 0x5d39ecu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fc27); f_0041c901(cpu); if cpu.returned_to != 0x46fc27 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fc33; continue;
 }
 0x46fc33 => {
  { let v: u32 = 0x5d39f0u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fc49); f_0041c901(cpu); if cpu.returned_to != 0x46fc49 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fc55; continue;
 }
 0x46fc55 => {
  { let v: u32 = 0x5d39f4u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fc6b); f_0041c901(cpu); if cpu.returned_to != 0x46fc6b { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fc77; continue;
 }
 0x46fc77 => {
  { let v: u32 = 0x5d39f8u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fc8d); f_0041c901(cpu); if cpu.returned_to != 0x46fc8d { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fc99; continue;
 }
 0x46fc99 => {
  { let v: u32 = 0x5d39fcu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fcaf); f_0041c901(cpu); if cpu.returned_to != 0x46fcaf { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fcbb; continue;
 }
 0x46fcbb => {
  { let v: u32 = 0x5d3a00u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fcd1); f_0041c901(cpu); if cpu.returned_to != 0x46fcd1 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fcdd; continue;
 }
 0x46fcdd => {
  { let v: u32 = 0x5d3a0cu32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fcf3); f_0041c901(cpu); if cpu.returned_to != 0x46fcf3 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fcff; continue;
 }
 0x46fcff => {
  { let v: u32 = 0x5d3a10u32; cpu.edx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fd15); f_0041c901(cpu); if cpu.returned_to != 0x46fd15 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fd21; continue;
 }
 0x46fd21 => {
  { let v: u32 = 0x4d680eu32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fd37); f_0041c901(cpu); if cpu.returned_to != 0x46fd37 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fd43; continue;
 }
 0x46fd43 => {
  { let v: u32 = 0x5242ccu32; cpu.edx = v; }
  { let v: u32 = 0x8a9u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fd59); f_0041c901(cpu); if cpu.returned_to != 0x46fd59 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fd65; continue;
 }
 0x46fd65 => {
  { let v: u32 = 0x524b80u32; cpu.edx = v; }
  { let v: u32 = 0x14a0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fd7b); f_0041c901(cpu); if cpu.returned_to != 0x46fd7b { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fd87; continue;
 }
 0x46fd87 => {
  cpu.push(0x46fd8c); f_004707cb(cpu); if cpu.returned_to != 0x46fd8c { return; }
  { let v: u32 = 0x4b94dcu32; cpu.edx = v; }
  { let v: u32 = 0xf00u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fda2); f_0041c901(cpu); if cpu.returned_to != 0x46fda2 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fdae; continue;
 }
 0x46fdae => {
  { let v: u32 = 0x4ad05cu32; cpu.edx = v; }
  { let v: u32 = 0xf00u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fdc4); f_0041c901(cpu); if cpu.returned_to != 0x46fdc4 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fdd0; continue;
 }
 0x46fdd0 => {
  cpu.push(0x46fdd5); f_00470710(cpu); if cpu.returned_to != 0x46fdd5 { return; }
  { let v: u32 = 0x522514u32; cpu.edx = v; }
  { let v: u32 = 0xaf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fdeb); f_0041c901(cpu); if cpu.returned_to != 0x46fdeb { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fdf7; continue;
 }
 0x46fdf7 => {
  cpu.push(0x46fdfc); f_004708ae(cpu); if cpu.returned_to != 0x46fdfc { return; }
  { let v: u32 = 0x520e94u32; cpu.edx = v; }
  { let v: u32 = 0xac8u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fe12); f_0041c901(cpu); if cpu.returned_to != 0x46fe12 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fe1e; continue;
 }
 0x46fe1e => {
  { let v: u32 = cpu.r32(0x4d6b44u32); cpu.edx = v; }
  { let v: u32 = 0x9c40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fe35); f_0041c901(cpu); if cpu.returned_to != 0x46fe35 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fe41; continue;
 }
 0x46fe41 => {
  cpu.push(0x46fe46); f_0047095a(cpu); if cpu.returned_to != 0x46fe46 { return; }
  { let v: u32 = 0x4efb50u32; cpu.edx = v; }
  { let v: u32 = 0x157c0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fe5c); f_0041c901(cpu); if cpu.returned_to != 0x46fe5c { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fe68; continue;
 }
 0x46fe68 => {
  cpu.push(0x46fe6d); f_00470d07(cpu); if cpu.returned_to != 0x46fe6d { return; }
  { let v: u32 = 0x505310u32; cpu.edx = v; }
  { let v: u32 = 0x3e80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fe83); f_0041c901(cpu); if cpu.returned_to != 0x46fe83 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fe8f; continue;
 }
 0x46fe8f => {
  cpu.push(0x46fe94); f_00470c8d(cpu); if cpu.returned_to != 0x46fe94 { return; }
  { let v: u32 = 0x509290u32; cpu.edx = v; }
  { let v: u32 = 0xac80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46feaa); f_0041c901(cpu); if cpu.returned_to != 0x46feaa { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46feb6; continue;
 }
 0x46feb6 => {
  cpu.push(0x46febb); f_00470b21(cpu); if cpu.returned_to != 0x46febb { return; }
  { let v: u32 = 0x513fc8u32; cpu.edx = v; }
  { let v: u32 = 0x898u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fed1); f_0041c901(cpu); if cpu.returned_to != 0x46fed1 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46fedd; continue;
 }
 0x46fedd => {
  cpu.push(0x46fee2); f_004709b2(cpu); if cpu.returned_to != 0x46fee2 { return; }
  { let v: u32 = 0x4ba552u32; cpu.edx = v; }
  { let v: u32 = 0x12c04u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46fef8); f_0041c901(cpu); if cpu.returned_to != 0x46fef8 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46ff04; continue;
 }
 0x46ff04 => {
  cpu.push(0x46ff09); f_00470db0(cpu); if cpu.returned_to != 0x46ff09 { return; }
  { let v: u32 = 0x5293a0u32; cpu.edx = v; }
  { let v: u32 = 0x1900u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46ff1f); f_0041c901(cpu); if cpu.returned_to != 0x46ff1f { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46ff2b; continue;
 }
 0x46ff2b => {
  cpu.push(0x46ff30); f_004705cf(cpu); if cpu.returned_to != 0x46ff30 { return; }
  { let v: u32 = 0x52ae94u32; cpu.edx = v; }
  { let v: u32 = 0x780u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46ff46); f_0041c901(cpu); if cpu.returned_to != 0x46ff46 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46ff52; continue;
 }
 0x46ff52 => {
  cpu.push(0x46ff57); f_00470683(cpu); if cpu.returned_to != 0x46ff57 { return; }
  { let v: u32 = 0x527780u32; cpu.edx = v; }
  { let v: u32 = 0x1c20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46ff6d); f_0041c901(cpu); if cpu.returned_to != 0x46ff6d { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46ff79; continue;
 }
 0x46ff79 => {
  cpu.push(0x46ff7e); f_0047061b(cpu); if cpu.returned_to != 0x46ff7e { return; }
  { let v: u32 = 0x52bbf0u32; cpu.edx = v; }
  { let v: u32 = 0x4380u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46ff94); f_0041c901(cpu); if cpu.returned_to != 0x46ff94 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46ffa0; continue;
 }
 0x46ffa0 => {
  cpu.push(0x46ffa5); f_0047057c(cpu); if cpu.returned_to != 0x46ffa5 { return; }
  { let v: u32 = 0x559754u32; cpu.edx = v; }
  { let v: u32 = 0x1f40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46ffbb); f_0041c901(cpu); if cpu.returned_to != 0x46ffbb { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  pc = 0x46ffc1; continue;
 }
 0x46ffc1 => {
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46ffc7; continue;
 }
 0x46ffc7 => {
  cpu.push(0x46ffcc); f_004704fb(cpu); if cpu.returned_to != 0x46ffcc { return; }
  { let v: u32 = 0x4e1ff4u32; cpu.edx = v; }
  { let v: u32 = 0x410u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x46ffe2); f_0041c901(cpu); if cpu.returned_to != 0x46ffe2 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x46ffee; continue;
 }
 0x46ffee => {
  cpu.push(0x46fff3); f_00470485(cpu); if cpu.returned_to != 0x46fff3 { return; }
  { cpu.pushad(); }
  cpu.push(0x46fff9); f_00450cfe(cpu); if cpu.returned_to != 0x46fff9 { return; }
  let t = cpu.r32(0x4d6f70u32); cpu.push(0x46ffff); cpu.call(t); if cpu.returned_to != 0x46ffff { return; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x470000; continue;
 }
 0x46fffc => {
  cpu.unimplemented(0x46fffc, "Outsd");
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x51u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x51u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x470007); f_004478a1(cpu); if cpu.returned_to != 0x470007 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x470008; continue;
 }
 0x470000 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x470007); f_004478a1(cpu); if cpu.returned_to != 0x470007 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x470008; continue;
 }
 0x470008 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x470009; continue;
 }
 0x470009 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x470017); f_0047ac88(cpu); if cpu.returned_to != 0x470017 { return; }
  { cpu.popad(); }
  cpu.push(0x47001d); f_0043f250(cpu); if cpu.returned_to != 0x47001d { return; }
  pc = 0x47001d; continue;
 }
 0x470010 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), ((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.of { cpu.poll(); pc = 0x46ffc1; continue; }
  pc = 0x470015; continue;
 }
 0x470014 => {
  { cpu.lods(1, false); }
  pc = 0x470015; continue;
 }
 0x470015 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.popad(); }
  cpu.push(0x47001d); f_0043f250(cpu); if cpu.returned_to != 0x47001d { return; }
  pc = 0x47001d; continue;
 }
 0x47001d => {
  cpu.push(0x470022); f_004172f7(cpu); if cpu.returned_to != 0x470022 { return; }
  pc = 0x470022; continue;
 }
 0x470020 => {
  { cpu.interrupts_off = true; }
  cpu.call(0x470021); return;
 }
 0x470022 => {
  { let v: u32 = 0x55b6b8u32; cpu.esi = v; }
  pc = 0x470027; continue;
 }
 0x470023 => {
  { let v: u32 = 0xbf0055b6u32; cpu.eax = v; }
  { let v: u32 = 0xb90055b9u32; cpu.eax = v; }
  pc = 0x47002d; continue;
 }
 0x470026 => {
  { let v: u8 = cpu.r8(0x55b9b8u32.wrapping_add(cpu.edi)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x55b9b8u32.wrapping_add(cpu.edi), v); }
  pc = 0x47002c; continue;
 }
 0x470027 => {
  { let v: u32 = 0x55b9b8u32; cpu.edi = v; }
  pc = 0x47002c; continue;
 }
 0x47002c => {
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b358cu32, v); }
  { cpu.popad(); }
  { let v = cpu.r32(0x5cea34u32); cpu.push(v); }
  pc = 0x470044; continue;
 }
 0x47002d => {
  { let v: u8 = cpu.rol8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0u32); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, false); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b358cu32, v); }
  { cpu.popad(); }
  { let v = cpu.r32(0x5cea34u32); cpu.push(v); }
  pc = 0x470044; continue;
 }
 0x470038 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffffffu32.wrapping_add(cpu.ecx), v); }
  pc = 0x47003f; continue;
 }
 0x47003f => {
  { let v: u32 = cpu.eax ^ 0x5cea34u32; cpu.eax = v; }
  pc = 0x470044; continue;
 }
 0x470044 => {
  cpu.push(0x470049); f_0040d640(cpu); if cpu.returned_to != 0x470049 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x470050; continue;
 }
 0x470046 => {
  cpu.unimplemented(0x470046, "Aad");
  { let v: u32 = cpu.inc32(cpu.r32(0xc08504c4u32.wrapping_add(cpu.ebx))); cpu.w32(0xc08504c4u32.wrapping_add(cpu.ebx), v); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x470050; continue;
 }
 0x470047 => {
  { cpu.cf = true; }
  { let v: u32 = cpu.inc32(cpu.r32(0xc08504c4u32.wrapping_add(cpu.ebx))); cpu.w32(0xc08504c4u32.wrapping_add(cpu.ebx), v); }
  if !cpu.zf { pc = 0x4700ce; continue; }
  pc = 0x470050; continue;
 }
 0x470050 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5cf454u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x56c9dau32, v); }
  { cpu.sub8(cpu.r8(0x56d2d7u32), 0x0u8); }
  if !cpu.zf { pc = 0x470067; continue; }
  pc = 0x470067; continue;
 }
 0x470059 => {
  cpu.unimplemented(0x470059, "Fcmove");
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(cpu.r8(0x56d2d7u32), 0x0u8); }
  if !cpu.zf { pc = 0x470067; continue; }
  pc = 0x470067; continue;
 }
 0x470060 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  pc = 0x470061; continue;
 }
 0x470061 => {
  { let v: u8 = cpu.rcl8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.ecx & 0xff)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.ebp)), ((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x470067; continue;
 }
 0x470067 => {
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b70u32, v); }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x470079; continue;
 }
 0x47006b => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffffa3u32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa3u32.wrapping_add(cpu.esi), v); }
  if cpu.of { cpu.poll(); pc = 0x46fffc; continue; }
  pc = 0x470071; continue;
 }
 0x470070 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x470079; continue;
 }
 0x470071 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffffa1u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa1u32.wrapping_add(cpu.esi), v); }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  pc = 0x470078; continue;
 }
 0x470077 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  pc = 0x470078; continue;
 }
 0x470078 => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffffa3u32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa3u32.wrapping_add(cpu.esi), v); }
  if cpu.cf { cpu.poll(); pc = 0x470008; continue; }
  pc = 0x47007d; continue;
 }
 0x470079 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b72u32, v); }
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b74u32, v); }
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b76u32, v); }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b78u32, v); }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b7au32, v); }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b7cu32, v); }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b7eu32, v); }
  cpu.push(0x4700cc); f_0041c1a0(cpu); if cpu.returned_to != 0x4700cc { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47007d => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x47007e; continue;
 }
 0x47007e => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffffa1u32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa1u32.wrapping_add(cpu.esi), v); }
  cpu.push(cpu.segments[1] as u32); cpu.push(0x470088); cpu.call(0x66004d67); if cpu.returned_to != 0x470088 { return; }
  pc = 0x470088; continue;
 }
 0x470080 => {
  { let v: u32 = cpu.r32(0x4d679au32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b74u32, v); }
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b76u32, v); }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b78u32, v); }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b7au32, v); }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b7cu32, v); }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b7eu32, v); }
  cpu.push(0x4700cc); f_0041c1a0(cpu); if cpu.returned_to != 0x4700cc { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x470082 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffffa3u32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa3u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { cpu.poll(); pc = 0x470014; continue; }
  pc = 0x470089; continue;
 }
 0x470088 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b76u32, v); }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b78u32, v); }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b7au32, v); }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b7cu32, v); }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b7eu32, v); }
  cpu.push(0x4700cc); f_0041c1a0(cpu); if cpu.returned_to != 0x4700cc { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x470089 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffffa1u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa1u32.wrapping_add(cpu.esi), v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffffa3u32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa3u32.wrapping_add(cpu.esi), v); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x470020; continue; }
  pc = 0x470095; continue;
 }
 0x470095 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffffa1u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa1u32.wrapping_add(cpu.esi), v); }
  { let a = cpu.esp; let b = cpu.eax; { let v: u32 = b; cpu.esp = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffffa3u32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa3u32.wrapping_add(cpu.esi), v); }
  if cpu.sf { cpu.poll(); pc = 0x47002c; continue; }
  pc = 0x4700a1; continue;
 }
 0x4700a1 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffffa1u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa1u32.wrapping_add(cpu.esi), v); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffffa3u32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa3u32.wrapping_add(cpu.esi), v); }
  if cpu.pf() { cpu.poll(); pc = 0x470038; continue; }
  pc = 0x4700ad; continue;
 }
 0x4700a4 => {
  { let v: u32 = cpu.r32(0x4d6798u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b7au32, v); }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b7cu32, v); }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x588b7eu32, v); }
  cpu.push(0x4700cc); f_0041c1a0(cpu); if cpu.returned_to != 0x4700cc { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4700ad => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffffa1u32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa1u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffffa3u32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa3u32.wrapping_add(cpu.esi), v); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x470044; continue; }
  pc = 0x4700b9; continue;
 }
 0x4700b9 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffffa1u32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffffa1u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x66004d67u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x588b7eu32, v); }
  cpu.push(0x4700cc); f_0041c1a0(cpu); if cpu.returned_to != 0x4700cc { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4700ce => {
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4700e1; continue; }
  pc = 0x4700d8; continue;
 }
 0x4700d8 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4700de); f_0040d640(cpu); if cpu.returned_to != 0x4700de { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x4700e1; continue;
 }
 0x4700e1 => {
  { cpu.sub8(cpu.r8(0x56d2d7u32), 0x0u8); }
  if !cpu.zf { pc = 0x47012b; continue; }
  pc = 0x4700ea; continue;
 }
 0x4700ea => {
  { cpu.pushad(); }
  { let v = cpu.r32(0x4d681cu32); cpu.push(v); }
  { let v = cpu.r32(0x4d6820u32); cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6814u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d681cu32, v); }
  { let v: u16 = 0x8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0xbeu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x54a7d0u32; cpu.esi = v; }
  { let v: u8 = 0x5au8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x47011e); f_00447fe3(cpu); if cpu.returned_to != 0x47011e { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6820u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d681cu32, v); }
  { cpu.popad(); }
  pc = 0x47012b; continue;
 }
 0x47012b => {
  cpu.push(0x470130); f_0041c1a0(cpu); if cpu.returned_to != 0x470130 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470132(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470132); }
 let mut pc: u32 = 0x470132;
 loop { match pc {
 0x470132 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5cea73u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5cea30u32, v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5cea4bu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0x9u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47015d); f_0040d5d0(cpu); if cpu.returned_to != 0x47015d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x4701d9; continue; }
  pc = 0x470164; continue;
 }
 0x470164 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6828u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5cea30u32, v); }
  { let v: u32 = 0x5ceb65u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x470184); f_0041bfed(cpu); if cpu.returned_to != 0x470184 { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x4701d9; continue; }
  pc = 0x47018c; continue;
 }
 0x47018c => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x47019f); f_0041bfed(cpu); if cpu.returned_to != 0x47019f { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x4701d9; continue; }
  pc = 0x4701a7; continue;
 }
 0x4701a7 => {
  { let v: u32 = 0x5ceb65u32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  cpu.push(0x4701bd); f_0041bfed(cpu); if cpu.returned_to != 0x4701bd { return; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x581b46u32)); }
  if !cpu.zf { pc = 0x4701d9; continue; }
  pc = 0x4701c5; continue;
 }
 0x4701c5 => {
  { let v = cpu.r32(0x5cea30u32); cpu.push(v); }
  cpu.push(0x4701d0); f_0040d640(cpu); if cpu.returned_to != 0x4701d0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4701d9; continue; }
  pc = 0x4701d7; continue;
 }
 0x4701d7 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4701d9 => {
  { let v: u32 = cpu.r32(0x5cea30u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4701ec; continue; }
  pc = 0x4701e3; continue;
 }
 0x4701e3 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4701e9); f_0040d640(cpu); if cpu.returned_to != 0x4701e9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4701ec; continue;
 }
 0x4701ec => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004701e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4701e8); }
 let mut pc: u32 = 0x4701e8;
 loop { match pc {
 0x4701e8 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xc3f904c4u32.wrapping_add(cpu.ebx))); cpu.w32(0xc3f904c4u32.wrapping_add(cpu.ebx), v); }
  f_004701ee(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004701ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4701ee); }
 let mut pc: u32 = 0x4701ee;
 loop { match pc {
 0x4701ee => {
  { let v: u8 = 0x0u8; cpu.w8(0x5cea73u32, v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5cea4bu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = 0x9u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5cea34u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x470218); f_0040d400(cpu); if cpu.returned_to != 0x470218 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x4702af; continue; }
  pc = 0x470224; continue;
 }
 0x470224 => {
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47022c); f_0040d370(cpu); if cpu.returned_to != 0x47022c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6824u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x4d6824u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x4702af; continue; }
  pc = 0x47023e; continue;
 }
 0x47023e => {
  { let v: u32 = cpu.r32(0x4d6824u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5cea34u32, v); }
  { let v: u32 = 0x5ceb67u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x47025e); f_0041c901(cpu); if cpu.returned_to != 0x47025e { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4702af; continue; }
  pc = 0x470266; continue;
 }
 0x470266 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x470279); f_0041c901(cpu); if cpu.returned_to != 0x470279 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4702af; continue; }
  pc = 0x470281; continue;
 }
 0x470281 => {
  { let v: u32 = 0x5ceb68u32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x470297); f_0041c901(cpu); if cpu.returned_to != 0x470297 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x4702af; continue; }
  pc = 0x47029f; continue;
 }
 0x47029f => {
  { let v = cpu.r32(0x5cea34u32); cpu.push(v); }
  cpu.push(0x4702aa); f_0040d640(cpu); if cpu.returned_to != 0x4702aa { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4702af => {
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x4702c2; continue; }
  pc = 0x4702b9; continue;
 }
 0x4702b9 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4702bf); f_0040d640(cpu); if cpu.returned_to != 0x4702bf { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x4702c2; continue;
 }
 0x4702c2 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004702c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4702c4); }
 let mut pc: u32 = 0x4702c4;
 loop { match pc {
 0x4702c4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5cea73u32, v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5ceae0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = 0x9u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4702e4); f_0040d400(cpu); if cpu.returned_to != 0x4702e4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x470387; continue; }
  pc = 0x4702f0; continue;
 }
 0x4702f0 => {
  { cpu.pushad(); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5cea34u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x470302); f_0040d370(cpu); if cpu.returned_to != 0x470302 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6824u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x4d6824u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x470387; continue; }
  pc = 0x470314; continue;
 }
 0x470314 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6824u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5cea34u32, v); }
  { let v: u32 = 0x5ceb67u32; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x470336); f_0041c901(cpu); if cpu.returned_to != 0x470336 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x470387; continue; }
  pc = 0x47033e; continue;
 }
 0x47033e => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x470351); f_0041c901(cpu); if cpu.returned_to != 0x470351 { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x470387; continue; }
  pc = 0x470359; continue;
 }
 0x470359 => {
  { let v: u32 = 0x5ceb68u32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  cpu.push(0x47036f); f_0041c901(cpu); if cpu.returned_to != 0x47036f { return; }
  { cpu.sub32(cpu.r32(0x4d6a94u32), cpu.ecx); }
  if !cpu.zf { pc = 0x470387; continue; }
  pc = 0x470377; continue;
 }
 0x470377 => {
  { let v = cpu.r32(0x5cea34u32); cpu.push(v); }
  cpu.push(0x470382); f_0040d640(cpu); if cpu.returned_to != 0x470382 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x470387 => {
  { let v: u32 = cpu.r32(0x5cea34u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x47039a; continue; }
  pc = 0x470391; continue;
 }
 0x470391 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x470397); f_0040d640(cpu); if cpu.returned_to != 0x470397 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x47039a; continue;
 }
 0x47039a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470458(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470458); }
 let mut pc: u32 = 0x470458;
 loop { match pc {
 0x470458 => {
  { let v: u32 = 0x4e1ff4u32; cpu.esi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x470462; continue;
 }
 0x470462 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x15u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x47047d; continue; }
  pc = 0x470471; continue;
 }
 0x470471 => {
  { cpu.sub32(cpu.r32(0x15u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x47047d; continue; }
  pc = 0x47047a; continue;
 }
 0x47047a => {
  { let v: u32 = cpu.r32(0x15u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x15u32.wrapping_add(cpu.esi), v); }
  pc = 0x47047d; continue;
 }
 0x47047d => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x1au32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470462; continue; }
  pc = 0x470484; continue;
 }
 0x470484 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470485(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470485); }
 let mut pc: u32 = 0x470485;
 loop { match pc {
 0x470485 => {
  { let v: u32 = 0x4e1ff4u32; cpu.esi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x47048f; continue;
 }
 0x47048f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x15u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x4704aa; continue; }
  pc = 0x47049e; continue;
 }
 0x47049e => {
  { cpu.sub32(cpu.r32(0x15u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x4704aa; continue; }
  pc = 0x4704a7; continue;
 }
 0x4704a7 => {
  { let v: u32 = cpu.r32(0x15u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x15u32.wrapping_add(cpu.esi), v); }
  pc = 0x4704aa; continue;
 }
 0x4704aa => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x1au32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47048f; continue; }
  pc = 0x4704b1; continue;
 }
 0x4704b1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004704b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4704b2); }
 let mut pc: u32 = 0x4704b2;
 loop { match pc {
 0x4704b2 => {
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x4704bc; continue;
 }
 0x4704bc => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x4704d7; continue; }
  pc = 0x4704cb; continue;
 }
 0x4704cb => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x4704d7; continue; }
  pc = 0x4704d4; continue;
 }
 0x4704d4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x4704d7; continue;
 }
 0x4704d7 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x4704f3; continue; }
  pc = 0x4704e7; continue;
 }
 0x4704e7 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x4704f3; continue; }
  pc = 0x4704f0; continue;
 }
 0x4704f0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x4704f3; continue;
 }
 0x4704f3 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4704bc; continue; }
  pc = 0x4704fa; continue;
 }
 0x4704fa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004704fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4704fb); }
 let mut pc: u32 = 0x4704fb;
 loop { match pc {
 0x4704fb => {
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x470505; continue;
 }
 0x470505 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470520; continue; }
  pc = 0x470514; continue;
 }
 0x470514 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470520; continue; }
  pc = 0x47051d; continue;
 }
 0x47051d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x470520; continue;
 }
 0x470520 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x47053c; continue; }
  pc = 0x470530; continue;
 }
 0x470530 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x47053c; continue; }
  pc = 0x470539; continue;
 }
 0x470539 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x47053c; continue;
 }
 0x47053c => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470505; continue; }
  pc = 0x470543; continue;
 }
 0x470543 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470544(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470544); }
 let mut pc: u32 = 0x470544;
 loop { match pc {
 0x470544 => {
  { let v: u32 = 0x52bbf0u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x47054e; continue;
 }
 0x47054e => {
  { let v: u32 = cpu.r32(0x4d6a38u32); cpu.eax = v; }
  { let v: u32 = 0x52ff70u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470563; continue; }
  pc = 0x470560; continue;
 }
 0x470560 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x470563; continue;
 }
 0x470563 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.sf { pc = 0x470575; continue; }
  pc = 0x47056b; continue;
 }
 0x47056b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) | 0x80000000u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x470575; continue;
 }
 0x470575 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47054e; continue; }
  pc = 0x47057b; continue;
 }
 0x47057b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047057c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47057c); }
 let mut pc: u32 = 0x47057c;
 loop { match pc {
 0x47057c => {
  { let v: u32 = 0x52bbf0u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x470586; continue;
 }
 0x470586 => {
  { let v: u32 = cpu.r32(0x4d6a38u32); cpu.eax = v; }
  { let v: u32 = 0x52ff70u32; cpu.ebx = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x80000000u32); }
  if cpu.zf { pc = 0x4705a5; continue; }
  pc = 0x470598; continue;
 }
 0x470598 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) & 0x7fffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x4705b0; continue;
 }
 0x4705a5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x4705b0; continue; }
  pc = 0x4705ad; continue;
 }
 0x4705ad => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x4705b0; continue;
 }
 0x4705b0 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470586; continue; }
  pc = 0x4705b6; continue;
 }
 0x4705b6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004705b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4705b7); }
 let mut pc: u32 = 0x4705b7;
 loop { match pc {
 0x4705b7 => {
  { let v: u32 = 0x5293a0u32; cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  pc = 0x4705c1; continue;
 }
 0x4705c1 => {
  { let v: u32 = cpu.r32(0x12u32.wrapping_add(cpu.esi)).wrapping_sub(0x47b541u32); cpu.w32(0x12u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4705c1; continue; }
  pc = 0x4705ce; continue;
 }
 0x4705ce => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004705cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4705cf); }
 let mut pc: u32 = 0x4705cf;
 loop { match pc {
 0x4705cf => {
  { let v: u32 = 0x5293a0u32; cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  pc = 0x4705d9; continue;
 }
 0x4705d9 => {
  { let v: u32 = cpu.r32(0x12u32.wrapping_add(cpu.esi)).wrapping_add(0x47b541u32); cpu.w32(0x12u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4705d9; continue; }
  pc = 0x4705e6; continue;
 }
 0x4705e6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004705e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4705e7); }
 let mut pc: u32 = 0x4705e7;
 loop { match pc {
 0x4705e7 => {
  { let v: u32 = 0x527780u32; cpu.esi = v; }
  { let v: u32 = 0x96u32; cpu.ecx = v; }
  pc = 0x4705f1; continue;
 }
 0x4705f1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)).wrapping_sub(0x485e18u32); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470613; continue; }
  pc = 0x470607; continue;
 }
 0x470607 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470613; continue; }
  pc = 0x470610; continue;
 }
 0x470610 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x470613; continue;
 }
 0x470613 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x30u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4705f1; continue; }
  pc = 0x47061a; continue;
 }
 0x47061a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047061b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47061b); }
 let mut pc: u32 = 0x47061b;
 loop { match pc {
 0x47061b => {
  { let v: u32 = 0x527780u32; cpu.esi = v; }
  { let v: u32 = 0x96u32; cpu.ecx = v; }
  pc = 0x470625; continue;
 }
 0x470625 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)).wrapping_add(0x485e18u32); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470647; continue; }
  pc = 0x47063b; continue;
 }
 0x47063b => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470647; continue; }
  pc = 0x470644; continue;
 }
 0x470644 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x470647; continue;
 }
 0x470647 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x30u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470625; continue; }
  pc = 0x47064e; continue;
 }
 0x47064e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047064f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47064f); }
 let mut pc: u32 = 0x47064f;
 loop { match pc {
 0x47064f => {
  { let v: u32 = 0x52ae94u32; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x470659; continue;
 }
 0x470659 => {
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.esi)).wrapping_sub(0x47b541u32); cpu.w32(0xau32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xeu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x47067b; continue; }
  pc = 0x47066f; continue;
 }
 0x47066f => {
  { cpu.sub32(cpu.r32(0xeu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x47067b; continue; }
  pc = 0x470678; continue;
 }
 0x470678 => {
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0xeu32.wrapping_add(cpu.esi), v); }
  pc = 0x47067b; continue;
 }
 0x47067b => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x3cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470659; continue; }
  pc = 0x470682; continue;
 }
 0x470682 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470683(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470683); }
 let mut pc: u32 = 0x470683;
 loop { match pc {
 0x470683 => {
  { let v: u32 = 0x52ae94u32; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x47068d; continue;
 }
 0x47068d => {
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.esi)).wrapping_add(0x47b541u32); cpu.w32(0xau32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xeu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x4706af; continue; }
  pc = 0x4706a3; continue;
 }
 0x4706a3 => {
  { cpu.sub32(cpu.r32(0xeu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x4706af; continue; }
  pc = 0x4706ac; continue;
 }
 0x4706ac => {
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0xeu32.wrapping_add(cpu.esi), v); }
  pc = 0x4706af; continue;
 }
 0x4706af => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x3cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47068d; continue; }
  pc = 0x4706b6; continue;
 }
 0x4706b6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004706b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4706b7); }
 let mut pc: u32 = 0x4706b7;
 loop { match pc {
 0x4706b7 => {
  { let v: u32 = 0x4b94dcu32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x4706c1; continue;
 }
 0x4706c1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x4706dc; continue; }
  pc = 0x4706d0; continue;
 }
 0x4706d0 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x4706dc; continue; }
  pc = 0x4706d9; continue;
 }
 0x4706d9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x4706dc; continue;
 }
 0x4706dc => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4706c1; continue; }
  pc = 0x4706e3; continue;
 }
 0x4706e3 => {
  { let v: u32 = 0x4ad05cu32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x4706ed; continue;
 }
 0x4706ed => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470708; continue; }
  pc = 0x4706fc; continue;
 }
 0x4706fc => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470708; continue; }
  pc = 0x470705; continue;
 }
 0x470705 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x470708; continue;
 }
 0x470708 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4706ed; continue; }
  pc = 0x47070f; continue;
 }
 0x47070f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470710); }
 let mut pc: u32 = 0x470710;
 loop { match pc {
 0x470710 => {
  { let v: u32 = 0x4b94dcu32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x47071a; continue;
 }
 0x47071a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470735; continue; }
  pc = 0x470729; continue;
 }
 0x470729 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470735; continue; }
  pc = 0x470732; continue;
 }
 0x470732 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x470735; continue;
 }
 0x470735 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47071a; continue; }
  pc = 0x47073c; continue;
 }
 0x47073c => {
  { let v: u32 = 0x4ad05cu32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x470746; continue;
 }
 0x470746 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470761; continue; }
  pc = 0x470755; continue;
 }
 0x470755 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470761; continue; }
  pc = 0x47075e; continue;
 }
 0x47075e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x470761; continue;
 }
 0x470761 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470746; continue; }
  pc = 0x470768; continue;
 }
 0x470768 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470769(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470769); }
 let mut pc: u32 = 0x470769;
 loop { match pc {
 0x470769 => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x470773; continue;
 }
 0x470773 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x47078b; continue; }
  pc = 0x470781; continue;
 }
 0x470781 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x47078b; continue; }
  pc = 0x470789; continue;
 }
 0x470789 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x47078b; continue;
 }
 0x47078b => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x4707a7; continue; }
  pc = 0x47079b; continue;
 }
 0x47079b => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x4707a7; continue; }
  pc = 0x4707a4; continue;
 }
 0x4707a4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x4707a7; continue;
 }
 0x4707a7 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x4707c3; continue; }
  pc = 0x4707b7; continue;
 }
 0x4707b7 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x4707c3; continue; }
  pc = 0x4707c0; continue;
 }
 0x4707c0 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x4707c3; continue;
 }
 0x4707c3 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470773; continue; }
  pc = 0x4707ca; continue;
 }
 0x4707ca => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004707cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4707cb); }
 let mut pc: u32 = 0x4707cb;
 loop { match pc {
 0x4707cb => {
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x4707d5; continue;
 }
 0x4707d5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x4707ed; continue; }
  pc = 0x4707e3; continue;
 }
 0x4707e3 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x4707ed; continue; }
  pc = 0x4707eb; continue;
 }
 0x4707eb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x4707ed; continue;
 }
 0x4707ed => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470809; continue; }
  pc = 0x4707fd; continue;
 }
 0x4707fd => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470809; continue; }
  pc = 0x470806; continue;
 }
 0x470806 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x470809; continue;
 }
 0x470809 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470825; continue; }
  pc = 0x470819; continue;
 }
 0x470819 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470825; continue; }
  pc = 0x470822; continue;
 }
 0x470822 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x470825; continue;
 }
 0x470825 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4707d5; continue; }
  pc = 0x47082c; continue;
 }
 0x47082c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047082d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47082d); }
 let mut pc: u32 = 0x47082d;
 loop { match pc {
 0x47082d => {
  { let v: u32 = 0x522514u32; cpu.esi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x470837; continue;
 }
 0x470837 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x16u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470852; continue; }
  pc = 0x470846; continue;
 }
 0x470846 => {
  { cpu.sub32(cpu.r32(0x16u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470852; continue; }
  pc = 0x47084f; continue;
 }
 0x47084f => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x16u32.wrapping_add(cpu.esi), v); }
  pc = 0x470852; continue;
 }
 0x470852 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1eu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x47086e; continue; }
  pc = 0x470862; continue;
 }
 0x470862 => {
  { cpu.sub32(cpu.r32(0x1eu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x47086e; continue; }
  pc = 0x47086b; continue;
 }
 0x47086b => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x1eu32.wrapping_add(cpu.esi), v); }
  pc = 0x47086e; continue;
 }
 0x47086e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2eu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x47088a; continue; }
  pc = 0x47087e; continue;
 }
 0x47087e => {
  { cpu.sub32(cpu.r32(0x2eu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x47088a; continue; }
  pc = 0x470887; continue;
 }
 0x470887 => {
  { let v: u32 = cpu.r32(0x2eu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x2eu32.wrapping_add(cpu.esi), v); }
  pc = 0x47088a; continue;
 }
 0x47088a => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x3cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x4708a6; continue; }
  pc = 0x47089a; continue;
 }
 0x47089a => {
  { cpu.sub32(cpu.r32(0x3cu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x4708a6; continue; }
  pc = 0x4708a3; continue;
 }
 0x4708a3 => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x3cu32.wrapping_add(cpu.esi), v); }
  pc = 0x4708a6; continue;
 }
 0x4708a6 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x50u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470837; continue; }
  pc = 0x4708ad; continue;
 }
 0x4708ad => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004708ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4708ae); }
 let mut pc: u32 = 0x4708ae;
 loop { match pc {
 0x4708ae => {
  { let v: u32 = 0x522514u32; cpu.esi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x4708b8; continue;
 }
 0x4708b8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x16u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x4708d3; continue; }
  pc = 0x4708c7; continue;
 }
 0x4708c7 => {
  { cpu.sub32(cpu.r32(0x16u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x4708d3; continue; }
  pc = 0x4708d0; continue;
 }
 0x4708d0 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x16u32.wrapping_add(cpu.esi), v); }
  pc = 0x4708d3; continue;
 }
 0x4708d3 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1eu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x4708ef; continue; }
  pc = 0x4708e3; continue;
 }
 0x4708e3 => {
  { cpu.sub32(cpu.r32(0x1eu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x4708ef; continue; }
  pc = 0x4708ec; continue;
 }
 0x4708ec => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x1eu32.wrapping_add(cpu.esi), v); }
  pc = 0x4708ef; continue;
 }
 0x4708ef => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2eu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x47090b; continue; }
  pc = 0x4708ff; continue;
 }
 0x4708ff => {
  { cpu.sub32(cpu.r32(0x2eu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x47090b; continue; }
  pc = 0x470908; continue;
 }
 0x470908 => {
  { let v: u32 = cpu.r32(0x2eu32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x2eu32.wrapping_add(cpu.esi), v); }
  pc = 0x47090b; continue;
 }
 0x47090b => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x3cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470927; continue; }
  pc = 0x47091b; continue;
 }
 0x47091b => {
  { cpu.sub32(cpu.r32(0x3cu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470927; continue; }
  pc = 0x470924; continue;
 }
 0x470924 => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x3cu32.wrapping_add(cpu.esi), v); }
  pc = 0x470927; continue;
 }
 0x470927 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x50u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4708b8; continue; }
  pc = 0x47092e; continue;
 }
 0x47092e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047092f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47092f); }
 let mut pc: u32 = 0x47092f;
 loop { match pc {
 0x47092f => {
  { let v: u32 = cpu.r32(0x4d6b44u32); cpu.esi = v; }
  { let v: u32 = 0x1388u32; cpu.ecx = v; }
  pc = 0x47093a; continue;
 }
 0x47093a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470952; continue; }
  pc = 0x470948; continue;
 }
 0x470948 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470952; continue; }
  pc = 0x470950; continue;
 }
 0x470950 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x470952; continue;
 }
 0x470952 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47093a; continue; }
  pc = 0x470959; continue;
 }
 0x470959 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047095a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47095a); }
 let mut pc: u32 = 0x47095a;
 loop { match pc {
 0x47095a => {
  { let v: u32 = cpu.r32(0x4d6b44u32); cpu.esi = v; }
  { let v: u32 = 0x1388u32; cpu.ecx = v; }
  pc = 0x470965; continue;
 }
 0x470965 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x47097d; continue; }
  pc = 0x470973; continue;
 }
 0x470973 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x47097d; continue; }
  pc = 0x47097b; continue;
 }
 0x47097b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x47097d; continue;
 }
 0x47097d => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470965; continue; }
  pc = 0x470984; continue;
 }
 0x470984 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470985(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470985); }
 let mut pc: u32 = 0x470985;
 loop { match pc {
 0x470985 => {
  { let v: u32 = 0x513fc8u32; cpu.esi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x47098f; continue;
 }
 0x47098f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x4709aa; continue; }
  pc = 0x47099e; continue;
 }
 0x47099e => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x4709aa; continue; }
  pc = 0x4709a7; continue;
 }
 0x4709a7 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x4709aa; continue;
 }
 0x4709aa => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x2cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47098f; continue; }
  pc = 0x4709b1; continue;
 }
 0x4709b1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004709b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4709b2); }
 let mut pc: u32 = 0x4709b2;
 loop { match pc {
 0x4709b2 => {
  { let v: u32 = 0x513fc8u32; cpu.esi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x4709bc; continue;
 }
 0x4709bc => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x4709d7; continue; }
  pc = 0x4709cb; continue;
 }
 0x4709cb => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x4709d7; continue; }
  pc = 0x4709d4; continue;
 }
 0x4709d4 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x4709d7; continue;
 }
 0x4709d7 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x2cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4709bc; continue; }
  pc = 0x4709de; continue;
 }
 0x4709de => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004709df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4709df); }
 let mut pc: u32 = 0x4709df;
 loop { match pc {
 0x4709df => {
  { let v: u32 = 0x509290u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x4709e9; continue;
 }
 0x4709e9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470a04; continue; }
  pc = 0x4709f8; continue;
 }
 0x4709f8 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470a04; continue; }
  pc = 0x470a01; continue;
 }
 0x470a01 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x470a04; continue;
 }
 0x470a04 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470a20; continue; }
  pc = 0x470a14; continue;
 }
 0x470a14 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470a20; continue; }
  pc = 0x470a1d; continue;
 }
 0x470a1d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x470a20; continue;
 }
 0x470a20 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470a3c; continue; }
  pc = 0x470a30; continue;
 }
 0x470a30 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470a3c; continue; }
  pc = 0x470a39; continue;
 }
 0x470a39 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x470a3c; continue;
 }
 0x470a3c => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470a58; continue; }
  pc = 0x470a4c; continue;
 }
 0x470a4c => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470a58; continue; }
  pc = 0x470a55; continue;
 }
 0x470a55 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x470a58; continue;
 }
 0x470a58 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470a74; continue; }
  pc = 0x470a68; continue;
 }
 0x470a68 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470a74; continue; }
  pc = 0x470a71; continue;
 }
 0x470a71 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x470a74; continue;
 }
 0x470a74 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470a90; continue; }
  pc = 0x470a84; continue;
 }
 0x470a84 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470a90; continue; }
  pc = 0x470a8d; continue;
 }
 0x470a8d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x470a90; continue;
 }
 0x470a90 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470aac; continue; }
  pc = 0x470aa0; continue;
 }
 0x470aa0 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470aac; continue; }
  pc = 0x470aa9; continue;
 }
 0x470aa9 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  pc = 0x470aac; continue;
 }
 0x470aac => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xa6u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470ad1; continue; }
  pc = 0x470abf; continue;
 }
 0x470abf => {
  { cpu.sub32(cpu.r32(0xa6u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470ad1; continue; }
  pc = 0x470acb; continue;
 }
 0x470acb => {
  { let v: u32 = cpu.r32(0xa6u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0xa6u32.wrapping_add(cpu.esi), v); }
  pc = 0x470ad1; continue;
 }
 0x470ad1 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470aed; continue; }
  pc = 0x470ae1; continue;
 }
 0x470ae1 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470aed; continue; }
  pc = 0x470aea; continue;
 }
 0x470aea => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  pc = 0x470aed; continue;
 }
 0x470aed => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0x2084u32); }
  if cpu.zf { pc = 0x470b13; continue; }
  pc = 0x470af7; continue;
 }
 0x470af7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470b12; continue; }
  pc = 0x470b06; continue;
 }
 0x470b06 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470b12; continue; }
  pc = 0x470b0f; continue;
 }
 0x470b0f => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  pc = 0x470b12; continue;
 }
 0x470b12 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x470b13; continue;
 }
 0x470b13 => {
  { let v: u32 = cpu.add32(cpu.esi, 0xb8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4709e9; continue; }
  pc = 0x470b20; continue;
 }
 0x470b20 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470b21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470b21); }
 let mut pc: u32 = 0x470b21;
 loop { match pc {
 0x470b21 => {
  { let v: u32 = 0x509290u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x470b2b; continue;
 }
 0x470b2b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470b46; continue; }
  pc = 0x470b3a; continue;
 }
 0x470b3a => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470b46; continue; }
  pc = 0x470b43; continue;
 }
 0x470b43 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x470b46; continue;
 }
 0x470b46 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470b62; continue; }
  pc = 0x470b56; continue;
 }
 0x470b56 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470b62; continue; }
  pc = 0x470b5f; continue;
 }
 0x470b5f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x470b62; continue;
 }
 0x470b62 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470b7e; continue; }
  pc = 0x470b72; continue;
 }
 0x470b72 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470b7e; continue; }
  pc = 0x470b7b; continue;
 }
 0x470b7b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x470b7e; continue;
 }
 0x470b7e => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470b9a; continue; }
  pc = 0x470b8e; continue;
 }
 0x470b8e => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470b9a; continue; }
  pc = 0x470b97; continue;
 }
 0x470b97 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x470b9a; continue;
 }
 0x470b9a => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470bb6; continue; }
  pc = 0x470baa; continue;
 }
 0x470baa => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470bb6; continue; }
  pc = 0x470bb3; continue;
 }
 0x470bb3 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x470bb6; continue;
 }
 0x470bb6 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470bd2; continue; }
  pc = 0x470bc6; continue;
 }
 0x470bc6 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470bd2; continue; }
  pc = 0x470bcf; continue;
 }
 0x470bcf => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x470bd2; continue;
 }
 0x470bd2 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470bee; continue; }
  pc = 0x470be2; continue;
 }
 0x470be2 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470bee; continue; }
  pc = 0x470beb; continue;
 }
 0x470beb => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  pc = 0x470bee; continue;
 }
 0x470bee => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xa6u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470c13; continue; }
  pc = 0x470c01; continue;
 }
 0x470c01 => {
  { cpu.sub32(cpu.r32(0xa6u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470c13; continue; }
  pc = 0x470c0d; continue;
 }
 0x470c0d => {
  { let v: u32 = cpu.r32(0xa6u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0xa6u32.wrapping_add(cpu.esi), v); }
  pc = 0x470c13; continue;
 }
 0x470c13 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470c2f; continue; }
  pc = 0x470c23; continue;
 }
 0x470c23 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470c2f; continue; }
  pc = 0x470c2c; continue;
 }
 0x470c2c => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  pc = 0x470c2f; continue;
 }
 0x470c2f => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0x2084u32); }
  if cpu.zf { pc = 0x470c55; continue; }
  pc = 0x470c39; continue;
 }
 0x470c39 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470c54; continue; }
  pc = 0x470c48; continue;
 }
 0x470c48 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470c54; continue; }
  pc = 0x470c51; continue;
 }
 0x470c51 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  pc = 0x470c54; continue;
 }
 0x470c54 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x470c55; continue;
 }
 0x470c55 => {
  { let v: u32 = cpu.add32(cpu.esi, 0xb8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470b2b; continue; }
  pc = 0x470c62; continue;
 }
 0x470c62 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470c63(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470c63); }
 let mut pc: u32 = 0x470c63;
 loop { match pc {
 0x470c63 => {
  { let v: u32 = 0x505310u32; cpu.esi = v; }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  pc = 0x470c6d; continue;
 }
 0x470c6d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470c85; continue; }
  pc = 0x470c7b; continue;
 }
 0x470c7b => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470c85; continue; }
  pc = 0x470c83; continue;
 }
 0x470c83 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x470c85; continue;
 }
 0x470c85 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470c6d; continue; }
  pc = 0x470c8c; continue;
 }
 0x470c8c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470c8d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470c8d); }
 let mut pc: u32 = 0x470c8d;
 loop { match pc {
 0x470c8d => {
  { let v: u32 = 0x505310u32; cpu.esi = v; }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  pc = 0x470c97; continue;
 }
 0x470c97 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470caf; continue; }
  pc = 0x470ca5; continue;
 }
 0x470ca5 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470caf; continue; }
  pc = 0x470cad; continue;
 }
 0x470cad => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x470caf; continue;
 }
 0x470caf => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470c97; continue; }
  pc = 0x470cb6; continue;
 }
 0x470cb6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470cb7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470cb7); }
 let mut pc: u32 = 0x470cb7;
 loop { match pc {
 0x470cb7 => {
  { let v: u32 = 0x4efb50u32; cpu.esi = v; }
  { let v: u32 = 0x7d0u32; cpu.ecx = v; }
  pc = 0x470cc1; continue;
 }
 0x470cc1 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)).wrapping_sub(0x4826aau32); cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470ce3; continue; }
  pc = 0x470cd7; continue;
 }
 0x470cd7 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470ce3; continue; }
  pc = 0x470ce0; continue;
 }
 0x470ce0 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  pc = 0x470ce3; continue;
 }
 0x470ce3 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470cff; continue; }
  pc = 0x470cf3; continue;
 }
 0x470cf3 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470cff; continue; }
  pc = 0x470cfc; continue;
 }
 0x470cfc => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  pc = 0x470cff; continue;
 }
 0x470cff => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x2cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470cc1; continue; }
  pc = 0x470d06; continue;
 }
 0x470d06 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470d07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470d07); }
 let mut pc: u32 = 0x470d07;
 loop { match pc {
 0x470d07 => {
  { let v: u32 = 0x4efb50u32; cpu.esi = v; }
  { let v: u32 = 0x7d0u32; cpu.ecx = v; }
  pc = 0x470d11; continue;
 }
 0x470d11 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)).wrapping_add(0x4826aau32); cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470d33; continue; }
  pc = 0x470d27; continue;
 }
 0x470d27 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470d33; continue; }
  pc = 0x470d30; continue;
 }
 0x470d30 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x24u32.wrapping_add(cpu.esi), v); }
  pc = 0x470d33; continue;
 }
 0x470d33 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470d4f; continue; }
  pc = 0x470d43; continue;
 }
 0x470d43 => {
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470d4f; continue; }
  pc = 0x470d4c; continue;
 }
 0x470d4c => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x28u32.wrapping_add(cpu.esi), v); }
  pc = 0x470d4f; continue;
 }
 0x470d4f => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x2cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470d11; continue; }
  pc = 0x470d56; continue;
 }
 0x470d56 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470d57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470d57); }
 let mut pc: u32 = 0x470d57;
 loop { match pc {
 0x470d57 => {
  { let v: u32 = 0x4ba552u32; cpu.esi = v; }
  { let v: u32 = 0x12c0u32; cpu.ecx = v; }
  pc = 0x470d61; continue;
 }
 0x470d61 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470d7c; continue; }
  pc = 0x470d70; continue;
 }
 0x470d70 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470d7c; continue; }
  pc = 0x470d79; continue;
 }
 0x470d79 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x470d7c; continue;
 }
 0x470d7c => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470d61; continue; }
  pc = 0x470d83; continue;
 }
 0x470d83 => {
  { let v: u32 = 0x4c3b52u32; cpu.esi = v; }
  { let v: u32 = 0x12c0u32; cpu.ecx = v; }
  pc = 0x470d8d; continue;
 }
 0x470d8d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470da8; continue; }
  pc = 0x470d9c; continue;
 }
 0x470d9c => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470da8; continue; }
  pc = 0x470da5; continue;
 }
 0x470da5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_sub(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x470da8; continue;
 }
 0x470da8 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470d8d; continue; }
  pc = 0x470daf; continue;
 }
 0x470daf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470db0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470db0); }
 let mut pc: u32 = 0x470db0;
 loop { match pc {
 0x470db0 => {
  { let v: u32 = 0x4ba552u32; cpu.esi = v; }
  { let v: u32 = 0x12c0u32; cpu.ecx = v; }
  pc = 0x470dba; continue;
 }
 0x470dba => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470dd5; continue; }
  pc = 0x470dc9; continue;
 }
 0x470dc9 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470dd5; continue; }
  pc = 0x470dd2; continue;
 }
 0x470dd2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x470dd5; continue;
 }
 0x470dd5 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470dba; continue; }
  pc = 0x470ddc; continue;
 }
 0x470ddc => {
  { let v: u32 = 0x4c3b52u32; cpu.esi = v; }
  { let v: u32 = 0x12c0u32; cpu.ecx = v; }
  pc = 0x470de6; continue;
 }
 0x470de6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x470e01; continue; }
  pc = 0x470df5; continue;
 }
 0x470df5 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x470e01; continue; }
  pc = 0x470dfe; continue;
 }
 0x470dfe => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)).wrapping_add(cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x470e01; continue;
 }
 0x470e01 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x470de6; continue; }
  pc = 0x470e08; continue;
 }
 0x470e08 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470e09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470e09); }
 let mut pc: u32 = 0x470e09;
 loop { match pc {
 0x470e09 => {
  cpu.push(0x470e0e); f_004108e0(cpu); if cpu.returned_to != 0x470e0e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470e10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470e10); }
 let mut pc: u32 = 0x470e10;
 loop { match pc {
 0x470e10 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.w32(0x5ceb6cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5ceb70u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5ceb74u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5ceb78u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x57584eu32, v); }
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x470e38; continue;
 }
 0x470e38 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x470e8e; continue; }
  pc = 0x470e43; continue;
 }
 0x470e43 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x470e8e; continue; }
  pc = 0x470e5f; continue;
 }
 0x470e5f => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ebx); cpu.edx = v; }
  cpu.push(0x470e6b); f_004714b9(cpu); if cpu.returned_to != 0x470e6b { return; }
  if !cpu.cf { pc = 0x470e8e; continue; }
  pc = 0x470e6d; continue;
 }
 0x470e6d => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4b839cu32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x57584eu32), 0xffffffffu32); }
  if cpu.zf { pc = 0x470e9a; continue; }
  pc = 0x470e86; continue;
 }
 0x470e86 => {
  { cpu.sub32(cpu.r32(0x57584eu32), cpu.eax); }
  if cpu.zf { pc = 0x470eac; continue; }
  pc = 0x470e8e; continue;
 }
 0x470e8e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x470e38; continue; }
  pc = 0x470e97; continue;
 }
 0x470e97 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x470e9a => {
  { let v: u32 = cpu.eax; cpu.w32(0x57584eu32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x57584eu32); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x470eac => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470eb3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470eb3); }
 let mut pc: u32 = 0x470eb3;
 loop { match pc {
 0x470eb3 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  cpu.push(0x470ec6); f_00470f58(cpu); if cpu.returned_to != 0x470ec6 { return; }
  if cpu.cf { pc = 0x470ecb; continue; }
  pc = 0x470ec8; continue;
 }
 0x470ec8 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x470ecb => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x559754u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b64u32, v); }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x5756ceu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5756d0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x470ef7); f_00470f05(cpu); if cpu.returned_to != 0x470ef7 { return; }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4d6b64u32); cpu.edi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470f05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470f05); }
 let mut pc: u32 = 0x470f05;
 loop { match pc {
 0x470f05 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = 0x5293a0u32; cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  pc = 0x470f28; continue;
 }
 0x470f28 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x470f51; continue; }
  pc = 0x470f2f; continue;
 }
 0x470f2f => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x470f51; continue; }
  pc = 0x470f35; continue;
 }
 0x470f35 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), (cpu.ebx as u16)); }
  if !cpu.zf { pc = 0x470f51; continue; }
  pc = 0x470f3b; continue;
 }
 0x470f3b => {
  { let v: u16 = 0xffffu16; cpu.w16(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)) & 0x7eu8; cpu.w8(0x35u32.wrapping_add(cpu.esi), v); }
  { cpu.sub16(cpu.r16(0x27u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x470f51; continue; }
  pc = 0x470f4c; continue;
 }
 0x470f4c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x470f51; continue;
 }
 0x470f51 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x40u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x470f28; continue; }
  pc = 0x470f56; continue;
 }
 0x470f56 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470f58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470f58); }
 let mut pc: u32 = 0x470f58;
 loop { match pc {
 0x470f58 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x52aca0u32; cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x470f66; continue;
 }
 0x470f66 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x470f9e; continue; }
  pc = 0x470f6f; continue;
 }
 0x470f6f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x40u32); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x470f9e; continue; }
  pc = 0x470f9a; continue;
 }
 0x470f9a => {
  pc = 0x470fa9; continue;
 }
 0x470f9e => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xau32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x470f66; continue; }
  pc = 0x470fa3; continue;
 }
 0x470fa3 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x470fa9 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5756ceu32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5756d0u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00470fc5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x470fc5); }
 let mut pc: u32 = 0x470fc5;
 loop { match pc {
 0x470fc5 => {
  { cpu.and32(cpu.r32(0x58c87au32), 0x1u32); }
  if cpu.zf { pc = 0x470ff5; continue; }
  pc = 0x470fd1; continue;
 }
 0x470fd1 => {
  { cpu.pushad(); }
  cpu.push(0x470fd7); f_00403030(cpu); if cpu.returned_to != 0x470fd7 { return; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.popad(); }
  if !cpu.zf { pc = 0x470ff5; continue; }
  pc = 0x470fdc; continue;
 }
 0x470fdc => {
  cpu.push(0x470fe1); f_0047111f(cpu); if cpu.returned_to != 0x470fe1 { return; }
  if !cpu.cf { pc = 0x47111d; continue; }
  pc = 0x470fe7; continue;
 }
 0x470fe7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x4710a6; continue;
 }
 0x470ff5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x47100d; continue;
 }
 0x47100d => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x471105; continue; }
  pc = 0x471018; continue;
 }
 0x471018 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x471105; continue; }
  pc = 0x471025; continue;
 }
 0x471025 => {
  { cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x471105; continue; }
  pc = 0x471033; continue;
 }
 0x471033 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x4d6934u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x471103; continue; }
  pc = 0x471055; continue;
 }
 0x471055 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x4d6938u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edi as u16), cpu.r16(0x4d6936u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x471103; continue; }
  pc = 0x47106e; continue;
 }
 0x47106e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x4b8b48u32; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6938u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6934u32)); }
  if (cpu.sf != cpu.of) { pc = 0x471103; continue; }
  pc = 0x47109d; continue;
 }
 0x47109d => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6936u32)); }
  if (cpu.sf != cpu.of) { pc = 0x471103; continue; }
  pc = 0x4710a6; continue;
 }
 0x4710a6 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d6938u32, v); }
  { let v: u32 = 0x4au32; cpu.eax = v; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x4b8b48u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 3u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v: u16 = cpu.r16(0x4d6938u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x471103 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x471105; continue;
 }
 0x471105 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47100d; continue; }
  pc = 0x47110f; continue;
 }
 0x47110f => {
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x47111d; continue;
 }
 0x47111d => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047111f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47111f); }
 let mut pc: u32 = 0x47111f;
 loop { match pc {
 0x47111f => {
  { cpu.pushad(); }
  cpu.push(0x471125); f_004712e3(cpu); if cpu.returned_to != 0x471125 { return; }
  if !cpu.cf { pc = 0x471137; continue; }
  pc = 0x471127; continue;
 }
 0x471127 => {
  { let v: u32 = cpu.r32(0x4d6bf4u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x471137; continue; }
  pc = 0x471134; continue;
 }
 0x471134 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x471137 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047113a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47113a); }
 let mut pc: u32 = 0x47113a;
 loop { match pc {
 0x47113a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x471152; continue;
 }
 0x471152 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x4711f7; continue; }
  pc = 0x47115d; continue;
 }
 0x47115d => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x4711f7; continue; }
  pc = 0x47116a; continue;
 }
 0x47116a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x4d6934u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4711f5; continue; }
  pc = 0x47117a; continue;
 }
 0x47117a => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x47118a; continue; }
  pc = 0x471186; continue;
 }
 0x471186 => {
  { let v: u16 = (cpu.edi as u16).wrapping_sub(0x10u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x47118a; continue;
 }
 0x47118a => {
  { cpu.sub16((cpu.edi as u16), cpu.r16(0x4d6936u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4711f5; continue; }
  pc = 0x471193; continue;
 }
 0x471193 => {
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4b8b48u32); cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x4711bc; continue; }
  pc = 0x4711b9; continue;
 }
 0x4711b9 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x4711bc; continue;
 }
 0x4711bc => {
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6934u32)); }
  if (cpu.sf != cpu.of) { pc = 0x4711f5; continue; }
  pc = 0x4711cf; continue;
 }
 0x4711cf => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6936u32)); }
  if (cpu.sf != cpu.of) { pc = 0x4711f5; continue; }
  pc = 0x4711d8; continue;
 }
 0x4711d8 => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4711f5 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4711f7; continue;
 }
 0x4711f7 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x471152; continue; }
  pc = 0x471201; continue;
 }
 0x471201 => {
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00471211(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x471211); }
 let mut pc: u32 = 0x471211;
 loop { match pc {
 0x471211 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x471229; continue;
 }
 0x471229 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x4712c9; continue; }
  pc = 0x471234; continue;
 }
 0x471234 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x4712c9; continue; }
  pc = 0x471241; continue;
 }
 0x471241 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x56d30eu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x4d6934u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4712c7; continue; }
  pc = 0x47125c; continue;
 }
 0x47125c => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edi as u16), cpu.r16(0x4d6936u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4712c7; continue; }
  pc = 0x47126a; continue;
 }
 0x47126a => {
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4b8b48u32); cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x56d30eu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x471292; continue; }
  pc = 0x471292; continue;
 }
 0x471292 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6934u32)); }
  if (cpu.sf != cpu.of) { pc = 0x4712c7; continue; }
  pc = 0x4712a1; continue;
 }
 0x4712a1 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6936u32)); }
  if (cpu.sf != cpu.of) { pc = 0x4712c7; continue; }
  pc = 0x4712aa; continue;
 }
 0x4712aa => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4712c7 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4712c9; continue;
 }
 0x4712c9 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x471229; continue; }
  pc = 0x4712d3; continue;
 }
 0x4712d3 => {
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004712e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4712e3); }
 let mut pc: u32 = 0x4712e3;
 loop { match pc {
 0x4712e3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x4712fb; continue;
 }
 0x4712fb => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x4713b4; continue; }
  pc = 0x471306; continue;
 }
 0x471306 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x4713b4; continue; }
  pc = 0x471313; continue;
 }
 0x471313 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x4d6934u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4713b2; continue; }
  pc = 0x471327; continue;
 }
 0x471327 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x471337; continue; }
  pc = 0x471333; continue;
 }
 0x471333 => {
  { let v: u16 = (cpu.edi as u16).wrapping_sub(0x10u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x471337; continue;
 }
 0x471337 => {
  { cpu.sub16((cpu.edi as u16), cpu.r16(0x4d6936u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4713b2; continue; }
  pc = 0x471340; continue;
 }
 0x471340 => {
  { cpu.sub8(cpu.r8(0x5b3d24u32), 0x0u8); }
  if !cpu.zf { pc = 0x471350; continue; }
  pc = 0x471349; continue;
 }
 0x471349 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x4713b2; continue; }
  pc = 0x471350; continue;
 }
 0x471350 => {
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4b8b48u32); cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x471379; continue; }
  pc = 0x471376; continue;
 }
 0x471376 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x471379; continue;
 }
 0x471379 => {
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6934u32)); }
  if (cpu.sf != cpu.of) { pc = 0x4713b2; continue; }
  pc = 0x47138c; continue;
 }
 0x47138c => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6936u32)); }
  if (cpu.sf != cpu.of) { pc = 0x4713b2; continue; }
  pc = 0x471395; continue;
 }
 0x471395 => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4713b2 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x4713b4; continue;
 }
 0x4713b4 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4712fb; continue; }
  pc = 0x4713be; continue;
 }
 0x4713be => {
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004713ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4713ce); }
 let mut pc: u32 = 0x4713ce;
 loop { match pc {
 0x4713ce => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x522514u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x4713e6; continue;
 }
 0x4713e6 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x47148b; continue; }
  pc = 0x4713f1; continue;
 }
 0x4713f1 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x47148b; continue; }
  pc = 0x4713fe; continue;
 }
 0x4713fe => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x4d6934u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x471489; continue; }
  pc = 0x47140e; continue;
 }
 0x47140e => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x47141e; continue; }
  pc = 0x47141a; continue;
 }
 0x47141a => {
  { let v: u16 = (cpu.edi as u16).wrapping_sub(0x10u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x47141e; continue;
 }
 0x47141e => {
  { cpu.sub16((cpu.edi as u16), cpu.r16(0x4d6936u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x471489; continue; }
  pc = 0x471427; continue;
 }
 0x471427 => {
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4b8b48u32); cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x471450; continue; }
  pc = 0x47144d; continue;
 }
 0x47144d => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x471450; continue;
 }
 0x471450 => {
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d6934u32)); }
  if (cpu.sf != cpu.of) { pc = 0x471489; continue; }
  pc = 0x471463; continue;
 }
 0x471463 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6936u32)); }
  if (cpu.sf != cpu.of) { pc = 0x471489; continue; }
  pc = 0x47146c; continue;
 }
 0x47146c => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6bf4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x471489 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x47148b; continue;
 }
 0x47148b => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4713e6; continue; }
  pc = 0x471495; continue;
 }
 0x471495 => {
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004714a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4714a5); }
 let mut pc: u32 = 0x4714a5;
 loop { match pc {
 0x4714a5 => {
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x4714b7; continue; }
  pc = 0x4714a9; continue;
 }
 0x4714a9 => {
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4714b7; continue; }
  pc = 0x4714ad; continue;
 }
 0x4714ad => {
  { cpu.sub32(cpu.edi, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x4714b7; continue; }
  pc = 0x4714b1; continue;
 }
 0x4714b1 => {
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4714b7; continue; }
  pc = 0x4714b5; continue;
 }
 0x4714b5 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4714b7 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004714b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4714b9); }
 let mut pc: u32 = 0x4714b9;
 loop { match pc {
 0x4714b9 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x5ceb6cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5ceb70u32); cpu.edi = v; }
  cpu.push(0x4714cb); f_004714a5(cpu); if cpu.returned_to != 0x4714cb { return; }
  if cpu.cf { pc = 0x471536; continue; }
  pc = 0x4714cd; continue;
 }
 0x4714cd => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x5ceb74u32)); cpu.esi = v; }
  cpu.push(0x4714d8); f_004714a5(cpu); if cpu.returned_to != 0x4714d8 { return; }
  if cpu.cf { pc = 0x471536; continue; }
  pc = 0x4714da; continue;
 }
 0x4714da => {
  { let v: u32 = cpu.r32(0x5ceb6cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5ceb70u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5ceb78u32)); cpu.edi = v; }
  cpu.push(0x4714f1); f_004714a5(cpu); if cpu.returned_to != 0x4714f1 { return; }
  if cpu.cf { pc = 0x471536; continue; }
  pc = 0x4714f3; continue;
 }
 0x4714f3 => {
  { let v: u32 = cpu.r32(0x5ceb6cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5ceb70u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x5ceb74u32)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5ceb78u32)); cpu.edi = v; }
  cpu.push(0x471510); f_004714a5(cpu); if cpu.returned_to != 0x471510 { return; }
  if cpu.cf { pc = 0x471536; continue; }
  pc = 0x471512; continue;
 }
 0x471512 => {
  { let v: u32 = cpu.r32(0x5ceb74u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5ceb78u32); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 1u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x5ceb6cu32)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5ceb70u32)); cpu.edi = v; }
  cpu.push(0x471533); f_004714a5(cpu); if cpu.returned_to != 0x471533 { return; }
  if cpu.cf { pc = 0x471536; continue; }
  pc = 0x471535; continue;
 }
 0x471535 => {
  { cpu.cf = false; }
  pc = 0x471536; continue;
 }
 0x471536 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00471538(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x471538); }
 let mut pc: u32 = 0x471538;
 loop { match pc {
 0x471538 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x471579; continue; }
  pc = 0x471542; continue;
 }
 0x471542 => {
  { cpu.sub8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x47156a; continue; }
  pc = 0x471547; continue;
 }
 0x471547 => {
  { cpu.sub8((cpu.ecx as u8), 0x3u8); }
  if cpu.zf { pc = 0x47155b; continue; }
  pc = 0x47154c; continue;
 }
 0x47154c => {
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x471586; continue;
 }
 0x47155b => {
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x471586; continue;
 }
 0x47156a => {
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x471586; continue;
 }
 0x471579 => {
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x471586; continue;
 }
 0x471586 => {
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x471597); f_004714a5(cpu); if cpu.returned_to != 0x471597 { return; }
  if !cpu.cf { pc = 0x4715cf; continue; }
  pc = 0x471599; continue;
 }
 0x471599 => {
  cpu.push(0x47159e); f_004184ca(cpu); if cpu.returned_to != 0x47159e { return; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x575a5bu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4715b7); f_00420989(cpu); if cpu.returned_to != 0x4715b7 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x4715d2; continue; }
  pc = 0x4715c4; continue;
 }
 0x4715c4 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if cpu.zf { pc = 0x4715d2; continue; }
  pc = 0x4715cf; continue;
 }
 0x4715cf => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4715d2 => {
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004715d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4715d5); }
 let mut pc: u32 = 0x4715d5;
 loop { match pc {
 0x4715d5 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x471616; continue; }
  pc = 0x4715df; continue;
 }
 0x4715df => {
  { cpu.sub8((cpu.ecx as u8), 0x2u8); }
  if cpu.zf { pc = 0x471607; continue; }
  pc = 0x4715e4; continue;
 }
 0x4715e4 => {
  { cpu.sub8((cpu.ecx as u8), 0x3u8); }
  if cpu.zf { pc = 0x4715f8; continue; }
  pc = 0x4715e9; continue;
 }
 0x4715e9 => {
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x471623; continue;
 }
 0x4715f8 => {
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x471623; continue;
 }
 0x471607 => {
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x471623; continue;
 }
 0x471616 => {
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x471623; continue;
 }
 0x471623 => {
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x471634); f_004714a5(cpu); if cpu.returned_to != 0x471634 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00471636(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x471636); }
 let mut pc: u32 = 0x471636;
 loop { match pc {
 0x471636 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5ceb6cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5ceb70u32, v); }
  { let v: u32 = 0x20u32; cpu.w32(0x5ceb74u32, v); }
  { let v: u32 = 0x20u32; cpu.w32(0x5ceb78u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  pc = 0x471677; continue;
 }
 0x471677 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x4716c3; continue; }
  pc = 0x471681; continue;
 }
 0x471681 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ebx); cpu.edx = v; }
  cpu.push(0x4716a2); f_004714b9(cpu); if cpu.returned_to != 0x4716a2 { return; }
  if !cpu.cf { pc = 0x4716c3; continue; }
  pc = 0x4716a4; continue;
 }
 0x4716a4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5ceb6cu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5ceb70u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x5ceb6cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5ceb70u32); cpu.edi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4716c3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x28u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x471677; continue; }
  pc = 0x4716cc; continue;
 }
 0x4716cc => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004716d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4716d0); }
 let mut pc: u32 = 0x4716d0;
 loop { match pc {
 0x4716d0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  pc = 0x4716e6; continue;
 }
 0x4716e6 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x471745; continue; }
  pc = 0x4716ed; continue;
 }
 0x4716ed => {
  { cpu.sub8(cpu.r8(0x59da94u32), 0x0u8); }
  if !cpu.zf { pc = 0x4716fc; continue; }
  pc = 0x4716f6; continue;
 }
 0x4716f6 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x50u8); }
  if cpu.zf { pc = 0x471745; continue; }
  pc = 0x4716fc; continue;
 }
 0x4716fc => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x471745; continue; }
  pc = 0x471701; continue;
 }
 0x471701 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x471745; continue; }
  pc = 0x471707; continue;
 }
 0x471707 => {
  { cpu.sub8(cpu.r8(0x59dafdu32), 0x0u8); }
  if !cpu.zf { pc = 0x471720; continue; }
  pc = 0x471710; continue;
 }
 0x471710 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 3u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x471728; continue;
 }
 0x471720 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x471728; continue;
 }
 0x471728 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x471745; continue; }
  pc = 0x471732; continue;
 }
 0x471732 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x471745; continue; }
  pc = 0x47173b; continue;
 }
 0x47173b => {
  { let v: u8 = 0x0u8; cpu.w8(0x59dafdu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x471745 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4716e6; continue; }
  pc = 0x47174a; continue;
 }
 0x47174a => {
  { let v: u8 = 0x0u8; cpu.w8(0x59dafdu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00471754(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x471754); }
 let mut pc: u32 = 0x471754;
 loop { match pc {
 0x471754 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x4717cf; continue; }
  pc = 0x471777; continue;
 }
 0x471777 => {
  { cpu.sub8(cpu.r8(0x59da94u32), 0x0u8); }
  if !cpu.zf { pc = 0x471786; continue; }
  pc = 0x471780; continue;
 }
 0x471780 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x50u8); }
  if cpu.zf { pc = 0x4717cf; continue; }
  pc = 0x471786; continue;
 }
 0x471786 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x4717cf; continue; }
  pc = 0x47178b; continue;
 }
 0x47178b => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x4717cf; continue; }
  pc = 0x471791; continue;
 }
 0x471791 => {
  { cpu.sub8(cpu.r8(0x59dafdu32), 0x0u8); }
  if !cpu.zf { pc = 0x4717aa; continue; }
  pc = 0x47179a; continue;
 }
 0x47179a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 3u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x4717b2; continue;
 }
 0x4717aa => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x4717b2; continue;
 }
 0x4717b2 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4717cf; continue; }
  pc = 0x4717bc; continue;
 }
 0x4717bc => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4717cf; continue; }
  pc = 0x4717c5; continue;
 }
 0x4717c5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x59dafdu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4717cf => {
  { let v: u8 = 0x0u8; cpu.w8(0x59dafdu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004717d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4717d9); }
 let mut pc: u32 = 0x4717d9;
 loop { match pc {
 0x4717d9 => {
  f_004716d0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004717e0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4717e0); } r_004717e0(cpu, 0x4717e0); }
pub fn f_004718e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4718e8); } r_004717e0(cpu, 0x4718e8); }
fn r_004717e0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4717e0 => {
  { cpu.sub8(cpu.r8(0x54acd1u32), 0x2u8); }
  if (cpu.sf != cpu.of) { pc = 0x471a82; continue; }
  pc = 0x4717ed; continue;
 }
 0x4717ed => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if !cpu.zf { pc = 0x471a82; continue; }
  pc = 0x4717fa; continue;
 }
 0x4717fa => {
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.push(0x471808); f_00417e7e(cpu); if cpu.returned_to != 0x471808 { return; }
  { cpu.popad(); }
  cpu.push(0x47180e); f_00471c01(cpu); if cpu.returned_to != 0x47180e { return; }
  if cpu.cf { pc = 0x471a82; continue; }
  pc = 0x471814; continue;
 }
 0x471814 => {
  { cpu.sub8(cpu.r8(0x56c6e1u32), 0x0u8); }
  if !cpu.zf { pc = 0x471861; continue; }
  pc = 0x47181d; continue;
 }
 0x47181d => {
  { cpu.sub32(cpu.r32(0x5791c4u32), 0x0u32); }
  if cpu.zf { pc = 0x47183f; continue; }
  pc = 0x471826; continue;
 }
 0x471826 => {
  { let v: u32 = 0x30u32; cpu.w32(0x56c95fu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5791c4u32, v); }
  pc = 0x471a7b; continue;
 }
 0x47183f => {
  { let v: u32 = 0x0u32; cpu.w32(0x5791c4u32, v); }
  { cpu.sub32(cpu.r32(0x56c95fu32), 0x0u32); }
  if cpu.zf { pc = 0x4718e2; continue; }
  pc = 0x471856; continue;
 }
 0x471856 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x56c95fu32)); cpu.w32(0x56c95fu32, v); }
  pc = 0x4718e2; continue;
 }
 0x471861 => {
  { cpu.sub32(cpu.r32(0x5791c4u32), 0x1u32); }
  if !cpu.zf { pc = 0x47188f; continue; }
  pc = 0x47186a; continue;
 }
 0x47186a => {
  { let v: u32 = cpu.r32(0x581b5au32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x581b5eu32), cpu.eax); }
  if !cpu.zf { pc = 0x471a51; continue; }
  pc = 0x47187b; continue;
 }
 0x47187b => {
  { cpu.sub32(cpu.r32(0x581b62u32), 0x0u32); }
  if cpu.zf { pc = 0x47188f; continue; }
  pc = 0x471884; continue;
 }
 0x471884 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x581b62u32)); cpu.w32(0x581b62u32, v); }
  pc = 0x471a51; continue;
 }
 0x47188f => {
  { let v: u32 = cpu.r32(0x581b5au32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x581b5eu32, v); }
  { let v: u32 = 0x96u32; cpu.w32(0x581b62u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56c9b8u32, v); }
  cpu.push(0x4718af); f_004301f7(cpu); if cpu.returned_to != 0x4718af { return; }
  { let v: u32 = cpu.r32(0x581b5au32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x581b5eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9b8u32, v); }
  if cpu.cf { pc = 0x4718e2; continue; }
  pc = 0x4718c2; continue;
 }
 0x4718c2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x5791c4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x579182u32, v); }
  { let v: u32 = 0x28u32; cpu.w32(0x56c95fu32, v); }
  pc = 0x471a51; continue;
 }
 0x4718e2 => {
  { cpu.sub32(cpu.r32(0x5791c4u32), 0x3u32); }
  if cpu.zf { pc = 0x471a51; continue; }
  pc = 0x4718ef; continue;
 }
 0x4718e8 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.edx)), ((cpu.eax >> 8) as u8)); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4718ef; continue;
 }
 0x4718ef => {
  { let v: u8 = 0x1u8; cpu.w8(0x56c9b8u32, v); }
  cpu.push(0x4718fb); f_0042f95b(cpu); if cpu.returned_to != 0x4718fb { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9b8u32, v); }
  if cpu.cf { pc = 0x47191d; continue; }
  pc = 0x471904; continue;
 }
 0x471904 => {
  { let v: u32 = 0x3u32; cpu.w32(0x5791c4u32, v); }
  { let v: u32 = 0x28u32; cpu.w32(0x56c95fu32, v); }
  pc = 0x471a51; continue;
 }
 0x47191d => {
  { cpu.sub32(cpu.r32(0x5791c4u32), 0x2u32); }
  if cpu.zf { pc = 0x471a51; continue; }
  pc = 0x47192a; continue;
 }
 0x47192a => {
  { let v: u8 = 0x1u8; cpu.w8(0x56c9b8u32, v); }
  cpu.push(0x471936); f_0042f7dc(cpu); if cpu.returned_to != 0x471936 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9b8u32, v); }
  if cpu.cf { pc = 0x471958; continue; }
  pc = 0x47193f; continue;
 }
 0x47193f => {
  { let v: u32 = 0x2u32; cpu.w32(0x5791c4u32, v); }
  { let v: u32 = 0x28u32; cpu.w32(0x56c95fu32, v); }
  pc = 0x471a51; continue;
 }
 0x471958 => {
  { cpu.sub16(cpu.r16(0x56c9e8u32), 0x0u16); }
  if cpu.zf { pc = 0x4719df; continue; }
  pc = 0x471962; continue;
 }
 0x471962 => {
  { cpu.sub32(cpu.r32(0x5791c4u32), 0x4u32); }
  if cpu.zf { pc = 0x4719bb; continue; }
  pc = 0x47196b; continue;
 }
 0x47196b => {
  { let v: u32 = cpu.r32(0x579259u32); cpu.ebp = v; }
  { cpu.pushad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x58c465u32, v); }
  cpu.push(0x47197e); f_0044cbe1(cpu); if cpu.returned_to != 0x47197e { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  cpu.push(0x471985); f_00419d49(cpu); if cpu.returned_to != 0x471985 { return; }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x4d675au32).wrapping_sub(0x40u16); cpu.w16(0x4d675au32, v); }
  { let v: u16 = cpu.r16(0x4d675cu32).wrapping_sub(0x20u16); cpu.w16(0x4d675cu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x56c9e8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x14u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b2dc4u32, v); }
  { let v: u32 = 0x4u32; cpu.w32(0x5791c4u32, v); }
  { let v: u32 = 0x28u32; cpu.w32(0x56c95fu32, v); }
  pc = 0x4719bb; continue;
 }
 0x4719bb => {
  { cpu.sub16(cpu.r16(0x56c9e8u32), 0x1u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x471a51; continue; }
  pc = 0x4719c9; continue;
 }
 0x4719c9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x5b3588u32, v); }
  cpu.push(0x4719d5); f_00471ec2(cpu); if cpu.returned_to != 0x4719d5 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x5791c4u32, v); }
  pc = 0x4719df; continue;
 }
 0x4719df => {
  { cpu.sub32(cpu.r32(0x5b358cu32), 0x80u32); }
  if (cpu.sf == cpu.of) { pc = 0x471a4c; continue; }
  pc = 0x4719eb; continue;
 }
 0x4719eb => {
  pc = 0x471a4c; continue;
 }
 0x471a4c => {
  cpu.push(0x471a51); f_00471ab8(cpu); if cpu.returned_to != 0x471a51 { return; }
  pc = 0x471a51; continue;
 }
 0x471a51 => {
  { cpu.sub32(cpu.r32(0x56c95fu32), 0x0u32); }
  if cpu.zf { pc = 0x471a6a; continue; }
  pc = 0x471a5a; continue;
 }
 0x471a5a => {
  { let v: u32 = cpu.dec32(cpu.r32(0x56c95fu32)); cpu.w32(0x56c95fu32, v); }
  { cpu.sub32(cpu.r32(0x56c95fu32), 0x0u32); }
  if !cpu.zf { pc = 0x471a6a; continue; }
  pc = 0x471a69; continue;
 }
 0x471a69 => {
  {  }
  pc = 0x471a6a; continue;
 }
 0x471a6a => {
  { cpu.sub16(cpu.r16(0x56c9e8u32), 0x0u16); }
  if cpu.zf { pc = 0x471a7b; continue; }
  pc = 0x471a74; continue;
 }
 0x471a74 => {
  { let v: u16 = cpu.dec16(cpu.r16(0x56c9e8u32)); cpu.w16(0x56c9e8u32, v); }
  pc = 0x471a7b; continue;
 }
 0x471a7b => {
  cpu.push(0x471a80); f_00471c6f(cpu); if cpu.returned_to != 0x471a80 { return; }
  if cpu.cf { pc = 0x471a88; continue; }
  pc = 0x471a82; continue;
 }
 0x471a82 => {
  cpu.push(0x471a87); f_00471a95(cpu); if cpu.returned_to != 0x471a87 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x471a88 => {
  { let v: u8 = 0x3u8; cpu.w8(0x54acd1u32, v); }
  cpu.push(0x471a94); f_00471a95(cpu); if cpu.returned_to != 0x471a94 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00471a95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x471a95); }
 let mut pc: u32 = 0x471a95;
 loop { match pc {
 0x471a95 => {
  { let v: u32 = 0x578d7eu32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x57917eu32, v); }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x471aa5; continue;
 }
 0x471aa5 => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x471aa5; continue; }
  pc = 0x471ab7; continue;
 }
 0x471ab7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00471ab8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x471ab8); }
 let mut pc: u32 = 0x471ab8;
 loop { match pc {
 0x471ab8 => {
  { cpu.sub32(cpu.r32(0x56c95fu32), 0x0u32); }
  if cpu.zf { pc = 0x471ac2; continue; }
  pc = 0x471ac1; continue;
 }
 0x471ac1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x471ac2 => {
  { cpu.sub8(cpu.r8(0x56c9b9u32), 0x3u8); }
  if !cpu.zf { pc = 0x471ad9; continue; }
  pc = 0x471acb; continue;
 }
 0x471acb => {
  { let v: u8 = 0x0u8; cpu.w8(0x56c9b9u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9bau32, v); }
  pc = 0x471ad9; continue;
 }
 0x471ad9 => {
  { cpu.sub8(cpu.r8(0x56c9b9u32), 0x0u8); }
  if !cpu.zf { pc = 0x471b36; continue; }
  pc = 0x471ae2; continue;
 }
 0x471ae2 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x56c9bau32)); cpu.w8(0x56c9bau32, v); }
  { cpu.sub8(cpu.r8(0x56c9bau32), 0x4u8); }
  if !cpu.zf { pc = 0x471b01; continue; }
  pc = 0x471af1; continue;
 }
 0x471af1 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56c9b9u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9bau32, v); }
  pc = 0x471b36; continue;
 }
 0x471b01 => {
  { let v: u8 = 0x1u8; cpu.w8(0x56c9b8u32, v); }
  cpu.push(0x471b0d); f_0042fad8(cpu); if cpu.returned_to != 0x471b0d { return; }
  { let v: u32 = 0x6u32; cpu.w32(0x5791c4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x579182u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9b8u32, v); }
  if cpu.cf { pc = 0x471b36; continue; }
  pc = 0x471b27; continue;
 }
 0x471b27 => {
  { let v: u32 = 0x28u32; cpu.w32(0x56c95fu32, v); }
  pc = 0x471c00; continue;
 }
 0x471b36 => {
  { cpu.sub8(cpu.r8(0x56c9b9u32), 0x1u8); }
  if !cpu.zf { pc = 0x471b90; continue; }
  pc = 0x471b3f; continue;
 }
 0x471b3f => {
  { let v: u8 = cpu.inc8(cpu.r8(0x56c9bau32)); cpu.w8(0x56c9bau32, v); }
  { cpu.sub8(cpu.r8(0x56c9bau32), 0x4u8); }
  if !cpu.zf { pc = 0x471b5e; continue; }
  pc = 0x471b4e; continue;
 }
 0x471b4e => {
  { let v: u8 = 0x0u8; cpu.w8(0x56c9b9u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9bau32, v); }
  pc = 0x471b90; continue;
 }
 0x471b5e => {
  { let v: u8 = 0x1u8; cpu.w8(0x56c9b8u32, v); }
  cpu.push(0x471b6a); f_0042fc4a(cpu); if cpu.returned_to != 0x471b6a { return; }
  { let v: u32 = 0x7u32; cpu.w32(0x5791c4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x579182u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x56c9b8u32, v); }
  if cpu.cf { pc = 0x471b90; continue; }
  pc = 0x471b84; continue;
 }
 0x471b84 => {
  { let v: u32 = 0x28u32; cpu.w32(0x56c95fu32, v); }
  pc = 0x471c00; continue;
 }
 0x471b90 => {
  { cpu.sub8(cpu.r8(0x56c9b9u32), 0x2u8); }
  pc = 0x471c00; continue;
 }
 0x471c00 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00471c01(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x471c01); } r_00471c01(cpu, 0x471c01); }
pub fn f_00471c19(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x471c19); } r_00471c01(cpu, 0x471c19); }
fn r_00471c01(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x471c01 => {
  { let v: u32 = cpu.r32(0x56c9dbu32); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x4b94d0u32), cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x471c11; continue; }
  pc = 0x471c0f; continue;
 }
 0x471c0f => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x471c11 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  cpu.push(0x471c20); f_0041c808(cpu); if cpu.returned_to != 0x471c20 { return; }
  { cpu.sub16((cpu.eax as u16), 0x32u16); }
  if (cpu.sf == cpu.of) { pc = 0x471c45; continue; }
  pc = 0x471c26; continue;
 }
 0x471c19 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x471c20); f_0041c808(cpu); if cpu.returned_to != 0x471c20 { return; }
  { cpu.sub16((cpu.eax as u16), 0x32u16); }
  if (cpu.sf == cpu.of) { pc = 0x471c45; continue; }
  pc = 0x471c26; continue;
 }
 0x471c26 => {
  { let v: u8 = 0x0u8; cpu.w8(0x588b81u32, v); }
  { cpu.sub8(cpu.r8(0x588b80u32), 0x2u8); }
  if cpu.zf { pc = 0x471c45; continue; }
  pc = 0x471c36; continue;
 }
 0x471c36 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x588b80u32)); cpu.w8(0x588b80u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x471c43); f_00456ca1(cpu); if cpu.returned_to != 0x471c43 { return; }
  pc = 0x471c65; continue;
 }
 0x471c45 => {
  { let v: u8 = 0x0u8; cpu.w8(0x588b80u32, v); }
  { cpu.sub8(cpu.r8(0x588b81u32), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x471c26; continue; }
  pc = 0x471c55; continue;
 }
 0x471c55 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x588b81u32)); cpu.w8(0x588b81u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x471c65); f_00456b9e(cpu); if cpu.returned_to != 0x471c65 { return; }
  pc = 0x471c65; continue;
 }
 0x471c65 => {
  { let v: u16 = 0x1u16; cpu.w16(0x4d69d8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00471c6f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x471c6f); } r_00471c6f(cpu, 0x471c6f); }
pub fn f_00471ce8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x471ce8); } r_00471c6f(cpu, 0x471ce8); }
fn r_00471c6f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x471c6f => {
  { let v: u32 = cpu.r32(0x54accdu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x15u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x471c9e; continue; }
  pc = 0x471c7d; continue;
 }
 0x471c7d => {
  { cpu.sub8(cpu.r8(0x54acd1u32), 0x3u8); }
  if cpu.zf { pc = 0x471e7c; continue; }
  pc = 0x471c8a; continue;
 }
 0x471c8a => {
  { let v: u32 = cpu.r32(0x4b94d0u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0xau32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x56c9dbu32, v); }
  pc = 0x471e7c; continue;
 }
 0x471c9e => {
  { cpu.sub32(cpu.r32(0x4d6a18u32), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x471e7e; continue; }
  pc = 0x471caa; continue;
 }
 0x471caa => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x7fu8); }
  if cpu.zf { pc = 0x471d47; continue; }
  pc = 0x471cb4; continue;
 }
 0x471cb4 => {
  { cpu.sub8(cpu.r8(0x579182u32), 0x0u8); }
  if !cpu.zf { pc = 0x471cc5; continue; }
  pc = 0x471cbd; continue;
 }
 0x471cbd => {
  { cpu.sub8((cpu.eax as u8), 0x7eu8); }
  if cpu.zf { pc = 0x471e0d; continue; }
  pc = 0x471cc5; continue;
 }
 0x471cc5 => {
  { cpu.sub8((cpu.eax as u8), 0x7du8); }
  if cpu.zf { pc = 0x471d84; continue; }
  pc = 0x471ccd; continue;
 }
 0x471ccd => {
  { cpu.sub8((cpu.eax as u8), 0x7cu8); }
  if cpu.zf { pc = 0x471dca; continue; }
  pc = 0x471cd5; continue;
 }
 0x471cd5 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f4cu32, v); }
  { let v: u32 = cpu.r32(0x9u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6aecu32, v); }
  { cpu.sub8(cpu.r8(0x4e1f4cu32), 0x5u8); }
  if !cpu.zf { pc = 0x471cfc; continue; }
  pc = 0x471cf0; continue;
 }
 0x471ce8 => {
  { cpu.sub32(cpu.eax, 0x4e1f4cu32); }
  { let v: u32 = cpu.add32(cpu.eax, 0x78800c75u32); cpu.eax = v; }
  if !cpu.sf { pc = 0x471cf6; continue; }
  pc = 0x471cf4; continue;
 }
 0x471cf0 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x2u8); }
  pc = 0x471cf4; continue;
 }
 0x471cf4 => {
  if !cpu.zf { pc = 0x471cfc; continue; }
  pc = 0x471cf6; continue;
 }
 0x471cf6 => {
  {  }
  pc = 0x471e6e; continue;
 }
 0x471cfc => {
  { let v: u16 = cpu.r16(0x3u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68aeu32, v); }
  { let v: u16 = cpu.r16(0x5u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b0u32, v); }
  { let v: u16 = cpu.r16(0x7u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68b2u32, v); }
  { let v: u16 = cpu.r16(0x4d6756u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x471d26; continue; }
  pc = 0x471d26; continue;
 }
 0x471d26 => {
  { let v: u32 = cpu.esi.wrapping_add(0x19u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x54accdu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x590aa2u32, v); }
  cpu.push(0x471d3b); f_00413337(cpu); if cpu.returned_to != 0x471d3b { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x590aa2u32, v); }
  cpu.poll(); pc = 0x471c6f; continue;
 }
 0x471d47 => {
  { let v: u32 = cpu.r32(0xdu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.zf { pc = 0x471d5c; continue; }
  pc = 0x471d4f; continue;
 }
 0x471d4f => {
  { cpu.sub32(cpu.r32(0x11u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x471e6e; continue; }
  pc = 0x471d5c; continue;
 }
 0x471d5c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d675au32, v); }
  { let v: u32 = cpu.r32(0x11u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d675cu32, v); }
  { let v: u16 = cpu.r16(0x4d6756u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); }
  { let v: u32 = 0x28u32; cpu.w32(0x56c95fu32, v); }
  pc = 0x471e6e; continue;
 }
 0x471d84 => {
  { let v: u32 = cpu.r32(0x5b2dc4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x5b3856u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x5b3857u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b2dc4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3856u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3857u32, v); }
  cpu.push(0x471db3); f_0044d9c6(cpu); if cpu.returned_to != 0x471db3 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b3857u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b3856u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b2dc4u32, v); }
  pc = 0x471e6e; continue;
 }
 0x471dca => {
  { let v: u32 = cpu.r32(0x5b2dc4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x5b3856u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x5b3857u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b2dc4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3856u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3857u32, v); }
  cpu.push(0x471df9); f_0044da18(cpu); if cpu.returned_to != 0x471df9 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b3857u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5b3856u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5b2dc4u32, v); }
  pc = 0x471e6e; continue;
 }
 0x471e0d => {
  { let v: u32 = cpu.r32(0x9u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x0u32); }
  if cpu.zf { pc = 0x471e1b; continue; }
  pc = 0x471e15; continue;
 }
 0x471e15 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x522514u32); cpu.ebp = v; }
  pc = 0x471e1b; continue;
 }
 0x471e1b => {
  { let v: u32 = cpu.ebp; cpu.w32(0x5b3588u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3857u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5b3856u32, v); }
  { cpu.sub32(cpu.ebp, 0x0u32); }
  if cpu.zf { pc = 0x471e5b; continue; }
  pc = 0x471e34; continue;
 }
 0x471e34 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x471e5b; continue; }
  pc = 0x471e41; continue;
 }
 0x471e41 => {
  { cpu.sub32(cpu.r32(0x576e88u32), 0x1u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x471e5b; continue; }
  pc = 0x471e4a; continue;
 }
 0x471e4a => {
  { let v: u32 = cpu.ebp; cpu.w32(0x579259u32, v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x56c9e8u32, v); }
  pc = 0x471e6e; continue;
 }
 0x471e5b => {
  { let v: u16 = 0x0u16; cpu.w16(0x56c9e8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b3588u32, v); }
  pc = 0x471e6e; continue;
 }
 0x471e6e => {
  { let v: u32 = cpu.esi.wrapping_add(0x19u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x54accdu32, v); }
  cpu.poll(); pc = 0x471c6f; continue;
 }
 0x471e7c => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x471e7e => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00471e80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x471e80); }
 let mut pc: u32 = 0x471e80;
 loop { match pc {
 0x471e80 => {
  { cpu.sub8(cpu.r8(0x54acd1u32), 0x1u8); }
  if !cpu.zf { pc = 0x471ec1; continue; }
  pc = 0x471e89; continue;
 }
 0x471e89 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x54accdu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x557544u32); }
  if (cpu.sf == cpu.of) { pc = 0x471ebf; continue; }
  pc = 0x471e99; continue;
 }
 0x471e99 => {
  { let v: u32 = cpu.r32(0x4d6a18u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x7eu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(0x522514u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x9u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x19u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x54accdu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x15u32.wrapping_add(cpu.edi), v); }
  pc = 0x471ebf; continue;
 }
 0x471ebf => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x471ec1; continue;
 }
 0x471ec1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00471ec2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x471ec2); }
 let mut pc: u32 = 0x471ec2;
 loop { match pc {
 0x471ec2 => {
  { cpu.sub8(cpu.r8(0x54acd1u32), 0x1u8); }
  if !cpu.zf { pc = 0x471eff; continue; }
  pc = 0x471ecb; continue;
 }
 0x471ecb => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x54accdu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x557544u32); }
  if (cpu.sf == cpu.of) { pc = 0x471efd; continue; }
  pc = 0x471edb; continue;
 }
 0x471edb => {
  { let v: u32 = cpu.r32(0x4d6a18u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x7eu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x19u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x54accdu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x15u32.wrapping_add(cpu.edi), v); }
  pc = 0x471efd; continue;
 }
 0x471efd => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x471eff; continue;
 }
 0x471eff => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00471f00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x471f00); }
 let mut pc: u32 = 0x471f00;
 loop { match pc {
 0x471f00 => {
  { cpu.sub8(cpu.r8(0x54acd1u32), 0x1u8); }
  if !cpu.zf { pc = 0x471f36; continue; }
  pc = 0x471f09; continue;
 }
 0x471f09 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x54accdu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x557544u32); }
  if (cpu.sf == cpu.of) { pc = 0x471f34; continue; }
  pc = 0x471f19; continue;
 }
 0x471f19 => {
  { let v: u32 = cpu.r32(0x4d6a18u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x7du8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x19u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x54accdu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x15u32.wrapping_add(cpu.edi), v); }
  pc = 0x471f34; continue;
 }
 0x471f34 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x471f36; continue;
 }
 0x471f36 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00471f37(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x471f37); }
 let mut pc: u32 = 0x471f37;
 loop { match pc {
 0x471f37 => {
  { cpu.sub8(cpu.r8(0x54acd1u32), 0x1u8); }
  if !cpu.zf { pc = 0x471f6d; continue; }
  pc = 0x471f40; continue;
 }
 0x471f40 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x54accdu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x557544u32); }
  if (cpu.sf == cpu.of) { pc = 0x471f6b; continue; }
  pc = 0x471f50; continue;
 }
 0x471f50 => {
  { let v: u32 = cpu.r32(0x4d6a18u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x7cu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x19u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x54accdu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x15u32.wrapping_add(cpu.edi), v); }
  pc = 0x471f6b; continue;
 }
 0x471f6b => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x471f6d; continue;
 }
 0x471f6d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00471f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x471f70); }
 let mut pc: u32 = 0x471f70;
 loop { match pc {
 0x471f70 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x471f7a; continue; }
  pc = 0x471f79; continue;
 }
 0x471f79 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x471f7a => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if !cpu.zf { pc = 0x471faa; continue; }
  pc = 0x471f84; continue;
 }
 0x471f84 => {
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x526020u32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x471f93; continue;
 }
 0x471f93 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x471fa4; continue; }
  pc = 0x471f9a; continue;
 }
 0x471f9a => {
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x471fa4; continue; }
  pc = 0x471fa0; continue;
 }
 0x471fa0 => {
  { let v: u8 = 0x4u8; cpu.w8(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x471fa4; continue;
 }
 0x471fa4 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x471f93; continue; }
  pc = 0x471faa; continue;
 }
 0x471faa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00471fab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x471fab); }
 let mut pc: u32 = 0x471fab;
 loop { match pc {
 0x471fab => {
  cpu.push(0x471fb0); f_00472070(cpu); if cpu.returned_to != 0x471fb0 { return; }
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x471fba; continue;
 }
 0x471fba => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x472022; continue; }
  pc = 0x471fc1; continue;
 }
 0x471fc1 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = 0x4d954cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4e158cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(cpu.r16(0x0u32.wrapping_add(cpu.esi))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  cpu.push(0x471ff2); f_00472028(cpu); if cpu.returned_to != 0x471ff2 { return; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(cpu.r16(0x2u32.wrapping_add(cpu.esi))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.push(0x472005); f_00472028(cpu); if cpu.returned_to != 0x472005 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  cpu.push(0x47200e); f_00472028(cpu); if cpu.returned_to != 0x47200e { return; }
  { let v: u32 = cpu.shl32(cpu.ecx, 7u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  cpu.push(0x472020); f_00472028(cpu); if cpu.returned_to != 0x472020 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x472022; continue;
 }
 0x472022 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x471fba; continue; }
  pc = 0x472027; continue;
 }
 0x472027 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00472028(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x472028); }
 let mut pc: u32 = 0x472028;
 loop { match pc {
 0x472028 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x47206f; continue; }
  pc = 0x47202c; continue;
 }
 0x47202c => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  pc = 0x47203a; continue;
 }
 0x47203a => {
  { cpu.sub32(cpu.edi, 0x4d954cu32); }
  if cpu.cf { pc = 0x472067; continue; }
  pc = 0x472042; continue;
 }
 0x472042 => {
  { cpu.sub32(cpu.edi, 0x4dd54cu32); }
  if !cpu.cf { pc = 0x472067; continue; }
  pc = 0x47204a; continue;
 }
 0x47204a => {
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x4ea12cu32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x472067; continue; }
  pc = 0x47205c; continue;
 }
 0x47205c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  cpu.push(0x472065); f_00472128(cpu); if cpu.returned_to != 0x472065 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x472067; continue;
 }
 0x472067 => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47203a; continue; }
  pc = 0x47206c; continue;
 }
 0x47206c => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x47206f; continue;
 }
 0x47206f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00472070(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x472070); }
 let mut pc: u32 = 0x472070;
 loop { match pc {
 0x472070 => {
  { let v: u32 = 0x4e240cu32; cpu.edi = v; }
  { let v: u32 = 0x1000u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x47207f; continue;
 }
 0x47207f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x6u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47207f; continue; }
  pc = 0x472087; continue;
 }
 0x472087 => {
  { let v: u32 = 0x4e240cu32; cpu.w32(0x5ceb7cu32, v); }
  cpu.push(0x472096); f_00472097(cpu); if cpu.returned_to != 0x472096 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00472097(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x472097); }
 let mut pc: u32 = 0x472097;
 loop { match pc {
 0x472097 => {
  { let v: u32 = 0x5ceb84u32; cpu.edi = v; }
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004720a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4720a6); }
 let mut pc: u32 = 0x4720a6;
 loop { match pc {
 0x4720a6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if cpu.zf { pc = 0x4720db; continue; }
  pc = 0x4720b5; continue;
 }
 0x4720b5 => {
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0x7u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x5ceb84u32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4720db => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8((cpu.edx as u8), (cpu.edx as u8)); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004720e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4720e1); }
 let mut pc: u32 = 0x4720e1;
 loop { match pc {
 0x4720e1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0x7u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x5ceb84u32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); cpu.w8(0x5ceb84u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047210f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47210f); }
 let mut pc: u32 = 0x47210f;
 loop { match pc {
 0x47210f => {
  { let v: u32 = cpu.r32(0x5ceb7cu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4e840cu32); }
  if !cpu.cf { pc = 0x472126; continue; }
  pc = 0x47211d; continue;
 }
 0x47211d => {
  { let v: u32 = cpu.add32(cpu.r32(0x5ceb7cu32), 0x6u32); cpu.w32(0x5ceb7cu32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x472126 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00472128(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x472128); }
 let mut pc: u32 = 0x472128;
 loop { match pc {
 0x472128 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5cf384u32, v); }
  cpu.push(0x472134); f_0047210f(cpu); if cpu.returned_to != 0x472134 { return; }
  if cpu.cf { pc = 0x472197; continue; }
  pc = 0x47213a; continue;
 }
 0x47213a => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4ea12cu32.wrapping_add(cpu.ebx)), 0x1u8); }
  if cpu.zf { pc = 0x47214e; continue; }
  pc = 0x472145; continue;
 }
 0x472145 => {
  cpu.push(0x47214a); f_0041c743(cpu); if cpu.returned_to != 0x47214a { return; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x47214e; continue;
 }
 0x47214e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5cf384u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4d954cu32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffff80u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x4u8); }
  if (cpu.sf != cpu.of) { pc = 0x47217d; continue; }
  pc = 0x47217b; continue;
 }
 0x47217b => {
  { let v: u8 = 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x47217d; continue;
 }
 0x47217d => {
  cpu.push(0x472182); f_004721a2(cpu); if cpu.returned_to != 0x472182 { return; }
  if !cpu.zf { pc = 0x47219f; continue; }
  pc = 0x472184; continue;
 }
 0x472184 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x472197); f_004720e1(cpu); if cpu.returned_to != 0x472197 { return; }
  pc = 0x472197; continue;
 }
 0x472197 => {
  { let v: u32 = cpu.r32(0x5cf384u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47219f => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.poll(); pc = 0x472197; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004721a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4721a2); }
 let mut pc: u32 = 0x4721a2;
 loop { match pc {
 0x4721a2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x4721ae); f_004720a6(cpu); if cpu.returned_to != 0x4721ae { return; }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004721ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4721ba); }
 let mut pc: u32 = 0x4721ba;
 loop { match pc {
 0x4721ba => {
  { let v: u8 = 0x1u8; cpu.w8(0x593a9au32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x4e240cu32; cpu.edi = v; }
  { let v: u32 = 0x1000u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6a00u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4d6ba4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4d6a04u32); cpu.edx = v; }
  pc = 0x4721e2; continue;
 }
 0x4721e2 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x4721ee; continue; }
  pc = 0x4721e9; continue;
 }
 0x4721e9 => {
  cpu.push(0x4721ee); f_00472201(cpu); if cpu.returned_to != 0x4721ee { return; }
  pc = 0x4721ee; continue;
 }
 0x4721ee => {
  { let v: u32 = cpu.add32(cpu.edi, 0x6u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4721e2; continue; }
  pc = 0x4721f4; continue;
 }
 0x4721f4 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x593a9au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00472201(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x472201); }
 let mut pc: u32 = 0x472201;
 loop { match pc {
 0x472201 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x472246; continue; }
  pc = 0x47220d; continue;
 }
 0x47220d => {
  { cpu.sub32(cpu.eax, 0xfffffff8u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x472246; continue; }
  pc = 0x472212; continue;
 }
 0x472212 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4d6ba8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x472246; continue; }
  pc = 0x472222; continue;
 }
 0x472222 => {
  { cpu.sub32(cpu.ebx, 0xfffffff8u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x472246; continue; }
  pc = 0x472227; continue;
 }
 0x472227 => {
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { pc = 0x472248; continue; }
  pc = 0x47222d; continue;
 }
 0x47222d => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x9dbu32); cpu.ecx = v; }
  let t = cpu.r32(0x4d6f38u32); cpu.push(0x472242); cpu.call(t); if cpu.returned_to != 0x472242 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x472246; continue;
 }
 0x472246 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x472248 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0x8u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x9e0u32.wrapping_add(cpu.ecx); cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x472269); cpu.call(t); if cpu.returned_to != 0x472269 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x5u32.wrapping_add(cpu.edi))); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.edi)), 0xfu8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  if !cpu.zf { cpu.poll(); pc = 0x472246; continue; }
  pc = 0x472276; continue;
 }
 0x472276 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x5u32.wrapping_add(cpu.edi))); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047227b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47227b); }
 let mut pc: u32 = 0x47227b;
 loop { match pc {
 0x47227b => {
  { let v: u32 = 0x526020u32; cpu.edi = v; }
  { let v: u32 = 0x180u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x5242ccu32; cpu.esi = v; }
  { let v: u32 = 0x526020u32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x472298; continue;
 }
 0x472298 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x472315; continue; }
  pc = 0x4722a3; continue;
 }
 0x4722a3 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x3u8); }
  if !cpu.zf { pc = 0x472315; continue; }
  pc = 0x4722ad; continue;
 }
 0x4722ad => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x2u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4722cd; continue; }
  pc = 0x4722b7; continue;
 }
 0x4722b7 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x4u8); }
  if !cpu.zf { pc = 0x472315; continue; }
  pc = 0x4722bd; continue;
 }
 0x4722bd => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4722cd; continue; }
  pc = 0x4722c7; continue;
 }
 0x4722c7 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esi)), 0x8u8); }
  if !cpu.zf { pc = 0x472315; continue; }
  pc = 0x4722cd; continue;
 }
 0x4722cd => {
  { let v: u16 = cpu.r8(0x12u32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x13u32.wrapping_add(cpu.esi)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { cpu.pushad(); }
  cpu.push(0x4722f0); f_00420989(cpu); if cpu.returned_to != 0x4722f0 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x23u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4e158cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x472312); f_004723d7(cpu); if cpu.returned_to != 0x472312 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x6u32); cpu.edi = v; }
  pc = 0x472315; continue;
 }
 0x472315 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x1eu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x472298; continue; }
  pc = 0x47231f; continue;
 }
 0x47231f => {
  { let v: u8 = 0x1u8; cpu.w8(0x5ceb81u32, v); }
  cpu.push(0x47232b); f_00466edd(cpu); if cpu.returned_to != 0x47232b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047232c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47232c); }
 let mut pc: u32 = 0x47232c;
 loop { match pc {
 0x47232c => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0x526020u32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ebp = v; }
  pc = 0x47233b; continue;
 }
 0x47233b => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x47234d; continue; }
  pc = 0x472342; continue;
 }
 0x472342 => {
  { let v: u32 = cpu.edi.wrapping_add(0x6u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47233b; continue; }
  pc = 0x472348; continue;
 }
 0x472348 => {
  pc = 0x4723ce; continue;
 }
 0x47234d => {
  { let v: u8 = 0x0u8; cpu.w8(0x5ceb81u32, v); }
  { let v: u16 = cpu.r8(0x12u32.wrapping_add(cpu.esi)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x13u32.wrapping_add(cpu.esi)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x4723ce; continue; }
  pc = 0x472368; continue;
 }
 0x472368 => {
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.esi)) & 0xf0u8; cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x472397; continue; }
  pc = 0x472381; continue;
 }
 0x472381 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x472387); f_0041c6a6(cpu); if cpu.returned_to != 0x472387 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x472393; continue; }
  pc = 0x47238d; continue;
 }
 0x47238d => {
  { let v: u8 = 0xbu8; cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x472397; continue;
 }
 0x472393 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  pc = 0x472397; continue;
 }
 0x472397 => {
  { cpu.pushad(); }
  cpu.push(0x47239d); f_00420989(cpu); if cpu.returned_to != 0x47239d { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x23u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.popad(); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x4723bb); f_004723d7(cpu); if cpu.returned_to != 0x4723bb { return; }
  cpu.push(0x4723c0); f_0046535a(cpu); if cpu.returned_to != 0x4723c0 { return; }
  cpu.push(0x4723c5); f_00465532(cpu); if cpu.returned_to != 0x4723c5 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x5ceb81u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4723ce => {
  { let v: u8 = 0x1u8; cpu.w8(0x5ceb81u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004723d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4723d7); }
 let mut pc: u32 = 0x4723d7;
 loop { match pc {
 0x4723d7 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x5ceb81u32), 0x0u8); }
  if !cpu.zf { pc = 0x47241e; continue; }
  pc = 0x4723e1; continue;
 }
 0x4723e1 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4723e7); f_00448783(cpu); if cpu.returned_to != 0x4723e7 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.cf { pc = 0x47241e; continue; }
  pc = 0x4723ea; continue;
 }
 0x4723ea => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4723f8); f_0047a2d9(cpu); if cpu.returned_to != 0x4723f8 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let a = cpu.edi; let b = cpu.esi; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.esi = v; } }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x1cu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x47241a); f_00448500(cpu); if cpu.returned_to != 0x47241a { return; }
  { let v: u8 = cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edi)), 0x7fu8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  pc = 0x47241e; continue;
 }
 0x47241e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00472420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x472420); }
 let mut pc: u32 = 0x472420;
 loop { match pc {
 0x472420 => {
  { let v: u32 = 0x559754u32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = 0x4b8658u32; cpu.eax = v; }
  pc = 0x47242f; continue;
 }
 0x47242f => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.sf { pc = 0x47244b; continue; }
  pc = 0x472435; continue;
 }
 0x472435 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), cpu.eax); }
  if !cpu.zf { pc = 0x47244b; continue; }
  pc = 0x47243a; continue;
 }
 0x47243a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.ecx >> 8) as u8), (cpu.ecx & 0xff)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = !((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x13u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x47244b; continue;
 }
 0x47244b => {
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x47242f; continue; }
  pc = 0x472451; continue;
 }
 0x472451 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00472452(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x472452); }
 let mut pc: u32 = 0x472452;
 loop { match pc {
 0x472452 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x526020u32; cpu.ebp = v; }
  { let v: u32 = 0x40u32; cpu.w32(0x4d6be0u32, v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x472477; continue;
 }
 0x472477 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x472499; continue; }
  pc = 0x472480; continue;
 }
 0x472480 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16).wrapping_add(0x4u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x8u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  cpu.push(0x472497); f_004714a5(cpu); if cpu.returned_to != 0x472497 { return; }
  if cpu.cf { pc = 0x4724ac; continue; }
  pc = 0x472499; continue;
 }
 0x472499 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x4d6be0u32)); cpu.w32(0x4d6be0u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x472477; continue; }
  pc = 0x4724a4; continue;
 }
 0x4724a4 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4724ac => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4b94bcu32, v); }
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
pub fn f_004724be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4724be); }
 let mut pc: u32 = 0x4724be;
 loop { match pc {
 0x4724be => {
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x0u32); }
  if cpu.zf { pc = 0x4724df; continue; }
  pc = 0x4724c7; continue;
 }
 0x4724c7 => {
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x4d6a08u32), 0x10u32); }
  if (cpu.sf != cpu.of) { pc = 0x472614; continue; }
  pc = 0x4724da; continue;
 }
 0x4724da => {
  pc = 0x472a2d; continue;
 }
 0x4724df => {
  { let v: u32 = cpu.r32(0x4d6ab4u32); cpu.edi = v; }
  { let v: u32 = 0x526020u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4d6ba4u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x4d6ba8u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x472516; continue;
 }
 0x472516 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x47260a; continue; }
  pc = 0x472521; continue;
 }
 0x472521 => {
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x472540; continue; }
  pc = 0x472527; continue;
 }
 0x472527 => {
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x472540; continue; }
  pc = 0x47252d; continue;
 }
 0x47252d => {
  { let v: u8 = cpu.inc8(cpu.r8(0x5u32.wrapping_add(cpu.esi))); cpu.w8(0x5u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.esi)), 0xbu8); }
  if cpu.zf { pc = 0x47253c; continue; }
  pc = 0x472536; continue;
 }
 0x472536 => {
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.esi)), 0x12u8); }
  if !cpu.zf { pc = 0x472540; continue; }
  pc = 0x47253c; continue;
 }
 0x47253c => {
  { let v: u8 = 0xffu8; cpu.w8(0x5u32.wrapping_add(cpu.esi), v); }
  pc = 0x472540; continue;
 }
 0x472540 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47260a; continue; }
  pc = 0x472549; continue;
 }
 0x472549 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47260a; continue; }
  pc = 0x472553; continue;
 }
 0x472553 => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x47260a; continue; }
  pc = 0x47255c; continue;
 }
 0x47255c => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x47260a; continue; }
  pc = 0x472566; continue;
 }
 0x472566 => {
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x472587; continue; }
  pc = 0x47256f; continue;
 }
 0x47256f => {
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  if (cpu.sf == cpu.of) { pc = 0x472587; continue; }
  pc = 0x472577; continue;
 }
 0x472577 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x472593; continue; }
  pc = 0x472587; continue;
 }
 0x472587 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x47260a; continue;
 }
 0x472593 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x30u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d677eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1e1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4725e1; continue; }
  pc = 0x4725d0; continue;
 }
 0x4725d0 => {
  { cpu.sub8(cpu.r8(0x5u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x4725e1; continue; }
  pc = 0x4725d6; continue;
 }
 0x4725d6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x5u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xc74u32); cpu.eax = v; }
  pc = 0x4725e1; continue;
 }
 0x4725e1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4725f6; continue; }
  pc = 0x4725f4; continue;
 }
 0x4725f4 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4725f6; continue;
 }
 0x4725f6 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xe0u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x10u8; cpu.w8(0xfu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x47260a; continue;
 }
 0x47260a => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x472516; continue; }
  pc = 0x472614; continue;
 }
 0x472614 => {
  { let v: u16 = cpu.r16(0x4d6a00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x20u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6a04u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x4d6ba4u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x4d6ba8u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d6796u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x53u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x56c9c3u32, v); }
  { let v: u16 = cpu.r16(0x4d6798u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1bu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x56c9c5u32, v); }
  { let v: u16 = cpu.r16(0x4d6792u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x53u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x56c9c7u32, v); }
  { let v: u16 = cpu.r16(0x4d6794u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1bu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x56c9c9u32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x56c9cbu32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x56c9cfu32, v); }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if (cpu.sf != cpu.of) { pc = 0x4726e2; continue; }
  pc = 0x472698; continue;
 }
 0x472698 => {
  { let v: u16 = cpu.r16(0x4d679au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x53u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x56c9cbu32, v); }
  { let v: u16 = cpu.r16(0x4d679cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1bu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x56c9cdu32, v); }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x4u16); }
  if (cpu.sf != cpu.of) { pc = 0x4726e2; continue; }
  pc = 0x4726c2; continue;
 }
 0x4726c2 => {
  { let v: u16 = cpu.r16(0x4d679eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x53u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x56c9cfu32, v); }
  { let v: u16 = cpu.r16(0x4d67a0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1bu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x56c9d1u32, v); }
  pc = 0x4726e2; continue;
 }
 0x4726e2 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x472708; continue; }
  pc = 0x4726f0; continue;
 }
 0x4726f0 => {
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  if (cpu.sf == cpu.of) { pc = 0x472708; continue; }
  pc = 0x4726f8; continue;
 }
 0x4726f8 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x472714; continue; }
  pc = 0x472708; continue;
 }
 0x472708 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x4727ad; continue;
 }
 0x472714 => {
  { cpu.sub8(cpu.r8(0x56d306u32), 0x0u8); }
  if cpu.zf { pc = 0x4727ad; continue; }
  pc = 0x472721; continue;
 }
 0x472721 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x56c9c3u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4727ad; continue; }
  pc = 0x47272e; continue;
 }
 0x47272e => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x56c9c5u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4727ad; continue; }
  pc = 0x47273b; continue;
 }
 0x47273b => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x56c9c3u32)); }
  if (cpu.sf != cpu.of) { pc = 0x4727ad; continue; }
  pc = 0x472748; continue;
 }
 0x472748 => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x56c9c5u32)); }
  if (cpu.sf != cpu.of) { pc = 0x4727ad; continue; }
  pc = 0x472751; continue;
 }
 0x472751 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x56c9c3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x56c9c5u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xffddu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d677eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4dbu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d677eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4e3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x4727ad; continue;
 }
 0x4727ad => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x4727d2; continue; }
  pc = 0x4727ba; continue;
 }
 0x4727ba => {
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  if (cpu.sf == cpu.of) { pc = 0x4727d2; continue; }
  pc = 0x4727c2; continue;
 }
 0x4727c2 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x4727de; continue; }
  pc = 0x4727d2; continue;
 }
 0x4727d2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x472877; continue;
 }
 0x4727de => {
  { cpu.sub8(cpu.r8(0x56d307u32), 0x0u8); }
  if cpu.zf { pc = 0x472877; continue; }
  pc = 0x4727eb; continue;
 }
 0x4727eb => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x56c9c7u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x472877; continue; }
  pc = 0x4727f8; continue;
 }
 0x4727f8 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x56c9c9u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x472877; continue; }
  pc = 0x472805; continue;
 }
 0x472805 => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x56c9c7u32)); }
  if (cpu.sf != cpu.of) { pc = 0x472877; continue; }
  pc = 0x472812; continue;
 }
 0x472812 => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x56c9c9u32)); }
  if (cpu.sf != cpu.of) { pc = 0x472877; continue; }
  pc = 0x47281b; continue;
 }
 0x47281b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x56c9c7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x56c9c9u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xffddu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d677eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4dfu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d677eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4e3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x472877; continue;
 }
 0x472877 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x47289c; continue; }
  pc = 0x472884; continue;
 }
 0x472884 => {
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  if (cpu.sf == cpu.of) { pc = 0x47289c; continue; }
  pc = 0x47288c; continue;
 }
 0x47288c => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x4728a8; continue; }
  pc = 0x47289c; continue;
 }
 0x47289c => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x47294f; continue;
 }
 0x4728a8 => {
  { cpu.sub8(cpu.r8(0x56d308u32), 0x0u8); }
  if cpu.zf { pc = 0x47294f; continue; }
  pc = 0x4728b5; continue;
 }
 0x4728b5 => {
  { cpu.sub16(cpu.r16(0x56c9cbu32), 0xffffu16); }
  if cpu.zf { pc = 0x47294f; continue; }
  pc = 0x4728c3; continue;
 }
 0x4728c3 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x56c9cbu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47294f; continue; }
  pc = 0x4728d0; continue;
 }
 0x4728d0 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x56c9cdu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47294f; continue; }
  pc = 0x4728dd; continue;
 }
 0x4728dd => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x56c9cbu32)); }
  if (cpu.sf != cpu.of) { pc = 0x47294f; continue; }
  pc = 0x4728ea; continue;
 }
 0x4728ea => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x56c9cdu32)); }
  if (cpu.sf != cpu.of) { pc = 0x47294f; continue; }
  pc = 0x4728f3; continue;
 }
 0x4728f3 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x56c9cbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x56c9cdu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xffddu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d677eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4e7u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d677eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4e3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x47294f; continue;
 }
 0x47294f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x5764deu32), 0x0u8); }
  if !cpu.zf { pc = 0x472974; continue; }
  pc = 0x47295c; continue;
 }
 0x47295c => {
  { cpu.sub32(cpu.edi, 0x4d8234u32); }
  if (cpu.sf == cpu.of) { pc = 0x472974; continue; }
  pc = 0x472964; continue;
 }
 0x472964 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6ac0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4d94f6u32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x472980; continue; }
  pc = 0x472974; continue;
 }
 0x472974 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5764deu32, v); }
  pc = 0x472a27; continue;
 }
 0x472980 => {
  { cpu.sub8(cpu.r8(0x56d309u32), 0x0u8); }
  if cpu.zf { pc = 0x472a27; continue; }
  pc = 0x47298d; continue;
 }
 0x47298d => {
  { cpu.sub16(cpu.r16(0x56c9cfu32), 0xffffu16); }
  if cpu.zf { pc = 0x472a27; continue; }
  pc = 0x47299b; continue;
 }
 0x47299b => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x56c9cfu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x472a27; continue; }
  pc = 0x4729a8; continue;
 }
 0x4729a8 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x56c9d1u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x472a27; continue; }
  pc = 0x4729b5; continue;
 }
 0x4729b5 => {
  { cpu.sub16((cpu.ecx as u16), cpu.r16(0x56c9cfu32)); }
  if (cpu.sf != cpu.of) { pc = 0x472a27; continue; }
  pc = 0x4729c2; continue;
 }
 0x4729c2 => {
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x56c9d1u32)); }
  if (cpu.sf != cpu.of) { pc = 0x472a27; continue; }
  pc = 0x4729cb; continue;
 }
 0x4729cb => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x56c9cfu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x56c9d1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xffddu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d677eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4ebu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x4d677eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4e3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x472a27; continue;
 }
 0x472a27 => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6ab4u32, v); }
  pc = 0x472a2d; continue;
 }
 0x472a2d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00472a2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x472a2e); }
 let mut pc: u32 = 0x472a2e;
 loop { match pc {
 0x472a2e => {
  { cpu.sub8(cpu.r8(0x4e1fc5u32), 0x0u8); }
  if cpu.zf { pc = 0x472afe; continue; }
  pc = 0x472a3b; continue;
 }
 0x472a3b => {
  { let v: u32 = 0x559754u32; cpu.edi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x472a45; continue;
 }
 0x472a45 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x0u16); }
  if (cpu.sf != cpu.of) { pc = 0x472af4; continue; }
  pc = 0x472a50; continue;
 }
 0x472a50 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4b839cu32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x472a90; continue; }
  pc = 0x472a60; continue;
 }
 0x472a60 => {
  { cpu.sub8((cpu.eax as u8), 0x15u8); }
  if cpu.zf { pc = 0x472a90; continue; }
  pc = 0x472a64; continue;
 }
 0x472a64 => {
  { cpu.sub8((cpu.eax as u8), 0x16u8); }
  if cpu.zf { pc = 0x472a90; continue; }
  pc = 0x472a68; continue;
 }
 0x472a68 => {
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if cpu.zf { pc = 0x472a90; continue; }
  pc = 0x472a6c; continue;
 }
 0x472a6c => {
  { cpu.sub8((cpu.eax as u8), 0x18u8); }
  if cpu.zf { pc = 0x472a90; continue; }
  pc = 0x472a70; continue;
 }
 0x472a70 => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x472a90; continue; }
  pc = 0x472a74; continue;
 }
 0x472a74 => {
  { cpu.sub8((cpu.eax as u8), 0x1au8); }
  if cpu.zf { pc = 0x472a90; continue; }
  pc = 0x472a78; continue;
 }
 0x472a78 => {
  { cpu.sub8((cpu.eax as u8), 0x1fu8); }
  if cpu.zf { pc = 0x472a90; continue; }
  pc = 0x472a7c; continue;
 }
 0x472a7c => {
  { cpu.sub8((cpu.eax as u8), 0x21u8); }
  if cpu.zf { pc = 0x472a90; continue; }
  pc = 0x472a80; continue;
 }
 0x472a80 => {
  { cpu.sub8((cpu.eax as u8), 0x23u8); }
  if cpu.zf { pc = 0x472a90; continue; }
  pc = 0x472a84; continue;
 }
 0x472a84 => {
  { cpu.sub8((cpu.eax as u8), 0x25u8); }
  if cpu.zf { pc = 0x472a90; continue; }
  pc = 0x472a88; continue;
 }
 0x472a88 => {
  { cpu.sub8((cpu.eax as u8), 0x1du8); }
  if !cpu.zf { pc = 0x472af4; continue; }
  pc = 0x472a90; continue;
 }
 0x472a90 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x472a97); f_004859a8(cpu); if cpu.returned_to != 0x472a97 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  cpu.push(0x472aa3); f_0041b74b(cpu); if cpu.returned_to != 0x472aa3 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x4d6a00u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x4d6a04u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x472af4; continue; }
  pc = 0x472abf; continue;
 }
 0x472abf => {
  { cpu.and16((cpu.ebx as u16), (cpu.ebx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x472af4; continue; }
  pc = 0x472ac4; continue;
 }
 0x472ac4 => {
  { let v: u16 = cpu.r16(0x4d6ba4u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub(0x30u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebp as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x472af4; continue; }
  pc = 0x472ad4; continue;
 }
 0x472ad4 => {
  { let v: u16 = cpu.r16(0x4d6ba8u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub(0x8u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.ebp as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x472af4; continue; }
  pc = 0x472ae4; continue;
 }
 0x472ae4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x4e1fe6u32; cpu.esi = v; }
  cpu.push(0x472af2); f_00447fe3(cpu); if cpu.returned_to != 0x472af2 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x472af4; continue;
 }
 0x472af4 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x28u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x472a45; continue; }
  pc = 0x472afe; continue;
 }
 0x472afe => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00472b00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x472b00); } r_00472b00(cpu, 0x472b00); }
pub fn f_00472fe8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x472fe8); } r_00472b00(cpu, 0x472fe8); }
fn r_00472b00(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x472b00 => {
  { cpu.sub16(cpu.r16(0x81u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x472b32; continue; }
  pc = 0x472b0c; continue;
 }
 0x472b0c => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  { let v: u16 = cpu.inc16(cpu.r16(0x81u32.wrapping_add(cpu.ebp))); cpu.w16(0x81u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0x81u32.wrapping_add(cpu.ebp)), 0x7u16); }
  if cpu.zf { pc = 0x472b28; continue; }
  pc = 0x472b27; continue;
 }
 0x472b27 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x472b28 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x81u32.wrapping_add(cpu.ebp), v); }
  pc = 0x472b32; continue;
 }
 0x472b32 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x472b56; continue; }
  pc = 0x472b39; continue;
 }
 0x472b39 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x472b3f); f_0046b31b(cpu); if cpu.returned_to != 0x472b3f { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x472bec; continue; }
  pc = 0x472b46; continue;
 }
 0x472b46 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x472f48; continue; }
  pc = 0x472b51; continue;
 }
 0x472b51 => {
  pc = 0x472bed; continue;
 }
 0x472b56 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x472b64; continue; }
  pc = 0x472b5f; continue;
 }
 0x472b5f => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x472b69; continue; }
  pc = 0x472b64; continue;
 }
 0x472b64 => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x472b69; continue;
 }
 0x472b69 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x472b88; continue; }
  pc = 0x472b70; continue;
 }
 0x472b70 => {
  { let v: u8 = 0x4u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  pc = 0x472b89; continue;
 }
 0x472b88 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x472b89 => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x472bec; continue; }
  pc = 0x472b95; continue;
 }
 0x472b95 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x472bb5); f_00481755(cpu); if cpu.returned_to != 0x472bb5 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x472bbf); f_0045bcfc(cpu); if cpu.returned_to != 0x472bbf { return; }
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x472bca); f_00472dbd(cpu); if cpu.returned_to != 0x472bca { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x472bd9; continue; }
  pc = 0x472bd4; continue;
 }
 0x472bd4 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x472bde; continue; }
  pc = 0x472bd9; continue;
 }
 0x472bd9 => {
  { let v: u32 = 0x5b355cu32; cpu.eax = v; }
  pc = 0x472bde; continue;
 }
 0x472bde => {
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), cpu.r8(0x20u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x472bec); f_004824fa(cpu); if cpu.returned_to != 0x472bec { return; }
  pc = 0x472bec; continue;
 }
 0x472bec => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x472bed => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x472f48; continue; }
  pc = 0x472bf8; continue;
 }
 0x472bf8 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x472f48; continue; }
  pc = 0x472c03; continue;
 }
 0x472c03 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x472d76; continue; }
  pc = 0x472c10; continue;
 }
 0x472c10 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x472c1e; continue; }
  pc = 0x472c19; continue;
 }
 0x472c19 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x472c23; continue; }
  pc = 0x472c1e; continue;
 }
 0x472c1e => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x472c23; continue;
 }
 0x472c23 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3du32.wrapping_add(cpu.ebp))); cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x472c2e; continue; }
  pc = 0x472c29; continue;
 }
 0x472c29 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  pc = 0x472c2e; continue;
 }
 0x472c2e => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x472c62; continue; }
  pc = 0x472c34; continue;
 }
 0x472c34 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = !(cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x472c5d; continue; }
  pc = 0x472c43; continue;
 }
 0x472c43 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)) & 0x7u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x472d6e; continue; }
  pc = 0x472c5d; continue;
 }
 0x472c5d => {
  { let v: u8 = 0xfu8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x472c62; continue;
 }
 0x472c62 => {
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x472d6e; continue; }
  pc = 0x472c6d; continue;
 }
 0x472c6d => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x472c74); f_0043fede(cpu); if cpu.returned_to != 0x472c74 { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x472d6e; continue; }
  pc = 0x472c7d; continue;
 }
 0x472c7d => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x472d6e; continue; }
  pc = 0x472c89; continue;
 }
 0x472c89 => {
  cpu.push(0x472c8e); f_004782f8(cpu); if cpu.returned_to != 0x472c8e { return; }
  if cpu.cf { pc = 0x472d6e; continue; }
  pc = 0x472c94; continue;
 }
 0x472c94 => {
  cpu.push(0x472c99); f_0047a294(cpu); if cpu.returned_to != 0x472c99 { return; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x509290u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5cf388u32, v); }
  cpu.push(0x472cb1); f_00453ff6(cpu); if cpu.returned_to != 0x472cb1 { return; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x472cc9); f_0045bcfc(cpu); if cpu.returned_to != 0x472cc9 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0xcu16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f4eu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x575f28u32, v); }
  cpu.push(0x472ced); f_0045309c(cpu); if cpu.returned_to != 0x472ced { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x472d0b; continue; }
  pc = 0x472d00; continue;
 }
 0x472d00 => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x472d0b; continue; }
  pc = 0x472d09; continue;
 }
 0x472d09 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x472d0b; continue;
 }
 0x472d0b => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x472d1c); f_0045bd38(cpu); if cpu.returned_to != 0x472d1c { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x472d25); f_00481755(cpu); if cpu.returned_to != 0x472d25 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x34u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x472d75; continue; }
  pc = 0x472d34; continue;
 }
 0x472d34 => {
  { let v: u32 = cpu.r32(0x5cf388u32); cpu.esi = v; }
  cpu.push(0x472d3f); f_004527fe(cpu); if cpu.returned_to != 0x472d3f { return; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x4e1f4eu32) as u32; cpu.esi = v; }
  cpu.push(0x472d4c); f_00472dbd(cpu); if cpu.returned_to != 0x472d4c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x472d5b; continue; }
  pc = 0x472d56; continue;
 }
 0x472d56 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x472d60; continue; }
  pc = 0x472d5b; continue;
 }
 0x472d5b => {
  { let v: u32 = 0x5b355cu32; cpu.eax = v; }
  pc = 0x472d60; continue;
 }
 0x472d60 => {
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), cpu.r8(0x20u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x472d6e); f_004824fa(cpu); if cpu.returned_to != 0x472d6e { return; }
  pc = 0x472d6e; continue;
 }
 0x472d6e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x472d75 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x472d76 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x74u32.wrapping_add(cpu.ebp))); cpu.w8(0x74u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x74u32.wrapping_add(cpu.ebp)) & 0x7u8; cpu.w8(0x74u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x74u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let a = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); let b = ((cpu.eax >> 8) as u8); { let v: u8 = b; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x472d8f); f_0043fede(cpu); if cpu.returned_to != 0x472d8f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x472d6e; continue; }
  pc = 0x472d99; continue;
 }
 0x472d99 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x472d6e; continue; }
  pc = 0x472da9; continue;
 }
 0x472da9 => {
  cpu.push(0x472dae); f_004782f8(cpu); if cpu.returned_to != 0x472dae { return; }
  if cpu.cf { cpu.poll(); pc = 0x472d6e; continue; }
  pc = 0x472db0; continue;
 }
 0x472db0 => {
  { let v: u8 = cpu.r8(0x74u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x472c94; continue;
 }
 0x472f48 => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x472f5e; continue; }
  pc = 0x472f51; continue;
 }
 0x472f51 => {
  { cpu.sub8(cpu.r8(0x4e1e4au32), 0x2u8); }
  if !cpu.zf { pc = 0x47305a; continue; }
  pc = 0x472f5e; continue;
 }
 0x472f5e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x472f6c; continue; }
  pc = 0x472f67; continue;
 }
 0x472f67 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x472f71; continue; }
  pc = 0x472f6c; continue;
 }
 0x472f6c => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x472f71; continue;
 }
 0x472f71 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x14u32.wrapping_add(cpu.edi))); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  if !cpu.sf { pc = 0x472f7a; continue; }
  pc = 0x472f76; continue;
 }
 0x472f76 => {
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x472f7a; continue;
 }
 0x472f7a => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x47305a; continue; }
  pc = 0x472f84; continue;
 }
 0x472f84 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f4eu32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4d6bd4u32, v); }
  cpu.push(0x472fb3); f_00453316(cpu); if cpu.returned_to != 0x472fb3 { return; }
  if cpu.cf { pc = 0x47305a; continue; }
  pc = 0x472fb9; continue;
 }
 0x472fb9 => {
  { cpu.pushad(); }
  cpu.push(0x472fbf); f_0046c3d4(cpu); if cpu.returned_to != 0x472fbf { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x472fe7; continue; }
  pc = 0x472fc2; continue;
 }
 0x472fc2 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5cf44cu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x5cf44eu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.esi = v; }
  cpu.push(0x472fe0); f_0045309c(cpu); if cpu.returned_to != 0x472fe0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x473009; continue;
 }
 0x472fe7 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x473009; continue; }
  pc = 0x472fee; continue;
 }
 0x472fe8 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x473009; continue; }
  pc = 0x472fee; continue;
 }
 0x472fee => {
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0x7fu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xefu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x473007); f_0044bc7b(cpu); if cpu.returned_to != 0x473007 { return; }
  pc = 0x47305a; continue;
 }
 0x473009 => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x47301e; continue; }
  pc = 0x473010; continue;
 }
 0x473010 => {
  { let v: u16 = cpu.r16(0x5cf44cu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5cf44eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x47301e; continue;
 }
 0x47301e => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x47302d); f_00481755(cpu); if cpu.returned_to != 0x47302d { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x39u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x47305b; continue; }
  pc = 0x473035; continue;
 }
 0x473035 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if cpu.zf { pc = 0x47305a; continue; }
  pc = 0x47303e; continue;
 }
 0x47303e => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x47304c; continue; }
  pc = 0x473046; continue;
 }
 0x473046 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x473050; continue;
 }
 0x47304c => {
  { let v: u8 = cpu.inc8(cpu.r8(0x39u32.wrapping_add(cpu.ebp))); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x473050; continue;
 }
 0x473050 => {
  { let v: u8 = cpu.and8(cpu.r8(0x39u32.wrapping_add(cpu.ebp)), 0x7u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x47305a; continue;
 }
 0x47305a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47305b => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x473127; continue; }
  pc = 0x473066; continue;
 }
 0x473066 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f4eu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x473076); f_0045bcfc(cpu); if cpu.returned_to != 0x473076 { return; }
  cpu.push(0x47307b); f_00453316(cpu); if cpu.returned_to != 0x47307b { return; }
  if cpu.cf { pc = 0x473122; continue; }
  pc = 0x473081; continue;
 }
 0x473081 => {
  { let v = cpu.r32(0x4d6bd4u32); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x5cf388u32, v); }
  cpu.push(0x473092); f_0046c3d4(cpu); if cpu.returned_to != 0x473092 { return; }
  if cpu.cf { pc = 0x473122; continue; }
  pc = 0x473098; continue;
 }
 0x473098 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x8u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.esi = v; }
  cpu.push(0x4730ac); f_0045309c(cpu); if cpu.returned_to != 0x4730ac { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x4730cf; continue; }
  pc = 0x4730c4; continue;
 }
 0x4730c4 => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x4730cf; continue; }
  pc = 0x4730cd; continue;
 }
 0x4730cd => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x4730cf; continue;
 }
 0x4730cf => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5cf388u32); cpu.esi = v; }
  cpu.push(0x4730e6); f_004527fe(cpu); if cpu.returned_to != 0x4730e6 { return; }
  cpu.push(0x4730eb); f_0045bd38(cpu); if cpu.returned_to != 0x4730eb { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x4e1f4eu32) as u32; cpu.esi = v; }
  cpu.push(0x473100); f_00472dbd(cpu); if cpu.returned_to != 0x473100 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x47310f; continue; }
  pc = 0x47310a; continue;
 }
 0x47310a => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x473114; continue; }
  pc = 0x47310f; continue;
 }
 0x47310f => {
  { let v: u32 = 0x5b355cu32; cpu.eax = v; }
  pc = 0x473114; continue;
 }
 0x473114 => {
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), cpu.r8(0x20u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x473122); f_004824fa(cpu); if cpu.returned_to != 0x473122 { return; }
  pc = 0x473122; continue;
 }
 0x473122 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x473127; continue;
 }
 0x473127 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00472dbd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x472dbd); }
 let mut pc: u32 = 0x472dbd;
 loop { match pc {
 0x472dbd => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x472f20; continue; }
  pc = 0x472dc9; continue;
 }
 0x472dc9 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x16u8); }
  if cpu.zf { pc = 0x472ea5; continue; }
  pc = 0x472dd4; continue;
 }
 0x472dd4 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.zf { pc = 0x472f07; continue; }
  pc = 0x472ddf; continue;
 }
 0x472ddf => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x5cf40eu32.wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x5cf40cu32.wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x5cf390u32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cf38cu32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2au16; cpu.w16(0x4d6934u32, v); }
  { let v: u8 = 0x20u8; cpu.w8(0x5d5f23u32, v); }
  { let v: u8 = 0x64u8; cpu.w8(0x5d5f24u32, v); }
  cpu.push(0x472e2e); f_00480e87(cpu); if cpu.returned_to != 0x472e2e { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x5d5f24u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  if cpu.cf { pc = 0x472e44; continue; }
  pc = 0x472e40; continue;
 }
 0x472e40 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x472e44; continue;
 }
 0x472e44 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x5cf42eu32.wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x5cf42cu32.wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x5cf3d0u32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cf3ccu32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2au16; cpu.w16(0x4d6934u32, v); }
  { let v: u8 = 0x20u8; cpu.w8(0x5d5f23u32, v); }
  { let v: u8 = 0x64u8; cpu.w8(0x5d5f24u32, v); }
  cpu.push(0x472e8d); f_00480e87(cpu); if cpu.returned_to != 0x472e8d { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x5d5f24u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6934u32, v); }
  if cpu.cf { pc = 0x472ea3; continue; }
  pc = 0x472e9f; continue;
 }
 0x472e9f => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x472ea3; continue;
 }
 0x472ea3 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x472ea5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x472eb5; continue; }
  pc = 0x472eb0; continue;
 }
 0x472eb0 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x472eba; continue; }
  pc = 0x472eb5; continue;
 }
 0x472eb5 => {
  { let v: u32 = 0x5b355cu32; cpu.eax = v; }
  pc = 0x472eba; continue;
 }
 0x472eba => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x63u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x472eda); f_004679cb(cpu); if cpu.returned_to != 0x472eda { return; }
  if cpu.cf { pc = 0x472eef; continue; }
  pc = 0x472edc; continue;
 }
 0x472edc => {
  { let v: u8 = 0x2u8; cpu.w8(0x59dadau32, v); }
  { let v: u16 = cpu.r16(0x5cc64eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x59dad8u32, v); }
  pc = 0x472eef; continue;
 }
 0x472eef => {
  { cpu.popad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x5ba1c1u32, v); }
  cpu.push(0x472efc); f_004820da(cpu); if cpu.returned_to != 0x472efc { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5ba1c1u32, v); }
  if cpu.cf { pc = 0x472f05; continue; }
  pc = 0x472f05; continue;
 }
 0x472f05 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x472f07 => {
  { let v: u8 = 0x19u8; cpu.w8(0x5d5f25u32, v); }
  cpu.push(0x472f13); f_0048126b(cpu); if cpu.returned_to != 0x472f13 { return; }
  if cpu.cf { pc = 0x472f1e; continue; }
  pc = 0x472f15; continue;
 }
 0x472f15 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x80u8); cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  pc = 0x472f1e; continue;
 }
 0x472f1e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x472f20 => {
  { let v: u8 = 0x19u8; cpu.w8(0x5d5f25u32, v); }
  { let v: u8 = 0x64u8; cpu.w8(0x5d5f26u32, v); }
  cpu.push(0x472f33); f_0048126b(cpu); if cpu.returned_to != 0x472f33 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x5d5f26u32, v); }
  if cpu.cf { pc = 0x472f40; continue; }
  pc = 0x472f3c; continue;
 }
 0x472f3c => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x472f40; continue;
 }
 0x472f40 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00472f42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x472f42); }
 let mut pc: u32 = 0x472f42;
 loop { match pc {
 0x472f42 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047315f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47315f); }
 let mut pc: u32 = 0x47315f;
 loop { match pc {
 0x47315f => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x52bbf0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x4731fc; continue; }
  pc = 0x473178; continue;
 }
 0x473178 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x4731fa; continue; }
  pc = 0x473185; continue;
 }
 0x473185 => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x47319a; continue; }
  pc = 0x473195; continue;
 }
 0x473195 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x47319f; continue; }
  pc = 0x47319a; continue;
 }
 0x47319a => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x47319f; continue;
 }
 0x47319f => {
  { let v: u32 = cpu.edi; cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5cc621u32, v); }
  cpu.push(0x4731b4); f_00458ee1(cpu); if cpu.returned_to != 0x4731b4 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cc621u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x4731dd; continue; }
  pc = 0x4731c5; continue;
 }
 0x4731c5 => {
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x4731cf; continue; }
  pc = 0x4731ca; continue;
 }
 0x4731ca => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x4731d4; continue; }
  pc = 0x4731cf; continue;
 }
 0x4731cf => {
  { let v: u32 = 0x5b355cu32; cpu.edi = v; }
  pc = 0x4731d4; continue;
 }
 0x4731d4 => {
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x4731dd; continue;
 }
 0x4731dd => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x75u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x2au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x76u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4731fa => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4731fc => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x473202); f_00473240(cpu); if cpu.returned_to != 0x473202 { return; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { cpu.poll(); pc = 0x4731fa; continue; }
  pc = 0x473207; continue;
 }
 0x473207 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x8au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x8bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x63u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x75u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x47315f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00473240(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x473240); }
 let mut pc: u32 = 0x473240;
 loop { match pc {
 0x473240 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4d6bd8u32, v); }
  pc = 0x473254; continue;
 }
 0x473254 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x4d6bd8u32)); cpu.w32(0x4d6bd8u32, v); }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x4d6bd8u32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u32 = cpu.r8(0x4au32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4732ca; continue; }
  pc = 0x473280; continue;
 }
 0x473280 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x4732ca; continue; }
  pc = 0x473285; continue;
 }
 0x473285 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x4732ca; continue; }
  pc = 0x47328d; continue;
 }
 0x47328d => {
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6a38u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x4732be; continue; }
  pc = 0x4732bb; continue;
 }
 0x4732bb => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x473254; continue; }
  pc = 0x4732be; continue;
 }
 0x4732be => {
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4732ca => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x473254; continue; }
  pc = 0x4732ce; continue;
 }
 0x4732ce => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004732d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4732d8); }
 let mut pc: u32 = 0x4732d8;
 loop { match pc {
 0x4732d8 => {
  cpu.push(0x4732dd); f_004737c6(cpu); if cpu.returned_to != 0x4732dd { return; }
  cpu.push(0x4732e2); f_0047362e(cpu); if cpu.returned_to != 0x4732e2 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cf452u32, v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.esi = v; }
  pc = 0x4732f3; continue;
 }
 0x4732f3 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x4732ff); f_00473454(cpu); if cpu.returned_to != 0x4732ff { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x28u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xd7u8); }
  if cpu.cf { cpu.poll(); pc = 0x4732f3; continue; }
  pc = 0x47330d; continue;
 }
 0x47330d => {
  { let v: u8 = 0x0u8; cpu.w8(0x5cf452u32, v); }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0xffu32); cpu.esi = v; }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0xd7u8); }
  if cpu.cf { cpu.poll(); pc = 0x4732f3; continue; }
  pc = 0x473323; continue;
 }
 0x473323 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00473324(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x473324); }
 let mut pc: u32 = 0x473324;
 loop { match pc {
 0x473324 => {
  { cpu.sub8(cpu.r8(0x5cf454u32), 0x1u8); }
  if cpu.cf { pc = 0x473414; continue; }
  pc = 0x473331; continue;
 }
 0x473331 => {
  if !cpu.zf { pc = 0x47338d; continue; }
  pc = 0x473333; continue;
 }
 0x473333 => {
  { let v: u8 = 0x2u8; cpu.w8(0x5cf454u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5cf8ebu32, v); }
  { let v = cpu.r32(0x5cf8f3u32); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x5cf8efu32, v); }
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5cf8f3u32); cpu.edi = v; }
  { let v: u32 = 0x358cu32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v = cpu.r32(0x4d6b4cu32); cpu.push(v); }
  { let v = cpu.r32(0x5cf8f3u32); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6b4cu32, v); }
  cpu.push(0x47337a); f_004737c6(cpu); if cpu.returned_to != 0x47337a { return; }
  cpu.push(0x47337f); f_0047362e(cpu); if cpu.returned_to != 0x47337f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cf452u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6b4cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47338d => {
  { let v = cpu.r32(0x4d6b4cu32); cpu.push(v); }
  { let v = cpu.r32(0x5cf8f3u32); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6b4cu32, v); }
  { let v: u32 = cpu.r32(0x5cf8ebu32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xd7u32); }
  if cpu.zf { pc = 0x4733ed; continue; }
  pc = 0x4733ad; continue;
 }
 0x4733ad => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5cf8efu32); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5cf452u32, v); }
  pc = 0x4733bc; continue;
 }
 0x4733bc => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x4733c8); f_00473454(cpu); if cpu.returned_to != 0x4733c8 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x28u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xd7u8); }
  if cpu.cf { cpu.poll(); pc = 0x4733bc; continue; }
  pc = 0x4733d6; continue;
 }
 0x4733d6 => {
  { let v: u32 = cpu.add32(cpu.r32(0x5cf8efu32), 0xffu32); cpu.w32(0x5cf8efu32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x5cf8ebu32)); cpu.w32(0x5cf8ebu32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6b4cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4733ed => {
  { let v: u32 = cpu.pop(); cpu.w32(0x4d6b4cu32, v); }
  { let v: u32 = cpu.r32(0x5cf8f3u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.edi = v; }
  { let v: u32 = 0x358cu32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u8 = 0x0u8; cpu.w8(0x5cf454u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5cf455u32, v); }
  pc = 0x473414; continue;
 }
 0x473414 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00473415(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x473415); }
 let mut pc: u32 = 0x473415;
 loop { match pc {
 0x473415 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x5cf494u32; cpu.edi = v; }
  { let v: u32 = 0xd7u32; cpu.ecx = v; }
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
pub fn f_0047342d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47342d); }
 let mut pc: u32 = 0x47342d;
 loop { match pc {
 0x47342d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x5cf7f0u32; cpu.edi = v; }
  { let v: u32 = 0x35u32; cpu.ecx = v; }
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
pub fn f_00473445(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x473445); }
 let mut pc: u32 = 0x473445;
 loop { match pc {
 0x473445 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x5cf451u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5cf7f0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00473454(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x473454); }
 let mut pc: u32 = 0x473454;
 loop { match pc {
 0x473454 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x4735a1; continue; }
  pc = 0x47345c; continue;
 }
 0x47345c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5cf450u32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x5cf451u32, v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4d6b4cu32)); cpu.esi = v; }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  { let v = ((cpu.ebx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4d6b4cu32)); cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x4735a1; continue; }
  pc = 0x473492; continue;
 }
 0x473492 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0x0u16); }
  if cpu.zf { pc = 0x4735a1; continue; }
  pc = 0x47349d; continue;
 }
 0x47349d => {
  { cpu.sub8(cpu.r8(0x5cf452u32), 0x0u8); }
  if cpu.zf { pc = 0x4734ac; continue; }
  pc = 0x4734a6; continue;
 }
 0x4734a6 => {
  cpu.push(0x4734ab); f_00473445(cpu); if cpu.returned_to != 0x4734ab { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4734ac => {
  { let v: u32 = cpu.esi; cpu.w32(0x5cf8d3u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5cf8d7u32, v); }
  cpu.push(0x4734bd); f_00473415(cpu); if cpu.returned_to != 0x4734bd { return; }
  cpu.push(0x4734c2); f_0047342d(cpu); if cpu.returned_to != 0x4734c2 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5cf450u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32; cpu.w32(0x5cf494u32.wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u8 = cpu.r8(0x5cf451u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffu32; cpu.w32(0x5cf494u32.wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5cf8dbu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5cf8dfu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5cf8e3u32, v); }
  pc = 0x473502; continue;
 }
 0x473502 => {
  { let v: u32 = cpu.r32(0x5cf8d3u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x5cf8e3u32)); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0xau32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x4735a7; continue; }
  pc = 0x473518; continue;
 }
 0x473518 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6b4cu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5cf8c7u32, v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5cf8cbu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5cf494u32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5cf7f0u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x5cf8d7u32)); }
  if cpu.zf { pc = 0x473597; continue; }
  pc = 0x47354f; continue;
 }
 0x47354f => {
  cpu.push(0x473554); f_004735b4(cpu); if cpu.returned_to != 0x473554 { return; }
  { let v: u32 = cpu.r32(0x5cf8d3u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x5cf8e3u32)); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x5cf451u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5cf494u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x4735a1; continue; }
  pc = 0x473579; continue;
 }
 0x473579 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5cf8dbu32)); }
  if !cpu.cf { pc = 0x47358c; continue; }
  pc = 0x473581; continue;
 }
 0x473581 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5cf8dbu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5cf8dfu32, v); }
  pc = 0x47358c; continue;
 }
 0x47358c => {
  { let v: u32 = cpu.inc32(cpu.r32(0x5cf8e3u32)); cpu.w32(0x5cf8e3u32, v); }
  cpu.poll(); pc = 0x473502; continue;
 }
 0x473597 => {
  { let v: u8 = cpu.r8(0x5cf451u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5cf8dfu32, v); }
  pc = 0x4735a1; continue;
 }
 0x4735a1 => {
  { let v: u8 = cpu.r8(0x5cf8dfu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4735a7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5cf452u32, v); }
  { let v: u8 = cpu.r8(0x5cf8dfu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004735b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4735b4); }
 let mut pc: u32 = 0x4735b4;
 loop { match pc {
 0x4735b4 => {
  { let v: u32 = cpu.esp; cpu.w32(0x5cf8e7u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5cf453u32, v); }
  { let v = cpu.r32(0x5cf8c7u32); cpu.push(v); }
  { let v = cpu.r32(0x5cf8cbu32); cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  pc = 0x4735d2; continue;
 }
 0x4735d2 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x473627; continue; }
  pc = 0x4735d5; continue;
 }
 0x4735d5 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x5cf494u32.wrapping_add(cpu.edx.wrapping_mul(4)))); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x4735d2; continue; }
  pc = 0x4735e1; continue;
 }
 0x4735e1 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x4735e3; continue;
 }
 0x4735e3 => {
  { cpu.sub8(cpu.r8(0xau32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1))), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x4735d2; continue; }
  pc = 0x4735ea; continue;
 }
 0x4735ea => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x5cf494u32.wrapping_add(cpu.edx.wrapping_mul(4)))); }
  if !cpu.cf { pc = 0x473624; continue; }
  pc = 0x473601; continue;
 }
 0x473601 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5cf494u32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v: u8 = cpu.r8(0x5cf453u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5cf7f0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6b4cu32)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x473624; continue;
 }
 0x473624 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x4735e3; continue;
 }
 0x473627 => {
  { let v: u32 = cpu.r32(0x5cf8e7u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047362e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47362e); }
 let mut pc: u32 = 0x47362e;
 loop { match pc {
 0x47362e => {
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.esi = v; }
  { let v: u8 = 0xd7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x473636; continue;
 }
 0x473636 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5cf8cfu32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x473647); f_00473711(cpu); if cpu.returned_to != 0x473647 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x5d39d4u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if cpu.zf { pc = 0x4736f2; continue; }
  pc = 0x473657; continue;
 }
 0x473657 => {
  { let v: u32 = 0x5cf458u32; cpu.ebp = v; }
  pc = 0x47365c; continue;
 }
 0x47365c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4d6b4cu32)); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.div8(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x5cf8cfu32)); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x5cf8cfu32)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0x7fu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r16(0x8u32.wrapping_add(cpu.edx)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 7u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x400u32); }
  if (cpu.sf != cpu.of) { pc = 0x4736ca; continue; }
  pc = 0x4736c5; continue;
 }
 0x4736c5 => {
  { let v: u32 = 0x3ffu32; cpu.ebx = v; }
  pc = 0x4736ca; continue;
 }
 0x4736ca => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6accu32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x5cf8cfu32)); cpu.esi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x19u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x5cf8cfu32)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x5cf8cfu32)); cpu.w32(0x5cf8cfu32, v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x47365c; continue; }
  pc = 0x4736f2; continue;
 }
 0x4736f2 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x5cf8cfu32)); cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.w8(0xau32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x5cf8cfu32)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0xffu32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x473636; continue; }
  pc = 0x473710; continue;
 }
 0x473710 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00473711(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x473711); } r_00473711(cpu, 0x473711); }
pub fn f_00473727(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x473727); } r_00473711(cpu, 0x473727); }
fn r_00473711(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x473711 => {
  { cpu.pushad(); }
  { let v: u32 = 0x0u32; cpu.w32(0x5d39d4u32, v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x47375c; continue; }
  pc = 0x473723; continue;
 }
 0x473723 => {
  { let v: u32 = 0x5cf458u32; cpu.edi = v; }
  { let v: u8 = 0xd7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.ebp = v; }
  pc = 0x473730; continue;
 }
 0x473727 => {
  { let v: u8 = cpu.r8(0x4c2d8bd7u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x4c2d8bd7u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x66u32); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), 0x19740000u32); }
  pc = 0x473739; continue;
 }
 0x473730 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x473752; continue; }
  pc = 0x473739; continue;
 }
 0x473739 => {
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if cpu.zf { pc = 0x473752; continue; }
  pc = 0x47373d; continue;
 }
 0x47373d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x473798; continue; }
  pc = 0x473747; continue;
 }
 0x473747 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x47376b; continue; }
  pc = 0x473752; continue;
 }
 0x473752 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xffu32); cpu.ebp = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x473730; continue; }
  pc = 0x47375c; continue;
 }
 0x47375c => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47375e => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x5d39d4u32)); cpu.w32(0x5d39d4u32, v); }
  cpu.poll(); pc = 0x473752; continue;
 }
 0x47376b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x47375e; continue; }
  pc = 0x473775; continue;
 }
 0x473775 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x47375e; continue; }
  pc = 0x473780; continue;
 }
 0x473780 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x47375e; continue; }
  pc = 0x47378b; continue;
 }
 0x47378b => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x47375e; continue; }
  pc = 0x473796; continue;
 }
 0x473796 => {
  cpu.poll(); pc = 0x473752; continue;
 }
 0x473798 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x6u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x47375e; continue; }
  pc = 0x4737a3; continue;
 }
 0x4737a3 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x6u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x47375e; continue; }
  pc = 0x4737ae; continue;
 }
 0x4737ae => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x47375e; continue; }
  pc = 0x4737b9; continue;
 }
 0x4737b9 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { cpu.poll(); pc = 0x47375e; continue; }
  pc = 0x4737c4; continue;
 }
 0x4737c4 => {
  cpu.poll(); pc = 0x473752; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004737c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4737c6); }
 let mut pc: u32 = 0x4737c6;
 loop { match pc {
 0x4737c6 => {
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.esi = v; }
  { let v: u8 = 0xd7u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x4737d3; continue;
 }
 0x4737d3 => {
  { let v: u32 = 0x35u32; cpu.ecx = v; }
  { let v: u32 = 0x28u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.add32(cpu.esi, 0xffu32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x4737d3; continue; }
  pc = 0x4737e7; continue;
 }
 0x4737e7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004737e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4737e8); }
 let mut pc: u32 = 0x4737e8;
 loop { match pc {
 0x4737e8 => {
  cpu.push(0x4737ed); f_00473dab(cpu); if cpu.returned_to != 0x4737ed { return; }
  cpu.push(0x4737f2); f_00473c96(cpu); if cpu.returned_to != 0x4737f2 { return; }
  cpu.push(0x4737f7); f_00473ac7(cpu); if cpu.returned_to != 0x4737f7 { return; }
  { let v: u32 = 0x26u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5ade00u32); cpu.esi = v; }
  cpu.push(0x47380c); f_00474d56(cpu); if cpu.returned_to != 0x47380c { return; }
  { let v: u32 = 0xe0u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4b94d0u32); cpu.ecx = v; }
  cpu.push(0x473823); f_00474a20(cpu); if cpu.returned_to != 0x473823 { return; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x2u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x473851; continue; }
  pc = 0x47382d; continue;
 }
 0x47382d => {
  { let v: u16 = cpu.r16(0x56d328u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x56d32eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4b94c0u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4b94c2u32, v); }
  cpu.push(0x47384c); f_00474c20(cpu); if cpu.returned_to != 0x47384c { return; }
  cpu.push(0x473851); f_00474bd9(cpu); if cpu.returned_to != 0x473851 { return; }
  pc = 0x473851; continue;
 }
 0x473851 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5ade3cu32); cpu.esi = v; }
  cpu.push(0x473866); f_00474f51(cpu); if cpu.returned_to != 0x473866 { return; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32; cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5ade3cu32); cpu.esi = v; }
  cpu.push(0x473884); f_00474c5f(cpu); if cpu.returned_to != 0x473884 { return; }
  cpu.push(0x473889); f_004739bf(cpu); if cpu.returned_to != 0x473889 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x473891); cpu.call(t); if cpu.returned_to != 0x473891 { return; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5de71cu32, v); }
  cpu.push(0x4738a2); f_0040bca0(cpu); if cpu.returned_to != 0x4738a2 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x4738b4); f_00417ca0(cpu); if cpu.returned_to != 0x4738b4 { return; }
  pc = 0x4738b4; continue;
 }
 0x4738b4 => {
  cpu.push(0x4738b9); f_00415156(cpu); if cpu.returned_to != 0x4738b9 { return; }
  cpu.push(0x4738be); f_0041c122(cpu); if cpu.returned_to != 0x4738be { return; }
  if cpu.cf { pc = 0x4738ce; continue; }
  pc = 0x4738c0; continue;
 }
 0x4738c0 => {
  cpu.push(0x4738c5); f_0041c2ad(cpu); if cpu.returned_to != 0x4738c5 { return; }
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x3u8); }
  if cpu.zf { cpu.poll(); pc = 0x4738b4; continue; }
  pc = 0x4738ce; continue;
 }
 0x4738ce => {
  { cpu.pushad(); }
  cpu.push(0x4738d4); f_004030f0(cpu); if cpu.returned_to != 0x4738d4 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004738d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4738d6); }
 let mut pc: u32 = 0x4738d6;
 loop { match pc {
 0x4738d6 => {
  cpu.push(0x4738db); f_00473dab(cpu); if cpu.returned_to != 0x4738db { return; }
  cpu.push(0x4738e0); f_00473b80(cpu); if cpu.returned_to != 0x4738e0 { return; }
  cpu.push(0x4738e5); f_00473ac7(cpu); if cpu.returned_to != 0x4738e5 { return; }
  { let v: u32 = 0x26u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5ade00u32); cpu.esi = v; }
  cpu.push(0x4738fa); f_00474d56(cpu); if cpu.returned_to != 0x4738fa { return; }
  { let v: u32 = 0xe0u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4b94d0u32); cpu.ecx = v; }
  cpu.push(0x473911); f_00474a20(cpu); if cpu.returned_to != 0x473911 { return; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x2u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47393f; continue; }
  pc = 0x47391b; continue;
 }
 0x47391b => {
  { let v: u16 = cpu.r16(0x56d328u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x56d32eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4b94c0u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4b94c2u32, v); }
  cpu.push(0x47393a); f_00474c20(cpu); if cpu.returned_to != 0x47393a { return; }
  cpu.push(0x47393f); f_00474bd9(cpu); if cpu.returned_to != 0x47393f { return; }
  pc = 0x47393f; continue;
 }
 0x47393f => {
  { let v: u32 = 0x58u32; cpu.eax = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5ade38u32); cpu.esi = v; }
  cpu.push(0x473954); f_00474f51(cpu); if cpu.returned_to != 0x473954 { return; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32; cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x5ade38u32); cpu.esi = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  cpu.push(0x473972); f_00474c5f(cpu); if cpu.returned_to != 0x473972 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x47397a); cpu.call(t); if cpu.returned_to != 0x47397a { return; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5de71cu32, v); }
  cpu.push(0x47398b); f_0040bca0(cpu); if cpu.returned_to != 0x47398b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x47399d); f_00417ca0(cpu); if cpu.returned_to != 0x47399d { return; }
  pc = 0x47399d; continue;
 }
 0x47399d => {
  cpu.push(0x4739a2); f_00415156(cpu); if cpu.returned_to != 0x4739a2 { return; }
  cpu.push(0x4739a7); f_0041c122(cpu); if cpu.returned_to != 0x4739a7 { return; }
  if cpu.cf { pc = 0x4739b7; continue; }
  pc = 0x4739a9; continue;
 }
 0x4739a9 => {
  cpu.push(0x4739ae); f_0041c2ad(cpu); if cpu.returned_to != 0x4739ae { return; }
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x3u8); }
  if cpu.zf { cpu.poll(); pc = 0x47399d; continue; }
  pc = 0x4739b7; continue;
 }
 0x4739b7 => {
  { cpu.pushad(); }
  cpu.push(0x4739bd); f_004030f0(cpu); if cpu.returned_to != 0x4739bd { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004739bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4739bf); }
 let mut pc: u32 = 0x4739bf;
 loop { match pc {
 0x4739bf => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x2u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4739ca; continue; }
  pc = 0x4739c9; continue;
 }
 0x4739c9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4739ca => {
  cpu.push(0x4739cf); f_00473a6c(cpu); if cpu.returned_to != 0x4739cf { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ecx as u16); cpu.mul16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d00e5u32, v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x56d314u32), 0x5u8); }
  if cpu.zf { pc = 0x473a18; continue; }
  pc = 0x4739ec; continue;
 }
 0x4739ec => {
  { let v: u32 = 0x5ae0b4u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x5a997bu32; cpu.edi = v; }
  pc = 0x4739fa; continue;
 }
 0x4739fa => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x473a06; continue; }
  pc = 0x473a00; continue;
 }
 0x473a00 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x4739fa; continue;
 }
 0x473a06 => {
  { let v: u32 = cpu.r32(0x5ade04u32); cpu.esi = v; }
  pc = 0x473a0c; continue;
 }
 0x473a0c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x473a0c; continue; }
  pc = 0x473a16; continue;
 }
 0x473a16 => {
  pc = 0x473a47; continue;
 }
 0x473a18 => {
  { let v: u32 = 0x5a997bu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5ade04u32); cpu.esi = v; }
  pc = 0x473a23; continue;
 }
 0x473a23 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x473a23; continue; }
  pc = 0x473a2d; continue;
 }
 0x473a2d => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x5ae0b4u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  pc = 0x473a37; continue;
 }
 0x473a37 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x473a43; continue; }
  pc = 0x473a3d; continue;
 }
 0x473a3d => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x473a37; continue;
 }
 0x473a43 => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x473a47; continue;
 }
 0x473a47 => {
  { let v: u32 = 0x5a997bu32; cpu.esi = v; }
  cpu.push(0x473a51); f_0047502d(cpu); if cpu.returned_to != 0x473a51 { return; }
  { let v: u32 = 0xa0u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = 0x5a997bu32; cpu.esi = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  cpu.push(0x473a6b); f_00474e59(cpu); if cpu.returned_to != 0x473a6b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00473a6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x473a6c); }
 let mut pc: u32 = 0x473a6c;
 loop { match pc {
 0x473a6c => {
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x56d302u32), 0x0u8); }
  if !cpu.zf { pc = 0x473aa3; continue; }
  pc = 0x473a7a; continue;
 }
 0x473a7a => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x2u16); }
  if cpu.zf { pc = 0x473aa3; continue; }
  pc = 0x473a85; continue;
 }
 0x473a85 => {
  { cpu.sub8(cpu.r8(0x56d303u32), 0x0u8); }
  if !cpu.zf { pc = 0x473aa3; continue; }
  pc = 0x473a8e; continue;
 }
 0x473a8e => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x3u16); }
  if cpu.zf { pc = 0x473aa3; continue; }
  pc = 0x473a99; continue;
 }
 0x473a99 => {
  { cpu.sub8(cpu.r8(0x56d304u32), 0x0u8); }
  if !cpu.zf { pc = 0x473aa3; continue; }
  pc = 0x473aa2; continue;
 }
 0x473aa2 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x473aa3; continue;
 }
 0x473aa3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00473aa4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x473aa4); }
 let mut pc: u32 = 0x473aa4;
 loop { match pc {
 0x473aa4 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5820fcu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x56d302u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x473ac1; continue; }
  pc = 0x473abb; continue;
 }
 0x473abb => {
  cpu.push(0x473ac0); f_004737e8(cpu); if cpu.returned_to != 0x473ac0 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x473ac1 => {
  cpu.push(0x473ac6); f_004738d6(cpu); if cpu.returned_to != 0x473ac6 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00473ac7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x473ac7); }
 let mut pc: u32 = 0x473ac7;
 loop { match pc {
 0x473ac7 => {
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x2u16); }
  if !cpu.zf { pc = 0x473ad2; continue; }
  pc = 0x473ad1; continue;
 }
 0x473ad1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x473ad2 => {
  { cpu.pushad(); }
  { let v: u32 = 0x581bd9u32; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4d67f6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0xa0u32; cpu.w32(0x581be1u32, v); }
  { let v: u32 = 0x5ae08cu32; cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x4d67f6u32), 0x4u16); }
  if cpu.zf { pc = 0x473b0a; continue; }
  pc = 0x473afb; continue;
 }
 0x473afb => {
  { let v: u32 = 0x5ae088u32; cpu.edi = v; }
  { let v: u32 = 0x90u32; cpu.w32(0x581be1u32, v); }
  pc = 0x473b0a; continue;
 }
 0x473b0a => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x473b1d; continue; }
  pc = 0x473b10; continue;
 }
 0x473b10 => {
  cpu.push(0x473b15); f_00473a6c(cpu); if cpu.returned_to != 0x473b15 { return; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4d6bdcu32, v); }
  pc = 0x473b27; continue;
 }
 0x473b1d => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  pc = 0x473b27; continue;
 }
 0x473b27 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x4d6bdcu32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d00e5u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u32 = 0x30u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x581be1u32); cpu.ebx = v; }
  cpu.push(0x473b50); f_00474e59(cpu); if cpu.returned_to != 0x473b50 { return; }
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.ecx = v; }
  { let v: u32 = 0x575f47u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0xc0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x581be1u32); cpu.ebx = v; }
  cpu.push(0x473b6f); f_00474e59(cpu); if cpu.returned_to != 0x473b6f { return; }
  { let v: u32 = cpu.r32(0x581be1u32).wrapping_sub(0x10u32); cpu.w32(0x581be1u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.sub32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x473b0a; continue; }
  pc = 0x473b7e; continue;
 }
 0x473b7e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00473b80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x473b80); }
 let mut pc: u32 = 0x473b80;
 loop { match pc {
 0x473b80 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x594410u32, v); }
  { let v: u32 = cpu.r32(0x5cf961u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x56c725u32, v); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x473c29; continue; }
  pc = 0x473bb1; continue;
 }
 0x473bb1 => {
  { cpu.sub8(cpu.r8(0x5a23c8u32), 0x0u8); }
  if cpu.zf { pc = 0x473be6; continue; }
  pc = 0x473bba; continue;
 }
 0x473bba => {
  { let v: u32 = cpu.r32(0x5d0091u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x56c729u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x56c729u32, v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x56c721u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d2bd3u32, v); }
  pc = 0x473be6; continue;
 }
 0x473be6 => {
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x473beb; continue;
 }
 0x473beb => {
  { let v: u32 = cpu.r32(0x5d2bd3u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d2bd3u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x576e98u32, v); }
  cpu.push(0x473c09); f_004478cb(cpu); if cpu.returned_to != 0x473c09 { return; }
  if !cpu.cf { pc = 0x473c23; continue; }
  pc = 0x473c0b; continue;
 }
 0x473c0b => {
  { cpu.sub8(cpu.r8(0x576e98u32), 0x0u8); }
  if !cpu.zf { f_0041c653(cpu); return; }
  pc = 0x473c18; continue;
 }
 0x473c18 => {
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if cpu.zf { f_0041c653(cpu); return; }
  pc = 0x473c21; continue;
 }
 0x473c21 => {
  cpu.poll(); pc = 0x473beb; continue;
 }
 0x473c23 => {
  cpu.push(0x473c28); f_004275fd(cpu); if cpu.returned_to != 0x473c28 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x473c29 => {
  { let v: u32 = cpu.r32(0x5d2bd3u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d2bd3u32, v); }
  { let v: u32 = 0x576c1eu32; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5d2bd3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = 0x5d00f3u32; cpu.edi = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x473c5a; continue;
 }
 0x473c5a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x473c67; continue; }
  pc = 0x473c60; continue;
 }
 0x473c60 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x473c5a; continue; }
  pc = 0x473c67; continue;
 }
 0x473c67 => {
  { let v: u32 = 0x6d626c2eu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x594410u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x576c0cu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5d00f3u32; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x5d00f3u32; cpu.edx = v; }
  cpu.push(0x473c90); f_004275fd(cpu); if cpu.returned_to != 0x473c90 { return; }
  cpu.push(0x473c95); f_004209f6(cpu); if cpu.returned_to != 0x473c95 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00473c96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x473c96); }
 let mut pc: u32 = 0x473c96;
 loop { match pc {
 0x473c96 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x594410u32, v); }
  { let v: u32 = cpu.r32(0x5cf975u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x56c725u32, v); }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x473d3e; continue; }
  pc = 0x473cc7; continue;
 }
 0x473cc7 => {
  { cpu.sub8(cpu.r8(0x5a23c8u32), 0x0u8); }
  if cpu.zf { pc = 0x473cfc; continue; }
  pc = 0x473cd0; continue;
 }
 0x473cd0 => {
  { let v: u32 = cpu.r32(0x5d0091u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x56c729u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x56c729u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x56c721u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d2bd7u32, v); }
  pc = 0x473cfc; continue;
 }
 0x473cfc => {
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x473d01; continue;
 }
 0x473d01 => {
  { let v: u32 = cpu.r32(0x5d2bd7u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d2bd7u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x576e98u32, v); }
  cpu.push(0x473d1e); f_004478cb(cpu); if cpu.returned_to != 0x473d1e { return; }
  if !cpu.cf { pc = 0x473d38; continue; }
  pc = 0x473d20; continue;
 }
 0x473d20 => {
  { cpu.sub8(cpu.r8(0x576e98u32), 0x0u8); }
  if !cpu.zf { f_0041c653(cpu); return; }
  pc = 0x473d2d; continue;
 }
 0x473d2d => {
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if cpu.zf { f_0041c653(cpu); return; }
  pc = 0x473d36; continue;
 }
 0x473d36 => {
  cpu.poll(); pc = 0x473d01; continue;
 }
 0x473d38 => {
  cpu.push(0x473d3d); f_004275fd(cpu); if cpu.returned_to != 0x473d3d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x473d3e => {
  { let v: u32 = cpu.r32(0x5d2bd7u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d2bd7u32, v); }
  { let v: u32 = 0x576c5eu32; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5d2bd7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = 0x5d0100u32; cpu.edi = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x473d6f; continue;
 }
 0x473d6f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x473d7c; continue; }
  pc = 0x473d75; continue;
 }
 0x473d75 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x473d6f; continue; }
  pc = 0x473d7c; continue;
 }
 0x473d7c => {
  { let v: u32 = 0x6d626c2eu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x594410u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x576c0cu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5d0100u32; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x5d0100u32; cpu.edx = v; }
  cpu.push(0x473da5); f_004275fd(cpu); if cpu.returned_to != 0x473da5 { return; }
  cpu.push(0x473daa); f_004209f6(cpu); if cpu.returned_to != 0x473daa { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00473dab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x473dab); }
 let mut pc: u32 = 0x473dab;
 loop { match pc {
 0x473dab => {
  { let v: u8 = cpu.r8(0x4e1fa1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x140u16; cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x4d6ba8u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6856u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d8u32, v); }
  cpu.push(0x473de3); f_00427e4a(cpu); if cpu.returned_to != 0x473de3 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6932u32, v); }
  cpu.push(0x473def); f_004175d9(cpu); if cpu.returned_to != 0x473def { return; }
  { let v: u16 = 0x140u16; cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x4d6ba8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b3u32, v); }
  cpu.push(0x473e14); f_00417e23(cpu); if cpu.returned_to != 0x473e14 { return; }
  { let v: u32 = cpu.r32(0x4b94d0u32); cpu.ecx = v; }
  cpu.push(0x473e1f); f_00474aaf(cpu); if cpu.returned_to != 0x473e1f { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1fa1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00473e2a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x473e2a); } r_00473e2a(cpu, 0x473e2a); }
pub fn f_00474000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x474000); } r_00473e2a(cpu, 0x474000); }
fn r_00473e2a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x473e2a => {
  { let v: u32 = cpu.eax; cpu.w32(0x5d2bdbu32, v); }
  { let v: u8 = cpu.r8(0x4e1fa1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x473e3a); f_004209f6(cpu); if cpu.returned_to != 0x473e3a { return; }
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x473e44); f_00417dab(cpu); if cpu.returned_to != 0x473e44 { return; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x473e53); f_00401000(cpu); if cpu.returned_to != 0x473e53 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  cpu.push(0x473e5b); f_004010e0(cpu); if cpu.returned_to != 0x473e5b { return; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x5978c6u32), 0x1u8); }
  if !cpu.zf { pc = 0x473e6f; continue; }
  pc = 0x473e65; continue;
 }
 0x473e65 => {
  cpu.push(0x473e6a); f_0047429a(cpu); if cpu.returned_to != 0x473e6a { return; }
  pc = 0x47412f; continue;
 }
 0x473e6f => {
  { cpu.sub8(cpu.r8(0x5d00cdu32), 0x0u8); }
  if cpu.zf { pc = 0x47412f; continue; }
  pc = 0x473e7c; continue;
 }
 0x473e7c => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x473e89); f_00402d70(cpu); if cpu.returned_to != 0x473e89 { return; }
  cpu.push(0x473e8e); f_00405320(cpu); if cpu.returned_to != 0x473e8e { return; }
  { cpu.popad(); }
  cpu.push(0x473e94); f_0047ad0f(cpu); if cpu.returned_to != 0x473e94 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x56d395u32, v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.push(0x473ea5); f_0047ae6f(cpu); if cpu.returned_to != 0x473ea5 { return; }
  { let v: u32 = 0x25u32; cpu.eax = v; }
  cpu.push(0x473eaf); f_0047ae6f(cpu); if cpu.returned_to != 0x473eaf { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x56d395u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d8u32, v); }
  { let v: u16 = 0x140u16; cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x4d6ba8u32, v); }
  cpu.push(0x473edf); f_00427e4a(cpu); if cpu.returned_to != 0x473edf { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6932u32, v); }
  cpu.push(0x473eeb); f_004175d9(cpu); if cpu.returned_to != 0x473eeb { return; }
  { let v: u16 = 0x140u16; cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x4d6ba8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b3u32, v); }
  cpu.push(0x473f10); f_00417e23(cpu); if cpu.returned_to != 0x473f10 { return; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x473f20; continue; }
  pc = 0x473f19; continue;
 }
 0x473f19 => {
  cpu.push(0x473f1e); f_00474136(cpu); if cpu.returned_to != 0x473f1e { return; }
  pc = 0x473f7a; continue;
 }
 0x473f20 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5cf94du32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x473f43; continue;
 }
 0x473f43 => {
  { let v: u32 = cpu.r32(0x5d2bcfu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d2bcfu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x576e98u32, v); }
  cpu.push(0x473f60); f_004478cb(cpu); if cpu.returned_to != 0x473f60 { return; }
  if !cpu.cf { pc = 0x473f7a; continue; }
  pc = 0x473f62; continue;
 }
 0x473f62 => {
  { cpu.sub8(cpu.r8(0x576e98u32), 0x0u8); }
  if !cpu.zf { f_0041c653(cpu); return; }
  pc = 0x473f6f; continue;
 }
 0x473f6f => {
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if cpu.zf { f_0041c653(cpu); return; }
  pc = 0x473f78; continue;
 }
 0x473f78 => {
  cpu.poll(); pc = 0x473f43; continue;
 }
 0x473f7a => {
  cpu.push(0x473f7f); f_004275fd(cpu); if cpu.returned_to != 0x473f7f { return; }
  cpu.push(0x473f84); f_00416db1(cpu); if cpu.returned_to != 0x473f84 { return; }
  { let v: u32 = 0x78u32; cpu.eax = v; }
  { let v: u32 = 0x84u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x31u32; cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x5d2bdbu32), 0x0u32); }
  if cpu.zf { pc = 0x473fb1; continue; }
  pc = 0x473f9e; continue;
 }
 0x473f9e => {
  { let v: u32 = 0x4efu32; cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x5d2bdbu32), 0x2u32); }
  if cpu.zf { pc = 0x473fb1; continue; }
  pc = 0x473fac; continue;
 }
 0x473fac => {
  { let v: u32 = 0x4f0u32; cpu.ecx = v; }
  pc = 0x473fb1; continue;
 }
 0x473fb1 => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x473fb7); cpu.call(t); if cpu.returned_to != 0x473fb7 { return; }
  cpu.push(0x473fbc); f_00416dc1(cpu); if cpu.returned_to != 0x473fbc { return; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x474005; continue; }
  pc = 0x473fc5; continue;
 }
 0x473fc5 => {
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x576464u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x576468u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x57646cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x576470u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x57647cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5820f1u32, v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x58c644u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x474005; continue; }
  pc = 0x473ffe; continue;
 }
 0x473ffe => {
  { let v: u32 = 0x5d0158u32; cpu.esi = v; }
  pc = 0x47401b; continue;
 }
 0x474000 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47401b; continue;
 }
 0x474005 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = 0x4ae77cu32.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x47401b; continue;
 }
 0x47401b => {
  { let v: u32 = cpu.esi; cpu.w32(0x4d6bd4u32, v); }
  { let v: u32 = 0x5d00ceu32; cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.esi = v; }
  cpu.push(0x474038); f_00474f51(cpu); if cpu.returned_to != 0x474038 { return; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32; cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4d6bd4u32); cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x474063; continue; }
  pc = 0x47405e; continue;
 }
 0x47405e => {
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  pc = 0x474063; continue;
 }
 0x474063 => {
  cpu.push(0x474068); f_00474c5f(cpu); if cpu.returned_to != 0x474068 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x47407e; continue; }
  pc = 0x474078; continue;
 }
 0x474078 => {
  { let v: u8 = cpu.r8(0x5757e5u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x47407e; continue;
 }
 0x47407e => {
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5ae040u32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x4740c9; continue; }
  pc = 0x4740a0; continue;
 }
 0x4740a0 => {
  { let v: u32 = cpu.r32(0x5ade40u32); cpu.esi = v; }
  cpu.push(0x4740ab); f_00474f51(cpu); if cpu.returned_to != 0x4740ab { return; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32; cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x18u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5ade40u32); cpu.esi = v; }
  cpu.push(0x4740c9); f_00474c5f(cpu); if cpu.returned_to != 0x4740c9 { return; }
  pc = 0x4740c9; continue;
 }
 0x4740c9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x4740d6); f_00402d70(cpu); if cpu.returned_to != 0x4740d6 { return; }
  cpu.push(0x4740db); f_00405320(cpu); if cpu.returned_to != 0x4740db { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x4740e4); cpu.call(t); if cpu.returned_to != 0x4740e4 { return; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5de71cu32, v); }
  cpu.push(0x4740f5); f_0040bca0(cpu); if cpu.returned_to != 0x4740f5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x474107); f_00417ca0(cpu); if cpu.returned_to != 0x474107 { return; }
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u32 = 0x588860u32; cpu.edi = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  pc = 0x474116; continue;
 }
 0x474116 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x474116; continue; }
  pc = 0x474123; continue;
 }
 0x474123 => {
  cpu.push(0x474128); f_00416da6(cpu); if cpu.returned_to != 0x474128 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x575531u32, v); }
  pc = 0x47412f; continue;
 }
 0x47412f => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1fa1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474136(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x474136); }
 let mut pc: u32 = 0x474136;
 loop { match pc {
 0x474136 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x4ae7eau32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x594410u32, v); }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  pc = 0x474157; continue;
 }
 0x474157 => {
  { let v: u32 = cpu.r32(0x5d2bcfu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d2bcfu32, v); }
  { let v: u32 = 0x576c9eu32; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4e1559u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5d2bcfu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = 0x576c11u32; cpu.edi = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x474189; continue;
 }
 0x474189 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x474196; continue; }
  pc = 0x47418f; continue;
 }
 0x47418f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x474189; continue; }
  pc = 0x474196; continue;
 }
 0x474196 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x6d626c2eu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x594410u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x576c0cu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x576c11u32; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x576c11u32; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x576e98u32, v); }
  cpu.push(0x4741c8); f_004478cb(cpu); if cpu.returned_to != 0x4741c8 { return; }
  if !cpu.cf { pc = 0x4741e5; continue; }
  pc = 0x4741ca; continue;
 }
 0x4741ca => {
  { cpu.sub8(cpu.r8(0x576e98u32), 0x0u8); }
  if !cpu.zf { f_0041c653(cpu); return; }
  pc = 0x4741d7; continue;
 }
 0x4741d7 => {
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if cpu.zf { f_0041c653(cpu); return; }
  pc = 0x4741e0; continue;
 }
 0x4741e0 => {
  cpu.poll(); pc = 0x474157; continue;
 }
 0x4741e5 => {
  { let v: u32 = 0x576c11u32; cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004741eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4741eb); }
 let mut pc: u32 = 0x4741eb;
 loop { match pc {
 0x4741eb => {
  { cpu.sub8(cpu.r8(0x5791c8u32), 0x0u8); }
  if !cpu.zf { pc = 0x474201; continue; }
  pc = 0x4741f4; continue;
 }
 0x4741f4 => {
  { cpu.sub8(cpu.r8(0x56d2d7u32), 0x0u8); }
  if cpu.zf { pc = 0x474299; continue; }
  pc = 0x474201; continue;
 }
 0x474201 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x56d2e1u32)); cpu.w8(0x56d2e1u32, v); }
  { cpu.sub8(cpu.r8(0x56d2e1u32), 0x3u8); }
  if !cpu.zf { pc = 0x474299; continue; }
  pc = 0x474214; continue;
 }
 0x474214 => {
  { let v: u8 = 0x0u8; cpu.w8(0x56d2e1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x474228); f_00402d70(cpu); if cpu.returned_to != 0x474228 { return; }
  cpu.push(0x47422d); f_00405320(cpu); if cpu.returned_to != 0x47422d { return; }
  { cpu.popad(); }
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x474238); f_00417dab(cpu); if cpu.returned_to != 0x474238 { return; }
  cpu.push(0x47423d); f_00417e23(cpu); if cpu.returned_to != 0x47423d { return; }
  { let v: u32 = 0x56d2c2u32; cpu.edx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5824d5u32, v); }
  cpu.push(0x47424e); f_004275fd(cpu); if cpu.returned_to != 0x47424e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5824d5u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x47425d); cpu.call(t); if cpu.returned_to != 0x47425d { return; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5de71cu32, v); }
  cpu.push(0x47426e); f_0040bca0(cpu); if cpu.returned_to != 0x47426e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x474280); f_00417ca0(cpu); if cpu.returned_to != 0x474280 { return; }
  { let v: u32 = 0x2000u32; cpu.ecx = v; }
  cpu.push(0x47428a); f_00414b44(cpu); if cpu.returned_to != 0x47428a { return; }
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x474294); f_00417dab(cpu); if cpu.returned_to != 0x474294 { return; }
  cpu.push(0x474299); f_00417e23(cpu); if cpu.returned_to != 0x474299 { return; }
  pc = 0x474299; continue;
 }
 0x474299 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047429a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47429a); }
 let mut pc: u32 = 0x47429a;
 loop { match pc {
 0x47429a => {
  { let v = cpu.r16(0x4d6ba8u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5820fbu32, v); }
  { let v: u8 = cpu.r8(0x4e1fa1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x5824e3u32), 0x0u8); }
  if !cpu.zf { pc = 0x47430a; continue; }
  pc = 0x4742b7; continue;
 }
 0x4742b7 => {
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x4742c1); f_00417dab(cpu); if cpu.returned_to != 0x4742c1 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x4742ce); f_00402d70(cpu); if cpu.returned_to != 0x4742ce { return; }
  cpu.push(0x4742d3); f_00405320(cpu); if cpu.returned_to != 0x4742d3 { return; }
  { cpu.popad(); }
  cpu.push(0x4742d9); f_00417e23(cpu); if cpu.returned_to != 0x4742d9 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6932u32, v); }
  cpu.push(0x4742e5); f_004175d9(cpu); if cpu.returned_to != 0x4742e5 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x4742f2); f_00402d70(cpu); if cpu.returned_to != 0x4742f2 { return; }
  cpu.push(0x4742f7); f_00405320(cpu); if cpu.returned_to != 0x4742f7 { return; }
  { cpu.popad(); }
  { let v: u16 = 0x140u16; cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x4d6ba8u32, v); }
  pc = 0x47430a; continue;
 }
 0x47430a => {
  { let v: u8 = 0x1u8; cpu.w8(0x5824d5u32, v); }
  { let v: u32 = 0x5824cau32; cpu.edx = v; }
  cpu.push(0x47431b); f_004275fd(cpu); if cpu.returned_to != 0x47431b { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5824d5u32, v); }
  { cpu.sub8(cpu.r8(0x5824e0u32), 0x0u8); }
  if !cpu.zf { pc = 0x47434c; continue; }
  pc = 0x47432b; continue;
 }
 0x47432b => {
  cpu.push(0x474330); f_00416db1(cpu); if cpu.returned_to != 0x474330 { return; }
  { let v: u32 = 0x78u32; cpu.eax = v; }
  { let v: u32 = 0x84u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x31u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x474347); cpu.call(t); if cpu.returned_to != 0x474347 { return; }
  cpu.push(0x47434c); f_00416dc1(cpu); if cpu.returned_to != 0x47434c { return; }
  pc = 0x47434c; continue;
 }
 0x47434c => {
  { let v: u8 = 0x1u8; cpu.w8(0x5824d6u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x47435b); cpu.call(t); if cpu.returned_to != 0x47435b { return; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5de71cu32, v); }
  cpu.push(0x47436c); f_0040bca0(cpu); if cpu.returned_to != 0x47436c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x5824e0u32), 0x0u8); }
  if !cpu.zf { pc = 0x4743aa; continue; }
  pc = 0x474379; continue;
 }
 0x474379 => {
  { cpu.sub8(cpu.r8(0x5824e3u32), 0x0u8); }
  if !cpu.zf { pc = 0x4743aa; continue; }
  pc = 0x474382; continue;
 }
 0x474382 => {
  cpu.push(0x474387); f_00416da6(cpu); if cpu.returned_to != 0x474387 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x575531u32, v); }
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x47439c); f_00417ca0(cpu); if cpu.returned_to != 0x47439c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1fa1u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6ba8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4743aa => {
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { let v: u32 = 0x55b6b8u32; cpu.edi = v; }
  pc = 0x4743b9; continue;
 }
 0x4743b9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x4743b9; continue; }
  pc = 0x4743c6; continue;
 }
 0x4743c6 => {
  cpu.push(0x4743cb); f_00417e3e(cpu); if cpu.returned_to != 0x4743cb { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1fa1u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6ba8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047449e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47449e); } r_0047449e(cpu, 0x47449e); }
pub fn f_004744cc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4744cc); } r_0047449e(cpu, 0x4744cc); }
fn r_0047449e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x47449e => {
  { let v: u32 = 0x1u32; cpu.w32(0x581b4du32, v); }
  { let v: u8 = cpu.r8(0x4e1fa1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x140u16; cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x4d6ba8u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6856u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d8u32, v); }
  cpu.push(0x4744e0); f_00427e4a(cpu); if cpu.returned_to != 0x4744e0 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6932u32, v); }
  cpu.push(0x4744ec); f_004175d9(cpu); if cpu.returned_to != 0x4744ec { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x581b4du32, v); }
  { let v: u16 = 0x140u16; cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x4d6ba8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b3u32, v); }
  cpu.push(0x47451b); f_00417e23(cpu); if cpu.returned_to != 0x47451b { return; }
  { let v: u32 = cpu.r32(0x4b94d0u32); cpu.ecx = v; }
  cpu.push(0x474526); f_00474aaf(cpu); if cpu.returned_to != 0x474526 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4b94c4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4b94c8u32); cpu.ebx = v; }
  cpu.push(0x47453a); f_00430b3f(cpu); if cpu.returned_to != 0x47453a { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4b94ceu32, v); }
  cpu.push(0x474545); f_00473b80(cpu); if cpu.returned_to != 0x474545 { return; }
  { let v: u32 = cpu.r32(0x4b94c4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4b94c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4b94d0u32); cpu.ecx = v; }
  cpu.push(0x47455b); f_00430abb(cpu); if cpu.returned_to != 0x47455b { return; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = 0x4ae77cu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x5d00ceu32; cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.r32(0x5ade38u32); cpu.esi = v; }
  cpu.push(0x474588); f_00474f51(cpu); if cpu.returned_to != 0x474588 { return; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32; cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x5ade38u32); cpu.esi = v; }
  cpu.push(0x4745a5); f_00474c5f(cpu); if cpu.returned_to != 0x4745a5 { return; }
  cpu.push(0x4745aa); f_00474b53(cpu); if cpu.returned_to != 0x4745aa { return; }
  cpu.push(0x4745af); f_00474b92(cpu); if cpu.returned_to != 0x4745af { return; }
  { let v: u32 = 0xe0u32; cpu.eax = v; }
  { let v: u32 = 0x46u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4b94d0u32); cpu.ecx = v; }
  cpu.push(0x4745c6); f_00474a20(cpu); if cpu.returned_to != 0x4745c6 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x4745ce); cpu.call(t); if cpu.returned_to != 0x4745ce { return; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5de71cu32, v); }
  cpu.push(0x4745df); f_0040bca0(cpu); if cpu.returned_to != 0x4745df { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.push(0x4745e8); f_004030d0(cpu); if cpu.returned_to != 0x4745e8 { return; }
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x4745f6); f_00417ca0(cpu); if cpu.returned_to != 0x4745f6 { return; }
  { cpu.sub8(cpu.r8(0x56d2d7u32), 0x0u8); }
  if cpu.zf { pc = 0x47460b; continue; }
  pc = 0x4745ff; continue;
 }
 0x4744cc => {
  cpu.unimplemented(0x4744cc, "Salc");
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xffffffc7u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffffc7u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4d67d8u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x4744e0); f_00427e4a(cpu); if cpu.returned_to != 0x4744e0 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6932u32, v); }
  cpu.push(0x4744ec); f_004175d9(cpu); if cpu.returned_to != 0x4744ec { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x581b4du32, v); }
  { let v: u16 = 0x140u16; cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x4d6ba8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b3u32, v); }
  cpu.push(0x47451b); f_00417e23(cpu); if cpu.returned_to != 0x47451b { return; }
  { let v: u32 = cpu.r32(0x4b94d0u32); cpu.ecx = v; }
  cpu.push(0x474526); f_00474aaf(cpu); if cpu.returned_to != 0x474526 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4b94c4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4b94c8u32); cpu.ebx = v; }
  cpu.push(0x47453a); f_00430b3f(cpu); if cpu.returned_to != 0x47453a { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4b94ceu32, v); }
  cpu.push(0x474545); f_00473b80(cpu); if cpu.returned_to != 0x474545 { return; }
  { let v: u32 = cpu.r32(0x4b94c4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4b94c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4b94d0u32); cpu.ecx = v; }
  cpu.push(0x47455b); f_00430abb(cpu); if cpu.returned_to != 0x47455b { return; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = 0x4ae77cu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x5d00ceu32; cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.r32(0x5ade38u32); cpu.esi = v; }
  cpu.push(0x474588); f_00474f51(cpu); if cpu.returned_to != 0x474588 { return; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32; cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x5ade38u32); cpu.esi = v; }
  cpu.push(0x4745a5); f_00474c5f(cpu); if cpu.returned_to != 0x4745a5 { return; }
  cpu.push(0x4745aa); f_00474b53(cpu); if cpu.returned_to != 0x4745aa { return; }
  cpu.push(0x4745af); f_00474b92(cpu); if cpu.returned_to != 0x4745af { return; }
  { let v: u32 = 0xe0u32; cpu.eax = v; }
  { let v: u32 = 0x46u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4b94d0u32); cpu.ecx = v; }
  cpu.push(0x4745c6); f_00474a20(cpu); if cpu.returned_to != 0x4745c6 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x4745ce); cpu.call(t); if cpu.returned_to != 0x4745ce { return; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5de71cu32, v); }
  cpu.push(0x4745df); f_0040bca0(cpu); if cpu.returned_to != 0x4745df { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  cpu.push(0x4745e8); f_004030d0(cpu); if cpu.returned_to != 0x4745e8 { return; }
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x4745f6); f_00417ca0(cpu); if cpu.returned_to != 0x4745f6 { return; }
  { cpu.sub8(cpu.r8(0x56d2d7u32), 0x0u8); }
  if cpu.zf { pc = 0x47460b; continue; }
  pc = 0x4745ff; continue;
 }
 0x4745ff => {
  { let v: u32 = 0x2bcu32; cpu.ecx = v; }
  cpu.push(0x474609); f_00414b44(cpu); if cpu.returned_to != 0x474609 { return; }
  pc = 0x47464c; continue;
 }
 0x47460b => {
  cpu.push(0x474610); f_00415156(cpu); if cpu.returned_to != 0x474610 { return; }
  cpu.push(0x474615); f_0041c122(cpu); if cpu.returned_to != 0x474615 { return; }
  if cpu.cf { pc = 0x474625; continue; }
  pc = 0x474617; continue;
 }
 0x474617 => {
  cpu.push(0x47461c); f_0041c2ad(cpu); if cpu.returned_to != 0x47461c { return; }
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x3u8); }
  if cpu.zf { cpu.poll(); pc = 0x47460b; continue; }
  pc = 0x474625; continue;
 }
 0x474625 => {
  cpu.push(0x47462a); f_00470e09(cpu); if cpu.returned_to != 0x47462a { return; }
  { let v: u32 = cpu.r32(0x4b94d0u32); cpu.ecx = v; }
  cpu.push(0x474635); f_00474aaf(cpu); if cpu.returned_to != 0x474635 { return; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4b94ceu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x575f24u32); cpu.edx = v; }
  cpu.push(0x47464c); f_00430eae(cpu); if cpu.returned_to != 0x47464c { return; }
  pc = 0x47464c; continue;
 }
 0x47464c => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47465b); f_00401000(cpu); if cpu.returned_to != 0x47465b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x474664); f_004030f0(cpu); if cpu.returned_to != 0x474664 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1fa1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047466d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47466d); }
 let mut pc: u32 = 0x47466d;
 loop { match pc {
 0x47466d => {
  { let v: u8 = cpu.r8(0x4e1fa1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x140u16; cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x4d6ba8u32, v); }
  cpu.push(0x47468a); f_00427e4a(cpu); if cpu.returned_to != 0x47468a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x4d6932u32, v); }
  cpu.push(0x474696); f_004175d9(cpu); if cpu.returned_to != 0x474696 { return; }
  { let v: u16 = 0x140u16; cpu.w16(0x4d6ba4u32, v); }
  { let v: u16 = 0xc8u16; cpu.w16(0x4d6ba8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x4746b5); f_00402d70(cpu); if cpu.returned_to != 0x4746b5 { return; }
  cpu.push(0x4746ba); f_00405320(cpu); if cpu.returned_to != 0x4746ba { return; }
  { cpu.popad(); }
  { let v: u8 = cpu.r8(0x4e1e4bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1e4bu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4746cd); f_00417e23(cpu); if cpu.returned_to != 0x4746cd { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x56d314u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d010fu32, v); }
  { let v: u16 = cpu.r16(0x58667bu32.wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x2u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d0113u32, v); }
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x58668bu32.wrapping_add(cpu.ecx.wrapping_mul(2)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d012bu32, v); }
  { let v: u32 = 0x5d010fu32; cpu.esi = v; }
  cpu.push(0x47470f); f_0041b447(cpu); if cpu.returned_to != 0x47470f { return; }
  cpu.push(0x474714); f_00473c96(cpu); if cpu.returned_to != 0x474714 { return; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4d67dau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xf0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = 0x4ae77cu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x5d00ceu32; cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.r32(0x5ade3cu32); cpu.esi = v; }
  cpu.push(0x474741); f_00474f51(cpu); if cpu.returned_to != 0x474741 { return; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xa0u32; cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x5ade3cu32); cpu.esi = v; }
  cpu.push(0x47475e); f_00474c5f(cpu); if cpu.returned_to != 0x47475e { return; }
  cpu.push(0x474763); f_00474ace(cpu); if cpu.returned_to != 0x474763 { return; }
  cpu.push(0x474768); f_00474b0e(cpu); if cpu.returned_to != 0x474768 { return; }
  { let v: u32 = 0xe0u32; cpu.eax = v; }
  { let v: u32 = 0x46u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4b94d0u32); cpu.ecx = v; }
  cpu.push(0x47477f); f_00474a20(cpu); if cpu.returned_to != 0x47477f { return; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x4747e9; continue; }
  pc = 0x474788; continue;
 }
 0x474788 => {
  { cpu.sub8(cpu.r8(0x56d2d7u32), 0x0u8); }
  if !cpu.zf { pc = 0x4747e9; continue; }
  pc = 0x474791; continue;
 }
 0x474791 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = 0xa3u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x199bu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4747a8); cpu.call(t); if cpu.returned_to != 0x4747a8 { return; }
  { let v: u32 = 0x7au32; cpu.eax = v; }
  { let v: u32 = 0xa3u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x199cu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4747bf); cpu.call(t); if cpu.returned_to != 0x4747bf { return; }
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { let v: u32 = 0xa3u32; cpu.ebx = v; }
  { let v: u32 = 0x199fu32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4747d4); cpu.call(t); if cpu.returned_to != 0x4747d4 { return; }
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0xa3u32; cpu.ebx = v; }
  { let v: u32 = 0x19a0u32; cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x4747e9); cpu.call(t); if cpu.returned_to != 0x4747e9 { return; }
  pc = 0x4747e9; continue;
 }
 0x4747e9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x4747f1); cpu.call(t); if cpu.returned_to != 0x4747f1 { return; }
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5de71cu32, v); }
  cpu.push(0x474802); f_0040bca0(cpu); if cpu.returned_to != 0x474802 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.popad(); }
  { cpu.sub8(cpu.r8(0x56d2d7u32), 0x0u8); }
  if !cpu.zf { pc = 0x474838; continue; }
  pc = 0x47480f; continue;
 }
 0x47480f => {
  { let v: u16 = 0xaeeu16; cpu.w16(0x4d6856u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d6u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d67d8u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x474837); f_00402d60(cpu); if cpu.returned_to != 0x474837 { return; }
  { cpu.popad(); }
  pc = 0x474838; continue;
 }
 0x474838 => {
  { let v: u32 = 0x55b9b8u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x474846); f_00417ca0(cpu); if cpu.returned_to != 0x474846 { return; }
  cpu.push(0x47484b); f_004030d0(cpu); if cpu.returned_to != 0x47484b { return; }
  { cpu.sub8(cpu.r8(0x56d2d7u32), 0x0u8); }
  if cpu.zf { pc = 0x474863; continue; }
  pc = 0x474854; continue;
 }
 0x474854 => {
  { let v: u32 = 0x2bcu32; cpu.ecx = v; }
  cpu.push(0x47485e); f_00414b44(cpu); if cpu.returned_to != 0x47485e { return; }
  pc = 0x4749af; continue;
 }
 0x474863 => {
  cpu.push(0x474868); f_00415156(cpu); if cpu.returned_to != 0x474868 { return; }
  cpu.push(0x47486d); f_0041c122(cpu); if cpu.returned_to != 0x47486d { return; }
  { cpu.sub8((cpu.eax as u8), 0x71u8); }
  if cpu.zf { pc = 0x4749af; continue; }
  pc = 0x474875; continue;
 }
 0x474875 => {
  { cpu.sub8((cpu.eax as u8), 0x51u8); }
  if cpu.zf { pc = 0x4749af; continue; }
  pc = 0x47487d; continue;
 }
 0x47487d => {
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if cpu.zf { pc = 0x4748bc; continue; }
  pc = 0x474886; continue;
 }
 0x474886 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x474893); f_00402d70(cpu); if cpu.returned_to != 0x474893 { return; }
  cpu.push(0x474898); f_00405320(cpu); if cpu.returned_to != 0x474898 { return; }
  { cpu.popad(); }
  pc = 0x474899; continue;
 }
 0x474899 => {
  cpu.push(0x47489e); f_00415156(cpu); if cpu.returned_to != 0x47489e { return; }
  cpu.push(0x4748a3); f_0041c122(cpu); if cpu.returned_to != 0x4748a3 { return; }
  if cpu.cf { pc = 0x4749af; continue; }
  pc = 0x4748a9; continue;
 }
 0x4748a9 => {
  cpu.push(0x4748ae); f_0041c2ad(cpu); if cpu.returned_to != 0x4748ae { return; }
  { cpu.and8(cpu.r8(0x4e15a5u32), 0x3u8); }
  if cpu.zf { cpu.poll(); pc = 0x474899; continue; }
  pc = 0x4748b7; continue;
 }
 0x4748b7 => {
  pc = 0x4749af; continue;
 }
 0x4748bc => {
  { cpu.sub8((cpu.eax as u8), 0x72u8); }
  if cpu.zf { pc = 0x47495c; continue; }
  pc = 0x4748c4; continue;
 }
 0x4748c4 => {
  { cpu.sub8((cpu.eax as u8), 0x52u8); }
  if cpu.zf { pc = 0x47495c; continue; }
  pc = 0x4748cc; continue;
 }
 0x4748cc => {
  { let v: u8 = 0x0u8; cpu.w8(0x5d010eu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5d010du32, v); }
  cpu.push(0x4748df); f_0041b488(cpu); if cpu.returned_to != 0x4748df { return; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = 0xa3u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5d010eu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x199bu32); cpu.ecx = v; }
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x474903); cpu.call(t); if cpu.returned_to != 0x474903 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x78u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x47490e); cpu.call(t); if cpu.returned_to != 0x47490e { return; }
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { let v: u32 = 0xa3u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5d010du32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x199fu32); cpu.ecx = v; }
  { cpu.pushad(); }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x474932); cpu.call(t); if cpu.returned_to != 0x474932 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x78u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x47493d); cpu.call(t); if cpu.returned_to != 0x47493d { return; }
  { cpu.sub8(cpu.r8(0x5d010eu32), 0x3u8); }
  if cpu.zf { pc = 0x4749af; continue; }
  pc = 0x474946; continue;
 }
 0x474946 => {
  { cpu.sub8(cpu.r8(0x5d010du32), 0x3u8); }
  if cpu.zf { pc = 0x47495c; continue; }
  pc = 0x47494f; continue;
 }
 0x47494f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  let t = cpu.r32(0x4d6f84u32); cpu.push(0x474957); cpu.call(t); if cpu.returned_to != 0x474957 { return; }
  cpu.poll(); pc = 0x474863; continue;
 }
 0x47495c => {
  cpu.push(0x474961); f_00470e09(cpu); if cpu.returned_to != 0x474961 { return; }
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x47496b); f_00417dab(cpu); if cpu.returned_to != 0x47496b { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x47497a); f_00401000(cpu); if cpu.returned_to != 0x47497a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x47498b); f_00402d70(cpu); if cpu.returned_to != 0x47498b { return; }
  cpu.push(0x474990); f_00405320(cpu); if cpu.returned_to != 0x474990 { return; }
  { cpu.popad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6856u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e4bu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4749a6); f_004030f0(cpu); if cpu.returned_to != 0x4749a6 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1fa1u32, v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4749af => {
  cpu.push(0x4749b4); f_00470e09(cpu); if cpu.returned_to != 0x4749b4 { return; }
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x4749be); f_00417dab(cpu); if cpu.returned_to != 0x4749be { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { cpu.pushad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4749cd); f_00401000(cpu); if cpu.returned_to != 0x4749cd { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4e15b1u32, v); }
  { cpu.pushad(); }
  cpu.push(0x4749dd); f_00402d70(cpu); if cpu.returned_to != 0x4749dd { return; }
  cpu.push(0x4749e2); f_00405320(cpu); if cpu.returned_to != 0x4749e2 { return; }
  { cpu.popad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6856u32, v); }
  cpu.push(0x4749f1); f_004030f0(cpu); if cpu.returned_to != 0x4749f1 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1e4bu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1fa1u32, v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474a00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x474a00); }
 let mut pc: u32 = 0x474a00;
 loop { match pc {
 0x474a00 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5d010eu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474a08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x474a08); }
 let mut pc: u32 = 0x474a08;
 loop { match pc {
 0x474a08 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5d010du32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474a10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x474a10); }
 let mut pc: u32 = 0x474a10;
 loop { match pc {
 0x474a10 => {
  { let v: u8 = 0x3u8; cpu.w8(0x5d010eu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474a18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x474a18); }
 let mut pc: u32 = 0x474a18;
 loop { match pc {
 0x474a18 => {
  { let v: u8 = 0x3u8; cpu.w8(0x5d010du32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474a20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x474a20); }
 let mut pc: u32 = 0x474a20;
 loop { match pc {
 0x474a20 => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x474a28); f_00474aaf(cpu); if cpu.returned_to != 0x474a28 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d00ebu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5d00efu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5d00e7u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5d00e7u32); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x474a4e); f_0041b6f1(cpu); if cpu.returned_to != 0x474a4e { return; }
  { let v: u32 = 0x54a7d0u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x4e1fe6u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5d00ebu32); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  cpu.push(0x474a6f); f_0041b6f1(cpu); if cpu.returned_to != 0x474a6f { return; }
  { let v: u8 = cpu.r8(0x4e1fe6u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x4e1fe7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5d00efu32); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  cpu.push(0x474a93); f_0041b6f1(cpu); if cpu.returned_to != 0x474a93 { return; }
  { let v: u16 = cpu.r16(0x4e1fe6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x54a7d0u32; cpu.esi = v; }
  cpu.push(0x474aad); f_00474d56(cpu); if cpu.returned_to != 0x474aad { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474aaf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x474aaf); }
 let mut pc: u32 = 0x474aaf;
 loop { match pc {
 0x474aaf => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xe10u32; cpu.ebx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3cu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474ace(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x474ace); }
 let mut pc: u32 = 0x474ace;
 loop { match pc {
 0x474ace => {
  { let v: u32 = cpu.r32(0x5ade34u32); cpu.edi = v; }
  pc = 0x474ad4; continue;
 }
 0x474ad4 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.sf { pc = 0x474aee; continue; }
  pc = 0x474ae4; continue;
 }
 0x474ae4 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x474aec); f_00474c5f(cpu); if cpu.returned_to != 0x474aec { return; }
  pc = 0x474b04; continue;
 }
 0x474aee => {
  cpu.push(0x474af3); f_00474d56(cpu); if cpu.returned_to != 0x474af3 { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xd4u32; cpu.eax = v; }
  { let v: u32 = 0x5aa13au32; cpu.esi = v; }
  cpu.push(0x474b03); f_00474d56(cpu); if cpu.returned_to != 0x474b03 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x474b04; continue;
 }
 0x474b04 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if !cpu.zf { cpu.poll(); pc = 0x474ad4; continue; }
  pc = 0x474b0d; continue;
 }
 0x474b0d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474b0e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x474b0e); }
 let mut pc: u32 = 0x474b0e;
 loop { match pc {
 0x474b0e => {
  { let v: u32 = 0x5a90a5u32; cpu.esi = v; }
  pc = 0x474b13; continue;
 }
 0x474b13 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x474b52; continue; }
  pc = 0x474b20; continue;
 }
 0x474b20 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x474b32); f_0041b6f1(cpu); if cpu.returned_to != 0x474b32 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x4e1fe6u32; cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if !cpu.sf { pc = 0x474b47; continue; }
  pc = 0x474b3e; continue;
 }
 0x474b3e => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.push(0x474b45); f_00474c5f(cpu); if cpu.returned_to != 0x474b45 { return; }
  pc = 0x474b4c; continue;
 }
 0x474b47 => {
  cpu.push(0x474b4c); f_00474d56(cpu); if cpu.returned_to != 0x474b4c { return; }
  pc = 0x474b4c; continue;
 }
 0x474b4c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xau32); cpu.esi = v; }
  cpu.poll(); pc = 0x474b13; continue;
 }
 0x474b52 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474b53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x474b53); }
 let mut pc: u32 = 0x474b53;
 loop { match pc {
 0x474b53 => {
  { let v: u32 = cpu.r32(0x5ade0cu32); cpu.edi = v; }
  pc = 0x474b59; continue;
 }
 0x474b59 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.sf { pc = 0x474b72; continue; }
  pc = 0x474b69; continue;
 }
 0x474b69 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.push(0x474b70); f_00474c5f(cpu); if cpu.returned_to != 0x474b70 { return; }
  pc = 0x474b88; continue;
 }
 0x474b72 => {
  cpu.push(0x474b77); f_00474d56(cpu); if cpu.returned_to != 0x474b77 { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xd4u32; cpu.eax = v; }
  { let v: u32 = 0x5aa13au32; cpu.esi = v; }
  cpu.push(0x474b87); f_00474d56(cpu); if cpu.returned_to != 0x474b87 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x474b88; continue;
 }
 0x474b88 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if !cpu.zf { cpu.poll(); pc = 0x474b59; continue; }
  pc = 0x474b91; continue;
 }
 0x474b91 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474b92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x474b92); }
 let mut pc: u32 = 0x474b92;
 loop { match pc {
 0x474b92 => {
  { let v: u32 = 0x5a907fu32; cpu.esi = v; }
  pc = 0x474b97; continue;
 }
 0x474b97 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x474bd8; continue; }
  pc = 0x474ba4; continue;
 }
 0x474ba4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x474bb6); f_0041b6f1(cpu); if cpu.returned_to != 0x474bb6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x4e1fe6u32; cpu.esi = v; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.sf { pc = 0x474bcd; continue; }
  pc = 0x474bc3; continue;
 }
 0x474bc3 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x474bcb); f_00474c5f(cpu); if cpu.returned_to != 0x474bcb { return; }
  pc = 0x474bd2; continue;
 }
 0x474bcd => {
  cpu.push(0x474bd2); f_00474d56(cpu); if cpu.returned_to != 0x474bd2 { return; }
  pc = 0x474bd2; continue;
 }
 0x474bd2 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xau32); cpu.esi = v; }
  cpu.poll(); pc = 0x474b97; continue;
 }
 0x474bd8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474bd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x474bd9); }
 let mut pc: u32 = 0x474bd9;
 loop { match pc {
 0x474bd9 => {
  { let v: u32 = 0x5a90c1u32; cpu.esi = v; }
  pc = 0x474bde; continue;
 }
 0x474bde => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x474c1f; continue; }
  pc = 0x474beb; continue;
 }
 0x474beb => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x474bfd); f_0041b6f1(cpu); if cpu.returned_to != 0x474bfd { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x4e1fe6u32; cpu.esi = v; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.sf { pc = 0x474c14; continue; }
  pc = 0x474c0a; continue;
 }
 0x474c0a => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x474c12); f_00474c5f(cpu); if cpu.returned_to != 0x474c12 { return; }
  pc = 0x474c19; continue;
 }
 0x474c14 => {
  cpu.push(0x474c19); f_00474d56(cpu); if cpu.returned_to != 0x474c19 { return; }
  pc = 0x474c19; continue;
 }
 0x474c19 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xau32); cpu.esi = v; }
  cpu.poll(); pc = 0x474bde; continue;
 }
 0x474c1f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474c20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x474c20); }
 let mut pc: u32 = 0x474c20;
 loop { match pc {
 0x474c20 => {
  { let v: u32 = cpu.r32(0x5ade10u32); cpu.edi = v; }
  pc = 0x474c26; continue;
 }
 0x474c26 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.esi = v; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if !cpu.sf { pc = 0x474c3f; continue; }
  pc = 0x474c36; continue;
 }
 0x474c36 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  cpu.push(0x474c3d); f_00474c5f(cpu); if cpu.returned_to != 0x474c3d { return; }
  pc = 0x474c55; continue;
 }
 0x474c3f => {
  cpu.push(0x474c44); f_00474d56(cpu); if cpu.returned_to != 0x474c44 { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xd4u32; cpu.eax = v; }
  { let v: u32 = 0x5aa13au32; cpu.esi = v; }
  cpu.push(0x474c54); f_00474d56(cpu); if cpu.returned_to != 0x474c54 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x474c55; continue;
 }
 0x474c55 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if !cpu.zf { cpu.poll(); pc = 0x474c26; continue; }
  pc = 0x474c5e; continue;
 }
 0x474c5e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474c5f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x474c5f); }
 let mut pc: u32 = 0x474c5f;
 loop { match pc {
 0x474c5f => {
  { let v: u16 = 0x0u16; cpu.w16(0x5d00e5u32, v); }
  pc = 0x474c68; continue;
 }
 0x474c68 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x474c72); f_0041ee78(cpu); if cpu.returned_to != 0x474c72 { return; }
  if cpu.cf { pc = 0x474d3f; continue; }
  pc = 0x474c78; continue;
 }
 0x474c78 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x474d55; continue; }
  pc = 0x474c80; continue;
 }
 0x474c80 => {
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x474c8f; continue; }
  pc = 0x474c85; continue;
 }
 0x474c85 => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  pc = 0x474d38; continue;
 }
 0x474c8f => {
  { cpu.sub8((cpu.edx as u8), 0x41u8); }
  if (cpu.sf != cpu.of) { pc = 0x474c9e; continue; }
  pc = 0x474c94; continue;
 }
 0x474c94 => {
  { cpu.sub8((cpu.edx as u8), 0x5au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x474c9e; continue; }
  pc = 0x474c99; continue;
 }
 0x474c99 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x41u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x474d09; continue;
 }
 0x474c9e => {
  { cpu.sub8((cpu.edx as u8), 0x61u8); }
  if (cpu.sf != cpu.of) { pc = 0x474cad; continue; }
  pc = 0x474ca3; continue;
 }
 0x474ca3 => {
  { cpu.sub8((cpu.edx as u8), 0x7au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x474cad; continue; }
  pc = 0x474ca8; continue;
 }
 0x474ca8 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x61u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x474d09; continue;
 }
 0x474cad => {
  { cpu.sub8((cpu.edx as u8), 0x30u8); }
  if (cpu.sf != cpu.of) { pc = 0x474cbf; continue; }
  pc = 0x474cb2; continue;
 }
 0x474cb2 => {
  { cpu.sub8((cpu.edx as u8), 0x39u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x474cbf; continue; }
  pc = 0x474cb7; continue;
 }
 0x474cb7 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x1au8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x474d09; continue;
 }
 0x474cbf => {
  { cpu.sub8((cpu.edx as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x474ccb; continue; }
  pc = 0x474cc4; continue;
 }
 0x474cc4 => {
  { let v: u32 = 0x1954u32; cpu.ecx = v; }
  pc = 0x474d11; continue;
 }
 0x474ccb => {
  { cpu.sub8((cpu.edx as u8), 0x2cu8); }
  if !cpu.zf { pc = 0x474cd7; continue; }
  pc = 0x474cd0; continue;
 }
 0x474cd0 => {
  { let v: u32 = 0x1955u32; cpu.ecx = v; }
  pc = 0x474d11; continue;
 }
 0x474cd7 => {
  { cpu.sub8((cpu.edx as u8), 0x3fu8); }
  if !cpu.zf { pc = 0x474ce3; continue; }
  pc = 0x474cdc; continue;
 }
 0x474cdc => {
  { let v: u32 = 0x1956u32; cpu.ecx = v; }
  pc = 0x474d11; continue;
 }
 0x474ce3 => {
  { cpu.sub8((cpu.edx as u8), 0x22u8); }
  if !cpu.zf { pc = 0x474cef; continue; }
  pc = 0x474ce8; continue;
 }
 0x474ce8 => {
  { let v: u32 = 0x1957u32; cpu.ecx = v; }
  pc = 0x474d11; continue;
 }
 0x474cef => {
  { cpu.sub8((cpu.edx as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x474cfb; continue; }
  pc = 0x474cf4; continue;
 }
 0x474cf4 => {
  { let v: u32 = 0x1958u32; cpu.ecx = v; }
  pc = 0x474d11; continue;
 }
 0x474cfb => {
  { cpu.sub8((cpu.edx as u8), 0x3au8); }
  if !cpu.zf { pc = 0x474d07; continue; }
  pc = 0x474d00; continue;
 }
 0x474d00 => {
  { let v: u32 = 0x1959u32; cpu.ecx = v; }
  pc = 0x474d11; continue;
 }
 0x474d07 => {
  pc = 0x474d3a; continue;
 }
 0x474d09 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1930u32); cpu.ecx = v; }
  pc = 0x474d11; continue;
 }
 0x474d11 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x5d00e5u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x474d23); cpu.call(t); if cpu.returned_to != 0x474d23 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1930u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5cf8f8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x474d38; continue;
 }
 0x474d38 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x474d3a; continue;
 }
 0x474d3a => {
  cpu.poll(); pc = 0x474c68; continue;
 }
 0x474d3f => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x578d6bu32)); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x474d4c); cpu.call(t); if cpu.returned_to != 0x474d4c { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x581a04u32); cpu.ecx = v; }
  cpu.poll(); pc = 0x474d38; continue;
 }
 0x474d55 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474d56(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x474d56); } r_00474d56(cpu, 0x474d56); }
pub fn f_00474e45(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x474e45); } r_00474d56(cpu, 0x474e45); }
pub fn f_00474e49(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x474e49); } r_00474d56(cpu, 0x474e49); }
fn r_00474d56(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x474d56 => {
  { let v: u16 = 0x0u16; cpu.w16(0x5d00e5u32, v); }
  pc = 0x474d5f; continue;
 }
 0x474d5f => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x474d69); f_0041f3e7(cpu); if cpu.returned_to != 0x474d69 { return; }
  if cpu.cf { pc = 0x474e42; continue; }
  pc = 0x474d6f; continue;
 }
 0x474d6f => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x474e58; continue; }
  pc = 0x474d77; continue;
 }
 0x474d77 => {
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x474d86; continue; }
  pc = 0x474d7c; continue;
 }
 0x474d7c => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  pc = 0x474e3b; continue;
 }
 0x474d86 => {
  { cpu.sub8((cpu.edx as u8), 0x41u8); }
  if (cpu.sf != cpu.of) { pc = 0x474d95; continue; }
  pc = 0x474d8b; continue;
 }
 0x474d8b => {
  { cpu.sub8((cpu.edx as u8), 0x5au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x474d95; continue; }
  pc = 0x474d90; continue;
 }
 0x474d90 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x41u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x474e0c; continue;
 }
 0x474d95 => {
  { cpu.sub8((cpu.edx as u8), 0x61u8); }
  if (cpu.sf != cpu.of) { pc = 0x474da4; continue; }
  pc = 0x474d9a; continue;
 }
 0x474d9a => {
  { cpu.sub8((cpu.edx as u8), 0x7au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x474da4; continue; }
  pc = 0x474d9f; continue;
 }
 0x474d9f => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x61u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x474e0c; continue;
 }
 0x474da4 => {
  { cpu.sub8((cpu.edx as u8), 0x30u8); }
  if (cpu.sf != cpu.of) { pc = 0x474db6; continue; }
  pc = 0x474da9; continue;
 }
 0x474da9 => {
  { cpu.sub8((cpu.edx as u8), 0x39u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x474db6; continue; }
  pc = 0x474dae; continue;
 }
 0x474dae => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x1au8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x474e0c; continue;
 }
 0x474db6 => {
  { cpu.sub8((cpu.edx as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x474dc2; continue; }
  pc = 0x474dbb; continue;
 }
 0x474dbb => {
  { let v: u32 = 0x1994u32; cpu.ecx = v; }
  pc = 0x474e14; continue;
 }
 0x474dc2 => {
  { cpu.sub8((cpu.edx as u8), 0x2cu8); }
  if !cpu.zf { pc = 0x474dce; continue; }
  pc = 0x474dc7; continue;
 }
 0x474dc7 => {
  { let v: u32 = 0x1995u32; cpu.ecx = v; }
  pc = 0x474e14; continue;
 }
 0x474dce => {
  { cpu.sub8((cpu.edx as u8), 0x3fu8); }
  if !cpu.zf { pc = 0x474dda; continue; }
  pc = 0x474dd3; continue;
 }
 0x474dd3 => {
  { let v: u32 = 0x1996u32; cpu.ecx = v; }
  pc = 0x474e14; continue;
 }
 0x474dda => {
  { cpu.sub8((cpu.edx as u8), 0x22u8); }
  if !cpu.zf { pc = 0x474de6; continue; }
  pc = 0x474ddf; continue;
 }
 0x474ddf => {
  { let v: u32 = 0x1997u32; cpu.ecx = v; }
  pc = 0x474e14; continue;
 }
 0x474de6 => {
  { cpu.sub8((cpu.edx as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x474df2; continue; }
  pc = 0x474deb; continue;
 }
 0x474deb => {
  { let v: u32 = 0x1998u32; cpu.ecx = v; }
  pc = 0x474e14; continue;
 }
 0x474df2 => {
  { cpu.sub8((cpu.edx as u8), 0x3au8); }
  if !cpu.zf { pc = 0x474dfe; continue; }
  pc = 0x474df7; continue;
 }
 0x474df7 => {
  { let v: u32 = 0x1999u32; cpu.ecx = v; }
  pc = 0x474e14; continue;
 }
 0x474dfe => {
  { cpu.sub8((cpu.edx as u8), 0x2du8); }
  if !cpu.zf { pc = 0x474e0a; continue; }
  pc = 0x474e03; continue;
 }
 0x474e03 => {
  { let v: u32 = 0x199au32; cpu.ecx = v; }
  pc = 0x474e14; continue;
 }
 0x474e0a => {
  pc = 0x474e3d; continue;
 }
 0x474e0c => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1970u32); cpu.ecx = v; }
  pc = 0x474e14; continue;
 }
 0x474e14 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x5d00e5u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x474e26); cpu.call(t); if cpu.returned_to != 0x474e26 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1970u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5cf922u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x474e3b; continue;
 }
 0x474e3b => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x474e3d; continue;
 }
 0x474e3d => {
  cpu.poll(); pc = 0x474d5f; continue;
 }
 0x474e42 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x578d6bu32)); cpu.ebx = v; }
  pc = 0x474e49; continue;
 }
 0x474e45 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x15ff0057u32.wrapping_add(cpu.ebp)), 0x28u32); cpu.ecx = v; }
  cpu.unimplemented(0x474e4c, "Outsd");
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0xffffff8bu32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffff8bu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax | 0x581a04u32; cpu.eax = v; }
  cpu.poll(); pc = 0x474e3b; continue;
 }
 0x474e49 => {
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x474e4f); cpu.call(t); if cpu.returned_to != 0x474e4f { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x581a04u32); cpu.ecx = v; }
  cpu.poll(); pc = 0x474e3b; continue;
 }
 0x474e58 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474e59(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x474e59); } r_00474e59(cpu, 0x474e59); }
pub fn f_00474f01(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x474f01); } r_00474e59(cpu, 0x474f01); }
fn r_00474e59(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x474e59 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x474f50; continue; }
  pc = 0x474e66; continue;
 }
 0x474e66 => {
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x474e75; continue; }
  pc = 0x474e6b; continue;
 }
 0x474e6b => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  pc = 0x474f49; continue;
 }
 0x474e75 => {
  { cpu.sub8((cpu.edx as u8), 0x81u8); }
  if !cpu.zf { pc = 0x474e94; continue; }
  pc = 0x474e7a; continue;
 }
 0x474e7a => {
  { let v: u32 = 0x1b5au32; cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x474e89); cpu.call(t); if cpu.returned_to != 0x474e89 { return; }
  { cpu.popad(); }
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  pc = 0x474f49; continue;
 }
 0x474e94 => {
  { cpu.sub8((cpu.edx as u8), 0x41u8); }
  if (cpu.sf != cpu.of) { pc = 0x474ea3; continue; }
  pc = 0x474e99; continue;
 }
 0x474e99 => {
  { cpu.sub8((cpu.edx as u8), 0x5au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x474ea3; continue; }
  pc = 0x474e9e; continue;
 }
 0x474e9e => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x41u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x474f1a; continue;
 }
 0x474ea3 => {
  { cpu.sub8((cpu.edx as u8), 0x61u8); }
  if (cpu.sf != cpu.of) { pc = 0x474eb2; continue; }
  pc = 0x474ea8; continue;
 }
 0x474ea8 => {
  { cpu.sub8((cpu.edx as u8), 0x7au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x474eb2; continue; }
  pc = 0x474ead; continue;
 }
 0x474ead => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x61u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x474f1a; continue;
 }
 0x474eb2 => {
  { cpu.sub8((cpu.edx as u8), 0x30u8); }
  if (cpu.sf != cpu.of) { pc = 0x474ec4; continue; }
  pc = 0x474eb7; continue;
 }
 0x474eb7 => {
  { cpu.sub8((cpu.edx as u8), 0x39u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x474ec4; continue; }
  pc = 0x474ebc; continue;
 }
 0x474ebc => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x1au8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x474f1a; continue;
 }
 0x474ec4 => {
  { cpu.sub8((cpu.edx as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x474ed0; continue; }
  pc = 0x474ec9; continue;
 }
 0x474ec9 => {
  { let v: u32 = 0x1aceu32; cpu.ecx = v; }
  pc = 0x474f22; continue;
 }
 0x474ed0 => {
  { cpu.sub8((cpu.edx as u8), 0x2cu8); }
  if !cpu.zf { pc = 0x474edc; continue; }
  pc = 0x474ed5; continue;
 }
 0x474ed5 => {
  { let v: u32 = 0x1acfu32; cpu.ecx = v; }
  pc = 0x474f22; continue;
 }
 0x474edc => {
  { cpu.sub8((cpu.edx as u8), 0x3fu8); }
  if !cpu.zf { pc = 0x474ee8; continue; }
  pc = 0x474ee1; continue;
 }
 0x474ee1 => {
  { let v: u32 = 0x1ad1u32; cpu.ecx = v; }
  pc = 0x474f22; continue;
 }
 0x474ee8 => {
  { cpu.sub8((cpu.edx as u8), 0x22u8); }
  if !cpu.zf { pc = 0x474ef4; continue; }
  pc = 0x474eed; continue;
 }
 0x474eed => {
  { let v: u32 = 0x1ad2u32; cpu.ecx = v; }
  pc = 0x474f22; continue;
 }
 0x474ef4 => {
  { cpu.sub8((cpu.edx as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x474f00; continue; }
  pc = 0x474ef9; continue;
 }
 0x474ef9 => {
  { let v: u32 = 0x1ad3u32; cpu.ecx = v; }
  pc = 0x474f22; continue;
 }
 0x474f00 => {
  { cpu.sub8((cpu.edx as u8), 0x3au8); }
  if !cpu.zf { pc = 0x474f0c; continue; }
  pc = 0x474f05; continue;
 }
 0x474f01 => {
  { cpu.interrupts_off = true; }
  { cpu.sub8(((cpu.edx >> 8) as u8), cpu.r8(0x7u32.wrapping_add(cpu.ebp))); }
  pc = 0x474f05; continue;
 }
 0x474f05 => {
  { let v: u32 = 0x1ad4u32; cpu.ecx = v; }
  pc = 0x474f22; continue;
 }
 0x474f0c => {
  { cpu.sub8((cpu.edx as u8), 0x2du8); }
  if !cpu.zf { pc = 0x474f18; continue; }
  pc = 0x474f11; continue;
 }
 0x474f11 => {
  { let v: u32 = 0x1ad5u32; cpu.ecx = v; }
  pc = 0x474f22; continue;
 }
 0x474f18 => {
  pc = 0x474f4b; continue;
 }
 0x474f1a => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1aaau32); cpu.ecx = v; }
  pc = 0x474f22; continue;
 }
 0x474f22 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x5d00e5u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6f28u32); cpu.push(0x474f34); cpu.call(t); if cpu.returned_to != 0x474f34 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1aaau32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5cf922u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x474f49; continue;
 }
 0x474f49 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x474f4b; continue;
 }
 0x474f4b => {
  cpu.poll(); pc = 0x474e59; continue;
 }
 0x474f50 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00474f51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x474f51); }
 let mut pc: u32 = 0x474f51;
 loop { match pc {
 0x474f51 => {
  { let v: u16 = 0x0u16; cpu.w16(0x5d00e3u32, v); }
  pc = 0x474f5a; continue;
 }
 0x474f5a => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x474f64); f_0041ee78(cpu); if cpu.returned_to != 0x474f64 { return; }
  if cpu.cf { pc = 0x475026; continue; }
  pc = 0x474f6a; continue;
 }
 0x474f6a => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x47501f; continue; }
  pc = 0x474f72; continue;
 }
 0x474f72 => {
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x474f81; continue; }
  pc = 0x474f77; continue;
 }
 0x474f77 => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  pc = 0x475013; continue;
 }
 0x474f81 => {
  { cpu.sub8((cpu.edx as u8), 0x41u8); }
  if (cpu.sf != cpu.of) { pc = 0x474f90; continue; }
  pc = 0x474f86; continue;
 }
 0x474f86 => {
  { cpu.sub8((cpu.edx as u8), 0x5au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x474f90; continue; }
  pc = 0x474f8b; continue;
 }
 0x474f8b => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x41u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x474ffb; continue;
 }
 0x474f90 => {
  { cpu.sub8((cpu.edx as u8), 0x61u8); }
  if (cpu.sf != cpu.of) { pc = 0x474f9f; continue; }
  pc = 0x474f95; continue;
 }
 0x474f95 => {
  { cpu.sub8((cpu.edx as u8), 0x7au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x474f9f; continue; }
  pc = 0x474f9a; continue;
 }
 0x474f9a => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x61u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x474ffb; continue;
 }
 0x474f9f => {
  { cpu.sub8((cpu.edx as u8), 0x30u8); }
  if (cpu.sf != cpu.of) { pc = 0x474fb1; continue; }
  pc = 0x474fa4; continue;
 }
 0x474fa4 => {
  { cpu.sub8((cpu.edx as u8), 0x39u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x474fb1; continue; }
  pc = 0x474fa9; continue;
 }
 0x474fa9 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x1au8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x474ffb; continue;
 }
 0x474fb1 => {
  { cpu.sub8((cpu.edx as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x474fbd; continue; }
  pc = 0x474fb6; continue;
 }
 0x474fb6 => {
  { let v: u32 = 0x1954u32; cpu.ecx = v; }
  pc = 0x475003; continue;
 }
 0x474fbd => {
  { cpu.sub8((cpu.edx as u8), 0x2cu8); }
  if !cpu.zf { pc = 0x474fc9; continue; }
  pc = 0x474fc2; continue;
 }
 0x474fc2 => {
  { let v: u32 = 0x1955u32; cpu.ecx = v; }
  pc = 0x475003; continue;
 }
 0x474fc9 => {
  { cpu.sub8((cpu.edx as u8), 0x3fu8); }
  if !cpu.zf { pc = 0x474fd5; continue; }
  pc = 0x474fce; continue;
 }
 0x474fce => {
  { let v: u32 = 0x1956u32; cpu.ecx = v; }
  pc = 0x475003; continue;
 }
 0x474fd5 => {
  { cpu.sub8((cpu.edx as u8), 0x22u8); }
  if !cpu.zf { pc = 0x474fe1; continue; }
  pc = 0x474fda; continue;
 }
 0x474fda => {
  { let v: u32 = 0x1957u32; cpu.ecx = v; }
  pc = 0x475003; continue;
 }
 0x474fe1 => {
  { cpu.sub8((cpu.edx as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x474fed; continue; }
  pc = 0x474fe6; continue;
 }
 0x474fe6 => {
  { let v: u32 = 0x1958u32; cpu.ecx = v; }
  pc = 0x475003; continue;
 }
 0x474fed => {
  { cpu.sub8((cpu.edx as u8), 0x3au8); }
  if !cpu.zf { pc = 0x474ff9; continue; }
  pc = 0x474ff2; continue;
 }
 0x474ff2 => {
  { let v: u32 = 0x1959u32; cpu.ecx = v; }
  pc = 0x475003; continue;
 }
 0x474ff9 => {
  pc = 0x47501a; continue;
 }
 0x474ffb => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1930u32); cpu.ecx = v; }
  pc = 0x475003; continue;
 }
 0x475003 => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x1930u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5cf8f8u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x3u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x475013; continue;
 }
 0x475013 => {
  { let v: u16 = cpu.r16(0x5d00e3u32).wrapping_add((cpu.ecx as u16)); cpu.w16(0x5d00e3u32, v); }
  pc = 0x47501a; continue;
 }
 0x47501a => {
  cpu.poll(); pc = 0x474f5a; continue;
 }
 0x47501f => {
  { let v: u16 = cpu.r16(0x5d00e3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475026 => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  cpu.poll(); pc = 0x475013; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047502d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47502d); }
 let mut pc: u32 = 0x47502d;
 loop { match pc {
 0x47502d => {
  { let v: u16 = 0x0u16; cpu.w16(0x5d00e3u32, v); }
  pc = 0x475036; continue;
 }
 0x475036 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x475040); f_0041f3e7(cpu); if cpu.returned_to != 0x475040 { return; }
  if cpu.cf { pc = 0x475102; continue; }
  pc = 0x475046; continue;
 }
 0x475046 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4750fb; continue; }
  pc = 0x47504e; continue;
 }
 0x47504e => {
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x47505d; continue; }
  pc = 0x475053; continue;
 }
 0x475053 => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  pc = 0x4750ef; continue;
 }
 0x47505d => {
  { cpu.sub8((cpu.edx as u8), 0x41u8); }
  if (cpu.sf != cpu.of) { pc = 0x47506c; continue; }
  pc = 0x475062; continue;
 }
 0x475062 => {
  { cpu.sub8((cpu.edx as u8), 0x5au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47506c; continue; }
  pc = 0x475067; continue;
 }
 0x475067 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x41u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x4750d7; continue;
 }
 0x47506c => {
  { cpu.sub8((cpu.edx as u8), 0x61u8); }
  if (cpu.sf != cpu.of) { pc = 0x47507b; continue; }
  pc = 0x475071; continue;
 }
 0x475071 => {
  { cpu.sub8((cpu.edx as u8), 0x7au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47507b; continue; }
  pc = 0x475076; continue;
 }
 0x475076 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x61u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x4750d7; continue;
 }
 0x47507b => {
  { cpu.sub8((cpu.edx as u8), 0x30u8); }
  if (cpu.sf != cpu.of) { pc = 0x47508d; continue; }
  pc = 0x475080; continue;
 }
 0x475080 => {
  { cpu.sub8((cpu.edx as u8), 0x39u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x47508d; continue; }
  pc = 0x475085; continue;
 }
 0x475085 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x1au8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x4750d7; continue;
 }
 0x47508d => {
  { cpu.sub8((cpu.edx as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x475099; continue; }
  pc = 0x475092; continue;
 }
 0x475092 => {
  { let v: u32 = 0x1994u32; cpu.ecx = v; }
  pc = 0x4750df; continue;
 }
 0x475099 => {
  { cpu.sub8((cpu.edx as u8), 0x2cu8); }
  if !cpu.zf { pc = 0x4750a5; continue; }
  pc = 0x47509e; continue;
 }
 0x47509e => {
  { let v: u32 = 0x1995u32; cpu.ecx = v; }
  pc = 0x4750df; continue;
 }
 0x4750a5 => {
  { cpu.sub8((cpu.edx as u8), 0x3fu8); }
  if !cpu.zf { pc = 0x4750b1; continue; }
  pc = 0x4750aa; continue;
 }
 0x4750aa => {
  { let v: u32 = 0x1996u32; cpu.ecx = v; }
  pc = 0x4750df; continue;
 }
 0x4750b1 => {
  { cpu.sub8((cpu.edx as u8), 0x22u8); }
  if !cpu.zf { pc = 0x4750bd; continue; }
  pc = 0x4750b6; continue;
 }
 0x4750b6 => {
  { let v: u32 = 0x1997u32; cpu.ecx = v; }
  pc = 0x4750df; continue;
 }
 0x4750bd => {
  { cpu.sub8((cpu.edx as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x4750c9; continue; }
  pc = 0x4750c2; continue;
 }
 0x4750c2 => {
  { let v: u32 = 0x1998u32; cpu.ecx = v; }
  pc = 0x4750df; continue;
 }
 0x4750c9 => {
  { cpu.sub8((cpu.edx as u8), 0x3au8); }
  if !cpu.zf { pc = 0x4750d5; continue; }
  pc = 0x4750ce; continue;
 }
 0x4750ce => {
  { let v: u32 = 0x1999u32; cpu.ecx = v; }
  pc = 0x4750df; continue;
 }
 0x4750d5 => {
  pc = 0x4750f6; continue;
 }
 0x4750d7 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1970u32); cpu.ecx = v; }
  pc = 0x4750df; continue;
 }
 0x4750df => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x1970u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5cf922u32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x3u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x4750ef; continue;
 }
 0x4750ef => {
  { let v: u16 = cpu.r16(0x5d00e3u32).wrapping_add((cpu.ecx as u16)); cpu.w16(0x5d00e3u32, v); }
  pc = 0x4750f6; continue;
 }
 0x4750f6 => {
  cpu.poll(); pc = 0x475036; continue;
 }
 0x4750fb => {
  { let v: u16 = cpu.r16(0x5d00e3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475102 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  cpu.poll(); pc = 0x4750ef; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475109(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x475109); }
 let mut pc: u32 = 0x475109;
 loop { match pc {
 0x475109 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  pc = 0x47511a; continue;
 }
 0x47511a => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x47511a; continue; }
  pc = 0x475126; continue;
 }
 0x475126 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047516c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47516c); }
 let mut pc: u32 = 0x47516c;
 loop { match pc {
 0x47516c => {
  cpu.push(0x475171); f_004765de(cpu); if cpu.returned_to != 0x475171 { return; }
  cpu.push(0x475176); f_004765de(cpu); if cpu.returned_to != 0x475176 { return; }
  cpu.push(0x47517b); f_004765de(cpu); if cpu.returned_to != 0x47517b { return; }
  { let v: u32 = 0x475186u32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475186(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x475186); }
 let mut pc: u32 = 0x475186;
 loop { match pc {
 0x475186 => {
  cpu.push(0x47518b); f_004765de(cpu); if cpu.returned_to != 0x47518b { return; }
  cpu.push(0x475190); f_004765de(cpu); if cpu.returned_to != 0x475190 { return; }
  cpu.push(0x475195); f_004765f2(cpu); if cpu.returned_to != 0x475195 { return; }
  { let v: u32 = 0x4751a0u32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475149(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x475149); } r_00475149(cpu, 0x475149); }
pub fn f_004751a0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4751a0); } r_00475149(cpu, 0x4751a0); }
pub fn f_0047580c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47580c); } r_00475149(cpu, 0x47580c); }
fn r_00475149(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x475128 => {
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x4e1f84u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4753a0; continue; }
  pc = 0x475143; continue;
 }
 0x475143 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1f84u32)); cpu.w8(0x4e1f84u32, v); }
  pc = 0x475149; continue;
 }
 0x475149 => {
  cpu.push(0x47514e); f_0047531f(cpu); if cpu.returned_to != 0x47514e { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edi); cpu.edi = v; }
  cpu.push(0x475157); f_004765de(cpu); if cpu.returned_to != 0x475157 { return; }
  cpu.push(0x47515c); f_004765de(cpu); if cpu.returned_to != 0x47515c { return; }
  cpu.push(0x475161); f_004765de(cpu); if cpu.returned_to != 0x475161 { return; }
  { let v: u32 = 0x47516cu32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4751a0 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x4753a0; continue; }
  pc = 0x4751ab; continue;
 }
 0x4751ab => {
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x1eu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x4751be); f_00476738(cpu); if cpu.returned_to != 0x4751be { return; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x1eu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4751d7u32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47529d => {
  cpu.push(0x4752a2); f_004752b8(cpu); if cpu.returned_to != 0x4752a2 { return; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x65u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4753a0 => {
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x1eu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x4753b3); f_00476738(cpu); if cpu.returned_to != 0x4753b3 { return; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x1eu32.wrapping_add(cpu.eax), v); }
  if cpu.cf { cpu.poll(); pc = 0x47529d; continue; }
  pc = 0x4753c7; continue;
 }
 0x4753c7 => {
  { let v: u32 = 0x4753d2u32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47580c => {
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x475950; continue; }
  pc = 0x47581d; continue;
 }
 0x47581d => {
  { let v: u32 = 0x5d2d90u32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x475829; continue;
 }
 0x475829 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.sf { pc = 0x475839; continue; }
  pc = 0x47582f; continue;
 }
 0x47582f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.cf { pc = 0x475839; continue; }
  pc = 0x475835; continue;
 }
 0x475835 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x475839; continue;
 }
 0x475839 => {
  { let v: u32 = cpu.esi.wrapping_add(0x64u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x475829; continue; }
  pc = 0x47583e; continue;
 }
 0x47583e => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x475882; continue; }
  pc = 0x475842; continue;
 }
 0x475842 => {
  { cpu.sub8(cpu.r8(0x4e1f84u32), 0x7fu8); }
  if cpu.zf { cpu.poll(); pc = 0x475128; continue; }
  pc = 0x47584f; continue;
 }
 0x47584f => {
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x475859); f_00477677(cpu); if cpu.returned_to != 0x475859 { return; }
  if cpu.cf { pc = 0x475881; continue; }
  pc = 0x47585b; continue;
 }
 0x47585b => {
  cpu.push(0x475860); f_00475a67(cpu); if cpu.returned_to != 0x475860 { return; }
  if cpu.cf { pc = 0x47587c; continue; }
  pc = 0x475862; continue;
 }
 0x475862 => {
  { cpu.sub32(cpu.r32(0x4d6b88u32), 0x0u32); }
  if cpu.zf { pc = 0x4759f9; continue; }
  pc = 0x47586f; continue;
 }
 0x47586f => {
  { cpu.sub32(cpu.r32(0x4d6b84u32), 0x0u32); }
  if cpu.zf { pc = 0x4759ac; continue; }
  pc = 0x47587c; continue;
 }
 0x47587c => {
  cpu.push(0x475881); f_004752b8(cpu); if cpu.returned_to != 0x475881 { return; }
  pc = 0x475881; continue;
 }
 0x475881 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475882 => {
  cpu.push(0x475887); f_00475b35(cpu); if cpu.returned_to != 0x475887 { return; }
  cpu.push(0x47588c); f_004762b0(cpu); if cpu.returned_to != 0x47588c { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x4e8570u32; cpu.w32(0x4d6c84u32, v); }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x4758ae; continue; }
  pc = 0x4758a4; continue;
 }
 0x4758a4 => {
  { let v: u32 = 0x4e85b3u32; cpu.w32(0x4d6c84u32, v); }
  pc = 0x4758ae; continue;
 }
 0x4758ae => {
  cpu.push(0x4758b3); f_00476c22(cpu); if cpu.returned_to != 0x4758b3 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x47595b; continue; }
  pc = 0x4758ba; continue;
 }
 0x4758ba => {
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = 0x4c3b52u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = 0x2580u32; cpu.ecx = v; }
  pc = 0x4758d3; continue;
 }
 0x4758d3 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x47594b; continue; }
  pc = 0x4758dd; continue;
 }
 0x4758dd => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x4758e6; continue; }
  pc = 0x4758e2; continue;
 }
 0x4758e2 => {
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x4758e8; continue;
 }
 0x4758e6 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  pc = 0x4758e8; continue;
 }
 0x4758e8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.lods(2, false); }
  { let v: u32 = cpu.eax.wrapping_add(0x4dd54cu32); cpu.eax = v; }
  cpu.push(0x4758f8); f_00476462(cpu); if cpu.returned_to != 0x4758f8 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x47594b; continue; }
  pc = 0x475908; continue;
 }
 0x475908 => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) & 0x7du8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x64u8; cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  pc = 0x475931; continue;
 }
 0x475931 => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x47594a; continue; }
  pc = 0x475938; continue;
 }
 0x475938 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x52bbf0u32); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x47594a; continue;
 }
 0x47594a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47594b => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { pc = 0x475956; continue; }
  pc = 0x475950; continue;
 }
 0x475950 => {
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475956 => {
  cpu.poll(); pc = 0x4758d3; continue;
 }
 0x47595b => {
  cpu.push(0x475960); f_00475a17(cpu); if cpu.returned_to != 0x475960 { return; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x2u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x98u32.wrapping_add(cpu.ebp)) | 0x80u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x47598a); f_0042f53c(cpu); if cpu.returned_to != 0x47598a { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x4759ab; continue; }
  pc = 0x475997; continue;
 }
 0x475997 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x52bbf0u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x4759ab; continue;
 }
 0x4759ab => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4759ac => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0xfu8); }
  if cpu.zf { pc = 0x475a12; continue; }
  pc = 0x4759b5; continue;
 }
 0x4759b5 => {
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x4759bf; continue;
 }
 0x4759bf => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x4c3b52u32; cpu.edi = v; }
  { let v: u32 = 0x2580u32; cpu.ecx = v; }
  pc = 0x4759d1; continue;
 }
 0x4759d1 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x4759dd; continue; }
  pc = 0x4759d7; continue;
 }
 0x4759d7 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4759d1; continue; }
  pc = 0x4759dc; continue;
 }
 0x4759dc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4759dd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x475931; continue;
 }
 0x4759f9 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0xfu8); }
  if cpu.zf { pc = 0x475a12; continue; }
  pc = 0x475a08; continue;
 }
 0x475a08 => {
  { let v: u16 = (cpu.ebx as u16) & 0xfff0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x4759bf; continue;
 }
 0x475a12 => {
  cpu.poll(); pc = 0x47587c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004751d7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4751d7); } r_004751d7(cpu, 0x4751d7); }
pub fn f_004751f0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4751f0); } r_004751d7(cpu, 0x4751f0); }
pub fn f_00475218(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x475218); } r_004751d7(cpu, 0x475218); }
pub fn f_00475240(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x475240); } r_004751d7(cpu, 0x475240); }
pub fn f_00475268(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x475268); } r_004751d7(cpu, 0x475268); }
fn r_004751d7(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4751d7 => {
  cpu.push(0x4751dc); f_004754d8(cpu); if cpu.returned_to != 0x4751dc { return; }
  if cpu.cf { pc = 0x4751ff; continue; }
  pc = 0x4751de; continue;
 }
 0x4751de => {
  { let v: u8 = 0x14u8; cpu.w8(0x4e1f75u32, v); }
  pc = 0x4751e5; continue;
 }
 0x4751e5 => {
  { let v: u32 = 0x4751f0u32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4751f0 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1f75u32)); cpu.w8(0x4e1f75u32, v); }
  if cpu.zf { pc = 0x4751ff; continue; }
  pc = 0x4751f8; continue;
 }
 0x4751f8 => {
  cpu.push(0x4751fd); f_00475cd3(cpu); if cpu.returned_to != 0x4751fd { return; }
  if cpu.cf { cpu.poll(); pc = 0x4751e5; continue; }
  pc = 0x4751ff; continue;
 }
 0x4751ff => {
  cpu.push(0x475204); f_00475618(cpu); if cpu.returned_to != 0x475204 { return; }
  if cpu.cf { pc = 0x475227; continue; }
  pc = 0x475206; continue;
 }
 0x475206 => {
  { let v: u8 = 0x14u8; cpu.w8(0x4e1f75u32, v); }
  pc = 0x47520d; continue;
 }
 0x47520d => {
  { let v: u32 = 0x475218u32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475218 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1f75u32)); cpu.w8(0x4e1f75u32, v); }
  if cpu.zf { pc = 0x475227; continue; }
  pc = 0x475220; continue;
 }
 0x475220 => {
  cpu.push(0x475225); f_00475cd3(cpu); if cpu.returned_to != 0x475225 { return; }
  if cpu.cf { cpu.poll(); pc = 0x47520d; continue; }
  pc = 0x475227; continue;
 }
 0x475227 => {
  cpu.push(0x47522c); f_00475c23(cpu); if cpu.returned_to != 0x47522c { return; }
  if cpu.cf { pc = 0x47524f; continue; }
  pc = 0x47522e; continue;
 }
 0x47522e => {
  { let v: u8 = 0x14u8; cpu.w8(0x4e1f75u32, v); }
  pc = 0x475235; continue;
 }
 0x475235 => {
  { let v: u32 = 0x475240u32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475240 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1f75u32)); cpu.w8(0x4e1f75u32, v); }
  if cpu.zf { pc = 0x47524f; continue; }
  pc = 0x475248; continue;
 }
 0x475248 => {
  cpu.push(0x47524d); f_00475cd3(cpu); if cpu.returned_to != 0x47524d { return; }
  if cpu.cf { cpu.poll(); pc = 0x475235; continue; }
  pc = 0x47524f; continue;
 }
 0x47524f => {
  cpu.push(0x475254); f_00475c7b(cpu); if cpu.returned_to != 0x475254 { return; }
  if cpu.cf { pc = 0x475277; continue; }
  pc = 0x475256; continue;
 }
 0x475256 => {
  { let v: u8 = 0x14u8; cpu.w8(0x4e1f75u32, v); }
  pc = 0x47525d; continue;
 }
 0x47525d => {
  { let v: u32 = 0x475268u32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475268 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1f75u32)); cpu.w8(0x4e1f75u32, v); }
  if cpu.zf { pc = 0x475277; continue; }
  pc = 0x475270; continue;
 }
 0x475270 => {
  cpu.push(0x475275); f_00475cd3(cpu); if cpu.returned_to != 0x475275 { return; }
  if cpu.cf { cpu.poll(); pc = 0x47525d; continue; }
  pc = 0x475277; continue;
 }
 0x475277 => {
  { let v: u32 = 0x475282u32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475282(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x475282); }
 let mut pc: u32 = 0x475282;
 loop { match pc {
 0x475282 => {
  cpu.push(0x475287); f_0047580c(cpu); if cpu.returned_to != 0x475287 { return; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x65u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004752b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4752b8); }
 let mut pc: u32 = 0x4752b8;
 loop { match pc {
 0x4752b8 => {
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x4752df; continue; }
  pc = 0x4752ca; continue;
 }
 0x4752ca => {
  if !cpu.sf { pc = 0x47530c; continue; }
  pc = 0x4752cc; continue;
 }
 0x4752cc => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  cpu.push(0x4752df); f_0046ab56(cpu); if cpu.returned_to != 0x4752df { return; }
  pc = 0x4752df; continue;
 }
 0x4752df => {
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x4752e9); f_00489968(cpu); if cpu.returned_to != 0x4752e9 { return; }
  cpu.push(0x4752ee); f_0043ffdb(cpu); if cpu.returned_to != 0x4752ee { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x47530b; continue; }
  pc = 0x4752f5; continue;
 }
 0x4752f5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x475306; continue; }
  pc = 0x4752fd; continue;
 }
 0x4752fd => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x475306; continue; }
  pc = 0x475302; continue;
 }
 0x475302 => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.eax)) & 0xfcu8; cpu.w8(0x17u32.wrapping_add(cpu.eax), v); }
  pc = 0x475306; continue;
 }
 0x475306 => {
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  pc = 0x47530b; continue;
 }
 0x47530b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47530c => {
  { let v: u8 = cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0xf7u8); cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x475319); f_00489968(cpu); if cpu.returned_to != 0x475319 { return; }
  cpu.push(0x47531e); f_0043ffdb(cpu); if cpu.returned_to != 0x47531e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047531f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47531f); }
 let mut pc: u32 = 0x47531f;
 loop { match pc {
 0x47531f => {
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x475336; continue; }
  pc = 0x47532c; continue;
 }
 0x47532c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d2becu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475336 => {
  { let v: u8 = cpu.r8(0xaeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xaeu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xaeu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x475379; continue; }
  pc = 0x475352; continue;
 }
 0x475352 => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xfff0u32); cpu.ebx = v; }
  { let v: u32 = 0x4dd54cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5d2becu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475379 => {
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xfff0u32); cpu.ebx = v; }
  { let v: u32 = 0x4dd54cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5d2becu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004753d2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4753d2); } r_004753d2(cpu, 0x4753d2); }
pub fn f_004753fa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4753fa); } r_004753d2(cpu, 0x4753fa); }
pub fn f_00475400(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x475400); } r_004753d2(cpu, 0x475400); }
pub fn f_00475432(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x475432); } r_004753d2(cpu, 0x475432); }
pub fn f_0047546a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47546a); } r_004753d2(cpu, 0x47546a); }
pub fn f_004754a2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4754a2); } r_004753d2(cpu, 0x4754a2); }
fn r_004753d2(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4753d2 => {
  cpu.push(0x4753d7); f_00475538(cpu); if cpu.returned_to != 0x4753d7 { return; }
  if cpu.cf { pc = 0x47540a; continue; }
  pc = 0x4753d9; continue;
 }
 0x4753d9 => {
  { let v: u32 = 0x475d96u32; cpu.w32(0x4d6b70u32, v); }
  { let v: u32 = 0xffffff80u32; cpu.edx = v; }
  { let v: u8 = 0x14u8; cpu.w8(0x4e1f75u32, v); }
  pc = 0x4753ef; continue;
 }
 0x4753ef => {
  { let v: u32 = 0x4753fau32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4753fa => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1f75u32)); cpu.w8(0x4e1f75u32, v); }
  pc = 0x475400; continue;
 }
 0x475400 => {
  if cpu.zf { pc = 0x47540a; continue; }
  pc = 0x475402; continue;
 }
 0x475402 => {
  let t = cpu.r32(0x4d6b70u32); cpu.push(0x475408); cpu.call(t); if cpu.returned_to != 0x475408 { return; }
  if cpu.cf { cpu.poll(); pc = 0x4753ef; continue; }
  pc = 0x47540a; continue;
 }
 0x47540a => {
  cpu.push(0x47540f); f_00475570(cpu); if cpu.returned_to != 0x47540f { return; }
  if cpu.cf { pc = 0x475442; continue; }
  pc = 0x475411; continue;
 }
 0x475411 => {
  { let v: u32 = 0x475ebeu32; cpu.w32(0x4d6b70u32, v); }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u8 = 0x14u8; cpu.w8(0x4e1f75u32, v); }
  pc = 0x475427; continue;
 }
 0x475427 => {
  { let v: u32 = 0x475432u32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475432 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1f75u32)); cpu.w8(0x4e1f75u32, v); }
  if cpu.zf { pc = 0x475442; continue; }
  pc = 0x47543a; continue;
 }
 0x47543a => {
  let t = cpu.r32(0x4d6b70u32); cpu.push(0x475440); cpu.call(t); if cpu.returned_to != 0x475440 { return; }
  if cpu.cf { cpu.poll(); pc = 0x475427; continue; }
  pc = 0x475442; continue;
 }
 0x475442 => {
  cpu.push(0x475447); f_004755a8(cpu); if cpu.returned_to != 0x475447 { return; }
  if cpu.cf { pc = 0x47547a; continue; }
  pc = 0x475449; continue;
 }
 0x475449 => {
  { let v: u32 = 0x475fe7u32; cpu.w32(0x4d6b70u32, v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u8 = 0x14u8; cpu.w8(0x4e1f75u32, v); }
  pc = 0x47545f; continue;
 }
 0x47545f => {
  { let v: u32 = 0x47546au32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47546a => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1f75u32)); cpu.w8(0x4e1f75u32, v); }
  if cpu.zf { pc = 0x47547a; continue; }
  pc = 0x475472; continue;
 }
 0x475472 => {
  let t = cpu.r32(0x4d6b70u32); cpu.push(0x475478); cpu.call(t); if cpu.returned_to != 0x475478 { return; }
  if cpu.cf { cpu.poll(); pc = 0x47545f; continue; }
  pc = 0x47547a; continue;
 }
 0x47547a => {
  cpu.push(0x47547f); f_004755e0(cpu); if cpu.returned_to != 0x47547f { return; }
  if cpu.cf { pc = 0x4754b2; continue; }
  pc = 0x475481; continue;
 }
 0x475481 => {
  { let v: u32 = 0x47610au32; cpu.w32(0x4d6b70u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u8 = 0x14u8; cpu.w8(0x4e1f75u32, v); }
  pc = 0x475497; continue;
 }
 0x475497 => {
  { let v: u32 = 0x4754a2u32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4754a2 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1f75u32)); cpu.w8(0x4e1f75u32, v); }
  if cpu.zf { pc = 0x4754b2; continue; }
  pc = 0x4754aa; continue;
 }
 0x4754aa => {
  let t = cpu.r32(0x4d6b70u32); cpu.push(0x4754b0); cpu.call(t); if cpu.returned_to != 0x4754b0 { return; }
  if cpu.cf { cpu.poll(); pc = 0x475497; continue; }
  pc = 0x4754b2; continue;
 }
 0x4754b2 => {
  { let v: u32 = 0x4754bdu32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004754bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4754bd); }
 let mut pc: u32 = 0x4754bd;
 loop { match pc {
 0x4754bd => {
  cpu.push(0x4754c2); f_0047580c(cpu); if cpu.returned_to != 0x4754c2 { return; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x65u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4d6a60u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004754d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4754d8); }
 let mut pc: u32 = 0x4754d8;
 loop { match pc {
 0x4754d8 => {
  cpu.push(0x4754dd); f_00475bb0(cpu); if cpu.returned_to != 0x4754dd { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x5d2d90u32, v); }
  { let v: u32 = 0x5d2d90u32; cpu.w32(0x4d6b74u32, v); }
  { let v: u32 = 0x5d2d92u32; cpu.w32(0x4d6b78u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f6fu32, v); }
  cpu.push(0x475506); f_00475678(cpu); if cpu.returned_to != 0x475506 { return; }
  if !cpu.cf { pc = 0x47552a; continue; }
  pc = 0x475508; continue;
 }
 0x475508 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f6fu32, v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x80u32); cpu.esi = v; }
  cpu.push(0x47551a); f_00475678(cpu); if cpu.returned_to != 0x47551a { return; }
  if !cpu.cf { pc = 0x47552a; continue; }
  pc = 0x47551c; continue;
 }
 0x47551c => {
  { let v: u32 = cpu.sub32(cpu.esi, 0x100u32); cpu.esi = v; }
  cpu.push(0x475527); f_00475678(cpu); if cpu.returned_to != 0x475527 { return; }
  if !cpu.cf { pc = 0x47552a; continue; }
  pc = 0x475529; continue;
 }
 0x475529 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47552a => {
  cpu.push(0x47552f); f_00475be5(cpu); if cpu.returned_to != 0x47552f { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffff80u32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475538(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x475538); }
 let mut pc: u32 = 0x475538;
 loop { match pc {
 0x475538 => {
  cpu.push(0x47553d); f_00475bb0(cpu); if cpu.returned_to != 0x47553d { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x5d2d90u32, v); }
  { let v: u32 = 0x5d2d90u32; cpu.w32(0x4d6b74u32, v); }
  { let v: u32 = 0x5d2d92u32; cpu.w32(0x4d6b78u32, v); }
  cpu.push(0x47555f); f_004757a4(cpu); if cpu.returned_to != 0x47555f { return; }
  if !cpu.cf { pc = 0x475562; continue; }
  pc = 0x475561; continue;
 }
 0x475561 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475562 => {
  cpu.push(0x475567); f_00475be5(cpu); if cpu.returned_to != 0x475567 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffff80u32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x475570); }
 let mut pc: u32 = 0x475570;
 loop { match pc {
 0x475570 => {
  cpu.push(0x475575); f_00475bb0(cpu); if cpu.returned_to != 0x475575 { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x5d2df4u32, v); }
  { let v: u32 = 0x5d2df4u32; cpu.w32(0x4d6b74u32, v); }
  { let v: u32 = 0x5d2df6u32; cpu.w32(0x4d6b78u32, v); }
  cpu.push(0x475597); f_004757a4(cpu); if cpu.returned_to != 0x475597 { return; }
  if !cpu.cf { pc = 0x47559a; continue; }
  pc = 0x475599; continue;
 }
 0x475599 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47559a => {
  cpu.push(0x47559f); f_00475be5(cpu); if cpu.returned_to != 0x47559f { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004755a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4755a8); }
 let mut pc: u32 = 0x4755a8;
 loop { match pc {
 0x4755a8 => {
  cpu.push(0x4755ad); f_00475bb0(cpu); if cpu.returned_to != 0x4755ad { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x5d2e58u32, v); }
  { let v: u32 = 0x5d2e58u32; cpu.w32(0x4d6b74u32, v); }
  { let v: u32 = 0x5d2e5au32; cpu.w32(0x4d6b78u32, v); }
  cpu.push(0x4755cf); f_004757cf(cpu); if cpu.returned_to != 0x4755cf { return; }
  if !cpu.cf { pc = 0x4755d2; continue; }
  pc = 0x4755d1; continue;
 }
 0x4755d1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4755d2 => {
  cpu.push(0x4755d7); f_00475be5(cpu); if cpu.returned_to != 0x4755d7 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004755e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4755e0); }
 let mut pc: u32 = 0x4755e0;
 loop { match pc {
 0x4755e0 => {
  cpu.push(0x4755e5); f_00475bb0(cpu); if cpu.returned_to != 0x4755e5 { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x5d2ebcu32, v); }
  { let v: u32 = 0x5d2ebcu32; cpu.w32(0x4d6b74u32, v); }
  { let v: u32 = 0x5d2ebeu32; cpu.w32(0x4d6b78u32, v); }
  cpu.push(0x475607); f_004757cf(cpu); if cpu.returned_to != 0x475607 { return; }
  if !cpu.cf { pc = 0x47560a; continue; }
  pc = 0x475609; continue;
 }
 0x475609 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47560a => {
  cpu.push(0x47560f); f_00475be5(cpu); if cpu.returned_to != 0x47560f { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475618(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x475618); } r_00475618(cpu, 0x475618); }
pub fn f_00475653(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x475653); } r_00475618(cpu, 0x475653); }
fn r_00475618(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x475618 => {
  cpu.push(0x47561d); f_00475bb0(cpu); if cpu.returned_to != 0x47561d { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x5d2df4u32, v); }
  { let v: u32 = 0x5d2df4u32; cpu.w32(0x4d6b74u32, v); }
  { let v: u32 = 0x5d2df6u32; cpu.w32(0x4d6b78u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f6fu32, v); }
  cpu.push(0x475646); f_00475678(cpu); if cpu.returned_to != 0x475646 { return; }
  if !cpu.cf { pc = 0x47566a; continue; }
  pc = 0x475648; continue;
 }
 0x475648 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f6fu32, v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x80u32); cpu.esi = v; }
  cpu.push(0x47565a); f_00475678(cpu); if cpu.returned_to != 0x47565a { return; }
  if !cpu.cf { pc = 0x47566a; continue; }
  pc = 0x47565c; continue;
 }
 0x475653 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x47565a); f_00475678(cpu); if cpu.returned_to != 0x47565a { return; }
  if !cpu.cf { pc = 0x47566a; continue; }
  pc = 0x47565c; continue;
 }
 0x47565c => {
  { let v: u32 = cpu.sub32(cpu.esi, 0x100u32); cpu.esi = v; }
  cpu.push(0x475667); f_00475678(cpu); if cpu.returned_to != 0x475667 { return; }
  if !cpu.cf { pc = 0x47566a; continue; }
  pc = 0x475669; continue;
 }
 0x475669 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47566a => {
  cpu.push(0x47566f); f_00475be5(cpu); if cpu.returned_to != 0x47566f { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475678(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x475678); }
 let mut pc: u32 = 0x475678;
 loop { match pc {
 0x475678 => {
  { let v: u32 = cpu.r32(0x4d6b84u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x4756df; continue; }
  pc = 0x475686; continue;
 }
 0x475686 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if cpu.zf { pc = 0x4756f4; continue; }
  pc = 0x475691; continue;
 }
 0x475691 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if cpu.zf { pc = 0x4756ff; continue; }
  pc = 0x47569c; continue;
 }
 0x47569c => {
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x4756c3; continue; }
  pc = 0x4756af; continue;
 }
 0x4756af => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x81u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x4756e1; continue; }
  pc = 0x4756ba; continue;
 }
 0x4756ba => {
  { let v: u32 = cpu.eax.wrapping_add(0x100u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x4756e1; continue; }
  pc = 0x4756c3; continue;
 }
 0x4756c3 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x4756df; continue; }
  pc = 0x4756cd; continue;
 }
 0x4756cd => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7fu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x4756ed; continue; }
  pc = 0x4756d6; continue;
 }
 0x4756d6 => {
  { let v: u32 = cpu.eax.wrapping_add(0x100u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x4756ed; continue; }
  pc = 0x4756df; continue;
 }
 0x4756df => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4756e1 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.esi = v; }
  pc = 0x4756e4; continue;
 }
 0x4756e4 => {
  { let v: u8 = cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x4756eb); f_0047623b(cpu); if cpu.returned_to != 0x4756eb { return; }
  pc = 0x4756f2; continue;
 }
 0x4756ed => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.esi = v; }
  cpu.poll(); pc = 0x4756e4; continue;
 }
 0x4756f2 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4756f4 => {
  { cpu.sub8(cpu.r8(0x4e1f6fu32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x4756f2; continue; }
  pc = 0x4756fd; continue;
 }
 0x4756fd => {
  cpu.poll(); pc = 0x4756e1; continue;
 }
 0x4756ff => {
  { cpu.sub8(cpu.r8(0x4e1f6fu32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x4756f2; continue; }
  pc = 0x475708; continue;
 }
 0x475708 => {
  cpu.poll(); pc = 0x4756ed; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047570a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47570a); }
 let mut pc: u32 = 0x47570a;
 loop { match pc {
 0x47570a => {
  { let v: u32 = cpu.r32(0x4d6b88u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x475776; continue; }
  pc = 0x475718; continue;
 }
 0x475718 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x80u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if cpu.zf { pc = 0x47578e; continue; }
  pc = 0x475727; continue;
 }
 0x475727 => {
  { let v: u32 = cpu.eax.wrapping_add(0x100u32); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if cpu.zf { pc = 0x475799; continue; }
  pc = 0x475734; continue;
 }
 0x475734 => {
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xffffff80u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x475759; continue; }
  pc = 0x475747; continue;
 }
 0x475747 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x81u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x475778; continue; }
  pc = 0x475752; continue;
 }
 0x475752 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x475778; continue; }
  pc = 0x475759; continue;
 }
 0x475759 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x475776; continue; }
  pc = 0x475766; continue;
 }
 0x475766 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x7fu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x475784; continue; }
  pc = 0x47576f; continue;
 }
 0x47576f => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x475784; continue; }
  pc = 0x475776; continue;
 }
 0x475776 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475778 => {
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.edi); cpu.esi = v; }
  pc = 0x47577b; continue;
 }
 0x47577b => {
  { let v: u8 = cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x475782); f_0047623b(cpu); if cpu.returned_to != 0x475782 { return; }
  pc = 0x47578c; continue;
 }
 0x475784 => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.edi); cpu.esi = v; }
  cpu.poll(); pc = 0x47577b; continue;
 }
 0x47578c => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47578e => {
  { cpu.sub8(cpu.r8(0x4e1f6fu32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x47578c; continue; }
  pc = 0x475797; continue;
 }
 0x475797 => {
  cpu.poll(); pc = 0x475778; continue;
 }
 0x475799 => {
  { cpu.sub8(cpu.r8(0x4e1f6fu32), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x47578c; continue; }
  pc = 0x4757a2; continue;
 }
 0x4757a2 => {
  cpu.poll(); pc = 0x475784; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004757a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4757a4); }
 let mut pc: u32 = 0x4757a4;
 loop { match pc {
 0x4757a4 => {
  { let v: u32 = cpu.r32(0x4d6b84u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x4757cd; continue; }
  pc = 0x4757ae; continue;
 }
 0x4757ae => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4dd54cu32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4dd54cu32); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xff80u32; cpu.eax = v; }
  { let v: u32 = cpu.esi & 0x7fu32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4dd54cu32); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4757cd => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004757cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4757cf); }
 let mut pc: u32 = 0x4757cf;
 loop { match pc {
 0x4757cf => {
  { let v: u32 = cpu.r32(0x4d6b88u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x47580a; continue; }
  pc = 0x4757d9; continue;
 }
 0x4757d9 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4dd54cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4dd54cu32); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0xff80u32; cpu.esi = v; }
  { let v: u32 = cpu.esi | cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4dd54cu32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x475808; continue; }
  pc = 0x4757fb; continue;
 }
 0x4757fb => {
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.zf { pc = 0x475808; continue; }
  pc = 0x475802; continue;
 }
 0x475802 => {
  { let v: u32 = cpu.sub32(cpu.esi, 0x80u32); cpu.esi = v; }
  pc = 0x475808; continue;
 }
 0x475808 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47580a => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475a17(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x475a17); }
 let mut pc: u32 = 0x475a17;
 loop { match pc {
 0x475a17 => {
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  cpu.push(0x475a22); f_00489968(cpu); if cpu.returned_to != 0x475a22 { return; }
  { let v: u32 = 0x5d2f54u32; cpu.esi = v; }
  { let v: u32 = 0x4ba552u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = 0x2580u32; cpu.ecx = v; }
  pc = 0x475a36; continue;
 }
 0x475a36 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x475a60; continue; }
  pc = 0x475a3c; continue;
 }
 0x475a3c => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x475a45; continue; }
  pc = 0x475a41; continue;
 }
 0x475a41 => {
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x475a47; continue;
 }
 0x475a45 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  pc = 0x475a47; continue;
 }
 0x475a47 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x475a60; continue; }
  pc = 0x475a59; continue;
 }
 0x475a59 => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475a60 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x475a36; continue; }
  pc = 0x475a66; continue;
 }
 0x475a66 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475a67(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x475a67); } r_00475a67(cpu, 0x475a67); }
pub fn f_00475b01(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x475b01); } r_00475a67(cpu, 0x475b01); }
fn r_00475a67(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x475a67 => {
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bf8u32, v); }
  cpu.push(0x475a7b); f_00476604(cpu); if cpu.returned_to != 0x475a7b { return; }
  if cpu.cf { pc = 0x475b33; continue; }
  pc = 0x475a81; continue;
 }
 0x475a81 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x14u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x475b31; continue; }
  pc = 0x475a8b; continue;
 }
 0x475a8b => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x475a98); f_00420989(cpu); if cpu.returned_to != 0x475a98 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x7fu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x4d6bf8u32)); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0xffu8); }
  if cpu.zf { pc = 0x475b33; continue; }
  pc = 0x475ab2; continue;
 }
 0x475ab2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0xfu32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x475ac1; continue; }
  pc = 0x475abe; continue;
 }
 0x475abe => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x475ac1; continue;
 }
 0x475ac1 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bd8u32, v); }
  cpu.push(0x475acb); f_00420989(cpu); if cpu.returned_to != 0x475acb { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x7fu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x4d6bf8u32)); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0xffu8); }
  if cpu.zf { pc = 0x475b33; continue; }
  pc = 0x475ae1; continue;
 }
 0x475ae1 => {
  { let v: u32 = cpu.ebx.wrapping_add(0xfu32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x475aee; continue; }
  pc = 0x475aeb; continue;
 }
 0x475aeb => {
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  pc = 0x475aee; continue;
 }
 0x475aee => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  cpu.push(0x475af9); f_00420989(cpu); if cpu.returned_to != 0x475af9 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x7fu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x4d6bf8u32)); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0xffu8); }
  if cpu.zf { pc = 0x475b33; continue; }
  pc = 0x475b0d; continue;
 }
 0x475b01 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x475b06; continue; }
  pc = 0x475b03; continue;
 }
 0x475b03 => {
  { let v: u32 = cpu.eax | 0x4d6bf8u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0xffu8); }
  if cpu.zf { pc = 0x475b33; continue; }
  pc = 0x475b0d; continue;
 }
 0x475b06 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x2674ff39u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x2674ff39u32.wrapping_add(cpu.eax), v); }
  pc = 0x475b0d; continue;
 }
 0x475b0d => {
  { let v: u32 = cpu.r32(0x4d6bd8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d6bdcu32); cpu.ebx = v; }
  cpu.push(0x475b1d); f_00420989(cpu); if cpu.returned_to != 0x475b1d { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x7fu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x4d6bf8u32)); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0xffu8); }
  if cpu.zf { pc = 0x475b33; continue; }
  pc = 0x475b31; continue;
 }
 0x475b31 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475b33 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475b35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x475b35); }
 let mut pc: u32 = 0x475b35;
 loop { match pc {
 0x475b35 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6b74u32, v); }
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b78u32, v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x475b53); f_00476604(cpu); if cpu.returned_to != 0x475b53 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x475ba6; continue; }
  pc = 0x475b56; continue;
 }
 0x475b56 => {
  { let v: u16 = (cpu.ecx as u16) & 0xfff0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xfff0u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 4u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.edx as u16), 3u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.edx as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4dd54cu32); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x5d2bf0u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x475b88; continue;
 }
 0x475b88 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.sf { pc = 0x475bab; continue; }
  pc = 0x475b95; continue;
 }
 0x475b95 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x475b88; continue; }
  pc = 0x475b9a; continue;
 }
 0x475b9a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x475ba5); f_0047623b(cpu); if cpu.returned_to != 0x475ba5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x475ba6; continue;
 }
 0x475ba6 => {
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475bab => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.poll(); pc = 0x475ba6; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475bb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x475bb0); }
 let mut pc: u32 = 0x475bb0;
 loop { match pc {
 0x475bb0 => {
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x56u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x58u32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x475bc9; continue; }
  pc = 0x475bc4; continue;
 }
 0x475bc4 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4dd54cu32); cpu.eax = v; }
  pc = 0x475bc9; continue;
 }
 0x475bc9 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x475bd3; continue; }
  pc = 0x475bcd; continue;
 }
 0x475bcd => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x4dd54cu32); cpu.ebx = v; }
  pc = 0x475bd3; continue;
 }
 0x475bd3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b84u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6b88u32, v); }
  { let v: u32 = cpu.r32(0x5d2becu32); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475be5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x475be5); }
 let mut pc: u32 = 0x475be5;
 loop { match pc {
 0x475be5 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(0x4dd54cu32); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x4d6a30u32)); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475c23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x475c23); }
 let mut pc: u32 = 0x475c23;
 loop { match pc {
 0x475c23 => {
  cpu.push(0x475c28); f_00475bb0(cpu); if cpu.returned_to != 0x475c28 { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x5d2e58u32, v); }
  { let v: u32 = 0x5d2e58u32; cpu.w32(0x4d6b74u32, v); }
  { let v: u32 = 0x5d2e5au32; cpu.w32(0x4d6b78u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f6fu32, v); }
  cpu.push(0x475c51); f_0047570a(cpu); if cpu.returned_to != 0x475c51 { return; }
  if !cpu.cf { pc = 0x475c6d; continue; }
  pc = 0x475c53; continue;
 }
 0x475c53 => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f6fu32, v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x475c60); f_0047570a(cpu); if cpu.returned_to != 0x475c60 { return; }
  if !cpu.cf { pc = 0x475c6d; continue; }
  pc = 0x475c62; continue;
 }
 0x475c62 => {
  { let v: u32 = cpu.sub32(cpu.esi, 0x2u32); cpu.esi = v; }
  cpu.push(0x475c6a); f_0047570a(cpu); if cpu.returned_to != 0x475c6a { return; }
  if !cpu.cf { pc = 0x475c6d; continue; }
  pc = 0x475c6c; continue;
 }
 0x475c6c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475c6d => {
  cpu.push(0x475c72); f_00475be5(cpu); if cpu.returned_to != 0x475c72 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475c7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x475c7b); }
 let mut pc: u32 = 0x475c7b;
 loop { match pc {
 0x475c7b => {
  cpu.push(0x475c80); f_00475bb0(cpu); if cpu.returned_to != 0x475c80 { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x5d2ebcu32, v); }
  { let v: u32 = 0x5d2ebcu32; cpu.w32(0x4d6b74u32, v); }
  { let v: u32 = 0x5d2ebeu32; cpu.w32(0x4d6b78u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f6fu32, v); }
  cpu.push(0x475ca9); f_0047570a(cpu); if cpu.returned_to != 0x475ca9 { return; }
  if !cpu.cf { pc = 0x475cc5; continue; }
  pc = 0x475cab; continue;
 }
 0x475cab => {
  { let v: u8 = 0x1u8; cpu.w8(0x4e1f6fu32, v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x475cb8); f_0047570a(cpu); if cpu.returned_to != 0x475cb8 { return; }
  if !cpu.cf { pc = 0x475cc5; continue; }
  pc = 0x475cba; continue;
 }
 0x475cba => {
  { let v: u32 = cpu.sub32(cpu.esi, 0x2u32); cpu.esi = v; }
  cpu.push(0x475cc2); f_0047570a(cpu); if cpu.returned_to != 0x475cc2 { return; }
  if !cpu.cf { pc = 0x475cc5; continue; }
  pc = 0x475cc4; continue;
 }
 0x475cc4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475cc5 => {
  cpu.push(0x475cca); f_00475be5(cpu); if cpu.returned_to != 0x475cca { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475cd3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x475cd3); }
 let mut pc: u32 = 0x475cd3;
 loop { match pc {
 0x475cd3 => {
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u16 = 0x4bu16; cpu.w16(0x4d6918u32, v); }
  pc = 0x475cea; continue;
 }
 0x475cea => {
  { let v: u16 = cpu.dec16(cpu.r16(0x4d6918u32)); cpu.w16(0x4d6918u32, v); }
  if !cpu.sf { pc = 0x475cf5; continue; }
  pc = 0x475cf3; continue;
 }
 0x475cf3 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475cf5 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x4d6a30u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x4000u32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x475d43; continue; }
  pc = 0x475d0b; continue;
 }
 0x475d0b => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xfau8); }
  if !cpu.cf { pc = 0x475d43; continue; }
  pc = 0x475d10; continue;
 }
 0x475d10 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x475d45; continue; }
  pc = 0x475d19; continue;
 }
 0x475d19 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x475d64; continue; }
  pc = 0x475d22; continue;
 }
 0x475d22 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x475d32; continue; }
  pc = 0x475d2e; continue;
 }
 0x475d2e => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x475d36; continue; }
  pc = 0x475d32; continue;
 }
 0x475d32 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x475cea; continue; }
  pc = 0x475d34; continue;
 }
 0x475d34 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475d36 => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x475d40); f_0047623b(cpu); if cpu.returned_to != 0x475d40 { return; }
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  pc = 0x475d43; continue;
 }
 0x475d43 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475d45 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x475d51); f_0047623b(cpu); if cpu.returned_to != 0x475d51 { return; }
  if cpu.cf { pc = 0x475d93; continue; }
  pc = 0x475d53; continue;
 }
 0x475d53 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x475cea; continue; }
  pc = 0x475d62; continue;
 }
 0x475d62 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475d64 => {
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x475d6c); f_0047623b(cpu); if cpu.returned_to != 0x475d6c { return; }
  if cpu.cf { pc = 0x475d93; continue; }
  pc = 0x475d6e; continue;
 }
 0x475d6e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x475d89; continue; }
  pc = 0x475d85; continue;
 }
 0x475d85 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x475d36; continue; }
  pc = 0x475d89; continue;
 }
 0x475d89 => {
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x475cea; continue; }
  pc = 0x475d91; continue;
 }
 0x475d91 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475d93 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00475d96(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x475d96); } r_00475d96(cpu, 0x475d96); }
pub fn f_00475ebe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x475ebe); } r_00475d96(cpu, 0x475ebe); }
pub fn f_00475fe7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x475fe7); } r_00475d96(cpu, 0x475fe7); }
pub fn f_0047610a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47610a); } r_00475d96(cpu, 0x47610a); }
fn r_00475d96(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x475d96 => {
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u16 = 0x4bu16; cpu.w16(0x4d6918u32, v); }
  pc = 0x475dad; continue;
 }
 0x475dad => {
  { let v: u32 = 0x475d96u32; cpu.w32(0x4d6b70u32, v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x4d6918u32)); cpu.w16(0x4d6918u32, v); }
  if cpu.sf { pc = 0x475e60; continue; }
  pc = 0x475dc4; continue;
 }
 0x475dc4 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x4d6a30u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x4000u32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x476239; continue; }
  pc = 0x475dde; continue;
 }
 0x475dde => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xfau8); }
  if !cpu.cf { pc = 0x476239; continue; }
  pc = 0x475de7; continue;
 }
 0x475de7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x475e62; continue; }
  pc = 0x475df4; continue;
 }
 0x475df4 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x475e62; continue; }
  pc = 0x475dfe; continue;
 }
 0x475dfe => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x475e18; continue; }
  pc = 0x475e07; continue;
 }
 0x475e07 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x475e93; continue; }
  pc = 0x475e18; continue;
 }
 0x475e18 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x475e26; continue;
 }
 0x475e26 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x475e34; continue;
 }
 0x475e34 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x80u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x80u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x475e45; continue;
 }
 0x475e45 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x81u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x81u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x475e56; continue;
 }
 0x475e56 => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x475dad; continue; }
  pc = 0x475e60; continue;
 }
 0x475e60 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475e62 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x475e6e); f_0047623b(cpu); if cpu.returned_to != 0x475e6e { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x476239; continue; }
  pc = 0x475e75; continue;
 }
 0x475e75 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x475e86; continue; }
  pc = 0x475e79; continue;
 }
 0x475e79 => {
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x475ffe; continue;
 }
 0x475e86 => {
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x476121; continue;
 }
 0x475e93 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x475e99); f_0047623b(cpu); if cpu.returned_to != 0x475e99 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x475eae; continue; }
  pc = 0x475e9e; continue;
 }
 0x475e9e => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.edi = v; }
  pc = 0x476121; continue;
 }
 0x475eae => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.edi = v; }
  pc = 0x475ffe; continue;
 }
 0x475ebe => {
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u16 = 0x4bu16; cpu.w16(0x4d6918u32, v); }
  pc = 0x475ed5; continue;
 }
 0x475ed5 => {
  { let v: u32 = 0x475ebeu32; cpu.w32(0x4d6b70u32, v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x4d6918u32)); cpu.w16(0x4d6918u32, v); }
  if cpu.sf { cpu.poll(); pc = 0x475e60; continue; }
  pc = 0x475eec; continue;
 }
 0x475eec => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x4d6a30u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x4000u32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x476239; continue; }
  pc = 0x475f06; continue;
 }
 0x475f06 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xfau8); }
  if !cpu.cf { pc = 0x476239; continue; }
  pc = 0x475f0f; continue;
 }
 0x475f0f => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x475f91; continue; }
  pc = 0x475f20; continue;
 }
 0x475f20 => {
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x475f91; continue; }
  pc = 0x475f2d; continue;
 }
 0x475f2d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x475f47; continue; }
  pc = 0x475f36; continue;
 }
 0x475f36 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x475fc5; continue; }
  pc = 0x475f47; continue;
 }
 0x475f47 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x475f55; continue;
 }
 0x475f55 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x475f63; continue;
 }
 0x475f63 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x80u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x80u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x475f74; continue;
 }
 0x475f74 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x81u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x81u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x475f85; continue;
 }
 0x475f85 => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x475ed5; continue; }
  pc = 0x475f8f; continue;
 }
 0x475f8f => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x475f91 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x475f9d); f_0047623b(cpu); if cpu.returned_to != 0x475f9d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x476239; continue; }
  pc = 0x475fa4; continue;
 }
 0x475fa4 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x475fb5; continue; }
  pc = 0x475fa8; continue;
 }
 0x475fa8 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x475ffe; continue;
 }
 0x475fb5 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x476121; continue;
 }
 0x475fc5 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x475fcb); f_0047623b(cpu); if cpu.returned_to != 0x475fcb { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x475fdd; continue; }
  pc = 0x475fd0; continue;
 }
 0x475fd0 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.esi); cpu.edi = v; }
  pc = 0x476121; continue;
 }
 0x475fdd => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0xffffff80u32.wrapping_add(cpu.esi); cpu.edi = v; }
  pc = 0x475ffe; continue;
 }
 0x475fe7 => {
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u16 = 0x4bu16; cpu.w16(0x4d6918u32, v); }
  pc = 0x475ffe; continue;
 }
 0x475ffe => {
  { let v: u32 = 0x475fe7u32; cpu.w32(0x4d6b70u32, v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x4d6918u32)); cpu.w16(0x4d6918u32, v); }
  if cpu.sf { cpu.poll(); pc = 0x475e60; continue; }
  pc = 0x476015; continue;
 }
 0x476015 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x4d6a30u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x4000u32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x476239; continue; }
  pc = 0x47602f; continue;
 }
 0x47602f => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xfau8); }
  if !cpu.cf { pc = 0x476239; continue; }
  pc = 0x476038; continue;
 }
 0x476038 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x4760b4; continue; }
  pc = 0x476046; continue;
 }
 0x476046 => {
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x4760b4; continue; }
  pc = 0x476053; continue;
 }
 0x476053 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x47606a; continue; }
  pc = 0x47605c; continue;
 }
 0x47605c => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x4760e5; continue; }
  pc = 0x47606a; continue;
 }
 0x47606a => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x476078; continue;
 }
 0x476078 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x476086; continue;
 }
 0x476086 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x80u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x80u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x476097; continue;
 }
 0x476097 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x81u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x81u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x4760a8; continue;
 }
 0x4760a8 => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x475ffe; continue; }
  pc = 0x4760b2; continue;
 }
 0x4760b2 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4760b4 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4760c0); f_0047623b(cpu); if cpu.returned_to != 0x4760c0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x476239; continue; }
  pc = 0x4760c7; continue;
 }
 0x4760c7 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x4760d8; continue; }
  pc = 0x4760cb; continue;
 }
 0x4760cb => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.poll(); pc = 0x475ed5; continue;
 }
 0x4760d8 => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xffffff80u32; cpu.edx = v; }
  cpu.poll(); pc = 0x475dad; continue;
 }
 0x4760e5 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4760eb); f_0047623b(cpu); if cpu.returned_to != 0x4760eb { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x4760fd; continue; }
  pc = 0x4760f0; continue;
 }
 0x4760f0 => {
  { let v: u32 = 0xffffff80u32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.edi = v; }
  cpu.poll(); pc = 0x475dad; continue;
 }
 0x4760fd => {
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.edi = v; }
  cpu.poll(); pc = 0x475ed5; continue;
 }
 0x47610a => {
  { let v: u32 = 0x28u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u16 = 0x4bu16; cpu.w16(0x4d6918u32, v); }
  pc = 0x476121; continue;
 }
 0x476121 => {
  { let v: u32 = 0x47610au32; cpu.w32(0x4d6b70u32, v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x4d6918u32)); cpu.w16(0x4d6918u32, v); }
  if cpu.sf { cpu.poll(); pc = 0x475e60; continue; }
  pc = 0x476138; continue;
 }
 0x476138 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x4d6a30u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x4000u32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x476239; continue; }
  pc = 0x476152; continue;
 }
 0x476152 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xfau8); }
  if !cpu.cf { pc = 0x476239; continue; }
  pc = 0x47615b; continue;
 }
 0x47615b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x4761d2; continue; }
  pc = 0x476168; continue;
 }
 0x476168 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x4761d2; continue; }
  pc = 0x476175; continue;
 }
 0x476175 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x47618c; continue; }
  pc = 0x47617e; continue;
 }
 0x47617e => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x4761ff; continue; }
  pc = 0x47618c; continue;
 }
 0x47618c => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x47619a; continue;
 }
 0x47619a => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x4761a8; continue;
 }
 0x4761a8 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x80u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x80u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x4761b9; continue;
 }
 0x4761b9 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x81u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x81u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x476224; continue; }
  pc = 0x4761c6; continue;
 }
 0x4761c6 => {
  { let v: u8 = cpu.inc8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x476121; continue; }
  pc = 0x4761d0; continue;
 }
 0x4761d0 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4761d2 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4761de); f_0047623b(cpu); if cpu.returned_to != 0x4761de { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x476239; continue; }
  pc = 0x4761e1; continue;
 }
 0x4761e1 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x4761f2; continue; }
  pc = 0x4761e5; continue;
 }
 0x4761e5 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  cpu.poll(); pc = 0x475ed5; continue;
 }
 0x4761f2 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0xffffff80u32; cpu.edx = v; }
  cpu.poll(); pc = 0x475dad; continue;
 }
 0x4761ff => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x476205); f_0047623b(cpu); if cpu.returned_to != 0x476205 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x476217; continue; }
  pc = 0x47620a; continue;
 }
 0x47620a => {
  { let v: u32 = 0xffffff80u32; cpu.edx = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.esi); cpu.edi = v; }
  cpu.poll(); pc = 0x475dad; continue;
 }
 0x476217 => {
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.esi); cpu.edi = v; }
  cpu.poll(); pc = 0x475ed5; continue;
 }
 0x476224 => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x80u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x476236); f_0047623b(cpu); if cpu.returned_to != 0x476236 { return; }
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  pc = 0x476239; continue;
 }
 0x476239 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0047623b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x47623b); }
 let mut pc: u32 = 0x47623b;
 loop { match pc {
 0x47623b => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_sub(0x4dd54cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4d6b74u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4d6b78u32); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x47627c; continue; }
  pc = 0x476255; continue;
 }
 0x476255 => {
  { cpu.sub16((cpu.esi as u16), cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x47625f; continue; }
  pc = 0x47625b; continue;
 }
 0x47625b => {
  { let v: u8 = cpu.dec8(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x4762ac; continue;
 }
 0x47625f => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if (cpu.sf != cpu.of) { pc = 0x47627c; continue; }
  pc = 0x476265; continue;
 }
 0x476265 => {
  { cpu.sub16((cpu.esi as u16), cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x47627c; continue; }
  pc = 0x47626b; continue;
 }
 0x47626b => {
  { let v: u32 = cpu.r32(0x4d6b78u32).wrapping_sub(0x2u32); cpu.w32(0x4d6b78u32, v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.sub8(((cpu.eax >> 8) as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x4762ac; continue;
 }
 0x47627c => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebx)), 0x30u8); }
  if cpu.cf { pc = 0x476293; continue; }
  pc = 0x476282; continue;
 }
 0x476282 => {
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x47628a); f_004762b0(cpu); if cpu.returned_to != 0x47628a { return; }
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebx)), 0x30u8); }
  if !cpu.cf { pc = 0x4762a8; continue; }
  pc = 0x476293; continue;
 }
 0x476293 => {
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4d6b78u32).wrapping_add(0x2u32); cpu.w32(0x4d6b78u32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebx)), 0x30u8); }
  if cpu.cf { pc = 0x4762ac; continue; }
  pc = 0x4762a8; continue;
 }
 0x4762a8 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4762ac => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004762b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4762b0); }
 let mut pc: u32 = 0x4762b0;
 loop { match pc {
 0x4762b0 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6bdcu32, v); }
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if (cpu.cf || cpu.zf) { pc = 0x476331; continue; }
  pc = 0x4762c3; continue;
 }
 0x4762c3 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x4762d0; continue;
 }
 0x4762d0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x4762da); f_00476347(cpu); if cpu.returned_to != 0x4762da { return; }
  if cpu.cf { pc = 0x476315; continue; }
  pc = 0x4762dc; continue;
 }
 0x4762dc => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x476315; continue; }
  pc = 0x4762e3; continue;
 }
 0x4762e3 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x476315; continue; }
  pc = 0x4762ea; continue;
 }
 0x4762ea => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x2fu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let a = cpu.edi; let b = cpu.esi; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.movs(2, true); }
  { let v: u16 = 0xffffu16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x476320; continue;
 }
 0x476315 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x476320; continue;
 }
 0x476320 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1u32.wrapping_add(cpu.ebx))); }
  if !cpu.cf { pc = 0x476330; continue; }
  pc = 0x47632e; continue;
 }
 0x47632e => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4762d0; continue; }
  pc = 0x476330; continue;
 }
 0x476330 => {
  { cpu.popad(); }
  pc = 0x476331; continue;
 }
 0x476331 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b78u32, v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.neg8(cpu.r8(0x1u32.wrapping_add(cpu.ebx))); cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00476347(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x476347); }
 let mut pc: u32 = 0x476347;
 loop { match pc {
 0x476347 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = 0x2u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x476385; continue; }
  pc = 0x47636d; continue;
 }
 0x47636d => {
  { let v: u32 = 0x513f10u32; cpu.ebp = v; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  pc = 0x476397; continue;
 }
 0x476385 => {
  { let v: u32 = 0x513f10u32; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  pc = 0x476397; continue;
 }
 0x476397 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4dd54cu32); cpu.eax = v; }
  cpu.push(0x4763ce); f_00476462(cpu); if cpu.returned_to != 0x4763ce { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4763e6); f_0048bcbb(cpu); if cpu.returned_to != 0x4763e6 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x4d6bdcu32)); }
  if !cpu.zf { pc = 0x4763ff; continue; }
  pc = 0x4763ef; continue;
 }
 0x4763ef => {
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x476414; continue;
 }
 0x4763ff => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4dd54cu32); cpu.eax = v; }
  cpu.push(0x47640c); f_00476462(cpu); if cpu.returned_to != 0x47640c { return; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x476414; continue;
 }
 0x476414 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x47641b); f_00489637(cpu); if cpu.returned_to != 0x47641b { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x476420; continue; }
  pc = 0x47641f; continue;
 }
 0x47641f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x476420 => {
  { let v: u8 = 0x1u8; cpu.w8(0x60u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x4d6918u32).wrapping_sub(0x5u16); cpu.w16(0x4d6918u32, v); }
  pc = 0x47642d; continue;
 }
 0x47642d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x476434); f_00489897(cpu); if cpu.returned_to != 0x476434 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x9u8); }
  if !cpu.zf { pc = 0x47644b; continue; }
  pc = 0x47643d; continue;
 }
 0x47643d => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { cpu.poll(); pc = 0x47642d; continue; }
  pc = 0x476444; continue;
 }
 0x476444 => {
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  pc = 0x476460; continue;
 }
 0x47644b => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x476460; continue; }
  pc = 0x476452; continue;
 }
 0x476452 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x476460; continue; }
  pc = 0x476459; continue;
 }
 0x476459 => {
  { let v: u8 = 0x2u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x47642d; continue;
 }
 0x476460 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00476462(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x476462); }
 let mut pc: u32 = 0x476462;
 loop { match pc {
 0x476462 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x4dd54cu32); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x80u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x47649e; continue; }
  pc = 0x476496; continue;
 }
 0x476496 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x7u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x47649e; continue;
 }
 0x47649e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004764a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4764a0); }
 let mut pc: u32 = 0x4764a0;
 loop { match pc {
 0x4764a0 => {
  { cpu.sub32(cpu.r32(0x4d6a60u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x47659e; continue; }
  pc = 0x4764ad; continue;
 }
 0x4764ad => {
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x52bbf0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6a28u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x18u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x1cu32.wrapping_add(cpu.ebx), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x476519; continue; }
  pc = 0x4764f2; continue;
 }
 0x4764f2 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x476519; continue; }
  pc = 0x4764f9; continue;
 }
 0x4764f9 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x476519; continue; }
  pc = 0x476500; continue;
 }
 0x476500 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x476519; continue; }
  pc = 0x476507; continue;
 }
 0x476507 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x476515; continue; }
  pc = 0x47650e; continue;
 }
 0x47650e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x476519; continue; }
  pc = 0x476515; continue;
 }
 0x476515 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x476519; continue;
 }
 0x476519 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.eax)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.eax)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x18u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5d2f24u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5d2f28u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5d2f2cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5d2f30u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5d2f34u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5d2f38u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5d2f3cu32); cpu.ebp = v; }
  let t = cpu.r32(0x4d6a60u32); cpu.push(0x476562); cpu.call(t); if cpu.returned_to != 0x476562 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d2f24u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5d2f28u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5d2f2cu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5d2f30u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5d2f34u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5d2f38u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5d2f3cu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x18u32.wrapping_add(cpu.eax), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x1cu32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47659e => {
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x4765a9; continue;
 }
 0x4765a9 => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x513f10u32); }
  if cpu.cf { pc = 0x4765bc; continue; }
  pc = 0x4765b7; continue;
 }
 0x4765b7 => {
  { let v: u32 = 0x509290u32; cpu.ebp = v; }
  pc = 0x4765bc; continue;
 }
 0x4765bc => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x4765db; continue; }
  pc = 0x4765c3; continue;
 }
 0x4765c3 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4d6a34u32, v); }
  { let v: u32 = 0x475149u32; cpu.w32(0x4d6a60u32, v); }
  { let v: u8 = 0x7fu8; cpu.w8(0x4e1f84u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4765db => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x4765a9; continue; }
  pc = 0x4765dd; continue;
 }
 0x4765dd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004765de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4765de); }
 let mut pc: u32 = 0x4765de;
 loop { match pc {
 0x4765de => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4d6a30u32)); cpu.edi = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.r32(0x4d6a30u32)); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004765f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4765f2); }
 let mut pc: u32 = 0x4765f2;
 loop { match pc {
 0x4765f2 => {
  { let v: u32 = 0x5d2d90u32; cpu.edi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00476604(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x476604); }
 let mut pc: u32 = 0x476604;
 loop { match pc {
 0x476604 => {
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x476681; continue; }
  pc = 0x476618; continue;
 }
 0x476618 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x476668; continue; }
  pc = 0x476622; continue;
 }
 0x476622 => {
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x476668; continue; }
  pc = 0x47662c; continue;
 }
 0x47662c => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x476656; continue; }
  pc = 0x476633; continue;
 }
 0x476633 => {
  if cpu.sf { pc = 0x476656; continue; }
  pc = 0x476635; continue;
 }
 0x476635 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x52bc03u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x509290u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x476656 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x4766e5; continue; }
  pc = 0x476661; continue;
 }
 0x476661 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x47669e; continue; }
  pc = 0x476668; continue;
 }
 0x476668 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x4766e3; continue; }
  pc = 0x476674; continue;
 }
 0x476674 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x476681 => {
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x47668a); f_00448238(cpu); if cpu.returned_to != 0x47668a { return; }
  if cpu.cf { pc = 0x476692; continue; }
  pc = 0x47668c; continue;
 }
 0x47668c => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x476692 => {
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xf7u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x4766e3; continue;
 }
 0x47669e => {
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x4766b0; continue; }
  pc = 0x4766a8; continue;
 }
 0x4766a8 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x476668; continue; }
  pc = 0x4766b0; continue;
 }
 0x4766b0 => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if !cpu.zf { pc = 0x4766c5; continue; }
  pc = 0x4766b9; continue;
 }
 0x4766b9 => {
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4766c5 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x476668; continue; }
  pc = 0x4766cf; continue;
 }
 0x4766cf => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4766d5); f_00451959(cpu); if cpu.returned_to != 0x4766d5 { return; }
  if cpu.cf { pc = 0x4766e2; continue; }
  pc = 0x4766d7; continue;
 }
 0x4766d7 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4766e2 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x4766e3; continue;
 }
 0x4766e3 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4766e5 => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xfb2eu16); }
  if cpu.zf { pc = 0x47670b; continue; }
  pc = 0x4766ef; continue;
 }
 0x4766ef => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xef1fu16); }
  if cpu.zf { pc = 0x4766fe; continue; }
  pc = 0x4766f8; continue;
 }
 0x4766f8 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.poll(); pc = 0x476668; continue;
 }
 0x4766fe => {
  { let v: u32 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x509290u32); cpu.esi = v; }
  pc = 0x47672d; continue;
 }
 0x47670b => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x47671b; continue; }
  pc = 0x476715; continue;
 }
 0x476715 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.poll(); pc = 0x476668; continue;
 }
 0x47671b => {
  { let v: u32 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x52bbf0u32); cpu.esi = v; }
  cpu.push(0x47672b); f_0045bc6e(cpu); if cpu.returned_to != 0x47672b { return; }
  if cpu.cf { cpu.poll(); pc = 0x4766e2; continue; }
  pc = 0x47672d; continue;
 }
 0x47672d => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00476738(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x476738); }
 let mut pc: u32 = 0x476738;
 loop { match pc {
 0x476738 => {
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6bf8u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x42u32.wrapping_add(cpu.eax), v); }
  { cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x476759; continue; }
  pc = 0x476755; continue;
 }
 0x476755 => {
  { let v: u8 = 0x0u8; cpu.w8(0x42u32.wrapping_add(cpu.eax), v); }
  pc = 0x476759; continue;
 }
 0x476759 => {
  { let v: u8 = cpu.r8(0x4e1f84u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x2u8; cpu.w8(0x4e1f57u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f58u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4e1f59u32, v); }
  cpu.push(0x476774); f_00476604(cpu); if cpu.returned_to != 0x476774 { return; }
  if cpu.cf { pc = 0x476907; continue; }
  pc = 0x47677a; continue;
 }
 0x47677a => {
  cpu.push(0x47677f); f_00476911(cpu); if cpu.returned_to != 0x47677f { return; }
  { let v: u16 = cpu.r16(0x4d68ecu32).wrapping_add(0x10u16); cpu.w16(0x4d68ecu32, v); }
  pc = 0x476787; continue;
 }
 0x476787 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x4d68e8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4d68eau32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4d6a30u32)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4dd54cu32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x4767c9; continue; }
  pc = 0x4767be; continue;
 }
 0x4767be => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6bf8u32)); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x4767d5; continue; }
  pc = 0x4767c9; continue;
 }
 0x4767c9 => {
  { cpu.sub8(cpu.r8(0x4e1f57u32), 0x0u8); }
  if !cpu.zf { pc = 0x4767dc; continue; }
  pc = 0x4767d2; continue;
 }
 0x4767d2 => {
  { let v: u8 = 0xffu8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x4767d5; continue;
 }
 0x4767d5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f57u32, v); }
  pc = 0x4767dc; continue;
 }
 0x4767dc => {
  { cpu.sub32(cpu.r32(0x4d6a70u32), 0x4769d0u32); }
  if !cpu.zf { pc = 0x476846; continue; }
  pc = 0x4767ec; continue;
 }
 0x4767ec => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x476802; continue; }
  pc = 0x4767f7; continue;
 }
 0x4767f7 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6bf8u32)); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x47680f; continue; }
  pc = 0x476802; continue;
 }
 0x476802 => {
  { cpu.sub8(cpu.r8(0x4e1f58u32), 0x0u8); }
  if !cpu.zf { pc = 0x476816; continue; }
  pc = 0x47680b; continue;
 }
 0x47680b => {
  { let v: u8 = 0xffu8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  pc = 0x47680f; continue;
 }
 0x47680f => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f58u32, v); }
  pc = 0x476816; continue;
 }
 0x476816 => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x47682c; continue; }
  pc = 0x476821; continue;
 }
 0x476821 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6bf8u32)); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x47683d; continue; }
  pc = 0x47682c; continue;
 }
 0x47682c => {
  { cpu.sub8(cpu.r8(0x4e1f59u32), 0x0u8); }
  if !cpu.zf { pc = 0x4768a0; continue; }
  pc = 0x476839; continue;
 }
 0x476839 => {
  { let v: u8 = 0xffu8; cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  pc = 0x47683d; continue;
 }
 0x47683d => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f59u32, v); }
  pc = 0x4768a0; continue;
 }
 0x476846 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x47685f; continue; }
  pc = 0x476854; continue;
 }
 0x476854 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6bf8u32)); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x47686f; continue; }
  pc = 0x47685f; continue;
 }
 0x47685f => {
  { cpu.sub8(cpu.r8(0x4e1f58u32), 0x0u8); }
  if !cpu.zf { pc = 0x476876; continue; }
  pc = 0x476868; continue;
 }
 0x476868 => {
  { let v: u8 = 0xffu8; cpu.w8(0x80u32.wrapping_add(cpu.edi), v); }
  pc = 0x47686f; continue;
 }
 0x47686f => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f58u32, v); }
  pc = 0x476876; continue;
 }
 0x476876 => {
  { let v: u8 = cpu.r8(0xffffff80u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7fu32; cpu.ebx = v; }
  { cpu.sub8((cpu.ebx as u8), 0x1eu8); }
  if cpu.zf { pc = 0x47688c; continue; }
  pc = 0x476881; continue;
 }
 0x476881 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4d6bf8u32)); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0xffu8); }
  if cpu.zf { pc = 0x476899; continue; }
  pc = 0x47688c; continue;
 }
 0x47688c => {
  { cpu.sub8(cpu.r8(0x4e1f59u32), 0x0u8); }
  if !cpu.zf { pc = 0x4768a0; continue; }
  pc = 0x476895; continue;
 }
 0x476895 => {
  { let v: u8 = 0xffu8; cpu.w8(0xffffff80u32.wrapping_add(cpu.edi), v); }
  pc = 0x476899; continue;
 }
 0x476899 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f59u32, v); }
  pc = 0x4768a0; continue;
 }
 0x4768a0 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x4d68ecu32), 0x0u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x4768fc; continue; }
  pc = 0x4768ab; continue;
 }
 0x4768ab => {
  { let v: u8 = 0x10u8; cpu.w8(0x4d6860u32, v); }
  { let v: u16 = cpu.r16(0x4d68eeu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x4d6a70u32); cpu.push(0x4768be); cpu.call(t); if cpu.returned_to != 0x4768be { return; }
  { let v: u16 = cpu.r16(0x4d68ecu32).wrapping_sub(0x10u16); cpu.w16(0x4d68ecu32, v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1f58u32)); cpu.w8(0x4e1f58u32, v); }
  if !cpu.sf { pc = 0x4768d5; continue; }
  pc = 0x4768ce; continue;
 }
 0x4768ce => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f58u32, v); }
  pc = 0x4768d5; continue;
 }
 0x4768d5 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1f59u32)); cpu.w8(0x4e1f59u32, v); }
  if !cpu.sf { pc = 0x4768e4; continue; }
  pc = 0x4768dd; continue;
 }
 0x4768dd => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f59u32, v); }
  pc = 0x4768e4; continue;
 }
 0x4768e4 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4e1f57u32)); cpu.w8(0x4e1f57u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x476787; continue; }
  pc = 0x4768f0; continue;
 }
 0x4768f0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x4e1f57u32, v); }
  cpu.poll(); pc = 0x476787; continue;
 }
 0x4768fc => {
  { let v: u32 = cpu.r32(0x4d6bf8u32); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x42u32.wrapping_add(cpu.eax), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x476907 => {
  { let v: u32 = cpu.r32(0x4d6bf8u32); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x42u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00476911(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x476911); }
 let mut pc: u32 = 0x476911;
 loop { match pc {
 0x476911 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x8u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.esi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.esi as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.esi as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x476944; continue; }
  pc = 0x476932; continue;
 }
 0x476932 => {
  { cpu.sub8(cpu.r8(0xaeu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x476944; continue; }
  pc = 0x47693c; continue;
 }
 0x47693c => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xfu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x476944; continue;
 }
 0x476944 => {
  { let v: u32 = 0xfff0u32; cpu.esi = v; }
  { let v: u32 = cpu.eax & cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx & cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.edx & cpu.esi; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d68e8u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d68eau32, v); }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x476973; continue; }
  pc = 0x476963; continue;
 }
 0x476963 => {
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let a = (cpu.edx as u16); let b = (cpu.ebx as u16); { let v: u16 = b; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } }
  { let v: u16 = 0xffffu16; cpu.w16(0x4d68eeu32, v); }
  pc = 0x47697c; continue;
 }
 0x476973 => {
  { let v: u16 = 0x1u16; cpu.w16(0x4d68eeu32, v); }
  pc = 0x47697c; continue;
 }
 0x47697c => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x47698e; continue; }
  pc = 0x476988; continue;
 }
 0x476988 => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x47698e; continue;
 }
 0x47698e => {
  { cpu.sub16(cpu.r16(0x4d68eeu32), 0x0u16); }
  if !cpu.sf { pc = 0x47699b; continue; }
  pc = 0x476998; continue;
 }
 0x476998 => {
  { let v: u16 = cpu.neg16((cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x47699b; continue;
 }
 0x47699b => {
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if cpu.cf { pc = 0x4769b8; continue; }
  pc = 0x4769a0; continue;
 }
 0x4769a0 => {
  { let v: u16 = (cpu.ecx as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4d68ecu32, v); }
  { let v: u32 = 0x4769efu32; cpu.w32(0x4d6a70u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4769b8 => {
  { let v: u16 = (cpu.edx as u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4d68ecu32, v); }
  { let v: u32 = 0x4769d0u32; cpu.w32(0x4d6a70u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00476a0e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x476a0e); }
 let mut pc: u32 = 0x476a0e;
 loop { match pc {
 0x476a0e => {
  { let v: u32 = 0xd7u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x476a1b; continue;
 }
 0x476a1b => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x476a3b; continue; }
  pc = 0x476a22; continue;
 }
 0x476a22 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.cf { pc = 0x476a3b; continue; }
  pc = 0x476a27; continue;
 }
 0x476a27 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4u32.wrapping_add(cpu.esi))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x476a3b; continue; }
  pc = 0x476a2d; continue;
 }
 0x476a2d => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if cpu.cf { pc = 0x476a3b; continue; }
  pc = 0x476a33; continue;
 }
 0x476a33 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x6u32.wrapping_add(cpu.esi))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x476a3b; continue; }
  pc = 0x476a39; continue;
 }
 0x476a39 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x476a3b => {
  { let v: u32 = cpu.esi.wrapping_add(0xffu32); cpu.esi = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x476a1b; continue; }
  pc = 0x476a45; continue;
 }
 0x476a45 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00476a47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x476a47); }
 let mut pc: u32 = 0x476a47;
 loop { match pc {
 0x476a47 => {
  { let v: u32 = 0x4e8570u32; cpu.w32(0x4d6c84u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x476a62; continue; }
  pc = 0x476a58; continue;
 }
 0x476a58 => {
  { let v: u32 = 0x4e85b3u32; cpu.w32(0x4d6c84u32, v); }
  pc = 0x476a62; continue;
 }
 0x476a62 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4e1f9bu32, v); }
  { let v: u32 = 0x5d2c20u32; cpu.w32(0x4d6b8cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5d2f48u32, v); }
  { let v: u32 = cpu.r32(0x5d2f48u32).wrapping_sub(0x4u32); cpu.w32(0x5d2f48u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5d2f4cu32, v); }
  { cpu.pushad(); }
  { let v = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x58849cu32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x3840u32); }
  if cpu.cf { pc = 0x476bb7; continue; }
  pc = 0x476aca; continue;
 }
 0x476aca => {
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x476ad9); f_00476fdb(cpu); if cpu.returned_to != 0x476ad9 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x476bb7; continue; }
  pc = 0x476ae1; continue;
 }
 0x476ae1 => {
  { let v = cpu.r32(0x5d2c18u32); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x5d2c1cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x476b00); f_00476fdb(cpu); if cpu.returned_to != 0x476b00 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x476bb7; continue; }
  pc = 0x476b10; continue;
 }
 0x476b10 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d697eu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6980u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x476b25); f_00476a0e(cpu); if cpu.returned_to != 0x476b25 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x476bb7; continue; }
  pc = 0x476b2e; continue;
 }
 0x476b2e => {
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x476b38); f_00476a0e(cpu); if cpu.returned_to != 0x476b38 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x476bb7; continue; }
  pc = 0x476b41; continue;
 }
 0x476b41 => {
  { cpu.sub8((cpu.edx as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x476bb7; continue; }
  pc = 0x476b49; continue;
 }
 0x476b49 => {
  cpu.push(0x476b4e); f_00476bfe(cpu); if cpu.returned_to != 0x476b4e { return; }
  if cpu.cf { pc = 0x476bb7; continue; }
  pc = 0x476b54; continue;
 }
 0x476b54 => {
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.r32(0x5d2c18u32), 0x0u32); }
  if !cpu.zf { pc = 0x476b6b; continue; }
  pc = 0x476b66; continue;
 }
 0x476b66 => {
  cpu.push(0x476b6b); f_00476e83(cpu); if cpu.returned_to != 0x476b6b { return; }
  pc = 0x476b6b; continue;
 }
 0x476b6b => {
  cpu.push(0x476b70); f_00476e1c(cpu); if cpu.returned_to != 0x476b70 { return; }
  if !cpu.cf { pc = 0x476b78; continue; }
  pc = 0x476b72; continue;
 }
 0x476b72 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x476bb7; continue;
 }
 0x476b78 => {
  { let v: u16 = cpu.r16(0x4d697eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6980u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.r32(0x5d2c1cu32), 0x0u32); }
  if !cpu.zf { pc = 0x476b93; continue; }
  pc = 0x476b8e; continue;
 }
 0x476b8e => {
  cpu.push(0x476b93); f_00476e83(cpu); if cpu.returned_to != 0x476b93 { return; }
  pc = 0x476b93; continue;
 }
 0x476b93 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if cpu.cf { pc = 0x476bb7; continue; }
  pc = 0x476b99; continue;
 }
 0x476b99 => {
  cpu.push(0x476b9e); f_00476e83(cpu); if cpu.returned_to != 0x476b9e { return; }
  cpu.push(0x476ba3); f_00476ee6(cpu); if cpu.returned_to != 0x476ba3 { return; }
  { cpu.sub8(cpu.r8(0x5d2f41u32), 0x0u8); }
  if !cpu.zf { pc = 0x476bb1; continue; }
  pc = 0x476bac; continue;
 }
 0x476bac => {
  cpu.push(0x476bb1); f_00476d4d(cpu); if cpu.returned_to != 0x476bb1 { return; }
  pc = 0x476bb1; continue;
 }
 0x476bb1 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x476bb7 => {
  cpu.push(0x476bbc); f_00476bca(cpu); if cpu.returned_to != 0x476bbc { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00476bca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x476bca); }
 let mut pc: u32 = 0x476bca;
 loop { match pc {
 0x476bca => {
  { let v: u32 = cpu.r32(0x4e1f9bu32); cpu.ecx = v; }
  { let v: u32 = 0x5d2c20u32; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x476be6; continue;
 }
 0x476bdc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x476be6; continue;
 }
 0x476be6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.sf { cpu.poll(); pc = 0x476bdc; continue; }
  pc = 0x476be9; continue;
 }
 0x476be9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4e1f9bu32, v); }
  { let v: u32 = 0x5d2c20u32; cpu.w32(0x4d6b8cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00476bfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x476bfe); }
 let mut pc: u32 = 0x476bfe;
 loop { match pc {
 0x476bfe => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4d6b4cu32)); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x28u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), 0xffu8); }
  if cpu.zf { pc = 0x476c1e; continue; }
  pc = 0x476c1a; continue;
 }
 0x476c1a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x476c1e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00476c22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x476c22); }
 let mut pc: u32 = 0x476c22;
 loop { match pc {
 0x476c22 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = 0x5d2f50u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4d6a34u32); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x476c95; continue; }
  pc = 0x476c3f; continue;
 }
 0x476c3f => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x476c53; continue;
 }
 0x476c53 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x4dd54cu32); cpu.eax = v; }
  cpu.push(0x476c62); f_00476462(cpu); if cpu.returned_to != 0x476c62 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5d2f42u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5d2f44u32, v); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x476c53; continue; }
  pc = 0x476c80; continue;
 }
 0x476c80 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x5d2f50u32; cpu.esi = v; }
  cpu.push(0x476c90); f_0048c798(cpu); if cpu.returned_to != 0x476c90 { return; }
  { cpu.sub32(cpu.eax, 0x1eu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x476c97; continue; }
  pc = 0x476c95; continue;
 }
 0x476c95 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x476c97 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x56c6cfu32, v); }
  { let v: u32 = 0x5b9d10u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x13u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x5d2f42u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d2f44u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5d2f41u32, v); }
  cpu.push(0x476ce1); f_00476a47(cpu); if cpu.returned_to != 0x476ce1 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5d2f41u32, v); }
  if cpu.cf { pc = 0x476d43; continue; }
  pc = 0x476cea; continue;
 }
 0x476cea => {
  { let v: u32 = 0x5d2f50u32; cpu.edi = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0x5b9d10u32; cpu.esi = v; }
  pc = 0x476d0d; continue;
 }
 0x476d0d => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x476d28; continue; }
  pc = 0x476d11; continue;
 }
 0x476d11 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x476d0d; continue; }
  pc = 0x476d28; continue;
 }
 0x476d28 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x5d2f50u32; cpu.esi = v; }
  cpu.push(0x476d38); f_0048c798(cpu); if cpu.returned_to != 0x476d38 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x56c6cfu32, v); }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x476d43 => {
  { let v: u8 = 0x0u8; cpu.w8(0x56c6cfu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00476d4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x476d4d); }
 let mut pc: u32 = 0x476d4d;
 loop { match pc {
 0x476d4d => {
  { let v: u32 = cpu.r32(0x5d2f48u32); cpu.ebx = v; }
  { let v: u32 = 0x5d2c20u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5d2f4cu32); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x476d72; continue;
 }
 0x476d72 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x476d72; continue; }
  pc = 0x476d80; continue;
 }
 0x476d80 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x5d2c20u32; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x476d91); f_0048c798(cpu); if cpu.returned_to != 0x476d91 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5da9edu32, v); }
  cpu.push(0x476daf); f_0048c798(cpu); if cpu.returned_to != 0x476daf { return; }
  { cpu.sub8(cpu.r8(0x581c00u32), 0x0u8); }
  if !cpu.zf { pc = 0x476db8; continue; }
  pc = 0x476db8; continue;
 }
 0x476db8 => {
  { cpu.sub8(cpu.r8(0x4d67dau32), 0x5u8); }
  if !cpu.zf { pc = 0x476dcf; continue; }
  pc = 0x476dc1; continue;
 }
 0x476dc1 => {
  { cpu.sub8(cpu.r8(0x5da9edu32), 0x2u8); }
  if cpu.cf { pc = 0x476dcf; continue; }
  pc = 0x476dca; continue;
 }
 0x476dca => {
  { let v: u32 = 0xfffffffu32; cpu.eax = v; }
  pc = 0x476dcf; continue;
 }
 0x476dcf => {
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x476deb; continue; }
  pc = 0x476dd3; continue;
 }
 0x476dd3 => {
  { let v: u32 = cpu.r32(0x5d2f4cu32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x581c00u32), 0x0u8); }
  if !cpu.zf { pc = 0x476dea; continue; }
  pc = 0x476de2; continue;
 }
 0x476de2 => {
  { let v: u8 = cpu.or8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x80u8); cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  pc = 0x476dea; continue;
 }
 0x476dea => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x476deb => {
  { let v: u32 = cpu.r32(0x5d2f48u32); cpu.ebx = v; }
  pc = 0x476df1; continue;
 }
 0x476df1 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x476e03; continue; }
  pc = 0x476df5; continue;
 }
 0x476df5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.poll(); pc = 0x476df1; continue;
 }
 0x476e03 => {
  { let v: u32 = cpu.r32(0x5d2f48u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x5d2c24u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d2c26u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x476e1b); f_00476e83(cpu); if cpu.returned_to != 0x476e1b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00476e1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x476e1c); }
 let mut pc: u32 = 0x476e1c;
 loop { match pc {
 0x476e1c => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4d6b4cu32); cpu.edi = v; }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  pc = 0x476e2b; continue;
 }
 0x476e2b => {
  { cpu.sub8((cpu.ebx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x476e7f; continue; }
  pc = 0x476e2f; continue;
 }
 0x476e2f => {
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4d6b4cu32)); cpu.edi = v; }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if cpu.zf { pc = 0x476e81; continue; }
  pc = 0x476e4a; continue;
 }
 0x476e4a => {
  { let v = (cpu.edx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4d6b4cu32)); cpu.edi = v; }
  { cpu.sub8((cpu.ebx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x476e7f; continue; }
  pc = 0x476e58; continue;
 }
 0x476e58 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xff80u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0xfu32); cpu.ebx = v; }
  cpu.push(0x476e7b); f_00476e83(cpu); if cpu.returned_to != 0x476e7b { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if !cpu.cf { cpu.poll(); pc = 0x476e2b; continue; }
  pc = 0x476e7e; continue;
 }
 0x476e7e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x476e7f => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x476e81 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00476e83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x476e83); }
 let mut pc: u32 = 0x476e83;
 loop { match pc {
 0x476e83 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x476e85; continue;
 }
 0x476e85 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x476ea9; continue; }
  pc = 0x476e8b; continue;
 }
 0x476e8b => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x4c3b52u32); }
  if cpu.cf { cpu.poll(); pc = 0x476e85; continue; }
  pc = 0x476e96; continue;
 }
 0x476e96 => {
  { cpu.sub8(cpu.r8(0x56c6cfu32), 0x1u8); }
  if !cpu.zf { pc = 0x476ea7; continue; }
  pc = 0x476e9f; continue;
 }
 0x476e9f => {
  { cpu.sub32(cpu.edi, 0x5b9f90u32); }
  if cpu.cf { cpu.poll(); pc = 0x476e85; continue; }
  pc = 0x476ea7; continue;
 }
 0x476ea7 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x476ea9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x476ee4; continue; }
  pc = 0x476ebb; continue;
 }
 0x476ebb => {
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x4e1f9bu32)); cpu.w32(0x4e1f9bu32, v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d6b8cu32); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4d6b8cu32, v); }
  { cpu.sub32(cpu.ebx, 0x5d2d60u32); }
  if cpu.cf { pc = 0x476ee1; continue; }
  pc = 0x476ede; continue;
 }
 0x476ede => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x476ee1 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x476ee4; continue;
 }
 0x476ee4 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00476ee6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x476ee6); }
 let mut pc: u32 = 0x476ee6;
 loop { match pc {
 0x476ee6 => {
  { let v: u32 = cpu.r32(0x4e1f9bu32); cpu.edx = v; }
  { let v: u32 = 0x5d2c20u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x476ef8); f_0040d930(cpu); if cpu.returned_to != 0x476ef8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4e1f9bu32, v); }
  { cpu.sub32(cpu.r32(0x4e1f9bu32), 0x5u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x476fda; continue; }
  pc = 0x476f0d; continue;
 }
 0x476f0d => {
  { let v: u32 = 0x5d2c20u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6926u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6928u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d692au32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d692cu32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d692eu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6930u32, v); }
  cpu.push(0x476f54); f_00489c78(cpu); if cpu.returned_to != 0x476f54 { return; }
  if cpu.cf { pc = 0x476f73; continue; }
  pc = 0x476f56; continue;
 }
 0x476f56 => {
  { let v: u32 = 0x5d2c20u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.dec32(cpu.r32(0x4e1f9bu32)); cpu.w32(0x4e1f9bu32, v); }
  pc = 0x476f73; continue;
 }
 0x476f73 => {
  { let v: u32 = cpu.r32(0x4d6b8cu32); cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6926u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6928u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d692au32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d692cu32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d692eu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6930u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x476fbf); f_00489c78(cpu); if cpu.returned_to != 0x476fbf { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x476fda; continue; }
  pc = 0x476fc2; continue;
 }
 0x476fc2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.dec32(cpu.r32(0x4e1f9bu32)); cpu.w32(0x4e1f9bu32, v); }
  pc = 0x476fda; continue;
 }
 0x476fda => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00476fdb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x476fdb); }
 let mut pc: u32 = 0x476fdb;
 loop { match pc {
 0x476fdb => {
  { cpu.pushad(); }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5d2d60u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5d2d62u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5d2d64u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5d2d66u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5d2d68u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5d2d6au32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5d2d6cu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x5d2d6eu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5d2c18u32, v); }
  cpu.push(0x477035); f_00420989(cpu); if cpu.returned_to != 0x477035 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x7u8); }
  if cpu.zf { pc = 0x47704f; continue; }
  pc = 0x477045; continue;
 }
 0x477045 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x33u8); }
  if cpu.zf { pc = 0x47704f; continue; }
  pc = 0x47704a; continue;
 }
 0x47704a => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x17u8); }
  if !cpu.zf { pc = 0x47706e; continue; }
  pc = 0x47704f; continue;
 }
 0x47704f => {
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.w32(0x5d2c18u32, v); }
  cpu.push(0x47706c); f_004774c5(cpu); if cpu.returned_to != 0x47706c { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x47706e => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffff80u32; cpu.ebp = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0xfff0u16; cpu.w16(0x4d693au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693eu32, v); }
  { let v = cpu.r16(0x4d6934u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d6936u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x4770ab); f_004775fb(cpu); if cpu.returned_to != 0x4770ab { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x4770e1; continue; }
  pc = 0x4770af; continue;
 }
 0x4770af => {
  { let v: u16 = cpu.r16(0x4d693eu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d60u32, v); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d70u32, v); }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d72u32, v); }
  pc = 0x4770e1; continue;
 }
 0x4770e1 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6936u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6934u32, v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x80u32; cpu.ebp = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0x10u16; cpu.w16(0x4d693au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693eu32, v); }
  { let v = cpu.r16(0x4d6934u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d6936u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x477126); f_004775fb(cpu); if cpu.returned_to != 0x477126 { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x47715c; continue; }
  pc = 0x47712a; continue;
 }
 0x47712a => {
  { let v: u16 = cpu.r16(0x4d693eu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d62u32, v); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d74u32, v); }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d76u32, v); }
  pc = 0x47715c; continue;
 }
 0x47715c => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6936u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6934u32, v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u16 = 0x10u16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693eu32, v); }
  { let v = cpu.r16(0x4d6934u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d6936u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x4771a1); f_004775fb(cpu); if cpu.returned_to != 0x4771a1 { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x4771d7; continue; }
  pc = 0x4771a5; continue;
 }
 0x4771a5 => {
  { let v: u16 = cpu.r16(0x4d693eu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d64u32, v); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d78u32, v); }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d7au32, v); }
  pc = 0x4771d7; continue;
 }
 0x4771d7 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6936u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6934u32, v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  { let v: u16 = 0xfff0u16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693eu32, v); }
  { let v = cpu.r16(0x4d6934u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d6936u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x47721c); f_004775fb(cpu); if cpu.returned_to != 0x47721c { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x477252; continue; }
  pc = 0x477220; continue;
 }
 0x477220 => {
  { let v: u16 = cpu.r16(0x4d693eu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d66u32, v); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d7cu32, v); }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d7eu32, v); }
  pc = 0x477252; continue;
 }
 0x477252 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6936u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6934u32, v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffff81u32; cpu.ebp = v; }
  { let v: u16 = 0x10u16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0xfff0u16; cpu.w16(0x4d693au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693eu32, v); }
  { let v = cpu.r16(0x4d6934u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d6936u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x477297); f_004775fb(cpu); if cpu.returned_to != 0x477297 { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x4772d6; continue; }
  pc = 0x47729b; continue;
 }
 0x47729b => {
  { let v: u16 = cpu.r16(0x4d693eu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d68u32, v); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d80u32, v); }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d82u32, v); }
  pc = 0x4772d6; continue;
 }
 0x4772d6 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6936u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6934u32, v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x81u32; cpu.ebp = v; }
  { let v: u16 = 0x10u16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0x10u16; cpu.w16(0x4d693au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693eu32, v); }
  { let v = cpu.r16(0x4d6934u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d6936u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x47731b); f_004775fb(cpu); if cpu.returned_to != 0x47731b { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x47735a; continue; }
  pc = 0x47731f; continue;
 }
 0x47731f => {
  { let v: u16 = cpu.r16(0x4d693eu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d6au32, v); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d84u32, v); }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d86u32, v); }
  pc = 0x47735a; continue;
 }
 0x47735a => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6936u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6934u32, v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x7fu32; cpu.ebp = v; }
  { let v: u16 = 0xfff0u16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0x10u16; cpu.w16(0x4d693au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693eu32, v); }
  { let v = cpu.r16(0x4d6934u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d6936u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x47739f); f_004775fb(cpu); if cpu.returned_to != 0x47739f { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x4773de; continue; }
  pc = 0x4773a3; continue;
 }
 0x4773a3 => {
  { let v: u16 = cpu.r16(0x4d693eu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d6cu32, v); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d88u32, v); }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d8au32, v); }
  pc = 0x4773de; continue;
 }
 0x4773de => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6936u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6934u32, v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffff7fu32; cpu.ebp = v; }
  { let v: u16 = 0xfff0u16; cpu.w16(0x4d6938u32, v); }
  { let v: u16 = 0xfff0u16; cpu.w16(0x4d693au32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4d693eu32, v); }
  { let v = cpu.r16(0x4d6934u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x4d6936u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.push(0x477423); f_004775fb(cpu); if cpu.returned_to != 0x477423 { return; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x477462; continue; }
  pc = 0x477427; continue;
 }
 0x477427 => {
  { let v: u16 = cpu.r16(0x4d693eu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d6eu32, v); }
  { let v: u16 = cpu.r16(0x4d6934u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d8cu32, v); }
  { let v: u16 = cpu.r16(0x4d6936u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x5d2d8eu32, v); }
  pc = 0x477462; continue;
 }
 0x477462 => {
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6936u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x4d6934u32, v); }
  { let v: u32 = 0x5d2d60u32; cpu.esi = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  pc = 0x477481; continue;
 }
 0x477481 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if !cpu.cf { pc = 0x47748d; continue; }
  pc = 0x477488; continue;
 }
 0x477488 => {
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x47748d; continue;
 }
 0x47748d => {
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x477481; continue; }
  pc = 0x477491; continue;
 }
 0x477491 => {
  { cpu.sub16((cpu.ebx as u16), 0xffffu16); }
  if cpu.zf { pc = 0x4774c2; continue; }
  pc = 0x477497; continue;
 }
 0x477497 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8((cpu.ebx as u8), 2u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4d6936u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r16(0x4d6936u32) as u32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x5d2d70u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x5d2d72u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x4774c0); f_004774c5(cpu); if cpu.returned_to != 0x4774c0 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4774c2 => {
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004774c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4774c5); }
 let mut pc: u32 = 0x4774c5;
 loop { match pc {
 0x4774c5 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d2c10u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5d2c14u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x4774e3); f_00420989(cpu); if cpu.returned_to != 0x4774e3 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x7u8); }
  if cpu.zf { pc = 0x477503; continue; }
  pc = 0x4774f5; continue;
 }
 0x4774f5 => {
  { cpu.sub8((cpu.ecx as u8), 0x33u8); }
  if cpu.zf { pc = 0x477503; continue; }
  pc = 0x4774fa; continue;
 }
 0x4774fa => {
  { cpu.sub8((cpu.ecx as u8), 0x17u8); }
  if !cpu.zf { pc = 0x4775f9; continue; }
  pc = 0x477503; continue;
 }
 0x477503 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x477506; continue;
 }
 0x477506 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if cpu.zf { pc = 0x47751a; continue; }
  pc = 0x477510; continue;
 }
 0x477510 => {
  { cpu.sub8((cpu.edx as u8), 0x33u8); }
  if cpu.zf { pc = 0x47751a; continue; }
  pc = 0x477515; continue;
 }
 0x477515 => {
  { cpu.sub8((cpu.edx as u8), 0x17u8); }
  if !cpu.zf { pc = 0x477521; continue; }
  pc = 0x47751a; continue;
 }
 0x47751a => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x10u32); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x477506; continue; }
  pc = 0x477521; continue;
 }
 0x477521 => {
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x47756a; continue; }
  pc = 0x477526; continue;
 }
 0x477526 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x47752c; continue;
 }
 0x47752c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if cpu.zf { pc = 0x477540; continue; }
  pc = 0x477536; continue;
 }
 0x477536 => {
  { cpu.sub8((cpu.edx as u8), 0x33u8); }
  if cpu.zf { pc = 0x477540; continue; }
  pc = 0x47753b; continue;
 }
 0x47753b => {
  { cpu.sub8((cpu.edx as u8), 0x17u8); }
  if !cpu.zf { pc = 0x47754e; continue; }
  pc = 0x477540; continue;
 }
 0x477540 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x4d678eu32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x47752c; continue; }
  pc = 0x47754e; continue;
 }
 0x47754e => {
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if cpu.zf { pc = 0x477562; continue; }
  pc = 0x477553; continue;
 }
 0x477553 => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if !cpu.zf { pc = 0x47756a; continue; }
  pc = 0x477558; continue;
 }
 0x477558 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d2c10u32, v); }
  pc = 0x47756a; continue;
 }
 0x477562 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d2c10u32, v); }
  pc = 0x47756a; continue;
 }
 0x47756a => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x47756d; continue;
 }
 0x47756d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if cpu.zf { pc = 0x477581; continue; }
  pc = 0x477577; continue;
 }
 0x477577 => {
  { cpu.sub8((cpu.edx as u8), 0x33u8); }
  if cpu.zf { pc = 0x477581; continue; }
  pc = 0x47757c; continue;
 }
 0x47757c => {
  { cpu.sub8((cpu.edx as u8), 0x17u8); }
  if !cpu.zf { pc = 0x47758d; continue; }
  pc = 0x477581; continue;
 }
 0x477581 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x80u32); cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x10u32); cpu.ebx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x47756d; continue; }
  pc = 0x47758d; continue;
 }
 0x47758d => {
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x4775e6; continue; }
  pc = 0x477596; continue;
 }
 0x477596 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x80u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  pc = 0x4775a1; continue;
 }
 0x4775a1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if cpu.zf { pc = 0x4775b5; continue; }
  pc = 0x4775ab; continue;
 }
 0x4775ab => {
  { cpu.sub8((cpu.edx as u8), 0x33u8); }
  if cpu.zf { pc = 0x4775b5; continue; }
  pc = 0x4775b0; continue;
 }
 0x4775b0 => {
  { cpu.sub8((cpu.edx as u8), 0x17u8); }
  if !cpu.zf { pc = 0x4775c8; continue; }
  pc = 0x4775b5; continue;
 }
 0x4775b5 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x80u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x4d6790u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x4775a1; continue; }
  pc = 0x4775c8; continue;
 }
 0x4775c8 => {
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if cpu.zf { pc = 0x4775dd; continue; }
  pc = 0x4775cd; continue;
 }
 0x4775cd => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if !cpu.zf { pc = 0x4775e6; continue; }
  pc = 0x4775d2; continue;
 }
 0x4775d2 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x20u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5d2c14u32, v); }
  pc = 0x4775e6; continue;
 }
 0x4775dd => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x30u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5d2c14u32, v); }
  pc = 0x4775e6; continue;
 }
 0x4775e6 => {
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x5d2c10u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5d2c14u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0xfu32); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4775f9 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004775fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4775fb); }
 let mut pc: u32 = 0x4775fb;
 loop { match pc {
 0x4775fb => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x4775fc; continue;
 }
 0x4775fc => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x477675; continue; }
  pc = 0x47760a; continue;
 }
 0x47760a => {
  { cpu.sub16(cpu.r16(0x4d6934u32), 0x800u16); }
  if !cpu.cf { pc = 0x477675; continue; }
  pc = 0x477619; continue;
 }
 0x477619 => {
  { cpu.sub16(cpu.r16(0x4d6936u32), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x477675; continue; }
  pc = 0x477623; continue;
 }
 0x477623 => {
  { cpu.sub16(cpu.r16(0x4d6936u32), 0x800u16); }
  if !cpu.cf { pc = 0x477675; continue; }
  pc = 0x47762e; continue;
 }
 0x47762e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x47766c; continue; }
  pc = 0x477636; continue;
 }
 0x477636 => {
  { cpu.sub8((cpu.eax as u8), 0x33u8); }
  if cpu.zf { pc = 0x47766c; continue; }
  pc = 0x47763a; continue;
 }
 0x47763a => {
  { cpu.sub8((cpu.eax as u8), 0x17u8); }
  if cpu.zf { pc = 0x47766c; continue; }
  pc = 0x47763e; continue;
 }
 0x47763e => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x477645; continue; }
  pc = 0x477643; continue;
 }
 0x477643 => {
  { let v: u8 = (cpu.edx as u8) | ((cpu.ecx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x477645; continue;
 }
 0x477645 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x4d693eu32)); cpu.w16(0x4d693eu32, v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x4d6938u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6934u32).wrapping_add((cpu.edi as u16)); cpu.w16(0x4d6934u32, v); }
  { let v: u16 = cpu.r16(0x4d693au32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4d6936u32).wrapping_add((cpu.edi as u16)); cpu.w16(0x4d6936u32, v); }
  cpu.poll(); pc = 0x4775fc; continue;
 }
 0x47766c => {
  { let v: u8 = cpu.or8((cpu.ecx as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.inc16(cpu.r16(0x4d693eu32)); cpu.w16(0x4d693eu32, v); }
  pc = 0x477675; continue;
 }
 0x477675 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_00477677(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x477677); }
 let mut pc: u32 = 0x477677;
 loop { match pc {
 0x477677 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x47769f; continue; }
  pc = 0x47767f; continue;
 }
 0x47767f => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x4776c8; continue; }
  pc = 0x477686; continue;
 }
 0x477686 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x477694; continue; }
  pc = 0x47768f; continue;
 }
 0x47768f => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x477699; continue; }
  pc = 0x477694; continue;
 }
 0x477694 => {
  { let v: u32 = 0x5b355cu32; cpu.eax = v; }
  pc = 0x477699; continue;
 }
 0x477699 => {
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.eax)), 0x2u8); }
  if cpu.zf { pc = 0x4776c8; continue; }
  pc = 0x47769f; continue;
 }
 0x47769f => {
  { let v: u32 = cpu.r32(0x4d6b84u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b7cu32, v); }
  { let v: u32 = cpu.r32(0x4d6b88u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4d6b80u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x5d2f40u32, v); }
  cpu.push(0x4776bf); f_0045b70e(cpu); if cpu.returned_to != 0x4776bf { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x5d2f40u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4776c8 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004776d0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4776d0); } r_004776d0(cpu, 0x4776d0); }
pub fn f_004776d9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4776d9); } r_004776d0(cpu, 0x4776d9); }
pub fn f_00477733(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x477733); } r_004776d0(cpu, 0x477733); }
pub fn f_0047775d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47775d); } r_004776d0(cpu, 0x47775d); }
pub fn f_00477783(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x477783); } r_004776d0(cpu, 0x477783); }
pub fn f_004777b6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4777b6); } r_004776d0(cpu, 0x4777b6); }
pub fn f_004777f7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4777f7); } r_004776d0(cpu, 0x4777f7); }
pub fn f_00477838(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x477838); } r_004776d0(cpu, 0x477838); }
pub fn f_0047786d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47786d); } r_004776d0(cpu, 0x47786d); }
pub fn f_0047789c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47789c); } r_004776d0(cpu, 0x47789c); }
pub fn f_004778c9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4778c9); } r_004776d0(cpu, 0x4778c9); }
pub fn f_004778ea(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4778ea); } r_004776d0(cpu, 0x4778ea); }
pub fn f_00477913(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x477913); } r_004776d0(cpu, 0x477913); }
pub fn f_00477941(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x477941); } r_004776d0(cpu, 0x477941); }
pub fn f_0047796d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x47796d); } r_004776d0(cpu, 0x47796d); }
pub fn f_00477999(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x477999); } r_004776d0(cpu, 0x477999); }
pub fn f_004779cf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4779cf); } r_004776d0(cpu, 0x4779cf); }
pub fn f_00477a01(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x477a01); } r_004776d0(cpu, 0x477a01); }
pub fn f_00477a31(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x477a31); } r_004776d0(cpu, 0x477a31); }
pub fn f_00477a5c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x477a5c); } r_004776d0(cpu, 0x477a5c); }
pub fn f_00477a88(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x477a88); } r_004776d0(cpu, 0x477a88); }
pub fn f_00477aaf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x477aaf); } r_004776d0(cpu, 0x477aaf); }
pub fn f_00477ae2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x477ae2); } r_004776d0(cpu, 0x477ae2); }
pub fn f_00477b0b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x477b0b); } r_004776d0(cpu, 0x477b0b); }
pub fn f_00477b3a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x477b3a); } r_004776d0(cpu, 0x477b3a); }
pub fn f_00477b65(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x477b65); } r_004776d0(cpu, 0x477b65); }
fn r_004776d0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4776d0 => {
  cpu.push(0x4776d5); f_004852ec(cpu); if cpu.returned_to != 0x4776d5 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x4776f6; continue;
 }
 0x4776d9 => {
  if cpu.of { pc = 0x47774d; continue; }
  pc = 0x4776db; continue;
 }
 0x4776db => {
  cpu.unimplemented(0x4776db, "Outsd");
  cpu.unimplemented(0x4776dc, "Arpl");
  if !cpu.cf { pc = 0x477740; continue; }
  pc = 0x4776e1; continue;
 }
 0x4776e1 => {
  cpu.unimplemented(0x4776e1, "Arpl");
  if cpu.zf { pc = 0x47775b; continue; }
  pc = 0x4776e6; continue;
 }
 0x4776e6 => {
  if cpu.cf { pc = 0x47774d; continue; }
  pc = 0x4776e8; continue;
 }
 0x4776e8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x477751; continue; }
  pc = 0x4776ec; continue;
 }
 0x4776ec => {
  if cpu.cf { pc = 0x477760; continue; }
  pc = 0x4776ee; continue;
 }
 0x4776ee => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x736569u32); cpu.esi = v; }
  pc = 0x4776f6; continue;
 }
 0x4776f6 => {
  { let v: u32 = 0x4776d9u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x477707); f_004790d5(cpu); if cpu.returned_to != 0x477707 { return; }
  { cpu.sub8(cpu.r8(0x4e155au32), 0x0u8); }
  if !cpu.zf { pc = 0x477b64; continue; }
  pc = 0x477714; continue;
 }
 0x477714 => {
  { cpu.sub16(cpu.r16(0x4d69d8u32), 0x0u16); }
  if cpu.zf { pc = 0x477b64; continue; }
  pc = 0x477722; continue;
 }
 0x477722 => {
  { cpu.sub8(cpu.r8(0x4e1557u32), 0x0u8); }
  if !cpu.zf { pc = 0x477b64; continue; }
  pc = 0x47772f; continue;
 }
 0x47772f => {
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477748; continue;
 }
 0x477733 => {
  if !cpu.cf { pc = 0x47779a; continue; }
  pc = 0x477735; continue;
 }
 0x477735 => {
  if cpu.zf { pc = 0x477796; continue; }
  pc = 0x477737; continue;
 }
 0x477737 => {
  if !cpu.zf { pc = 0x4777a7; continue; }
  pc = 0x477739; continue;
 }
 0x477739 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x74u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(2))), 0x69727265u32); cpu.esi = v; }
  if cpu.zf { pc = 0x4777b2; continue; }
  pc = 0x477743; continue;
 }
 0x477740 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x736569u32); cpu.esi = v; }
  pc = 0x477748; continue;
 }
 0x477743 => {
  if cpu.cf { pc = 0x4777ae; continue; }
  pc = 0x477745; continue;
 }
 0x477745 => {
  if !cpu.cf { pc = 0x477748; continue; }
  pc = 0x477748; continue;
 }
 0x477748 => {
  { let v: u32 = 0x477733u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x477759); f_00479ba4(cpu); if cpu.returned_to != 0x477759 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x47775b; continue;
 }
 0x47774d => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), ((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  if (!cpu.cf && !cpu.zf) { pc = 0x477798; continue; }
  pc = 0x477751; continue;
 }
 0x477751 => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffff9du32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff9du32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x477759); f_00479ba4(cpu); if cpu.returned_to != 0x477759 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x47775b; continue;
 }
 0x47775b => {
  pc = 0x47776e; continue;
 }
 0x47775d => {
  { cpu.popad(); }
  if cpu.zf { pc = 0x4777d4; continue; }
  pc = 0x477760; continue;
 }
 0x477760 => {
  { cpu.popad(); }
  cpu.unimplemented(0x477761, "Arpl");
  if cpu.zf { pc = 0x4777ce; continue; }
  pc = 0x477766; continue;
 }
 0x477766 => {
  { cpu.popad(); }
  if cpu.zf { pc = 0x4777c8; continue; }
  pc = 0x477769; continue;
 }
 0x477769 => {
  cpu.unimplemented(0x477769, "Outsw");
  if cpu.cf { pc = 0x4777e1; continue; }
  pc = 0x47776d; continue;
 }
 0x47776d => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.eax, 0x5ce90fu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x4777be; continue; }
  pc = 0x477777; continue;
 }
 0x47776e => {
  { let v: u32 = 0x47775du32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x47777f); f_0045463b(cpu); if cpu.returned_to != 0x47777f { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477798; continue;
 }
 0x477777 => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffff9du32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff9du32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x47777f); f_0045463b(cpu); if cpu.returned_to != 0x47777f { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477798; continue;
 }
 0x477783 => {
  if !cpu.cf { pc = 0x4777ea; continue; }
  pc = 0x477785; continue;
 }
 0x477785 => {
  cpu.unimplemented(0x477785, "Outsb");
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.cf { pc = 0x4777f9; continue; }
  pc = 0x47778a; continue;
 }
 0x47778a => {
  cpu.unimplemented(0x47778a, "Bound");
  if !cpu.cf { pc = 0x4777ee; continue; }
  pc = 0x47778f; continue;
 }
 0x47778f => {
  if cpu.zf { pc = 0x477800; continue; }
  pc = 0x477791; continue;
 }
 0x477791 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x477792, "Insb");
  { cpu.popad(); }
  if !cpu.cf { pc = 0x477798; continue; }
  pc = 0x477798; continue;
 }
 0x477796 => {
  if !cpu.cf { pc = 0x477798; continue; }
  pc = 0x477798; continue;
 }
 0x477798 => {
  { let v: u32 = 0x477783u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x4777a9); f_00477d03(cpu); if cpu.returned_to != 0x4777a9 { return; }
  { cpu.sub8(cpu.r8(0x5d3985u32), 0x0u8); }
  if cpu.zf { pc = 0x4777e7; continue; }
  pc = 0x4777b2; continue;
 }
 0x47779a => {
  cpu.unimplemented(0x47779a, "Psubsw");
  if (!cpu.cf && !cpu.zf) { pc = 0x4777e8; continue; }
  pc = 0x4777a1; continue;
 }
 0x4777a1 => {
  { let v: u8 = cpu.r8(0xffffff9du32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xffffff9du32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x4777a9); f_00477d03(cpu); if cpu.returned_to != 0x4777a9 { return; }
  { cpu.sub8(cpu.r8(0x5d3985u32), 0x0u8); }
  if cpu.zf { pc = 0x4777e7; continue; }
  pc = 0x4777b2; continue;
 }
 0x4777a7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8(cpu.r8(0x5d3985u32), 0x0u8); }
  if cpu.zf { pc = 0x4777e7; continue; }
  pc = 0x4777b2; continue;
 }
 0x4777ae => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x4777e7; continue; }
  pc = 0x4777b2; continue;
 }
 0x4777b2 => {
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x4777cc; continue;
 }
 0x4777b6 => {
  if cpu.zf { pc = 0x47781d; continue; }
  pc = 0x4777b8; continue;
 }
 0x4777b8 => {
  if cpu.cf { pc = 0x47782c; continue; }
  pc = 0x4777ba; continue;
 }
 0x4777ba => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x61725f79u32); cpu.esi = v; }
  pc = 0x4777c2; continue;
 }
 0x4777be => {
  if !cpu.sf { pc = 0x47781f; continue; }
  pc = 0x4777c0; continue;
 }
 0x4777c0 => {
  if cpu.cf { pc = 0x477823; continue; }
  pc = 0x4777c2; continue;
 }
 0x4777c2 => {
  { cpu.popad(); }
  if cpu.cf { pc = 0x477825; continue; }
  pc = 0x4777c6; continue;
 }
 0x4777c6 => {
  cpu.unimplemented(0x4777c6, "Arpl");
  cpu.unimplemented(0x4777c9, "Arpl");
  pc = 0x4777cc; continue;
 }
 0x4777c8 => {
  cpu.unimplemented(0x4777c8, "Arpl");
  pc = 0x4777cc; continue;
 }
 0x4777cc => {
  { let v: u32 = 0x4777b6u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x4777dd); f_00477efa(cpu); if cpu.returned_to != 0x4777dd { return; }
  cpu.push(0x4777e2); f_00485770(cpu); if cpu.returned_to != 0x4777e2 { return; }
  cpu.push(0x4777e7); f_00485841(cpu); if cpu.returned_to != 0x4777e7 { return; }
  pc = 0x4777e7; continue;
 }
 0x4777ce => {
  cpu.unimplemented(0x4777ce, "Psubsw");
  if (!cpu.cf && !cpu.zf) { pc = 0x47781c; continue; }
  pc = 0x4777d5; continue;
 }
 0x4777d4 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x4777d5; continue;
 }
 0x4777d5 => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffff9du32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff9du32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x4777dd); f_00477efa(cpu); if cpu.returned_to != 0x4777dd { return; }
  cpu.push(0x4777e2); f_00485770(cpu); if cpu.returned_to != 0x4777e2 { return; }
  cpu.push(0x4777e7); f_00485841(cpu); if cpu.returned_to != 0x4777e7 { return; }
  pc = 0x4777e7; continue;
 }
 0x4777e1 => {
  { let v: u8 = cpu.add8((cpu.eax as u8), ((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x4777e6; continue; }
  pc = 0x4777e6; continue;
 }
 0x4777e6 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x4777e8; continue;
 }
 0x4777e7 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x5d39e0u32)); cpu.w32(0x5d39e0u32, v); }
  if !cpu.sf { pc = 0x477b64; continue; }
  pc = 0x4777f3; continue;
 }
 0x4777e8 => {
  { let v: u32 = cpu.or32(cpu.eax, 0x5d39e0u32); cpu.eax = v; }
  if !cpu.sf { pc = 0x477b64; continue; }
  pc = 0x4777f3; continue;
 }
 0x4777ea => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), cpu.ebx); }
  if !cpu.sf { pc = 0x477b64; continue; }
  pc = 0x4777f3; continue;
 }
 0x4777ee => {
  { let v: u32 = cpu.esi; cpu.w32(0x3u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4777f3; continue;
 }
 0x4777f3 => {
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477809; continue;
 }
 0x4777f7 => {
  if cpu.cf { pc = 0x47785e; continue; }
  pc = 0x4777f9; continue;
 }
 0x4777f9 => {
  if !cpu.cf { pc = 0x477860; continue; }
  pc = 0x4777fb; continue;
 }
 0x4777fb => {
  if cpu.zf { pc = 0x47785c; continue; }
  pc = 0x4777fd; continue;
 }
 0x4777fd => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x74u32.wrapping_add(cpu.esi)), 0x645f6c65u32); cpu.ebp = v; }
  cpu.unimplemented(0x477804, "Insb");
  { cpu.popad(); }
  if !cpu.sf { pc = 0x477809; continue; }
  pc = 0x477809; continue;
 }
 0x477800 => {
  cpu.unimplemented(0x477800, "Insb");
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x477803, "Insb");
  { cpu.popad(); }
  if !cpu.sf { pc = 0x477809; continue; }
  pc = 0x477809; continue;
 }
 0x477809 => {
  { let v: u32 = 0x4777f7u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x47781a); f_00479d06(cpu); if cpu.returned_to != 0x47781a { return; }
  { let v: u8 = cpu.r8(0x5d3d3eu32) ^ 0x1u8; cpu.w8(0x5d3d3eu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b02e4u32, v); }
  { cpu.sub8(cpu.r8(0x5d3985u32), 0x0u8); }
  if !cpu.zf { pc = 0x477869; continue; }
  pc = 0x477834; continue;
 }
 0x47781c => {
  { cpu.sub32(cpu.eax, 0xc701005du32); }
  { let v: u32 = cpu.eax.wrapping_add(0x5b02e4u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8(cpu.r8(0x5d3985u32), 0x0u8); }
  if !cpu.zf { pc = 0x477869; continue; }
  pc = 0x477834; continue;
 }
 0x47781d => {
  { cpu.sub32(cpu.eax, 0xc701005du32); }
  { let v: u32 = cpu.eax.wrapping_add(0x5b02e4u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8(cpu.r8(0x5d3985u32), 0x0u8); }
  if !cpu.zf { pc = 0x477869; continue; }
  pc = 0x477834; continue;
 }
 0x47781f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5b02e4u32, v); }
  { cpu.sub8(cpu.r8(0x5d3985u32), 0x0u8); }
  if !cpu.zf { pc = 0x477869; continue; }
  pc = 0x477834; continue;
 }
 0x477823 => {
  { let v: u8 = cpu.port_in(0x2u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x477825; continue;
 }
 0x477825 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x5d39853du32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x5d39853du32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.zf { pc = 0x477869; continue; }
  pc = 0x477834; continue;
 }
 0x47782c => {
  { cpu.sub32(cpu.eax, 0x5d3985u32); }
  { let v: u8 = cpu.add8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), ((cpu.edx >> 8) as u8)); cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x477834; continue;
 }
 0x477834 => {
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x47784e; continue;
 }
 0x477838 => {
  if cpu.zf { pc = 0x47789f; continue; }
  pc = 0x47783a; continue;
 }
 0x47783a => {
  if cpu.cf { pc = 0x4778ae; continue; }
  pc = 0x47783c; continue;
 }
 0x47783c => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x61725f79u32); cpu.esi = v; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x4778a7; continue; }
  pc = 0x477848; continue;
 }
 0x477848 => {
  cpu.unimplemented(0x477848, "Arpl");
  cpu.unimplemented(0x47784b, "Arpl");
  pc = 0x47784e; continue;
 }
 0x47784e => {
  { let v: u32 = 0x477838u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x47785f); f_00477efa(cpu); if cpu.returned_to != 0x47785f { return; }
  cpu.push(0x477864); f_00485770(cpu); if cpu.returned_to != 0x477864 { return; }
  cpu.push(0x477869); f_00485841(cpu); if cpu.returned_to != 0x477869 { return; }
  pc = 0x477869; continue;
 }
 0x47785c => {
  { cpu.push(0x2b); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x477864); f_00485770(cpu); if cpu.returned_to != 0x477864 { return; }
  cpu.push(0x477869); f_00485841(cpu); if cpu.returned_to != 0x477869 { return; }
  pc = 0x477869; continue;
 }
 0x47785e => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x477860; continue;
 }
 0x477860 => {
  { let v: u8 = (cpu.eax as u8) | 0xdfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x477869); f_00485841(cpu); if cpu.returned_to != 0x477869 { return; }
  pc = 0x477869; continue;
 }
 0x477869 => {
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477887; continue;
 }
 0x47786d => {
  cpu.unimplemented(0x47786d, "Insd");
  cpu.unimplemented(0x47786e, "Outsd");
  cpu.unimplemented(0x47786f, "Outsb");
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x6d6f685fu32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x4778e1; continue; }
  pc = 0x47787c; continue;
 }
 0x47787c => {
  if cpu.cf { pc = 0x4778f0; continue; }
  pc = 0x47787e; continue;
 }
 0x47787e => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x20736569u32); cpu.esi = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.eax, 0x5ce90fu32); cpu.eax = v; }
  cpu.unimplemented(0x47788d, "Insd");
  if cpu.sf { pc = 0x4778d7; continue; }
  pc = 0x477890; continue;
 }
 0x477887 => {
  { let v: u32 = 0x47786du32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x477898); f_0048cc45(cpu); if cpu.returned_to != 0x477898 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x4778b4; continue;
 }
 0x477890 => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffff9du32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff9du32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x477898); f_0048cc45(cpu); if cpu.returned_to != 0x477898 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x4778b4; continue;
 }
 0x47789c => {
  { cpu.popad(); }
  { let v = 0x75u32; cpu.push(v); }
  if !cpu.cf { pc = 0x477916; continue; }
  pc = 0x4778a2; continue;
 }
 0x47789f => {
  if !cpu.zf { pc = 0x477914; continue; }
  pc = 0x4778a1; continue;
 }
 0x4778a1 => {
  if cpu.zf { pc = 0x477902; continue; }
  pc = 0x4778a3; continue;
 }
 0x4778a2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x4778a3; continue;
 }
 0x4778a3 => {
  cpu.unimplemented(0x4778a3, "Outsw");
  if cpu.cf { pc = 0x477906; continue; }
  pc = 0x4778a7; continue;
 }
 0x4778a7 => {
  if !cpu.zf { pc = 0x477917; continue; }
  pc = 0x4778a9; continue;
 }
 0x4778a9 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x62u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(2))), 0x6e616c61u32); cpu.esi = v; }
  cpu.unimplemented(0x4778b1, "Arpl");
  pc = 0x4778b4; continue;
 }
 0x4778ae => {
  cpu.unimplemented(0x4778ae, "Insb");
  { cpu.popad(); }
  cpu.unimplemented(0x4778b0, "Outsb");
  cpu.unimplemented(0x4778b1, "Arpl");
  pc = 0x4778b4; continue;
 }
 0x4778b4 => {
  { let v: u32 = 0x47789cu32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x4778c5); f_0048cbe8(cpu); if cpu.returned_to != 0x4778c5 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x4778d5; continue;
 }
 0x4778c9 => {
  if !cpu.cf { pc = 0x47793b; continue; }
  pc = 0x4778cb; continue;
 }
 0x4778cb => {
  cpu.unimplemented(0x4778cb, "Insb");
  { let v: u32 = cpu.imul2_32(cpu.r32(0x75u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(2))), 0x7374696eu32); cpu.esi = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.eax, 0x5ce90fu32); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  if cpu.sf { pc = 0x477925; continue; }
  pc = 0x4778de; continue;
 }
 0x4778d5 => {
  { let v: u32 = 0x4778c9u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  pc = 0x4778e1; continue;
 }
 0x4778d7 => {
  cpu.unimplemented(0x4778d7, "Psubsw");
  if cpu.sf { pc = 0x477925; continue; }
  pc = 0x4778de; continue;
 }
 0x4778de => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffff9du32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff9du32.wrapping_add(cpu.ecx), v); }
  pc = 0x4778e1; continue;
 }
 0x4778e1 => {
  cpu.push(0x4778e6); f_00477b65(cpu); if cpu.returned_to != 0x4778e6 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x4778fe; continue;
 }
 0x4778ea => {
  { cpu.popad(); }
  if cpu.cf { pc = 0x477954; continue; }
  pc = 0x4778ef; continue;
 }
 0x4778ef => {
  if !cpu.cf { pc = 0x477964; continue; }
  pc = 0x4778f1; continue;
 }
 0x4778f0 => {
  if !cpu.cf { pc = 0x47795b; continue; }
  pc = 0x4778f2; continue;
 }
 0x4778f1 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x65u32.wrapping_add(cpu.esi)), 0x70706f5fu32); cpu.esi = v; }
  pc = 0x4778f8; continue;
 }
 0x4778f2 => {
  if (cpu.cf || cpu.zf) { pc = 0x477959; continue; }
  pc = 0x4778f4; continue;
 }
 0x4778f4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x4778f5, "Outsd");
  if cpu.of { pc = 0x477968; continue; }
  pc = 0x4778f8; continue;
 }
 0x4778f8 => {
  cpu.unimplemented(0x4778f8, "Outsd");
  cpu.unimplemented(0x4778f9, "Outsb");
  cpu.unimplemented(0x4778fa, "Outsb");
  if cpu.zf { pc = 0x4778fe; continue; }
  pc = 0x4778fe; continue;
 }
 0x4778fe => {
  { let v: u32 = 0x4778eau32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x47790f); f_00454280(cpu); if cpu.returned_to != 0x47790f { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x47792c; continue;
 }
 0x477902 => {
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  { let v: u8 = cpu.add8((cpu.edx as u8), ((cpu.ecx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x47794e; continue; }
  pc = 0x477907; continue;
 }
 0x477906 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x477907; continue;
 }
 0x477907 => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffff9du32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff9du32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x47790f); f_00454280(cpu); if cpu.returned_to != 0x47790f { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x47792c; continue;
 }
 0x477913 => {
  cpu.unimplemented(0x477913, "Arpl");
  pc = 0x477916; continue;
 }
 0x477914 => {
  { let v = 0x5f6b6365u32; cpu.push(v); }
  cpu.unimplemented(0x477919, "Outsw");
  if cpu.cf { pc = 0x47797c; continue; }
  pc = 0x47791d; continue;
 }
 0x477916 => {
  cpu.unimplemented(0x477916, "Arpl");
  cpu.unimplemented(0x477919, "Outsw");
  if cpu.cf { pc = 0x47797c; continue; }
  pc = 0x47791d; continue;
 }
 0x477917 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x66u32.wrapping_add(cpu.edi)), 0x6fu32); cpu.ebx = v; }
  if cpu.cf { pc = 0x47797c; continue; }
  pc = 0x47791d; continue;
 }
 0x47791d => {
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x477922, "Bound");
  pc = 0x477925; continue;
 }
 0x477925 => {
  cpu.unimplemented(0x477925, "Insb");
  { let v: u32 = cpu.imul2_32(cpu.r32(0x67u32.wrapping_add(cpu.esi).wrapping_add(cpu.fs_base)), 0x5c70073u32); cpu.ebp = v; }
  cpu.unimplemented(0x47792e, "Psubsw");
  if !cpu.sf { pc = 0x47797c; continue; }
  pc = 0x477935; continue;
 }
 0x47792c => {
  { let v: u32 = 0x477913u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x47793d); f_00478b03(cpu); if cpu.returned_to != 0x47793d { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477958; continue;
 }
 0x477935 => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffff9du32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff9du32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x47793d); f_00478b03(cpu); if cpu.returned_to != 0x47793d { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477958; continue;
 }
 0x47793b => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477958; continue;
 }
 0x477941 => {
  cpu.unimplemented(0x477941, "Arpl");
  cpu.unimplemented(0x477944, "Arpl");
  cpu.unimplemented(0x477947, "Outsw");
  if cpu.cf { pc = 0x4779aa; continue; }
  pc = 0x47794b; continue;
 }
 0x47794b => {
  { cpu.popad(); }
  pc = 0x47794e; continue;
 }
 0x47794e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x477950, "Bound");
  if !cpu.cf { pc = 0x477958; continue; }
  pc = 0x477958; continue;
 }
 0x477954 => {
  if !cpu.cf { pc = 0x477958; continue; }
  pc = 0x477958; continue;
 }
 0x477958 => {
  { let v: u32 = 0x477941u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  pc = 0x477964; continue;
 }
 0x477959 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x5ce90fu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  if !cpu.sf { pc = 0x4779a8; continue; }
  pc = 0x477961; continue;
 }
 0x47795b => {
  cpu.call(0x798879bc); return;
 }
 0x477961 => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffff9du32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff9du32.wrapping_add(cpu.ecx), v); }
  pc = 0x477964; continue;
 }
 0x477964 => {
  cpu.push(0x477969); f_00478c01(cpu); if cpu.returned_to != 0x477969 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477984; continue;
 }
 0x477968 => {
  { let v: u8 = cpu.add8(cpu.r8(0x686317ebu32.wrapping_add(cpu.eax)), (cpu.ebx as u8)); cpu.w8(0x686317ebu32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x47796f, "Arpl");
  cpu.unimplemented(0x477973, "Outsw");
  if cpu.cf { pc = 0x4779d6; continue; }
  pc = 0x477977; continue;
 }
 0x47796d => {
  cpu.unimplemented(0x47796d, "Arpl");
  cpu.unimplemented(0x477970, "Arpl");
  cpu.unimplemented(0x477973, "Outsw");
  if cpu.cf { pc = 0x4779d6; continue; }
  pc = 0x477977; continue;
 }
 0x477977 => {
  cpu.unimplemented(0x477977, "Bound");
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x47797d, "Arpl");
  cpu.unimplemented(0x477980, "Outsb");
  if !cpu.cf { pc = 0x477984; continue; }
  pc = 0x477984; continue;
 }
 0x47797c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x47797d, "Arpl");
  cpu.unimplemented(0x477980, "Outsb");
  if !cpu.cf { pc = 0x477984; continue; }
  pc = 0x477984; continue;
 }
 0x477984 => {
  { let v: u32 = 0x47796du32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x477995); f_00478a64(cpu); if cpu.returned_to != 0x477995 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x4779b4; continue;
 }
 0x477999 => {
  cpu.unimplemented(0x477999, "Arpl");
  cpu.unimplemented(0x47799c, "Arpl");
  if cpu.of { pc = 0x477a13; continue; }
  pc = 0x4779a1; continue;
 }
 0x4779a1 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi)), 0x5f797469u32); cpu.ebp = v; }
  pc = 0x4779a8; continue;
 }
 0x4779a8 => {
  if cpu.zf { pc = 0x477a0f; continue; }
  pc = 0x4779aa; continue;
 }
 0x4779aa => {
  if cpu.cf { pc = 0x477a1e; continue; }
  pc = 0x4779ac; continue;
 }
 0x4779ac => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x736569u32); cpu.esi = v; }
  pc = 0x4779b4; continue;
 }
 0x4779b4 => {
  { let v: u32 = 0x477999u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x4779c5); f_00478322(cpu); if cpu.returned_to != 0x4779c5 { return; }
  if cpu.cf { pc = 0x477b64; continue; }
  pc = 0x4779cb; continue;
 }
 0x4779cb => {
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x4779ec; continue;
 }
 0x4779cf => {
  cpu.unimplemented(0x4779cf, "Arpl");
  cpu.unimplemented(0x4779d2, "Arpl");
  cpu.unimplemented(0x4779d5, "Bound");
  pc = 0x4779d8; continue;
 }
 0x4779d6 => {
  if !cpu.zf { pc = 0x477a41; continue; }
  pc = 0x4779d8; continue;
 }
 0x4779d8 => {
  cpu.unimplemented(0x4779d8, "Insb");
  { let v: u32 = cpu.imul2_32(cpu.r32(0x67u32.wrapping_add(cpu.esi).wrapping_add(cpu.fs_base)), 0x6e755f73u32); cpu.ebp = v; }
  if cpu.cf { pc = 0x477a44; continue; }
  pc = 0x4779e5; continue;
 }
 0x4779e5 => {
  { cpu.popad(); }
  if cpu.zf { pc = 0x477a5c; continue; }
  pc = 0x4779e8; continue;
 }
 0x4779e8 => {
  { cpu.popad(); }
  cpu.unimplemented(0x4779e9, "Arpl");
  pc = 0x4779ec; continue;
 }
 0x4779ec => {
  { let v: u32 = 0x4779cfu32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x4779fd); f_004784c0(cpu); if cpu.returned_to != 0x4779fd { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477a1c; continue;
 }
 0x477a01 => {
  if cpu.cf { pc = 0x477a68; continue; }
  pc = 0x477a03; continue;
 }
 0x477a03 => {
  cpu.unimplemented(0x477a03, "Arpl");
  if cpu.zf { pc = 0x477a7d; continue; }
  pc = 0x477a08; continue;
 }
 0x477a08 => {
  if cpu.cf { pc = 0x477a6f; continue; }
  pc = 0x477a0a; continue;
 }
 0x477a0a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.unimplemented(0x477a0b, "Insb");
  cpu.unimplemented(0x477a0c, "Outsd");
  if !cpu.cf { pc = 0x477a83; continue; }
  pc = 0x477a0f; continue;
 }
 0x477a0f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x477a77; continue; }
  pc = 0x477a12; continue;
 }
 0x477a12 => {
  if cpu.cf { pc = 0x477a86; continue; }
  pc = 0x477a14; continue;
 }
 0x477a13 => {
  if cpu.cf { pc = 0x477a7e; continue; }
  pc = 0x477a15; continue;
 }
 0x477a14 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x736569u32); cpu.esi = v; }
  pc = 0x477a1c; continue;
 }
 0x477a15 => {
  if cpu.zf { pc = 0x477a86; continue; }
  pc = 0x477a17; continue;
 }
 0x477a17 => {
  if cpu.cf { pc = 0x477a82; continue; }
  pc = 0x477a19; continue;
 }
 0x477a19 => {
  if !cpu.cf { pc = 0x477a1c; continue; }
  pc = 0x477a1c; continue;
 }
 0x477a1c => {
  { let v: u32 = 0x477a01u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x477a2d); f_004786c5(cpu); if cpu.returned_to != 0x477a2d { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477a47; continue;
 }
 0x477a1e => {
  cpu.unimplemented(0x477a1e, "Psubsw");
  if cpu.pf() { pc = 0x477a6c; continue; }
  pc = 0x477a25; continue;
 }
 0x477a25 => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffff9du32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff9du32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x477a2d); f_004786c5(cpu); if cpu.returned_to != 0x477a2d { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477a47; continue;
 }
 0x477a31 => {
  cpu.unimplemented(0x477a31, "Arpl");
  if cpu.cf { pc = 0x477a96; continue; }
  pc = 0x477a37; continue;
 }
 0x477a37 => {
  if cpu.cf { pc = 0x477a9e; continue; }
  pc = 0x477a39; continue;
 }
 0x477a39 => {
  cpu.unimplemented(0x477a39, "Arpl");
  if cpu.zf { pc = 0x477ab3; continue; }
  pc = 0x477a3e; continue;
 }
 0x477a3e => {
  if cpu.cf { pc = 0x477aa5; continue; }
  pc = 0x477a40; continue;
 }
 0x477a40 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x477a41; continue;
 }
 0x477a41 => {
  cpu.unimplemented(0x477a41, "Insb");
  { cpu.popad(); }
  pc = 0x477a44; continue;
 }
 0x477a44 => {
  if !cpu.cf { pc = 0x477a47; continue; }
  pc = 0x477a47; continue;
 }
 0x477a47 => {
  { let v: u32 = 0x477a31u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x477a58); f_00479048(cpu); if cpu.returned_to != 0x477a58 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477a73; continue;
 }
 0x477a5c => {
  cpu.unimplemented(0x477a5c, "Arpl");
  cpu.unimplemented(0x477a5f, "Arpl");
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.of { pc = 0x477ad4; continue; }
  pc = 0x477a68; continue;
 }
 0x477a68 => {
  { cpu.popad(); }
  if !cpu.sf { pc = 0x477ad0; continue; }
  pc = 0x477a6b; continue;
 }
 0x477a6b => {
  if cpu.cf { pc = 0x477acc; continue; }
  pc = 0x477a6d; continue;
 }
 0x477a6c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x477a6d; continue;
 }
 0x477a6d => {
  if !cpu.zf { pc = 0x477add; continue; }
  pc = 0x477a6f; continue;
 }
 0x477a6f => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(2))), 0xe90f05c7u32); cpu.esi = v; }
  pc = 0x477a77; continue;
 }
 0x477a73 => {
  { let v: u32 = 0x477a5cu32; cpu.w32(0x5ce90fu32, v); }
  pc = 0x477a7d; continue;
 }
 0x477a77 => {
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(2))).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(2)), v); }
  { let v: u8 = cpu.add8(cpu.r8(0xffffff9du32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff9du32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x477a84); f_00479237(cpu); if cpu.returned_to != 0x477a84 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477a86; continue;
 }
 0x477a7d => {
  { cpu.popad(); }
  pc = 0x477a7e; continue;
 }
 0x477a7e => {
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x477a84); f_00479237(cpu); if cpu.returned_to != 0x477a84 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477a86; continue;
 }
 0x477a82 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477a86; continue;
 }
 0x477a83 => {
  { let v: u8 = cpu.r8(0x686312ebu32.wrapping_add(cpu.eax)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x686312ebu32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x477a8a, "Arpl");
  if !cpu.zf { pc = 0x477afe; continue; }
  pc = 0x477a90; continue;
 }
 0x477a86 => {
  pc = 0x477a9a; continue;
 }
 0x477a88 => {
  cpu.unimplemented(0x477a88, "Arpl");
  cpu.unimplemented(0x477a8b, "Arpl");
  if !cpu.zf { pc = 0x477afe; continue; }
  pc = 0x477a90; continue;
 }
 0x477a90 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x5fu32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(2))), 0x656d6f68u32); cpu.esi = v; }
  if !cpu.cf { pc = 0x477a9a; continue; }
  pc = 0x477a9a; continue;
 }
 0x477a96 => {
  cpu.unimplemented(0x477a96, "Insd");
  if !cpu.cf { pc = 0x477a9a; continue; }
  pc = 0x477a9a; continue;
 }
 0x477a9a => {
  { let v: u32 = 0x477a88u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  pc = 0x477aa5; continue;
 }
 0x477a9e => {
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x6100477au32.wrapping_add(cpu.eax)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x6100477au32.wrapping_add(cpu.eax), v); }
  pc = 0x477aa5; continue;
 }
 0x477aa5 => {
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x477aab); f_004790f3(cpu); if cpu.returned_to != 0x477aab { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477acd; continue;
 }
 0x477aaf => {
  cpu.unimplemented(0x477aaf, "Arpl");
  cpu.unimplemented(0x477ab2, "Arpl");
  cpu.unimplemented(0x477ab5, "Outsw");
  if cpu.cf { pc = 0x477b18; continue; }
  pc = 0x477ab9; continue;
 }
 0x477ab3 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x66u32.wrapping_add(cpu.edi)), 0x6fu32); cpu.ebx = v; }
  if cpu.cf { pc = 0x477b18; continue; }
  pc = 0x477ab9; continue;
 }
 0x477ab9 => {
  if cpu.cf { pc = 0x477b20; continue; }
  pc = 0x477abb; continue;
 }
 0x477abb => {
  if cpu.of { pc = 0x477b1e; continue; }
  pc = 0x477abd; continue;
 }
 0x477abd => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x61u32.wrapping_add(cpu.edx)), 0x5f656c62u32); cpu.esi = v; }
  if (cpu.cf || cpu.zf) { pc = 0x477b2b; continue; }
  pc = 0x477ac6; continue;
 }
 0x477ac6 => {
  { let v = 0x656c6369u32; cpu.push(v); }
  if !cpu.cf { pc = 0x477acd; continue; }
  pc = 0x477acd; continue;
 }
 0x477acc => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x5ce90fu32); cpu.eax = v; }
  { cpu.scas(4, 0); }
  pc = 0x477ad4; continue;
 }
 0x477acd => {
  { let v: u32 = 0x477aafu32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x477ade); f_004787ca(cpu); if cpu.returned_to != 0x477ade { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477af6; continue;
 }
 0x477ad0 => {
  cpu.call(0x7af67b31); return;
 }
 0x477ad4 => {
  if cpu.pf() { pc = 0x477b1d; continue; }
  pc = 0x477ad6; continue;
 }
 0x477ad6 => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffff9du32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff9du32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x477ade); f_004787ca(cpu); if cpu.returned_to != 0x477ade { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477af6; continue;
 }
 0x477add => {
  { let v: u8 = cpu.r8(0x686314ebu32.wrapping_add(cpu.eax)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x686314ebu32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x477ae4, "Arpl");
  if !cpu.zf { pc = 0x477b58; continue; }
  pc = 0x477aea; continue;
 }
 0x477ae2 => {
  cpu.unimplemented(0x477ae2, "Arpl");
  cpu.unimplemented(0x477ae5, "Arpl");
  if !cpu.zf { pc = 0x477b58; continue; }
  pc = 0x477aea; continue;
 }
 0x477aea => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x5fu32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(2))), 0x67726174u32); cpu.esi = v; }
  if cpu.zf { pc = 0x477b68; continue; }
  pc = 0x477af5; continue;
 }
 0x477af5 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax.wrapping_add(0x5ce90fu32); cpu.eax = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { pc = 0x477b78; continue; }
  pc = 0x477afe; continue;
 }
 0x477af6 => {
  { let v: u32 = 0x477ae2u32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x477b07); f_004780a7(cpu); if cpu.returned_to != 0x477b07 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477b25; continue;
 }
 0x477afe => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffff9du32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff9du32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x477b07); f_004780a7(cpu); if cpu.returned_to != 0x477b07 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477b25; continue;
 }
 0x477b0b => {
  cpu.unimplemented(0x477b0b, "Arpl");
  cpu.unimplemented(0x477b0e, "Arpl");
  if cpu.cf { pc = 0x477b78; continue; }
  pc = 0x477b13; continue;
 }
 0x477b13 => {
  if cpu.of { pc = 0x477b76; continue; }
  pc = 0x477b15; continue;
 }
 0x477b15 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x5fu32.wrapping_add(cpu.edx)), 0x6564726fu32); cpu.esi = v; }
  if cpu.cf { pc = 0x477b91; continue; }
  pc = 0x477b1e; continue;
 }
 0x477b18 => {
  cpu.unimplemented(0x477b18, "Outsd");
  if cpu.cf { cpu.call(0x477b7f); return; }
  pc = 0x477b1b; continue;
 }
 0x477b1b => {
  if cpu.cf { pc = 0x477b91; continue; }
  pc = 0x477b1e; continue;
 }
 0x477b1d => {
  if !cpu.cf { cpu.call(0x477b7e); return; }
  pc = 0x477b1f; continue;
 }
 0x477b1e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x477b1f; continue;
 }
 0x477b1f => {
  if (cpu.cf || cpu.zf) { pc = 0x477b82; continue; }
  pc = 0x477b21; continue;
 }
 0x477b20 => {
  { cpu.popad(); }
  pc = 0x477b21; continue;
 }
 0x477b21 => {
  cpu.unimplemented(0x477b21, "Insb");
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffc7u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))), 0x5ce90f05u32); cpu.esp = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  if !cpu.pf() { pc = 0x477b75; continue; }
  pc = 0x477b2e; continue;
 }
 0x477b25 => {
  { let v: u32 = 0x477b0bu32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x477b36); f_0047810c(cpu); if cpu.returned_to != 0x477b36 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477b53; continue;
 }
 0x477b2b => {
  { let v: u32 = cpu.edi | cpu.r32(0x47u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  pc = 0x477b2e; continue;
 }
 0x477b2e => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffff9du32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff9du32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x477b36); f_0047810c(cpu); if cpu.returned_to != 0x477b36 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  pc = 0x477b53; continue;
 }
 0x477b3a => {
  cpu.unimplemented(0x477b3a, "Arpl");
  cpu.unimplemented(0x477b3d, "Arpl");
  cpu.unimplemented(0x477b40, "Outsb");
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x477bae; continue; }
  pc = 0x477b49; continue;
 }
 0x477b49 => {
  if cpu.cf { pc = 0x477bbd; continue; }
  pc = 0x477b4b; continue;
 }
 0x477b4b => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x736569u32); cpu.esi = v; }
  pc = 0x477b53; continue;
 }
 0x477b53 => {
  { let v: u32 = 0x477b3au32; cpu.w32(0x5ce90fu32, v); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.push(0x477b64); f_004796f2(cpu); if cpu.returned_to != 0x477b64 { return; }
  pc = 0x477b64; continue;
 }
 0x477b58 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), ((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  if !cpu.pf() { pc = 0x477ba3; continue; }
  pc = 0x477b5c; continue;
 }
 0x477b5c => {
  { let v: u8 = cpu.add8(cpu.r8(0xffffff9du32.wrapping_add(cpu.ecx)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff9du32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x477b64); f_004796f2(cpu); if cpu.returned_to != 0x477b64 { return; }
  pc = 0x477b64; continue;
 }
 0x477b64 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x477b65 => {
  { let v: u32 = cpu.r32(0x5d3d26u32); cpu.esi = v; }
  { let v: u32 = 0x5d349cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5d3d2au32); cpu.ecx = v; }
  pc = 0x477b76; continue;
 }
 0x477b68 => {
  { cpu.sub32(cpu.eax, 0x9cbf005du32); }
  { let v: u8 = (cpu.eax as u8) ^ 0x5du8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x5d3d2a0du32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x5d3d2a0du32.wrapping_add(cpu.ebx), v); }
  pc = 0x477b75; continue;
 }
 0x477b75 => {
  { let v: u8 = cpu.r8(0x524b80u32.wrapping_add(cpu.ebp)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x524b80u32.wrapping_add(cpu.ebp), v); }
  pc = 0x477b7b; continue;
 }
 0x477b76 => {
  { let v: u32 = 0x524b80u32; cpu.ebp = v; }
  pc = 0x477b7b; continue;
 }
 0x477b78 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.add8(((cpu.ebx >> 8) as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.pop(); }
  cpu.call(0x477b7d); return;
 }
 0x477b7b => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x477b82; continue;
 }
 0x477b82 => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x477bf3; continue; }
  pc = 0x477b89; continue;
 }
 0x477b89 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x477b90); f_004853e8(cpu); if cpu.returned_to != 0x477b90 { return; }
  cpu.push(0x477b95); f_0048537d(cpu); if cpu.returned_to != 0x477b95 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x477bf3; continue; }
  pc = 0x477b9d; continue;
 }
 0x477b91 => {
  cpu.push(0x477b96); cpu.call(0x89477c6d); if cpu.returned_to != 0x477b96 { return; }
  { cpu.pop(); }
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x477bf3; continue; }
  pc = 0x477b9d; continue;
 }
 0x477b9d => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u8 = cpu.r8(0x5092c1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x477bf3; continue; }
  pc = 0x477bb1; continue;
 }
 0x477ba3 => {
  { let v: u16 = cpu.adc16((cpu.ecx as u16), cpu.r16(0x5092c180u32.wrapping_add(cpu.edx))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x477bf3; continue; }
  pc = 0x477bb1; continue;
 }
 0x477bae => {
  { let v: u32 = cpu.r32(0x42u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.esi); cpu.w32(0x42u32.wrapping_add(cpu.ebp), v); }
  pc = 0x477bb1; continue;
 }
 0x477bb1 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x13u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x509290u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x26u8); }
  if cpu.zf { pc = 0x477bed; continue; }
  pc = 0x477bc4; continue;
 }
 0x477bbd => {
  { let v: u8 = cpu.r8(0x74263078u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x74263078u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x74243078u32.wrapping_add(cpu.eax)).wrapping_sub(cpu.eax); cpu.w32(0x74243078u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax & cpu.r32(0x75027978u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.esp, cpu.r32(0xffffff8bu32.wrapping_add(cpu.esi))); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x477be0); f_0047a2d9(cpu); if cpu.returned_to != 0x477be0 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x477bf3; continue; }
  pc = 0x477be7; continue;
 }
 0x477bc4 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x24u8); }
  if cpu.zf { pc = 0x477bed; continue; }
  pc = 0x477bca; continue;
 }
 0x477bca => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x2u8); }
  if !cpu.zf { pc = 0x477bf3; continue; }
  pc = 0x477bd0; continue;
 }
 0x477bd0 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x477be0); f_0047a2d9(cpu); if cpu.returned_to != 0x477be0 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x477bf3; continue; }
  pc = 0x477be7; continue;
 }
 0x477be7 => {
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x477bf3; continue; }
  pc = 0x477bed; continue;
 }
 0x477bed => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x477bf3; continue;
 }
 0x477bf3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x16u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x48u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x477b7b; continue; }
  pc = 0x477c05; continue;
 }
 0x477c05 => {
  { let v: u32 = 0x5d349cu32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x477c11; continue;
 }
 0x477c11 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x477c1b; continue; }
  pc = 0x477c19; continue;
 }
 0x477c19 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  pc = 0x477c1b; continue;
 }
 0x477c1b => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x477c11; continue; }
  pc = 0x477c22; continue;
 }
 0x477c22 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5d39ecu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5d39f0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5d39f4u32, v); }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(cpu.r8(0x524a4du32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5da9f4u32)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x5d3a00u32)); cpu.eax = v; }
  if !cpu.sf { pc = 0x477c55; continue; }
  pc = 0x477c53; continue;
 }
 0x477c53 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x477c55; continue;
 }
 0x477c55 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5d39ecu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x477c62; continue; }
  pc = 0x477c5d; continue;
 }
 0x477c5d => {
  { let v: u32 = cpu.r32(0x5d39ecu32); cpu.eax = v; }
  pc = 0x477c62; continue;
 }
 0x477c62 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5d39f8u32, v); }
  { let v: u32 = cpu.r32(0x5d39f0u32).wrapping_sub(cpu.eax); cpu.w32(0x5d39f0u32, v); }
  { let v: u32 = cpu.r32(0x5d39e4u32); cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.r32(0x5d39f0u32)); cpu.esi = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x477c8c; continue; }
  pc = 0x477c7b; continue;
 }
 0x477c7b => {
  { let v: u32 = cpu.r32(0x5d39f0u32).wrapping_add(cpu.esi); cpu.w32(0x5d39f0u32, v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.esi); cpu.eax = v; }
  if !cpu.sf { pc = 0x477c87; continue; }
  pc = 0x477c85; continue;
 }
 0x477c85 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x477c87; continue;
 }
 0x477c87 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5d39f8u32, v); }
  pc = 0x477c8c; continue;
 }
 0x477c8c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5d3d3fu32)); }
  if cpu.zf { pc = 0x477c95; continue; }
  pc = 0x477c94; continue;
 }
 0x477c94 => {
  {  }
  pc = 0x477c95; continue;
 }
 0x477c95 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5d3d3fu32, v); }
  { let v: u32 = 0x524b80u32; cpu.esi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x5d349cu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x477cab; continue;
 }
 0x477cab => {
  { cpu.sub8(cpu.r8(0x15u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x477cec; continue; }
  pc = 0x477cb1; continue;
 }
 0x477cb1 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.w8(0x5d385fu32, v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)) & 0xefu8; cpu.w8(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x477cd8; continue; }
  pc = 0x477cc8; continue;
 }
 0x477cc8 => {
  { cpu.sub8(cpu.r8(0x5d385fu32), 0x0u8); }
  if cpu.zf { pc = 0x477cec; continue; }
  pc = 0x477cd1; continue;
 }
 0x477cd1 => {
  cpu.push(0x477cd6); f_00477e9e(cpu); if cpu.returned_to != 0x477cd6 { return; }
  pc = 0x477cec; continue;
 }
 0x477cd8 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0xffffffffu32); }
  if cpu.zf { pc = 0x477cec; continue; }
  pc = 0x477ce0; continue;
 }
 0x477ce0 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x8u8); }
  if cpu.zf { pc = 0x477cec; continue; }
  pc = 0x477ce6; continue;
 }
 0x477ce6 => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)) | 0x10u8; cpu.w8(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x477cec; continue;
 }
 0x477cec => {
  { let v: u32 = cpu.esi.wrapping_add(0x16u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x477cab; continue; }
  pc = 0x477cf6; continue;
 }
 0x477cf6 => {
  { let v: u32 = cpu.edx; cpu.w32(0x5d39fcu32, v); }
  { let v: u32 = cpu.sub32(cpu.r32(0x5d39f4u32), cpu.edx); cpu.w32(0x5d39f4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
