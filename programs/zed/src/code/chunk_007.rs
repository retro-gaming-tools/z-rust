#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_0103f67d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103f67d); }
 let mut pc: u32 = 0x103f67d;
 loop { match pc {
 0x103f67d => {
  { let v: u8 = 0xffu8; cpu.w8(0x42u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103f682(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103f682); }
 let mut pc: u32 = 0x103f682;
 loop { match pc {
 0x103f682 => {
  { let v: u8 = 0xffu8; cpu.w8(0x3eu32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103f687(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103f687); }
 let mut pc: u32 = 0x103f687;
 loop { match pc {
 0x103f687 => {
  { let v: u8 = 0xffu8; cpu.w8(0x3fu32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103f68c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103f68c); }
 let mut pc: u32 = 0x103f68c;
 loop { match pc {
 0x103f68c => {
  { let v: u8 = 0xffu8; cpu.w8(0x40u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103f691(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103f691); }
 let mut pc: u32 = 0x103f691;
 loop { match pc {
 0x103f691 => {
  { let v: u8 = 0xffu8; cpu.w8(0x41u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103f696(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103f696); }
 let mut pc: u32 = 0x103f696;
 loop { match pc {
 0x103f696 => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103f6a9; continue; }
  pc = 0x103f69a; continue;
 }
 0x103f69a => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103f6b7; continue; }
  pc = 0x103f69e; continue;
 }
 0x103f69e => {
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x103f6b0; continue; }
  pc = 0x103f6a2; continue;
 }
 0x103f6a2 => {
  { let v: u32 = 0x41u32; cpu.eax = v; }
  pc = 0x103f714; continue;
 }
 0x103f6a9 => {
  { let v: u32 = 0x3eu32; cpu.eax = v; }
  pc = 0x103f6be; continue;
 }
 0x103f6b0 => {
  { let v: u32 = 0x40u32; cpu.eax = v; }
  pc = 0x103f6be; continue;
 }
 0x103f6b7 => {
  { let v: u32 = 0x3fu32; cpu.eax = v; }
  pc = 0x103f714; continue;
 }
 0x103f6be => {
  { let v: u8 = 0xffu8; cpu.w8(0x10f6ee8u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6f2cu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6f70u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6fb4u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f684eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6892u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f68d6u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f691au32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f612eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6172u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f61b6u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f61fau32.wrapping_add(cpu.eax), v); }
  pc = 0x103f768; continue;
 }
 0x103f714 => {
  { let v: u8 = 0xffu8; cpu.w8(0x10f703cu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f7080u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f70c4u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f7108u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f69a2u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f69e6u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6a2au32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6a6eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6282u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f62c6u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f630au32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f634eu32.wrapping_add(cpu.eax), v); }
  pc = 0x103f768; continue;
 }
 0x103f768 => {
  { let v: u8 = 0xffu8; cpu.w8(0x10f5fdau32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f601eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6062u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f60a6u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6d94u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6dd8u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6e1cu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6e60u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f66fau32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f673eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f6782u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10f67c6u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103f7c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103f7c0); }
 let mut pc: u32 = 0x103f7c0;
 loop { match pc {
 0x103f7c0 => {
  { let v: u8 = 0x2u8; cpu.w8(0x10ef20du32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef203u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef206u32, v); }
  cpu.push(0x103f7da); f_0103ff78(cpu); if cpu.returned_to != 0x103f7da { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3d63u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x112601cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1126350u32, v); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x103f807; continue; }
  pc = 0x103f7fa; continue;
 }
 0x103f7fa => {
  { cpu.sub8(cpu.r8(0x10e3d7fu32), 0x2u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x103f8c4; continue; }
  pc = 0x103f807; continue;
 }
 0x103f807 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef201u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef202u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef209u32, v); }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3df1u32, v); }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3df3u32, v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef20au32, v); }
  { let v: u32 = 0x1152e54u32; cpu.w32(0x10e3f88u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1126358u32, v); }
  cpu.push(0x103f857); f_0103fad4(cpu); if cpu.returned_to != 0x103f857 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3d63u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x11260e8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1126350u32, v); }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef201u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef202u32, v); }
  { let v: u8 = 0x78u8; cpu.w8(0x10ef209u32, v); }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3df1u32, v); }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3df3u32, v); }
  { let v: u32 = 0x78u32; cpu.esi = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef20au32, v); }
  { let v: u32 = 0x1152f0cu32; cpu.w32(0x10e3f88u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1126358u32, v); }
  cpu.push(0x103f8c3); f_0103fad4(cpu); if cpu.returned_to != 0x103f8c3 { return; }
  pc = 0x103f8c3; continue;
 }
 0x103f8c3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103f8c4 => {
  { let v: u32 = 0x10ee98bu32; cpu.esi = v; }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x78u32.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  pc = 0x103f8d4; continue;
 }
 0x103f8d4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103f8d4; continue; }
  pc = 0x103f8e1; continue;
 }
 0x103f8e1 => {
  { let v: u32 = 0x10eea7bu32; cpu.esi = v; }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x78u32.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  pc = 0x103f8f1; continue;
 }
 0x103f8f1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x103f8f1; continue; }
  pc = 0x103f8fe; continue;
 }
 0x103f8fe => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3d63u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x112601cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1126350u32, v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef201u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef202u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef209u32, v); }
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3df1u32, v); }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3df3u32, v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef20au32, v); }
  { let v: u32 = 0x1152e54u32; cpu.w32(0x10e3f88u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1126358u32, v); }
  cpu.push(0x103f965); f_0103fad4(cpu); if cpu.returned_to != 0x103f965 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3d63u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x11260e8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1126350u32, v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef201u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef202u32, v); }
  { let v: u8 = 0x3cu8; cpu.w8(0x10ef209u32, v); }
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3df1u32, v); }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3df3u32, v); }
  { let v: u32 = 0x3cu32; cpu.esi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef20au32, v); }
  { let v: u32 = 0x1152f0cu32; cpu.w32(0x10e3f88u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1126358u32, v); }
  cpu.push(0x103f9d1); f_0103fad4(cpu); if cpu.returned_to != 0x103f9d1 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3d63u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x11261b4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1126350u32, v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef201u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef202u32, v); }
  { let v: u8 = 0x78u8; cpu.w8(0x10ef209u32, v); }
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3df1u32, v); }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3df3u32, v); }
  { let v: u32 = 0x78u32; cpu.esi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x10ef20au32, v); }
  { let v: u32 = 0x1158494u32; cpu.w32(0x10e3f88u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1126358u32, v); }
  cpu.push(0x103fa3d); f_0103fad4(cpu); if cpu.returned_to != 0x103fa3d { return; }
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x3u16); }
  if cpu.zf { cpu.poll(); pc = 0x103f8c3; continue; }
  pc = 0x103fa4b; continue;
 }
 0x103fa4b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3d63u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1126280u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1126350u32, v); }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef201u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef202u32, v); }
  { let v: u8 = 0xb4u8; cpu.w8(0x10ef209u32, v); }
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3df1u32, v); }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3df3u32, v); }
  { let v: u32 = 0xb4u32; cpu.esi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x10ef20au32, v); }
  { let v: u32 = 0x115854cu32; cpu.w32(0x10e3f88u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1126358u32, v); }
  cpu.push(0x103fab7); f_0103fad4(cpu); if cpu.returned_to != 0x103fab7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103fab8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103fab8); }
 let mut pc: u32 = 0x103fab8;
 loop { match pc {
 0x103fab8 => {
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x103fac2; continue;
 }
 0x103fac2 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x103faca; continue; }
  pc = 0x103fac8; continue;
 }
 0x103fac8 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103faca => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103fac2; continue; }
  pc = 0x103fad2; continue;
 }
 0x103fad2 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103fad4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103fad4); }
 let mut pc: u32 = 0x103fad4;
 loop { match pc {
 0x103fad4 => {
  { let v: u32 = 0x0u32; cpu.w32(0x112634cu32, v); }
  pc = 0x103fade; continue;
 }
 0x103fade => {
  { cpu.sub8(cpu.r8(0x10ee98bu32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x103fafe; continue; }
  pc = 0x103fae7; continue;
 }
 0x103fae7 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef37bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11296e4u32, v); }
  cpu.push(0x103fafc); f_0103fb02(cpu); if cpu.returned_to != 0x103fafc { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x103fafe; continue;
 }
 0x103fafe => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x103fade; continue; }
  pc = 0x103fb01; continue;
 }
 0x103fb01 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103fb02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103fb02); }
 let mut pc: u32 = 0x103fb02;
 loop { match pc {
 0x103fb02 => {
  { let v: u32 = cpu.esi; cpu.w32(0x1126354u32, v); }
  { cpu.sub8(cpu.r8(0x11296e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x103fb49; continue; }
  pc = 0x103fb11; continue;
 }
 0x103fb11 => {
  { cpu.sub8(cpu.r8(0x10ef37bu32), 0x0u8); }
  if !cpu.zf { pc = 0x103fb3e; continue; }
  pc = 0x103fb1a; continue;
 }
 0x103fb1a => {
  { cpu.sub8(cpu.r8(0x10eea7bu32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x103fb49; continue; }
  pc = 0x103fb23; continue;
 }
 0x103fb23 => {
  { cpu.sub8(cpu.r8(0x10eea7bu32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x103fb49; continue; }
  pc = 0x103fb2c; continue;
 }
 0x103fb2c => {
  { cpu.sub8(cpu.r8(0x10eea7bu32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x103fb49; continue; }
  pc = 0x103fb35; continue;
 }
 0x103fb35 => {
  { cpu.sub8(cpu.r8(0x10eea7bu32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.zf { pc = 0x103fb49; continue; }
  pc = 0x103fb3e; continue;
 }
 0x103fb3e => {
  cpu.push(0x103fb43); f_0103ff79(cpu); if cpu.returned_to != 0x103fb43 { return; }
  cpu.push(0x103fb48); f_0103fb5b(cpu); if cpu.returned_to != 0x103fb48 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103fb49 => {
  cpu.push(0x103fb4e); f_0103ff79(cpu); if cpu.returned_to != 0x103fb4e { return; }
  cpu.push(0x103fb53); f_0103ffdb(cpu); if cpu.returned_to != 0x103fb53 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0xa5u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103fb5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103fb5b); }
 let mut pc: u32 = 0x103fb5b;
 loop { match pc {
 0x103fb5b => {
  { let v: u32 = cpu.r32(0x10e4194u32); cpu.esi = v; }
  { let v: u32 = cpu.r8(0x10ef202u32) as u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x15u8); }
  if cpu.zf { pc = 0x103fd10; continue; }
  pc = 0x103fb70; continue;
 }
 0x103fb70 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x103fd10; continue; }
  pc = 0x103fb7f; continue;
 }
 0x103fb7f => {
  { cpu.sub32(cpu.r32(0x2u32.wrapping_add(cpu.ebx)), 0x6d616c46u32); }
  if !cpu.zf { pc = 0x103fb9f; continue; }
  pc = 0x103fb88; continue;
 }
 0x103fb88 => {
  { cpu.sub16(cpu.r16(0x6u32.wrapping_add(cpu.ebx)), 0x7265u16); }
  if !cpu.zf { pc = 0x103fb9f; continue; }
  pc = 0x103fb90; continue;
 }
 0x103fb90 => {
  { let v: u32 = 0x6f727950u32; cpu.w32(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x73u8; cpu.w8(0x6u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7u32.wrapping_add(cpu.ebx), v); }
  pc = 0x103fb9f; continue;
 }
 0x103fb9f => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x103fbe8; continue; }
  pc = 0x103fba3; continue;
 }
 0x103fba3 => {
  { let v: u32 = cpu.add32(cpu.r32(0x1126350u32), 0x4u32); cpu.w32(0x1126350u32, v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x103fbb0); f_0103fdc2(cpu); if cpu.returned_to != 0x103fbb0 { return; }
  cpu.push(0x103fbb5); f_010404bf(cpu); if cpu.returned_to != 0x103fbb5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x103fbe6; continue; }
  pc = 0x103fbb8; continue;
 }
 0x103fbb8 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3cu32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x10ef37bu32), 0x0u8); }
  if cpu.zf { pc = 0x103fbce; continue; }
  pc = 0x103fbca; continue;
 }
 0x103fbca => {
  { let v: u8 = 0xffu8; cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x103fbce; continue;
 }
 0x103fbce => {
  { let v: u8 = cpu.r8(0x48u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x41u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x103fbd9); f_01040152(cpu); if cpu.returned_to != 0x103fbd9 { return; }
  if cpu.cf { pc = 0x103fbe6; continue; }
  pc = 0x103fbdb; continue;
 }
 0x103fbdb => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10ef202u32)); cpu.w8(0x10ef202u32, v); }
  cpu.poll(); pc = 0x103fb5b; continue;
 }
 0x103fbe6 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103fbe8 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ef201u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x10ef209u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10eea7bu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4194u32); cpu.esi = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v = cpu.r8(0x1cu32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x55f0u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8((cpu.edx as u8), 0xau8); }
  if cpu.zf { pc = 0x103fc84; continue; }
  pc = 0x103fc36; continue;
 }
 0x103fc36 => {
  { cpu.sub8((cpu.edx as u8), 0xeu8); }
  if cpu.zf { pc = 0x103fcca; continue; }
  pc = 0x103fc3f; continue;
 }
 0x103fc3f => {
  cpu.push(0x103fc44); f_0103fdc2(cpu); if cpu.returned_to != 0x103fc44 { return; }
  { cpu.sub8(cpu.r8(0x10ef37bu32), 0x0u8); }
  if !cpu.zf { pc = 0x103fc69; continue; }
  pc = 0x103fc4d; continue;
 }
 0x103fc4d => {
  { let v: u8 = cpu.r8(0x10ef202u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xfcu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103fc69; continue;
 }
 0x103fc69 => {
  cpu.push(0x103fc6e); f_010404bf(cpu); if cpu.returned_to != 0x103fc6e { return; }
  if cpu.cf { pc = 0x103fc82; continue; }
  pc = 0x103fc70; continue;
 }
 0x103fc70 => {
  cpu.push(0x103fc75); f_01040152(cpu); if cpu.returned_to != 0x103fc75 { return; }
  if cpu.cf { pc = 0x103fc82; continue; }
  pc = 0x103fc77; continue;
 }
 0x103fc77 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10ef202u32)); cpu.w8(0x10ef202u32, v); }
  cpu.poll(); pc = 0x103fb5b; continue;
 }
 0x103fc82 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103fc84 => {
  cpu.push(0x103fc89); f_0103fd8b(cpu); if cpu.returned_to != 0x103fc89 { return; }
  if cpu.cf { cpu.poll(); pc = 0x103fc82; continue; }
  pc = 0x103fc8b; continue;
 }
 0x103fc8b => {
  { let v: u8 = 0x3eu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x2710u16; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x2710u16; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x103fcaf); f_0103fd3d(cpu); if cpu.returned_to != 0x103fcaf { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11757a8u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x10ef202u32)); cpu.w8(0x10ef202u32, v); }
  cpu.poll(); pc = 0x103fb5b; continue;
 }
 0x103fcca => {
  cpu.push(0x103fccf); f_0103fd8b(cpu); if cpu.returned_to != 0x103fccf { return; }
  if cpu.cf { cpu.poll(); pc = 0x103fc82; continue; }
  pc = 0x103fcd1; continue;
 }
 0x103fcd1 => {
  { let v: u8 = 0x3eu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x2710u16; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x2710u16; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x103fcf5); f_0103fd3d(cpu); if cpu.returned_to != 0x103fcf5 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11757a8u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x10ef202u32)); cpu.w8(0x10ef202u32, v); }
  cpu.poll(); pc = 0x103fb5b; continue;
 }
 0x103fd10 => {
  { cpu.sub8(cpu.r8(0x10ef37bu32), 0x0u8); }
  if !cpu.zf { pc = 0x103fd3b; continue; }
  pc = 0x103fd19; continue;
 }
 0x103fd19 => {
  { let v: u8 = cpu.r8(0x10ef201u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef201u32, v); }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x10ef209u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x10ee98bu32.wrapping_add(cpu.esi), v); }
  pc = 0x103fd3b; continue;
 }
 0x103fd3b => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103fd3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103fd3d); }
 let mut pc: u32 = 0x103fd3d;
 loop { match pc {
 0x103fd3d => {
  { let v: u32 = cpu.r32(0x10e3f94u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4650u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x103fd4e; continue;
 }
 0x103fd4e => {
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v = cpu.r8(0x1cu32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x55f0u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x103fd7a; continue; }
  pc = 0x103fd77; continue;
 }
 0x103fd77 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103fd7a => {
  { let v: u32 = cpu.esi.wrapping_add(0x52u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x103fd4e; continue; }
  pc = 0x103fd80; continue;
 }
 0x103fd80 => {
  { let v: u32 = cpu.r32(0x10e3f94u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4650u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103fd8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103fd8b); }
 let mut pc: u32 = 0x103fd8b;
 loop { match pc {
 0x103fd8b => {
  cpu.push(0x103fd90); f_0103fdc2(cpu); if cpu.returned_to != 0x103fd90 { return; }
  { cpu.sub8(cpu.r8(0x10ef37bu32), 0x0u8); }
  if !cpu.zf { pc = 0x103fdb5; continue; }
  pc = 0x103fd99; continue;
 }
 0x103fd99 => {
  { let v: u8 = cpu.r8(0x10ef202u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xfcu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x103fdb5; continue;
 }
 0x103fdb5 => {
  cpu.push(0x103fdba); f_010404bf(cpu); if cpu.returned_to != 0x103fdba { return; }
  if cpu.cf { pc = 0x103fdc1; continue; }
  pc = 0x103fdbc; continue;
 }
 0x103fdbc => {
  cpu.push(0x103fdc1); f_01040152(cpu); if cpu.returned_to != 0x103fdc1 { return; }
  pc = 0x103fdc1; continue;
 }
 0x103fdc1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103fdc2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103fdc2); }
 let mut pc: u32 = 0x103fdc2;
 loop { match pc {
 0x103fdc2 => {
  { cpu.sub8(cpu.r8(0x10ef37bu32), 0x0u8); }
  if cpu.zf { pc = 0x103fdd9; continue; }
  pc = 0x103fdcb; continue;
 }
 0x103fdcb => {
  { let v: u16 = cpu.r16(0x10e3f43u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3f45u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103fdd9 => {
  { let v: u16 = 0x0u16; cpu.w16(0x1126360u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1126362u32, v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10ef201u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x10ef209u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x103fe17; continue; }
  pc = 0x103fe0a; continue;
 }
 0x103fe0a => {
  { cpu.sub8(cpu.r8(0x10ef209u32), 0x78u8); }
  if cpu.zf { pc = 0x103ff15; continue; }
  pc = 0x103fe17; continue;
 }
 0x103fe17 => {
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10eea7bu32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x103fe7e; continue; }
  pc = 0x103fe23; continue;
 }
 0x103fe23 => {
  { let v: u16 = 0xfffeu16; cpu.w16(0x1126360u32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x1126362u32, v); }
  { cpu.sub8(cpu.r8(0x10eea7bu32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x103fe7e; continue; }
  pc = 0x103fe3e; continue;
 }
 0x103fe3e => {
  { cpu.sub8(cpu.r8(0x10eea7bu32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x103fe7e; continue; }
  pc = 0x103fe47; continue;
 }
 0x103fe47 => {
  { cpu.sub8(cpu.r8(0x10eea7bu32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.zf { pc = 0x103fe7e; continue; }
  pc = 0x103fe50; continue;
 }
 0x103fe50 => {
  { let v: u16 = 0x0u16; cpu.w16(0x1126360u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1126362u32, v); }
  { let v: u32 = cpu.r32(0x1126350u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3df1u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x10e3df3u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103fe7e => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x103febd; continue; }
  pc = 0x103fe87; continue;
 }
 0x103fe87 => {
  { cpu.sub8(cpu.r8(0x10ef20au32), 0x0u8); }
  if cpu.zf { pc = 0x103febd; continue; }
  pc = 0x103fe90; continue;
 }
 0x103fe90 => {
  { cpu.sub8(cpu.r8(0x10ef20au32), 0x2u8); }
  if cpu.zf { pc = 0x103febd; continue; }
  pc = 0x103fe99; continue;
 }
 0x103fe99 => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x112634cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10fd0adu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x41u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10fd29fu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103fedf; continue;
 }
 0x103febd => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x112634cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10fd0adu32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x41u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10fd299u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x103fedf; continue;
 }
 0x103fedf => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x41u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x1125246u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1125244u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3df1u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3df3u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x1126360u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x1126362u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.r32(0x112634cu32)); cpu.w32(0x112634cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x103ff15 => {
  { let v: u16 = 0x0u16; cpu.w16(0x1126360u32, v); }
  { let v: u16 = 0xfff8u16; cpu.w16(0x1126362u32, v); }
  { cpu.sub8(cpu.r8(0x10eea7bu32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x103ff54; continue; }
  pc = 0x103ff30; continue;
 }
 0x103ff30 => {
  { cpu.sub8(cpu.r8(0x10eea7bu32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x103ff54; continue; }
  pc = 0x103ff39; continue;
 }
 0x103ff39 => {
  { cpu.sub8(cpu.r8(0x10eea7bu32.wrapping_add(cpu.esi)), 0x6u8); }
  if cpu.zf { pc = 0x103ff54; continue; }
  pc = 0x103ff42; continue;
 }
 0x103ff42 => {
  { let v: u16 = 0x0u16; cpu.w16(0x1126360u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x1126362u32, v); }
  pc = 0x103ff54; continue;
 }
 0x103ff54 => {
  { let v: u32 = 0xau32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = 0x116e61au32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x1126360u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x1126362u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ff78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ff78); }
 let mut pc: u32 = 0x103ff78;
 loop { match pc {
 0x103ff78 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ff79(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x103ff79); }
 let mut pc: u32 = 0x103ff79;
 loop { match pc {
 0x103ff79 => {
  { cpu.sub8(cpu.r8(0x10ef37bu32), 0x0u8); }
  if cpu.zf { pc = 0x103ff8d; continue; }
  pc = 0x103ff82; continue;
 }
 0x103ff82 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x52u32); cpu.eax = v; }
  pc = 0x103ffb7; continue;
 }
 0x103ff8d => {
  { let v: u32 = 0x78u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x103ffaa; continue; }
  pc = 0x103ff9b; continue;
 }
 0x103ff9b => {
  { cpu.sub16(cpu.r16(0x10e3d7fu32), 0x2u16); }
  if (cpu.cf || cpu.zf) { pc = 0x103ffaa; continue; }
  pc = 0x103ffa5; continue;
 }
 0x103ffa5 => {
  { let v: u32 = 0x3cu32; cpu.eax = v; }
  pc = 0x103ffaa; continue;
 }
 0x103ffaa => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.ecx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10ee98bu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.imul2_16((cpu.eax as u16), 0x52u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x103ffb7; continue;
 }
 0x103ffb7 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4194u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef201u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef202u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0103ffdb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x103ffdb); } r_0103ffdb(cpu, 0x103ffdb); }
pub fn f_01040000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1040000); } r_0103ffdb(cpu, 0x1040000); }
pub fn f_01040001(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1040001); } r_0103ffdb(cpu, 0x1040001); }
fn r_0103ffdb(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x103ffdb => {
  { let v: u32 = cpu.r32(0x10e4194u32); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x1cu32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x2cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x55f0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4198u32, v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10ef201u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x10ef209u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x10ee98bu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee984u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3df7u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef1ffu32, v); }
  cpu.push(0x104003d); f_0103fdc2(cpu); if cpu.returned_to != 0x104003d { return; }
  cpu.push(0x1040042); f_01040582(cpu); if cpu.returned_to != 0x1040042 { return; }
  cpu.push(0x1040047); f_01040575(cpu); if cpu.returned_to != 0x1040047 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10ef201u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x10ef209u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10eea7bu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8((cpu.ebx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x1040073; continue; }
  pc = 0x104006d; continue;
 }
 0x1040000 => {
  cpu.unimplemented(0x1040000, "Aas");
  pc = 0x1040001; continue;
 }
 0x1040001 => {
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.r32(0x10e4198u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.esp); cpu.w32(0x10e4198u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10ef201u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x10ef209u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x10ee98bu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee984u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3df7u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef1ffu32, v); }
  cpu.push(0x104003d); f_0103fdc2(cpu); if cpu.returned_to != 0x104003d { return; }
  cpu.push(0x1040042); f_01040582(cpu); if cpu.returned_to != 0x1040042 { return; }
  cpu.push(0x1040047); f_01040575(cpu); if cpu.returned_to != 0x1040047 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10ef201u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x10ef209u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10eea7bu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8((cpu.ebx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x1040073; continue; }
  pc = 0x104006d; continue;
 }
 0x104006d => {
  { let v: u8 = 0x5u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1040099; continue;
 }
 0x1040073 => {
  { cpu.sub8((cpu.ebx as u8), 0x4u8); }
  if !cpu.zf { pc = 0x1040082; continue; }
  pc = 0x1040078; continue;
 }
 0x1040078 => {
  { let v: u8 = 0x15u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x32u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1040099; continue;
 }
 0x1040082 => {
  { cpu.sub8((cpu.ebx as u8), 0x5u8); }
  if !cpu.zf { pc = 0x1040091; continue; }
  pc = 0x1040087; continue;
 }
 0x1040087 => {
  { let v: u8 = 0x16u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x33u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1040099; continue;
 }
 0x1040091 => {
  { let v: u8 = 0x17u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x34u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1040099; continue;
 }
 0x1040099 => {
  cpu.push(0x104009e); f_01052fba(cpu); if cpu.returned_to != 0x104009e { return; }
  { cpu.sub8(cpu.r8(0x10ef37bu32), 0x0u8); }
  if !cpu.zf { pc = 0x10400ea; continue; }
  pc = 0x10400a7; continue;
 }
 0x10400a7 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = cpu.r8(0x10ef201u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), cpu.r8(0x10ef209u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x10400c7); f_01066588(cpu); if cpu.returned_to != 0x10400c7 { return; }
  { let v: u8 = 0x4u8; cpu.w8(0x3fu32.wrapping_add(cpu.esi), v); }
  pc = 0x10400cb; continue;
 }
 0x10400cb => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e3fc0u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x112635cu32, v); }
  { let v: u32 = cpu.r32(0x10e4198u32); cpu.ebx = v; }
  cpu.push(0x10400e9); f_010400f1(cpu); if cpu.returned_to != 0x10400e9 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10400ea => {
  cpu.push(0x10400ef); f_01066278(cpu); if cpu.returned_to != 0x10400ef { return; }
  cpu.poll(); pc = 0x10400cb; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010400f1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10400f1); } r_010400f1(cpu, 0x10400f1); }
pub fn f_01040103(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1040103); } r_010400f1(cpu, 0x1040103); }
fn r_010400f1(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10400f1 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x24u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x26u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x25u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x23u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x19u32; cpu.eax = v; }
  { let v = cpu.r8(0x5u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x75u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x76u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x26u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x24u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x25u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1040103 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x88u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x25u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x23u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x19u32; cpu.eax = v; }
  { let v = cpu.r8(0x5u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x75u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x76u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x21u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x26u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x24u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x25u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01040152(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1040152); } r_01040152(cpu, 0x1040152); }
pub fn f_01040214(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1040214); } r_01040152(cpu, 0x1040214); }
fn r_01040152(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1040152 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ef201u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x10ef209u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x10eea7bu32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x10e4194u32); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x1cu32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u16 = cpu.imul2_16((cpu.eax as u16), 0x2cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.edi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x55f0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4198u32, v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x10ef37bu32), 0x0u8); }
  if cpu.zf { pc = 0x10401a9; continue; }
  pc = 0x10401a6; continue;
 }
 0x10401a6 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10401a9; continue;
 }
 0x10401a9 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x9du32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8((cpu.edx as u8), 0x7u8); }
  if cpu.zf { pc = 0x104049d; continue; }
  pc = 0x10401bb; continue;
 }
 0x10401bb => {
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x1040216; continue; }
  pc = 0x10401c0; continue;
 }
 0x10401c0 => {
  { cpu.sub8((cpu.edx as u8), 0xfu8); }
  if cpu.zf { pc = 0x10402bc; continue; }
  pc = 0x10401c9; continue;
 }
 0x10401c9 => {
  { cpu.sub8((cpu.edx as u8), 0x10u8); }
  if cpu.zf { pc = 0x10402e0; continue; }
  pc = 0x10401d2; continue;
 }
 0x10401d2 => {
  { cpu.sub8((cpu.edx as u8), 0x8u8); }
  if cpu.zf { pc = 0x1040269; continue; }
  pc = 0x10401db; continue;
 }
 0x10401db => {
  { cpu.sub8((cpu.edx as u8), 0x9u8); }
  if cpu.zf { pc = 0x1040326; continue; }
  pc = 0x10401e4; continue;
 }
 0x10401e4 => {
  { cpu.sub8((cpu.edx as u8), 0xbu8); }
  if cpu.zf { pc = 0x1040379; continue; }
  pc = 0x10401ed; continue;
 }
 0x10401ed => {
  { cpu.sub8((cpu.edx as u8), 0xcu8); }
  if cpu.zf { pc = 0x10403d2; continue; }
  pc = 0x10401f6; continue;
 }
 0x10401f6 => {
  { cpu.sub8((cpu.edx as u8), 0xau8); }
  if cpu.zf { pc = 0x1040425; continue; }
  pc = 0x10401ff; continue;
 }
 0x10401ff => {
  { cpu.sub8((cpu.edx as u8), 0x11u8); }
  if cpu.zf { pc = 0x104031b; continue; }
  pc = 0x1040208; continue;
 }
 0x1040208 => {
  { cpu.sub8((cpu.edx as u8), 0x12u8); }
  if cpu.zf { pc = 0x1040310; continue; }
  pc = 0x1040211; continue;
 }
 0x1040211 => {
  f_010402e7(cpu); return;
 }
 0x1040214 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1040216; continue;
 }
 0x1040216 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x104021c); f_01049f22(cpu); if cpu.returned_to != 0x104021c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x10404bd; continue; }
  pc = 0x1040223; continue;
 }
 0x1040223 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x1040240); f_010400f1(cpu); if cpu.returned_to != 0x1040240 { return; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7fu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1040250; continue; }
  pc = 0x104024e; continue;
 }
 0x104024e => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1040250; continue;
 }
 0x1040250 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x1040267); f_01052ccd(cpu); if cpu.returned_to != 0x1040267 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1040269 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x104026f); f_01049f22(cpu); if cpu.returned_to != 0x104026f { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x10404bd; continue; }
  pc = 0x1040276; continue;
 }
 0x1040276 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x1040293); f_010400f1(cpu); if cpu.returned_to != 0x1040293 { return; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7fu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10402a3; continue; }
  pc = 0x10402a1; continue;
 }
 0x10402a1 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10402a3; continue;
 }
 0x10402a3 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x10402ba); f_01052d1e(cpu); if cpu.returned_to != 0x10402ba { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10402bc => {
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124f57u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x63u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x75u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x76u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10402e0 => {
  cpu.push(0x10402e5); f_010402e7(cpu); if cpu.returned_to != 0x10402e5 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1040310 => {
  cpu.push(0x1040315); f_010402e7(cpu); if cpu.returned_to != 0x1040315 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104031b => {
  cpu.push(0x1040320); f_010402e7(cpu); if cpu.returned_to != 0x1040320 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x79u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1040326 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x104032c); f_01049f22(cpu); if cpu.returned_to != 0x104032c { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x10404bd; continue; }
  pc = 0x1040333; continue;
 }
 0x1040333 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x1040350); f_010400f1(cpu); if cpu.returned_to != 0x1040350 { return; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7fu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1040360; continue; }
  pc = 0x104035e; continue;
 }
 0x104035e => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1040360; continue;
 }
 0x1040360 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x1040377); f_01052d6f(cpu); if cpu.returned_to != 0x1040377 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1040379 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x104037f); f_01049f22(cpu); if cpu.returned_to != 0x104037f { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x10404bd; continue; }
  pc = 0x1040386; continue;
 }
 0x1040386 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x10403a3); f_010400f1(cpu); if cpu.returned_to != 0x10403a3 { return; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7fu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10403b3; continue; }
  pc = 0x10403b1; continue;
 }
 0x10403b1 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10403b3; continue;
 }
 0x10403b3 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x63u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x10403d0); f_01052e81(cpu); if cpu.returned_to != 0x10403d0 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10403d2 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10403d8); f_01049f22(cpu); if cpu.returned_to != 0x10403d8 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x10404bd; continue; }
  pc = 0x10403df; continue;
 }
 0x10403df => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x10403fc); f_010400f1(cpu); if cpu.returned_to != 0x10403fc { return; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7fu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x104040c; continue; }
  pc = 0x104040a; continue;
 }
 0x104040a => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x104040c; continue;
 }
 0x104040c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x1040423); f_01052ed2(cpu); if cpu.returned_to != 0x1040423 { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1040425 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x104042b); f_01049f22(cpu); if cpu.returned_to != 0x104042b { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x10404bd; continue; }
  pc = 0x1040432; continue;
 }
 0x1040432 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x104044f); f_010400f1(cpu); if cpu.returned_to != 0x104044f { return; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x7fu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x104045f; continue; }
  pc = 0x104045d; continue;
 }
 0x104045d => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x104045f; continue;
 }
 0x104045f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11296acu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x1040480); f_01052dc0(cpu); if cpu.returned_to != 0x1040480 { return; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x63u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x8au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x8bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x20u8); cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104049d => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10404a3); f_01049f22(cpu); if cpu.returned_to != 0x10404a3 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if cpu.cf { pc = 0x10404bd; continue; }
  pc = 0x10404a6; continue;
 }
 0x10404a6 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10404b3); f_010400f1(cpu); if cpu.returned_to != 0x10404b3 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x10404bb); f_01052e3f(cpu); if cpu.returned_to != 0x10404bb { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10404bd => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010402e7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10402e7); } r_010402e7(cpu, 0x10402e7); }
pub fn f_01040300(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1040300); } r_010402e7(cpu, 0x1040300); }
fn r_010402e7(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10402e7 => {
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x63u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1040300; continue;
 }
 0x1040300 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x75u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x104030e); f_010540e0(cpu); if cpu.returned_to != 0x104030e { return; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010404bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10404bf); }
 let mut pc: u32 = 0x10404bf;
 loop { match pc {
 0x10404bf => {
  cpu.push(0x10404c4); f_01040582(cpu); if cpu.returned_to != 0x10404c4 { return; }
  if cpu.cf { pc = 0x1040539; continue; }
  pc = 0x10404c6; continue;
 }
 0x10404c6 => {
  { let v: u8 = 0x4u8; cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x10ef37bu32), 0x0u8); }
  if !cpu.zf { pc = 0x104053a; continue; }
  pc = 0x10404d3; continue;
 }
 0x10404d3 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = cpu.r8(0x10ef201u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x10ef209u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x10404f3); f_01066588(cpu); if cpu.returned_to != 0x10404f3 { return; }
  { let v: u8 = 0x4u8; cpu.w8(0x3fu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x10e3f7cu32, v); }
  pc = 0x10404fd; continue;
 }
 0x10404fd => {
  cpu.push(0x1040502); f_01040575(cpu); if cpu.returned_to != 0x1040502 { return; }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x10ef20au32), 0x0u8); }
  if !cpu.zf { pc = 0x1040519; continue; }
  pc = 0x104050f; continue;
 }
 0x104050f => {
  { let v: u8 = 0x0u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0xefu8); cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1040519 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1040538; continue; }
  pc = 0x1040522; continue;
 }
 0x1040522 => {
  { cpu.sub8(cpu.r8(0x10e3f39u32), 0x0u8); }
  if cpu.zf { pc = 0x1040538; continue; }
  pc = 0x104052b; continue;
 }
 0x104052b => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x1040538; continue; }
  pc = 0x1040534; continue;
 }
 0x1040534 => {
  { let v: u8 = cpu.or8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x10u8); cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1040538; continue;
 }
 0x1040538 => {
  { cpu.cf = false; }
  pc = 0x1040539; continue;
 }
 0x1040539 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104053a => {
  { cpu.sub8(cpu.r8(0x10ef202u32), 0x0u8); }
  if !cpu.zf { pc = 0x104054a; continue; }
  pc = 0x1040543; continue;
 }
 0x1040543 => {
  cpu.push(0x1040548); f_01066278(cpu); if cpu.returned_to != 0x1040548 { return; }
  cpu.poll(); pc = 0x10404fd; continue;
 }
 0x104054a => {
  { let v: u32 = cpu.r32(0x10e3f7cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x11757a4u32); cpu.eax = v; }
  { let v: u8 = 0x48u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  cpu.push(0x104056c); f_01066588(cpu); if cpu.returned_to != 0x104056c { return; }
  { let v: u8 = 0x4u8; cpu.w8(0x3fu32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x10404fd; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01040575(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1040575); }
 let mut pc: u32 = 0x1040575;
 loop { match pc {
 0x1040575 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x10ee920u32.wrapping_add(cpu.eax))); cpu.w8(0x10ee920u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01040582(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1040582); }
 let mut pc: u32 = 0x1040582;
 loop { match pc {
 0x1040582 => {
  cpu.push(0x1040587); f_0103fab8(cpu); if cpu.returned_to != 0x1040587 { return; }
  if cpu.cf { pc = 0x1040780; continue; }
  pc = 0x104058d; continue;
 }
 0x104058d => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = 0xb8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4194u32); cpu.esi = v; }
  { let v: u32 = cpu.r8(0x10ef202u32) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.imul2_16((cpu.eax as u16), 0x2cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x1fu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xa1u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x24u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x77u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x34u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x6du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xadu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xafu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x7bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x7cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x40u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x41u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5bu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x42u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x44u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x48u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4eu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x50u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x52u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x54u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x5cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1040678); f_0105f215(cpu); if cpu.returned_to != 0x1040678 { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xb0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x7eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xaeu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x93u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x74u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x73u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x36u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x97u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5fu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10f6391u32; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10f66b6u32; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x88u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x94u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x95u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x99u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x9bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x9au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x9eu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x104070e); f_01059da3(cpu); if cpu.returned_to != 0x104070e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0xa5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10f7f68u32; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10f7f68u32; cpu.w32(0xa6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10f7f68u32; cpu.w32(0xb2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x12345678u32; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1040752); f_01068d78(cpu); if cpu.returned_to != 0x1040752 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ef20au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1040765); f_01040781(cpu); if cpu.returned_to != 0x1040765 { return; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ee950u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x10u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x104077f); f_0106868f(cpu); if cpu.returned_to != 0x104077f { return; }
  { cpu.cf = false; }
  pc = 0x1040780; continue;
 }
 0x1040780 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01040781(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1040781); }
 let mut pc: u32 = 0x1040781;
 loop { match pc {
 0x1040781 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x10ef37bu32), 0x0u8); }
  if !cpu.zf { pc = 0x10407ba; continue; }
  pc = 0x104078c; continue;
 }
 0x104078c => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x10407ba; continue; }
  pc = 0x1040795; continue;
 }
 0x1040795 => {
  { cpu.sub8(cpu.r8(0x10ef209u32), 0x78u8); }
  if !cpu.zf { pc = 0x10407ba; continue; }
  pc = 0x104079e; continue;
 }
 0x104079e => {
  { let v: u32 = cpu.r32(0x1126354u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x78u32); cpu.esi = v; }
  { let v: u32 = cpu.imul2_32(cpu.esi, 0xau32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x116e61au32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10407ba; continue;
 }
 0x10407ba => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010407c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10407c0); }
 let mut pc: u32 = 0x10407c0;
 loop { match pc {
 0x10407c0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1040803; continue; }
  pc = 0x10407cc; continue;
 }
 0x10407cc => {
  cpu.push(0x10407d1); f_010129b5(cpu); if cpu.returned_to != 0x10407d1 { return; }
  { let v: u32 = 0x15u32; cpu.eax = v; }
  cpu.push(0x10407db); f_01012780(cpu); if cpu.returned_to != 0x10407db { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1040803; continue; }
  pc = 0x10407e0; continue;
 }
 0x10407e0 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10407ea); f_01012bd0(cpu); if cpu.returned_to != 0x10407ea { return; }
  { let v: u32 = 0x16u32; cpu.eax = v; }
  cpu.push(0x10407f4); f_01012780(cpu); if cpu.returned_to != 0x10407f4 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1040803; continue; }
  pc = 0x10407f9; continue;
 }
 0x10407f9 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1040803); f_01012bd0(cpu); if cpu.returned_to != 0x1040803 { return; }
  pc = 0x1040803; continue;
 }
 0x1040803 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01040806(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1040806); }
 let mut pc: u32 = 0x1040806;
 loop { match pc {
 0x1040806 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1040849; continue; }
  pc = 0x1040812; continue;
 }
 0x1040812 => {
  cpu.push(0x1040817); f_010129b5(cpu); if cpu.returned_to != 0x1040817 { return; }
  { let v: u32 = 0x19u32; cpu.eax = v; }
  cpu.push(0x1040821); f_01012780(cpu); if cpu.returned_to != 0x1040821 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1040849; continue; }
  pc = 0x1040826; continue;
 }
 0x1040826 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1040830); f_01012bd0(cpu); if cpu.returned_to != 0x1040830 { return; }
  { let v: u32 = 0x1au32; cpu.eax = v; }
  cpu.push(0x104083a); f_01012780(cpu); if cpu.returned_to != 0x104083a { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1040849; continue; }
  pc = 0x104083f; continue;
 }
 0x104083f => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1040849); f_01012bd0(cpu); if cpu.returned_to != 0x1040849 { return; }
  pc = 0x1040849; continue;
 }
 0x1040849 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104084c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104084c); }
 let mut pc: u32 = 0x104084c;
 loop { match pc {
 0x104084c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x104088f; continue; }
  pc = 0x1040858; continue;
 }
 0x1040858 => {
  cpu.push(0x104085d); f_010129b5(cpu); if cpu.returned_to != 0x104085d { return; }
  { let v: u32 = 0x21u32; cpu.eax = v; }
  cpu.push(0x1040867); f_01012780(cpu); if cpu.returned_to != 0x1040867 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104088f; continue; }
  pc = 0x104086c; continue;
 }
 0x104086c => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1040876); f_01012bd0(cpu); if cpu.returned_to != 0x1040876 { return; }
  { let v: u32 = 0x22u32; cpu.eax = v; }
  cpu.push(0x1040880); f_01012780(cpu); if cpu.returned_to != 0x1040880 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104088f; continue; }
  pc = 0x1040885; continue;
 }
 0x1040885 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x104088f); f_01012bd0(cpu); if cpu.returned_to != 0x104088f { return; }
  pc = 0x104088f; continue;
 }
 0x104088f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01040892(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1040892); }
 let mut pc: u32 = 0x1040892;
 loop { match pc {
 0x1040892 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x10408d5; continue; }
  pc = 0x104089e; continue;
 }
 0x104089e => {
  cpu.push(0x10408a3); f_010129b5(cpu); if cpu.returned_to != 0x10408a3 { return; }
  { let v: u32 = 0x3bu32; cpu.eax = v; }
  cpu.push(0x10408ad); f_01012780(cpu); if cpu.returned_to != 0x10408ad { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10408d5; continue; }
  pc = 0x10408b2; continue;
 }
 0x10408b2 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10408bc); f_01012bd0(cpu); if cpu.returned_to != 0x10408bc { return; }
  { let v: u32 = 0x3cu32; cpu.eax = v; }
  cpu.push(0x10408c6); f_01012780(cpu); if cpu.returned_to != 0x10408c6 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10408d5; continue; }
  pc = 0x10408cb; continue;
 }
 0x10408cb => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x10408d5); f_01012bd0(cpu); if cpu.returned_to != 0x10408d5 { return; }
  pc = 0x10408d5; continue;
 }
 0x10408d5 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010408d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10408d8); }
 let mut pc: u32 = 0x10408d8;
 loop { match pc {
 0x10408d8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x104091b; continue; }
  pc = 0x10408e4; continue;
 }
 0x10408e4 => {
  cpu.push(0x10408e9); f_010129b5(cpu); if cpu.returned_to != 0x10408e9 { return; }
  { let v: u32 = 0x23u32; cpu.eax = v; }
  cpu.push(0x10408f3); f_01012780(cpu); if cpu.returned_to != 0x10408f3 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104091b; continue; }
  pc = 0x10408f8; continue;
 }
 0x10408f8 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1040902); f_01012bd0(cpu); if cpu.returned_to != 0x1040902 { return; }
  { let v: u32 = 0x24u32; cpu.eax = v; }
  cpu.push(0x104090c); f_01012780(cpu); if cpu.returned_to != 0x104090c { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104091b; continue; }
  pc = 0x1040911; continue;
 }
 0x1040911 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x104091b); f_01012bd0(cpu); if cpu.returned_to != 0x104091b { return; }
  pc = 0x104091b; continue;
 }
 0x104091b => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104091e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104091e); }
 let mut pc: u32 = 0x104091e;
 loop { match pc {
 0x104091e => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x10409a2; continue; }
  pc = 0x1040927; continue;
 }
 0x1040927 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x104096a; continue; }
  pc = 0x1040933; continue;
 }
 0x1040933 => {
  cpu.push(0x1040938); f_010129b5(cpu); if cpu.returned_to != 0x1040938 { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x1040942); f_01012780(cpu); if cpu.returned_to != 0x1040942 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104096a; continue; }
  pc = 0x1040947; continue;
 }
 0x1040947 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1040951); f_01012bd0(cpu); if cpu.returned_to != 0x1040951 { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x104095b); f_01012780(cpu); if cpu.returned_to != 0x104095b { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104096a; continue; }
  pc = 0x1040960; continue;
 }
 0x1040960 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x104096a); f_01012bd0(cpu); if cpu.returned_to != 0x104096a { return; }
  pc = 0x104096a; continue;
 }
 0x104096a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x1126365u32, v); }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x104097a; continue;
 }
 0x104097a => {
  { let v: u8 = cpu.r8(0x10e3e81u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ebx as u8), 0x3u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x10e3e81u32, v); }
  { let v: u8 = cpu.r8(0x11b4c8fu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11263c7u32, v); }
  cpu.push(0x104099b); f_010409d8(cpu); if cpu.returned_to != 0x104099b { return; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x104097a; continue; }
  pc = 0x104099d; continue;
 }
 0x104099d => {
  cpu.push(0x10409a2); f_010409a3(cpu); if cpu.returned_to != 0x10409a2 { return; }
  pc = 0x10409a2; continue;
 }
 0x10409a2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010409a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10409a3); }
 let mut pc: u32 = 0x10409a3;
 loop { match pc {
 0x10409a3 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10e3e81u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3da5u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xffffu16); }
  if cpu.zf { pc = 0x10409d6; continue; }
  pc = 0x10409bb; continue;
 }
 0x10409bb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.ebp = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10b0300u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1124e8au32, v); }
  cpu.push(0x10409d6); f_01054417(cpu); if cpu.returned_to != 0x10409d6 { return; }
  pc = 0x10409d6; continue;
 }
 0x10409d6 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010409d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10409d8); }
 let mut pc: u32 = 0x10409d8;
 loop { match pc {
 0x10409d8 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10e3e81u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3da5u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xffffu16); }
  if cpu.zf { pc = 0x1040a09; continue; }
  pc = 0x10409f0; continue;
 }
 0x10409f0 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xc1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x1040a09; continue; }
  pc = 0x10409fb; continue;
 }
 0x10409fb => {
  { let v: u32 = cpu.r8(0x11263c7u32) as u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10fd52eu32.wrapping_add(cpu.eax)), 0xffu8); }
  pc = 0x1040a09; continue;
 }
 0x1040a09 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01040a0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1040a0b); }
 let mut pc: u32 = 0x1040a0b;
 loop { match pc {
 0x1040a0b => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11263ecu32)); cpu.w8(0x11263ecu32, v); }
  { let v: u8 = cpu.and8(cpu.r8(0x11263ecu32), 0xfu8); cpu.w8(0x11263ecu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01040a19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1040a19); }
 let mut pc: u32 = 0x1040a19;
 loop { match pc {
 0x1040a19 => {
  { cpu.sub8(cpu.r8(0x11b51b9u32), 0x0u8); }
  if !cpu.zf { pc = 0x1040a23; continue; }
  pc = 0x1040a22; continue;
 }
 0x1040a22 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1040a23 => {
  { cpu.pushad(); }
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
  { let v = cpu.r16(0x10c6872u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10c6872u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r32(0x10e4114u32); cpu.push(v); }
  { let v: u32 = cpu.r16(0x10e3d85u32) as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10e4114u32, v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x10e4114u32), 4u32); cpu.w32(0x10e4114u32, v); }
  cpu.push(0x1040a9a); f_0107153d(cpu); if cpu.returned_to != 0x1040a9a { return; }
  { let v: u32 = 0x19u32; cpu.eax = v; }
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = 0x4e9u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1040aaf); cpu.call(t); if cpu.returned_to != 0x1040aaf { return; }
  { let v: u16 = 0x1au16; cpu.w16(0x10e40fcu32, v); }
  { let v: u16 = 0x8au16; cpu.w16(0x10e4100u32, v); }
  { let v: u16 = 0x78u16; cpu.w16(0x10e4104u32, v); }
  { let v: u16 = 0xa0u16; cpu.w16(0x10e4108u32, v); }
  { let v: u32 = 0x10fd2e0u32; cpu.esi = v; }
  { let v: u32 = 0x1du32; cpu.eax = v; }
  { let v: u32 = 0x8cu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10fcdb6u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10fcdbau32, v); }
  cpu.push(0x1040af2); f_01046968(cpu); if cpu.returned_to != 0x1040af2 { return; }
  { cpu.sub8(cpu.r8(0x10fd42fu32), 0x2u8); }
  if cpu.zf { pc = 0x1040bb5; continue; }
  pc = 0x1040aff; continue;
 }
 0x1040aff => {
  { cpu.sub8(cpu.r8(0x10fd41au32), 0x0u8); }
  if cpu.zf { pc = 0x1040b0e; continue; }
  pc = 0x1040b08; continue;
 }
 0x1040b08 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x10fd41au32)); cpu.w8(0x10fd41au32, v); }
  pc = 0x1040b0e; continue;
 }
 0x1040b0e => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10ee92eu32)); cpu.w8(0x10ee92eu32, v); }
  { let v: u8 = cpu.r8(0x10ee92eu32) & 0x7u8; cpu.w8(0x10ee92eu32, v); }
  { cpu.sub8(cpu.r8(0x10fd41au32), 0x0u8); }
  if !cpu.zf { pc = 0x1040b31; continue; }
  pc = 0x1040b24; continue;
 }
 0x1040b24 => {
  { cpu.sub8(cpu.r8(0x10ee92eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1040bb5; continue; }
  pc = 0x1040b31; continue;
 }
 0x1040b31 => {
  { let v: u32 = 0x10fd2e1u32; cpu.esi = v; }
  { let v: u32 = 0x10fd2e0u32; cpu.edi = v; }
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  pc = 0x1040b40; continue;
 }
 0x1040b40 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x1040b48; continue; }
  pc = 0x1040b46; continue;
 }
 0x1040b46 => {
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1040b48; continue;
 }
 0x1040b48 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1040b40; continue; }
  pc = 0x1040b4e; continue;
 }
 0x1040b4e => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x10fd41au32), 0x0u8); }
  if cpu.zf { pc = 0x1040b5e; continue; }
  pc = 0x1040b5a; continue;
 }
 0x1040b5a => {
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1040b6c; continue;
 }
 0x1040b5e => {
  { let v: u32 = cpu.r32(0x10fd413u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10fd2d8u32)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1040b6c; continue;
 }
 0x1040b6c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x10fd41au32), 0x0u8); }
  if !cpu.zf { pc = 0x1040bb5; continue; }
  pc = 0x1040b7b; continue;
 }
 0x1040b7b => {
  { cpu.sub8(cpu.r8(0x10fd42fu32), 0x2u8); }
  if cpu.zf { pc = 0x1040bb5; continue; }
  pc = 0x1040b84; continue;
 }
 0x1040b84 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10fd413u32)); cpu.w32(0x10fd413u32, v); }
  { let v: u32 = cpu.r32(0x10fd413u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10fd2d8u32)); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x1040bb5; continue; }
  pc = 0x1040b9b; continue;
 }
 0x1040b9b => {
  { let v: u32 = 0x0u32; cpu.w32(0x10fd413u32, v); }
  { cpu.sub8(cpu.r8(0x10fd42fu32), 0x1u8); }
  if !cpu.zf { pc = 0x1040bb5; continue; }
  pc = 0x1040bae; continue;
 }
 0x1040bae => {
  { let v: u8 = 0x2u8; cpu.w8(0x10fd42fu32, v); }
  pc = 0x1040bb5; continue;
 }
 0x1040bb5 => {
  cpu.push(0x1040bba); f_010715b4(cpu); if cpu.returned_to != 0x1040bba { return; }
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
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01040c19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1040c19); }
 let mut pc: u32 = 0x1040c19;
 loop { match pc {
 0x1040c19 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x10fd2d8u32), cpu.esi); }
  if cpu.zf { pc = 0x1040c7a; continue; }
  pc = 0x1040c22; continue;
 }
 0x1040c22 => {
  cpu.push(0x1040c27); f_0104683d(cpu); if cpu.returned_to != 0x1040c27 { return; }
  { cpu.sub32(cpu.eax, 0x5au32); }
  if (cpu.sf == cpu.of) { pc = 0x1040c53; continue; }
  pc = 0x1040c2c; continue;
 }
 0x1040c2c => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd42fu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10fd2d8u32, v); }
  { let v: u8 = 0x28u8; cpu.w8(0x10fd41au32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd41bu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10fd413u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1040c53 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fd42fu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10fd2d8u32, v); }
  { let v: u8 = 0x28u8; cpu.w8(0x10fd41au32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd41bu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10fd413u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1040c7a => {
  { let v: u32 = 0x0u32; cpu.w32(0x10fd413u32, v); }
  { let v: u8 = 0x28u8; cpu.w8(0x10fd41au32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd41bu32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01040c94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1040c94); }
 let mut pc: u32 = 0x1040c94;
 loop { match pc {
 0x1040c94 => {
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1040ca6; continue; }
  pc = 0x1040c9d; continue;
 }
 0x1040c9d => {
  { let v: u8 = 0x0u8; cpu.w8(0x11263c2u32, v); }
  pc = 0x1040cac; continue;
 }
 0x1040ca6 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11263c2u32)); cpu.w8(0x11263c2u32, v); }
  pc = 0x1040cac; continue;
 }
 0x1040cac => {
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1040cbe; continue; }
  pc = 0x1040cb5; continue;
 }
 0x1040cb5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11263c6u32, v); }
  pc = 0x1040cc4; continue;
 }
 0x1040cbe => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11263c6u32)); cpu.w8(0x11263c6u32, v); }
  pc = 0x1040cc4; continue;
 }
 0x1040cc4 => {
  { cpu.sub8(cpu.r8(0x11263ccu32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1040cd6; continue; }
  pc = 0x1040ccd; continue;
 }
 0x1040ccd => {
  { let v: u8 = 0x0u8; cpu.w8(0x11263ccu32, v); }
  pc = 0x1040cdc; continue;
 }
 0x1040cd6 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11263ccu32)); cpu.w8(0x11263ccu32, v); }
  pc = 0x1040cdc; continue;
 }
 0x1040cdc => {
  { cpu.sub8(cpu.r8(0x112636au32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1040cee; continue; }
  pc = 0x1040ce5; continue;
 }
 0x1040ce5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x112636au32, v); }
  pc = 0x1040cf4; continue;
 }
 0x1040cee => {
  { let v: u8 = cpu.inc8(cpu.r8(0x112636au32)); cpu.w8(0x112636au32, v); }
  pc = 0x1040cf4; continue;
 }
 0x1040cf4 => {
  { cpu.sub8(cpu.r8(0x11263c3u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1040d06; continue; }
  pc = 0x1040cfd; continue;
 }
 0x1040cfd => {
  { let v: u8 = 0x0u8; cpu.w8(0x11263c3u32, v); }
  pc = 0x1040d0c; continue;
 }
 0x1040d06 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11263c3u32)); cpu.w8(0x11263c3u32, v); }
  pc = 0x1040d0c; continue;
 }
 0x1040d0c => {
  { cpu.sub8(cpu.r8(0x10fd3eau32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1040d1e; continue; }
  pc = 0x1040d15; continue;
 }
 0x1040d15 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fd3eau32, v); }
  pc = 0x1040d24; continue;
 }
 0x1040d1e => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10fd3eau32)); cpu.w8(0x10fd3eau32, v); }
  pc = 0x1040d24; continue;
 }
 0x1040d24 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01040d25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1040d25); }
 let mut pc: u32 = 0x1040d25;
 loop { match pc {
 0x1040d25 => {
  { cpu.sub8(cpu.r8(0x1105e6au32), 0x0u8); }
  if cpu.zf { pc = 0x1040d43; continue; }
  pc = 0x1040d2e; continue;
 }
 0x1040d2e => {
  { cpu.sub8(cpu.r8(0x1105e6bu32), 0x0u8); }
  if cpu.zf { pc = 0x1040d3e; continue; }
  pc = 0x1040d37; continue;
 }
 0x1040d37 => {
  cpu.push(0x1040d3c); f_01071dbe(cpu); if cpu.returned_to != 0x1040d3c { return; }
  pc = 0x1040d43; continue;
 }
 0x1040d3e => {
  cpu.push(0x1040d43); f_01071c84(cpu); if cpu.returned_to != 0x1040d43 { return; }
  pc = 0x1040d43; continue;
 }
 0x1040d43 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01040d44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1040d44); }
 let mut pc: u32 = 0x1040d44;
 loop { match pc {
 0x1040d44 => {
  { let v: u32 = 0xc1u32; cpu.eax = v; }
  { let v: u32 = 0x5au32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x11282c0u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x11282c0u32, v); }
  { let v: u16 = cpu.r16(0x11282c2u32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x11282c2u32, v); }
  { let v: u16 = cpu.r16(0x11282c4u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x11282c4u32, v); }
  { let v: u16 = cpu.r16(0x11282c6u32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x11282c6u32, v); }
  { let v: u16 = cpu.r16(0x11282d4u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x11282d4u32, v); }
  { let v: u16 = cpu.r16(0x11282d6u32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x11282d6u32, v); }
  { let v: u16 = cpu.r16(0x11282d8u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x11282d8u32, v); }
  { let v: u16 = cpu.r16(0x11282dau32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x11282dau32, v); }
  { let v: u16 = cpu.r16(0x11282e8u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x11282e8u32, v); }
  { let v: u16 = cpu.r16(0x11282eau32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x11282eau32, v); }
  { let v: u16 = cpu.r16(0x11282ecu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x11282ecu32, v); }
  { let v: u16 = cpu.r16(0x11282eeu32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x11282eeu32, v); }
  { let v: u16 = cpu.r16(0x11282fcu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x11282fcu32, v); }
  { let v: u16 = cpu.r16(0x11282feu32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x11282feu32, v); }
  { let v: u16 = cpu.r16(0x1128300u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x1128300u32, v); }
  { let v: u16 = cpu.r16(0x1128302u32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x1128302u32, v); }
  { let v: u16 = cpu.r16(0x1128310u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x1128310u32, v); }
  { let v: u16 = cpu.r16(0x1128312u32).wrapping_add((cpu.ebx as u16)); cpu.w16(0x1128312u32, v); }
  { let v: u16 = cpu.r16(0x1128314u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x1128314u32, v); }
  { let v: u16 = cpu.add16(cpu.r16(0x1128316u32), (cpu.ebx as u16)); cpu.w16(0x1128316u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01040ddb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1040ddb); }
 let mut pc: u32 = 0x1040ddb;
 loop { match pc {
 0x1040ddb => {
  { let v: u32 = 0xc1u32; cpu.w32(0x11c5ba7u32, v); }
  { let v: u32 = 0x5au32; cpu.w32(0x11c5babu32, v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xc1u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x5au32); cpu.ebx = v; }
  { let v: u32 = 0x4f2u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1040e0c); cpu.call(t); if cpu.returned_to != 0x1040e0c { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xc1u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x5au32); cpu.ebx = v; }
  { let v: u32 = 0x4f2u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1040e29); cpu.call(t); if cpu.returned_to != 0x1040e29 { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x22u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xc1u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x5au32); cpu.ebx = v; }
  { let v: u32 = 0x4f2u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1040e46); cpu.call(t); if cpu.returned_to != 0x1040e46 { return; }
  { let v: u32 = 0xe8u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0x16e4u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1040e5b); cpu.call(t); if cpu.returned_to != 0x1040e5b { return; }
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x58u32; cpu.ebx = v; }
  { let v: u32 = 0x16e5u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1040e70); cpu.call(t); if cpu.returned_to != 0x1040e70 { return; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x16e6u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1040e85); cpu.call(t); if cpu.returned_to != 0x1040e85 { return; }
  { let v: u32 = 0xbbu32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0x4ebu32; cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0x10e3cdbu32), 0xbfu16); }
  if (cpu.sf != cpu.of) { pc = 0x1040ec4; continue; }
  pc = 0x1040e9f; continue;
 }
 0x1040e9f => {
  { cpu.sub16(cpu.r16(0x10e3cdbu32), 0x119u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1040ec4; continue; }
  pc = 0x1040eaa; continue;
 }
 0x1040eaa => {
  { cpu.sub16(cpu.r16(0x10e3cddu32), 0x59u16); }
  if (cpu.sf != cpu.of) { pc = 0x1040ec4; continue; }
  pc = 0x1040eb4; continue;
 }
 0x1040eb4 => {
  { cpu.sub16(cpu.r16(0x10e3cddu32), 0x8eu16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1040ec4; continue; }
  pc = 0x1040ebf; continue;
 }
 0x1040ebf => {
  { let v: u32 = 0x4f3u32; cpu.ecx = v; }
  pc = 0x1040ec4; continue;
 }
 0x1040ec4 => {
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1040eca); cpu.call(t); if cpu.returned_to != 0x1040eca { return; }
  { let v: u32 = cpu.r32(0x11c5ba7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c5babu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1131030u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x11c5ba3u32), 0x0u8); }
  if cpu.zf { pc = 0x1040ef1; continue; }
  pc = 0x1040eea; continue;
 }
 0x1040eea => {
  cpu.push(0x1040eef); f_0104681f(cpu); if cpu.returned_to != 0x1040eef { return; }
  pc = 0x1040ef6; continue;
 }
 0x1040ef1 => {
  cpu.push(0x1040ef6); f_0104682e(cpu); if cpu.returned_to != 0x1040ef6 { return; }
  pc = 0x1040ef6; continue;
 }
 0x1040ef6 => {
  { cpu.sub8(cpu.r8(0x1105e6au32), 0x0u8); }
  if !cpu.zf { pc = 0x1040f46; continue; }
  pc = 0x1040eff; continue;
 }
 0x1040eff => {
  { let v: u32 = cpu.r32(0x11c5ba7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c5babu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x31u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = 0x4edu32; cpu.ecx = v; }
  { cpu.pushad(); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1040f1c); cpu.call(t); if cpu.returned_to != 0x1040f1c { return; }
  { cpu.popad(); }
  { let v: u32 = 0x45du32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1040f28); cpu.call(t); if cpu.returned_to != 0x1040f28 { return; }
  { let v: u32 = cpu.r32(0x11c5ba7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c5babu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = 0x4eeu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1040f44); cpu.call(t); if cpu.returned_to != 0x1040f44 { return; }
  pc = 0x1040fc1; continue;
 }
 0x1040f46 => {
  { cpu.sub8(cpu.r8(0x1105e6bu32), 0x0u8); }
  if cpu.zf { pc = 0x1040f89; continue; }
  pc = 0x1040f4f; continue;
 }
 0x1040f4f => {
  { let v: u32 = cpu.r32(0x11c5ba7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c5babu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x31u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = 0x4ecu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1040f6b); cpu.call(t); if cpu.returned_to != 0x1040f6b { return; }
  { let v: u32 = cpu.r32(0x11c5ba7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c5babu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = 0x4efu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1040f87); cpu.call(t); if cpu.returned_to != 0x1040f87 { return; }
  pc = 0x1040fc1; continue;
 }
 0x1040f89 => {
  { let v: u32 = cpu.r32(0x11c5ba7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c5babu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x31u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = 0x4edu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1040fa5); cpu.call(t); if cpu.returned_to != 0x1040fa5 { return; }
  { let v: u32 = cpu.r32(0x11c5ba7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c5babu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = 0x4eeu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1040fc1); cpu.call(t); if cpu.returned_to != 0x1040fc1 { return; }
  pc = 0x1040fc1; continue;
 }
 0x1040fc1 => {
  { let v: u32 = cpu.r32(0x11c5ba7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c5babu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x15u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x113102cu32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x11c5ba4u32), 0x0u8); }
  if cpu.zf { pc = 0x1040fe8; continue; }
  pc = 0x1040fe1; continue;
 }
 0x1040fe1 => {
  cpu.push(0x1040fe6); f_0104681f(cpu); if cpu.returned_to != 0x1040fe6 { return; }
  pc = 0x1040fed; continue;
 }
 0x1040fe8 => {
  cpu.push(0x1040fed); f_0104682e(cpu); if cpu.returned_to != 0x1040fed { return; }
  pc = 0x1040fed; continue;
 }
 0x1040fed => {
  { cpu.sub8(cpu.r8(0x11302acu32), 0x0u8); }
  if cpu.zf { pc = 0x1041030; continue; }
  pc = 0x1040ff6; continue;
 }
 0x1040ff6 => {
  { let v: u32 = cpu.r32(0x11c5ba7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c5babu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x31u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x15u32); cpu.ebx = v; }
  { let v: u32 = 0x4ecu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1041012); cpu.call(t); if cpu.returned_to != 0x1041012 { return; }
  { let v: u32 = cpu.r32(0x11c5ba7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c5babu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x15u32); cpu.ebx = v; }
  { let v: u32 = 0x4efu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104102e); cpu.call(t); if cpu.returned_to != 0x104102e { return; }
  pc = 0x1041068; continue;
 }
 0x1041030 => {
  { let v: u32 = cpu.r32(0x11c5ba7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c5babu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x31u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x15u32); cpu.ebx = v; }
  { let v: u32 = 0x4edu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104104c); cpu.call(t); if cpu.returned_to != 0x104104c { return; }
  { let v: u32 = cpu.r32(0x11c5ba7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c5babu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x44u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x15u32); cpu.ebx = v; }
  { let v: u32 = 0x4eeu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1041068); cpu.call(t); if cpu.returned_to != 0x1041068 { return; }
  pc = 0x1041068; continue;
 }
 0x1041068 => {
  { let v: u32 = cpu.r32(0x11c5ba7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c5babu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x27u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1131358u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x11c5ba5u32), 0x0u8); }
  if cpu.zf { pc = 0x104108f; continue; }
  pc = 0x1041088; continue;
 }
 0x1041088 => {
  cpu.push(0x104108d); f_0104681f(cpu); if cpu.returned_to != 0x104108d { return; }
  pc = 0x1041094; continue;
 }
 0x104108f => {
  cpu.push(0x1041094); f_0104682e(cpu); if cpu.returned_to != 0x1041094 { return; }
  pc = 0x1041094; continue;
 }
 0x1041094 => {
  { let v: u32 = cpu.r32(0x11c5ba7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c5babu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x23u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x27u32); cpu.ebx = v; }
  { let v: u32 = 0x4f0u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10410b0); cpu.call(t); if cpu.returned_to != 0x10410b0 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1105eaau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x11c5bafu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11c5ba7u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11c5babu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x22u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x29u32); cpu.ebx = v; }
  { let v: u32 = 0x4f1u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10410da); cpu.call(t); if cpu.returned_to != 0x10410da { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5ba3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5ba4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5ba5u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5ba6u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010410f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10410f7); }
 let mut pc: u32 = 0x10410f7;
 loop { match pc {
 0x10410f7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5ba3u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010410ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10410ff); }
 let mut pc: u32 = 0x10410ff;
 loop { match pc {
 0x10410ff => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5ba3u32, v); }
  { cpu.sub8(cpu.r8(0x1105e6au32), 0x0u8); }
  if !cpu.zf { pc = 0x1041110; continue; }
  pc = 0x104110f; continue;
 }
 0x104110f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1041110 => {
  { let v: u8 = 0x1u8; cpu.w8(0x1105e6bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1105e6cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1041161; continue; }
  pc = 0x104112a; continue;
 }
 0x104112a => {
  cpu.push(0x104112f); f_010129b5(cpu); if cpu.returned_to != 0x104112f { return; }
  { let v: u32 = 0x39u32; cpu.eax = v; }
  cpu.push(0x1041139); f_01012780(cpu); if cpu.returned_to != 0x1041139 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1041161; continue; }
  pc = 0x104113e; continue;
 }
 0x104113e => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1041148); f_01012bd0(cpu); if cpu.returned_to != 0x1041148 { return; }
  { let v: u32 = 0x3au32; cpu.eax = v; }
  cpu.push(0x1041152); f_01012780(cpu); if cpu.returned_to != 0x1041152 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1041161; continue; }
  pc = 0x1041157; continue;
 }
 0x1041157 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1041161); f_01012bd0(cpu); if cpu.returned_to != 0x1041161 { return; }
  pc = 0x1041161; continue;
 }
 0x1041161 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01041164(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1041164); }
 let mut pc: u32 = 0x1041164;
 loop { match pc {
 0x1041164 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5ba3u32, v); }
  { cpu.sub8(cpu.r8(0x1105e6au32), 0x0u8); }
  if !cpu.zf { pc = 0x1041175; continue; }
  pc = 0x1041174; continue;
 }
 0x1041174 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1041175 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1105e6bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1105e6cu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x10411c6; continue; }
  pc = 0x104118f; continue;
 }
 0x104118f => {
  cpu.push(0x1041194); f_010129b5(cpu); if cpu.returned_to != 0x1041194 { return; }
  { let v: u32 = 0x39u32; cpu.eax = v; }
  cpu.push(0x104119e); f_01012780(cpu); if cpu.returned_to != 0x104119e { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10411c6; continue; }
  pc = 0x10411a3; continue;
 }
 0x10411a3 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10411ad); f_01012bd0(cpu); if cpu.returned_to != 0x10411ad { return; }
  { let v: u32 = 0x3au32; cpu.eax = v; }
  cpu.push(0x10411b7); f_01012780(cpu); if cpu.returned_to != 0x10411b7 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10411c6; continue; }
  pc = 0x10411bc; continue;
 }
 0x10411bc => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x10411c6); f_01012bd0(cpu); if cpu.returned_to != 0x10411c6 { return; }
  pc = 0x10411c6; continue;
 }
 0x10411c6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010411c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10411c9); }
 let mut pc: u32 = 0x10411c9;
 loop { match pc {
 0x10411c9 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5ba4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010411d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10411d1); }
 let mut pc: u32 = 0x10411d1;
 loop { match pc {
 0x10411d1 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5ba4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11302acu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1041222; continue; }
  pc = 0x10411eb; continue;
 }
 0x10411eb => {
  cpu.push(0x10411f0); f_010129b5(cpu); if cpu.returned_to != 0x10411f0 { return; }
  { let v: u32 = 0x39u32; cpu.eax = v; }
  cpu.push(0x10411fa); f_01012780(cpu); if cpu.returned_to != 0x10411fa { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1041222; continue; }
  pc = 0x10411ff; continue;
 }
 0x10411ff => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1041209); f_01012bd0(cpu); if cpu.returned_to != 0x1041209 { return; }
  { let v: u32 = 0x3au32; cpu.eax = v; }
  cpu.push(0x1041213); f_01012780(cpu); if cpu.returned_to != 0x1041213 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1041222; continue; }
  pc = 0x1041218; continue;
 }
 0x1041218 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1041222); f_01012bd0(cpu); if cpu.returned_to != 0x1041222 { return; }
  pc = 0x1041222; continue;
 }
 0x1041222 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01041225(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1041225); }
 let mut pc: u32 = 0x1041225;
 loop { match pc {
 0x1041225 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5ba4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11302acu32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1041276; continue; }
  pc = 0x104123f; continue;
 }
 0x104123f => {
  cpu.push(0x1041244); f_010129b5(cpu); if cpu.returned_to != 0x1041244 { return; }
  { let v: u32 = 0x39u32; cpu.eax = v; }
  cpu.push(0x104124e); f_01012780(cpu); if cpu.returned_to != 0x104124e { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1041276; continue; }
  pc = 0x1041253; continue;
 }
 0x1041253 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x104125d); f_01012bd0(cpu); if cpu.returned_to != 0x104125d { return; }
  { let v: u32 = 0x3au32; cpu.eax = v; }
  cpu.push(0x1041267); f_01012780(cpu); if cpu.returned_to != 0x1041267 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1041276; continue; }
  pc = 0x104126c; continue;
 }
 0x104126c => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1041276); f_01012bd0(cpu); if cpu.returned_to != 0x1041276 { return; }
  pc = 0x1041276; continue;
 }
 0x1041276 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01041279(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1041279); }
 let mut pc: u32 = 0x1041279;
 loop { match pc {
 0x1041279 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5ba5u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01041281(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1041281); }
 let mut pc: u32 = 0x1041281;
 loop { match pc {
 0x1041281 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x11c5ba7u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x23u32); cpu.eax = v; }
  { let v: u16 = 0xcu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1105eaau32, v); }
  cpu.push(0x10412a5); f_01014856(cpu); if cpu.returned_to != 0x10412a5 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x11a3af0u32; cpu.eax = v; }
  { let v: u32 = 0x11a37f0u32; cpu.edx = v; }
  { let v: u32 = 0x100u32; cpu.ebx = v; }
  cpu.push(0x10412ba); f_01014921(cpu); if cpu.returned_to != 0x10412ba { return; }
  { cpu.popad(); }
  cpu.push(0x10412c0); f_0107269d(cpu); if cpu.returned_to != 0x10412c0 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1041303; continue; }
  pc = 0x10412cc; continue;
 }
 0x10412cc => {
  cpu.push(0x10412d1); f_010129b5(cpu); if cpu.returned_to != 0x10412d1 { return; }
  { let v: u32 = 0x37u32; cpu.eax = v; }
  cpu.push(0x10412db); f_01012780(cpu); if cpu.returned_to != 0x10412db { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1041303; continue; }
  pc = 0x10412e0; continue;
 }
 0x10412e0 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10412ea); f_01012bd0(cpu); if cpu.returned_to != 0x10412ea { return; }
  { let v: u32 = 0x38u32; cpu.eax = v; }
  cpu.push(0x10412f4); f_01012780(cpu); if cpu.returned_to != 0x10412f4 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1041303; continue; }
  pc = 0x10412f9; continue;
 }
 0x10412f9 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1041303); f_01012bd0(cpu); if cpu.returned_to != 0x1041303 { return; }
  pc = 0x1041303; continue;
 }
 0x1041303 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x11c5ba5u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01041736(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1041736); }
 let mut pc: u32 = 0x1041736;
 loop { match pc {
 0x1041736 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x1105eaau32); cpu.eax = v; }
  cpu.push(0x1041741); f_01014856(cpu); if cpu.returned_to != 0x1041741 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x11a3af0u32; cpu.eax = v; }
  { let v: u32 = 0x11a37f0u32; cpu.edx = v; }
  { let v: u32 = 0x100u32; cpu.ebx = v; }
  cpu.push(0x1041756); f_01014921(cpu); if cpu.returned_to != 0x1041756 { return; }
  { cpu.popad(); }
  cpu.push(0x104175c); f_0107269d(cpu); if cpu.returned_to != 0x104175c { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104195c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104195c); }
 let mut pc: u32 = 0x104195c;
 loop { match pc {
 0x104195c => {
  { let v: u32 = 0xb0u32; cpu.eax = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = 0xb2bu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1041971); cpu.call(t); if cpu.returned_to != 0x1041971 { return; }
  { let v: u32 = 0x128u32; cpu.eax = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = 0xb2cu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1041986); cpu.call(t); if cpu.returned_to != 0x1041986 { return; }
  { let v: u32 = 0xb0u32; cpu.eax = v; }
  { let v: u32 = 0xb0u32; cpu.ebx = v; }
  { let v: u32 = 0xb2du32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104199b); cpu.call(t); if cpu.returned_to != 0x104199b { return; }
  { let v: u32 = 0x128u32; cpu.eax = v; }
  { let v: u32 = 0xb0u32; cpu.ebx = v; }
  { let v: u32 = 0xb2eu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10419b0); cpu.call(t); if cpu.returned_to != 0x10419b0 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01041b0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1041b0d); }
 let mut pc: u32 = 0x1041b0d;
 loop { match pc {
 0x1041b0d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0xffu32; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.push(0x1041b28); f_01037d64(cpu); if cpu.returned_to != 0x1041b28 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01041f82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1041f82); }
 let mut pc: u32 = 0x1041f82;
 loop { match pc {
 0x1041f82 => {
  { let v: u32 = cpu.r32(0x11310fcu32); cpu.esi = v; }
  { let v: u32 = 0x11c6051u32; cpu.edi = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  pc = 0x1041f92; continue;
 }
 0x1041f92 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1041f9f; continue; }
  pc = 0x1041f98; continue;
 }
 0x1041f98 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1041f92; continue; }
  pc = 0x1041f9f; continue;
 }
 0x1041f9f => {
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1105e83u32); cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x1041fae; continue;
 }
 0x1041fae => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1041fbb; continue; }
  pc = 0x1041fb4; continue;
 }
 0x1041fb4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1041fae; continue; }
  pc = 0x1041fbb; continue;
 }
 0x1041fbb => {
  { let v: u32 = cpu.r32(0x113138cu32); cpu.esi = v; }
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  pc = 0x1041fc6; continue;
 }
 0x1041fc6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1041fd3; continue; }
  pc = 0x1041fcc; continue;
 }
 0x1041fcc => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1041fc6; continue; }
  pc = 0x1041fd3; continue;
 }
 0x1041fd3 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x11c6051u32; cpu.esi = v; }
  cpu.push(0x1041fe0); f_01040c19(cpu); if cpu.returned_to != 0x1041fe0 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01041fe1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1041fe1); }
 let mut pc: u32 = 0x1041fe1;
 loop { match pc {
 0x1041fe1 => {
  { let v: u8 = cpu.r8(0x112c97cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fd373u32, v); }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if (cpu.sf != cpu.of) { pc = 0x1041fff; continue; }
  pc = 0x1041fef; continue;
 }
 0x1041fef => {
  { let v: u8 = 0x2u8; cpu.w8(0x112639du32, v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fcd59u32, v); }
  pc = 0x1042004; continue;
 }
 0x1041fff => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x112639du32, v); }
  pc = 0x1042004; continue;
 }
 0x1042004 => {
  { let v: u32 = 0x10fd374u32; cpu.esi = v; }
  { let v: u32 = 0x112636fu32; cpu.edi = v; }
  pc = 0x104200e; continue;
 }
 0x104200e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x104200e; continue; }
  pc = 0x1042018; continue;
 }
 0x1042018 => {
  { let v: u16 = cpu.r16(0x10e3d7fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11263c5u32, v); }
  { let v: u8 = cpu.r8(0x10fd388u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11263a0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104202e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104202e); }
 let mut pc: u32 = 0x104202e;
 loop { match pc {
 0x104202e => {
  { let v: u32 = 0x112636fu32; cpu.esi = v; }
  { let v: u32 = 0x10fd374u32; cpu.edi = v; }
  pc = 0x1042038; continue;
 }
 0x1042038 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1042038; continue; }
  pc = 0x1042042; continue;
 }
 0x1042042 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11263c5u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d7fu32, v); }
  cpu.push(0x1042054); f_01039358(cpu); if cpu.returned_to != 0x1042054 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11263a0u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fd388u32, v); }
  cpu.push(0x1042065); f_01044810(cpu); if cpu.returned_to != 0x1042065 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01042066(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1042066); }
 let mut pc: u32 = 0x1042066;
 loop { match pc {
 0x1042066 => {
  cpu.push(0x104206b); f_0104208c(cpu); if cpu.returned_to != 0x104206b { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x1042076); cpu.call(t); if cpu.returned_to != 0x1042076 { return; }
  cpu.push(0x104207b); f_0104208c(cpu); if cpu.returned_to != 0x104207b { return; }
  { cpu.sub8(cpu.r8(0x10fd3eau32), 0x1u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x104208b; continue; }
  pc = 0x1042084; continue;
 }
 0x1042084 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10fd3eau32, v); }
  pc = 0x104208b; continue;
 }
 0x104208b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104208c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104208c); }
 let mut pc: u32 = 0x104208c;
 loop { match pc {
 0x104208c => {
  { let v: u32 = 0xa8u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x19f7u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10420a1); cpu.call(t); if cpu.returned_to != 0x10420a1 { return; }
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x19f8u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10420b6); cpu.call(t); if cpu.returned_to != 0x10420b6 { return; }
  { let v: u32 = 0xa8u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0xeb2u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10420cb); cpu.call(t); if cpu.returned_to != 0x10420cb { return; }
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0xeb3u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10420e0); cpu.call(t); if cpu.returned_to != 0x10420e0 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10fd3eau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1042147; continue; }
  pc = 0x10420eb; continue;
 }
 0x10420eb => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x104211b; continue; }
  pc = 0x10420ef; continue;
 }
 0x10420ef => {
  { let v: u32 = 0xa8u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x1b09u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042104); cpu.call(t); if cpu.returned_to != 0x1042104 { return; }
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x1b0au32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042119); cpu.call(t); if cpu.returned_to != 0x1042119 { return; }
  pc = 0x1042171; continue;
 }
 0x104211b => {
  { let v: u32 = 0xa8u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x1b07u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042130); cpu.call(t); if cpu.returned_to != 0x1042130 { return; }
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x1b08u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042145); cpu.call(t); if cpu.returned_to != 0x1042145 { return; }
  pc = 0x1042171; continue;
 }
 0x1042147 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0xeb6u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104215c); cpu.call(t); if cpu.returned_to != 0x104215c { return; }
  { let v: u32 = 0x130u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0xeb7u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042171); cpu.call(t); if cpu.returned_to != 0x1042171 { return; }
  pc = 0x1042171; continue;
 }
 0x1042171 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01042172(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1042172); } r_01042172(cpu, 0x1042172); }
pub fn f_01042444(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1042444); } r_01042172(cpu, 0x1042444); }
fn r_01042172(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1042172 => {
  { let v: u32 = 0x18u32; cpu.eax = v; }
  { let v: u32 = 0x18u32; cpu.ebx = v; }
  { let v: u32 = 0xe07u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042187); cpu.call(t); if cpu.returned_to != 0x1042187 { return; }
  { let v: u32 = 0x90u32; cpu.eax = v; }
  { let v: u32 = 0x18u32; cpu.ebx = v; }
  { let v: u32 = 0xe08u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104219c); cpu.call(t); if cpu.returned_to != 0x104219c { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x98u32; cpu.ebx = v; }
  { let v: u32 = 0xe0du32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10421b1); cpu.call(t); if cpu.returned_to != 0x10421b1 { return; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = 0xeb8u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10421c6); cpu.call(t); if cpu.returned_to != 0x10421c6 { return; }
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x100u32; cpu.ebx = v; }
  { let v: u32 = 0xeb9u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10421db); cpu.call(t); if cpu.returned_to != 0x10421db { return; }
  { let v: u32 = 0xa8u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x19f7u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10421f0); cpu.call(t); if cpu.returned_to != 0x10421f0 { return; }
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x19f8u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042205); cpu.call(t); if cpu.returned_to != 0x1042205 { return; }
  { let v: u32 = 0xa8u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0xeb2u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104221a); cpu.call(t); if cpu.returned_to != 0x104221a { return; }
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0xeb3u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104222f); cpu.call(t); if cpu.returned_to != 0x104222f { return; }
  { let v: u32 = 0x60u32; cpu.eax = v; }
  { let v: u32 = 0x68u32; cpu.ebx = v; }
  { let v: u32 = 0x19f4u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042244); cpu.call(t); if cpu.returned_to != 0x1042244 { return; }
  { let v: u32 = 0x108u32; cpu.eax = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = 0xe0eu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042259); cpu.call(t); if cpu.returned_to != 0x1042259 { return; }
  { let v: u32 = 0xc5u32; cpu.eax = v; }
  { let v: u32 = 0x3bu32; cpu.ebx = v; }
  { let v: u32 = 0x19fdu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104226e); cpu.call(t); if cpu.returned_to != 0x104226e { return; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x104228c; continue; }
  pc = 0x1042277; continue;
 }
 0x1042277 => {
  { let v: u32 = 0x108u32; cpu.eax = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = 0x1b02u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104228c); cpu.call(t); if cpu.returned_to != 0x104228c { return; }
  pc = 0x104228c; continue;
 }
 0x104228c => {
  { cpu.sub8(cpu.r8(0x11263beu32), 0x0u8); }
  if cpu.zf { pc = 0x1042328; continue; }
  pc = 0x1042299; continue;
 }
 0x1042299 => {
  { let v: u32 = 0x18u32; cpu.eax = v; }
  { let v: u32 = 0x18u32; cpu.ebx = v; }
  { let v: u32 = 0xbdeu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10422ae); cpu.call(t); if cpu.returned_to != 0x10422ae { return; }
  { let v: u32 = 0x90u32; cpu.eax = v; }
  { let v: u32 = 0x18u32; cpu.ebx = v; }
  { let v: u32 = 0xbdfu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10422c3); cpu.call(t); if cpu.returned_to != 0x10422c3 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11263beu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xe0fu32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263beu32), 0x0u8); }
  if cpu.zf { pc = 0x1042328; continue; }
  pc = 0x10422db; continue;
 }
 0x10422db => {
  { let v: u32 = 0x108u32; cpu.eax = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xe0fu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10422f1); cpu.call(t); if cpu.returned_to != 0x10422f1 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263beu32), 0x2u8); }
  if cpu.zf { pc = 0x1042306; continue; }
  pc = 0x10422fb; continue;
 }
 0x10422fb => {
  { cpu.sub8(cpu.r8(0x11263beu32), 0x3u8); }
  if cpu.zf { pc = 0x1042318; continue; }
  pc = 0x1042304; continue;
 }
 0x1042304 => {
  pc = 0x1042328; continue;
 }
 0x1042306 => {
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x38u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042316); cpu.call(t); if cpu.returned_to != 0x1042316 { return; }
  pc = 0x1042328; continue;
 }
 0x1042318 => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x38u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042328); cpu.call(t); if cpu.returned_to != 0x1042328 { return; }
  pc = 0x1042328; continue;
 }
 0x1042328 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10423f8; continue; }
  pc = 0x1042335; continue;
 }
 0x1042335 => {
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if !cpu.zf { pc = 0x1042494; continue; }
  pc = 0x1042342; continue;
 }
 0x1042342 => {
  { let v: u32 = 0x108u32; cpu.eax = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = 0x1b06u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042357); cpu.call(t); if cpu.returned_to != 0x1042357 { return; }
  { cpu.sub8(cpu.r8(0x11263bfu32), 0x0u8); }
  if cpu.zf { pc = 0x1042494; continue; }
  pc = 0x1042364; continue;
 }
 0x1042364 => {
  { let v: u32 = 0x18u32; cpu.eax = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = 0xbe0u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042379); cpu.call(t); if cpu.returned_to != 0x1042379 { return; }
  { let v: u32 = 0x90u32; cpu.eax = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = 0xbe1u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104238e); cpu.call(t); if cpu.returned_to != 0x104238e { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11263bfu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1b03u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263bfu32), 0x0u8); }
  if cpu.zf { pc = 0x10423f3; continue; }
  pc = 0x10423a6; continue;
 }
 0x10423a6 => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1b03u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10423bc); cpu.call(t); if cpu.returned_to != 0x10423bc { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263bfu32), 0x2u8); }
  if cpu.zf { pc = 0x10423d1; continue; }
  pc = 0x10423c6; continue;
 }
 0x10423c6 => {
  { cpu.sub8(cpu.r8(0x11263bfu32), 0x3u8); }
  if cpu.zf { pc = 0x10423e3; continue; }
  pc = 0x10423cf; continue;
 }
 0x10423cf => {
  pc = 0x10423f3; continue;
 }
 0x10423d1 => {
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10423e1); cpu.call(t); if cpu.returned_to != 0x10423e1 { return; }
  pc = 0x10423f3; continue;
 }
 0x10423e3 => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10423f3); cpu.call(t); if cpu.returned_to != 0x10423f3 { return; }
  pc = 0x10423f3; continue;
 }
 0x10423f3 => {
  pc = 0x1042494; continue;
 }
 0x10423f8 => {
  { cpu.sub8(cpu.r8(0x11263bfu32), 0x0u8); }
  if cpu.zf { pc = 0x1042494; continue; }
  pc = 0x1042405; continue;
 }
 0x1042405 => {
  { let v: u32 = 0x18u32; cpu.eax = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = 0xbe0u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104241a); cpu.call(t); if cpu.returned_to != 0x104241a { return; }
  { let v: u32 = 0x90u32; cpu.eax = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = 0xbe1u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104242f); cpu.call(t); if cpu.returned_to != 0x104242f { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11263bfu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xe12u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263bfu32), 0x0u8); }
  if cpu.zf { pc = 0x1042494; continue; }
  pc = 0x1042447; continue;
 }
 0x1042444 => {
  { let v: u8 = cpu.r8(0xffffffb8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(2))).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0xffffffb8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(2)), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xe12u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104245d); cpu.call(t); if cpu.returned_to != 0x104245d { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263bfu32), 0x2u8); }
  if cpu.zf { pc = 0x1042472; continue; }
  pc = 0x1042467; continue;
 }
 0x1042447 => {
  { let v: u32 = 0x108u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xe12u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104245d); cpu.call(t); if cpu.returned_to != 0x104245d { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263bfu32), 0x2u8); }
  if cpu.zf { pc = 0x1042472; continue; }
  pc = 0x1042467; continue;
 }
 0x1042467 => {
  { cpu.sub8(cpu.r8(0x11263bfu32), 0x3u8); }
  if cpu.zf { pc = 0x1042484; continue; }
  pc = 0x1042470; continue;
 }
 0x1042470 => {
  pc = 0x1042494; continue;
 }
 0x1042472 => {
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042482); cpu.call(t); if cpu.returned_to != 0x1042482 { return; }
  pc = 0x1042494; continue;
 }
 0x1042484 => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042494); cpu.call(t); if cpu.returned_to != 0x1042494 { return; }
  pc = 0x1042494; continue;
 }
 0x1042494 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x104253d; continue; }
  pc = 0x10424a1; continue;
 }
 0x10424a1 => {
  { cpu.sub8(cpu.r8(0x11263c0u32), 0x0u8); }
  if cpu.zf { pc = 0x104253d; continue; }
  pc = 0x10424ae; continue;
 }
 0x10424ae => {
  { let v: u32 = 0x18u32; cpu.eax = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = 0xbe2u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10424c3); cpu.call(t); if cpu.returned_to != 0x10424c3 { return; }
  { let v: u32 = 0x90u32; cpu.eax = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = 0xbe3u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10424d8); cpu.call(t); if cpu.returned_to != 0x10424d8 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11263c0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0xe15u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263c0u32), 0x0u8); }
  if cpu.zf { pc = 0x104253d; continue; }
  pc = 0x10424f0; continue;
 }
 0x10424f0 => {
  { let v: u32 = 0x108u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xe15u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042506); cpu.call(t); if cpu.returned_to != 0x1042506 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263c0u32), 0x2u8); }
  if cpu.zf { pc = 0x104251b; continue; }
  pc = 0x1042510; continue;
 }
 0x1042510 => {
  { cpu.sub8(cpu.r8(0x11263c0u32), 0x3u8); }
  if cpu.zf { pc = 0x104252d; continue; }
  pc = 0x1042519; continue;
 }
 0x1042519 => {
  pc = 0x104253d; continue;
 }
 0x104251b => {
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x58u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104252b); cpu.call(t); if cpu.returned_to != 0x104252b { return; }
  pc = 0x104253d; continue;
 }
 0x104252d => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x58u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104253d); cpu.call(t); if cpu.returned_to != 0x104253d { return; }
  pc = 0x104253d; continue;
 }
 0x104253d => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1042564; continue; }
  pc = 0x1042546; continue;
 }
 0x1042546 => {
  { cpu.sub8(cpu.r8(0x11263c1u32), 0x0u8); }
  if cpu.zf { pc = 0x1042564; continue; }
  pc = 0x104254f; continue;
 }
 0x104254f => {
  { let v: u32 = 0x18u32; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  { let v: u32 = 0xbe4u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042564); cpu.call(t); if cpu.returned_to != 0x1042564 { return; }
  pc = 0x1042564; continue;
 }
 0x1042564 => {
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x0u8); }
  if cpu.zf { pc = 0x1042590; continue; }
  pc = 0x104256d; continue;
 }
 0x104256d => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x98u32; cpu.ebx = v; }
  { let v: u32 = 0xe0cu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x1u8); }
  if !cpu.zf { pc = 0x104258a; continue; }
  pc = 0x1042585; continue;
 }
 0x1042585 => {
  { let v: u32 = 0xe0bu32; cpu.ecx = v; }
  pc = 0x104258a; continue;
 }
 0x104258a => {
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042590); cpu.call(t); if cpu.returned_to != 0x1042590 { return; }
  pc = 0x1042590; continue;
 }
 0x1042590 => {
  { cpu.sub8(cpu.r8(0x11263c3u32), 0x0u8); }
  if cpu.zf { pc = 0x10425e3; continue; }
  pc = 0x1042599; continue;
 }
 0x1042599 => {
  { cpu.sub8(cpu.r8(0x11263c3u32), 0x1u8); }
  if !cpu.zf { pc = 0x10425b9; continue; }
  pc = 0x10425a2; continue;
 }
 0x10425a2 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0xe18u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10425b7); cpu.call(t); if cpu.returned_to != 0x10425b7 { return; }
  pc = 0x10425e3; continue;
 }
 0x10425b9 => {
  { let v: u32 = 0x50u32; cpu.eax = v; }
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = 0xeb0u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10425ce); cpu.call(t); if cpu.returned_to != 0x10425ce { return; }
  { let v: u32 = 0xc8u32; cpu.eax = v; }
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = 0xeb1u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10425e3); cpu.call(t); if cpu.returned_to != 0x10425e3 { return; }
  pc = 0x10425e3; continue;
 }
 0x10425e3 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1042717; continue; }
  pc = 0x10425f0; continue;
 }
 0x10425f0 => {
  { let v: u32 = 0xa8u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x19f7u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042605); cpu.call(t); if cpu.returned_to != 0x1042605 { return; }
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x19f8u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104261a); cpu.call(t); if cpu.returned_to != 0x104261a { return; }
  { let v: u32 = 0xb0u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x1b00u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104262f); cpu.call(t); if cpu.returned_to != 0x104262f { return; }
  { let v: u32 = 0x128u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x1b01u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042644); cpu.call(t); if cpu.returned_to != 0x1042644 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10fd3eau32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10426a9; continue; }
  pc = 0x104264f; continue;
 }
 0x104264f => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x104267f; continue; }
  pc = 0x1042653; continue;
 }
 0x1042653 => {
  { let v: u32 = 0xa8u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x19fbu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042668); cpu.call(t); if cpu.returned_to != 0x1042668 { return; }
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x19fcu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104267d); cpu.call(t); if cpu.returned_to != 0x104267d { return; }
  pc = 0x10426a9; continue;
 }
 0x104267f => {
  { let v: u32 = 0xa8u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x19f9u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042694); cpu.call(t); if cpu.returned_to != 0x1042694 { return; }
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x19fau32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10426a9); cpu.call(t); if cpu.returned_to != 0x10426a9 { return; }
  pc = 0x10426a9; continue;
 }
 0x10426a9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11263c4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x104277c; continue; }
  pc = 0x10426b8; continue;
 }
 0x10426b8 => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x10426eb; continue; }
  pc = 0x10426bc; continue;
 }
 0x10426bc => {
  { let v: u32 = 0xb0u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x19f2u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10426d1); cpu.call(t); if cpu.returned_to != 0x10426d1 { return; }
  { let v: u32 = 0x128u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x19f3u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10426e6); cpu.call(t); if cpu.returned_to != 0x10426e6 { return; }
  pc = 0x104277c; continue;
 }
 0x10426eb => {
  { let v: u32 = 0xb0u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x19f0u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042700); cpu.call(t); if cpu.returned_to != 0x1042700 { return; }
  { let v: u32 = 0x128u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x19f1u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042715); cpu.call(t); if cpu.returned_to != 0x1042715 { return; }
  pc = 0x104277c; continue;
 }
 0x1042717 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11263c4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x104277c; continue; }
  pc = 0x1042722; continue;
 }
 0x1042722 => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x1042752; continue; }
  pc = 0x1042726; continue;
 }
 0x1042726 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0xeb6u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104273b); cpu.call(t); if cpu.returned_to != 0x104273b { return; }
  { let v: u32 = 0x130u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0xeb7u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042750); cpu.call(t); if cpu.returned_to != 0x1042750 { return; }
  pc = 0x104277c; continue;
 }
 0x1042752 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0xeb4u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042767); cpu.call(t); if cpu.returned_to != 0x1042767 { return; }
  { let v: u32 = 0x130u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0xeb5u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104277c); cpu.call(t); if cpu.returned_to != 0x104277c { return; }
  pc = 0x104277c; continue;
 }
 0x104277c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x112639du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x112639du32, v); }
  { let v: u8 = cpu.r8(0x112639du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1130fbcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.eax = v; }
  { let v: u32 = 0x3fu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x11263beu32), 0x0u8); }
  if cpu.zf { pc = 0x10427b0; continue; }
  pc = 0x10427a9; continue;
 }
 0x10427a9 => {
  cpu.push(0x10427ae); f_0104681f(cpu); if cpu.returned_to != 0x10427ae { return; }
  pc = 0x10427b5; continue;
 }
 0x10427b0 => {
  cpu.push(0x10427b5); f_0104682e(cpu); if cpu.returned_to != 0x10427b5 { return; }
  pc = 0x10427b5; continue;
 }
 0x10427b5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11263c5u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112639du32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x10427cd; continue; }
  pc = 0x10427c8; continue;
 }
 0x10427c8 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x10427cd; continue;
 }
 0x10427cd => {
  { let v: u32 = cpu.r32(0x10fcd35u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x11263a0u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3fu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3e93u32); cpu.mul8(v); }
  { let v: u32 = 0x10c2394u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.eax = v; }
  { let v: u32 = 0x4fu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x11263bfu32), 0x0u8); }
  if cpu.zf { pc = 0x1042815; continue; }
  pc = 0x104280e; continue;
 }
 0x104280e => {
  cpu.push(0x1042813); f_0104681f(cpu); if cpu.returned_to != 0x1042813 { return; }
  pc = 0x104281a; continue;
 }
 0x1042815 => {
  cpu.push(0x104281a); f_0104682e(cpu); if cpu.returned_to != 0x104281a { return; }
  pc = 0x104281a; continue;
 }
 0x104281a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11263c5u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x112639du32), 0x2u8); }
  if cpu.zf { pc = 0x104282e; continue; }
  pc = 0x104282a; continue;
 }
 0x104282a => {
  { let v: u16 = 0x2u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x104282e; continue;
 }
 0x104282e => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1131004u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.eax = v; }
  { let v: u32 = 0x61u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x11263c0u32), 0x0u8); }
  if cpu.zf { pc = 0x1042851; continue; }
  pc = 0x104284a; continue;
 }
 0x104284a => {
  cpu.push(0x104284f); f_0104681f(cpu); if cpu.returned_to != 0x104284f { return; }
  pc = 0x1042856; continue;
 }
 0x1042851 => {
  cpu.push(0x1042856); f_0104682e(cpu); if cpu.returned_to != 0x1042856 { return; }
  pc = 0x1042856; continue;
 }
 0x1042856 => {
  { let v: u32 = 0xc8u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = 0x112636fu32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x1u8); }
  if cpu.zf { pc = 0x104287e; continue; }
  pc = 0x104286e; continue;
 }
 0x104286e => {
  { cpu.sub8(cpu.r8(0x11263c1u32), 0x0u8); }
  if !cpu.zf { pc = 0x104287e; continue; }
  pc = 0x1042877; continue;
 }
 0x1042877 => {
  cpu.push(0x104287c); f_0104682e(cpu); if cpu.returned_to != 0x104287c { return; }
  pc = 0x10428ad; continue;
 }
 0x104287e => {
  cpu.push(0x1042883); f_0104681f(cpu); if cpu.returned_to != 0x1042883 { return; }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x1u8); }
  if !cpu.zf { pc = 0x10428ad; continue; }
  pc = 0x104288c; continue;
 }
 0x104288c => {
  { cpu.sub8(cpu.r8(0x11263ecu32), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x10428ad; continue; }
  pc = 0x1042895; continue;
 }
 0x1042895 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10fcdb6u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10fcdbau32); cpu.ebx = v; }
  { let v: u32 = 0x39u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10428ac); cpu.call(t); if cpu.returned_to != 0x10428ac { return; }
  { cpu.popad(); }
  pc = 0x10428ad; continue;
 }
 0x10428ad => {
  { cpu.sub16(cpu.r16(0x10fd3efu32), 0x0u16); }
  if cpu.zf { pc = 0x10428d3; continue; }
  pc = 0x10428b7; continue;
 }
 0x10428b7 => {
  { let v: u32 = 0x60u32; cpu.eax = v; }
  { let v: u32 = 0x68u32; cpu.ebx = v; }
  { let v: u32 = 0x19f5u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10428cc); cpu.call(t); if cpu.returned_to != 0x10428cc { return; }
  { let v: u16 = cpu.dec16(cpu.r16(0x10fd3efu32)); cpu.w16(0x10fd3efu32, v); }
  pc = 0x10428d3; continue;
 }
 0x10428d3 => {
  { cpu.sub16(cpu.r16(0x10fd3f1u32), 0x0u16); }
  if cpu.zf { pc = 0x10428f9; continue; }
  pc = 0x10428dd; continue;
 }
 0x10428dd => {
  { let v: u32 = 0x60u32; cpu.eax = v; }
  { let v: u32 = 0x68u32; cpu.ebx = v; }
  { let v: u32 = 0x19f6u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10428f2); cpu.call(t); if cpu.returned_to != 0x10428f2 { return; }
  { let v: u16 = cpu.dec16(cpu.r16(0x10fd3f1u32)); cpu.w16(0x10fd3f1u32, v); }
  pc = 0x10428f9; continue;
 }
 0x10428f9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010428fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10428fa); }
 let mut pc: u32 = 0x10428fa;
 loop { match pc {
 0x10428fa => {
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x1042904); f_010725e4(cpu); if cpu.returned_to != 0x1042904 { return; }
  cpu.push(0x1042909); f_01042a0d(cpu); if cpu.returned_to != 0x1042909 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x104291b); f_010385ad(cpu); if cpu.returned_to != 0x104291b { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x1042923); cpu.call(t); if cpu.returned_to != 0x1042923 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  { let v: u32 = 0x10fcd70u32; cpu.edx = v; }
  cpu.push(0x1042934); f_01045c08(cpu); if cpu.returned_to != 0x1042934 { return; }
  cpu.push(0x1042939); f_01042ace(cpu); if cpu.returned_to != 0x1042939 { return; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x1042941); cpu.call(t); if cpu.returned_to != 0x1042941 { return; }
  cpu.push(0x1042946); f_0107162e(cpu); if cpu.returned_to != 0x1042946 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u16 = 0xad8u16; cpu.w16(0x10e3db7u32, v); }
  { let v: u32 = 0x11a3af0u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1042964); f_0107259c(cpu); if cpu.returned_to != 0x1042964 { return; }
  { let v: u32 = 0x1128480u32; cpu.esi = v; }
  cpu.push(0x104296e); f_01059ad5(cpu); if cpu.returned_to != 0x104296e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11263e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd431u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263ceu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263cdu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c2u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c6u32, v); }
  pc = 0x1042998; continue;
 }
 0x1042998 => {
  cpu.push(0x104299d); f_01042ace(cpu); if cpu.returned_to != 0x104299d { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11263ceu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263cdu32, v); }
  cpu.push(0x10429b0); f_01040c94(cpu); if cpu.returned_to != 0x10429b0 { return; }
  cpu.push(0x10429b5); f_01072629(cpu); if cpu.returned_to != 0x10429b5 { return; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x10429c0); cpu.call(t); if cpu.returned_to != 0x10429c0 { return; }
  cpu.push(0x10429c5); f_01059ae3(cpu); if cpu.returned_to != 0x10429c5 { return; }
  cpu.push(0x10429ca); f_0105eb1d(cpu); if cpu.returned_to != 0x10429ca { return; }
  { cpu.sub8((cpu.eax as u8), 0x71u8); }
  if cpu.zf { pc = 0x1042a04; continue; }
  pc = 0x10429ce; continue;
 }
 0x10429ce => {
  { cpu.sub8((cpu.eax as u8), 0x51u8); }
  if cpu.zf { pc = 0x1042a04; continue; }
  pc = 0x10429d2; continue;
 }
 0x10429d2 => {
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x6u8); }
  if cpu.zf { pc = 0x10429f4; continue; }
  pc = 0x10429db; continue;
 }
 0x10429db => {
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x6u8); }
  if cpu.zf { pc = 0x10429f9; continue; }
  pc = 0x10429e4; continue;
 }
 0x10429e4 => {
  cpu.push(0x10429e9); f_0105eb1d(cpu); if cpu.returned_to != 0x10429e9 { return; }
  if !cpu.cf { cpu.poll(); pc = 0x1042998; continue; }
  pc = 0x10429ef; continue;
 }
 0x10429ef => {
  cpu.poll(); pc = 0x1042998; continue;
 }
 0x10429f4 => {
  cpu.push(0x10429f9); f_01042a23(cpu); if cpu.returned_to != 0x10429f9 { return; }
  pc = 0x10429f9; continue;
 }
 0x10429f9 => {
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x1042a03); f_010725e4(cpu); if cpu.returned_to != 0x1042a03 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1042a04 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fcdcfu32, v); }
  cpu.poll(); pc = 0x10429f9; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01042a0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1042a0d); }
 let mut pc: u32 = 0x1042a0d;
 loop { match pc {
 0x1042a0d => {
  { let v: u16 = cpu.r16(0x112639eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fd3a2u32, v); }
  { let v: u8 = cpu.r8(0x10fcd59u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fd3a1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01042a23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1042a23); }
 let mut pc: u32 = 0x1042a23;
 loop { match pc {
 0x1042a23 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10fd3a2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x112639eu32, v); }
  { let v: u8 = cpu.r8(0x10fd3a1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fcd59u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01042a3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1042a3b); }
 let mut pc: u32 = 0x1042a3b;
 loop { match pc {
 0x1042a3b => {
  { let v: u32 = 0xa8u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x1a2eu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042a50); cpu.call(t); if cpu.returned_to != 0x1042a50 { return; }
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x1a2fu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042a65); cpu.call(t); if cpu.returned_to != 0x1042a65 { return; }
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x0u8); }
  if cpu.zf { pc = 0x1042acd; continue; }
  pc = 0x1042a6e; continue;
 }
 0x1042a6e => {
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x1u8); }
  if cpu.zf { pc = 0x1042aa3; continue; }
  pc = 0x1042a77; continue;
 }
 0x1042a77 => {
  { let v: u32 = 0xa8u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x1a30u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042a8c); cpu.call(t); if cpu.returned_to != 0x1042a8c { return; }
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x1a31u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042aa1); cpu.call(t); if cpu.returned_to != 0x1042aa1 { return; }
  pc = 0x1042acd; continue;
 }
 0x1042aa3 => {
  { let v: u32 = 0xa8u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x1a2cu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042ab8); cpu.call(t); if cpu.returned_to != 0x1042ab8 { return; }
  { let v: u32 = 0x120u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x1a2du32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042acd); cpu.call(t); if cpu.returned_to != 0x1042acd { return; }
  pc = 0x1042acd; continue;
 }
 0x1042acd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01042ace(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1042ace); }
 let mut pc: u32 = 0x1042ace;
 loop { match pc {
 0x1042ace => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x28u32; cpu.ebx = v; }
  { let v: u32 = 0x1a03u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042ae3); cpu.call(t); if cpu.returned_to != 0x1042ae3 { return; }
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = 0x1a10u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042af8); cpu.call(t); if cpu.returned_to != 0x1042af8 { return; }
  { let v: u32 = 0x88u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = 0x1a11u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042b0d); cpu.call(t); if cpu.returned_to != 0x1042b0d { return; }
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = 0x19feu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042b22); cpu.call(t); if cpu.returned_to != 0x1042b22 { return; }
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = 0x19ffu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042b37); cpu.call(t); if cpu.returned_to != 0x1042b37 { return; }
  { let v: u32 = 0xf8u32; cpu.eax = v; }
  { let v: u32 = 0x38u32; cpu.ebx = v; }
  { let v: u32 = 0x1a0cu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042b4c); cpu.call(t); if cpu.returned_to != 0x1042b4c { return; }
  { let v: u32 = 0xd0u32; cpu.eax = v; }
  { let v: u32 = 0x47u32; cpu.ebx = v; }
  { let v: u32 = 0x1a00u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042b61); cpu.call(t); if cpu.returned_to != 0x1042b61 { return; }
  { cpu.sub8(cpu.r8(0x11263ceu32), 0x0u8); }
  if cpu.zf { pc = 0x1042bfd; continue; }
  pc = 0x1042b6e; continue;
 }
 0x1042b6e => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x28u32; cpu.ebx = v; }
  { let v: u32 = 0x1a05u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042b83); cpu.call(t); if cpu.returned_to != 0x1042b83 { return; }
  { let v: u32 = 0x98u32; cpu.eax = v; }
  { let v: u32 = 0x28u32; cpu.ebx = v; }
  { let v: u32 = 0x1a06u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042b98); cpu.call(t); if cpu.returned_to != 0x1042b98 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x11263ceu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1a09u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263ceu32), 0x0u8); }
  if cpu.zf { pc = 0x1042bfd; continue; }
  pc = 0x1042bb0; continue;
 }
 0x1042bb0 => {
  { let v: u32 = 0xf8u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1a09u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042bc6); cpu.call(t); if cpu.returned_to != 0x1042bc6 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263ceu32), 0x2u8); }
  if cpu.zf { pc = 0x1042bdb; continue; }
  pc = 0x1042bd0; continue;
 }
 0x1042bd0 => {
  { cpu.sub8(cpu.r8(0x11263ceu32), 0x3u8); }
  if cpu.zf { pc = 0x1042bed; continue; }
  pc = 0x1042bd9; continue;
 }
 0x1042bd9 => {
  pc = 0x1042bfd; continue;
 }
 0x1042bdb => {
  { let v: u32 = 0x110u32; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042beb); cpu.call(t); if cpu.returned_to != 0x1042beb { return; }
  pc = 0x1042bfd; continue;
 }
 0x1042bed => {
  { let v: u32 = 0x108u32; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042bfd); cpu.call(t); if cpu.returned_to != 0x1042bfd { return; }
  pc = 0x1042bfd; continue;
 }
 0x1042bfd => {
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x0u8); }
  if cpu.zf { pc = 0x1042c50; continue; }
  pc = 0x1042c06; continue;
 }
 0x1042c06 => {
  { let v: u32 = 0x1a0du32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x1u8); }
  if cpu.zf { pc = 0x1042c40; continue; }
  pc = 0x1042c14; continue;
 }
 0x1042c14 => {
  { let v: u32 = 0x1a0eu32; cpu.ecx = v; }
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042c29); cpu.call(t); if cpu.returned_to != 0x1042c29 { return; }
  { let v: u32 = 0x88u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = 0x1a0fu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042c3e); cpu.call(t); if cpu.returned_to != 0x1042c3e { return; }
  pc = 0x1042c50; continue;
 }
 0x1042c40 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042c50); cpu.call(t); if cpu.returned_to != 0x1042c50 { return; }
  pc = 0x1042c50; continue;
 }
 0x1042c50 => {
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x0u8); }
  if cpu.zf { pc = 0x1042c9b; continue; }
  pc = 0x1042c59; continue;
 }
 0x1042c59 => {
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = 0x1a12u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x1u8); }
  if cpu.zf { pc = 0x1042c95; continue; }
  pc = 0x1042c71; continue;
 }
 0x1042c71 => {
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = 0x1a13u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042c86); cpu.call(t); if cpu.returned_to != 0x1042c86 { return; }
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = 0x1a14u32; cpu.ecx = v; }
  pc = 0x1042c95; continue;
 }
 0x1042c95 => {
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042c9b); cpu.call(t); if cpu.returned_to != 0x1042c9b { return; }
  pc = 0x1042c9b; continue;
 }
 0x1042c9b => {
  { let v: u16 = cpu.r16(0x112639eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1042cab); f_01059cfe(cpu); if cpu.returned_to != 0x1042cab { return; }
  { let v: u16 = 0xddu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4au16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10ef397u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x11263ceu32), 0x0u8); }
  if cpu.zf { pc = 0x1042cc8; continue; }
  pc = 0x1042cc1; continue;
 }
 0x1042cc1 => {
  cpu.push(0x1042cc6); f_0104681f(cpu); if cpu.returned_to != 0x1042cc6 { return; }
  pc = 0x1042ccd; continue;
 }
 0x1042cc8 => {
  cpu.push(0x1042ccd); f_0104682e(cpu); if cpu.returned_to != 0x1042ccd { return; }
  pc = 0x1042ccd; continue;
 }
 0x1042ccd => {
  { let v: u32 = 0xd7u32; cpu.eax = v; }
  { let v: u32 = 0x5bu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1130fe0u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x10fcd59u32), 0x0u8); }
  if cpu.zf { pc = 0x1042cec; continue; }
  pc = 0x1042ce6; continue;
 }
 0x1042ce6 => {
  { let v: u32 = cpu.r32(0x1130fdcu32); cpu.esi = v; }
  pc = 0x1042cec; continue;
 }
 0x1042cec => {
  { cpu.sub8(cpu.r8(0x11263cdu32), 0x0u8); }
  if cpu.zf { pc = 0x1042cfc; continue; }
  pc = 0x1042cf5; continue;
 }
 0x1042cf5 => {
  cpu.push(0x1042cfa); f_0104681f(cpu); if cpu.returned_to != 0x1042cfa { return; }
  pc = 0x1042d01; continue;
 }
 0x1042cfc => {
  cpu.push(0x1042d01); f_0104682e(cpu); if cpu.returned_to != 0x1042d01 { return; }
  pc = 0x1042d01; continue;
 }
 0x1042d01 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01042d02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1042d02); }
 let mut pc: u32 = 0x1042d02;
 loop { match pc {
 0x1042d02 => {
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x1042d0c); f_010725e4(cpu); if cpu.returned_to != 0x1042d0c { return; }
  cpu.push(0x1042d11); f_01042df9(cpu); if cpu.returned_to != 0x1042d11 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1042d23); f_010385ad(cpu); if cpu.returned_to != 0x1042d23 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x1042d2b); cpu.call(t); if cpu.returned_to != 0x1042d2b { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  { let v: u32 = 0x10fcd98u32; cpu.edx = v; }
  cpu.push(0x1042d3c); f_01045c08(cpu); if cpu.returned_to != 0x1042d3c { return; }
  cpu.push(0x1042d41); f_01042e71(cpu); if cpu.returned_to != 0x1042d41 { return; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x1042d49); cpu.call(t); if cpu.returned_to != 0x1042d49 { return; }
  cpu.push(0x1042d4e); f_0107162e(cpu); if cpu.returned_to != 0x1042d4e { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u16 = 0xad8u16; cpu.w16(0x10e3db7u32, v); }
  { let v: u32 = 0x11a3af0u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1042d6c); f_0107259c(cpu); if cpu.returned_to != 0x1042d6c { return; }
  { let v: u32 = 0x11284e6u32; cpu.esi = v; }
  cpu.push(0x1042d76); f_01059ad5(cpu); if cpu.returned_to != 0x1042d76 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11263e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd431u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c2u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c6u32, v); }
  pc = 0x1042d92; continue;
 }
 0x1042d92 => {
  cpu.push(0x1042d97); f_01042e71(cpu); if cpu.returned_to != 0x1042d97 { return; }
  cpu.push(0x1042d9c); f_01045429(cpu); if cpu.returned_to != 0x1042d9c { return; }
  cpu.push(0x1042da1); f_01040c94(cpu); if cpu.returned_to != 0x1042da1 { return; }
  cpu.push(0x1042da6); f_01040a0b(cpu); if cpu.returned_to != 0x1042da6 { return; }
  cpu.push(0x1042dab); f_01072629(cpu); if cpu.returned_to != 0x1042dab { return; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x1042db6); cpu.call(t); if cpu.returned_to != 0x1042db6 { return; }
  cpu.push(0x1042dbb); f_01059ae3(cpu); if cpu.returned_to != 0x1042dbb { return; }
  cpu.push(0x1042dc0); f_0105eb1d(cpu); if cpu.returned_to != 0x1042dc0 { return; }
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if cpu.zf { pc = 0x1042de0; continue; }
  pc = 0x1042dc4; continue;
 }
 0x1042dc4 => {
  { cpu.sub8((cpu.eax as u8), 0x71u8); }
  if cpu.zf { pc = 0x1042df0; continue; }
  pc = 0x1042dc8; continue;
 }
 0x1042dc8 => {
  { cpu.sub8((cpu.eax as u8), 0x51u8); }
  if cpu.zf { pc = 0x1042df0; continue; }
  pc = 0x1042dcc; continue;
 }
 0x1042dcc => {
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x6u8); }
  if cpu.zf { pc = 0x1042de0; continue; }
  pc = 0x1042dd5; continue;
 }
 0x1042dd5 => {
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x6u8); }
  if cpu.zf { pc = 0x1042de5; continue; }
  pc = 0x1042dde; continue;
 }
 0x1042dde => {
  cpu.poll(); pc = 0x1042d92; continue;
 }
 0x1042de0 => {
  cpu.push(0x1042de5); f_01042e19(cpu); if cpu.returned_to != 0x1042de5 { return; }
  pc = 0x1042de5; continue;
 }
 0x1042de5 => {
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x1042def); f_010725e4(cpu); if cpu.returned_to != 0x1042def { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1042df0 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fcdcfu32, v); }
  cpu.poll(); pc = 0x1042de5; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01042df9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1042df9); }
 let mut pc: u32 = 0x1042df9;
 loop { match pc {
 0x1042df9 => {
  { let v: u8 = cpu.r8(0x1124bfdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fd389u32, v); }
  { let v: u16 = cpu.r16(0x10e3e81u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fd38au32, v); }
  { let v: u8 = cpu.r8(0x11263c7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fd38bu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01042e19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1042e19); }
 let mut pc: u32 = 0x1042e19;
 loop { match pc {
 0x1042e19 => {
  { let v: u8 = cpu.r8(0x10fd389u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124bfdu32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10fd38au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e81u32, v); }
  { let v: u8 = cpu.r8(0x10fd38bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd52eu32.wrapping_add(cpu.eax)), 0xffu8); }
  if !cpu.zf { pc = 0x1042e6b; continue; }
  pc = 0x1042e3e; continue;
 }
 0x1042e3e => {
  { let v: u8 = 0x1u8; cpu.w8(0x11263c7u32, v); }
  pc = 0x1042e45; continue;
 }
 0x1042e45 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11263c7u32)); cpu.w8(0x11263c7u32, v); }
  { cpu.sub8(cpu.r8(0x11263c7u32), 0x10u8); }
  if !cpu.zf { pc = 0x1042e5b; continue; }
  pc = 0x1042e54; continue;
 }
 0x1042e54 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11263c7u32, v); }
  pc = 0x1042e5b; continue;
 }
 0x1042e5b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11263c7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd52eu32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x1042e45; continue; }
  pc = 0x1042e6b; continue;
 }
 0x1042e6b => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11263c7u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01042e71(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1042e71); }
 let mut pc: u32 = 0x1042e71;
 loop { match pc {
 0x1042e71 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = 0x1a43u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042e86); cpu.call(t); if cpu.returned_to != 0x1042e86 { return; }
  { let v: u32 = 0x100u32; cpu.eax = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = 0x1a42u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042e9b); cpu.call(t); if cpu.returned_to != 0x1042e9b { return; }
  { let v: u32 = 0xc5u32; cpu.eax = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v: u32 = 0x1a02u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042eb0); cpu.call(t); if cpu.returned_to != 0x1042eb0 { return; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x1a44u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042ec5); cpu.call(t); if cpu.returned_to != 0x1042ec5 { return; }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x1a45u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042eda); cpu.call(t); if cpu.returned_to != 0x1042eda { return; }
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x169eu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042eef); cpu.call(t); if cpu.returned_to != 0x1042eef { return; }
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x169fu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042f04); cpu.call(t); if cpu.returned_to != 0x1042f04 { return; }
  { cpu.sub8(cpu.r8(0x1126364u32), 0x0u8); }
  if cpu.zf { pc = 0x1042f87; continue; }
  pc = 0x1042f0d; continue;
 }
 0x1042f0d => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = 0x1a32u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042f22); cpu.call(t); if cpu.returned_to != 0x1042f22 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1126364u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1a36u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126364u32), 0x0u8); }
  if cpu.zf { pc = 0x1042f87; continue; }
  pc = 0x1042f3a; continue;
 }
 0x1042f3a => {
  { let v: u32 = 0x100u32; cpu.eax = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1a36u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042f50); cpu.call(t); if cpu.returned_to != 0x1042f50 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126364u32), 0x2u8); }
  if cpu.zf { pc = 0x1042f65; continue; }
  pc = 0x1042f5a; continue;
 }
 0x1042f5a => {
  { cpu.sub8(cpu.r8(0x1126364u32), 0x3u8); }
  if cpu.zf { pc = 0x1042f77; continue; }
  pc = 0x1042f63; continue;
 }
 0x1042f63 => {
  pc = 0x1042f87; continue;
 }
 0x1042f65 => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x38u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042f75); cpu.call(t); if cpu.returned_to != 0x1042f75 { return; }
  pc = 0x1042f87; continue;
 }
 0x1042f77 => {
  { let v: u32 = 0x110u32; cpu.eax = v; }
  { let v: u32 = 0x38u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042f87); cpu.call(t); if cpu.returned_to != 0x1042f87 { return; }
  pc = 0x1042f87; continue;
 }
 0x1042f87 => {
  { cpu.sub8(cpu.r8(0x1126365u32), 0x0u8); }
  if cpu.zf { pc = 0x104300a; continue; }
  pc = 0x1042f90; continue;
 }
 0x1042f90 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = 0x1a34u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042fa5); cpu.call(t); if cpu.returned_to != 0x1042fa5 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1126365u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1a39u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126365u32), 0x0u8); }
  if cpu.zf { pc = 0x104300a; continue; }
  pc = 0x1042fbd; continue;
 }
 0x1042fbd => {
  { let v: u32 = 0x100u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1a39u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042fd3); cpu.call(t); if cpu.returned_to != 0x1042fd3 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126365u32), 0x2u8); }
  if cpu.zf { pc = 0x1042fe8; continue; }
  pc = 0x1042fdd; continue;
 }
 0x1042fdd => {
  { cpu.sub8(cpu.r8(0x1126365u32), 0x3u8); }
  if cpu.zf { pc = 0x1042ffa; continue; }
  pc = 0x1042fe6; continue;
 }
 0x1042fe6 => {
  pc = 0x104300a; continue;
 }
 0x1042fe8 => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1042ff8); cpu.call(t); if cpu.returned_to != 0x1042ff8 { return; }
  pc = 0x104300a; continue;
 }
 0x1042ffa => {
  { let v: u32 = 0x110u32; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104300a); cpu.call(t); if cpu.returned_to != 0x104300a { return; }
  pc = 0x104300a; continue;
 }
 0x104300a => {
  { cpu.sub8(cpu.r8(0x1126366u32), 0x0u8); }
  if cpu.zf { pc = 0x104308d; continue; }
  pc = 0x1043013; continue;
 }
 0x1043013 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x38u32; cpu.ebx = v; }
  { let v: u32 = 0x1a33u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043028); cpu.call(t); if cpu.returned_to != 0x1043028 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1126366u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1a3cu32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126366u32), 0x0u8); }
  if cpu.zf { pc = 0x104308d; continue; }
  pc = 0x1043040; continue;
 }
 0x1043040 => {
  { let v: u32 = 0x100u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1a3cu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043056); cpu.call(t); if cpu.returned_to != 0x1043056 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126366u32), 0x2u8); }
  if cpu.zf { pc = 0x104306b; continue; }
  pc = 0x1043060; continue;
 }
 0x1043060 => {
  { cpu.sub8(cpu.r8(0x1126366u32), 0x3u8); }
  if cpu.zf { pc = 0x104307d; continue; }
  pc = 0x1043069; continue;
 }
 0x1043069 => {
  pc = 0x104308d; continue;
 }
 0x104306b => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104307b); cpu.call(t); if cpu.returned_to != 0x104307b { return; }
  pc = 0x104308d; continue;
 }
 0x104307d => {
  { let v: u32 = 0x110u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104308d); cpu.call(t); if cpu.returned_to != 0x104308d { return; }
  pc = 0x104308d; continue;
 }
 0x104308d => {
  { cpu.sub8(cpu.r8(0x1126367u32), 0x0u8); }
  if cpu.zf { pc = 0x1043110; continue; }
  pc = 0x1043096; continue;
 }
 0x1043096 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0x1a35u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10430ab); cpu.call(t); if cpu.returned_to != 0x10430ab { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1126367u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1a3fu32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126367u32), 0x0u8); }
  if cpu.zf { pc = 0x1043110; continue; }
  pc = 0x10430c3; continue;
 }
 0x10430c3 => {
  { let v: u32 = 0x100u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x1a3fu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10430d9); cpu.call(t); if cpu.returned_to != 0x10430d9 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126367u32), 0x2u8); }
  if cpu.zf { pc = 0x10430ee; continue; }
  pc = 0x10430e3; continue;
 }
 0x10430e3 => {
  { cpu.sub8(cpu.r8(0x1126367u32), 0x3u8); }
  if cpu.zf { pc = 0x1043100; continue; }
  pc = 0x10430ec; continue;
 }
 0x10430ec => {
  pc = 0x1043110; continue;
 }
 0x10430ee => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10430fe); cpu.call(t); if cpu.returned_to != 0x10430fe { return; }
  pc = 0x1043110; continue;
 }
 0x1043100 => {
  { let v: u32 = 0x110u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043110); cpu.call(t); if cpu.returned_to != 0x1043110 { return; }
  pc = 0x1043110; continue;
 }
 0x1043110 => {
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x0u8); }
  if cpu.zf { pc = 0x1043163; continue; }
  pc = 0x1043119; continue;
 }
 0x1043119 => {
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x1u8); }
  if cpu.zf { pc = 0x104314e; continue; }
  pc = 0x1043122; continue;
 }
 0x1043122 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x1a47u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043137); cpu.call(t); if cpu.returned_to != 0x1043137 { return; }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x1a48u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104314c); cpu.call(t); if cpu.returned_to != 0x104314c { return; }
  pc = 0x1043163; continue;
 }
 0x104314e => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = 0x1a46u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043163); cpu.call(t); if cpu.returned_to != 0x1043163 { return; }
  pc = 0x1043163; continue;
 }
 0x1043163 => {
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x0u8); }
  if cpu.zf { pc = 0x10431bb; continue; }
  pc = 0x104316c; continue;
 }
 0x104316c => {
  { let v: u32 = 0x1a12u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x1u8); }
  if cpu.zf { pc = 0x10431a6; continue; }
  pc = 0x104317a; continue;
 }
 0x104317a => {
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x169bu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104318f); cpu.call(t); if cpu.returned_to != 0x104318f { return; }
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x169cu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10431a4); cpu.call(t); if cpu.returned_to != 0x10431a4 { return; }
  pc = 0x10431bb; continue;
 }
 0x10431a6 => {
  { let v: u32 = 0xa0u32; cpu.eax = v; }
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = 0x169du32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10431bb); cpu.call(t); if cpu.returned_to != 0x10431bb { return; }
  pc = 0x10431bb; continue;
 }
 0x10431bb => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1124bfdu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x112ca44u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x1u16); }
  if !cpu.zf { pc = 0x10431d9; continue; }
  pc = 0x10431d2; continue;
 }
 0x10431d2 => {
  { let v: u32 = 0x10f7a1fu32; cpu.esi = v; }
  pc = 0x1043203; continue;
 }
 0x10431d9 => {
  { let v: u16 = 0xc200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  cpu.push(0x10431f1); f_01059cfe(cpu); if cpu.returned_to != 0x10431f1 { return; }
  { let v: u32 = 0x10ef397u32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x10431fb; continue;
 }
 0x10431fb => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x30u8); }
  if !cpu.zf { pc = 0x1043203; continue; }
  pc = 0x1043200; continue;
 }
 0x1043200 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10431fb; continue; }
  pc = 0x1043203; continue;
 }
 0x1043203 => {
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x1126364u32), 0x0u8); }
  if cpu.zf { pc = 0x104321d; continue; }
  pc = 0x1043216; continue;
 }
 0x1043216 => {
  cpu.push(0x104321b); f_0104681f(cpu); if cpu.returned_to != 0x104321b { return; }
  pc = 0x1043222; continue;
 }
 0x104321d => {
  cpu.push(0x1043222); f_0104682e(cpu); if cpu.returned_to != 0x1043222 { return; }
  pc = 0x1043222; continue;
 }
 0x1043222 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11263c7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if (cpu.sf != cpu.of) { pc = 0x104324c; continue; }
  pc = 0x104322f; continue;
 }
 0x104322f => {
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1194208u32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1194209u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x119420au32, v); }
  pc = 0x104325a; continue;
 }
 0x104324c => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1194208u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1194209u32, v); }
  pc = 0x104325a; continue;
 }
 0x104325a => {
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = 0x1194208u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x1126366u32), 0x0u8); }
  if cpu.zf { pc = 0x1043279; continue; }
  pc = 0x1043272; continue;
 }
 0x1043272 => {
  cpu.push(0x1043277); f_0104681f(cpu); if cpu.returned_to != 0x1043277 { return; }
  pc = 0x104327e; continue;
 }
 0x1043279 => {
  cpu.push(0x104327e); f_0104682e(cpu); if cpu.returned_to != 0x104327e { return; }
  pc = 0x104327e; continue;
 }
 0x104327e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3e81u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x31u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1194208u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1194209u32, v); }
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = 0x1194208u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x1126365u32), 0x0u8); }
  if cpu.zf { pc = 0x10432b8; continue; }
  pc = 0x10432b1; continue;
 }
 0x10432b1 => {
  cpu.push(0x10432b6); f_0104681f(cpu); if cpu.returned_to != 0x10432b6 { return; }
  pc = 0x10432bd; continue;
 }
 0x10432b8 => {
  cpu.push(0x10432bd); f_0104682e(cpu); if cpu.returned_to != 0x10432bd { return; }
  pc = 0x10432bd; continue;
 }
 0x10432bd => {
  { let v: u16 = cpu.r16(0x1124e8au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x10432cd); f_01045c71(cpu); if cpu.returned_to != 0x10432cd { return; }
  { let v: u16 = 0xccu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x70u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10ef397u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x1126367u32), 0x0u8); }
  if cpu.zf { pc = 0x10432ea; continue; }
  pc = 0x10432e3; continue;
 }
 0x10432e3 => {
  cpu.push(0x10432e8); f_0104681f(cpu); if cpu.returned_to != 0x10432e8 { return; }
  pc = 0x10432ef; continue;
 }
 0x10432ea => {
  cpu.push(0x10432ef); f_0104682e(cpu); if cpu.returned_to != 0x10432ef { return; }
  pc = 0x10432ef; continue;
 }
 0x10432ef => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010432f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10432f0); }
 let mut pc: u32 = 0x10432f0;
 loop { match pc {
 0x10432f0 => {
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x10432fa); f_010725e4(cpu); if cpu.returned_to != 0x10432fa { return; }
  cpu.push(0x10432ff); f_01043420(cpu); if cpu.returned_to != 0x10432ff { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1043311); f_010385ad(cpu); if cpu.returned_to != 0x1043311 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x1043319); cpu.call(t); if cpu.returned_to != 0x1043319 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  { let v: u32 = 0x10fcd78u32; cpu.edx = v; }
  cpu.push(0x104332a); f_01045c08(cpu); if cpu.returned_to != 0x104332a { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  cpu.push(0x1043336); f_01045429(cpu); if cpu.returned_to != 0x1043336 { return; }
  cpu.push(0x104333b); f_010434b0(cpu); if cpu.returned_to != 0x104333b { return; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x1043343); cpu.call(t); if cpu.returned_to != 0x1043343 { return; }
  cpu.push(0x1043348); f_0107162e(cpu); if cpu.returned_to != 0x1043348 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u32 = 0x11a3af0u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x104335d); f_0107259c(cpu); if cpu.returned_to != 0x104335d { return; }
  { let v: u32 = 0x1128600u32; cpu.esi = v; }
  cpu.push(0x1043367); f_01059ad5(cpu); if cpu.returned_to != 0x1043367 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11263e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd431u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c2u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263ccu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c6u32, v); }
  pc = 0x104338a; continue;
 }
 0x104338a => {
  cpu.push(0x104338f); f_010434b0(cpu); if cpu.returned_to != 0x104338f { return; }
  cpu.push(0x1043394); f_01045429(cpu); if cpu.returned_to != 0x1043394 { return; }
  cpu.push(0x1043399); f_01040c94(cpu); if cpu.returned_to != 0x1043399 { return; }
  cpu.push(0x104339e); f_01040a0b(cpu); if cpu.returned_to != 0x104339e { return; }
  cpu.push(0x10433a3); f_01072629(cpu); if cpu.returned_to != 0x10433a3 { return; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x10433ae); cpu.call(t); if cpu.returned_to != 0x10433ae { return; }
  cpu.push(0x10433b3); f_01059ae3(cpu); if cpu.returned_to != 0x10433b3 { return; }
  cpu.push(0x10433b8); f_0105eb1d(cpu); if cpu.returned_to != 0x10433b8 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11263ebu32, v); }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x10433d2; continue; }
  pc = 0x10433c6; continue;
 }
 0x10433c6 => {
  { cpu.sub8((cpu.eax as u8), 0x71u8); }
  if cpu.zf { pc = 0x1043417; continue; }
  pc = 0x10433ca; continue;
 }
 0x10433ca => {
  { cpu.sub8((cpu.eax as u8), 0x51u8); }
  if cpu.zf { pc = 0x1043417; continue; }
  pc = 0x10433ce; continue;
 }
 0x10433ce => {
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if cpu.zf { pc = 0x1043407; continue; }
  pc = 0x10433d2; continue;
 }
 0x10433d2 => {
  cpu.push(0x10433d7); f_010455ac(cpu); if cpu.returned_to != 0x10433d7 { return; }
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x6u8); }
  if cpu.zf { pc = 0x1043407; continue; }
  pc = 0x10433e0; continue;
 }
 0x10433e0 => {
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x6u8); }
  if cpu.zf { pc = 0x104340c; continue; }
  pc = 0x10433e9; continue;
 }
 0x10433e9 => {
  { cpu.sub8(cpu.r8(0x112636au32), 0x6u8); }
  if cpu.zf { pc = 0x10433f4; continue; }
  pc = 0x10433f2; continue;
 }
 0x10433f2 => {
  cpu.poll(); pc = 0x104338a; continue;
 }
 0x10433f4 => {
  cpu.push(0x10433f9); f_01043b27(cpu); if cpu.returned_to != 0x10433f9 { return; }
  { cpu.sub8(cpu.r8(0x10fcdcfu32), 0x0u8); }
  if !cpu.zf { pc = 0x104340c; continue; }
  pc = 0x1043402; continue;
 }
 0x1043402 => {
  cpu.poll(); pc = 0x10432f0; continue;
 }
 0x1043407 => {
  cpu.push(0x104340c); f_01043473(cpu); if cpu.returned_to != 0x104340c { return; }
  pc = 0x104340c; continue;
 }
 0x104340c => {
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x1043416); f_010725e4(cpu); if cpu.returned_to != 0x1043416 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1043417 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fcdcfu32, v); }
  cpu.poll(); pc = 0x104340c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01043420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1043420); }
 let mut pc: u32 = 0x1043420;
 loop { match pc {
 0x1043420 => {
  { let v: u8 = cpu.r8(0x1124bfdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fd389u32, v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11263c7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd52eu32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1043447; continue; }
  pc = 0x104343d; continue;
 }
 0x104343d => {
  cpu.push(0x1043442); f_0104091e(cpu); if cpu.returned_to != 0x1043442 { return; }
  { let v: u8 = cpu.r8(0x11263c7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1043447; continue;
 }
 0x1043447 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fd38bu32, v); }
  { let v: u16 = cpu.r16(0x10e3e81u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fd38au32, v); }
  { let v: u8 = cpu.r8(0x112c97du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fd38cu32, v); }
  { let v: u32 = 0x10ef383u32; cpu.esi = v; }
  { let v: u32 = 0x10fd38du32; cpu.edi = v; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01043473(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1043473); }
 let mut pc: u32 = 0x1043473;
 loop { match pc {
 0x1043473 => {
  { let v: u8 = cpu.r8(0x10fd389u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124bfdu32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10fd38au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e81u32, v); }
  { let v: u8 = cpu.r8(0x10fd38bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11263c7u32, v); }
  { let v: u8 = cpu.r8(0x10fd38cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x112c97du32, v); }
  { let v: u32 = 0x10fd38du32; cpu.esi = v; }
  { let v: u32 = 0x10ef383u32; cpu.edi = v; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010434b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10434b0); }
 let mut pc: u32 = 0x10434b0;
 loop { match pc {
 0x10434b0 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x18u32; cpu.ebx = v; }
  { let v: u32 = 0x19dcu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10434c5); cpu.call(t); if cpu.returned_to != 0x10434c5 { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x97u32; cpu.ebx = v; }
  { let v: u32 = 0x19ddu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10434da); cpu.call(t); if cpu.returned_to != 0x10434da { return; }
  { let v: u32 = 0x100u32; cpu.eax = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = 0x19edu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10434ef); cpu.call(t); if cpu.returned_to != 0x10434ef { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x1a22u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043504); cpu.call(t); if cpu.returned_to != 0x1043504 { return; }
  { let v: u32 = 0x78u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x1a23u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043519); cpu.call(t); if cpu.returned_to != 0x1043519 { return; }
  { let v: u32 = 0x60u32; cpu.eax = v; }
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = 0x1a27u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104352e); cpu.call(t); if cpu.returned_to != 0x104352e { return; }
  { let v: u32 = 0xd8u32; cpu.eax = v; }
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = 0x1a28u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043543); cpu.call(t); if cpu.returned_to != 0x1043543 { return; }
  { let v: u32 = 0xc0u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x1a49u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043558); cpu.call(t); if cpu.returned_to != 0x1043558 { return; }
  { let v: u32 = 0x138u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x1a4au32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104356d); cpu.call(t); if cpu.returned_to != 0x104356d { return; }
  { let v: u32 = 0xc5u32; cpu.eax = v; }
  { let v: u32 = 0x39u32; cpu.ebx = v; }
  { let v: u32 = 0x19dbu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043582); cpu.call(t); if cpu.returned_to != 0x1043582 { return; }
  { cpu.sub8(cpu.r8(0x1126364u32), 0x0u8); }
  if cpu.zf { pc = 0x1043605; continue; }
  pc = 0x104358b; continue;
 }
 0x104358b => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x18u32; cpu.ebx = v; }
  { let v: u32 = 0x1429u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10435a0); cpu.call(t); if cpu.returned_to != 0x10435a0 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1126364u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x19deu32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126364u32), 0x0u8); }
  if cpu.zf { pc = 0x1043605; continue; }
  pc = 0x10435b8; continue;
 }
 0x10435b8 => {
  { let v: u32 = 0x108u32; cpu.eax = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x19deu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10435ce); cpu.call(t); if cpu.returned_to != 0x10435ce { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126364u32), 0x2u8); }
  if cpu.zf { pc = 0x10435e3; continue; }
  pc = 0x10435d8; continue;
 }
 0x10435d8 => {
  { cpu.sub8(cpu.r8(0x1126364u32), 0x3u8); }
  if cpu.zf { pc = 0x10435f5; continue; }
  pc = 0x10435e1; continue;
 }
 0x10435e1 => {
  pc = 0x1043605; continue;
 }
 0x10435e3 => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x38u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10435f3); cpu.call(t); if cpu.returned_to != 0x10435f3 { return; }
  pc = 0x1043605; continue;
 }
 0x10435f5 => {
  { let v: u32 = 0x110u32; cpu.eax = v; }
  { let v: u32 = 0x38u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043605); cpu.call(t); if cpu.returned_to != 0x1043605 { return; }
  pc = 0x1043605; continue;
 }
 0x1043605 => {
  { cpu.sub8(cpu.r8(0x1126365u32), 0x0u8); }
  if cpu.zf { pc = 0x1043688; continue; }
  pc = 0x104360e; continue;
 }
 0x104360e => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = 0x142au32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043623); cpu.call(t); if cpu.returned_to != 0x1043623 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1126365u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x19e1u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126365u32), 0x0u8); }
  if cpu.zf { pc = 0x1043688; continue; }
  pc = 0x104363b; continue;
 }
 0x104363b => {
  { let v: u32 = 0x108u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x19e1u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043651); cpu.call(t); if cpu.returned_to != 0x1043651 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126365u32), 0x2u8); }
  if cpu.zf { pc = 0x1043666; continue; }
  pc = 0x104365b; continue;
 }
 0x104365b => {
  { cpu.sub8(cpu.r8(0x1126365u32), 0x3u8); }
  if cpu.zf { pc = 0x1043678; continue; }
  pc = 0x1043664; continue;
 }
 0x1043664 => {
  pc = 0x1043688; continue;
 }
 0x1043666 => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043676); cpu.call(t); if cpu.returned_to != 0x1043676 { return; }
  pc = 0x1043688; continue;
 }
 0x1043678 => {
  { let v: u32 = 0x110u32; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043688); cpu.call(t); if cpu.returned_to != 0x1043688 { return; }
  pc = 0x1043688; continue;
 }
 0x1043688 => {
  { cpu.sub8(cpu.r8(0x1126366u32), 0x0u8); }
  if cpu.zf { pc = 0x104370b; continue; }
  pc = 0x1043691; continue;
 }
 0x1043691 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = 0x142bu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10436a6); cpu.call(t); if cpu.returned_to != 0x10436a6 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1126366u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x19e4u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126366u32), 0x0u8); }
  if cpu.zf { pc = 0x104370b; continue; }
  pc = 0x10436be; continue;
 }
 0x10436be => {
  { let v: u32 = 0x108u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x19e4u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10436d4); cpu.call(t); if cpu.returned_to != 0x10436d4 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126366u32), 0x2u8); }
  if cpu.zf { pc = 0x10436e9; continue; }
  pc = 0x10436de; continue;
 }
 0x10436de => {
  { cpu.sub8(cpu.r8(0x1126366u32), 0x3u8); }
  if cpu.zf { pc = 0x10436fb; continue; }
  pc = 0x10436e7; continue;
 }
 0x10436e7 => {
  pc = 0x104370b; continue;
 }
 0x10436e9 => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x58u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10436f9); cpu.call(t); if cpu.returned_to != 0x10436f9 { return; }
  pc = 0x104370b; continue;
 }
 0x10436fb => {
  { let v: u32 = 0x110u32; cpu.eax = v; }
  { let v: u32 = 0x58u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104370b); cpu.call(t); if cpu.returned_to != 0x104370b { return; }
  pc = 0x104370b; continue;
 }
 0x104370b => {
  { cpu.sub8(cpu.r8(0x1126367u32), 0x0u8); }
  if cpu.zf { pc = 0x104378e; continue; }
  pc = 0x1043714; continue;
 }
 0x1043714 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0x142cu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043729); cpu.call(t); if cpu.returned_to != 0x1043729 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1126367u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x19e7u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126367u32), 0x0u8); }
  if cpu.zf { pc = 0x104378e; continue; }
  pc = 0x1043741; continue;
 }
 0x1043741 => {
  { let v: u32 = 0x108u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x19e7u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043757); cpu.call(t); if cpu.returned_to != 0x1043757 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126367u32), 0x2u8); }
  if cpu.zf { pc = 0x104376c; continue; }
  pc = 0x1043761; continue;
 }
 0x1043761 => {
  { cpu.sub8(cpu.r8(0x1126367u32), 0x3u8); }
  if cpu.zf { pc = 0x104377e; continue; }
  pc = 0x104376a; continue;
 }
 0x104376a => {
  pc = 0x104378e; continue;
 }
 0x104376c => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x68u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104377c); cpu.call(t); if cpu.returned_to != 0x104377c { return; }
  pc = 0x104378e; continue;
 }
 0x104377e => {
  { let v: u32 = 0x110u32; cpu.eax = v; }
  { let v: u32 = 0x68u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104378e); cpu.call(t); if cpu.returned_to != 0x104378e { return; }
  pc = 0x104378e; continue;
 }
 0x104378e => {
  { cpu.sub8(cpu.r8(0x1126368u32), 0x0u8); }
  if cpu.zf { pc = 0x1043811; continue; }
  pc = 0x1043797; continue;
 }
 0x1043797 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x58u32; cpu.ebx = v; }
  { let v: u32 = 0x19eeu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10437ac); cpu.call(t); if cpu.returned_to != 0x10437ac { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1126368u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x19eau32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126368u32), 0x0u8); }
  if cpu.zf { pc = 0x1043811; continue; }
  pc = 0x10437c4; continue;
 }
 0x10437c4 => {
  { let v: u32 = 0x108u32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x19eau32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10437da); cpu.call(t); if cpu.returned_to != 0x10437da { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x1126368u32), 0x2u8); }
  if cpu.zf { pc = 0x10437ef; continue; }
  pc = 0x10437e4; continue;
 }
 0x10437e4 => {
  { cpu.sub8(cpu.r8(0x1126368u32), 0x3u8); }
  if cpu.zf { pc = 0x1043801; continue; }
  pc = 0x10437ed; continue;
 }
 0x10437ed => {
  pc = 0x1043811; continue;
 }
 0x10437ef => {
  { let v: u32 = 0x118u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10437ff); cpu.call(t); if cpu.returned_to != 0x10437ff { return; }
  pc = 0x1043811; continue;
 }
 0x1043801 => {
  { let v: u32 = 0x110u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043811); cpu.call(t); if cpu.returned_to != 0x1043811 { return; }
  pc = 0x1043811; continue;
 }
 0x1043811 => {
  { cpu.sub8(cpu.r8(0x1126369u32), 0x0u8); }
  if cpu.zf { pc = 0x104382f; continue; }
  pc = 0x104381a; continue;
 }
 0x104381a => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = 0x19efu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104382f); cpu.call(t); if cpu.returned_to != 0x104382f { return; }
  pc = 0x104382f; continue;
 }
 0x104382f => {
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x0u8); }
  if cpu.zf { pc = 0x1043882; continue; }
  pc = 0x1043838; continue;
 }
 0x1043838 => {
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x1u8); }
  if cpu.zf { pc = 0x104386d; continue; }
  pc = 0x1043841; continue;
 }
 0x1043841 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x1a29u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043856); cpu.call(t); if cpu.returned_to != 0x1043856 { return; }
  { let v: u32 = 0x78u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x1a2au32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104386b); cpu.call(t); if cpu.returned_to != 0x104386b { return; }
  pc = 0x1043882; continue;
 }
 0x104386d => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x1a2bu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043882); cpu.call(t); if cpu.returned_to != 0x1043882 { return; }
  pc = 0x1043882; continue;
 }
 0x1043882 => {
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x0u8); }
  if cpu.zf { pc = 0x10438ef; continue; }
  pc = 0x104388b; continue;
 }
 0x104388b => {
  { let v: u32 = 0x1a12u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x1u8); }
  if cpu.zf { pc = 0x10438c5; continue; }
  pc = 0x1043899; continue;
 }
 0x1043899 => {
  { let v: u32 = 0xc0u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x1a4du32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10438ae); cpu.call(t); if cpu.returned_to != 0x10438ae { return; }
  { let v: u32 = 0x138u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x1a4eu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10438c3); cpu.call(t); if cpu.returned_to != 0x10438c3 { return; }
  pc = 0x10438ef; continue;
 }
 0x10438c5 => {
  { let v: u32 = 0xc0u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x1a4bu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10438da); cpu.call(t); if cpu.returned_to != 0x10438da { return; }
  { let v: u32 = 0x138u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x1a4cu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10438ef); cpu.call(t); if cpu.returned_to != 0x10438ef { return; }
  pc = 0x10438ef; continue;
 }
 0x10438ef => {
  { cpu.sub8(cpu.r8(0x112636au32), 0x0u8); }
  if cpu.zf { pc = 0x1043942; continue; }
  pc = 0x10438f8; continue;
 }
 0x10438f8 => {
  { cpu.sub8(cpu.r8(0x112636au32), 0x1u8); }
  if cpu.zf { pc = 0x104392d; continue; }
  pc = 0x1043901; continue;
 }
 0x1043901 => {
  { let v: u32 = 0x60u32; cpu.eax = v; }
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = 0x1a25u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043916); cpu.call(t); if cpu.returned_to != 0x1043916 { return; }
  { let v: u32 = 0xd8u32; cpu.eax = v; }
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = 0x1a26u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104392b); cpu.call(t); if cpu.returned_to != 0x104392b { return; }
  pc = 0x1043942; continue;
 }
 0x104392d => {
  { let v: u32 = 0x60u32; cpu.eax = v; }
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = 0x1a24u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043942); cpu.call(t); if cpu.returned_to != 0x1043942 { return; }
  pc = 0x1043942; continue;
 }
 0x1043942 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1124bfdu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x112ca44u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x1u16); }
  if !cpu.zf { pc = 0x1043960; continue; }
  pc = 0x1043959; continue;
 }
 0x1043959 => {
  { let v: u32 = 0x10f7a1fu32; cpu.esi = v; }
  pc = 0x104398a; continue;
 }
 0x1043960 => {
  { let v: u16 = 0xc200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  cpu.push(0x1043978); f_01059cfe(cpu); if cpu.returned_to != 0x1043978 { return; }
  { let v: u32 = 0x10ef397u32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x1043982; continue;
 }
 0x1043982 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x30u8); }
  if !cpu.zf { pc = 0x104398a; continue; }
  pc = 0x1043987; continue;
 }
 0x1043987 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1043982; continue; }
  pc = 0x104398a; continue;
 }
 0x104398a => {
  { let v: u32 = 0xcau32; cpu.eax = v; }
  { let v: u32 = 0x3du32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x1126364u32), 0x0u8); }
  if cpu.zf { pc = 0x10439a4; continue; }
  pc = 0x104399d; continue;
 }
 0x104399d => {
  cpu.push(0x10439a2); f_0104681f(cpu); if cpu.returned_to != 0x10439a2 { return; }
  pc = 0x10439a9; continue;
 }
 0x10439a4 => {
  cpu.push(0x10439a9); f_0104682e(cpu); if cpu.returned_to != 0x10439a9 { return; }
  pc = 0x10439a9; continue;
 }
 0x10439a9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11263c7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if (cpu.sf != cpu.of) { pc = 0x10439d3; continue; }
  pc = 0x10439b6; continue;
 }
 0x10439b6 => {
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1194208u32, v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1194209u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x119420au32, v); }
  pc = 0x10439e1; continue;
 }
 0x10439d3 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1194208u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1194209u32, v); }
  pc = 0x10439e1; continue;
 }
 0x10439e1 => {
  { let v: u32 = 0xcau32; cpu.eax = v; }
  { let v: u32 = 0x5cu32; cpu.ebx = v; }
  { let v: u32 = 0x1194208u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x1126366u32), 0x0u8); }
  if cpu.zf { pc = 0x1043a00; continue; }
  pc = 0x10439f9; continue;
 }
 0x10439f9 => {
  cpu.push(0x10439fe); f_0104681f(cpu); if cpu.returned_to != 0x10439fe { return; }
  pc = 0x1043a05; continue;
 }
 0x1043a00 => {
  cpu.push(0x1043a05); f_0104682e(cpu); if cpu.returned_to != 0x1043a05 { return; }
  pc = 0x1043a05; continue;
 }
 0x1043a05 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10e3e81u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x31u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1194208u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1194209u32, v); }
  { let v: u32 = 0xcau32; cpu.eax = v; }
  { let v: u32 = 0x4du32; cpu.ebx = v; }
  { let v: u32 = 0x1194208u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x1126365u32), 0x0u8); }
  if cpu.zf { pc = 0x1043a3f; continue; }
  pc = 0x1043a38; continue;
 }
 0x1043a38 => {
  cpu.push(0x1043a3d); f_0104681f(cpu); if cpu.returned_to != 0x1043a3d { return; }
  pc = 0x1043a44; continue;
 }
 0x1043a3f => {
  cpu.push(0x1043a44); f_0104682e(cpu); if cpu.returned_to != 0x1043a44 { return; }
  pc = 0x1043a44; continue;
 }
 0x1043a44 => {
  { let v: u16 = cpu.r16(0x1124e8au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1043a54); f_01045c71(cpu); if cpu.returned_to != 0x1043a54 { return; }
  { let v: u16 = 0xcau16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x6cu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10ef397u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x1126367u32), 0x0u8); }
  if cpu.zf { pc = 0x1043a71; continue; }
  pc = 0x1043a6a; continue;
 }
 0x1043a6a => {
  cpu.push(0x1043a6f); f_0104681f(cpu); if cpu.returned_to != 0x1043a6f { return; }
  pc = 0x1043a76; continue;
 }
 0x1043a71 => {
  cpu.push(0x1043a76); f_0104682e(cpu); if cpu.returned_to != 0x1043a76 { return; }
  pc = 0x1043a76; continue;
 }
 0x1043a76 => {
  { let v: u32 = 0xcau32; cpu.eax = v; }
  { let v: u32 = 0x7bu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1130fe4u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x112c97du32), 0x0u8); }
  if cpu.zf { pc = 0x1043a95; continue; }
  pc = 0x1043a8f; continue;
 }
 0x1043a8f => {
  { let v: u32 = cpu.r32(0x1130fe8u32); cpu.esi = v; }
  pc = 0x1043a95; continue;
 }
 0x1043a95 => {
  { cpu.sub8(cpu.r8(0x1126368u32), 0x0u8); }
  if cpu.zf { pc = 0x1043aa5; continue; }
  pc = 0x1043a9e; continue;
 }
 0x1043a9e => {
  cpu.push(0x1043aa3); f_0104681f(cpu); if cpu.returned_to != 0x1043aa3 { return; }
  pc = 0x1043aaa; continue;
 }
 0x1043aa5 => {
  cpu.push(0x1043aaa); f_0104682e(cpu); if cpu.returned_to != 0x1043aaa { return; }
  pc = 0x1043aaa; continue;
 }
 0x1043aaa => {
  { let v: u32 = 0x10ef383u32; cpu.esi = v; }
  cpu.push(0x1043ab4); f_0104683d(cpu); if cpu.returned_to != 0x1043ab4 { return; }
  { cpu.sub32(cpu.eax, 0x46u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1043acf; continue; }
  pc = 0x1043ab9; continue;
 }
 0x1043ab9 => {
  { let v: u32 = 0x46u32; cpu.ecx = v; }
  cpu.push(0x1043ac3); f_0104698f(cpu); if cpu.returned_to != 0x1043ac3 { return; }
  { let v: u32 = 0xcau32; cpu.eax = v; }
  { let v: u32 = 0x8bu32; cpu.ebx = v; }
  pc = 0x1043ade; continue;
 }
 0x1043acf => {
  { let v: u32 = 0xcau32; cpu.eax = v; }
  { let v: u32 = 0x8bu32; cpu.ebx = v; }
  { let v: u32 = 0x10ef383u32; cpu.esi = v; }
  pc = 0x1043ade; continue;
 }
 0x1043ade => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x1u8); }
  if cpu.zf { pc = 0x1043af7; continue; }
  pc = 0x1043ae7; continue;
 }
 0x1043ae7 => {
  { cpu.sub8(cpu.r8(0x1126369u32), 0x0u8); }
  if !cpu.zf { pc = 0x1043af7; continue; }
  pc = 0x1043af0; continue;
 }
 0x1043af0 => {
  cpu.push(0x1043af5); f_0104682e(cpu); if cpu.returned_to != 0x1043af5 { return; }
  pc = 0x1043b26; continue;
 }
 0x1043af7 => {
  cpu.push(0x1043afc); f_0104681f(cpu); if cpu.returned_to != 0x1043afc { return; }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x1u8); }
  if !cpu.zf { pc = 0x1043b26; continue; }
  pc = 0x1043b05; continue;
 }
 0x1043b05 => {
  { cpu.sub8(cpu.r8(0x11263ecu32), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x1043b26; continue; }
  pc = 0x1043b0e; continue;
 }
 0x1043b0e => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10fcdb6u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10fcdbau32); cpu.ebx = v; }
  { let v: u32 = 0x39u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043b25); cpu.call(t); if cpu.returned_to != 0x1043b25 { return; }
  { cpu.popad(); }
  pc = 0x1043b26; continue;
 }
 0x1043b26 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01043b27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1043b27); }
 let mut pc: u32 = 0x1043b27;
 loop { match pc {
 0x1043b27 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1043b6a; continue; }
  pc = 0x1043b33; continue;
 }
 0x1043b33 => {
  cpu.push(0x1043b38); f_010129b5(cpu); if cpu.returned_to != 0x1043b38 { return; }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  cpu.push(0x1043b42); f_01012780(cpu); if cpu.returned_to != 0x1043b42 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1043b6a; continue; }
  pc = 0x1043b47; continue;
 }
 0x1043b47 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1043b51); f_01012bd0(cpu); if cpu.returned_to != 0x1043b51 { return; }
  { let v: u32 = 0x1cu32; cpu.eax = v; }
  cpu.push(0x1043b5b); f_01012780(cpu); if cpu.returned_to != 0x1043b5b { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1043b6a; continue; }
  pc = 0x1043b60; continue;
 }
 0x1043b60 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1043b6a); f_01012bd0(cpu); if cpu.returned_to != 0x1043b6a { return; }
  pc = 0x1043b6a; continue;
 }
 0x1043b6a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x1043b76); f_010725e4(cpu); if cpu.returned_to != 0x1043b76 { return; }
  cpu.push(0x1043b7b); f_01043c9a(cpu); if cpu.returned_to != 0x1043b7b { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1043b8d); f_010385ad(cpu); if cpu.returned_to != 0x1043b8d { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x1043b95); cpu.call(t); if cpu.returned_to != 0x1043b95 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  { let v: u32 = 0x10fcd82u32; cpu.edx = v; }
  cpu.push(0x1043ba6); f_01045c08(cpu); if cpu.returned_to != 0x1043ba6 { return; }
  { let v: u8 = 0x2u8; cpu.w8(0x1128991u32, v); }
  cpu.push(0x1043bb2); f_01045705(cpu); if cpu.returned_to != 0x1043bb2 { return; }
  cpu.push(0x1043bb7); f_01043d02(cpu); if cpu.returned_to != 0x1043bb7 { return; }
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x1043bbf); cpu.call(t); if cpu.returned_to != 0x1043bbf { return; }
  cpu.push(0x1043bc4); f_0107162e(cpu); if cpu.returned_to != 0x1043bc4 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  { let v: u32 = 0x11a3af0u32; cpu.esi = v; }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1043bd9); f_0107259c(cpu); if cpu.returned_to != 0x1043bd9 { return; }
  { let v: u32 = 0x1128792u32; cpu.esi = v; }
  cpu.push(0x1043be3); f_01059ad5(cpu); if cpu.returned_to != 0x1043be3 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11263e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd431u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112636au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c2u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c6u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263e5u32, v); }
  pc = 0x1043c0d; continue;
 }
 0x1043c0d => {
  cpu.push(0x1043c12); f_01043d02(cpu); if cpu.returned_to != 0x1043c12 { return; }
  cpu.push(0x1043c17); f_01045705(cpu); if cpu.returned_to != 0x1043c17 { return; }
  cpu.push(0x1043c1c); f_01040c94(cpu); if cpu.returned_to != 0x1043c1c { return; }
  cpu.push(0x1043c21); f_01040a0b(cpu); if cpu.returned_to != 0x1043c21 { return; }
  cpu.push(0x1043c26); f_01072629(cpu); if cpu.returned_to != 0x1043c26 { return; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x1043c31); cpu.call(t); if cpu.returned_to != 0x1043c31 { return; }
  cpu.push(0x1043c36); f_01059ae3(cpu); if cpu.returned_to != 0x1043c36 { return; }
  { let v: u8 = 0xffu8; cpu.w8(0x11263ebu32, v); }
  cpu.push(0x1043c42); f_0105eb1d(cpu); if cpu.returned_to != 0x1043c42 { return; }
  if !cpu.cf { pc = 0x1043c6d; continue; }
  pc = 0x1043c44; continue;
 }
 0x1043c44 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11c633du32, v); }
  cpu.push(0x1043c4e); f_0105eb66(cpu); if cpu.returned_to != 0x1043c4e { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11263ebu32, v); }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1043c68; continue; }
  pc = 0x1043c5c; continue;
 }
 0x1043c5c => {
  { cpu.sub8((cpu.eax as u8), 0x71u8); }
  if cpu.zf { pc = 0x1043c91; continue; }
  pc = 0x1043c60; continue;
 }
 0x1043c60 => {
  { cpu.sub8((cpu.eax as u8), 0x51u8); }
  if cpu.zf { pc = 0x1043c91; continue; }
  pc = 0x1043c64; continue;
 }
 0x1043c64 => {
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if cpu.zf { pc = 0x1043c81; continue; }
  pc = 0x1043c68; continue;
 }
 0x1043c68 => {
  cpu.push(0x1043c6d); f_01045a21(cpu); if cpu.returned_to != 0x1043c6d { return; }
  pc = 0x1043c6d; continue;
 }
 0x1043c6d => {
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x6u8); }
  if cpu.zf { pc = 0x1043c81; continue; }
  pc = 0x1043c76; continue;
 }
 0x1043c76 => {
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x6u8); }
  if cpu.zf { pc = 0x1043c86; continue; }
  pc = 0x1043c7f; continue;
 }
 0x1043c7f => {
  cpu.poll(); pc = 0x1043c0d; continue;
 }
 0x1043c81 => {
  cpu.push(0x1043c86); f_01043cce(cpu); if cpu.returned_to != 0x1043c86 { return; }
  pc = 0x1043c86; continue;
 }
 0x1043c86 => {
  { let v: u32 = 0x300u32; cpu.eax = v; }
  cpu.push(0x1043c90); f_010725e4(cpu); if cpu.returned_to != 0x1043c90 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1043c91 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fcdcfu32, v); }
  cpu.poll(); pc = 0x1043c86; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01043c9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1043c9a); }
 let mut pc: u32 = 0x1043c9a;
 loop { match pc {
 0x1043c9a => {
  { let v: u32 = 0x10f7acdu32; cpu.esi = v; }
  { let v: u32 = 0x10fd3a3u32; cpu.edi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10f7fc8u32; cpu.esi = v; }
  { let v: u32 = 0x10fd3c6u32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x1124c5fu32; cpu.esi = v; }
  { let v: u32 = 0x10fd3d7u32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01043cce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1043cce); }
 let mut pc: u32 = 0x1043cce;
 loop { match pc {
 0x1043cce => {
  { let v: u32 = 0x10fd3a3u32; cpu.esi = v; }
  { let v: u32 = 0x10f7acdu32; cpu.edi = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10fd3c6u32; cpu.esi = v; }
  { let v: u32 = 0x10f7fc8u32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x10fd3d7u32; cpu.esi = v; }
  { let v: u32 = 0x1124c5fu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01043d02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1043d02); }
 let mut pc: u32 = 0x1043d02;
 loop { match pc {
 0x1043d02 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = 0x1a19u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043d17); cpu.call(t); if cpu.returned_to != 0x1043d17 { return; }
  { let v: u32 = 0xb0u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x16a0u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043d2c); cpu.call(t); if cpu.returned_to != 0x1043d2c { return; }
  { let v: u32 = 0x128u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x16a1u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043d41); cpu.call(t); if cpu.returned_to != 0x1043d41 { return; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x1a1du32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043d56); cpu.call(t); if cpu.returned_to != 0x1043d56 { return; }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x1a1eu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043d6b); cpu.call(t); if cpu.returned_to != 0x1043d6b { return; }
  { let v: u32 = 0xc3u32; cpu.eax = v; }
  { let v: u32 = 0x3bu32; cpu.ebx = v; }
  { let v: u32 = 0x1a01u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043d80); cpu.call(t); if cpu.returned_to != 0x1043d80 { return; }
  { cpu.sub8(cpu.r8(0x112636bu32), 0x0u8); }
  if cpu.zf { pc = 0x1043d9e; continue; }
  pc = 0x1043d89; continue;
 }
 0x1043d89 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = 0x1a15u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043d9e); cpu.call(t); if cpu.returned_to != 0x1043d9e { return; }
  pc = 0x1043d9e; continue;
 }
 0x1043d9e => {
  { cpu.sub8(cpu.r8(0x112636cu32), 0x0u8); }
  if cpu.zf { pc = 0x1043dbc; continue; }
  pc = 0x1043da7; continue;
 }
 0x1043da7 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x28u32; cpu.ebx = v; }
  { let v: u32 = 0x1a16u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043dbc); cpu.call(t); if cpu.returned_to != 0x1043dbc { return; }
  pc = 0x1043dbc; continue;
 }
 0x1043dbc => {
  { cpu.sub8(cpu.r8(0x112636du32), 0x0u8); }
  if cpu.zf { pc = 0x1043dda; continue; }
  pc = 0x1043dc5; continue;
 }
 0x1043dc5 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x38u32; cpu.ebx = v; }
  { let v: u32 = 0x1a17u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043dda); cpu.call(t); if cpu.returned_to != 0x1043dda { return; }
  pc = 0x1043dda; continue;
 }
 0x1043dda => {
  { cpu.sub8(cpu.r8(0x112636eu32), 0x0u8); }
  if cpu.zf { pc = 0x1043df8; continue; }
  pc = 0x1043de3; continue;
 }
 0x1043de3 => {
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = 0x48u32; cpu.ebx = v; }
  { let v: u32 = 0x1a18u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043df8); cpu.call(t); if cpu.returned_to != 0x1043df8 { return; }
  pc = 0x1043df8; continue;
 }
 0x1043df8 => {
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x0u8); }
  if cpu.zf { pc = 0x1043e4b; continue; }
  pc = 0x1043e01; continue;
 }
 0x1043e01 => {
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x1u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1043e21; continue; }
  pc = 0x1043e0a; continue;
 }
 0x1043e0a => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x1a1au32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043e1f); cpu.call(t); if cpu.returned_to != 0x1043e1f { return; }
  pc = 0x1043e4b; continue;
 }
 0x1043e21 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x1a1bu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043e36); cpu.call(t); if cpu.returned_to != 0x1043e36 { return; }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x1a1cu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043e4b); cpu.call(t); if cpu.returned_to != 0x1043e4b { return; }
  pc = 0x1043e4b; continue;
 }
 0x1043e4b => {
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x0u8); }
  if cpu.zf { pc = 0x1043e9e; continue; }
  pc = 0x1043e54; continue;
 }
 0x1043e54 => {
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x1u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1043e74; continue; }
  pc = 0x1043e5d; continue;
 }
 0x1043e5d => {
  { let v: u32 = 0xa8u32; cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.ebx = v; }
  { let v: u32 = 0x1a1fu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043e72); cpu.call(t); if cpu.returned_to != 0x1043e72 { return; }
  pc = 0x1043e9e; continue;
 }
 0x1043e74 => {
  { let v: u32 = 0xb0u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x1a20u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043e89); cpu.call(t); if cpu.returned_to != 0x1043e89 { return; }
  { let v: u32 = 0x128u32; cpu.eax = v; }
  { let v: u32 = 0x78u32; cpu.ebx = v; }
  { let v: u32 = 0x1a21u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043e9e); cpu.call(t); if cpu.returned_to != 0x1043e9e { return; }
  pc = 0x1043e9e; continue;
 }
 0x1043e9e => {
  { let v: u32 = 0x10f7acdu32; cpu.esi = v; }
  cpu.push(0x1043ea8); f_0104683d(cpu); if cpu.returned_to != 0x1043ea8 { return; }
  { cpu.sub32(cpu.eax, 0x4au32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1043ec3; continue; }
  pc = 0x1043ead; continue;
 }
 0x1043ead => {
  { let v: u32 = 0x4au32; cpu.ecx = v; }
  cpu.push(0x1043eb7); f_0104698f(cpu); if cpu.returned_to != 0x1043eb7 { return; }
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  pc = 0x1043ed2; continue;
 }
 0x1043ec3 => {
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0x10f7acdu32; cpu.esi = v; }
  pc = 0x1043ed2; continue;
 }
 0x1043ed2 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x1u8); }
  if cpu.zf { pc = 0x1043eeb; continue; }
  pc = 0x1043edb; continue;
 }
 0x1043edb => {
  { cpu.sub8(cpu.r8(0x112636bu32), 0x0u8); }
  if !cpu.zf { pc = 0x1043eeb; continue; }
  pc = 0x1043ee4; continue;
 }
 0x1043ee4 => {
  cpu.push(0x1043ee9); f_0104682e(cpu); if cpu.returned_to != 0x1043ee9 { return; }
  pc = 0x1043f1a; continue;
 }
 0x1043eeb => {
  cpu.push(0x1043ef0); f_0104681f(cpu); if cpu.returned_to != 0x1043ef0 { return; }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x1u8); }
  if !cpu.zf { pc = 0x1043f1a; continue; }
  pc = 0x1043ef9; continue;
 }
 0x1043ef9 => {
  { cpu.sub8(cpu.r8(0x11263ecu32), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x1043f1a; continue; }
  pc = 0x1043f02; continue;
 }
 0x1043f02 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10fcdb6u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10fcdbau32); cpu.ebx = v; }
  { let v: u32 = 0x39u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043f19); cpu.call(t); if cpu.returned_to != 0x1043f19 { return; }
  { cpu.popad(); }
  pc = 0x1043f1a; continue;
 }
 0x1043f1a => {
  { let v: u32 = 0x10f7fc8u32; cpu.esi = v; }
  cpu.push(0x1043f24); f_0104683d(cpu); if cpu.returned_to != 0x1043f24 { return; }
  { cpu.sub32(cpu.eax, 0x4au32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1043f3f; continue; }
  pc = 0x1043f29; continue;
 }
 0x1043f29 => {
  { let v: u32 = 0x4au32; cpu.ecx = v; }
  cpu.push(0x1043f33); f_0104698f(cpu); if cpu.returned_to != 0x1043f33 { return; }
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  pc = 0x1043f4e; continue;
 }
 0x1043f3f => {
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = 0x10f7fc8u32; cpu.esi = v; }
  pc = 0x1043f4e; continue;
 }
 0x1043f4e => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x2u8); }
  if cpu.zf { pc = 0x1043f67; continue; }
  pc = 0x1043f57; continue;
 }
 0x1043f57 => {
  { cpu.sub8(cpu.r8(0x112636cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1043f67; continue; }
  pc = 0x1043f60; continue;
 }
 0x1043f60 => {
  cpu.push(0x1043f65); f_0104682e(cpu); if cpu.returned_to != 0x1043f65 { return; }
  pc = 0x1043f96; continue;
 }
 0x1043f67 => {
  cpu.push(0x1043f6c); f_0104681f(cpu); if cpu.returned_to != 0x1043f6c { return; }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x2u8); }
  if !cpu.zf { pc = 0x1043f96; continue; }
  pc = 0x1043f75; continue;
 }
 0x1043f75 => {
  { cpu.sub8(cpu.r8(0x11263ecu32), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x1043f96; continue; }
  pc = 0x1043f7e; continue;
 }
 0x1043f7e => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10fcdb6u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10fcdbau32); cpu.ebx = v; }
  { let v: u32 = 0x39u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1043f95); cpu.call(t); if cpu.returned_to != 0x1043f95 { return; }
  { cpu.popad(); }
  pc = 0x1043f96; continue;
 }
 0x1043f96 => {
  { let v: u32 = 0x1124c5fu32; cpu.esi = v; }
  cpu.push(0x1043fa0); f_0104683d(cpu); if cpu.returned_to != 0x1043fa0 { return; }
  { cpu.sub32(cpu.eax, 0x4au32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1043fbb; continue; }
  pc = 0x1043fa5; continue;
 }
 0x1043fa5 => {
  { let v: u32 = 0x4au32; cpu.ecx = v; }
  cpu.push(0x1043faf); f_0104698f(cpu); if cpu.returned_to != 0x1043faf { return; }
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  pc = 0x1043fca; continue;
 }
 0x1043fbb => {
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x60u32; cpu.ebx = v; }
  { let v: u32 = 0x1124c5fu32; cpu.esi = v; }
  pc = 0x1043fca; continue;
 }
 0x1043fca => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x3u8); }
  if cpu.zf { pc = 0x1043fe3; continue; }
  pc = 0x1043fd3; continue;
 }
 0x1043fd3 => {
  { cpu.sub8(cpu.r8(0x112636du32), 0x0u8); }
  if !cpu.zf { pc = 0x1043fe3; continue; }
  pc = 0x1043fdc; continue;
 }
 0x1043fdc => {
  cpu.push(0x1043fe1); f_0104682e(cpu); if cpu.returned_to != 0x1043fe1 { return; }
  pc = 0x1044012; continue;
 }
 0x1043fe3 => {
  cpu.push(0x1043fe8); f_0104681f(cpu); if cpu.returned_to != 0x1043fe8 { return; }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x3u8); }
  if !cpu.zf { pc = 0x1044012; continue; }
  pc = 0x1043ff1; continue;
 }
 0x1043ff1 => {
  { cpu.sub8(cpu.r8(0x11263ecu32), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x1044012; continue; }
  pc = 0x1043ffa; continue;
 }
 0x1043ffa => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10fcdb6u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10fcdbau32); cpu.ebx = v; }
  { let v: u32 = 0x39u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1044011); cpu.call(t); if cpu.returned_to != 0x1044011 { return; }
  { cpu.popad(); }
  pc = 0x1044012; continue;
 }
 0x1044012 => {
  { let v: u32 = 0x112c98au32; cpu.esi = v; }
  cpu.push(0x104401c); f_0104683d(cpu); if cpu.returned_to != 0x104401c { return; }
  { cpu.sub32(cpu.eax, 0x4au32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1044037; continue; }
  pc = 0x1044021; continue;
 }
 0x1044021 => {
  { let v: u32 = 0x4au32; cpu.ecx = v; }
  cpu.push(0x104402b); f_0104698f(cpu); if cpu.returned_to != 0x104402b { return; }
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  pc = 0x1044046; continue;
 }
 0x1044037 => {
  { let v: u32 = 0xccu32; cpu.eax = v; }
  { let v: u32 = 0x70u32; cpu.ebx = v; }
  { let v: u32 = 0x112c98au32; cpu.esi = v; }
  pc = 0x1044046; continue;
 }
 0x1044046 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x4u8); }
  if cpu.zf { pc = 0x104405f; continue; }
  pc = 0x104404f; continue;
 }
 0x104404f => {
  { cpu.sub8(cpu.r8(0x112636eu32), 0x0u8); }
  if !cpu.zf { pc = 0x104405f; continue; }
  pc = 0x1044058; continue;
 }
 0x1044058 => {
  cpu.push(0x104405d); f_0104682e(cpu); if cpu.returned_to != 0x104405d { return; }
  pc = 0x104408e; continue;
 }
 0x104405f => {
  cpu.push(0x1044064); f_0104681f(cpu); if cpu.returned_to != 0x1044064 { return; }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x4u8); }
  if !cpu.zf { pc = 0x104408e; continue; }
  pc = 0x104406d; continue;
 }
 0x104406d => {
  { cpu.sub8(cpu.r8(0x11263ecu32), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x104408e; continue; }
  pc = 0x1044076; continue;
 }
 0x1044076 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10fcdb6u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10fcdbau32); cpu.ebx = v; }
  { let v: u32 = 0x39u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104408d); cpu.call(t); if cpu.returned_to != 0x104408d { return; }
  { cpu.popad(); }
  pc = 0x104408e; continue;
 }
 0x104408e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104408f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x104408f); } r_0104408f(cpu, 0x104408f); }
pub fn f_010440c4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10440c4); } r_0104408f(cpu, 0x10440c4); }
fn r_0104408f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x104408f => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = 0x18u32; cpu.ebx = v; }
  { let v: u32 = 0x469u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10440a4); cpu.call(t); if cpu.returned_to != 0x10440a4 { return; }
  { let v: u32 = 0x18u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x46eu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10440b9); cpu.call(t); if cpu.returned_to != 0x10440b9 { return; }
  { let v: u32 = 0x90u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x46fu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10440ce); cpu.call(t); if cpu.returned_to != 0x10440ce { return; }
  { cpu.sub8(cpu.r8(0x1126395u32), 0x0u8); }
  if cpu.zf { pc = 0x10440ec; continue; }
  pc = 0x10440d7; continue;
 }
 0x10440c4 => {
  cpu.unimplemented(0x10440c4, "Outsd");
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x10e42ecu32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1126395u32), 0x0u8); }
  if cpu.zf { pc = 0x10440ec; continue; }
  pc = 0x10440d7; continue;
 }
 0x10440d7 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = 0x18u32; cpu.ebx = v; }
  { let v: u32 = 0x45eu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10440ec); cpu.call(t); if cpu.returned_to != 0x10440ec { return; }
  pc = 0x10440ec; continue;
 }
 0x10440ec => {
  { cpu.sub8(cpu.r8(0x1126396u32), 0x0u8); }
  if cpu.zf { pc = 0x104410a; continue; }
  pc = 0x10440f5; continue;
 }
 0x10440f5 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = 0x45fu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104410a); cpu.call(t); if cpu.returned_to != 0x104410a { return; }
  pc = 0x104410a; continue;
 }
 0x104410a => {
  { cpu.sub8(cpu.r8(0x112639cu32), 0x0u8); }
  if cpu.zf { pc = 0x1044128; continue; }
  pc = 0x1044113; continue;
 }
 0x1044113 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = 0x30u32; cpu.ebx = v; }
  { let v: u32 = 0x460u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1044128); cpu.call(t); if cpu.returned_to != 0x1044128 { return; }
  pc = 0x1044128; continue;
 }
 0x1044128 => {
  { cpu.sub8(cpu.r8(0x1126397u32), 0x0u8); }
  if cpu.zf { pc = 0x1044146; continue; }
  pc = 0x1044131; continue;
 }
 0x1044131 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = 0x40u32; cpu.ebx = v; }
  { let v: u32 = 0x461u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1044146); cpu.call(t); if cpu.returned_to != 0x1044146 { return; }
  pc = 0x1044146; continue;
 }
 0x1044146 => {
  { cpu.sub8(cpu.r8(0x1126398u32), 0x0u8); }
  if cpu.zf { pc = 0x1044164; continue; }
  pc = 0x104414f; continue;
 }
 0x104414f => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { let v: u32 = 0x50u32; cpu.ebx = v; }
  { let v: u32 = 0x46du32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1044164); cpu.call(t); if cpu.returned_to != 0x1044164 { return; }
  pc = 0x1044164; continue;
 }
 0x1044164 => {
  { cpu.sub8(cpu.r8(0x112639bu32), 0x1u8); }
  if cpu.zf { pc = 0x1044197; continue; }
  pc = 0x104416d; continue;
 }
 0x104416d => {
  if (cpu.sf == cpu.of) { pc = 0x1044171; continue; }
  pc = 0x104416f; continue;
 }
 0x104416f => {
  pc = 0x10441c1; continue;
 }
 0x1044171 => {
  { let v: u32 = 0x18u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x470u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1044186); cpu.call(t); if cpu.returned_to != 0x1044186 { return; }
  { let v: u32 = 0x90u32; cpu.eax = v; }
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  { let v: u32 = 0x471u32; cpu.ecx = v; }
  pc = 0x10441bb; continue;
 }
 0x1044197 => {
  { let v: u32 = 0x18u32; cpu.eax = v; }
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = 0xb35u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10441ac); cpu.call(t); if cpu.returned_to != 0x10441ac { return; }
  { let v: u32 = 0x90u32; cpu.eax = v; }
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = 0xb36u32; cpu.ecx = v; }
  pc = 0x10441bb; continue;
 }
 0x10441bb => {
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10441c1); cpu.call(t); if cpu.returned_to != 0x10441c1 { return; }
  pc = 0x10441c1; continue;
 }
 0x10441c1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010441c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10441c2); }
 let mut pc: u32 = 0x10441c2;
 loop { match pc {
 0x10441c2 => {
  cpu.push(0x10441c7); f_010441f2(cpu); if cpu.returned_to != 0x10441c7 { return; }
  if cpu.cf { pc = 0x10441d3; continue; }
  pc = 0x10441c9; continue;
 }
 0x10441c9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x11b5190u32, v); }
  pc = 0x10441d3; continue;
 }
 0x10441d3 => {
  { cpu.sub8(cpu.r8(0x1128991u32), 0x0u8); }
  if !cpu.zf { pc = 0x10441ea; continue; }
  pc = 0x10441dc; continue;
 }
 0x10441dc => {
  cpu.push(0x10441e1); f_010445c6(cpu); if cpu.returned_to != 0x10441e1 { return; }
  { cpu.sub8(cpu.r8(0x112639bu32), 0x1u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10441ea; continue; }
  pc = 0x10441ea; continue;
 }
 0x10441ea => {
  { let v: u8 = 0x0u8; cpu.w8(0x1128991u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010441f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10441f2); }
 let mut pc: u32 = 0x10441f2;
 loop { match pc {
 0x10441f2 => {
  { cpu.sub8(cpu.r8(0x1126395u32), 0x1u8); }
  if (cpu.sf == cpu.of) { pc = 0x1044221; continue; }
  pc = 0x10441fb; continue;
 }
 0x10441fb => {
  { cpu.sub8(cpu.r8(0x112639bu32), 0x1u8); }
  if (cpu.sf == cpu.of) { pc = 0x1044221; continue; }
  pc = 0x1044204; continue;
 }
 0x1044204 => {
  { cpu.sub8(cpu.r8(0x1126396u32), 0x1u8); }
  if (cpu.sf == cpu.of) { pc = 0x1044221; continue; }
  pc = 0x104420d; continue;
 }
 0x104420d => {
  { cpu.sub8(cpu.r8(0x112639cu32), 0x1u8); }
  if (cpu.sf == cpu.of) { pc = 0x1044221; continue; }
  pc = 0x1044216; continue;
 }
 0x1044216 => {
  { cpu.sub8(cpu.r8(0x1126398u32), 0x1u8); }
  if (cpu.sf == cpu.of) { pc = 0x1044221; continue; }
  pc = 0x104421f; continue;
 }
 0x104421f => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044221 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044223(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044223); }
 let mut pc: u32 = 0x1044223;
 loop { match pc {
 0x1044223 => {
  cpu.push(0x1044228); f_010445c6(cpu); if cpu.returned_to != 0x1044228 { return; }
  { cpu.sub8(cpu.r8(0x1126395u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044231; continue; }
  pc = 0x1044231; continue;
 }
 0x1044231 => {
  { let v: u8 = 0x1u8; cpu.w8(0x1126395u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112639bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044247(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044247); }
 let mut pc: u32 = 0x1044247;
 loop { match pc {
 0x1044247 => {
  { cpu.sub8(cpu.r8(0x11c6304u32), 0x0u8); }
  if cpu.zf { pc = 0x1044251; continue; }
  pc = 0x1044250; continue;
 }
 0x1044250 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044251 => {
  { cpu.sub8(cpu.r8(0x1126395u32), 0x0u8); }
  if !cpu.zf { pc = 0x104425a; continue; }
  pc = 0x104425a; continue;
 }
 0x104425a => {
  cpu.push(0x104425f); f_010445c6(cpu); if cpu.returned_to != 0x104425f { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x1126396u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112639bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044275(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044275); }
 let mut pc: u32 = 0x1044275;
 loop { match pc {
 0x1044275 => {
  cpu.push(0x104427a); f_010445c6(cpu); if cpu.returned_to != 0x104427a { return; }
  { cpu.sub8(cpu.r8(0x1126395u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044283; continue; }
  pc = 0x1044283; continue;
 }
 0x1044283 => {
  { let v: u8 = 0x1u8; cpu.w8(0x112639cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112639bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044299(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044299); }
 let mut pc: u32 = 0x1044299;
 loop { match pc {
 0x1044299 => {
  cpu.push(0x104429e); f_010445c6(cpu); if cpu.returned_to != 0x104429e { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x1126397u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112639bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010442b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10442b4); }
 let mut pc: u32 = 0x10442b4;
 loop { match pc {
 0x10442b4 => {
  cpu.push(0x10442b9); f_010445c6(cpu); if cpu.returned_to != 0x10442b9 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x1126398u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112639bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010442cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10442cf); }
 let mut pc: u32 = 0x10442cf;
 loop { match pc {
 0x10442cf => {
  cpu.push(0x10442d4); f_010445c6(cpu); if cpu.returned_to != 0x10442d4 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x112639au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112639bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010442ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10442ea); }
 let mut pc: u32 = 0x10442ea;
 loop { match pc {
 0x10442ea => {
  { cpu.sub8(cpu.r8(0x112639bu32), 0x0u8); }
  if !cpu.zf { pc = 0x1044306; continue; }
  pc = 0x10442f3; continue;
 }
 0x10442f3 => {
  cpu.push(0x10442f8); f_010445c6(cpu); if cpu.returned_to != 0x10442f8 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x112639bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  pc = 0x1044306; continue;
 }
 0x1044306 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044307(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044307); }
 let mut pc: u32 = 0x1044307;
 loop { match pc {
 0x1044307 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef311u32, v); }
  cpu.push(0x1044313); f_010445c6(cpu); if cpu.returned_to != 0x1044313 { return; }
  { let v: u8 = 0x2u8; cpu.w8(0x1126395u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112639bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x104436b; continue; }
  pc = 0x1044334; continue;
 }
 0x1044334 => {
  cpu.push(0x1044339); f_010129b5(cpu); if cpu.returned_to != 0x1044339 { return; }
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  cpu.push(0x1044343); f_01012780(cpu); if cpu.returned_to != 0x1044343 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104436b; continue; }
  pc = 0x1044348; continue;
 }
 0x1044348 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1044352); f_01012bd0(cpu); if cpu.returned_to != 0x1044352 { return; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  cpu.push(0x104435c); f_01012780(cpu); if cpu.returned_to != 0x104435c { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104436b; continue; }
  pc = 0x1044361; continue;
 }
 0x1044361 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x104436b); f_01012bd0(cpu); if cpu.returned_to != 0x104436b { return; }
  pc = 0x104436b; continue;
 }
 0x104436b => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104436e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104436e); }
 let mut pc: u32 = 0x104436e;
 loop { match pc {
 0x104436e => {
  { cpu.sub8(cpu.r8(0x11c6304u32), 0x0u8); }
  if cpu.zf { pc = 0x1044378; continue; }
  pc = 0x1044377; continue;
 }
 0x1044377 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044378 => {
  cpu.push(0x104437d); f_010445c6(cpu); if cpu.returned_to != 0x104437d { return; }
  { let v: u8 = 0x2u8; cpu.w8(0x1126396u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112639bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x10443d5; continue; }
  pc = 0x104439e; continue;
 }
 0x104439e => {
  cpu.push(0x10443a3); f_010129b5(cpu); if cpu.returned_to != 0x10443a3 { return; }
  { let v: u32 = 0x1du32; cpu.eax = v; }
  cpu.push(0x10443ad); f_01012780(cpu); if cpu.returned_to != 0x10443ad { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10443d5; continue; }
  pc = 0x10443b2; continue;
 }
 0x10443b2 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10443bc); f_01012bd0(cpu); if cpu.returned_to != 0x10443bc { return; }
  { let v: u32 = 0x1eu32; cpu.eax = v; }
  cpu.push(0x10443c6); f_01012780(cpu); if cpu.returned_to != 0x10443c6 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10443d5; continue; }
  pc = 0x10443cb; continue;
 }
 0x10443cb => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x10443d5); f_01012bd0(cpu); if cpu.returned_to != 0x10443d5 { return; }
  pc = 0x10443d5; continue;
 }
 0x10443d5 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010443d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10443d8); }
 let mut pc: u32 = 0x10443d8;
 loop { match pc {
 0x10443d8 => {
  cpu.push(0x10443dd); f_010445c6(cpu); if cpu.returned_to != 0x10443dd { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112639bu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x112639cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1044435; continue; }
  pc = 0x10443fe; continue;
 }
 0x10443fe => {
  cpu.push(0x1044403); f_010129b5(cpu); if cpu.returned_to != 0x1044403 { return; }
  { let v: u32 = 0x1du32; cpu.eax = v; }
  cpu.push(0x104440d); f_01012780(cpu); if cpu.returned_to != 0x104440d { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044435; continue; }
  pc = 0x1044412; continue;
 }
 0x1044412 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x104441c); f_01012bd0(cpu); if cpu.returned_to != 0x104441c { return; }
  { let v: u32 = 0x1eu32; cpu.eax = v; }
  cpu.push(0x1044426); f_01012780(cpu); if cpu.returned_to != 0x1044426 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044435; continue; }
  pc = 0x104442b; continue;
 }
 0x104442b => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1044435); f_01012bd0(cpu); if cpu.returned_to != 0x1044435 { return; }
  pc = 0x1044435; continue;
 }
 0x1044435 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044438(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044438); }
 let mut pc: u32 = 0x1044438;
 loop { match pc {
 0x1044438 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef311u32, v); }
  cpu.push(0x1044444); f_010445c6(cpu); if cpu.returned_to != 0x1044444 { return; }
  { let v: u8 = 0x2u8; cpu.w8(0x1126397u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112639bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x104449c; continue; }
  pc = 0x1044465; continue;
 }
 0x1044465 => {
  cpu.push(0x104446a); f_010129b5(cpu); if cpu.returned_to != 0x104446a { return; }
  { let v: u32 = 0xdu32; cpu.eax = v; }
  cpu.push(0x1044474); f_01012780(cpu); if cpu.returned_to != 0x1044474 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104449c; continue; }
  pc = 0x1044479; continue;
 }
 0x1044479 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1044483); f_01012bd0(cpu); if cpu.returned_to != 0x1044483 { return; }
  { let v: u32 = 0xeu32; cpu.eax = v; }
  cpu.push(0x104448d); f_01012780(cpu); if cpu.returned_to != 0x104448d { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104449c; continue; }
  pc = 0x1044492; continue;
 }
 0x1044492 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x104449c); f_01012bd0(cpu); if cpu.returned_to != 0x104449c { return; }
  pc = 0x104449c; continue;
 }
 0x104449c => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104449f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104449f); }
 let mut pc: u32 = 0x104449f;
 loop { match pc {
 0x104449f => {
  cpu.push(0x10444a4); f_010445c6(cpu); if cpu.returned_to != 0x10444a4 { return; }
  { let v: u8 = 0x2u8; cpu.w8(0x1126398u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x10444f5; continue; }
  pc = 0x10444be; continue;
 }
 0x10444be => {
  cpu.push(0x10444c3); f_010129b5(cpu); if cpu.returned_to != 0x10444c3 { return; }
  { let v: u32 = 0xdu32; cpu.eax = v; }
  cpu.push(0x10444cd); f_01012780(cpu); if cpu.returned_to != 0x10444cd { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10444f5; continue; }
  pc = 0x10444d2; continue;
 }
 0x10444d2 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10444dc); f_01012bd0(cpu); if cpu.returned_to != 0x10444dc { return; }
  { let v: u32 = 0xeu32; cpu.eax = v; }
  cpu.push(0x10444e6); f_01012780(cpu); if cpu.returned_to != 0x10444e6 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10444f5; continue; }
  pc = 0x10444eb; continue;
 }
 0x10444eb => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x10444f5); f_01012bd0(cpu); if cpu.returned_to != 0x10444f5 { return; }
  pc = 0x10444f5; continue;
 }
 0x10444f5 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010444f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10444f8); }
 let mut pc: u32 = 0x10444f8;
 loop { match pc {
 0x10444f8 => {
  cpu.push(0x10444fd); f_010445c6(cpu); if cpu.returned_to != 0x10444fd { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112639bu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010445c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10445c6); }
 let mut pc: u32 = 0x10445c6;
 loop { match pc {
 0x10445c6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1126395u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112639cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1126396u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1126397u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1126398u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1126399u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112639au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010445f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10445f8); }
 let mut pc: u32 = 0x10445f8;
 loop { match pc {
 0x10445f8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010445f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10445f9); }
 let mut pc: u32 = 0x10445f9;
 loop { match pc {
 0x10445f9 => {
  { let v: u8 = cpu.r8(0x112639du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x1044610; continue; }
  pc = 0x1044602; continue;
 }
 0x1044602 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x112c97cu32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x11263c5u32, v); }
  pc = 0x104461b; continue;
 }
 0x1044610 => {
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x10fcd59u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x112c97cu32, v); }
  pc = 0x104461b; continue;
 }
 0x104461b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104461c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104461c); }
 let mut pc: u32 = 0x104461c;
 loop { match pc {
 0x104461c => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1044626; continue; }
  pc = 0x1044625; continue;
 }
 0x1044625 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044626 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1044669; continue; }
  pc = 0x1044632; continue;
 }
 0x1044632 => {
  cpu.push(0x1044637); f_010129b5(cpu); if cpu.returned_to != 0x1044637 { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x1044641); f_01012780(cpu); if cpu.returned_to != 0x1044641 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044669; continue; }
  pc = 0x1044646; continue;
 }
 0x1044646 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1044650); f_01012bd0(cpu); if cpu.returned_to != 0x1044650 { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x104465a); f_01012780(cpu); if cpu.returned_to != 0x104465a { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044669; continue; }
  pc = 0x104465f; continue;
 }
 0x104465f => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1044669); f_01012bd0(cpu); if cpu.returned_to != 0x1044669 { return; }
  pc = 0x1044669; continue;
 }
 0x1044669 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044691; continue; }
  pc = 0x1044674; continue;
 }
 0x1044674 => {
  { let v: u8 = 0x3u8; cpu.w8(0x11263beu32, v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x112639du32)); cpu.w8(0x112639du32, v); }
  { cpu.sub8(cpu.r8(0x112639du32), 0x0u8); }
  if !cpu.sf { pc = 0x1044691; continue; }
  pc = 0x104468a; continue;
 }
 0x104468a => {
  { let v: u8 = 0x2u8; cpu.w8(0x112639du32, v); }
  pc = 0x1044691; continue;
 }
 0x1044691 => {
  cpu.push(0x1044696); f_010445f9(cpu); if cpu.returned_to != 0x1044696 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044697(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044697); }
 let mut pc: u32 = 0x1044697;
 loop { match pc {
 0x1044697 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10446a1; continue; }
  pc = 0x10446a0; continue;
 }
 0x10446a0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10446a1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x10446e4; continue; }
  pc = 0x10446ad; continue;
 }
 0x10446ad => {
  cpu.push(0x10446b2); f_010129b5(cpu); if cpu.returned_to != 0x10446b2 { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x10446bc); f_01012780(cpu); if cpu.returned_to != 0x10446bc { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10446e4; continue; }
  pc = 0x10446c1; continue;
 }
 0x10446c1 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10446cb); f_01012bd0(cpu); if cpu.returned_to != 0x10446cb { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x10446d5); f_01012780(cpu); if cpu.returned_to != 0x10446d5 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10446e4; continue; }
  pc = 0x10446da; continue;
 }
 0x10446da => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x10446e4); f_01012bd0(cpu); if cpu.returned_to != 0x10446e4 { return; }
  pc = 0x10446e4; continue;
 }
 0x10446e4 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x104470c; continue; }
  pc = 0x10446ef; continue;
 }
 0x10446ef => {
  { let v: u8 = 0x2u8; cpu.w8(0x11263beu32, v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x112639du32)); cpu.w8(0x112639du32, v); }
  { cpu.sub8(cpu.r8(0x112639du32), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x104470c; continue; }
  pc = 0x1044705; continue;
 }
 0x1044705 => {
  { let v: u8 = 0x0u8; cpu.w8(0x112639du32, v); }
  pc = 0x104470c; continue;
 }
 0x104470c => {
  cpu.push(0x1044711); f_010445f9(cpu); if cpu.returned_to != 0x1044711 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044712(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044712); }
 let mut pc: u32 = 0x1044712;
 loop { match pc {
 0x1044712 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1044725; continue; }
  pc = 0x104471b; continue;
 }
 0x104471b => {
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if cpu.zf { pc = 0x1044725; continue; }
  pc = 0x1044724; continue;
 }
 0x1044724 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044725 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1044768; continue; }
  pc = 0x1044731; continue;
 }
 0x1044731 => {
  cpu.push(0x1044736); f_010129b5(cpu); if cpu.returned_to != 0x1044736 { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x1044740); f_01012780(cpu); if cpu.returned_to != 0x1044740 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044768; continue; }
  pc = 0x1044745; continue;
 }
 0x1044745 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x104474f); f_01012bd0(cpu); if cpu.returned_to != 0x104474f { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x1044759); f_01012780(cpu); if cpu.returned_to != 0x1044759 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044768; continue; }
  pc = 0x104475e; continue;
 }
 0x104475e => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1044768); f_01012bd0(cpu); if cpu.returned_to != 0x1044768 { return; }
  pc = 0x1044768; continue;
 }
 0x1044768 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044787; continue; }
  pc = 0x1044773; continue;
 }
 0x1044773 => {
  { let v: u8 = 0x3u8; cpu.w8(0x11263bfu32, v); }
  pc = 0x104477a; continue;
 }
 0x104477a => {
  { let v: u8 = cpu.dec8(cpu.r8(0x11263a0u32)); cpu.w8(0x11263a0u32, v); }
  { let v: u8 = cpu.r8(0x11263a0u32) & 0x7u8; cpu.w8(0x11263a0u32, v); }
  pc = 0x1044787; continue;
 }
 0x1044787 => {
  cpu.push(0x104478c); f_01044810(cpu); if cpu.returned_to != 0x104478c { return; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x104477a; continue; }
  pc = 0x1044790; continue;
 }
 0x1044790 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044791(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044791); }
 let mut pc: u32 = 0x1044791;
 loop { match pc {
 0x1044791 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10447a4; continue; }
  pc = 0x104479a; continue;
 }
 0x104479a => {
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if cpu.zf { pc = 0x10447a4; continue; }
  pc = 0x10447a3; continue;
 }
 0x10447a3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10447a4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x10447e7; continue; }
  pc = 0x10447b0; continue;
 }
 0x10447b0 => {
  cpu.push(0x10447b5); f_010129b5(cpu); if cpu.returned_to != 0x10447b5 { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x10447bf); f_01012780(cpu); if cpu.returned_to != 0x10447bf { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10447e7; continue; }
  pc = 0x10447c4; continue;
 }
 0x10447c4 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10447ce); f_01012bd0(cpu); if cpu.returned_to != 0x10447ce { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x10447d8); f_01012780(cpu); if cpu.returned_to != 0x10447d8 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10447e7; continue; }
  pc = 0x10447dd; continue;
 }
 0x10447dd => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x10447e7); f_01012bd0(cpu); if cpu.returned_to != 0x10447e7 { return; }
  pc = 0x10447e7; continue;
 }
 0x10447e7 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044806; continue; }
  pc = 0x10447f2; continue;
 }
 0x10447f2 => {
  { let v: u8 = 0x2u8; cpu.w8(0x11263bfu32, v); }
  pc = 0x10447f9; continue;
 }
 0x10447f9 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11263a0u32)); cpu.w8(0x11263a0u32, v); }
  { let v: u8 = cpu.r8(0x11263a0u32) & 0x7u8; cpu.w8(0x11263a0u32, v); }
  pc = 0x1044806; continue;
 }
 0x1044806 => {
  cpu.push(0x104480b); f_01044810(cpu); if cpu.returned_to != 0x104480b { return; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x10447f9; continue; }
  pc = 0x104480f; continue;
 }
 0x104480f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044810); }
 let mut pc: u32 = 0x1044810;
 loop { match pc {
 0x1044810 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11263c5u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10fcd35u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x11263a0u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fcd2fu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044832(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044832); }
 let mut pc: u32 = 0x1044832;
 loop { match pc {
 0x1044832 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x104484b; continue; }
  pc = 0x104483b; continue;
 }
 0x104483b => {
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x104484b; continue; }
  pc = 0x1044844; continue;
 }
 0x1044844 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11263c6u32, v); }
  pc = 0x104484b; continue;
 }
 0x104484b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104484c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104484c); }
 let mut pc: u32 = 0x104484c;
 loop { match pc {
 0x104484c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x104488f; continue; }
  pc = 0x1044858; continue;
 }
 0x1044858 => {
  cpu.push(0x104485d); f_010129b5(cpu); if cpu.returned_to != 0x104485d { return; }
  { let v: u32 = 0x23u32; cpu.eax = v; }
  cpu.push(0x1044867); f_01012780(cpu); if cpu.returned_to != 0x1044867 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104488f; continue; }
  pc = 0x104486c; continue;
 }
 0x104486c => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1044876); f_01012bd0(cpu); if cpu.returned_to != 0x1044876 { return; }
  { let v: u32 = 0x24u32; cpu.eax = v; }
  cpu.push(0x1044880); f_01012780(cpu); if cpu.returned_to != 0x1044880 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104488f; continue; }
  pc = 0x1044885; continue;
 }
 0x1044885 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x104488f); f_01012bd0(cpu); if cpu.returned_to != 0x104488f { return; }
  pc = 0x104488f; continue;
 }
 0x104488f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x10448aa; continue; }
  pc = 0x104489a; continue;
 }
 0x104489a => {
  { cpu.sub8(cpu.r8(0x11263c6u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x10448aa; continue; }
  pc = 0x10448a3; continue;
 }
 0x10448a3 => {
  { let v: u8 = 0x2u8; cpu.w8(0x11263c6u32, v); }
  pc = 0x10448aa; continue;
 }
 0x10448aa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010448ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10448ab); }
 let mut pc: u32 = 0x10448ab;
 loop { match pc {
 0x10448ab => {
  { cpu.sub8(cpu.r8(0x10fd411u32), 0x0u8); }
  if cpu.zf { pc = 0x10448be; continue; }
  pc = 0x10448b4; continue;
 }
 0x10448b4 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10448be; continue; }
  pc = 0x10448bd; continue;
 }
 0x10448bd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10448be => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x10448ec; continue; }
  pc = 0x10448c7; continue;
 }
 0x10448c7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x10448ec; continue; }
  pc = 0x10448d7; continue;
 }
 0x10448d7 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11263c2u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c3u32, v); }
  pc = 0x10448ec; continue;
 }
 0x10448ec => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010448ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10448ed); }
 let mut pc: u32 = 0x10448ed;
 loop { match pc {
 0x10448ed => {
  { cpu.sub8(cpu.r8(0x10fd411u32), 0x0u8); }
  if cpu.zf { pc = 0x1044900; continue; }
  pc = 0x10448f6; continue;
 }
 0x10448f6 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1044900; continue; }
  pc = 0x10448ff; continue;
 }
 0x10448ff => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044900 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044973; continue; }
  pc = 0x1044909; continue;
 }
 0x1044909 => {
  { cpu.sub8(cpu.r8(0x11263c2u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1044973; continue; }
  pc = 0x1044912; continue;
 }
 0x1044912 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1044955; continue; }
  pc = 0x104491e; continue;
 }
 0x104491e => {
  cpu.push(0x1044923); f_010129b5(cpu); if cpu.returned_to != 0x1044923 { return; }
  { let v: u32 = 0x19u32; cpu.eax = v; }
  cpu.push(0x104492d); f_01012780(cpu); if cpu.returned_to != 0x104492d { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044955; continue; }
  pc = 0x1044932; continue;
 }
 0x1044932 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x104493c); f_01012bd0(cpu); if cpu.returned_to != 0x104493c { return; }
  { let v: u32 = 0x1au32; cpu.eax = v; }
  cpu.push(0x1044946); f_01012780(cpu); if cpu.returned_to != 0x1044946 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044955; continue; }
  pc = 0x104494b; continue;
 }
 0x104494b => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1044955); f_01012bd0(cpu); if cpu.returned_to != 0x1044955 { return; }
  pc = 0x1044955; continue;
 }
 0x1044955 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x11263c2u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c3u32, v); }
  pc = 0x1044973; continue;
 }
 0x1044973 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044974(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044974); }
 let mut pc: u32 = 0x1044974;
 loop { match pc {
 0x1044974 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x104497e; continue; }
  pc = 0x104497d; continue;
 }
 0x104497d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104497e => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x10449ac; continue; }
  pc = 0x1044987; continue;
 }
 0x1044987 => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { cpu.sub8(cpu.r8(0x11263c3u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x10449ac; continue; }
  pc = 0x1044997; continue;
 }
 0x1044997 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11263c3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c2u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c4u32, v); }
  pc = 0x10449ac; continue;
 }
 0x10449ac => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010449ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10449ad); }
 let mut pc: u32 = 0x10449ad;
 loop { match pc {
 0x10449ad => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10449b7; continue; }
  pc = 0x10449b6; continue;
 }
 0x10449b6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10449b7 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044a2a; continue; }
  pc = 0x10449c0; continue;
 }
 0x10449c0 => {
  { cpu.sub8(cpu.r8(0x11263c3u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1044a2a; continue; }
  pc = 0x10449c9; continue;
 }
 0x10449c9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1044a0c; continue; }
  pc = 0x10449d5; continue;
 }
 0x10449d5 => {
  cpu.push(0x10449da); f_010129b5(cpu); if cpu.returned_to != 0x10449da { return; }
  { let v: u32 = 0xdu32; cpu.eax = v; }
  cpu.push(0x10449e4); f_01012780(cpu); if cpu.returned_to != 0x10449e4 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044a0c; continue; }
  pc = 0x10449e9; continue;
 }
 0x10449e9 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10449f3); f_01012bd0(cpu); if cpu.returned_to != 0x10449f3 { return; }
  { let v: u32 = 0xeu32; cpu.eax = v; }
  cpu.push(0x10449fd); f_01012780(cpu); if cpu.returned_to != 0x10449fd { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044a0c; continue; }
  pc = 0x1044a02; continue;
 }
 0x1044a02 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1044a0c); f_01012bd0(cpu); if cpu.returned_to != 0x1044a0c { return; }
  pc = 0x1044a0c; continue;
 }
 0x1044a0c => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x11263c3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c2u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c4u32, v); }
  pc = 0x1044a2a; continue;
 }
 0x1044a2a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044a2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044a2b); }
 let mut pc: u32 = 0x1044a2b;
 loop { match pc {
 0x1044a2b => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1044a3e; continue; }
  pc = 0x1044a34; continue;
 }
 0x1044a34 => {
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if cpu.zf { pc = 0x1044a3e; continue; }
  pc = 0x1044a3d; continue;
 }
 0x1044a3d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044a3e => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044a75; continue; }
  pc = 0x1044a47; continue;
 }
 0x1044a47 => {
  { cpu.sub8(cpu.r8(0x11263c4u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1044a75; continue; }
  pc = 0x1044a50; continue;
 }
 0x1044a50 => {
  { cpu.sub32(cpu.r32(0x112894cu32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x1044a75; continue; }
  pc = 0x1044a59; continue;
 }
 0x1044a59 => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11263c4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c2u32, v); }
  pc = 0x1044a75; continue;
 }
 0x1044a75 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044a76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044a76); }
 let mut pc: u32 = 0x1044a76;
 loop { match pc {
 0x1044a76 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1044a89; continue; }
  pc = 0x1044a7f; continue;
 }
 0x1044a7f => {
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if cpu.zf { pc = 0x1044a89; continue; }
  pc = 0x1044a88; continue;
 }
 0x1044a88 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044a89 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044b33; continue; }
  pc = 0x1044a96; continue;
 }
 0x1044a96 => {
  { cpu.sub8(cpu.r8(0x11263c4u32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1044b33; continue; }
  pc = 0x1044aa3; continue;
 }
 0x1044aa3 => {
  { cpu.sub32(cpu.r32(0x112894cu32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x1044b33; continue; }
  pc = 0x1044ab0; continue;
 }
 0x1044ab0 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11263bdu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c2u32, v); }
  { let v: u8 = 0x2u8; cpu.w8(0x11263c4u32, v); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1044b21; continue; }
  pc = 0x1044adc; continue;
 }
 0x1044adc => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1044b1f; continue; }
  pc = 0x1044ae8; continue;
 }
 0x1044ae8 => {
  cpu.push(0x1044aed); f_010129b5(cpu); if cpu.returned_to != 0x1044aed { return; }
  { let v: u32 = 0x13u32; cpu.eax = v; }
  cpu.push(0x1044af7); f_01012780(cpu); if cpu.returned_to != 0x1044af7 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044b1f; continue; }
  pc = 0x1044afc; continue;
 }
 0x1044afc => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1044b06); f_01012bd0(cpu); if cpu.returned_to != 0x1044b06 { return; }
  { let v: u32 = 0x14u32; cpu.eax = v; }
  cpu.push(0x1044b10); f_01012780(cpu); if cpu.returned_to != 0x1044b10 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044b1f; continue; }
  pc = 0x1044b15; continue;
 }
 0x1044b15 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1044b1f); f_01012bd0(cpu); if cpu.returned_to != 0x1044b1f { return; }
  pc = 0x1044b1f; continue;
 }
 0x1044b1f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1044b21; continue;
 }
 0x1044b21 => {
  { let v: u16 = 0x0u16; cpu.w16(0x10fd3efu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10fd3f1u32, v); }
  pc = 0x1044b33; continue;
 }
 0x1044b33 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044b34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044b34); }
 let mut pc: u32 = 0x1044b34;
 loop { match pc {
 0x1044b34 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044b6b; continue; }
  pc = 0x1044b3d; continue;
 }
 0x1044b3d => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1044b6b; continue; }
  pc = 0x1044b46; continue;
 }
 0x1044b46 => {
  { cpu.sub8(cpu.r8(0x10fd3eau32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1044b6b; continue; }
  pc = 0x1044b4f; continue;
 }
 0x1044b4f => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd3eau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c2u32, v); }
  pc = 0x1044b6b; continue;
 }
 0x1044b6b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044b6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044b6c); }
 let mut pc: u32 = 0x1044b6c;
 loop { match pc {
 0x1044b6c => {
  { cpu.sub8(cpu.r8(0x10fd3eau32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1044b91; continue; }
  pc = 0x1044b75; continue;
 }
 0x1044b75 => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd3eau32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c2u32, v); }
  pc = 0x1044b91; continue;
 }
 0x1044b91 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044c25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044c25); }
 let mut pc: u32 = 0x1044c25;
 loop { match pc {
 0x1044c25 => {
  { let v: u32 = cpu.r32(0x11310f8u32); cpu.esi = v; }
  cpu.push(0x1044c30); f_01040c19(cpu); if cpu.returned_to != 0x1044c30 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044c31(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044c31); }
 let mut pc: u32 = 0x1044c31;
 loop { match pc {
 0x1044c31 => {
  { let v: u32 = cpu.r32(0x11310fcu32); cpu.esi = v; }
  cpu.push(0x1044c3c); f_01040c19(cpu); if cpu.returned_to != 0x1044c3c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044c3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044c3d); }
 let mut pc: u32 = 0x1044c3d;
 loop { match pc {
 0x1044c3d => {
  { cpu.sub8(cpu.r8(0x1128991u32), 0x0u8); }
  if cpu.zf { pc = 0x1044c62; continue; }
  pc = 0x1044c46; continue;
 }
 0x1044c46 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11263beu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263bfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c1u32, v); }
  pc = 0x1044c62; continue;
 }
 0x1044c62 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1128991u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044c6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044c6a); }
 let mut pc: u32 = 0x1044c6a;
 loop { match pc {
 0x1044c6a => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1044c74; continue; }
  pc = 0x1044c73; continue;
 }
 0x1044c73 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044c74 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044ca0; continue; }
  pc = 0x1044c7d; continue;
 }
 0x1044c7d => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11263beu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263bfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c1u32, v); }
  pc = 0x1044ca0; continue;
 }
 0x1044ca0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044ca1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044ca1); }
 let mut pc: u32 = 0x1044ca1;
 loop { match pc {
 0x1044ca1 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1044cb4; continue; }
  pc = 0x1044caa; continue;
 }
 0x1044caa => {
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if cpu.zf { pc = 0x1044cb4; continue; }
  pc = 0x1044cb3; continue;
 }
 0x1044cb3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044cb4 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044ce0; continue; }
  pc = 0x1044cbd; continue;
 }
 0x1044cbd => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263beu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11263bfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c1u32, v); }
  pc = 0x1044ce0; continue;
 }
 0x1044ce0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044ce1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044ce1); }
 let mut pc: u32 = 0x1044ce1;
 loop { match pc {
 0x1044ce1 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1044ceb; continue; }
  pc = 0x1044cea; continue;
 }
 0x1044cea => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044ceb => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044d17; continue; }
  pc = 0x1044cf4; continue;
 }
 0x1044cf4 => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263beu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263bfu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11263c0u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c1u32, v); }
  pc = 0x1044d17; continue;
 }
 0x1044d17 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044d18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044d18); }
 let mut pc: u32 = 0x1044d18;
 loop { match pc {
 0x1044d18 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1044d22; continue; }
  pc = 0x1044d21; continue;
 }
 0x1044d21 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044d22 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044d4e; continue; }
  pc = 0x1044d2b; continue;
 }
 0x1044d2b => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263beu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263bfu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263c0u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11263c1u32, v); }
  pc = 0x1044d4e; continue;
 }
 0x1044d4e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044d4f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044d4f); }
 let mut pc: u32 = 0x1044d4f;
 loop { match pc {
 0x1044d4f => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1044d59; continue; }
  pc = 0x1044d58; continue;
 }
 0x1044d58 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044d59 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1044d9c; continue; }
  pc = 0x1044d65; continue;
 }
 0x1044d65 => {
  cpu.push(0x1044d6a); f_010129b5(cpu); if cpu.returned_to != 0x1044d6a { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x1044d74); f_01012780(cpu); if cpu.returned_to != 0x1044d74 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044d9c; continue; }
  pc = 0x1044d79; continue;
 }
 0x1044d79 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1044d83); f_01012bd0(cpu); if cpu.returned_to != 0x1044d83 { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x1044d8d); f_01012780(cpu); if cpu.returned_to != 0x1044d8d { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044d9c; continue; }
  pc = 0x1044d92; continue;
 }
 0x1044d92 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1044d9c); f_01012bd0(cpu); if cpu.returned_to != 0x1044d9c { return; }
  pc = 0x1044d9c; continue;
 }
 0x1044d9c => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044dc4; continue; }
  pc = 0x1044da7; continue;
 }
 0x1044da7 => {
  { let v: u8 = 0x3u8; cpu.w8(0x11263c0u32, v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x11263c5u32)); cpu.w8(0x11263c5u32, v); }
  { cpu.sub8(cpu.r8(0x11263c5u32), 0x1u8); }
  if !cpu.zf { pc = 0x1044dc4; continue; }
  pc = 0x1044dbd; continue;
 }
 0x1044dbd => {
  { let v: u8 = 0x4u8; cpu.w8(0x11263c5u32, v); }
  pc = 0x1044dc4; continue;
 }
 0x1044dc4 => {
  cpu.push(0x1044dc9); f_01044810(cpu); if cpu.returned_to != 0x1044dc9 { return; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x1044ddc; continue; }
  pc = 0x1044dcd; continue;
 }
 0x1044dcd => {
  { let v: u8 = cpu.dec8(cpu.r8(0x11263a0u32)); cpu.w8(0x11263a0u32, v); }
  { let v: u8 = cpu.r8(0x11263a0u32) & 0x7u8; cpu.w8(0x11263a0u32, v); }
  cpu.poll(); pc = 0x1044dc4; continue;
 }
 0x1044ddc => {
  { let v: u8 = cpu.r8(0x11263c5u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044de2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044de2); }
 let mut pc: u32 = 0x1044de2;
 loop { match pc {
 0x1044de2 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1044dec; continue; }
  pc = 0x1044deb; continue;
 }
 0x1044deb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044dec => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1044e2f; continue; }
  pc = 0x1044df8; continue;
 }
 0x1044df8 => {
  cpu.push(0x1044dfd); f_010129b5(cpu); if cpu.returned_to != 0x1044dfd { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x1044e07); f_01012780(cpu); if cpu.returned_to != 0x1044e07 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044e2f; continue; }
  pc = 0x1044e0c; continue;
 }
 0x1044e0c => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1044e16); f_01012bd0(cpu); if cpu.returned_to != 0x1044e16 { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x1044e20); f_01012780(cpu); if cpu.returned_to != 0x1044e20 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044e2f; continue; }
  pc = 0x1044e25; continue;
 }
 0x1044e25 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1044e2f); f_01012bd0(cpu); if cpu.returned_to != 0x1044e2f { return; }
  pc = 0x1044e2f; continue;
 }
 0x1044e2f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1044e57; continue; }
  pc = 0x1044e3a; continue;
 }
 0x1044e3a => {
  { let v: u8 = 0x2u8; cpu.w8(0x11263c0u32, v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x11263c5u32)); cpu.w8(0x11263c5u32, v); }
  { cpu.sub8(cpu.r8(0x11263c5u32), 0x5u8); }
  if !cpu.zf { pc = 0x1044e57; continue; }
  pc = 0x1044e50; continue;
 }
 0x1044e50 => {
  { let v: u8 = 0x2u8; cpu.w8(0x11263c5u32, v); }
  pc = 0x1044e57; continue;
 }
 0x1044e57 => {
  cpu.push(0x1044e5c); f_01044810(cpu); if cpu.returned_to != 0x1044e5c { return; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x1044e6f; continue; }
  pc = 0x1044e60; continue;
 }
 0x1044e60 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x11263a0u32)); cpu.w8(0x11263a0u32, v); }
  { let v: u8 = cpu.r8(0x11263a0u32) & 0x7u8; cpu.w8(0x11263a0u32, v); }
  cpu.poll(); pc = 0x1044e57; continue;
 }
 0x1044e6f => {
  { let v: u8 = cpu.r8(0x11263c5u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044e75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044e75); }
 let mut pc: u32 = 0x1044e75;
 loop { match pc {
 0x1044e75 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1044e7f; continue; }
  pc = 0x1044e7e; continue;
 }
 0x1044e7e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044e7f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1044ec2; continue; }
  pc = 0x1044e8b; continue;
 }
 0x1044e8b => {
  cpu.push(0x1044e90); f_010129b5(cpu); if cpu.returned_to != 0x1044e90 { return; }
  { let v: u32 = 0x15u32; cpu.eax = v; }
  cpu.push(0x1044e9a); f_01012780(cpu); if cpu.returned_to != 0x1044e9a { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044ec2; continue; }
  pc = 0x1044e9f; continue;
 }
 0x1044e9f => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1044ea9); f_01012bd0(cpu); if cpu.returned_to != 0x1044ea9 { return; }
  { let v: u32 = 0x16u32; cpu.eax = v; }
  cpu.push(0x1044eb3); f_01012780(cpu); if cpu.returned_to != 0x1044eb3 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044ec2; continue; }
  pc = 0x1044eb8; continue;
 }
 0x1044eb8 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1044ec2); f_01012bd0(cpu); if cpu.returned_to != 0x1044ec2 { return; }
  pc = 0x1044ec2; continue;
 }
 0x1044ec2 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x11263ceu32, v); }
  { let v: u16 = cpu.dec16(cpu.r16(0x112639eu32)); cpu.w16(0x112639eu32, v); }
  { let v: u16 = cpu.and16(cpu.r16(0x112639eu32), 0xfu16); cpu.w16(0x112639eu32, v); }
  { let v: u16 = cpu.r16(0x112639eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044ee2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044ee2); }
 let mut pc: u32 = 0x1044ee2;
 loop { match pc {
 0x1044ee2 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1044eec; continue; }
  pc = 0x1044eeb; continue;
 }
 0x1044eeb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1044eec => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1044f2f; continue; }
  pc = 0x1044ef8; continue;
 }
 0x1044ef8 => {
  cpu.push(0x1044efd); f_010129b5(cpu); if cpu.returned_to != 0x1044efd { return; }
  { let v: u32 = 0x15u32; cpu.eax = v; }
  cpu.push(0x1044f07); f_01012780(cpu); if cpu.returned_to != 0x1044f07 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044f2f; continue; }
  pc = 0x1044f0c; continue;
 }
 0x1044f0c => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1044f16); f_01012bd0(cpu); if cpu.returned_to != 0x1044f16 { return; }
  { let v: u32 = 0x16u32; cpu.eax = v; }
  cpu.push(0x1044f20); f_01012780(cpu); if cpu.returned_to != 0x1044f20 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044f2f; continue; }
  pc = 0x1044f25; continue;
 }
 0x1044f25 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1044f2f); f_01012bd0(cpu); if cpu.returned_to != 0x1044f2f { return; }
  pc = 0x1044f2f; continue;
 }
 0x1044f2f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x11263ceu32, v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x112639eu32)); cpu.w16(0x112639eu32, v); }
  { let v: u16 = cpu.and16(cpu.r16(0x112639eu32), 0xfu16); cpu.w16(0x112639eu32, v); }
  { let v: u16 = cpu.r16(0x112639eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044f4f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044f4f); }
 let mut pc: u32 = 0x1044f4f;
 loop { match pc {
 0x1044f4f => {
  { let v: u8 = 0x3u8; cpu.w8(0x11263cdu32, v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x10fcd59u32)); cpu.w8(0x10fcd59u32, v); }
  { let v: u8 = cpu.and8(cpu.r8(0x10fcd59u32), 0x1u8); cpu.w8(0x10fcd59u32, v); }
  cpu.push(0x1044f68); f_010445f9(cpu); if cpu.returned_to != 0x1044f68 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044f83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044f83); }
 let mut pc: u32 = 0x1044f83;
 loop { match pc {
 0x1044f83 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11263ceu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044f8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044f8b); }
 let mut pc: u32 = 0x1044f8b;
 loop { match pc {
 0x1044f8b => {
  { let v: u8 = 0x1u8; cpu.w8(0x11263cdu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01044f93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1044f93); }
 let mut pc: u32 = 0x1044f93;
 loop { match pc {
 0x1044f93 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1045010; continue; }
  pc = 0x1044f9c; continue;
 }
 0x1044f9c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1044fdf; continue; }
  pc = 0x1044fa8; continue;
 }
 0x1044fa8 => {
  cpu.push(0x1044fad); f_010129b5(cpu); if cpu.returned_to != 0x1044fad { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x1044fb7); f_01012780(cpu); if cpu.returned_to != 0x1044fb7 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044fdf; continue; }
  pc = 0x1044fbc; continue;
 }
 0x1044fbc => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1044fc6); f_01012bd0(cpu); if cpu.returned_to != 0x1044fc6 { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x1044fd0); f_01012780(cpu); if cpu.returned_to != 0x1044fd0 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1044fdf; continue; }
  pc = 0x1044fd5; continue;
 }
 0x1044fd5 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1044fdf); f_01012bd0(cpu); if cpu.returned_to != 0x1044fdf { return; }
  pc = 0x1044fdf; continue;
 }
 0x1044fdf => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x1126366u32, v); }
  pc = 0x1044fe8; continue;
 }
 0x1044fe8 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11263c7u32)); cpu.w8(0x11263c7u32, v); }
  { cpu.sub8(cpu.r8(0x11263c7u32), 0x10u8); }
  if !cpu.zf { pc = 0x1044ffe; continue; }
  pc = 0x1044ff7; continue;
 }
 0x1044ff7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11263c7u32, v); }
  pc = 0x1044ffe; continue;
 }
 0x1044ffe => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11263c7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd52eu32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x1045010; continue; }
  pc = 0x104500e; continue;
 }
 0x104500e => {
  cpu.poll(); pc = 0x1044fe8; continue;
 }
 0x1045010 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045011(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045011); }
 let mut pc: u32 = 0x1045011;
 loop { match pc {
 0x1045011 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x104508e; continue; }
  pc = 0x104501a; continue;
 }
 0x104501a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x104505d; continue; }
  pc = 0x1045026; continue;
 }
 0x1045026 => {
  cpu.push(0x104502b); f_010129b5(cpu); if cpu.returned_to != 0x104502b { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x1045035); f_01012780(cpu); if cpu.returned_to != 0x1045035 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104505d; continue; }
  pc = 0x104503a; continue;
 }
 0x104503a => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1045044); f_01012bd0(cpu); if cpu.returned_to != 0x1045044 { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x104504e); f_01012780(cpu); if cpu.returned_to != 0x104504e { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104505d; continue; }
  pc = 0x1045053; continue;
 }
 0x1045053 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x104505d); f_01012bd0(cpu); if cpu.returned_to != 0x104505d { return; }
  pc = 0x104505d; continue;
 }
 0x104505d => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x1126366u32, v); }
  pc = 0x1045066; continue;
 }
 0x1045066 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x11263c7u32)); cpu.w8(0x11263c7u32, v); }
  { cpu.sub8(cpu.r8(0x11263c7u32), 0x0u8); }
  if !cpu.sf { pc = 0x104507c; continue; }
  pc = 0x1045075; continue;
 }
 0x1045075 => {
  { let v: u8 = 0xfu8; cpu.w8(0x11263c7u32, v); }
  pc = 0x104507c; continue;
 }
 0x104507c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11263c7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd52eu32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x104508e; continue; }
  pc = 0x104508c; continue;
 }
 0x104508c => {
  cpu.poll(); pc = 0x1045066; continue;
 }
 0x104508e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104508f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104508f); }
 let mut pc: u32 = 0x104508f;
 loop { match pc {
 0x104508f => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1045113; continue; }
  pc = 0x1045098; continue;
 }
 0x1045098 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x10450db; continue; }
  pc = 0x10450a4; continue;
 }
 0x10450a4 => {
  cpu.push(0x10450a9); f_010129b5(cpu); if cpu.returned_to != 0x10450a9 { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x10450b3); f_01012780(cpu); if cpu.returned_to != 0x10450b3 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10450db; continue; }
  pc = 0x10450b8; continue;
 }
 0x10450b8 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10450c2); f_01012bd0(cpu); if cpu.returned_to != 0x10450c2 { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x10450cc); f_01012780(cpu); if cpu.returned_to != 0x10450cc { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10450db; continue; }
  pc = 0x10450d1; continue;
 }
 0x10450d1 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x10450db); f_01012bd0(cpu); if cpu.returned_to != 0x10450db { return; }
  pc = 0x10450db; continue;
 }
 0x10450db => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x1126365u32, v); }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x10450eb; continue;
 }
 0x10450eb => {
  { let v: u8 = cpu.r8(0x10e3e81u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ebx as u8), 0x3u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x10e3e81u32, v); }
  { let v: u8 = cpu.r8(0x11b4c8fu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11263c7u32, v); }
  cpu.push(0x104510c); f_010409d8(cpu); if cpu.returned_to != 0x104510c { return; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x10450eb; continue; }
  pc = 0x104510e; continue;
 }
 0x104510e => {
  cpu.push(0x1045113); f_010409a3(cpu); if cpu.returned_to != 0x1045113 { return; }
  pc = 0x1045113; continue;
 }
 0x1045113 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045114(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045114); }
 let mut pc: u32 = 0x1045114;
 loop { match pc {
 0x1045114 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x10451d2; continue; }
  pc = 0x1045121; continue;
 }
 0x1045121 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1045164; continue; }
  pc = 0x104512d; continue;
 }
 0x104512d => {
  cpu.push(0x1045132); f_010129b5(cpu); if cpu.returned_to != 0x1045132 { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x104513c); f_01012780(cpu); if cpu.returned_to != 0x104513c { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1045164; continue; }
  pc = 0x1045141; continue;
 }
 0x1045141 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x104514b); f_01012bd0(cpu); if cpu.returned_to != 0x104514b { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x1045155); f_01012780(cpu); if cpu.returned_to != 0x1045155 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1045164; continue; }
  pc = 0x104515a; continue;
 }
 0x104515a => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1045164); f_01012bd0(cpu); if cpu.returned_to != 0x1045164 { return; }
  pc = 0x1045164; continue;
 }
 0x1045164 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x1126367u32, v); }
  { let v: u16 = cpu.r16(0x1124e8au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x1045178; continue;
 }
 0x1045178 => {
  { let v: u16 = cpu.r16(0x10e3da5u32.wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x104518d; continue; }
  pc = 0x1045185; continue;
 }
 0x1045185 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if !cpu.zf { cpu.poll(); pc = 0x1045178; continue; }
  pc = 0x104518b; continue;
 }
 0x104518b => {
  pc = 0x10451d2; continue;
 }
 0x104518d => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x1045198; continue;
 }
 0x1045198 => {
  { let v: u16 = cpu.r16(0x10e3da5u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xffffu16); }
  if cpu.zf { pc = 0x10451b1; continue; }
  pc = 0x10451a6; continue;
 }
 0x10451a6 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xc1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x10451b9; continue; }
  pc = 0x10451b1; continue;
 }
 0x10451b1 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0x3u32); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1045198; continue; }
  pc = 0x10451b7; continue;
 }
 0x10451b7 => {
  pc = 0x10451d2; continue;
 }
 0x10451b9 => {
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.ebp = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10b0300u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1124e8au32, v); }
  pc = 0x10451d2; continue;
 }
 0x10451d2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010451d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10451d3); }
 let mut pc: u32 = 0x10451d3;
 loop { match pc {
 0x10451d3 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1045291; continue; }
  pc = 0x10451e0; continue;
 }
 0x10451e0 => {
  { let v: u8 = 0x3u8; cpu.w8(0x1126367u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x104522a; continue; }
  pc = 0x10451f3; continue;
 }
 0x10451f3 => {
  cpu.push(0x10451f8); f_010129b5(cpu); if cpu.returned_to != 0x10451f8 { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x1045202); f_01012780(cpu); if cpu.returned_to != 0x1045202 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104522a; continue; }
  pc = 0x1045207; continue;
 }
 0x1045207 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1045211); f_01012bd0(cpu); if cpu.returned_to != 0x1045211 { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x104521b); f_01012780(cpu); if cpu.returned_to != 0x104521b { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x104522a; continue; }
  pc = 0x1045220; continue;
 }
 0x1045220 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x104522a); f_01012bd0(cpu); if cpu.returned_to != 0x104522a { return; }
  pc = 0x104522a; continue;
 }
 0x104522a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1124e8au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x1045237; continue;
 }
 0x1045237 => {
  { let v: u16 = cpu.r16(0x10e3da5u32.wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x104524c; continue; }
  pc = 0x1045244; continue;
 }
 0x1045244 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x4u32); }
  if !cpu.zf { cpu.poll(); pc = 0x1045237; continue; }
  pc = 0x104524a; continue;
 }
 0x104524a => {
  pc = 0x1045291; continue;
 }
 0x104524c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  pc = 0x1045257; continue;
 }
 0x1045257 => {
  { let v: u16 = cpu.r16(0x10e3da5u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xffffu16); }
  if cpu.zf { pc = 0x1045270; continue; }
  pc = 0x1045265; continue;
 }
 0x1045265 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xc1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x1045278; continue; }
  pc = 0x1045270; continue;
 }
 0x1045270 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0x3u32); cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1045257; continue; }
  pc = 0x1045276; continue;
 }
 0x1045276 => {
  pc = 0x1045291; continue;
 }
 0x1045278 => {
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.ebp = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10b0300u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1124e8au32, v); }
  pc = 0x1045291; continue;
 }
 0x1045291 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045292(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045292); }
 let mut pc: u32 = 0x1045292;
 loop { match pc {
 0x1045292 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x10452f4; continue; }
  pc = 0x104529b; continue;
 }
 0x104529b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x10452de; continue; }
  pc = 0x10452a7; continue;
 }
 0x10452a7 => {
  cpu.push(0x10452ac); f_010129b5(cpu); if cpu.returned_to != 0x10452ac { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x10452b6); f_01012780(cpu); if cpu.returned_to != 0x10452b6 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10452de; continue; }
  pc = 0x10452bb; continue;
 }
 0x10452bb => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10452c5); f_01012bd0(cpu); if cpu.returned_to != 0x10452c5 { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x10452cf); f_01012780(cpu); if cpu.returned_to != 0x10452cf { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10452de; continue; }
  pc = 0x10452d4; continue;
 }
 0x10452d4 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x10452de); f_01012bd0(cpu); if cpu.returned_to != 0x10452de { return; }
  pc = 0x10452de; continue;
 }
 0x10452de => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x1126364u32, v); }
  { let v: u8 = cpu.r8(0x1124bfdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10452f4; continue; }
  pc = 0x10452f2; continue;
 }
 0x10452f2 => {
  { let v: u8 = 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10452f4; continue;
 }
 0x10452f4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124bfdu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010452fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10452fa); }
 let mut pc: u32 = 0x10452fa;
 loop { match pc {
 0x10452fa => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x104535f; continue; }
  pc = 0x1045303; continue;
 }
 0x1045303 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1045346; continue; }
  pc = 0x104530f; continue;
 }
 0x104530f => {
  cpu.push(0x1045314); f_010129b5(cpu); if cpu.returned_to != 0x1045314 { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x104531e); f_01012780(cpu); if cpu.returned_to != 0x104531e { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1045346; continue; }
  pc = 0x1045323; continue;
 }
 0x1045323 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x104532d); f_01012bd0(cpu); if cpu.returned_to != 0x104532d { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x1045337); f_01012780(cpu); if cpu.returned_to != 0x1045337 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1045346; continue; }
  pc = 0x104533c; continue;
 }
 0x104533c => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1045346); f_01012bd0(cpu); if cpu.returned_to != 0x1045346 { return; }
  pc = 0x1045346; continue;
 }
 0x1045346 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x1126364u32, v); }
  { let v: u8 = cpu.r8(0x1124bfdu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x104535f; continue; }
  pc = 0x104535a; continue;
 }
 0x104535a => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  pc = 0x104535f; continue;
 }
 0x104535f => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124bfdu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045365(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045365); }
 let mut pc: u32 = 0x1045365;
 loop { match pc {
 0x1045365 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x104537c; continue; }
  pc = 0x104536e; continue;
 }
 0x104536e => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1126364u32, v); }
  pc = 0x104537c; continue;
 }
 0x104537c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104537d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104537d); }
 let mut pc: u32 = 0x104537d;
 loop { match pc {
 0x104537d => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1045394; continue; }
  pc = 0x1045386; continue;
 }
 0x1045386 => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1126365u32, v); }
  pc = 0x1045394; continue;
 }
 0x1045394 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045395(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045395); }
 let mut pc: u32 = 0x1045395;
 loop { match pc {
 0x1045395 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x10453ac; continue; }
  pc = 0x104539e; continue;
 }
 0x104539e => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1126367u32, v); }
  pc = 0x10453ac; continue;
 }
 0x10453ac => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010453ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10453ad); }
 let mut pc: u32 = 0x10453ad;
 loop { match pc {
 0x10453ad => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x10453c4; continue; }
  pc = 0x10453b6; continue;
 }
 0x10453b6 => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1126366u32, v); }
  pc = 0x10453c4; continue;
 }
 0x10453c4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010453c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10453c5); }
 let mut pc: u32 = 0x10453c5;
 loop { match pc {
 0x10453c5 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x10453dc; continue; }
  pc = 0x10453ce; continue;
 }
 0x10453ce => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1126369u32, v); }
  pc = 0x10453dc; continue;
 }
 0x10453dc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010453dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10453dd); }
 let mut pc: u32 = 0x10453dd;
 loop { match pc {
 0x10453dd => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x10453f4; continue; }
  pc = 0x10453e6; continue;
 }
 0x10453e6 => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1126368u32, v); }
  pc = 0x10453f4; continue;
 }
 0x10453f4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010453f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10453f5); }
 let mut pc: u32 = 0x10453f5;
 loop { match pc {
 0x10453f5 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x104540e; continue; }
  pc = 0x10453fe; continue;
 }
 0x10453fe => {
  { cpu.sub8(cpu.r8(0x112636au32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x104540e; continue; }
  pc = 0x1045407; continue;
 }
 0x1045407 => {
  { let v: u8 = 0x1u8; cpu.w8(0x112636au32, v); }
  pc = 0x104540e; continue;
 }
 0x104540e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104540f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104540f); }
 let mut pc: u32 = 0x104540f;
 loop { match pc {
 0x104540f => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1045428; continue; }
  pc = 0x1045418; continue;
 }
 0x1045418 => {
  { cpu.sub8(cpu.r8(0x112636au32), 0x2u8); }
  if (cpu.sf == cpu.of) { pc = 0x1045428; continue; }
  pc = 0x1045421; continue;
 }
 0x1045421 => {
  { let v: u8 = 0x2u8; cpu.w8(0x112636au32, v); }
  pc = 0x1045428; continue;
 }
 0x1045428 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045429(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045429); }
 let mut pc: u32 = 0x1045429;
 loop { match pc {
 0x1045429 => {
  { cpu.sub8(cpu.r8(0x1128991u32), 0x0u8); }
  if cpu.zf { pc = 0x104545c; continue; }
  pc = 0x1045432; continue;
 }
 0x1045432 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1126364u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1126365u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1126366u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1126367u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1126368u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1126369u32, v); }
  pc = 0x104545c; continue;
 }
 0x104545c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104545d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104545d); }
 let mut pc: u32 = 0x104545d;
 loop { match pc {
 0x104545d => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x10454b9; continue; }
  pc = 0x1045466; continue;
 }
 0x1045466 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x10454a9; continue; }
  pc = 0x1045472; continue;
 }
 0x1045472 => {
  cpu.push(0x1045477); f_010129b5(cpu); if cpu.returned_to != 0x1045477 { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x1045481); f_01012780(cpu); if cpu.returned_to != 0x1045481 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10454a9; continue; }
  pc = 0x1045486; continue;
 }
 0x1045486 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1045490); f_01012bd0(cpu); if cpu.returned_to != 0x1045490 { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x104549a); f_01012780(cpu); if cpu.returned_to != 0x104549a { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10454a9; continue; }
  pc = 0x104549f; continue;
 }
 0x104549f => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x10454a9); f_01012bd0(cpu); if cpu.returned_to != 0x10454a9 { return; }
  pc = 0x10454a9; continue;
 }
 0x10454a9 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x1126368u32, v); }
  { let v: u8 = cpu.xor8(cpu.r8(0x112c97du32), 0x1u8); cpu.w8(0x112c97du32, v); }
  pc = 0x10454b9; continue;
 }
 0x10454b9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010454ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10454ba); }
 let mut pc: u32 = 0x10454ba;
 loop { match pc {
 0x10454ba => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1045516; continue; }
  pc = 0x10454c3; continue;
 }
 0x10454c3 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1045506; continue; }
  pc = 0x10454cf; continue;
 }
 0x10454cf => {
  cpu.push(0x10454d4); f_010129b5(cpu); if cpu.returned_to != 0x10454d4 { return; }
  { let v: u32 = 0x1fu32; cpu.eax = v; }
  cpu.push(0x10454de); f_01012780(cpu); if cpu.returned_to != 0x10454de { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1045506; continue; }
  pc = 0x10454e3; continue;
 }
 0x10454e3 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10454ed); f_01012bd0(cpu); if cpu.returned_to != 0x10454ed { return; }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  cpu.push(0x10454f7); f_01012780(cpu); if cpu.returned_to != 0x10454f7 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1045506; continue; }
  pc = 0x10454fc; continue;
 }
 0x10454fc => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1045506); f_01012bd0(cpu); if cpu.returned_to != 0x1045506 { return; }
  pc = 0x1045506; continue;
 }
 0x1045506 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x2u8; cpu.w8(0x1126368u32, v); }
  { let v: u8 = cpu.xor8(cpu.r8(0x112c97du32), 0x1u8); cpu.w8(0x112c97du32, v); }
  pc = 0x1045516; continue;
 }
 0x1045516 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045517(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045517); }
 let mut pc: u32 = 0x1045517;
 loop { match pc {
 0x1045517 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11263ccu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045527(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045527); }
 let mut pc: u32 = 0x1045527;
 loop { match pc {
 0x1045527 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x10455ab; continue; }
  pc = 0x1045530; continue;
 }
 0x1045530 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c5f7cu32, v); }
  { let v: u32 = 0x10fd38du32; cpu.w32(0x10fd55au32, v); }
  { let v: u32 = 0x10ef383u32; cpu.w32(0x10fd55eu32, v); }
  cpu.push(0x1045550); f_0105ec0c(cpu); if cpu.returned_to != 0x1045550 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11263e6u32, v); }
  { let v: u32 = 0x10ef383u32; cpu.esi = v; }
  { let v: u32 = 0xeu32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x11263edu32, v); }
  pc = 0x1045567; continue;
 }
 0x1045567 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1045576; continue; }
  pc = 0x104556d; continue;
 }
 0x104556d => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11263e6u32)); cpu.w8(0x11263e6u32, v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1045567; continue; }
  pc = 0x1045576; continue;
 }
 0x1045576 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11263e4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd431u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x11263e7u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x104559c); f_010385ad(cpu); if cpu.returned_to != 0x104559c { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x10455a4); cpu.call(t); if cpu.returned_to != 0x10455a4 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  pc = 0x10455ab; continue;
 }
 0x10455ab => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010455ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10455ac); }
 let mut pc: u32 = 0x10455ac;
 loop { match pc {
 0x10455ac => {
  { let v: u8 = cpu.r8(0x11263ebu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if !cpu.zf { pc = 0x10455e5; continue; }
  pc = 0x10455b5; continue;
 }
 0x10455b5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11263e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd431u32, v); }
  { let v: u32 = cpu.r32(0x10fd55au32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10fd55eu32); cpu.edi = v; }
  pc = 0x10455cf; continue;
 }
 0x10455cf => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10455cf; continue; }
  pc = 0x10455d9; continue;
 }
 0x10455d9 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  pc = 0x1045704; continue;
 }
 0x10455e5 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if cpu.zf { pc = 0x1045704; continue; }
  pc = 0x10455f2; continue;
 }
 0x10455f2 => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { pc = 0x104565f; continue; }
  pc = 0x10455f6; continue;
 }
 0x10455f6 => {
  { cpu.sub8(cpu.r8(0x11263e6u32), 0x0u8); }
  if cpu.zf { pc = 0x104565f; continue; }
  pc = 0x10455ff; continue;
 }
 0x10455ff => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1045642; continue; }
  pc = 0x104560b; continue;
 }
 0x104560b => {
  cpu.push(0x1045610); f_010129b5(cpu); if cpu.returned_to != 0x1045610 { return; }
  { let v: u32 = 0x21u32; cpu.eax = v; }
  cpu.push(0x104561a); f_01012780(cpu); if cpu.returned_to != 0x104561a { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1045642; continue; }
  pc = 0x104561f; continue;
 }
 0x104561f => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1045629); f_01012bd0(cpu); if cpu.returned_to != 0x1045629 { return; }
  { let v: u32 = 0x22u32; cpu.eax = v; }
  cpu.push(0x1045633); f_01012780(cpu); if cpu.returned_to != 0x1045633 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1045642; continue; }
  pc = 0x1045638; continue;
 }
 0x1045638 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1045642); f_01012bd0(cpu); if cpu.returned_to != 0x1045642 { return; }
  pc = 0x1045642; continue;
 }
 0x1045642 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11263e7u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x11263e7u32, v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x11263e6u32)); cpu.w8(0x11263e6u32, v); }
  pc = 0x1045704; continue;
 }
 0x104565f => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if !cpu.zf { pc = 0x104568c; continue; }
  pc = 0x1045663; continue;
 }
 0x1045663 => {
  { cpu.sub8(cpu.r8(0x11263e6u32), 0x0u8); }
  if cpu.zf { pc = 0x104568c; continue; }
  pc = 0x104566c; continue;
 }
 0x104566c => {
  { let v: u32 = cpu.r32(0x11263e7u32); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd431u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  pc = 0x1045704; continue;
 }
 0x104568c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x11263edu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x11263e6u32), (cpu.eax as u8)); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.sf == cpu.of) { pc = 0x1045704; continue; }
  pc = 0x104569b; continue;
 }
 0x104569b => {
  { cpu.sub8((cpu.eax as u8), 0x2cu8); }
  if cpu.zf { pc = 0x10456a7; continue; }
  pc = 0x104569f; continue;
 }
 0x104569f => {
  { cpu.sub8((cpu.eax as u8), 0x30u8); }
  if (cpu.sf != cpu.of) { pc = 0x1045704; continue; }
  pc = 0x10456a3; continue;
 }
 0x10456a3 => {
  { cpu.sub8((cpu.eax as u8), 0x39u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1045704; continue; }
  pc = 0x10456a7; continue;
 }
 0x10456a7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x10456ea; continue; }
  pc = 0x10456b3; continue;
 }
 0x10456b3 => {
  cpu.push(0x10456b8); f_010129b5(cpu); if cpu.returned_to != 0x10456b8 { return; }
  { let v: u32 = 0x35u32; cpu.eax = v; }
  cpu.push(0x10456c2); f_01012780(cpu); if cpu.returned_to != 0x10456c2 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10456ea; continue; }
  pc = 0x10456c7; continue;
 }
 0x10456c7 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10456d1); f_01012bd0(cpu); if cpu.returned_to != 0x10456d1 { return; }
  { let v: u32 = 0x36u32; cpu.eax = v; }
  cpu.push(0x10456db); f_01012780(cpu); if cpu.returned_to != 0x10456db { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10456ea; continue; }
  pc = 0x10456e0; continue;
 }
 0x10456e0 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x10456ea); f_01012bd0(cpu); if cpu.returned_to != 0x10456ea { return; }
  pc = 0x10456ea; continue;
 }
 0x10456ea => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11263e7u32); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x11263e7u32, v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x11263e6u32)); cpu.w8(0x11263e6u32, v); }
  pc = 0x1045704; continue;
 }
 0x1045704 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045705(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045705); }
 let mut pc: u32 = 0x1045705;
 loop { match pc {
 0x1045705 => {
  { cpu.sub8(cpu.r8(0x1128991u32), 0x0u8); }
  if cpu.zf { pc = 0x1045730; continue; }
  pc = 0x104570e; continue;
 }
 0x104570e => {
  { let v: u8 = 0x0u8; cpu.w8(0x112636bu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112636cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112636du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112636eu32, v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x1128991u32)); cpu.w8(0x1128991u32, v); }
  pc = 0x1045730; continue;
 }
 0x1045730 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045731(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045731); }
 let mut pc: u32 = 0x1045731;
 loop { match pc {
 0x1045731 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1045748; continue; }
  pc = 0x104573a; continue;
 }
 0x104573a => {
  { let v: u8 = 0x2u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112636bu32, v); }
  pc = 0x1045748; continue;
 }
 0x1045748 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045749(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045749); }
 let mut pc: u32 = 0x1045749;
 loop { match pc {
 0x1045749 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1045760; continue; }
  pc = 0x1045752; continue;
 }
 0x1045752 => {
  { let v: u8 = 0x2u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112636cu32, v); }
  pc = 0x1045760; continue;
 }
 0x1045760 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045761(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045761); }
 let mut pc: u32 = 0x1045761;
 loop { match pc {
 0x1045761 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1045778; continue; }
  pc = 0x104576a; continue;
 }
 0x104576a => {
  { let v: u8 = 0x2u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112636du32, v); }
  pc = 0x1045778; continue;
 }
 0x1045778 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045779(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045779); }
 let mut pc: u32 = 0x1045779;
 loop { match pc {
 0x1045779 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1045790; continue; }
  pc = 0x1045782; continue;
 }
 0x1045782 => {
  { let v: u8 = 0x2u8; cpu.w8(0x1128991u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112636eu32, v); }
  pc = 0x1045790; continue;
 }
 0x1045790 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045791(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045791); }
 let mut pc: u32 = 0x1045791;
 loop { match pc {
 0x1045791 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fd3e8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045799(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045799); }
 let mut pc: u32 = 0x1045799;
 loop { match pc {
 0x1045799 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10457a3; continue; }
  pc = 0x10457a2; continue;
 }
 0x10457a2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10457a3 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1045875; continue; }
  pc = 0x10457b0; continue;
 }
 0x10457b0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x10457f3; continue; }
  pc = 0x10457bc; continue;
 }
 0x10457bc => {
  cpu.push(0x10457c1); f_010129b5(cpu); if cpu.returned_to != 0x10457c1 { return; }
  { let v: u32 = 0x1du32; cpu.eax = v; }
  cpu.push(0x10457cb); f_01012780(cpu); if cpu.returned_to != 0x10457cb { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10457f3; continue; }
  pc = 0x10457d0; continue;
 }
 0x10457d0 => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x10457da); f_01012bd0(cpu); if cpu.returned_to != 0x10457da { return; }
  { let v: u32 = 0x1eu32; cpu.eax = v; }
  cpu.push(0x10457e4); f_01012780(cpu); if cpu.returned_to != 0x10457e4 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10457f3; continue; }
  pc = 0x10457e9; continue;
 }
 0x10457e9 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x10457f3); f_01012bd0(cpu); if cpu.returned_to != 0x10457f3 { return; }
  pc = 0x10457f3; continue;
 }
 0x10457f3 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x11c5f7cu32, v); }
  cpu.push(0x1045801); f_01044d18(cpu); if cpu.returned_to != 0x1045801 { return; }
  { let v: u32 = 0x10fd374u32; cpu.w32(0x10fd55au32, v); }
  { let v: u32 = 0x112636fu32; cpu.w32(0x10fd55eu32, v); }
  cpu.push(0x104581a); f_0105ec0c(cpu); if cpu.returned_to != 0x104581a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11263e6u32, v); }
  { let v: u32 = 0x112636fu32; cpu.esi = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x11263edu32, v); }
  pc = 0x1045831; continue;
 }
 0x1045831 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1045840; continue; }
  pc = 0x1045837; continue;
 }
 0x1045837 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11263e6u32)); cpu.w8(0x11263e6u32, v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1045831; continue; }
  pc = 0x1045840; continue;
 }
 0x1045840 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11263e4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd431u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x11263e7u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1045866); f_010385ad(cpu); if cpu.returned_to != 0x1045866 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x104586e); cpu.call(t); if cpu.returned_to != 0x104586e { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  pc = 0x1045875; continue;
 }
 0x1045875 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045876(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045876); }
 let mut pc: u32 = 0x1045876;
 loop { match pc {
 0x1045876 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1045903; continue; }
  pc = 0x1045883; continue;
 }
 0x1045883 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11c5f7cu32, v); }
  cpu.push(0x104588f); f_01045731(cpu); if cpu.returned_to != 0x104588f { return; }
  { let v: u32 = 0x10fd3a3u32; cpu.w32(0x10fd55au32, v); }
  { let v: u32 = 0x10f7acdu32; cpu.w32(0x10fd55eu32, v); }
  cpu.push(0x10458a8); f_0105ec0c(cpu); if cpu.returned_to != 0x10458a8 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11263e6u32, v); }
  { let v: u32 = 0x10f7acdu32; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x11263edu32, v); }
  pc = 0x10458bf; continue;
 }
 0x10458bf => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10458ce; continue; }
  pc = 0x10458c5; continue;
 }
 0x10458c5 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11263e6u32)); cpu.w8(0x11263e6u32, v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10458bf; continue; }
  pc = 0x10458ce; continue;
 }
 0x10458ce => {
  { let v: u8 = 0x1u8; cpu.w8(0x11263e4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd431u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x11263e7u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x10458f4); f_010385ad(cpu); if cpu.returned_to != 0x10458f4 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x10458fc); cpu.call(t); if cpu.returned_to != 0x10458fc { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  pc = 0x1045903; continue;
 }
 0x1045903 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045904(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045904); }
 let mut pc: u32 = 0x1045904;
 loop { match pc {
 0x1045904 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1045991; continue; }
  pc = 0x1045911; continue;
 }
 0x1045911 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c5f7cu32, v); }
  cpu.push(0x104591d); f_01045749(cpu); if cpu.returned_to != 0x104591d { return; }
  { let v: u32 = 0x10fd3c6u32; cpu.w32(0x10fd55au32, v); }
  { let v: u32 = 0x10f7fc8u32; cpu.w32(0x10fd55eu32, v); }
  cpu.push(0x1045936); f_0105ec0c(cpu); if cpu.returned_to != 0x1045936 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11263e6u32, v); }
  { let v: u32 = 0x10f7fc8u32; cpu.esi = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x11263edu32, v); }
  pc = 0x104594d; continue;
 }
 0x104594d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x104595c; continue; }
  pc = 0x1045953; continue;
 }
 0x1045953 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11263e6u32)); cpu.w8(0x11263e6u32, v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x104594d; continue; }
  pc = 0x104595c; continue;
 }
 0x104595c => {
  { let v: u8 = 0x2u8; cpu.w8(0x11263e4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd431u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x11263e7u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1045982); f_010385ad(cpu); if cpu.returned_to != 0x1045982 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x104598a); cpu.call(t); if cpu.returned_to != 0x104598a { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  pc = 0x1045991; continue;
 }
 0x1045991 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045992(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045992); }
 let mut pc: u32 = 0x1045992;
 loop { match pc {
 0x1045992 => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if !cpu.zf { pc = 0x1045a1f; continue; }
  pc = 0x104599f; continue;
 }
 0x104599f => {
  { let v: u8 = 0x0u8; cpu.w8(0x11c5f7cu32, v); }
  cpu.push(0x10459ab); f_01045761(cpu); if cpu.returned_to != 0x10459ab { return; }
  { let v: u32 = 0x10fd3d7u32; cpu.w32(0x10fd55au32, v); }
  { let v: u32 = 0x1124c5fu32; cpu.w32(0x10fd55eu32, v); }
  cpu.push(0x10459c4); f_0105ec0c(cpu); if cpu.returned_to != 0x10459c4 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11263e6u32, v); }
  { let v: u32 = 0x1124c5fu32; cpu.esi = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x11263edu32, v); }
  pc = 0x10459db; continue;
 }
 0x10459db => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10459ea; continue; }
  pc = 0x10459e1; continue;
 }
 0x10459e1 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11263e6u32)); cpu.w8(0x11263e6u32, v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10459db; continue; }
  pc = 0x10459ea; continue;
 }
 0x10459ea => {
  { let v: u8 = 0x3u8; cpu.w8(0x11263e4u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10fd431u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x11263e7u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v = (cpu.ecx as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = 0x3u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1045a10); f_010385ad(cpu); if cpu.returned_to != 0x1045a10 { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e433cu32); cpu.push(0x1045a18); cpu.call(t); if cpu.returned_to != 0x1045a18 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  pc = 0x1045a1f; continue;
 }
 0x1045a1f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045a20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045a20); }
 let mut pc: u32 = 0x1045a20;
 loop { match pc {
 0x1045a20 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045a21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045a21); }
 let mut pc: u32 = 0x1045a21;
 loop { match pc {
 0x1045a21 => {
  { let v: u8 = cpu.r8(0x11263ebu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x1045a2b; continue; }
  pc = 0x1045a2a; continue;
 }
 0x1045a2a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1045a2b => {
  { cpu.sub8((cpu.eax as u8), 0x1bu8); }
  if !cpu.zf { pc = 0x1045a5f; continue; }
  pc = 0x1045a2f; continue;
 }
 0x1045a2f => {
  { let v: u8 = 0x0u8; cpu.w8(0x11263e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd431u32, v); }
  { let v: u32 = cpu.r32(0x10fd55au32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10fd55eu32); cpu.edi = v; }
  pc = 0x1045a49; continue;
 }
 0x1045a49 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1045a49; continue; }
  pc = 0x1045a53; continue;
 }
 0x1045a53 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  pc = 0x1045bf1; continue;
 }
 0x1045a5f => {
  { cpu.sub8(cpu.r8(0x11263e4u32), 0x0u8); }
  if cpu.zf { pc = 0x1045bf1; continue; }
  pc = 0x1045a6c; continue;
 }
 0x1045a6c => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { pc = 0x1045ae7; continue; }
  pc = 0x1045a70; continue;
 }
 0x1045a70 => {
  { cpu.sub8(cpu.r8(0x11263e6u32), 0x0u8); }
  if cpu.zf { pc = 0x1045b24; continue; }
  pc = 0x1045a7d; continue;
 }
 0x1045a7d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1045ac0; continue; }
  pc = 0x1045a89; continue;
 }
 0x1045a89 => {
  cpu.push(0x1045a8e); f_010129b5(cpu); if cpu.returned_to != 0x1045a8e { return; }
  { let v: u32 = 0x21u32; cpu.eax = v; }
  cpu.push(0x1045a98); f_01012780(cpu); if cpu.returned_to != 0x1045a98 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1045ac0; continue; }
  pc = 0x1045a9d; continue;
 }
 0x1045a9d => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1045aa7); f_01012bd0(cpu); if cpu.returned_to != 0x1045aa7 { return; }
  { let v: u32 = 0x22u32; cpu.eax = v; }
  cpu.push(0x1045ab1); f_01012780(cpu); if cpu.returned_to != 0x1045ab1 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1045ac0; continue; }
  pc = 0x1045ab6; continue;
 }
 0x1045ab6 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1045ac0); f_01012bd0(cpu); if cpu.returned_to != 0x1045ac0 { return; }
  pc = 0x1045ac0; continue;
 }
 0x1045ac0 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11263e7u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0xffffffffu32.wrapping_add(cpu.edi)), 0x5eu8); }
  if !cpu.zf { pc = 0x1045ad6; continue; }
  pc = 0x1045ad2; continue;
 }
 0x1045ad2 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1045ad6; continue;
 }
 0x1045ad6 => {
  { let v: u32 = cpu.edi; cpu.w32(0x11263e7u32, v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x11263e6u32)); cpu.w8(0x11263e6u32, v); }
  pc = 0x1045bf1; continue;
 }
 0x1045ae7 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if !cpu.zf { pc = 0x1045b24; continue; }
  pc = 0x1045aeb; continue;
 }
 0x1045aeb => {
  { cpu.sub8(cpu.r8(0x11c5f7cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1045b01; continue; }
  pc = 0x1045af4; continue;
 }
 0x1045af4 => {
  { cpu.sub8(cpu.r8(0x11263e6u32), 0x0u8); }
  if cpu.zf { pc = 0x1045bf1; continue; }
  pc = 0x1045b01; continue;
 }
 0x1045b01 => {
  { let v: u32 = cpu.r32(0x11263e7u32); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11263e4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fd431u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ee974u32, v); }
  pc = 0x1045bf1; continue;
 }
 0x1045b24 => {
  { let v: u8 = cpu.r8(0x11263edu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x11263e6u32), (cpu.ebx as u8)); }
  if (cpu.sf == cpu.of) { pc = 0x1045bf1; continue; }
  pc = 0x1045b36; continue;
 }
 0x1045b36 => {
  cpu.push(0x1045b3b); f_01045bf2(cpu); if cpu.returned_to != 0x1045b3b { return; }
  if cpu.cf { pc = 0x1045b57; continue; }
  pc = 0x1045b3d; continue;
 }
 0x1045b3d => {
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if (cpu.sf != cpu.of) { pc = 0x1045b47; continue; }
  pc = 0x1045b41; continue;
 }
 0x1045b41 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1045b47; continue; }
  pc = 0x1045b45; continue;
 }
 0x1045b45 => {
  pc = 0x1045b57; continue;
 }
 0x1045b47 => {
  { cpu.sub8((cpu.eax as u8), 0x30u8); }
  if (cpu.sf != cpu.of) { pc = 0x1045bf1; continue; }
  pc = 0x1045b4f; continue;
 }
 0x1045b4f => {
  { cpu.sub8((cpu.eax as u8), 0x39u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1045bf1; continue; }
  pc = 0x1045b57; continue;
 }
 0x1045b57 => {
  { let v: u32 = cpu.r32(0x11263e7u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x10b00ebu32.wrapping_add(cpu.edi)), 0x0u8); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x1045b78; continue; }
  pc = 0x1045b6e; continue;
 }
 0x1045b6e => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), 0x60u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x1045bf1; continue; }
  pc = 0x1045b72; continue;
 }
 0x1045b72 => {
  { cpu.sub8((cpu.eax as u8), 0x1au8); }
  if (cpu.sf == cpu.of) { pc = 0x1045bf1; continue; }
  pc = 0x1045b76; continue;
 }
 0x1045b76 => {
  pc = 0x1045b9a; continue;
 }
 0x1045b78 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4158u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10b00ecu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x10b00edu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x1045b90; continue; }
  pc = 0x1045b90; continue;
 }
 0x1045b90 => {
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if (cpu.sf != cpu.of) { pc = 0x1045b9a; continue; }
  pc = 0x1045b94; continue;
 }
 0x1045b94 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1045b9a; continue; }
  pc = 0x1045b98; continue;
 }
 0x1045b98 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1045b9a; continue;
 }
 0x1045b9a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11b51b2u32), 0x0u16); }
  if cpu.zf { pc = 0x1045bdf; continue; }
  pc = 0x1045ba8; continue;
 }
 0x1045ba8 => {
  cpu.push(0x1045bad); f_010129b5(cpu); if cpu.returned_to != 0x1045bad { return; }
  { let v: u32 = 0x35u32; cpu.eax = v; }
  cpu.push(0x1045bb7); f_01012780(cpu); if cpu.returned_to != 0x1045bb7 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1045bdf; continue; }
  pc = 0x1045bbc; continue;
 }
 0x1045bbc => {
  { let v: u32 = 0x0u32; cpu.edx = v; }
  cpu.push(0x1045bc6); f_01012bd0(cpu); if cpu.returned_to != 0x1045bc6 { return; }
  { let v: u32 = 0x36u32; cpu.eax = v; }
  cpu.push(0x1045bd0); f_01012780(cpu); if cpu.returned_to != 0x1045bd0 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x1045bdf; continue; }
  pc = 0x1045bd5; continue;
 }
 0x1045bd5 => {
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.push(0x1045bdf); f_01012bd0(cpu); if cpu.returned_to != 0x1045bdf { return; }
  pc = 0x1045bdf; continue;
 }
 0x1045bdf => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x11263e7u32, v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x11263e6u32)); cpu.w8(0x11263e6u32, v); }
  pc = 0x1045bf1; continue;
 }
 0x1045bf1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045bf2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045bf2); }
 let mut pc: u32 = 0x1045bf2;
 loop { match pc {
 0x1045bf2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x11c5f7du32; cpu.esi = v; }
  pc = 0x1045bf8; continue;
 }
 0x1045bf8 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x1045c05; continue; }
  pc = 0x1045bfc; continue;
 }
 0x1045bfc => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x1045bf8; continue; }
  pc = 0x1045c02; continue;
 }
 0x1045c02 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1045c05 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045c08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045c08); }
 let mut pc: u32 = 0x1045c08;
 loop { match pc {
 0x1045c08 => {
  { let v: u32 = 0x10d0u32; cpu.ebx = v; }
  cpu.push(0x1045c12); f_0105dce8(cpu); if cpu.returned_to != 0x1045c12 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef31eu32, v); }
  { cpu.sub8(cpu.r8(0x11c632eu32), 0x0u8); }
  if cpu.zf { pc = 0x1045c3b; continue; }
  pc = 0x1045c22; continue;
 }
 0x1045c22 => {
  { let v: u32 = cpu.r32(0x10e4024u32); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x8u32; cpu.ebp = v; }
  cpu.push(0x1045c33); f_01033bea(cpu); if cpu.returned_to != 0x1045c33 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x1045c54; continue; }
  pc = 0x1045c36; continue;
 }
 0x1045c36 => {
  f_0105f11c(cpu); return;
 }
 0x1045c3b => {
  { let v: u32 = cpu.r32(0x10e4024u32); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x1045c4c); f_01033bea(cpu); if cpu.returned_to != 0x1045c4c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x1045c54; continue; }
  pc = 0x1045c4f; continue;
 }
 0x1045c4f => {
  f_0105f11c(cpu); return;
 }
 0x1045c54 => {
  { let v: u32 = cpu.r32(0x10e4024u32); cpu.esi = v; }
  { let v: u32 = 0x11a3af0u32; cpu.edi = v; }
  cpu.push(0x1045c64); f_0106f630(cpu); if cpu.returned_to != 0x1045c64 { return; }
  if !cpu.cf { pc = 0x1045c6b; continue; }
  pc = 0x1045c66; continue;
 }
 0x1045c66 => {
  f_0105f11c(cpu); return;
 }
 0x1045c6b => {
  cpu.push(0x1045c70); f_0105dd62(cpu); if cpu.returned_to != 0x1045c70 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045c71(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045c71); }
 let mut pc: u32 = 0x1045c71;
 loop { match pc {
 0x1045c71 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = 0x10ef397u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u16 = 0x10u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1045c8a; continue;
 }
 0x1045c8a => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { cpu.sub8((cpu.edx as u8), 0x9u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1045c9a; continue; }
  pc = 0x1045c95; continue;
 }
 0x1045c95 => {
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x37u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1045c9d; continue;
 }
 0x1045c9a => {
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1045c9d; continue;
 }
 0x1045c9d => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1045c8a; continue; }
  pc = 0x1045ca2; continue;
 }
 0x1045ca2 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045e4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045e4d); }
 let mut pc: u32 = 0x1045e4d;
 loop { match pc {
 0x1045e4d => {
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { let v: u32 = 0x11dc39cu32; cpu.edi = v; }
  { let v: u8 = cpu.sub8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x1045e5e); f_01037bbd(cpu); if cpu.returned_to != 0x1045e5e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045e5f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045e5f); }
 let mut pc: u32 = 0x1045e5f;
 loop { match pc {
 0x1045e5f => {
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { let v: u32 = 0x11dc39cu32; cpu.edi = v; }
  { let v: u8 = cpu.sub8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x1045e70); f_01037bbd(cpu); if cpu.returned_to != 0x1045e70 { return; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.pushad(); }
  { let v: u32 = 0x3ffu32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10fd546u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x1045e8b; continue; }
  pc = 0x1045e81; continue;
 }
 0x1045e81 => {
  { let v: u32 = 0x11dc39cu32; cpu.eax = v; }
  cpu.push(0x1045e8b); f_01045eca(cpu); if cpu.returned_to != 0x1045e8b { return; }
  pc = 0x1045e8b; continue;
 }
 0x1045e8b => {
  { let v: u32 = cpu.r32(0x10fd54au32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x1045e9f; continue; }
  pc = 0x1045e95; continue;
 }
 0x1045e95 => {
  { let v: u32 = 0x11dc91cu32; cpu.eax = v; }
  cpu.push(0x1045e9f); f_01045eca(cpu); if cpu.returned_to != 0x1045e9f { return; }
  pc = 0x1045e9f; continue;
 }
 0x1045e9f => {
  { let v: u32 = cpu.r32(0x10fd54eu32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x1045eb3; continue; }
  pc = 0x1045ea9; continue;
 }
 0x1045ea9 => {
  { let v: u32 = 0x11dce9cu32; cpu.eax = v; }
  cpu.push(0x1045eb3); f_01045eca(cpu); if cpu.returned_to != 0x1045eb3 { return; }
  pc = 0x1045eb3; continue;
 }
 0x1045eb3 => {
  { let v: u32 = cpu.r32(0x10fd552u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x1045ec7; continue; }
  pc = 0x1045ebd; continue;
 }
 0x1045ebd => {
  { let v: u32 = 0x11dd41cu32; cpu.eax = v; }
  cpu.push(0x1045ec7); f_01045eca(cpu); if cpu.returned_to != 0x1045ec7 { return; }
  pc = 0x1045ec7; continue;
 }
 0x1045ec7 => {
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045eca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045eca); }
 let mut pc: u32 = 0x1045eca;
 loop { match pc {
 0x1045eca => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  pc = 0x1045ece; continue;
 }
 0x1045ece => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x1045edc; continue; }
  pc = 0x1045ed2; continue;
 }
 0x1045ed2 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edi & cpu.ebp; cpu.edi = v; }
  cpu.poll(); pc = 0x1045ece; continue;
 }
 0x1045edc => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045edf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045edf); }
 let mut pc: u32 = 0x1045edf;
 loop { match pc {
 0x1045edf => {
  { let v: u16 = 0xc8u16; cpu.w16(0x10e4108u32, v); }
  { let v: u16 = 0x100u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1045ef1); f_010725e4(cpu); if cpu.returned_to != 0x1045ef1 { return; }
  cpu.push(0x1045ef6); f_0100abd9(cpu); if cpu.returned_to != 0x1045ef6 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee974u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ee976u32, v); }
  let t = cpu.r32(0x10e434cu32); cpu.push(0x1045f0a); cpu.call(t); if cpu.returned_to != 0x1045f0a { return; }
  cpu.push(0x1045f0f); f_0105ec4c(cpu); if cpu.returned_to != 0x1045f0f { return; }
  cpu.push(0x1045f14); f_01072682(cpu); if cpu.returned_to != 0x1045f14 { return; }
  { cpu.sub8(cpu.r8(0x10ef36fu32), 0xffu8); }
  if !cpu.zf { pc = 0x1045f28; continue; }
  pc = 0x1045f1d; continue;
 }
 0x1045f1d => {
  { cpu.push(cpu.segments[1] as u32); }
  cpu.push(0x1045f23); f_010aec25(cpu); if cpu.returned_to != 0x1045f23 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef36fu32, v); }
  pc = 0x1045f28; continue;
 }
 0x1045f28 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0xffu8); }
  if !cpu.zf { pc = 0x1045f36; continue; }
  pc = 0x1045f31; continue;
 }
 0x1045f31 => {
  cpu.push(0x1045f36); f_0100ae73(cpu); if cpu.returned_to != 0x1045f36 { return; }
  pc = 0x1045f36; continue;
 }
 0x1045f36 => {
  cpu.push(0x1045f3b); f_0100aecf(cpu); if cpu.returned_to != 0x1045f3b { return; }
  { cpu.sub16(cpu.r16(0x10e3e63u32), 0x0u16); }
  if cpu.zf { pc = 0x1045f5d; continue; }
  pc = 0x1045f45; continue;
 }
 0x1045f45 => {
  { let v: u32 = 0x112896bu32; cpu.eax = v; }
  { let v: u32 = 0x3a98u32; cpu.ebx = v; }
  cpu.push(0x1045f54); f_0105dc26(cpu); if cpu.returned_to != 0x1045f54 { return; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e63u32, v); }
  pc = 0x1045f5d; continue;
 }
 0x1045f5d => {
  { let v: u32 = 0x1128985u32; cpu.edx = v; }
  cpu.push(0x1045f67); f_01045c08(cpu); if cpu.returned_to != 0x1045f67 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x1128db6u32, v); }
  cpu.push(0x1045f73); f_0105996f(cpu); if cpu.returned_to != 0x1045f73 { return; }
  cpu.push(0x1045f78); f_0104408f(cpu); if cpu.returned_to != 0x1045f78 { return; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e4348u32); cpu.push(0x1045f81); cpu.call(t); if cpu.returned_to != 0x1045f81 { return; }
  cpu.push(0x1045f86); f_0107162e(cpu); if cpu.returned_to != 0x1045f86 { return; }
  { let v: u16 = 0xad8u16; cpu.w16(0x10e3db7u32, v); }
  { let v: u32 = 0x11a3af0u32; cpu.esi = v; }
  { let v: u16 = 0x300u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x1045f9d); f_0107259c(cpu); if cpu.returned_to != 0x1045f9d { return; }
  { let v: u32 = 0x112825cu32; cpu.esi = v; }
  cpu.push(0x1045fa7); f_01059ad5(cpu); if cpu.returned_to != 0x1045fa7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045fa8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045fa8); }
 let mut pc: u32 = 0x1045fa8;
 loop { match pc {
 0x1045fa8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045fb1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045fb1); }
 let mut pc: u32 = 0x1045fb1;
 loop { match pc {
 0x1045fb1 => {
  { let v: u32 = 0x3020100u32; cpu.w32(0x10ee948u32, v); }
  { let v: u32 = cpu.r32(0x1105c62u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1105c91u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee91du32, v); }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e40ecu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01045fd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1045fd8); }
 let mut pc: u32 = 0x1045fd8;
 loop { match pc {
 0x1045fd8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046293(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046293); }
 let mut pc: u32 = 0x1046293;
 loop { match pc {
 0x1046293 => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x10462a5; continue; }
  pc = 0x104629c; continue;
 }
 0x104629c => {
  { cpu.sub8(cpu.r8(0x10e3d7fu32), 0x2u8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10462f3; continue; }
  pc = 0x10462a5; continue;
 }
 0x10462a5 => {
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if cpu.zf { pc = 0x10462cc; continue; }
  pc = 0x10462bb; continue;
 }
 0x10462bb => {
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x3cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10462cc; continue;
 }
 0x10462cc => {
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10462dd; continue; }
  pc = 0x10462d5; continue;
 }
 0x10462d5 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x64u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10462dd; continue;
 }
 0x10462dd => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x40u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x40u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3f60u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3f64u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10462f3 => {
  { let v: u32 = cpu.r8(0x10ee91du32) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3d1du32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1bu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x10ee91du32), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x10462cc; continue; }
  pc = 0x1046313; continue;
 }
 0x1046313 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x3cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x10462cc; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046319(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046319); }
 let mut pc: u32 = 0x1046319;
 loop { match pc {
 0x1046319 => {
  { let v: u32 = 0x3020100u32; cpu.w32(0x10ee950u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10ee950u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ee948u32, v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x1046342; continue; }
  pc = 0x1046339; continue;
 }
 0x1046339 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee949u32, v); }
  pc = 0x1046349; continue;
 }
 0x1046342 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ee949u32, v); }
  pc = 0x1046349; continue;
 }
 0x1046349 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104634a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104634a); }
 let mut pc: u32 = 0x104634a;
 loop { match pc {
 0x104634a => {
  { let v: u32 = 0x10ef212u32; cpu.edi = v; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046359(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046359); }
 let mut pc: u32 = 0x1046359;
 loop { match pc {
 0x1046359 => {
  { let v: u32 = 0x11a37f0u32; cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x100u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x10u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x17u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x10); }
  cpu.push(0x1046370); f_010724a7(cpu); if cpu.returned_to != 0x1046370 { return; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x1046379); f_010725e4(cpu); if cpu.returned_to != 0x1046379 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046423(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046423); }
 let mut pc: u32 = 0x1046423;
 loop { match pc {
 0x1046423 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = 0x10c23e9u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x78u32; cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  f_01046448(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046448(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046448); }
 let mut pc: u32 = 0x1046448;
 loop { match pc {
 0x1046448 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046518(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046518); }
 let mut pc: u32 = 0x1046518;
 loop { match pc {
 0x1046518 => {
  { let v: u32 = 0x10ee98bu32; cpu.esi = v; }
  { let v: u32 = 0x10eea7bu32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1046527; continue;
 }
 0x1046527 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x1046548; continue; }
  pc = 0x104652c; continue;
 }
 0x104652c => {
  { let v: u8 = 0x52u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1129f80u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x104653e); f_010465c3(cpu); if cpu.returned_to != 0x104653e { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x1046546); f_01046560(cpu); if cpu.returned_to != 0x1046546 { return; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1046548; continue;
 }
 0x1046548 => {
  { cpu.stos(1, false); }
  { let v: u8 = 0x0u8; cpu.w8(0x77u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xefu32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x104655d; continue; }
  pc = 0x1046558; continue;
 }
 0x1046558 => {
  cpu.push(0x104655d); f_01046588(cpu); if cpu.returned_to != 0x104655d { return; }
  pc = 0x104655d; continue;
 }
 0x104655d => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1046527; continue; }
  pc = 0x104655f; continue;
 }
 0x104655f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046560(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046560); }
 let mut pc: u32 = 0x1046560;
 loop { match pc {
 0x1046560 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x10eea7bu32); cpu.edi = v; }
  { let v: u32 = 0x10eed4bu32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.edi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046588(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046588); }
 let mut pc: u32 = 0x1046588;
 loop { match pc {
 0x1046588 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edi.wrapping_sub(0x10eea7cu32); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x1129f80u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x466cu32); cpu.esi = v; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_sub((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x10465ac; continue;
 }
 0x10465ac => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  if cpu.zf { pc = 0x10465b6; continue; }
  pc = 0x10465b4; continue;
 }
 0x10465b4 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x10465b6; continue;
 }
 0x10465b6 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10465ac; continue; }
  pc = 0x10465b8; continue;
 }
 0x10465b8 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x10eec5bu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010465c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10465c3); }
 let mut pc: u32 = 0x10465c3;
 loop { match pc {
 0x10465c3 => {
  { let v: u32 = cpu.r16(0x1129f80u32) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x466cu32); cpu.esi = v; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0xffffffe4u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1046621; continue; }
  pc = 0x10465e3; continue;
 }
 0x10465e3 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x104661c; continue; }
  pc = 0x10465e9; continue;
 }
 0x10465e9 => {
  { let v: u16 = 0x2cu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.mul16(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x104661c; continue; }
  pc = 0x1046600; continue;
 }
 0x1046600 => {
  { let v: u8 = 0x2au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x55f0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), (cpu.eax as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x104661c; continue; }
  pc = 0x1046618; continue;
 }
 0x1046618 => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  pc = 0x104661c; continue;
 }
 0x104661c => {
  { let v: u32 = cpu.add32(cpu.esi, 0x2u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10465e3; continue; }
  pc = 0x1046621; continue;
 }
 0x1046621 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046622(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046622); }
 let mut pc: u32 = 0x1046622;
 loop { match pc {
 0x1046622 => {
  { let v: u32 = cpu.r32(0x10e3f94u32); cpu.edi = v; }
  { let v: u32 = 0x186au32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x10e3f94u32); cpu.ebx = v; }
  { let v: u32 = 0x10f7c2cu32; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.ebp = v; }
  cpu.push(0x104664c); f_01033bea(cpu); if cpu.returned_to != 0x104664c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef36cu32, v); }
  cpu.push(0x1046659); f_01036a3d(cpu); if cpu.returned_to != 0x1046659 { return; }
  cpu.push(0x104665e); f_01084148(cpu); if cpu.returned_to != 0x104665e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104665f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104665f); }
 let mut pc: u32 = 0x104665f;
 loop { match pc {
 0x104665f => {
  { let v: u32 = cpu.eax; cpu.w32(0x10fcdb6u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10fcdbau32, v); }
  { let v = cpu.r16(0x10e40fcu32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e4104u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e4100u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x10e4108u32); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e40fcu32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10fcd27u32)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x140u16); }
  if (cpu.sf != cpu.of) { pc = 0x104669e; continue; }
  pc = 0x104669a; continue;
 }
 0x104669a => {
  { let v: u16 = 0x140u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x104669e; continue;
 }
 0x104669e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e4104u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e4100u32, v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10fcd2bu32)); cpu.ebx = v; }
  { cpu.sub16((cpu.ebx as u16), 0xc8u16); }
  if (cpu.sf != cpu.of) { pc = 0x10466bc; continue; }
  pc = 0x10466b8; continue;
 }
 0x10466b8 => {
  { let v: u16 = 0xc8u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10466bc; continue;
 }
 0x10466bc => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e4108u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  pc = 0x10466c6; continue;
 }
 0x10466c6 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub16(cpu.r16(0x112d789u32), 0x54u16); }
  if !cpu.zf { pc = 0x10466e2; continue; }
  pc = 0x10466d5; continue;
 }
 0x10466d5 => {
  cpu.push(0x10466da); f_0104cb88(cpu); if cpu.returned_to != 0x10466da { return; }
  if cpu.cf { pc = 0x1046801; continue; }
  pc = 0x10466e0; continue;
 }
 0x10466e0 => {
  pc = 0x10466ed; continue;
 }
 0x10466e2 => {
  cpu.push(0x10466e7); f_0104d2d7(cpu); if cpu.returned_to != 0x10466e7 { return; }
  if cpu.cf { pc = 0x1046801; continue; }
  pc = 0x10466ed; continue;
 }
 0x10466ed => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x10467d1; continue; }
  pc = 0x10466f5; continue;
 }
 0x10466f5 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10fcc83u32, v); }
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x104670a; continue; }
  pc = 0x1046700; continue;
 }
 0x1046700 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x1046797; continue;
 }
 0x104670a => {
  { let v: u8 = 0xffu8; cpu.w8(0x11c5f90u32, v); }
  { cpu.sub8((cpu.edx as u8), 0x19u8); }
  if (cpu.sf == cpu.of) { pc = 0x104674a; continue; }
  pc = 0x1046716; continue;
 }
 0x1046716 => {
  { cpu.pushad(); }
  { let v: u32 = 0x459u32; cpu.ecx = v; }
  { cpu.sub16(cpu.r16(0x112d789u32), 0x54u16); }
  if cpu.zf { pc = 0x104672b; continue; }
  pc = 0x1046726; continue;
 }
 0x1046726 => {
  { let v: u32 = 0x44du32; cpu.ecx = v; }
  pc = 0x104672b; continue;
 }
 0x104672b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1046737); cpu.call(t); if cpu.returned_to != 0x1046737 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x6u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x40u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x11c5f90u32, v); }
  pc = 0x104674a; continue;
 }
 0x104674a => {
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if (cpu.sf != cpu.of) { pc = 0x1046797; continue; }
  pc = 0x104674f; continue;
 }
 0x104674f => {
  { cpu.sub8((cpu.edx as u8), 0x7au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1046768; continue; }
  pc = 0x1046754; continue;
 }
 0x1046754 => {
  { cpu.sub8((cpu.edx as u8), 0x61u8); }
  if (cpu.sf == cpu.of) { pc = 0x104676f; continue; }
  pc = 0x1046759; continue;
 }
 0x1046759 => {
  { cpu.sub8((cpu.edx as u8), 0x5au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1046768; continue; }
  pc = 0x104675e; continue;
 }
 0x104675e => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x21u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x33eu32; cpu.ebp = v; }
  pc = 0x1046777; continue;
 }
 0x1046768 => {
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x1046797; continue;
 }
 0x104676f => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x61u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x378u32; cpu.ebp = v; }
  pc = 0x1046777; continue;
 }
 0x1046777 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x112d789u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104678d); cpu.call(t); if cpu.returned_to != 0x104678d { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  pc = 0x1046797; continue;
 }
 0x1046797 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10fcc83u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x11c5f90u32), 0xffu8); }
  if cpu.zf { pc = 0x10467ae; continue; }
  pc = 0x10467a8; continue;
 }
 0x10467a8 => {
  { let v: u8 = cpu.r8(0x11c5f90u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10467ae; continue;
 }
 0x10467ae => {
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x10467c1; continue; }
  pc = 0x10467b3; continue;
 }
 0x10467b3 => {
  { cpu.sub8((cpu.eax as u8), 0x7fu8); }
  if (cpu.sf == cpu.of) { pc = 0x10467c1; continue; }
  pc = 0x10467b7; continue;
 }
 0x10467b7 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10fcc84u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x10467c1; continue;
 }
 0x10467c1 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10fcdb6u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10fcdbau32, v); }
  cpu.poll(); pc = 0x10466c6; continue;
 }
 0x10467d1 => {
  { cpu.popad(); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e4108u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e4100u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e4104u32, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x10e40fcu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e40fcu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e4100u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1046801 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x11bce99u32)); cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1046812); cpu.call(t); if cpu.returned_to != 0x1046812 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11c59bdu32)); cpu.eax = v; }
  cpu.poll(); pc = 0x10467c1; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104681f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104681f); }
 let mut pc: u32 = 0x104681f;
 loop { match pc {
 0x104681f => {
  { let v: u16 = 0x0u16; cpu.w16(0x112d789u32, v); }
  cpu.push(0x104682d); f_0104665f(cpu); if cpu.returned_to != 0x104682d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104682e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104682e); }
 let mut pc: u32 = 0x104682e;
 loop { match pc {
 0x104682e => {
  { let v: u16 = 0x54u16; cpu.w16(0x112d789u32, v); }
  cpu.push(0x104683c); f_0104665f(cpu); if cpu.returned_to != 0x104683c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104683d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104683d); }
 let mut pc: u32 = 0x104683d;
 loop { match pc {
 0x104683d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1046842; continue;
 }
 0x1046842 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.pushad(); }
  cpu.push(0x104684d); f_0104cb88(cpu); if cpu.returned_to != 0x104684d { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x1046885; continue; }
  pc = 0x1046850; continue;
 }
 0x1046850 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1046881; continue; }
  pc = 0x1046854; continue;
 }
 0x1046854 => {
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x1046867; continue; }
  pc = 0x1046859; continue;
 }
 0x1046859 => {
  { cpu.sub8((cpu.eax as u8), 0x7fu8); }
  if (cpu.sf == cpu.of) { pc = 0x104687f; continue; }
  pc = 0x104685d; continue;
 }
 0x104685d => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10fcc84u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x104687d; continue;
 }
 0x1046867 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x19u8); }
  if (cpu.sf == cpu.of) { pc = 0x104687f; continue; }
  pc = 0x104686f; continue;
 }
 0x104686f => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x40u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x10fcc84u32.wrapping_add(cpu.edx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x104687d; continue;
 }
 0x104687d => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x104687f; continue;
 }
 0x104687f => {
  cpu.poll(); pc = 0x1046842; continue;
 }
 0x1046881 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1046885 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11c59bdu32)); cpu.eax = v; }
  cpu.poll(); pc = 0x104687d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104688d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104688d); }
 let mut pc: u32 = 0x104688d;
 loop { match pc {
 0x104688d => {
  { let v: u32 = cpu.eax; cpu.w32(0x10fd41cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10fd420u32, v); }
  { cpu.pushad(); }
  pc = 0x1046899; continue;
 }
 0x1046899 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x10468a3); f_0104d2d7(cpu); if cpu.returned_to != 0x10468a3 { return; }
  if cpu.cf { pc = 0x104694a; continue; }
  pc = 0x10468a9; continue;
 }
 0x10468a9 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1046936; continue; }
  pc = 0x10468b1; continue;
 }
 0x10468b1 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10fd42cu32, v); }
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x10468c3; continue; }
  pc = 0x10468bc; continue;
 }
 0x10468bc => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x104690b; continue;
 }
 0x10468c3 => {
  { cpu.sub8((cpu.edx as u8), 0x7au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10468dc; continue; }
  pc = 0x10468c8; continue;
 }
 0x10468c8 => {
  { cpu.sub8((cpu.edx as u8), 0x61u8); }
  if (cpu.sf == cpu.of) { pc = 0x10468e3; continue; }
  pc = 0x10468cd; continue;
 }
 0x10468cd => {
  { cpu.sub8((cpu.edx as u8), 0x5au8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10468dc; continue; }
  pc = 0x10468d2; continue;
 }
 0x10468d2 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x21u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x33eu32; cpu.ebp = v; }
  pc = 0x10468eb; continue;
 }
 0x10468dc => {
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  pc = 0x104690b; continue;
 }
 0x10468e3 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x61u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x378u32; cpu.ebp = v; }
  pc = 0x10468eb; continue;
 }
 0x10468eb => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x10fd42du32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1046901); cpu.call(t); if cpu.returned_to != 0x1046901 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  pc = 0x104690b; continue;
 }
 0x104690b => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10fd42cu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x1046926; continue; }
  pc = 0x1046918; continue;
 }
 0x1046918 => {
  { cpu.sub8((cpu.eax as u8), 0x7fu8); }
  if (cpu.sf == cpu.of) { pc = 0x1046926; continue; }
  pc = 0x104691c; continue;
 }
 0x104691c => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10fcc84u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  pc = 0x1046926; continue;
 }
 0x1046926 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10fd41cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10fd420u32, v); }
  cpu.poll(); pc = 0x1046899; continue;
 }
 0x1046936 => {
  { cpu.popad(); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e40fcu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e4100u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104694a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x11bce99u32)); cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x104695b); cpu.call(t); if cpu.returned_to != 0x104695b { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11c59bdu32)); cpu.eax = v; }
  cpu.poll(); pc = 0x1046926; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046968(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046968); }
 let mut pc: u32 = 0x1046968;
 loop { match pc {
 0x1046968 => {
  { let v = cpu.r32(0x11c59bdu32); cpu.push(v); }
  { let v = cpu.r32(0x11bce99u32); cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10fd42du32, v); }
  cpu.push(0x1046982); f_0104688d(cpu); if cpu.returned_to != 0x1046982 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x11bce99u32, v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x11c59bdu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104698f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104698f); }
 let mut pc: u32 = 0x104698f;
 loop { match pc {
 0x104698f => {
  { let v: u32 = cpu.ecx; cpu.w32(0x10fd3ebu32, v); }
  pc = 0x1046995; continue;
 }
 0x1046995 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x104699d; continue; }
  pc = 0x104699a; continue;
 }
 0x104699a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x1046995; continue;
 }
 0x104699d => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  cpu.push(0x10469a8); f_0104cb88(cpu); if cpu.returned_to != 0x10469a8 { return; }
  { cpu.popad(); }
  if cpu.cf { pc = 0x10469de; continue; }
  pc = 0x10469ab; continue;
 }
 0x10469ab => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10469dc; continue; }
  pc = 0x10469af; continue;
 }
 0x10469af => {
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x10469c2; continue; }
  pc = 0x10469b4; continue;
 }
 0x10469b4 => {
  { cpu.sub8((cpu.eax as u8), 0x7fu8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x104699d; continue; }
  pc = 0x10469b8; continue;
 }
 0x10469b8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10fcc84u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10469d8; continue;
 }
 0x10469c2 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x19u8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x104699d; continue; }
  pc = 0x10469ca; continue;
 }
 0x10469ca => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x40u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x10fcc84u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10469d8; continue;
 }
 0x10469d8 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  if !cpu.sf { cpu.poll(); pc = 0x104699d; continue; }
  pc = 0x10469dc; continue;
 }
 0x10469dc => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10469de => {
  { let v: u32 = cpu.r32(0x11c59bdu32); cpu.eax = v; }
  cpu.poll(); pc = 0x10469d8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010469e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10469e5); }
 let mut pc: u32 = 0x10469e5;
 loop { match pc {
 0x10469e5 => {
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if cpu.zf { pc = 0x1046a5a; continue; }
  pc = 0x10469ee; continue;
 }
 0x10469ee => {
  { cpu.sub8(cpu.r8(0x112c97cu32), 0x2u8); }
  if (cpu.sf != cpu.of) { pc = 0x1046a23; continue; }
  pc = 0x10469f7; continue;
 }
 0x10469f7 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x11263a2u32)); cpu.w8(0x11263a2u32, v); }
  if !cpu.zf { pc = 0x1046a23; continue; }
  pc = 0x10469ff; continue;
 }
 0x10469ff => {
  { let v: u8 = 0x1eu8; cpu.w8(0x11263a2u32, v); }
  { cpu.pushad(); }
  { let v: u32 = 0x10c68b6u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x30u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  cpu.push(0x1046a22); f_01037d64(cpu); if cpu.returned_to != 0x1046a22 { return; }
  { cpu.popad(); }
  pc = 0x1046a23; continue;
 }
 0x1046a23 => {
  { let v: u8 = 0x30u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1046a27; continue;
 }
 0x1046a27 => {
  { let v: u32 = 0x10c68adu32; cpu.esi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x6u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1046a35); f_01045e5f(cpu); if cpu.returned_to != 0x1046a35 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x1124ee4u32), 0x0u8); }
  if cpu.zf { pc = 0x1046a43; continue; }
  pc = 0x1046a3f; continue;
 }
 0x1046a3f => {
  { let v: u8 = (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.poll(); pc = 0x1046a27; continue;
 }
 0x1046a43 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x36u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1046a27; continue; }
  pc = 0x1046a4a; continue;
 }
 0x1046a4a => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x0u8); }
  if !cpu.sf { pc = 0x1046b14; continue; }
  pc = 0x1046a55; continue;
 }
 0x1046a55 => {
  pc = 0x1046b1d; continue;
 }
 0x1046a5a => {
  { cpu.sub8(cpu.r8(0x112c97cu32), 0x2u8); }
  if (cpu.sf != cpu.of) { pc = 0x1046ad8; continue; }
  pc = 0x1046a63; continue;
 }
 0x1046a63 => {
  { cpu.sub32(cpu.r32(0x112894cu32), 0xffffffffu32); }
  if cpu.zf { pc = 0x1046ad8; continue; }
  pc = 0x1046a6c; continue;
 }
 0x1046a6c => {
  { cpu.pushad(); }
  { let v: u32 = 0x10c68b6u32; cpu.esi = v; }
  { let v: u8 = 0x31u8; cpu.w8(0x6u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1046a7b); f_01045e5f(cpu); if cpu.returned_to != 0x1046a7b { return; }
  { cpu.sub8(cpu.r8(0x1124ee4u32), 0x0u8); }
  if cpu.zf { pc = 0x1046a8b; continue; }
  pc = 0x1046a84; continue;
 }
 0x1046a84 => {
  { let v: u8 = 0xffu8; cpu.w8(0x112894du32, v); }
  pc = 0x1046a8b; continue;
 }
 0x1046a8b => {
  { let v: u32 = 0x10c68b6u32; cpu.esi = v; }
  { let v: u8 = 0x32u8; cpu.w8(0x6u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1046a99); f_01045e5f(cpu); if cpu.returned_to != 0x1046a99 { return; }
  { cpu.sub8(cpu.r8(0x1124ee4u32), 0x0u8); }
  if cpu.zf { pc = 0x1046aa9; continue; }
  pc = 0x1046aa2; continue;
 }
 0x1046aa2 => {
  { let v: u8 = 0xffu8; cpu.w8(0x112894eu32, v); }
  pc = 0x1046aa9; continue;
 }
 0x1046aa9 => {
  { let v: u32 = 0x10c68b6u32; cpu.esi = v; }
  { let v: u8 = 0x33u8; cpu.w8(0x6u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1046ab7); f_01045e5f(cpu); if cpu.returned_to != 0x1046ab7 { return; }
  { cpu.sub8(cpu.r8(0x1124ee4u32), 0x0u8); }
  if cpu.zf { pc = 0x1046ac7; continue; }
  pc = 0x1046ac0; continue;
 }
 0x1046ac0 => {
  { let v: u8 = 0xffu8; cpu.w8(0x112894fu32, v); }
  pc = 0x1046ac7; continue;
 }
 0x1046ac7 => {
  { cpu.popad(); }
  { cpu.sub32(cpu.r32(0x112894cu32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x1046ad8; continue; }
  pc = 0x1046ad1; continue;
 }
 0x1046ad1 => {
  { let v: u8 = 0xffu8; cpu.w8(0x11263a1u32, v); }
  pc = 0x1046ad8; continue;
 }
 0x1046ad8 => {
  { let v: u8 = cpu.r8(0x11263a0u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), cpu.r8(0x11263a1u32)); }
  if !cpu.zf { pc = 0x1046aee; continue; }
  pc = 0x1046ae6; continue;
 }
 0x1046ae6 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x11263a2u32)); cpu.w8(0x11263a2u32, v); }
  if !cpu.zf { pc = 0x1046b1d; continue; }
  pc = 0x1046aee; continue;
 }
 0x1046aee => {
  { let v: u8 = 0x3cu8; cpu.w8(0x11263a2u32, v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x11263a1u32, v); }
  { let v: u32 = 0x10c68adu32; cpu.esi = v; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), 0x30u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x6u32.wrapping_add(cpu.esi), v); }
  { cpu.pushad(); }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  cpu.push(0x1046b11); f_01037d64(cpu); if cpu.returned_to != 0x1046b11 { return; }
  { cpu.popad(); }
  pc = 0x1046b1d; continue;
 }
 0x1046b14 => {
  { let v: u8 = cpu.sub8((cpu.ebx as u8), 0x30u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x11263a0u32, v); }
  pc = 0x1046b1d; continue;
 }
 0x1046b1d => {
  cpu.push(0x1046b22); f_01044810(cpu); if cpu.returned_to != 0x1046b22 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046b23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046b23); }
 let mut pc: u32 = 0x1046b23;
 loop { match pc {
 0x1046b23 => {
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x1u8); }
  if cpu.zf { pc = 0x1046b6c; continue; }
  pc = 0x1046b2c; continue;
 }
 0x1046b2c => {
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x2u8); }
  if cpu.zf { pc = 0x1046b78; continue; }
  pc = 0x1046b35; continue;
 }
 0x1046b35 => {
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x3u8); }
  if cpu.zf { pc = 0x1046b84; continue; }
  pc = 0x1046b3e; continue;
 }
 0x1046b3e => {
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x4u8); }
  if cpu.zf { pc = 0x1046b90; continue; }
  pc = 0x1046b47; continue;
 }
 0x1046b47 => {
  { cpu.sub8(cpu.r8(0x10fd44bu32), 0x5u8); }
  if cpu.zf { pc = 0x1046b9c; continue; }
  pc = 0x1046b50; continue;
 }
 0x1046b50 => {
  pc = 0x1046ba8; continue;
 }
 0x1046b52 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1046b53 => {
  { let v: u32 = 0x112825cu32; cpu.edi = v; }
  pc = 0x1046b58; continue;
 }
 0x1046b58 => {
  { cpu.sub32(cpu.edi, 0x1128872u32); }
  if cpu.zf { cpu.poll(); pc = 0x1046b52; continue; }
  pc = 0x1046b60; continue;
 }
 0x1046b60 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if cpu.zf { pc = 0x1046ba8; continue; }
  pc = 0x1046b6a; continue;
 }
 0x1046b6a => {
  cpu.poll(); pc = 0x1046b58; continue;
 }
 0x1046b6c => {
  { let v: u32 = 0x11263eeu32; cpu.esi = v; }
  { let v: u32 = 0x1126a04u32; cpu.ebp = v; }
  cpu.poll(); pc = 0x1046b53; continue;
 }
 0x1046b78 => {
  { let v: u32 = 0x1126a04u32; cpu.esi = v; }
  { let v: u32 = 0x112701au32; cpu.ebp = v; }
  cpu.poll(); pc = 0x1046b53; continue;
 }
 0x1046b84 => {
  { let v: u32 = 0x112701au32; cpu.esi = v; }
  { let v: u32 = 0x1127630u32; cpu.ebp = v; }
  cpu.poll(); pc = 0x1046b53; continue;
 }
 0x1046b90 => {
  { let v: u32 = 0x1127630u32; cpu.esi = v; }
  { let v: u32 = 0x1127c46u32; cpu.ebp = v; }
  cpu.poll(); pc = 0x1046b53; continue;
 }
 0x1046b9c => {
  { let v: u32 = 0x1127c46u32; cpu.esi = v; }
  { let v: u32 = 0x112825cu32; cpu.ebp = v; }
  cpu.poll(); pc = 0x1046b53; continue;
 }
 0x1046ba8 => {
  cpu.push(0x1046bad); f_01040d44(cpu); if cpu.returned_to != 0x1046bad { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046bb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046bb0); }
 let mut pc: u32 = 0x1046bb0;
 loop { match pc {
 0x1046bb0 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10ee934u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10ee938u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10ee93cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10ee940u32, v); }
  { let v: u32 = 0x11757a4u32; cpu.esi = v; }
  { let v: u8 = 0xf0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1046bdf; continue;
 }
 0x1046bdf => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1046c2b; continue; }
  pc = 0x1046be5; continue;
 }
 0x1046be5 => {
  { cpu.sub8(cpu.r8(0x13u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1046c2b; continue; }
  pc = 0x1046beb; continue;
 }
 0x1046beb => {
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r8(0x13u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1046c2b; continue; }
  pc = 0x1046c06; continue;
 }
 0x1046c06 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10ee934u32.wrapping_add(cpu.eax))); cpu.w8(0x10ee934u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x10ee938u32; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1046c27; continue; }
  pc = 0x1046c17; continue;
 }
 0x1046c17 => {
  { let v: u32 = 0x10ee93cu32; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1046c27; continue; }
  pc = 0x1046c22; continue;
 }
 0x1046c22 => {
  { let v: u32 = 0x10ee940u32; cpu.edi = v; }
  pc = 0x1046c27; continue;
 }
 0x1046c27 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1))).wrapping_add(0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  pc = 0x1046c2b; continue;
 }
 0x1046c2b => {
  { let v: u32 = cpu.add32(cpu.esi, 0x48u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1046bdf; continue; }
  pc = 0x1046c32; continue;
 }
 0x1046c32 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046c33(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046c33); }
 let mut pc: u32 = 0x1046c33;
 loop { match pc {
 0x1046c33 => {
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = 0x116cbc8u32; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef358u32, v); }
  pc = 0x1046c49; continue;
 }
 0x1046c49 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1046c81; continue; }
  pc = 0x1046c4f; continue;
 }
 0x1046c4f => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x1046c81; continue; }
  pc = 0x1046c55; continue;
 }
 0x1046c55 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1046c81; continue; }
  pc = 0x1046c5b; continue;
 }
 0x1046c5b => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x20u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x14u32); cpu.edi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x10ef358u32)); cpu.w8(0x10ef358u32, v); }
  pc = 0x1046c81; continue;
 }
 0x1046c81 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1046c49; continue; }
  pc = 0x1046c89; continue;
 }
 0x1046c89 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046c8a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046c8a); }
 let mut pc: u32 = 0x1046c8a;
 loop { match pc {
 0x1046c8a => {
  { cpu.pushad(); }
  { let v: u32 = 0x116cbc8u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x10ef358u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1046cb3; continue; }
  pc = 0x1046c9a; continue;
 }
 0x1046c9a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1046c9c; continue;
 }
 0x1046c9c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x14u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1046c9c; continue; }
  pc = 0x1046cb3; continue;
 }
 0x1046cb3 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046cb5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046cb5); }
 let mut pc: u32 = 0x1046cb5;
 loop { match pc {
 0x1046cb5 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046cb7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046cb7); }
 let mut pc: u32 = 0x1046cb7;
 loop { match pc {
 0x1046cb7 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046d33(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1046d33); } r_01046d33(cpu, 0x1046d33); }
pub fn f_01046e49(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1046e49); } r_01046d33(cpu, 0x1046e49); }
fn r_01046d33(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1046d33 => {
  { let v: u32 = cpu.r32(0x10e4148u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.pushad(); }
  cpu.push(0x1046d44); f_010472cc(cpu); if cpu.returned_to != 0x1046d44 { return; }
  { cpu.popad(); }
  if !cpu.cf { pc = 0x1046d48; continue; }
  pc = 0x1046d47; continue;
 }
 0x1046d47 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1046d48 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x1046e58; continue; }
  pc = 0x1046d52; continue;
 }
 0x1046d52 => {
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x1046e75; continue; }
  pc = 0x1046d5c; continue;
 }
 0x1046d5c => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1046d78; continue; }
  pc = 0x1046d65; continue;
 }
 0x1046d65 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.push(0x1046d6f); f_01047a18(cpu); if cpu.returned_to != 0x1046d6f { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.zf { pc = 0x1046e58; continue; }
  pc = 0x1046d78; continue;
 }
 0x1046d78 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x7u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.ecx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e99u32, v); }
  if cpu.zf { pc = 0x1046dba; continue; }
  pc = 0x1046d9d; continue;
 }
 0x1046d9d => {
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1046dab; continue; }
  pc = 0x1046da8; continue;
 }
 0x1046da8 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x1046dab; continue;
 }
 0x1046dab => {
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1046dba; continue; }
  pc = 0x1046db7; continue;
 }
 0x1046db7 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  pc = 0x1046dba; continue;
 }
 0x1046dba => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1046dc3); f_0104f36d(cpu); if cpu.returned_to != 0x1046dc3 { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4000u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))), 0xffu8); }
  if cpu.zf { pc = 0x1046e58; continue; }
  pc = 0x1046de3; continue;
 }
 0x1046de3 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))), 0xffu8); }
  if cpu.zf { pc = 0x1046e58; continue; }
  pc = 0x1046df3; continue;
 }
 0x1046df3 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))), 0xffu8); }
  if cpu.zf { pc = 0x1046e58; continue; }
  pc = 0x1046e02; continue;
 }
 0x1046e02 => {
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))), 0xffu8); }
  if cpu.zf { pc = 0x1046e58; continue; }
  pc = 0x1046e11; continue;
 }
 0x1046e11 => {
  { cpu.sub8(cpu.r8(0x10e3e99u32), 0x0u8); }
  if cpu.zf { pc = 0x1046e5a; continue; }
  pc = 0x1046e1a; continue;
 }
 0x1046e1a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xfu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1046e2a); f_01050829(cpu); if cpu.returned_to != 0x1046e2a { return; }
  cpu.push(0x1046e2f); f_0106a2d3(cpu); if cpu.returned_to != 0x1046e2f { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) & 0xfdu8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ee980u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6eu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1046e49; continue;
 }
 0x1046e49 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x1152e54u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6du32.wrapping_add(cpu.ebp), v); }
  pc = 0x1046e58; continue;
 }
 0x1046e58 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1046e5a => {
  { let v: u8 = cpu.or8(cpu.r8(0x6fu32.wrapping_add(cpu.esi)), 0x1u8); cpu.w8(0x6fu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x10ee980u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6eu32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1046e6d); f_01046e49(cpu); if cpu.returned_to != 0x1046e6d { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x1046e73); f_010471ac(cpu); if cpu.returned_to != 0x1046e73 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1046e75 => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), cpu.r8(0x6u32.wrapping_add(cpu.esi))); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1046d78; continue; }
  pc = 0x1046e81; continue;
 }
 0x1046e81 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046e82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1046e82); }
 let mut pc: u32 = 0x1046e82;
 loop { match pc {
 0x1046e82 => {
  { cpu.sub32(cpu.ebp, 0x115dad4u32); }
  if cpu.zf { pc = 0x1046ef7; continue; }
  pc = 0x1046e8e; continue;
 }
 0x1046e8e => {
  { cpu.pushad(); }
  cpu.push(0x1046e94); f_01046c8a(cpu); if cpu.returned_to != 0x1046e94 { return; }
  { let v: u32 = 0x116cbc8u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x10ef358u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1046ef6; continue; }
  pc = 0x1046ea3; continue;
 }
 0x1046ea3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1046eba; continue;
 }
 0x1046eba => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), ((cpu.ecx >> 8) as u8)); }
  if cpu.zf { pc = 0x1046eef; continue; }
  pc = 0x1046ec6; continue;
 }
 0x1046ec6 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x1046eef; continue; }
  pc = 0x1046eca; continue;
 }
 0x1046eca => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  if !cpu.sf { pc = 0x1046ed3; continue; }
  pc = 0x1046ed1; continue;
 }
 0x1046ed1 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x1046ed3; continue;
 }
 0x1046ed3 => {
  { cpu.sub32(cpu.edx, 0x20u32); }
  if (cpu.sf == cpu.of) { pc = 0x1046eef; continue; }
  pc = 0x1046ed8; continue;
 }
 0x1046ed8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.ebx); cpu.edx = v; }
  if !cpu.sf { pc = 0x1046ee1; continue; }
  pc = 0x1046edf; continue;
 }
 0x1046edf => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x1046ee1; continue;
 }
 0x1046ee1 => {
  { cpu.sub32(cpu.edx, 0x20u32); }
  if (cpu.sf == cpu.of) { pc = 0x1046eef; continue; }
  pc = 0x1046ee6; continue;
 }
 0x1046ee6 => {
  { let v: u32 = cpu.esi; cpu.w32(0x10e4148u32, v); }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1046eef => {
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  { let v: u8 = cpu.dec8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1046eba; continue; }
  pc = 0x1046ef6; continue;
 }
 0x1046ef6 => {
  { cpu.popad(); }
  pc = 0x1046ef7; continue;
 }
 0x1046ef7 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01046fbd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1046fbd); } r_01046fbd(cpu, 0x1046fbd); }
pub fn f_010470b2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10470b2); } r_01046fbd(cpu, 0x10470b2); }
fn r_01046fbd(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1046fbd => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x7u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.ecx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e99u32, v); }
  if cpu.zf { pc = 0x1046fff; continue; }
  pc = 0x1046fe2; continue;
 }
 0x1046fe2 => {
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1046ff0; continue; }
  pc = 0x1046fed; continue;
 }
 0x1046fed => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  pc = 0x1046ff0; continue;
 }
 0x1046ff0 => {
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1046fff; continue; }
  pc = 0x1046ffc; continue;
 }
 0x1046ffc => {
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  pc = 0x1046fff; continue;
 }
 0x1046fff => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1047008); f_0104f36d(cpu); if cpu.returned_to != 0x1047008 { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4000u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))), 0xffu8); }
  if cpu.zf { pc = 0x10470c1; continue; }
  pc = 0x1047028; continue;
 }
 0x1047028 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))), 0xffu8); }
  if cpu.zf { pc = 0x10470c1; continue; }
  pc = 0x1047038; continue;
 }
 0x1047038 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))), 0xffu8); }
  if cpu.zf { pc = 0x10470c1; continue; }
  pc = 0x104704b; continue;
 }
 0x104704b => {
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))), 0xffu8); }
  if cpu.zf { pc = 0x10470c1; continue; }
  pc = 0x104705a; continue;
 }
 0x104705a => {
  { cpu.sub8(cpu.r8(0x10e3e99u32), 0x0u8); }
  if cpu.zf { pc = 0x10470c3; continue; }
  pc = 0x1047063; continue;
 }
 0x1047063 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xfu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.r32(0x10e3f84u32); cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f84u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x104708d); f_01050829(cpu); if cpu.returned_to != 0x104708d { return; }
  cpu.push(0x1047092); f_0106a2d3(cpu); if cpu.returned_to != 0x1047092 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3f84u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) & 0xfdu8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ee980u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6eu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x10470b2; continue;
 }
 0x10470b2 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x1152e54u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6du32.wrapping_add(cpu.ebp), v); }
  pc = 0x10470c1; continue;
 }
 0x10470c1 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10470c3 => {
  { let v: u8 = cpu.or8(cpu.r8(0x6fu32.wrapping_add(cpu.esi)), 0x1u8); cpu.w8(0x6fu32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x10ee980u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6eu32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10470d6); f_010470b2(cpu); if cpu.returned_to != 0x10470d6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x10470dc); f_010471ac(cpu); if cpu.returned_to != 0x10470dc { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010470de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10470de); }
 let mut pc: u32 = 0x10470de;
 loop { match pc {
 0x10470de => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub8((cpu.ebx as u8), cpu.r8(0x7u32.wrapping_add(cpu.eax))); }
  if (cpu.sf == cpu.of) { f_01046fbd(cpu); return; }
  pc = 0x10470f0; continue;
 }
 0x10470f0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010470f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10470f1); }
 let mut pc: u32 = 0x10470f1;
 loop { match pc {
 0x10470f1 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x104714e; continue; }
  pc = 0x10470fb; continue;
 }
 0x10470fb => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x6eu32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1152e54u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x1047147; continue; }
  pc = 0x104710f; continue;
 }
 0x104710f => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x1047140; continue; }
  pc = 0x1047115; continue;
 }
 0x1047115 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x70u32.wrapping_add(cpu.ebp))); cpu.w8(0x70u32.wrapping_add(cpu.ebp), v); }
  if cpu.zf { pc = 0x1047140; continue; }
  pc = 0x104711a; continue;
 }
 0x104711a => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1047127; continue; }
  pc = 0x1047124; continue;
 }
 0x1047124 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1047127; continue;
 }
 0x1047127 => {
  { cpu.sub16((cpu.eax as u16), 0x30u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1047140; continue; }
  pc = 0x104712d; continue;
 }
 0x104712d => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x104713a; continue; }
  pc = 0x1047137; continue;
 }
 0x1047137 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x104713a; continue;
 }
 0x104713a => {
  { cpu.sub16((cpu.eax as u16), 0x30u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x104714e; continue; }
  pc = 0x1047140; continue;
 }
 0x1047140 => {
  cpu.push(0x1047145); f_01047224(cpu); if cpu.returned_to != 0x1047145 { return; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1047147 => {
  cpu.push(0x104714c); f_01047150(cpu); if cpu.returned_to != 0x104714c { return; }
  if !cpu.cf { cpu.poll(); pc = 0x1047140; continue; }
  pc = 0x104714e; continue;
 }
 0x104714e => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047150(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1047150); }
 let mut pc: u32 = 0x1047150;
 loop { match pc {
 0x1047150 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x11757b7u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  pc = 0x104716b; continue;
 }
 0x104716b => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x10471a6; continue; }
  pc = 0x1047172; continue;
 }
 0x1047172 => {
  { let v: u16 = cpu.imul2_16((cpu.eax as u16), 0xb8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.eax)), 0x4u8); }
  if cpu.zf { pc = 0x10471a6; continue; }
  pc = 0x1047182; continue;
 }
 0x1047182 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.ebx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x104718e; continue; }
  pc = 0x104718b; continue;
 }
 0x104718b => {
  { let v: u16 = cpu.neg16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x104718e; continue;
 }
 0x104718e => {
  { cpu.sub16((cpu.edi as u16), 0x30u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10471a6; continue; }
  pc = 0x1047194; continue;
 }
 0x1047194 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x10471a0; continue; }
  pc = 0x104719d; continue;
 }
 0x104719d => {
  { let v: u16 = cpu.neg16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x10471a0; continue;
 }
 0x10471a0 => {
  { cpu.sub16((cpu.edi as u16), 0x30u16); }
  if (cpu.sf != cpu.of) { pc = 0x10471aa; continue; }
  pc = 0x10471a6; continue;
 }
 0x10471a6 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x104716b; continue; }
  pc = 0x10471a8; continue;
 }
 0x10471a8 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10471aa => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010471ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10471ac); }
 let mut pc: u32 = 0x10471ac;
 loop { match pc {
 0x10471ac => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x104721b; continue; }
  pc = 0x10471b2; continue;
 }
 0x10471b2 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10471bf); f_0104f36d(cpu); if cpu.returned_to != 0x10471bf { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x7f7fu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x7f7fu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x4u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) | 0x2u8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) & 0x80u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) | 0x13u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) & 0x80u8; cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) | 0x13u8; cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)) & 0x80u8; cpu.w8(0x80u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)) | 0x13u8; cpu.w8(0x80u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x81u32.wrapping_add(cpu.esi)) & 0x80u8; cpu.w8(0x81u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.or8(cpu.r8(0x81u32.wrapping_add(cpu.esi)), 0x13u8); cpu.w8(0x81u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0xafu8; cpu.w8(0x70u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104721b => {
  { let v: u8 = 0xafu8; cpu.w8(0x70u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x16u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047224(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1047224); }
 let mut pc: u32 = 0x1047224;
 loop { match pc {
 0x1047224 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x10472ae; continue; }
  pc = 0x104722e; continue;
 }
 0x104722e => {
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) & 0xfcu8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1047237); f_010472af(cpu); if cpu.returned_to != 0x1047237 { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x10472ae; continue; }
  pc = 0x1047241; continue;
 }
 0x1047241 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.esi)), 0x4u8); }
  if cpu.zf { pc = 0x104728b; continue; }
  pc = 0x104724a; continue;
 }
 0x104724a => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.esi)) & 0xfbu8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x104725d); f_0104f36d(cpu); if cpu.returned_to != 0x104725d { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4000u32); cpu.esi = v; }
  { let a = cpu.edi; let b = cpu.esi; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.esi = v; } }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x8080u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16) | cpu.r16(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0x80u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x8080u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16) | cpu.r16(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x80u32.wrapping_add(cpu.edi), v); }
  pc = 0x104728b; continue;
 }
 0x104728b => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x10472ae; continue; }
  pc = 0x1047291; continue;
 }
 0x1047291 => {
  cpu.push(0x1047296); f_0106a2d3(cpu); if cpu.returned_to != 0x1047296 { return; }
  if !cpu.cf { pc = 0x10472a3; continue; }
  pc = 0x1047298; continue;
 }
 0x1047298 => {
  cpu.push(0x104729d); f_0106a395(cpu); if cpu.returned_to != 0x104729d { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x104728b; continue; }
  pc = 0x10472a1; continue;
 }
 0x10472a1 => {
  pc = 0x10472ae; continue;
 }
 0x10472a3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  pc = 0x10472ae; continue;
 }
 0x10472ae => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010472af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10472af); }
 let mut pc: u32 = 0x10472af;
 loop { match pc {
 0x10472af => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x6eu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10472cb; continue; }
  pc = 0x10472ba; continue;
 }
 0x10472ba => {
  { let v = cpu.r8(0x6eu32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x6du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x6eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10472cb; continue;
 }
 0x10472cb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010472cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10472cc); }
 let mut pc: u32 = 0x10472cc;
 loop { match pc {
 0x10472cc => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef372u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x10472df; continue; }
  pc = 0x10472dc; continue;
 }
 0x10472dc => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  pc = 0x10472df; continue;
 }
 0x10472df => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x104735e; continue; }
  pc = 0x10472e9; continue;
 }
 0x10472e9 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), cpu.r16(0x2cu32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1047301; continue; }
  pc = 0x10472fe; continue;
 }
 0x10472fe => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1047301; continue;
 }
 0x1047301 => {
  { cpu.sub16((cpu.eax as u16), 0x20u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x104735e; continue; }
  pc = 0x1047307; continue;
 }
 0x1047307 => {
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1047310; continue; }
  pc = 0x104730d; continue;
 }
 0x104730d => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1047310; continue;
 }
 0x1047310 => {
  { cpu.sub16((cpu.ebx as u16), 0x20u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x104735e; continue; }
  pc = 0x1047316; continue;
 }
 0x1047316 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x1047332); f_0106a395(cpu); if cpu.returned_to != 0x1047332 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x10472cc; continue; }
  pc = 0x104733a; continue;
 }
 0x104733a => {
  cpu.push(0x104733f); f_0106a326(cpu); if cpu.returned_to != 0x104733f { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x104735c; continue; }
  pc = 0x1047345; continue;
 }
 0x1047345 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) & 0xfeu8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0xbfu8); cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef372u32, v); }
  cpu.push(0x104735c); f_01047360(cpu); if cpu.returned_to != 0x104735c { return; }
  pc = 0x104735c; continue;
 }
 0x104735c => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104735e => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047360(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1047360); }
 let mut pc: u32 = 0x1047360;
 loop { match pc {
 0x1047360 => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x1047378; continue; }
  pc = 0x1047373; continue;
 }
 0x1047373 => {
  cpu.push(0x1047378); f_01047379(cpu); if cpu.returned_to != 0x1047378 { return; }
  pc = 0x1047378; continue;
 }
 0x1047378 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047379(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1047379); }
 let mut pc: u32 = 0x1047379;
 loop { match pc {
 0x1047379 => {
  { let v: u8 = 0x1u8; cpu.w8(0x5au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x3au32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x50u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x52u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f84u32, v); }
  cpu.push(0x10473ab); f_0106b3b5(cpu); if cpu.returned_to != 0x10473ab { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010473ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10473ac); }
 let mut pc: u32 = 0x10473ac;
 loop { match pc {
 0x10473ac => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010473ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10473ad); }
 let mut pc: u32 = 0x10473ad;
 loop { match pc {
 0x10473ad => {
  { cpu.sub16(cpu.r16(0x48u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x10473cf; continue; }
  pc = 0x10473b4; continue;
 }
 0x10473b4 => {
  cpu.push(0x10473b9); f_010473e7(cpu); if cpu.returned_to != 0x10473b9 { return; }
  { let v: u8 = cpu.add8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x10u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10473c2); f_0106a269(cpu); if cpu.returned_to != 0x10473c2 { return; }
  { let v: u8 = cpu.r8(0x10ef343u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10473cf); f_0106db23(cpu); if cpu.returned_to != 0x10473cf { return; }
  pc = 0x10473cf; continue;
 }
 0x10473cf => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x3cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10473db); f_01067331(cpu); if cpu.returned_to != 0x10473db { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010473e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10473e7); }
 let mut pc: u32 = 0x10473e7;
 loop { match pc {
 0x10473e7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef343u32, v); }
  { let v: u8 = cpu.r8(0x60u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010473f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10473f9); }
 let mut pc: u32 = 0x10473f9;
 loop { match pc {
 0x10473f9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010473fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10473fa); }
 let mut pc: u32 = 0x10473fa;
 loop { match pc {
 0x10473fa => {
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x18u32; cpu.edi = v; }
  pc = 0x1047415; continue;
 }
 0x1047415 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1047460; continue; }
  pc = 0x104741c; continue;
 }
 0x104741c => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x1047460; continue; }
  pc = 0x1047422; continue;
 }
 0x1047422 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x104742d; continue; }
  pc = 0x104742a; continue;
 }
 0x104742a => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x104742d; continue;
 }
 0x104742d => {
  { cpu.sub16((cpu.edx as u16), (cpu.edi as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1047460; continue; }
  pc = 0x1047432; continue;
 }
 0x1047432 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x104743e; continue; }
  pc = 0x104743b; continue;
 }
 0x104743b => {
  { let v: u16 = cpu.neg16((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x104743e; continue;
 }
 0x104743e => {
  { cpu.sub16((cpu.edx as u16), (cpu.edi as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1047460; continue; }
  pc = 0x1047443; continue;
 }
 0x1047443 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1047451); f_0104f36d(cpu); if cpu.returned_to != 0x1047451 { return; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4000u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1047460 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1047415; continue; }
  pc = 0x1047465; continue;
 }
 0x1047465 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047466(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1047466); } r_01047466(cpu, 0x1047466); }
pub fn f_01047649(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1047649); } r_01047466(cpu, 0x1047649); }
fn r_01047466(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1047466 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1128b00u32, v); }
  cpu.push(0x1047472); f_010470f1(cpu); if cpu.returned_to != 0x1047472 { return; }
  cpu.push(0x1047477); f_010473f9(cpu); if cpu.returned_to != 0x1047477 { return; }
  cpu.push(0x104747c); f_01047abe(cpu); if cpu.returned_to != 0x104747c { return; }
  cpu.push(0x1047481); f_01047b2e(cpu); if cpu.returned_to != 0x1047481 { return; }
  cpu.push(0x1047486); f_01047d09(cpu); if cpu.returned_to != 0x1047486 { return; }
  { let v: u8 = cpu.dec8(cpu.r8(0x66u32.wrapping_add(cpu.ebp))); cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x104748f; continue; }
  pc = 0x104748b; continue;
 }
 0x104748b => {
  { let v: u8 = 0x0u8; cpu.w8(0x66u32.wrapping_add(cpu.ebp), v); }
  pc = 0x104748f; continue;
 }
 0x104748f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.sf { pc = 0x104749f; continue; }
  pc = 0x1047498; continue;
 }
 0x1047498 => {
  if cpu.zf { pc = 0x104749f; continue; }
  pc = 0x104749a; continue;
 }
 0x104749a => {
  cpu.push(0x104749f); f_010473ad(cpu); if cpu.returned_to != 0x104749f { return; }
  pc = 0x104749f; continue;
 }
 0x104749f => {
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { pc = 0x10474d5; continue; }
  pc = 0x10474a5; continue;
 }
 0x10474a5 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x104761b; continue; }
  pc = 0x10474af; continue;
 }
 0x10474af => {
  cpu.push(0x10474b4); f_0106a2d3(cpu); if cpu.returned_to != 0x10474b4 { return; }
  if !cpu.cf { pc = 0x10474c1; continue; }
  pc = 0x10474b6; continue;
 }
 0x10474b6 => {
  cpu.push(0x10474bb); f_0106a395(cpu); if cpu.returned_to != 0x10474bb { return; }
  if cpu.cf { pc = 0x10474c1; continue; }
  pc = 0x10474bd; continue;
 }
 0x10474bd => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x10474af; continue; }
  pc = 0x10474c1; continue;
 }
 0x10474c1 => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) & 0xfdu8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  pc = 0x104761b; continue;
 }
 0x10474d5 => {
  { cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x10474e7; continue; }
  pc = 0x10474db; continue;
 }
 0x10474db => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128b00u32, v); }
  pc = 0x104761b; continue;
 }
 0x10474e7 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { pc = 0x104761b; continue; }
  pc = 0x10474f7; continue;
 }
 0x10474f7 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3dbdu32, v); }
  { cpu.pushad(); }
  cpu.push(0x1047512); f_01072d00(cpu); if cpu.returned_to != 0x1047512 { return; }
  if cpu.cf { pc = 0x10475b9; continue; }
  pc = 0x1047518; continue;
 }
 0x1047518 => {
  { let v: u16 = cpu.r16(0x112c96eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x1047529; continue; }
  pc = 0x1047525; continue;
 }
 0x1047525 => {
  { let v: u16 = 0x0u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1047529; continue;
 }
 0x1047529 => {
  { cpu.sub16((cpu.edx as u16), 0x100u16); }
  if (cpu.sf != cpu.of) { pc = 0x1047534; continue; }
  pc = 0x1047530; continue;
 }
 0x1047530 => {
  { let v: u16 = 0xffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1047534; continue;
 }
 0x1047534 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x110d732u32, v); }
  { cpu.sub8(cpu.r8(0x10ef35du32), 0x0u8); }
  if !cpu.zf { pc = 0x10475b9; continue; }
  pc = 0x1047548; continue;
 }
 0x1047548 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x10475a9; continue; }
  pc = 0x1047552; continue;
 }
 0x1047552 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x1047597; continue; }
  pc = 0x1047558; continue;
 }
 0x1047558 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if cpu.zf { pc = 0x104758e; continue; }
  pc = 0x104755e; continue;
 }
 0x104755e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x1047585; continue; }
  pc = 0x1047564; continue;
 }
 0x1047564 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x10475a0; continue; }
  pc = 0x104756a; continue;
 }
 0x104756a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { pc = 0x1047597; continue; }
  pc = 0x1047570; continue;
 }
 0x1047570 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x10475b2; continue; }
  pc = 0x1047576; continue;
 }
 0x1047576 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x104758e; continue; }
  pc = 0x104757c; continue;
 }
 0x104757c => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef35du32, v); }
  pc = 0x10475b9; continue;
 }
 0x1047585 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef35du32, v); }
  pc = 0x10475b9; continue;
 }
 0x104758e => {
  { let v: u8 = 0x2u8; cpu.w8(0x10ef35du32, v); }
  pc = 0x10475b9; continue;
 }
 0x1047597 => {
  { let v: u8 = 0x3u8; cpu.w8(0x10ef35du32, v); }
  pc = 0x10475b9; continue;
 }
 0x10475a0 => {
  { let v: u8 = 0x4u8; cpu.w8(0x10ef35du32, v); }
  pc = 0x10475b9; continue;
 }
 0x10475a9 => {
  { let v: u8 = 0x5u8; cpu.w8(0x10ef35du32, v); }
  pc = 0x10475b9; continue;
 }
 0x10475b2 => {
  { let v: u8 = 0x7u8; cpu.w8(0x10ef35du32, v); }
  pc = 0x10475b9; continue;
 }
 0x10475b9 => {
  { cpu.popad(); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x7eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10688d9u32; cpu.w32(0x10e3fd8u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1128affu32, v); }
  let t = cpu.r32(0x1129320u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.push(0x10475d7); cpu.call(t); if cpu.returned_to != 0x10475d7 { return; }
  cpu.push(0x10475dc); f_010479dc(cpu); if cpu.returned_to != 0x10475dc { return; }
  { let v: u8 = cpu.r8(0x10ef343u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let a = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); let b = (cpu.ecx as u16); { let v: u16 = b; cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); } { let v: u16 = a; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } }
  { let a = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); let b = (cpu.edx as u16); { let v: u16 = b; cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); } { let v: u16 = a; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; } }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1047601); f_01068d78(cpu); if cpu.returned_to != 0x1047601 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(cpu.r16(0x10e3dbbu32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x10e3dbdu32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.edx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x104761b; continue; }
  pc = 0x1047616; continue;
 }
 0x1047616 => {
  cpu.push(0x104761b); f_0106db23(cpu); if cpu.returned_to != 0x104761b { return; }
  pc = 0x104761b; continue;
 }
 0x104761b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x104763c; continue; }
  pc = 0x1047624; continue;
 }
 0x1047624 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x104762a); f_01047a18(cpu); if cpu.returned_to != 0x104762a { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if cpu.zf { pc = 0x104765c; continue; }
  pc = 0x104762d; continue;
 }
 0x104762d => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if !cpu.zf { pc = 0x104815b; continue; }
  pc = 0x1047637; continue;
 }
 0x1047637 => {
  pc = 0x10477b8; continue;
 }
 0x104763c => {
  { let v: u8 = 0x66u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x3du32.wrapping_add(cpu.ebp), v); }
  pc = 0x10476d0; continue;
 }
 0x1047649 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x22u32.wrapping_add(cpu.edi))); cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  if !cpu.sf { pc = 0x1047657; continue; }
  pc = 0x104764e; continue;
 }
 0x104764e => {
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x1047657); f_01047b13(cpu); if cpu.returned_to != 0x1047657 { return; }
  pc = 0x1047657; continue;
 }
 0x1047657 => {
  pc = 0x10478d9; continue;
 }
 0x104765c => {
  { let v: u8 = cpu.dec8(cpu.r8(0x22u32.wrapping_add(cpu.edi))); cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  if !cpu.sf { pc = 0x104766a; continue; }
  pc = 0x1047661; continue;
 }
 0x1047661 => {
  { let v: u8 = 0x0u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x104766a); f_01047b13(cpu); if cpu.returned_to != 0x104766a { return; }
  pc = 0x104766a; continue;
 }
 0x104766a => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1047686; continue; }
  pc = 0x1047670; continue;
 }
 0x1047670 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x1047686; continue; }
  pc = 0x1047676; continue;
 }
 0x1047676 => {
  { cpu.sub8(cpu.r8(0x5au32.wrapping_add(cpu.ebp)), 0xfbu8); }
  if cpu.zf { pc = 0x1047686; continue; }
  pc = 0x104767c; continue;
 }
 0x104767c => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x10478d9; continue; }
  pc = 0x1047686; continue;
 }
 0x1047686 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e95u32, v); }
  cpu.push(0x10476a9); f_0104885a(cpu); if cpu.returned_to != 0x10476a9 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x10478d9; continue; }
  pc = 0x10476b4; continue;
 }
 0x10476b4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xbfu8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) & 0xfeu8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x10476c4); f_0106a326(cpu); if cpu.returned_to != 0x10476c4 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x1128b00u32, v); }
  pc = 0x10478d9; continue;
 }
 0x10476d0 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x104815b; continue; }
  pc = 0x10476da; continue;
 }
 0x10476da => {
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x10477a9; continue; }
  pc = 0x10476e4; continue;
 }
 0x10476e4 => {
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x50u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x104770d; continue; }
  pc = 0x104770d; continue;
 }
 0x104770d => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x104773d; continue; }
  pc = 0x1047713; continue;
 }
 0x1047713 => {
  { cpu.pushad(); }
  { let v: u32 = (cpu.ecx as u16) as u32; cpu.eax = v; }
  { let v: u32 = (cpu.edx as u16) as u32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x104772c; continue; }
  pc = 0x1047720; continue;
 }
 0x1047720 => {
  cpu.push(0x1047725); f_0100f6b8(cpu); if cpu.returned_to != 0x1047725 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0x1128b00u32, v); }
  pc = 0x104772c; continue;
 }
 0x104772c => {
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x10478d9; continue;
 }
 0x104773d => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1047743); f_01049cd8(cpu); if cpu.returned_to != 0x1047743 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if !cpu.zf { pc = 0x104776a; continue; }
  pc = 0x104774d; continue;
 }
 0x104774d => {
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128afcu32, v); }
  cpu.push(0x104775d); f_010484bf(cpu); if cpu.returned_to != 0x104775d { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x1128afcu32, v); }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) & 0xdfu8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1047789; continue;
 }
 0x104776a => {
  cpu.push(0x104776f); f_0104872b(cpu); if cpu.returned_to != 0x104776f { return; }
  cpu.push(0x1047774); f_010487ef(cpu); if cpu.returned_to != 0x1047774 { return; }
  { let v: u32 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)) as u32; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x104777e); f_01048362(cpu); if cpu.returned_to != 0x104777e { return; }
  cpu.push(0x1047783); f_01049895(cpu); if cpu.returned_to != 0x1047783 { return; }
  if cpu.cf { pc = 0x1047788; continue; }
  pc = 0x1047785; continue;
 }
 0x1047785 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1047788; continue;
 }
 0x1047788 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1047789; continue;
 }
 0x1047789 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub(cpu.r8(0x20u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x10477a4); f_0106b659(cpu); if cpu.returned_to != 0x10477a4 { return; }
  pc = 0x10478d9; continue;
 }
 0x10477a9 => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x104815b; continue; }
  pc = 0x10477b3; continue;
 }
 0x10477b3 => {
  pc = 0x10478d9; continue;
 }
 0x10477b8 => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x10477c8; continue; }
  pc = 0x10477be; continue;
 }
 0x10477be => {
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x104815b; continue; }
  pc = 0x10477c8; continue;
 }
 0x10477c8 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x10478e9; continue; }
  pc = 0x10477d5; continue;
 }
 0x10477d5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x10477fb; continue; }
  pc = 0x10477de; continue;
 }
 0x10477de => {
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef302u32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x10477fb); f_01064a95(cpu); if cpu.returned_to != 0x10477fb { return; }
  pc = 0x10477fb; continue;
 }
 0x10477fb => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x104782b; continue; }
  pc = 0x1047802; continue;
 }
 0x1047802 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = !(cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x1047827; continue; }
  pc = 0x1047810; continue;
 }
 0x1047810 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x15u32.wrapping_add(cpu.edi))); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)) & 0x7u8; cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x12u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10478d9; continue; }
  pc = 0x1047827; continue;
 }
 0x1047827 => {
  { let v: u8 = 0xau8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x104782b; continue;
 }
 0x104782b => {
  cpu.push(0x1047830); f_01047a18(cpu); if cpu.returned_to != 0x1047830 { return; }
  if cpu.zf { pc = 0x10478d9; continue; }
  pc = 0x1047836; continue;
 }
 0x1047836 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let a = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); let b = ((cpu.eax >> 8) as u8); { let v: u8 = b; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } }
  cpu.push(0x1047843); f_0106331c(cpu); if cpu.returned_to != 0x1047843 { return; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1047849); f_01067515(cpu); if cpu.returned_to != 0x1047849 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x1047935; continue; }
  pc = 0x1047853; continue;
 }
 0x1047853 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x104785b); f_0106331c(cpu); if cpu.returned_to != 0x104785b { return; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10478d9; continue; }
  pc = 0x1047866; continue;
 }
 0x1047866 => {
  cpu.push(0x104786b); f_0108a349(cpu); if cpu.returned_to != 0x104786b { return; }
  if cpu.cf { pc = 0x10478d9; continue; }
  pc = 0x1047871; continue;
 }
 0x1047871 => {
  cpu.push(0x1047876); f_0108c181(cpu); if cpu.returned_to != 0x1047876 { return; }
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x1152e54u32); cpu.esi = v; }
  { cpu.pushad(); }
  { let a = cpu.ebp; let b = cpu.esi; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.esi = v; } }
  cpu.push(0x104788b); f_0108b513(cpu); if cpu.returned_to != 0x104788b { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1047899); f_01065635(cpu); if cpu.returned_to != 0x1047899 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10478a2); f_01065635(cpu); if cpu.returned_to != 0x10478a2 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x10478a9); f_01047a42(cpu); if cpu.returned_to != 0x10478a9 { return; }
  cpu.push(0x10478ae); f_01064250(cpu); if cpu.returned_to != 0x10478ae { return; }
  if cpu.cf { pc = 0x10478cb; continue; }
  pc = 0x10478b0; continue;
 }
 0x10478b0 => {
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if !cpu.zf { pc = 0x10478cb; continue; }
  pc = 0x10478b6; continue;
 }
 0x10478b6 => {
  { let v: u8 = cpu.r8(0x77u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3eu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10ee980u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x44u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x4bu8; cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10478cb; continue;
 }
 0x10478cb => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x77u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3eu32.wrapping_add(cpu.ebx), v); }
  pc = 0x10478e8; continue;
 }
 0x10478d9 => {
  { cpu.sub8(cpu.r8(0x1128b00u32), 0x0u8); }
  if cpu.zf { pc = 0x10478e7; continue; }
  pc = 0x10478e2; continue;
 }
 0x10478e2 => {
  cpu.push(0x10478e7); f_0106b659(cpu); if cpu.returned_to != 0x10478e7 { return; }
  pc = 0x10478e7; continue;
 }
 0x10478e7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10478e8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10478e9 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x74u32.wrapping_add(cpu.ebp))); cpu.w8(0x74u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x74u32.wrapping_add(cpu.ebp)) & 0x7u8; cpu.w8(0x74u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x74u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let a = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); let b = ((cpu.eax >> 8) as u8); { let v: u8 = b; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } }
  cpu.push(0x10478fd); f_0106331c(cpu); if cpu.returned_to != 0x10478fd { return; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1047903); f_01067515(cpu); if cpu.returned_to != 0x1047903 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x104790c); f_0106331c(cpu); if cpu.returned_to != 0x104790c { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x10478d9; continue; }
  pc = 0x1047911; continue;
 }
 0x1047911 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x10478d9; continue; }
  pc = 0x1047920; continue;
 }
 0x1047920 => {
  cpu.push(0x1047925); f_0108a349(cpu); if cpu.returned_to != 0x1047925 { return; }
  if cpu.cf { cpu.poll(); pc = 0x10478d9; continue; }
  pc = 0x1047927; continue;
 }
 0x1047927 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x74u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  cpu.poll(); pc = 0x1047871; continue;
 }
 0x1047935 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x104793b); f_010675a1(cpu); if cpu.returned_to != 0x104793b { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1047944); f_0106331c(cpu); if cpu.returned_to != 0x1047944 { return; }
  cpu.push(0x1047949); f_01047b13(cpu); if cpu.returned_to != 0x1047949 { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x10478d9; continue; }
  pc = 0x104794e; continue;
 }
 0x104794e => {
  { cpu.sub8((cpu.edx as u8), 0x8u8); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x10478d9; continue; }
  pc = 0x1047953; continue;
 }
 0x1047953 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10478d9; continue; }
  pc = 0x1047959; continue;
 }
 0x1047959 => {
  { let v: u8 = 0x28u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x11a188cu32); cpu.esi = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10478d9; continue; }
  pc = 0x1047970; continue;
 }
 0x1047970 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x170u32); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1047986; continue; }
  pc = 0x1047980; continue;
 }
 0x1047980 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x10479a8; continue; }
  pc = 0x1047986; continue;
 }
 0x1047986 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x170u32); cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1047998; continue; }
  pc = 0x1047992; continue;
 }
 0x1047992 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { pc = 0x10479a8; continue; }
  pc = 0x1047998; continue;
 }
 0x1047998 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x1128b00u32), 0x0u8); }
  if cpu.zf { pc = 0x10479a7; continue; }
  pc = 0x10479a2; continue;
 }
 0x10479a2 => {
  cpu.push(0x10479a7); f_0106b659(cpu); if cpu.returned_to != 0x10479a7 { return; }
  pc = 0x10479a7; continue;
 }
 0x10479a7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10479a8 => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) | 0x20u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10479c7); f_01050508(cpu); if cpu.returned_to != 0x10479c7 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x10479cd); f_01047aaa(cpu); if cpu.returned_to != 0x10479cd { return; }
  { cpu.sub8(cpu.r8(0x1128b00u32), 0x0u8); }
  if cpu.zf { pc = 0x10479db; continue; }
  pc = 0x10479d6; continue;
 }
 0x10479d6 => {
  cpu.push(0x10479db); f_0106b659(cpu); if cpu.returned_to != 0x10479db { return; }
  pc = 0x10479db; continue;
 }
 0x10479db => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104815b => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1048171; continue; }
  pc = 0x1048164; continue;
 }
 0x1048164 => {
  { cpu.sub8(cpu.r8(0x10ef20du32), 0x2u8); }
  if !cpu.zf { pc = 0x1048221; continue; }
  pc = 0x1048171; continue;
 }
 0x1048171 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.zf { pc = 0x1048193; continue; }
  pc = 0x104817d; continue;
 }
 0x104817d => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)) & 0xefu8; cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) & 0xf7u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x7fu8); cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x104818e); f_0106b3b5(cpu); if cpu.returned_to != 0x104818e { return; }
  pc = 0x1048221; continue;
 }
 0x1048193 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x14u32.wrapping_add(cpu.edi))); cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  if !cpu.sf { pc = 0x104819f; continue; }
  pc = 0x104819b; continue;
 }
 0x104819b => {
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x104819f; continue;
 }
 0x104819f => {
  { let v: u8 = cpu.dec8(cpu.r8(0x3cu32.wrapping_add(cpu.ebp))); cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x1048221; continue; }
  pc = 0x10481a8; continue;
 }
 0x10481a8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef302u32, v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x11e2180u32, v); }
  cpu.push(0x10481d3); f_01064a95(cpu); if cpu.returned_to != 0x10481d3 { return; }
  if cpu.cf { pc = 0x1048221; continue; }
  pc = 0x10481d5; continue;
 }
 0x10481d5 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xeu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xcu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.r32(0x11e2180u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x10481f5; continue; }
  pc = 0x10481e6; continue;
 }
 0x10481e6 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x11e2180u32); cpu.esi = v; }
  { let a = cpu.ebp; let b = cpu.esi; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.esi = v; } }
  cpu.push(0x10481f4); f_01065635(cpu); if cpu.returned_to != 0x10481f4 { return; }
  { cpu.popad(); }
  pc = 0x10481f5; continue;
 }
 0x10481f5 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10481fb); f_01049cd8(cpu); if cpu.returned_to != 0x10481fb { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x15u32.wrapping_add(cpu.edi))); }
  if cpu.zf { pc = 0x1048222; continue; }
  pc = 0x1048201; continue;
 }
 0x1048201 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if cpu.zf { pc = 0x1048221; continue; }
  pc = 0x104820a; continue;
 }
 0x104820a => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(cpu.r8(0x15u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x1048216; continue; }
  pc = 0x1048211; continue;
 }
 0x1048211 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x15u32.wrapping_add(cpu.edi))); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  pc = 0x1048219; continue;
 }
 0x1048216 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x15u32.wrapping_add(cpu.edi))); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  pc = 0x1048219; continue;
 }
 0x1048219 => {
  { let v: u8 = cpu.and8(cpu.r8(0x15u32.wrapping_add(cpu.edi)), 0x7u8); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1048221; continue;
 }
 0x1048221 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048222 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1048254; continue; }
  pc = 0x104822e; continue;
 }
 0x104822e => {
  cpu.push(0x1048233); f_01048255(cpu); if cpu.returned_to != 0x1048233 { return; }
  if cpu.cf { pc = 0x1048250; continue; }
  pc = 0x1048235; continue;
 }
 0x1048235 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x1048245; continue; }
  pc = 0x104823b; continue;
 }
 0x104823b => {
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), cpu.r8(0x75u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1048250; continue;
 }
 0x1048245 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), cpu.r8(0x20u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x78u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1048250; continue;
 }
 0x1048250 => {
  { let v: u8 = 0x0u8; cpu.w8(0x3cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1048254; continue;
 }
 0x1048254 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010479dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10479dc); }
 let mut pc: u32 = 0x10479dc;
 loop { match pc {
 0x10479dc => {
  { cpu.sub8(cpu.r8(0x1128affu32), 0x0u8); }
  if cpu.zf { pc = 0x10479ee; continue; }
  pc = 0x10479e5; continue;
 }
 0x10479e5 => {
  { let v: u8 = 0x6u8; cpu.w8(0x9fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1047a17; continue;
 }
 0x10479ee => {
  { let v: u8 = cpu.dec8(cpu.r8(0x9fu32.wrapping_add(cpu.ebp))); cpu.w8(0x9fu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x10479ff; continue; }
  pc = 0x10479f6; continue;
 }
 0x10479f6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x9fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1047a17; continue;
 }
 0x10479ff => {
  if !cpu.zf { pc = 0x1047a17; continue; }
  pc = 0x1047a01; continue;
 }
 0x1047a01 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x9eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x1047a0f; continue; }
  pc = 0x1047a0c; continue;
 }
 0x1047a0c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1047a0f; continue;
 }
 0x1047a0f => {
  { let v: u8 = 0xffu8; cpu.w8(0x9eu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1047a17; continue;
 }
 0x1047a17 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047a18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1047a18); }
 let mut pc: u32 = 0x1047a18;
 loop { match pc {
 0x1047a18 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047a1b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1047a1b); }
 let mut pc: u32 = 0x1047a1b;
 loop { match pc {
 0x1047a1b => {
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1047a3d; continue; }
  pc = 0x1047a24; continue;
 }
 0x1047a24 => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if cpu.zf { pc = 0x1047a3e; continue; }
  pc = 0x1047a2e; continue;
 }
 0x1047a2e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1047a37); f_010576b2(cpu); if cpu.returned_to != 0x1047a37 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1047a3d; continue;
 }
 0x1047a3d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1047a3e => {
  { let v: u32 = cpu.sub32(cpu.ebp, 0x0u32); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047a42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1047a42); }
 let mut pc: u32 = 0x1047a42;
 loop { match pc {
 0x1047a42 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x22u8); }
  if cpu.zf { pc = 0x1047a64; continue; }
  pc = 0x1047a48; continue;
 }
 0x1047a48 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x16u8); }
  if cpu.zf { pc = 0x1047a64; continue; }
  pc = 0x1047a4e; continue;
 }
 0x1047a4e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x1047aa2; continue; }
  pc = 0x1047a58; continue;
 }
 0x1047a58 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x1047aa2; continue; }
  pc = 0x1047a5e; continue;
 }
 0x1047a5e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x1047a97; continue; }
  pc = 0x1047a64; continue;
 }
 0x1047a64 => {
  { cpu.sub8(cpu.r8(0x97u32.wrapping_add(cpu.esi)), 0x0u8); }
  if !cpu.zf { pc = 0x1047aa9; continue; }
  pc = 0x1047a6d; continue;
 }
 0x1047a6d => {
  { let v: u8 = 0x88u8; cpu.w8(0x97u32.wrapping_add(cpu.esi), v); }
  cpu.push(0x1047a79); f_01053613(cpu); if cpu.returned_to != 0x1047a79 { return; }
  if cpu.cf { pc = 0x1047a97; continue; }
  pc = 0x1047a7b; continue;
 }
 0x1047a7b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.sub8((cpu.eax as u8), 0x1eu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x1047a88; continue; }
  pc = 0x1047a86; continue;
 }
 0x1047a86 => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1047a88; continue;
 }
 0x1047a88 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1047a90); f_01053613(cpu); if cpu.returned_to != 0x1047a90 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if !cpu.cf { pc = 0x1047aa2; continue; }
  pc = 0x1047a97; continue;
 }
 0x1047a97 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1047a9e); f_01047ae4(cpu); if cpu.returned_to != 0x1047a9e { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1047aa9; continue;
 }
 0x1047aa2 => {
  { cpu.pushad(); }
  cpu.push(0x1047aa8); f_01047b13(cpu); if cpu.returned_to != 0x1047aa8 { return; }
  { cpu.popad(); }
  pc = 0x1047aa9; continue;
 }
 0x1047aa9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047aaa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1047aaa); } r_01047aaa(cpu, 0x1047aaa); }
pub fn f_01047aba(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1047aba); } r_01047aaa(cpu, 0x1047aba); }
fn r_01047aaa(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1047aaa => {
  { let v: u8 = 0x10u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1047aac; continue;
 }
 0x1047aac => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1047aba => {
  { let v: u8 = 0xf0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.poll(); pc = 0x1047aac; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047abe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1047abe); }
 let mut pc: u32 = 0x1047abe;
 loop { match pc {
 0x1047abe => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0xf0u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.sar8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0xfu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x1047ad7; continue; }
  pc = 0x1047ad5; continue;
 }
 0x1047ad5 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1047ad7; continue;
 }
 0x1047ad7 => {
  { cpu.sub8((cpu.ebx as u8), 0x2u8); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1047ade; continue; }
  pc = 0x1047adc; continue;
 }
 0x1047adc => {
  { let v: u8 = 0x2u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1047ade; continue;
 }
 0x1047ade => {
  { let v: u8 = cpu.or8((cpu.ebx as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047ae4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1047ae4); }
 let mut pc: u32 = 0x1047ae4;
 loop { match pc {
 0x1047ae4 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = 0x66u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x1au32.wrapping_add(cpu.edi)), 0x0u8); }
  if cpu.sf { pc = 0x1047afd; continue; }
  pc = 0x1047af8; continue;
 }
 0x1047af8 => {
  if cpu.zf { pc = 0x1047afd; continue; }
  pc = 0x1047afa; continue;
 }
 0x1047afa => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1047afd => {
  { cpu.sub8(cpu.r8(0x19u32.wrapping_add(cpu.edi)), 0x3u8); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1047afa; continue; }
  pc = 0x1047b03; continue;
 }
 0x1047b03 => {
  { let v: u8 = 0x1u8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x5u8; cpu.w8(0x1bu32.wrapping_add(cpu.edi), v); }
  cpu.push(0x1047b10); f_01047aaa(cpu); if cpu.returned_to != 0x1047b10 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047b13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1047b13); }
 let mut pc: u32 = 0x1047b13;
 loop { match pc {
 0x1047b13 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x1au32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.sf { pc = 0x1047b1f; continue; }
  pc = 0x1047b1c; continue;
 }
 0x1047b1c => {
  if cpu.zf { pc = 0x1047b1f; continue; }
  pc = 0x1047b1e; continue;
 }
 0x1047b1e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1047b1f => {
  { cpu.sub8(cpu.r8(0x19u32.wrapping_add(cpu.edi)), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x1047b1e; continue; }
  pc = 0x1047b25; continue;
 }
 0x1047b25 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1au32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x19u8; cpu.w8(0x1bu32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047b2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1047b2e); }
 let mut pc: u32 = 0x1047b2e;
 loop { match pc {
 0x1047b2e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x1047c21; continue; }
  pc = 0x1047b38; continue;
 }
 0x1047b38 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x1047c21; continue; }
  pc = 0x1047b42; continue;
 }
 0x1047b42 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1bu8); }
  if cpu.zf { pc = 0x1047c21; continue; }
  pc = 0x1047b4c; continue;
 }
 0x1047b4c => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x1047c21; continue; }
  pc = 0x1047b56; continue;
 }
 0x1047b56 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x1047c21; continue; }
  pc = 0x1047b60; continue;
 }
 0x1047b60 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x1047c21; continue; }
  pc = 0x1047b6a; continue;
 }
 0x1047b6a => {
  { cpu.sub8(cpu.r8(0x97u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1047b79; continue; }
  pc = 0x1047b73; continue;
 }
 0x1047b73 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x97u32.wrapping_add(cpu.ebp))); cpu.w8(0x97u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1047b79; continue;
 }
 0x1047b79 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1047bc1; continue; }
  pc = 0x1047b87; continue;
 }
 0x1047b87 => {
  if cpu.sf { pc = 0x1047ba4; continue; }
  pc = 0x1047b89; continue;
 }
 0x1047b89 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x1bu32.wrapping_add(cpu.ebp))); cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x1047bb3; continue; }
  pc = 0x1047b8e; continue;
 }
 0x1047b8e => {
  { let v: u8 = cpu.inc8(cpu.r8(0x19u32.wrapping_add(cpu.ebp))); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x19u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x1047bb3; continue; }
  pc = 0x1047b97; continue;
 }
 0x1047b97 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x1047ba2); f_01047d00(cpu); if cpu.returned_to != 0x1047ba2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1047ba4 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x19u32.wrapping_add(cpu.ebp))); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { pc = 0x1047bb3; continue; }
  pc = 0x1047ba9; continue;
 }
 0x1047ba9 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1au32.wrapping_add(cpu.ebp))); cpu.w8(0x1au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x1047bb2); f_01047aba(cpu); if cpu.returned_to != 0x1047bb2 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1047bb3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1047bb5 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x19u32.wrapping_add(cpu.ebp))); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x1047bbf); f_01047d00(cpu); if cpu.returned_to != 0x1047bbf { return; }
  pc = 0x1047bbf; continue;
 }
 0x1047bbf => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1047bc1 => {
  { cpu.sub8(cpu.r8(0x19u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1047bbf; continue; }
  pc = 0x1047bc7; continue;
 }
 0x1047bc7 => {
  if !cpu.zf { cpu.poll(); pc = 0x1047bb5; continue; }
  pc = 0x1047bc9; continue;
 }
 0x1047bc9 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x1bu32.wrapping_add(cpu.ebp))); cpu.w8(0x1bu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { cpu.poll(); pc = 0x1047bbf; continue; }
  pc = 0x1047bce; continue;
 }
 0x1047bce => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let a = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); let b = ((cpu.eax >> 8) as u8); { let v: u8 = b; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } }
  cpu.push(0x1047bde); f_0106331c(cpu); if cpu.returned_to != 0x1047bde { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1047be5); f_01067515(cpu); if cpu.returned_to != 0x1047be5 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1047bef); f_0106331c(cpu); if cpu.returned_to != 0x1047bef { return; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0xffu8); }
  if !cpu.zf { pc = 0x1047bf5; continue; }
  pc = 0x1047bf4; continue;
 }
 0x1047bf4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1047bf5 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x5u32.wrapping_add(cpu.edi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1047bf4; continue; }
  pc = 0x1047bfa; continue;
 }
 0x1047bfa => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x1047bf4; continue; }
  pc = 0x1047c01; continue;
 }
 0x1047c01 => {
  cpu.push(0x1047c06); f_0108c181(cpu); if cpu.returned_to != 0x1047c06 { return; }
  { let v: u8 = cpu.inc8(cpu.r8(0x19u32.wrapping_add(cpu.edi))); cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x63u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1047c15); f_01047c22(cpu); if cpu.returned_to != 0x1047c15 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = 0x66u8; cpu.w8(0x22u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1047c21 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047c22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1047c22); }
 let mut pc: u32 = 0x1047c22;
 loop { match pc {
 0x1047c22 => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x1152e54u32); cpu.esi = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1047c35); f_01064250(cpu); if cpu.returned_to != 0x1047c35 { return; }
  if cpu.cf { pc = 0x1047c52; continue; }
  pc = 0x1047c37; continue;
 }
 0x1047c37 => {
  { cpu.sub8(cpu.r8(0x3bu32.wrapping_add(cpu.ebx)), 0xffu8); }
  if !cpu.zf { pc = 0x1047c52; continue; }
  pc = 0x1047c3d; continue;
 }
 0x1047c3d => {
  { let v: u8 = cpu.r8(0x77u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3eu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x4bu8; cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10ee980u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3bu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x44u32.wrapping_add(cpu.ebx), v); }
  pc = 0x1047c52; continue;
 }
 0x1047c52 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1047c5c); f_01047ce0(cpu); if cpu.returned_to != 0x1047c5c { return; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.r16(0x42u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x44u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = cpu.r16(0x46u32.wrapping_add(cpu.ebp)); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x25u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x76u32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1047c94); f_01072d00(cpu); if cpu.returned_to != 0x1047c94 { return; }
  if cpu.cf { pc = 0x1047cb0; continue; }
  pc = 0x1047c96; continue;
 }
 0x1047c96 => {
  { let v: u8 = 0x1u8; cpu.w8(0x110d736u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11296b5u32, v); }
  { let v: u16 = cpu.r16(0x112c96eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110d734u32, v); }
  pc = 0x1047cb0; continue;
 }
 0x1047cb0 => {
  { cpu.popad(); }
  cpu.push(0x1047cb6); f_01050256(cpu); if cpu.returned_to != 0x1047cb6 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11296b5u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x76u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1047ccd); f_0106331c(cpu); if cpu.returned_to != 0x1047ccd { return; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x46u32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x44u32.wrapping_add(cpu.ebp), v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.w16(0x42u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047ce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1047ce0); }
 let mut pc: u32 = 0x1047ce0;
 loop { match pc {
 0x1047ce0 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1047cf6); f_010499c1(cpu); if cpu.returned_to != 0x1047cf6 { return; }
  if !cpu.cf { pc = 0x1047cfa; continue; }
  pc = 0x1047cf8; continue;
 }
 0x1047cf8 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1047cfa => {
  { let v: u8 = cpu.shr8(cpu.r8(0x2u32.wrapping_add(cpu.ebp)), 2u32); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047d00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1047d00); }
 let mut pc: u32 = 0x1047d00;
 loop { match pc {
 0x1047d00 => {
  { let v: u8 = 0x64u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0x24u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1bu32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047d09(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1047d09); } r_01047d09(cpu, 0x1047d09); }
pub fn f_01047d8b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1047d8b); } r_01047d09(cpu, 0x1047d8b); }
pub fn f_01047f94(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1047f94); } r_01047d09(cpu, 0x1047f94); }
fn r_01047d09(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1047d09 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { pc = 0x1047ece; continue; }
  pc = 0x1047d16; continue;
 }
 0x1047d16 => {
  { cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1047ece; continue; }
  pc = 0x1047d20; continue;
 }
 0x1047d20 => {
  { let v: u32 = cpu.r8(0x38u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  cpu.push(0x1047d29); f_010676ce(cpu); if cpu.returned_to != 0x1047d29 { return; }
  { let v: u32 = 0x11a0f95u32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
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
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1128afeu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1128afeu32), 0x7u8); }
  if cpu.zf { pc = 0x1047da7; continue; }
  pc = 0x1047d8e; continue;
 }
 0x1047d8b => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  if cpu.zf { pc = 0x1047da7; continue; }
  pc = 0x1047d8e; continue;
 }
 0x1047d8e => {
  { cpu.sub8(cpu.r8(0x1128afeu32), 0x33u8); }
  if cpu.zf { pc = 0x1047da7; continue; }
  pc = 0x1047d97; continue;
 }
 0x1047d97 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x5u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x5u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x5u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x5u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1047da7; continue;
 }
 0x1047da7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3eafu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3eb1u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3eb3u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3eb5u32, v); }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { let v = cpu.r32(0x10e3f84u32); cpu.push(v); }
  pc = 0x1047dd5; continue;
 }
 0x1047dd5 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1047ec7; continue; }
  pc = 0x1047dde; continue;
 }
 0x1047dde => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x1047dd5; continue; }
  pc = 0x1047df2; continue;
 }
 0x1047df2 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x12u8); }
  if cpu.zf { cpu.poll(); pc = 0x1047dd5; continue; }
  pc = 0x1047df8; continue;
 }
 0x1047df8 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x1047dd5; continue; }
  pc = 0x1047dfe; continue;
 }
 0x1047dfe => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.zf { cpu.poll(); pc = 0x1047dd5; continue; }
  pc = 0x1047e04; continue;
 }
 0x1047e04 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x2eu8); }
  if cpu.zf { cpu.poll(); pc = 0x1047dd5; continue; }
  pc = 0x1047e0a; continue;
 }
 0x1047e0a => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x1047dd5; continue; }
  pc = 0x1047e13; continue;
 }
 0x1047e13 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e93u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1047ecf; continue; }
  pc = 0x1047e24; continue;
 }
 0x1047e24 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e97u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1047ecf; continue; }
  pc = 0x1047e31; continue;
 }
 0x1047e31 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e95u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1047ecf; continue; }
  pc = 0x1047e42; continue;
 }
 0x1047e42 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e99u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1047ecf; continue; }
  pc = 0x1047e4f; continue;
 }
 0x1047e4f => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1047e64; continue; }
  pc = 0x1047e55; continue;
 }
 0x1047e55 => {
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x11757a4u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = 0x4bu8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x1047e64; continue;
 }
 0x1047e64 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1047e77; continue; }
  pc = 0x1047e6d; continue;
 }
 0x1047e6d => {
  cpu.push(0x1047e72); f_01047fa0(cpu); if cpu.returned_to != 0x1047e72 { return; }
  cpu.poll(); pc = 0x1047dd5; continue;
 }
 0x1047e77 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x10ef334u32, v); }
  cpu.push(0x1047e85); f_010504cc(cpu); if cpu.returned_to != 0x1047e85 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef334u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x1047dd5; continue; }
  pc = 0x1047e98; continue;
 }
 0x1047e98 => {
  cpu.push(0x1047e9d); f_010395d1(cpu); if cpu.returned_to != 0x1047e9d { return; }
  if cpu.cf { pc = 0x1047ec7; continue; }
  pc = 0x1047e9f; continue;
 }
 0x1047e9f => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x1047eb4; continue; }
  pc = 0x1047eb1; continue;
 }
 0x1047eb1 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1047eb4; continue;
 }
 0x1047eb4 => {
  { cpu.stos(2, false); }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  cpu.poll(); pc = 0x1047dd5; continue;
 }
 0x1047ec7 => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3f84u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1047ece; continue;
 }
 0x1047ece => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1047ecf => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3eafu32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1047dd5; continue; }
  pc = 0x1047ee0; continue;
 }
 0x1047ee0 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3eb3u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1047dd5; continue; }
  pc = 0x1047eed; continue;
 }
 0x1047eed => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3eb1u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1047dd5; continue; }
  pc = 0x1047efe; continue;
 }
 0x1047efe => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3eb5u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1047dd5; continue; }
  pc = 0x1047f0b; continue;
 }
 0x1047f0b => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1047f3a; continue; }
  pc = 0x1047f11; continue;
 }
 0x1047f11 => {
  { let v = cpu.r32(0x10e3f84u32); cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f84u32, v); }
  cpu.push(0x1047f2e); f_01047fd8(cpu); if cpu.returned_to != 0x1047f2e { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3f84u32, v); }
  if cpu.cf { cpu.poll(); pc = 0x1047dd5; continue; }
  pc = 0x1047f3a; continue;
 }
 0x1047f3a => {
  { let v: u16 = cpu.r16(0x10e3eb3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3eafu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3eafu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3eb5u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3eb1u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3eb1u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1047f89); f_01069fde(cpu); if cpu.returned_to != 0x1047f89 { return; }
  cpu.push(0x1047f8e); f_0106a04e(cpu); if cpu.returned_to != 0x1047f8e { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.poll(); pc = 0x1047dd5; continue;
 }
 0x1047f94 => {
  { cpu.pushad(); }
  cpu.push(0x1047f9a); f_01048075(cpu); if cpu.returned_to != 0x1047f9a { return; }
  { cpu.popad(); }
  cpu.poll(); pc = 0x1047dd5; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047fa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1047fa0); }
 let mut pc: u32 = 0x1047fa0;
 loop { match pc {
 0x1047fa0 => {
  { let v: u8 = cpu.r8(0x3au32.wrapping_add(cpu.ebp)).wrapping_sub(0xau8); cpu.w8(0x3au32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x3au32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x1047fae; continue; }
  pc = 0x1047faa; continue;
 }
 0x1047faa => {
  { let v: u8 = 0xau8; cpu.w8(0x3au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1047fae; continue;
 }
 0x1047fae => {
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x72u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x17u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x1047fc7; continue; }
  pc = 0x1047fc5; continue;
 }
 0x1047fc5 => {
  { let v: u8 = 0x2eu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1047fc7; continue;
 }
 0x1047fc7 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xe0u8; cpu.w8(0x73u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01047fd8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1047fd8); } r_01047fd8(cpu, 0x1047fd8); }
pub fn f_01048000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1048000); } r_01047fd8(cpu, 0x1048000); }
fn r_01047fd8(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1047fd8 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x104800c; continue; }
  pc = 0x1047fe3; continue;
 }
 0x1047fe3 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x104800c; continue; }
  pc = 0x1047fec; continue;
 }
 0x1047fec => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x104800c; continue; }
  pc = 0x1047ff5; continue;
 }
 0x1047ff5 => {
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x104800c; continue; }
  pc = 0x1048001; continue;
 }
 0x1048000 => {
  { let v: u32 = cpu.ecx | cpu.r32(0x7f248043u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x104800c; continue; }
  pc = 0x104800a; continue;
 }
 0x1048001 => {
  { let v: u8 = cpu.r8(0xffffff80u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x104800c; continue; }
  pc = 0x104800a; continue;
 }
 0x104800a => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104800c => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax & 0x303u32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x1048029; continue; }
  pc = 0x1048021; continue;
 }
 0x1048021 => {
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x39u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x104802b; continue; }
  pc = 0x1048029; continue;
 }
 0x1048029 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104802b => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.r8(0x11757b7u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xb8u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1152e5bu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 4u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1048058; continue; }
  pc = 0x1048056; continue;
 }
 0x1048056 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048058 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x13u32.wrapping_add(cpu.eax)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xb8u32); cpu.eax = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  cpu.push(0x1048073); f_01048075(cpu); if cpu.returned_to != 0x1048073 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01048075(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1048075); }
 let mut pc: u32 = 0x1048075;
 loop { match pc {
 0x1048075 => {
  cpu.push(0x104807a); f_01047a18(cpu); if cpu.returned_to != 0x104807a { return; }
  if cpu.zf { pc = 0x10480ad; continue; }
  pc = 0x104807c; continue;
 }
 0x104807c => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x10480be; continue; }
  pc = 0x1048088; continue;
 }
 0x1048088 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x10480be; continue; }
  pc = 0x104808e; continue;
 }
 0x104808e => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x10480be; continue; }
  pc = 0x1048094; continue;
 }
 0x1048094 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x11u8); }
  if cpu.zf { pc = 0x10480be; continue; }
  pc = 0x104809a; continue;
 }
 0x104809a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 4u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10480be; continue; }
  pc = 0x10480ad; continue;
 }
 0x10480ad => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.esi = v; }
  cpu.push(0x10480b9); f_01046fbd(cpu); if cpu.returned_to != 0x10480b9 { return; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10480bc => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10480be => {
  { let v: u32 = cpu.r8(0x39u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { let v: u32 = 0x1128abcu32.wrapping_add(cpu.eax.wrapping_mul(8)); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x10480cb; continue;
 }
 0x10480cb => {
  { cpu.sub32(cpu.edi, 0x10u32); }
  if cpu.zf { cpu.poll(); pc = 0x10480bc; continue; }
  pc = 0x10480d0; continue;
 }
 0x10480d0 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1)))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(1)))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x10480ea); f_0105c902(cpu); if cpu.returned_to != 0x10480ea { return; }
  if !cpu.cf { pc = 0x10480f1; continue; }
  pc = 0x10480ec; continue;
 }
 0x10480ec => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  cpu.poll(); pc = 0x10480cb; continue;
 }
 0x10480f1 => {
  { let v = cpu.r32(0x10e3f84u32); cpu.push(v); }
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f84u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1048114); f_01050829(cpu); if cpu.returned_to != 0x1048114 { return; }
  cpu.push(0x1048119); f_0106a2d3(cpu); if cpu.returned_to != 0x1048119 { return; }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  { let v: u8 = 0x4u8; cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ee980u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebp)) | 0x1u8; cpu.w8(0x6fu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3f84u32, v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.esi = v; }
  cpu.push(0x1048144); f_01046fbd(cpu); if cpu.returned_to != 0x1048144 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = 0xb8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6eu32.wrapping_add(cpu.ebx), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01048255(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1048255); } r_01048255(cpu, 0x1048255); }
pub fn f_0104840f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x104840f); } r_01048255(cpu, 0x104840f); }
fn r_01048255(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1048255 => {
  { let v: u8 = 0x0u8; cpu.w8(0x1128afcu32, v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x18u8); }
  if cpu.zf { f_010484bf(cpu); return; }
  pc = 0x1048266; continue;
 }
 0x1048266 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x1048310; continue; }
  pc = 0x1048270; continue;
 }
 0x1048270 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x10483b0; continue; }
  pc = 0x104827a; continue;
 }
 0x104827a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x24u8); }
  if cpu.zf { pc = 0x104830e; continue; }
  pc = 0x1048284; continue;
 }
 0x1048284 => {
  cpu.push(0x1048289); f_0104872b(cpu); if cpu.returned_to != 0x1048289 { return; }
  cpu.push(0x104828e); f_010487ef(cpu); if cpu.returned_to != 0x104828e { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef302u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x104829d); f_01064a95(cpu); if cpu.returned_to != 0x104829d { return; }
  if cpu.cf { pc = 0x104830e; continue; }
  pc = 0x10482a3; continue;
 }
 0x10482a3 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x10482be; continue; }
  pc = 0x10482b4; continue;
 }
 0x10482b4 => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x10482be; continue; }
  pc = 0x10482bc; continue;
 }
 0x10482bc => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10482be; continue;
 }
 0x10482be => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x10482d5; continue; }
  pc = 0x10482d0; continue;
 }
 0x10482d0 => {
  cpu.push(0x10482d5); f_01053764(cpu); if cpu.returned_to != 0x10482d5 { return; }
  pc = 0x10482d5; continue;
 }
 0x10482d5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10482de); f_01049cd8(cpu); if cpu.returned_to != 0x10482de { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x10ef302u32) as u32; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if !cpu.zf { pc = 0x10482fc; continue; }
  pc = 0x10482f0; continue;
 }
 0x10482f0 => {
  cpu.push(0x10482f5); f_0104968b(cpu); if cpu.returned_to != 0x10482f5 { return; }
  if cpu.cf { pc = 0x104830b; continue; }
  pc = 0x10482f7; continue;
 }
 0x10482f7 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x104830b; continue;
 }
 0x10482fc => {
  cpu.push(0x1048301); f_01048362(cpu); if cpu.returned_to != 0x1048301 { return; }
  cpu.push(0x1048306); f_01049895(cpu); if cpu.returned_to != 0x1048306 { return; }
  if cpu.cf { pc = 0x104830b; continue; }
  pc = 0x1048308; continue;
 }
 0x1048308 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x104830b; continue;
 }
 0x104830b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104830e => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048310 => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x8bu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x104832d; continue; }
  pc = 0x104831a; continue;
 }
 0x104831a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub16(cpu.r16(0xeu32.wrapping_add(cpu.edi)), 0xfb2eu16); }
  if cpu.zf { pc = 0x104835f; continue; }
  pc = 0x1048325; continue;
 }
 0x1048325 => {
  { cpu.sub16(cpu.r16(0xeu32.wrapping_add(cpu.edi)), 0xef1fu16); }
  if cpu.zf { pc = 0x104835f; continue; }
  pc = 0x104832d; continue;
 }
 0x104832d => {
  cpu.push(0x1048332); f_0104872b(cpu); if cpu.returned_to != 0x1048332 { return; }
  cpu.push(0x1048337); f_010487ef(cpu); if cpu.returned_to != 0x1048337 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef302u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1048346); f_01064a95(cpu); if cpu.returned_to != 0x1048346 { return; }
  if cpu.cf { pc = 0x104835f; continue; }
  pc = 0x1048348; continue;
 }
 0x1048348 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffffu32); cpu.ebx = v; }
  cpu.push(0x104835c); f_0100f6b8(cpu); if cpu.returned_to != 0x104835c { return; }
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104835f => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10483b0 => {
  cpu.push(0x10483b5); f_0104872b(cpu); if cpu.returned_to != 0x10483b5 { return; }
  cpu.push(0x10483ba); f_010487ef(cpu); if cpu.returned_to != 0x10483ba { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef302u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x10483c9); f_01064a95(cpu); if cpu.returned_to != 0x10483c9 { return; }
  if cpu.cf { pc = 0x104849d; continue; }
  pc = 0x10483cf; continue;
 }
 0x10483cf => {
  { cpu.sub8(cpu.r8(0x9au32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.cf { pc = 0x10483eb; continue; }
  pc = 0x10483d8; continue;
 }
 0x10483d8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x9au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x6u8; cpu.w8(0x9bu32.wrapping_add(cpu.ebp), v); }
  pc = 0x104849d; continue;
 }
 0x10483eb => {
  cpu.push(0x10483f0); f_0104849f(cpu); if cpu.returned_to != 0x10483f0 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x104840b; continue; }
  pc = 0x1048401; continue;
 }
 0x1048401 => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x104840b; continue; }
  pc = 0x1048409; continue;
 }
 0x1048409 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x104840b; continue;
 }
 0x104840b => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x104841c); f_01053764(cpu); if cpu.returned_to != 0x104841c { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1048425); f_01049cd8(cpu); if cpu.returned_to != 0x1048425 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x10ef302u32) as u32; cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x0u32; cpu.w32(0x1128d44u32, v); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x104844a); f_01072d00(cpu); if cpu.returned_to != 0x104844a { return; }
  if cpu.cf { pc = 0x1048466; continue; }
  pc = 0x104844c; continue;
 }
 0x104840f => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x104841c); f_01053764(cpu); if cpu.returned_to != 0x104841c { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1048425); f_01049cd8(cpu); if cpu.returned_to != 0x1048425 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x10ef302u32) as u32; cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = 0x0u32; cpu.w32(0x1128d44u32, v); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x104844a); f_01072d00(cpu); if cpu.returned_to != 0x104844a { return; }
  if cpu.cf { pc = 0x1048466; continue; }
  pc = 0x104844c; continue;
 }
 0x104844c => {
  { let v: u8 = 0x3u8; cpu.w8(0x110d736u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x11296b5u32, v); }
  { let v: u16 = cpu.r16(0x112c96eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110d734u32, v); }
  pc = 0x1048466; continue;
 }
 0x1048466 => {
  { cpu.popad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x71u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x63u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x104847f); f_0104a611(cpu); if cpu.returned_to != 0x104847f { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x11296b5u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x1128d44u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1048493; continue; }
  pc = 0x1048490; continue;
 }
 0x1048490 => {
  { let v: u8 = cpu.inc8(cpu.r8(0xbu32.wrapping_add(cpu.eax))); cpu.w8(0xbu32.wrapping_add(cpu.eax), v); }
  pc = 0x1048493; continue;
 }
 0x1048493 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x1u8); cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104849d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01048362(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1048362); }
 let mut pc: u32 = 0x1048362;
 loop { match pc {
 0x1048362 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x1048388; continue; }
  pc = 0x1048368; continue;
 }
 0x1048368 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x1048390; continue; }
  pc = 0x104836e; continue;
 }
 0x104836e => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x1048398; continue; }
  pc = 0x1048374; continue;
 }
 0x1048374 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x10483a0; continue; }
  pc = 0x104837a; continue;
 }
 0x104837a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if cpu.zf { pc = 0x10483a8; continue; }
  pc = 0x1048380; continue;
 }
 0x1048380 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1128da1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048388 => {
  { let v: u8 = 0x12u8; cpu.w8(0x1128da1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048390 => {
  { let v: u8 = 0x15u8; cpu.w8(0x1128da1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048398 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1128da1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10483a0 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1128da1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10483a8 => {
  { let v: u8 = 0x16u8; cpu.w8(0x1128da1u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104849f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104849f); }
 let mut pc: u32 = 0x104849f;
 loop { match pc {
 0x104849f => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10484b2); f_01063324(cpu); if cpu.returned_to != 0x10484b2 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x10484bd; continue; }
  pc = 0x10484b5; continue;
 }
 0x10484b5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10484bd); f_01064020(cpu); if cpu.returned_to != 0x10484bd { return; }
  pc = 0x10484bd; continue;
 }
 0x10484bd => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010484bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10484bf); }
 let mut pc: u32 = 0x10484bf;
 loop { match pc {
 0x10484bf => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x10484c6); f_010486e3(cpu); if cpu.returned_to != 0x10484c6 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef302u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1128afcu32), 0x0u8); }
  if !cpu.zf { pc = 0x10484e8; continue; }
  pc = 0x10484d9; continue;
 }
 0x10484d9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10484e0); f_01064a95(cpu); if cpu.returned_to != 0x10484e0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x10486e1; continue; }
  pc = 0x10484e8; continue;
 }
 0x10484e8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x1048503; continue; }
  pc = 0x10484f9; continue;
 }
 0x10484f9 => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x1048503; continue; }
  pc = 0x1048501; continue;
 }
 0x1048501 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1048503; continue;
 }
 0x1048503 => {
  { let v: u16 = cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1048514); f_01049cd8(cpu); if cpu.returned_to != 0x1048514 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x10ef302u32) as u32; cpu.esi = v; }
  { let v: u16 = 0x29u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u8 = 0x20u8; cpu.w8(0x1128d9fu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1128d3cu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1128d3eu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1128d40u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1128d42u32, v); }
  cpu.push(0x1048550); f_01049561(cpu); if cpu.returned_to != 0x1048550 { return; }
  if cpu.cf { pc = 0x1048555; continue; }
  pc = 0x1048552; continue;
 }
 0x1048552 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1048555; continue;
 }
 0x1048555 => {
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x104856c); f_010486e3(cpu); if cpu.returned_to != 0x104856c { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef302u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1128afcu32), 0x0u8); }
  if !cpu.zf { pc = 0x104858a; continue; }
  pc = 0x104857f; continue;
 }
 0x104857f => {
  cpu.push(0x1048584); f_01064a95(cpu); if cpu.returned_to != 0x1048584 { return; }
  if cpu.cf { pc = 0x10486e1; continue; }
  pc = 0x104858a; continue;
 }
 0x104858a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x10485a5; continue; }
  pc = 0x104859b; continue;
 }
 0x104859b => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x10485a5; continue; }
  pc = 0x10485a3; continue;
 }
 0x10485a3 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10485a5; continue;
 }
 0x10485a5 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10485b6); f_01049cd8(cpu); if cpu.returned_to != 0x10485b6 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x10ef302u32) as u32; cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x10e3e99u32) as u32; cpu.ebx = v; }
  { let v: u32 = 0x1128cc4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x1128d3cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1128d3eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1128d40u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1128d42u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x29u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x1128d9fu32, v); }
  cpu.push(0x1048610); f_01049561(cpu); if cpu.returned_to != 0x1048610 { return; }
  if cpu.cf { pc = 0x1048615; continue; }
  pc = 0x1048612; continue;
 }
 0x1048612 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1048615; continue;
 }
 0x1048615 => {
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x104862c); f_010486e3(cpu); if cpu.returned_to != 0x104862c { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef302u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x1128afcu32), 0x0u8); }
  if !cpu.zf { pc = 0x104864a; continue; }
  pc = 0x104863f; continue;
 }
 0x104863f => {
  cpu.push(0x1048644); f_01064a95(cpu); if cpu.returned_to != 0x1048644 { return; }
  if cpu.cf { pc = 0x10486e1; continue; }
  pc = 0x104864a; continue;
 }
 0x104864a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x67u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.zf { pc = 0x1048665; continue; }
  pc = 0x104865b; continue;
 }
 0x104865b => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x1048665; continue; }
  pc = 0x1048663; continue;
 }
 0x1048663 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1048665; continue;
 }
 0x1048665 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1048676); f_01049cd8(cpu); if cpu.returned_to != 0x1048676 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x10ef302u32) as u32; cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x10e3e99u32) as u32; cpu.ebx = v; }
  { let v: u32 = 0x1128ce4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x1128d3cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1128d3eu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1128d40u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1128d42u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x29u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u8 = 0xffu8; cpu.w8(0x1128d9fu32, v); }
  cpu.push(0x10486d0); f_01049561(cpu); if cpu.returned_to != 0x10486d0 { return; }
  if cpu.cf { pc = 0x10486d5; continue; }
  pc = 0x10486d2; continue;
 }
 0x10486d2 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10486d5; continue;
 }
 0x10486d5 => {
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10486e1 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010486e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10486e3); }
 let mut pc: u32 = 0x10486e3;
 loop { match pc {
 0x10486e3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1128b88u32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1128b84u32.wrapping_add(cpu.eax.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x15u32.wrapping_add(cpu.edi)) as u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e99u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x15u32.wrapping_add(cpu.edi))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1128c04u32.wrapping_add(cpu.edx.wrapping_mul(8)))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1128c08u32.wrapping_add(cpu.edx.wrapping_mul(8)))); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104872b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104872b); }
 let mut pc: u32 = 0x104872b;
 loop { match pc {
 0x104872b => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if !cpu.zf { pc = 0x1048759; continue; }
  pc = 0x1048731; continue;
 }
 0x1048731 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r8(0x112bd80u32.wrapping_add(cpu.ebx.wrapping_mul(2))) as i8 as i16 as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x112bd81u32.wrapping_add(cpu.ebx.wrapping_mul(2))) as i8 as i16 as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x2eu32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048759 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x22u8); }
  if cpu.zf { pc = 0x10487a6; continue; }
  pc = 0x104875f; continue;
 }
 0x104875f => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x112bb8fu32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x112bb90u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x104878a); f_0105373b(cpu); if cpu.returned_to != 0x104878a { return; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x39u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1048796); f_0106331c(cpu); if cpu.returned_to != 0x1048796 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10487a6 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x112bd90u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x112bd91u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.esi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x1128d1cu32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x1128d1eu32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010487ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10487ef); }
 let mut pc: u32 = 0x10487ef;
 loop { match pc {
 0x10487ef => {
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let a = (cpu.ebx as u16); let b = (cpu.esi as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; } }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x6u8); }
  if cpu.zf { pc = 0x1048849; continue; }
  pc = 0x1048803; continue;
 }
 0x1048803 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x1eu8); }
  if cpu.zf { pc = 0x1048816; continue; }
  pc = 0x1048809; continue;
 }
 0x1048809 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x19u8); }
  if cpu.zf { pc = 0x1048838; continue; }
  pc = 0x104880f; continue;
 }
 0x104880f => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x26u8); }
  if cpu.zf { pc = 0x1048827; continue; }
  pc = 0x1048815; continue;
 }
 0x1048815 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048816 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x1128b42u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x1128b44u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048827 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x1128b22u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x1128b24u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048838 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x1128b62u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x1128b64u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048849 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x1128b02u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x1128b04u32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104885a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104885a); }
 let mut pc: u32 = 0x104885a;
 loop { match pc {
 0x104885a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3e93u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3e95u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x10488a4; continue; }
  pc = 0x104889e; continue;
 }
 0x104889e => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x10488b6; continue; }
  pc = 0x10488a4; continue;
 }
 0x10488a4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if !cpu.zf { pc = 0x10488b4; continue; }
  pc = 0x10488b0; continue;
 }
 0x10488b0 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x10488b6; continue; }
  pc = 0x10488b4; continue;
 }
 0x10488b4 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10488b6; continue;
 }
 0x10488b6 => {
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10488cd; continue; }
  pc = 0x10488c6; continue;
 }
 0x10488c6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10488cd => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
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
pub fn f_010488d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10488d4); }
 let mut pc: u32 = 0x10488d4;
 loop { match pc {
 0x10488d4 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { pc = 0x10488f1; continue; }
  pc = 0x10488db; continue;
 }
 0x10488db => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { pc = 0x10488e8; continue; }
  pc = 0x10488e4; continue;
 }
 0x10488e4 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x10488db; continue; }
  pc = 0x10488e8; continue;
 }
 0x10488e8 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10488f1 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010488f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10488f3); }
 let mut pc: u32 = 0x10488f3;
 loop { match pc {
 0x10488f3 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x10488ff; continue; }
  pc = 0x10488f9; continue;
 }
 0x10488f9 => {
  cpu.push(0x10488fe); f_010488d4(cpu); if cpu.returned_to != 0x10488fe { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10488ff => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01048901(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1048901); }
 let mut pc: u32 = 0x1048901;
 loop { match pc {
 0x1048901 => {
  { cpu.sub16(cpu.r16(0x50u32.wrapping_add(cpu.ebp)), 0xd80u16); }
  if !cpu.zf { pc = 0x1048913; continue; }
  pc = 0x1048909; continue;
 }
 0x1048909 => {
  { let v: u16 = cpu.r16(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x54u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048913 => {
  cpu.push(0x1048918); f_01063324(cpu); if cpu.returned_to != 0x1048918 { return; }
  if cpu.cf { pc = 0x1048923; continue; }
  pc = 0x104891a; continue;
 }
 0x104891a => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  pc = 0x1048923; continue;
 }
 0x1048923 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01048924(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1048924); }
 let mut pc: u32 = 0x1048924;
 loop { match pc {
 0x1048924 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01048926(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1048926); }
 let mut pc: u32 = 0x1048926;
 loop { match pc {
 0x1048926 => {
  cpu.push(0x104892b); f_010576b2(cpu); if cpu.returned_to != 0x104892b { return; }
  cpu.push(0x1048930); f_01062b13(cpu); if cpu.returned_to != 0x1048930 { return; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01048938(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1048938); }
 let mut pc: u32 = 0x1048938;
 loop { match pc {
 0x1048938 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x115db8cu32; cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  pc = 0x1048948; continue;
 }
 0x1048948 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x1048956; continue; }
  pc = 0x104894f; continue;
 }
 0x104894f => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1048948; continue; }
  pc = 0x1048953; continue;
 }
 0x1048953 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048956 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01048959(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1048959); }
 let mut pc: u32 = 0x1048959;
 loop { match pc {
 0x1048959 => {
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = 0x115db8cu32; cpu.edi = v; }
  pc = 0x1048967; continue;
 }
 0x1048967 => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2au32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1048967; continue; }
  pc = 0x104896e; continue;
 }
 0x104896e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104896f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104896f); }
 let mut pc: u32 = 0x104896f;
 loop { match pc {
 0x104896f => {
  { let v: u32 = 0x115db8cu32; cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x1048979; continue;
 }
 0x1048979 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x1048989; continue; }
  pc = 0x1048980; continue;
 }
 0x1048980 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1048987); f_0104898f(cpu); if cpu.returned_to != 0x1048987 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1048989; continue;
 }
 0x1048989 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x2cu32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1048979; continue; }
  pc = 0x104898e; continue;
 }
 0x104898e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104898f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104898f); }
 let mut pc: u32 = 0x104898f;
 loop { match pc {
 0x104898f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.ebx as u16), 7u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x28u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10489ad); f_01048d97(cpu); if cpu.returned_to != 0x10489ad { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e9bu32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbdu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e9du32, v); }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x1128d6cu32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.push(0x10489d9); cpu.call(t); if cpu.returned_to != 0x10489d9 { return; }
  if !cpu.zf { pc = 0x1048ad0; continue; }
  pc = 0x10489df; continue;
 }
 0x10489df => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x1048a3a; continue; }
  pc = 0x10489f9; continue;
 }
 0x10489f9 => {
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0xffu8; cpu.w8(0x112c942u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x112c96cu32, v); }
  cpu.push(0x1048a15); f_01072e0c(cpu); if cpu.returned_to != 0x1048a15 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c96cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x112c942u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x32u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048a3a => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0xcu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x18u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x2au32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x1048a60; continue; }
  pc = 0x1048a4b; continue;
 }
 0x1048a4b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1048a53); f_01048c76(cpu); if cpu.returned_to != 0x1048a53 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1048a5b); f_01048d08(cpu); if cpu.returned_to != 0x1048a5b { return; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x32u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1048ac2; continue;
 }
 0x1048a60 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0xffu8; cpu.w8(0x112c942u32, v); }
  cpu.push(0x1048a6f); f_01072e0c(cpu); if cpu.returned_to != 0x1048a6f { return; }
  cpu.push(0x1048a74); f_01048d08(cpu); if cpu.returned_to != 0x1048a74 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x112c942u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x32u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1048ac2; continue; }
  pc = 0x1048a83; continue;
 }
 0x1048a83 => {
  { let v: u32 = cpu.r32(0x112c954u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x9u8); }
  if cpu.zf { pc = 0x1048ac2; continue; }
  pc = 0x1048a93; continue;
 }
 0x1048a93 => {
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x2au8); }
  if cpu.zf { pc = 0x1048ac2; continue; }
  pc = 0x1048a98; continue;
 }
 0x1048a98 => {
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x26u8); }
  if cpu.zf { pc = 0x1048ac2; continue; }
  pc = 0x1048a9d; continue;
 }
 0x1048a9d => {
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x28u8); }
  if cpu.zf { pc = 0x1048ac2; continue; }
  pc = 0x1048aa2; continue;
 }
 0x1048aa2 => {
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x27u8); }
  if cpu.zf { pc = 0x1048ac2; continue; }
  pc = 0x1048aa7; continue;
 }
 0x1048aa7 => {
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x1du8); }
  if cpu.zf { pc = 0x1048ac2; continue; }
  pc = 0x1048aac; continue;
 }
 0x1048aac => {
  { let v: u8 = 0x1u8; cpu.w8(0x10ef33au32, v); }
  cpu.push(0x1048ab8); f_01049528(cpu); if cpu.returned_to != 0x1048ab8 { return; }
  { let v: u8 = cpu.sub8((cpu.edx as u8), 0x32u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x1048ac2; continue; }
  pc = 0x1048abd; continue;
 }
 0x1048abd => {
  cpu.push(0x1048ac2); f_01049528(cpu); if cpu.returned_to != 0x1048ac2 { return; }
  pc = 0x1048ac2; continue;
 }
 0x1048ac2 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ef33au32, v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048ad0 => {
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.mul16(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x1au32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1048b1a; continue; }
  pc = 0x1048b0b; continue;
 }
 0x1048b0b => {
  if cpu.cf { cpu.poll(); pc = 0x10489df; continue; }
  pc = 0x1048b11; continue;
 }
 0x1048b11 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x10489df; continue; }
  pc = 0x1048b1a; continue;
 }
 0x1048b1a => {
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x1048c43; continue; }
  pc = 0x1048b24; continue;
 }
 0x1048b24 => {
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x1048c54; continue; }
  pc = 0x1048b2e; continue;
 }
 0x1048b2e => {
  { cpu.sub8(cpu.r8(0x2au32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1048c65; continue; }
  pc = 0x1048b38; continue;
 }
 0x1048b38 => {
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if (cpu.sf == cpu.of) { pc = 0x1048b4a; continue; }
  pc = 0x1048b3f; continue;
 }
 0x1048b3f => {
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1048b4a; continue;
 }
 0x1048b4a => {
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
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1048b70); f_0104f36d(cpu); if cpu.returned_to != 0x1048b70 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10f714bu32; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x2au32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x1048b88; continue; }
  pc = 0x1048b83; continue;
 }
 0x1048b83 => {
  { let v: u32 = 0x10f57a7u32; cpu.ebx = v; }
  pc = 0x1048b88; continue;
 }
 0x1048b88 => {
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if (cpu.cf || cpu.zf) { pc = 0x1048ba8; continue; }
  pc = 0x1048b8d; continue;
 }
 0x1048b8d => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10489df; continue;
 }
 0x1048ba8 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x10e3d07u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x1048bc2); cpu.call(t); if cpu.returned_to != 0x1048bc2 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.push(0x1048bc8); f_01048e7a(cpu); if cpu.returned_to != 0x1048bc8 { return; }
  { let v: u32 = 0x662u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x8u8); }
  if cpu.zf { pc = 0x1048bf0; continue; }
  pc = 0x1048bd3; continue;
 }
 0x1048bd3 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10e4130u32, v); }
  { let v: u16 = cpu.shl16(cpu.r16(0x10c052cu32), 1u32); cpu.w16(0x10c052cu32, v); }
  { let v: u32 = 0x538u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(cpu.r16(0x10e3d07u32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1048bf0; continue;
 }
 0x1048bf0 => {
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if cpu.zf { pc = 0x1048bfd; continue; }
  pc = 0x1048bf6; continue;
 }
 0x1048bf6 => {
  { let v: u16 = cpu.shl16(cpu.r16(0x10c052cu32), 1u32); cpu.w16(0x10c052cu32, v); }
  pc = 0x1048bfd; continue;
 }
 0x1048bfd => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  let t = cpu.r32(0x10e4308u32); cpu.push(0x1048c29); cpu.call(t); if cpu.returned_to != 0x1048c29 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1048c3d; continue; }
  pc = 0x1048c32; continue;
 }
 0x1048c32 => {
  { let v: u8 = cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0xfbu8); cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1048c3c); f_010499ec(cpu); if cpu.returned_to != 0x1048c3c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1048c3d; continue;
 }
 0x1048c3d => {
  cpu.push(0x1048c42); f_01048f24(cpu); if cpu.returned_to != 0x1048c42 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048c43 => {
  cpu.push(0x1048c48); f_010490c5(cpu); if cpu.returned_to != 0x1048c48 { return; }
  if cpu.cf { cpu.poll(); pc = 0x10489df; continue; }
  pc = 0x1048c4e; continue;
 }
 0x1048c4e => {
  cpu.push(0x1048c53); f_01048f24(cpu); if cpu.returned_to != 0x1048c53 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048c54 => {
  cpu.push(0x1048c59); f_010491f3(cpu); if cpu.returned_to != 0x1048c59 { return; }
  if cpu.cf { cpu.poll(); pc = 0x10489df; continue; }
  pc = 0x1048c5f; continue;
 }
 0x1048c5f => {
  cpu.push(0x1048c64); f_01048f24(cpu); if cpu.returned_to != 0x1048c64 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048c65 => {
  cpu.push(0x1048c6a); f_010492bb(cpu); if cpu.returned_to != 0x1048c6a { return; }
  if cpu.cf { cpu.poll(); pc = 0x10489df; continue; }
  pc = 0x1048c70; continue;
 }
 0x1048c70 => {
  cpu.push(0x1048c75); f_01048f24(cpu); if cpu.returned_to != 0x1048c75 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01048c76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1048c76); }
 let mut pc: u32 = 0x1048c76;
 loop { match pc {
 0x1048c76 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x9u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x15u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e99u32, v); }
  cpu.push(0x1048c98); f_0104f36d(cpu); if cpu.returned_to != 0x1048c98 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10f57a7u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e99u32, v); }
  { let v: u16 = 0x1u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1048cb2; continue;
 }
 0x1048cb2 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1048cb8); f_0105f215(cpu); if cpu.returned_to != 0x1048cb8 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x1048cc2; continue; }
  pc = 0x1048cbd; continue;
 }
 0x1048cbd => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x1048cc2; continue;
 }
 0x1048cc2 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x46u32); cpu.ecx = v; }
  cpu.push(0x1048ccc); f_0105f215(cpu); if cpu.returned_to != 0x1048ccc { return; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1048cdf); f_0105f215(cpu); if cpu.returned_to != 0x1048cdf { return; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3e93u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x10e3e95u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e99u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x1048d03); f_01048d2f(cpu); if cpu.returned_to != 0x1048d03 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1048cb2; continue; }
  pc = 0x1048d06; continue;
 }
 0x1048d06 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01048d08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1048d08); }
 let mut pc: u32 = 0x1048d08;
 loop { match pc {
 0x1048d08 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1048d18); f_01018e6e(cpu); if cpu.returned_to != 0x1048d18 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  if cpu.cf { pc = 0x1048d1e; continue; }
  pc = 0x1048d1c; continue;
 }
 0x1048d1c => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048d1e => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x15u32.wrapping_add(cpu.eax), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01048d2f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1048d2f); }
 let mut pc: u32 = 0x1048d2f;
 loop { match pc {
 0x1048d2f => {
  { cpu.pushad(); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1048d3a); f_01039af3(cpu); if cpu.returned_to != 0x1048d3a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { pc = 0x1048d95; continue; }
  pc = 0x1048d41; continue;
 }
 0x1048d41 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x10c5974u32); cpu.eax = v; }
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
  pc = 0x1048d95; continue;
 }
 0x1048d95 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01048d97(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1048d97); }
 let mut pc: u32 = 0x1048d97;
 loop { match pc {
 0x1048d97 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1048da2); f_010676ce(cpu); if cpu.returned_to != 0x1048da2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x11a0f95u32; cpu.esi = v; }
  pc = 0x1048da8; continue;
 }
 0x1048da8 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1048e17; continue; }
  pc = 0x1048dad; continue;
 }
 0x1048dad => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x1048e14; continue; }
  pc = 0x1048dc0; continue;
 }
 0x1048dc0 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { pc = 0x1048e14; continue; }
  pc = 0x1048dc6; continue;
 }
 0x1048dc6 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x1048dd2; continue; }
  pc = 0x1048dcc; continue;
 }
 0x1048dcc => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { pc = 0x1048e14; continue; }
  pc = 0x1048dd2; continue;
 }
 0x1048dd2 => {
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
  if (!cpu.cf && !cpu.zf) { pc = 0x1048e14; continue; }
  pc = 0x1048dfd; continue;
 }
 0x1048dfd => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x2bu8); }
  if !cpu.zf { pc = 0x1048e09; continue; }
  pc = 0x1048e03; continue;
 }
 0x1048e03 => {
  { let v: u8 = 0x2u8; cpu.w8(0x7du32.wrapping_add(cpu.edi), v); }
  pc = 0x1048e14; continue;
 }
 0x1048e09 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  cpu.push(0x1048e12); f_01052979(cpu); if cpu.returned_to != 0x1048e12 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x1048e14; continue;
 }
 0x1048e14 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x1048da8; continue;
 }
 0x1048e17 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01048e18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1048e18); }
 let mut pc: u32 = 0x1048e18;
 loop { match pc {
 0x1048e18 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1048e23); f_010676ce(cpu); if cpu.returned_to != 0x1048e23 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x11a0f95u32; cpu.esi = v; }
  pc = 0x1048e29; continue;
 }
 0x1048e29 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0xffu8); }
  if cpu.zf { pc = 0x1048e76; continue; }
  pc = 0x1048e2e; continue;
 }
 0x1048e2e => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x10e4154u32), cpu.edi); }
  if !cpu.zf { pc = 0x1048e73; continue; }
  pc = 0x1048e43; continue;
 }
 0x1048e43 => {
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
  { cpu.sub32(cpu.ebx, 0x240u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1048e73; continue; }
  pc = 0x1048e71; continue;
 }
 0x1048e71 => {
  pc = 0x1048e78; continue;
 }
 0x1048e73 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x1048e29; continue;
 }
 0x1048e76 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1048e78 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01048e7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1048e7a); }
 let mut pc: u32 = 0x1048e7a;
 loop { match pc {
 0x1048e7a => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x1au32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3dbbu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3dbdu32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
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
  if (cpu.cf || cpu.zf) { pc = 0x1048eec; continue; }
  pc = 0x1048eea; continue;
 }
 0x1048eea => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  pc = 0x1048eec; continue;
 }
 0x1048eec => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x1048ef4; continue; }
  pc = 0x1048ef2; continue;
 }
 0x1048ef2 => {
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  pc = 0x1048ef4; continue;
 }
 0x1048ef4 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 8u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x400u32); }
  if cpu.cf { pc = 0x1048f03; continue; }
  pc = 0x1048eff; continue;
 }
 0x1048eff => {
  { let v: u8 = 0x2eu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1048f0b; continue;
 }
 0x1048f03 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e4028u32)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1048f0b; continue;
 }
 0x1048f0b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10c052cu32, v); }
  { let v: u32 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4130u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01048f24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1048f24); }
 let mut pc: u32 = 0x1048f24;
 loop { match pc {
 0x1048f24 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r16(0x22u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0x24u32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 7u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x1048f3d); f_010676ce(cpu); if cpu.returned_to != 0x1048f3d { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e40c0u32, v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x20u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u8 = cpu.r8(0x1152e85u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1128d9au32, v); }
  { let v: u32 = 0x11a0f95u32; cpu.esi = v; }
  pc = 0x1048f61; continue;
 }
 0x1048f61 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x10490c3; continue; }
  pc = 0x1048f6c; continue;
 }
 0x1048f6c => {
  { let v: u8 = 0xb8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1128d9au32)); }
  if cpu.zf { cpu.poll(); pc = 0x1048f61; continue; }
  pc = 0x1048f88; continue;
 }
 0x1048f88 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x1048f61; continue; }
  pc = 0x1048f8e; continue;
 }
 0x1048f8e => {
  { cpu.and8(cpu.r8(0x62u32.wrapping_add(cpu.edi)), 0x40u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1048f61; continue; }
  pc = 0x1048f94; continue;
 }
 0x1048f94 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.edi)), 0x40u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1048f61; continue; }
  pc = 0x1048f9a; continue;
 }
 0x1048f9a => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x1048f61; continue; }
  pc = 0x1048fa0; continue;
 }
 0x1048fa0 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x7u8); }
  if cpu.zf { cpu.poll(); pc = 0x1048f61; continue; }
  pc = 0x1048fa6; continue;
 }
 0x1048fa6 => {
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.edi)), 0x12u8); }
  if cpu.zf { cpu.poll(); pc = 0x1048f61; continue; }
  pc = 0x1048fac; continue;
 }
 0x1048fac => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x22u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x20u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1048f61; continue; }
  pc = 0x1048fc6; continue;
 }
 0x1048fc6 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x24u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x20u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1048f61; continue; }
  pc = 0x1048fe0; continue;
 }
 0x1048fe0 => {
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
  if !cpu.zf { pc = 0x104901c; continue; }
  pc = 0x1049012; continue;
 }
 0x1049012 => {
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x1049019; continue; }
  pc = 0x1049017; continue;
 }
 0x1049017 => {
  { let v: u8 = 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1049019; continue;
 }
 0x1049019 => {
  { let v: u8 = cpu.shr8((cpu.eax as u8), 2u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x104901c; continue;
 }
 0x104901c => {
  { let v = (cpu.eax as u8); cpu.mul8(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x1048f61; continue; }
  pc = 0x1049029; continue;
 }
 0x1049029 => {
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1048f61; continue; }
  pc = 0x1049031; continue;
 }
 0x1049031 => {
  cpu.push(0x1049036); f_0105f215(cpu); if cpu.returned_to != 0x1049036 { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x8u32); cpu.ecx = v; }
  cpu.push(0x1049043); f_0105f215(cpu); if cpu.returned_to != 0x1049043 { return; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x8u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x22u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1049071; continue; }
  pc = 0x1049064; continue;
 }
 0x1049064 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x28u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x104907c; continue;
 }
 0x1049071 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x28u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x2cu32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x104907c; continue;
 }
 0x104907c => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x24u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1049092; continue; }
  pc = 0x1049085; continue;
 }
 0x1049085 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x104909d; continue;
 }
 0x1049092 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2eu32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x104909d; continue;
 }
 0x104909d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x62u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10490aa); f_01069fe6(cpu); if cpu.returned_to != 0x10490aa { return; }
  cpu.push(0x10490af); f_0106a04e(cpu); if cpu.returned_to != 0x10490af { return; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x10490bc; continue; }
  pc = 0x10490b5; continue;
 }
 0x10490b5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.esi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  pc = 0x10490bc; continue;
 }
 0x10490bc => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.poll(); pc = 0x1048f61; continue;
 }
 0x10490c3 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010490c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10490c5); }
 let mut pc: u32 = 0x10490c5;
 loop { match pc {
 0x10490c5 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)) & 0x3u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x17u32.wrapping_add(cpu.ebp)), 0xefu8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10490d6; continue; }
  pc = 0x10490d2; continue;
 }
 0x10490d2 => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)).wrapping_add(0x10u8); cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10490d6; continue;
 }
 0x10490d6 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1049113; continue; }
  pc = 0x10490dc; continue;
 }
 0x10490dc => {
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1049105; continue; }
  pc = 0x10490e2; continue;
 }
 0x10490e2 => {
  { let v: u16 = 0x2au16; cpu.w16(0x10e3e93u32, v); }
  { cpu.sub16(cpu.r16(0x26u32.wrapping_add(cpu.ebp)), 0x5u16); }
  if (cpu.sf == cpu.of) { pc = 0x1049113; continue; }
  pc = 0x10490f3; continue;
 }
 0x10490f3 => {
  { let v: u16 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e95u32, v); }
  cpu.push(0x1049103); f_01049383(cpu); if cpu.returned_to != 0x1049103 { return; }
  pc = 0x1049113; continue;
 }
 0x1049105 => {
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  cpu.push(0x1049113); f_01049383(cpu); if cpu.returned_to != 0x1049113 { return; }
  pc = 0x1049113; continue;
 }
 0x1049113 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x26u32.wrapping_add(cpu.ebp))); cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1049129); f_0104f36d(cpu); if cpu.returned_to != 0x1049129 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10f714bu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x1049154; continue; }
  pc = 0x104913d; continue;
 }
 0x104913d => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049154 => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x1049187; continue; }
  pc = 0x1049180; continue;
 }
 0x1049180 => {
  { let v: u32 = 0x14e1u32; cpu.ecx = v; }
  pc = 0x104919a; continue;
 }
 0x1049187 => {
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x1049194; continue; }
  pc = 0x104918d; continue;
 }
 0x104918d => {
  { let v: u32 = 0x1531u32; cpu.ecx = v; }
  pc = 0x104919a; continue;
 }
 0x1049194 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x656u32); cpu.ecx = v; }
  pc = 0x104919a; continue;
 }
 0x104919a => {
  { let v: u16 = 0x1u16; cpu.w16(0x10c052cu32, v); }
  { let v: u32 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x10491be); cpu.call(t); if cpu.returned_to != 0x10491be { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4130u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10491cd); f_010366a8(cpu); if cpu.returned_to != 0x10491cd { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x10491f1; continue; }
  pc = 0x10491d6; continue;
 }
 0x10491d6 => {
  { let v: u8 = cpu.r8(0x21u32.wrapping_add(cpu.ebp)) & 0xfbu8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3e9bu32).wrapping_add(0x6u16); cpu.w16(0x10e3e9bu32, v); }
  { let v: u16 = cpu.add16(cpu.r16(0x10e3e9du32), 0x4u16); cpu.w16(0x10e3e9du32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10491f0); f_010499ec(cpu); if cpu.returned_to != 0x10491f0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x10491f1; continue;
 }
 0x10491f1 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010491f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10491f3); }
 let mut pc: u32 = 0x10491f3;
 loop { match pc {
 0x10491f3 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1049201); f_01072d00(cpu); if cpu.returned_to != 0x1049201 { return; }
  if cpu.cf { pc = 0x1049216; continue; }
  pc = 0x1049203; continue;
 }
 0x1049203 => {
  { let v: u8 = 0x4u8; cpu.w8(0x110d736u32, v); }
  { let v: u16 = cpu.r16(0x112c96eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110d734u32, v); }
  pc = 0x1049216; continue;
 }
 0x1049216 => {
  { cpu.popad(); }
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)) & 0x3u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x17u32.wrapping_add(cpu.ebp)), 0xefu8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1049228; continue; }
  pc = 0x1049224; continue;
 }
 0x1049224 => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)).wrapping_add(0x10u8); cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1049228; continue;
 }
 0x1049228 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x26u32.wrapping_add(cpu.ebp))); cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x104923e); f_0104f36d(cpu); if cpu.returned_to != 0x104923e { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10f714bu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x1049269; continue; }
  pc = 0x1049252; continue;
 }
 0x1049252 => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049269 => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x656u32); cpu.ecx = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x10c052cu32, v); }
  { let v: u32 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4130u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10492b7); f_010366a8(cpu); if cpu.returned_to != 0x10492b7 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010492bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10492bb); }
 let mut pc: u32 = 0x10492bb;
 loop { match pc {
 0x10492bb => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10492c9); f_01072d00(cpu); if cpu.returned_to != 0x10492c9 { return; }
  if cpu.cf { pc = 0x10492de; continue; }
  pc = 0x10492cb; continue;
 }
 0x10492cb => {
  { let v: u8 = 0x5u8; cpu.w8(0x110d736u32, v); }
  { let v: u16 = cpu.r16(0x112c96eu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110d734u32, v); }
  pc = 0x10492de; continue;
 }
 0x10492de => {
  { cpu.popad(); }
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)) & 0x3u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x17u32.wrapping_add(cpu.ebp)), 0xefu8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10492f0; continue; }
  pc = 0x10492ec; continue;
 }
 0x10492ec => {
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)).wrapping_add(0x10u8); cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10492f0; continue;
 }
 0x10492f0 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x26u32.wrapping_add(cpu.ebp))); cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1049306); f_0104f36d(cpu); if cpu.returned_to != 0x1049306 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10f714bu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1fu32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x1049331; continue; }
  pc = 0x104931a; continue;
 }
 0x104931a => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049331 => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x179eu32); cpu.ecx = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x10c052cu32, v); }
  { let v: u32 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4130u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x104937f); f_010366a8(cpu); if cpu.returned_to != 0x104937f { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049383(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049383); }
 let mut pc: u32 = 0x1049383;
 loop { match pc {
 0x1049383 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x2u32; cpu.edi = v; }
  pc = 0x1049396; continue;
 }
 0x1049396 => {
  cpu.push(0x104939b); f_0104942f(cpu); if cpu.returned_to != 0x104939b { return; }
  if cpu.cf { pc = 0x104942d; continue; }
  pc = 0x10493a1; continue;
 }
 0x10493a1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x18u32.wrapping_add(cpu.ebp)), 0x80u8); }
  if !cpu.cf { pc = 0x10493b9; continue; }
  pc = 0x10493b1; continue;
 }
 0x10493b1 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x3u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10493b9; continue;
 }
 0x10493b9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x5u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x7u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub16(cpu.r16(0x10e3e93u32), 0x2au16); }
  if !cpu.zf { pc = 0x10493e9; continue; }
  pc = 0x10493e5; continue;
 }
 0x10493e5 => {
  { let v: u8 = 0x1u8; cpu.w8(0x7u32.wrapping_add(cpu.esi), v); }
  pc = 0x10493e9; continue;
 }
 0x10493e9 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  if cpu.zf { pc = 0x104942d; continue; }
  pc = 0x10493ee; continue;
 }
 0x10493ee => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(cpu.r16(0x10e3dbbu32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(cpu.r16(0x10e3dbdu32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x104941b; continue; }
  pc = 0x104940a; continue;
 }
 0x104940a => {
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x1049396; continue;
 }
 0x104941b => {
  { cpu.sub16((cpu.ecx as u16), 0xfff8u16); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x104940a; continue; }
  pc = 0x1049421; continue;
 }
 0x1049421 => {
  { cpu.sub16((cpu.edx as u16), 0x8u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x104940a; continue; }
  pc = 0x1049427; continue;
 }
 0x1049427 => {
  { cpu.sub16((cpu.edx as u16), 0xfff8u16); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x104940a; continue; }
  pc = 0x104942d; continue;
 }
 0x104942d => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104942f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104942f); }
 let mut pc: u32 = 0x104942f;
 loop { match pc {
 0x104942f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x115e424u32; cpu.esi = v; }
  { let v: u32 = 0x190u32; cpu.ecx = v; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  pc = 0x104943f; continue;
 }
 0x104943f => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x104944c; continue; }
  pc = 0x1049445; continue;
 }
 0x1049445 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x104943f; continue; }
  pc = 0x1049449; continue;
 }
 0x1049449 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104944c => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104944f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104944f); }
 let mut pc: u32 = 0x104944f;
 loop { match pc {
 0x104944f => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = 0x190u32; cpu.ecx = v; }
  { let v: u32 = 0x115e424u32; cpu.edi = v; }
  pc = 0x104945e; continue;
 }
 0x104945e => {
  { cpu.stos(4, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0x6u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x104945e; continue; }
  pc = 0x1049464; continue;
 }
 0x1049464 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049465(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049465); }
 let mut pc: u32 = 0x1049465;
 loop { match pc {
 0x1049465 => {
  { let v: u32 = 0x115e424u32; cpu.esi = v; }
  { let v: u32 = 0x190u32; cpu.ecx = v; }
  pc = 0x104946f; continue;
 }
 0x104946f => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x104951d; continue; }
  pc = 0x104947a; continue;
 }
 0x104947a => {
  { cpu.sub8(cpu.r8(0x6u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1049488; continue; }
  pc = 0x1049480; continue;
 }
 0x1049480 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x6u32.wrapping_add(cpu.esi))); cpu.w8(0x6u32.wrapping_add(cpu.esi), v); }
  pc = 0x104951d; continue;
 }
 0x1049488 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1049503; continue; }
  pc = 0x10494a3; continue;
 }
 0x10494a3 => {
  { cpu.sub16((cpu.ebx as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1049503; continue; }
  pc = 0x10494a9; continue;
 }
 0x10494a9 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1049503; continue; }
  pc = 0x10494b2; continue;
 }
 0x10494b2 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1049503; continue; }
  pc = 0x10494bb; continue;
 }
 0x10494bb => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.esi)) as u32; cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x1u8); }
  if cpu.zf { pc = 0x10494e1; continue; }
  pc = 0x10494c6; continue;
 }
 0x10494c6 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x65au32); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4130u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x10494f1; continue;
 }
 0x10494e1 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x65au32); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x10e4130u32, v); }
  pc = 0x10494f1; continue;
 }
 0x10494f1 => {
  { cpu.pushad(); }
  { let v: u16 = 0x1u16; cpu.w16(0x10c052cu32, v); }
  let t = cpu.r32(0x10e4308u32); cpu.push(0x1049501); cpu.call(t); if cpu.returned_to != 0x1049501 { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x1049503; continue;
 }
 0x1049503 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x5u32.wrapping_add(cpu.esi))); cpu.w8(0x5u32.wrapping_add(cpu.esi), v); }
  if !cpu.sf { pc = 0x104951d; continue; }
  pc = 0x1049508; continue;
 }
 0x1049508 => {
  { let v: u8 = 0x2u8; cpu.w8(0x5u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x4u32.wrapping_add(cpu.esi))); cpu.w8(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x4u32.wrapping_add(cpu.esi))); cpu.w8(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esi)), 0x7u8); }
  if (cpu.sf != cpu.of) { pc = 0x104951d; continue; }
  pc = 0x1049518; continue;
 }
 0x1049518 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x104951d; continue;
 }
 0x104951d => {
  { let v: u32 = cpu.add32(cpu.esi, 0xau32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x104946f; continue; }
  pc = 0x1049527; continue;
 }
 0x1049527 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049528(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049528); }
 let mut pc: u32 = 0x1049528;
 loop { match pc {
 0x1049528 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1049533); f_0105f215(cpu); if cpu.returned_to != 0x1049533 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1049543); f_0105f215(cpu); if cpu.returned_to != 0x1049543 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x1fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.eax as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x1049558); f_01072e0c(cpu); if cpu.returned_to != 0x1049558 { return; }
  cpu.push(0x104955d); f_01048d08(cpu); if cpu.returned_to != 0x104955d { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049561(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049561); }
 let mut pc: u32 = 0x1049561;
 loop { match pc {
 0x1049561 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if !cpu.zf { pc = 0x104956d; continue; }
  pc = 0x1049566; continue;
 }
 0x1049566 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x104956d; continue; }
  pc = 0x104956b; continue;
 }
 0x104956b => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104956d => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { cpu.poll(); pc = 0x104956b; continue; }
  pc = 0x1049571; continue;
 }
 0x1049571 => {
  cpu.push(0x1049576); f_010499c1(cpu); if cpu.returned_to != 0x1049576 { return; }
  if cpu.cf { pc = 0x104968a; continue; }
  pc = 0x104957c; continue;
 }
 0x104957c => {
  cpu.push(0x1049581); f_010497bf(cpu); if cpu.returned_to != 0x1049581 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e40c0u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x1049591); f_01072d00(cpu); if cpu.returned_to != 0x1049591 { return; }
  if cpu.cf { pc = 0x10495c7; continue; }
  pc = 0x1049593; continue;
 }
 0x1049593 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1128d9fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x10495c7; continue; }
  pc = 0x104959e; continue;
 }
 0x104959e => {
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x10495c7; continue; }
  pc = 0x10495a7; continue;
 }
 0x10495a7 => {
  cpu.push(0x10495ac); f_01012780(cpu); if cpu.returned_to != 0x10495ac { return; }
  cpu.push(0x10495b1); f_010356fe(cpu); if cpu.returned_to != 0x10495b1 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1128da2u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if cpu.zf { pc = 0x10495c7; continue; }
  pc = 0x10495be; continue;
 }
 0x10495be => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x10495c7; continue; }
  pc = 0x10495c2; continue;
 }
 0x10495c2 => {
  cpu.push(0x10495c7); f_01012b87(cpu); if cpu.returned_to != 0x10495c7 { return; }
  pc = 0x10495c7; continue;
 }
 0x10495c7 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10495d3); f_01049a2e(cpu); if cpu.returned_to != 0x10495d3 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x104956b; continue; }
  pc = 0x10495da; continue;
 }
 0x10495da => {
  { cpu.sub16(cpu.r16(0x10e3e93u32), 0x2au16); }
  if !cpu.zf { pc = 0x1049606; continue; }
  pc = 0x10495e4; continue;
 }
 0x10495e4 => {
  { let v: u8 = 0x15u8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.eax & cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx & cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.edx & cpu.edi; cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1049603); f_01002131(cpu); if cpu.returned_to != 0x1049603 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1049652; continue;
 }
 0x1049606 => {
  { cpu.sub16(cpu.r16(0x10e3e93u32), 0x29u16); }
  if !cpu.zf { pc = 0x1049632; continue; }
  pc = 0x1049610; continue;
 }
 0x1049610 => {
  { let v: u8 = 0x25u8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.eax & cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx & cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.edx & cpu.edi; cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x104962f); f_01002131(cpu); if cpu.returned_to != 0x104962f { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1049652; continue;
 }
 0x1049632 => {
  { let v: u8 = 0x3u8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.eax & cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx & cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.edx & cpu.edi; cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1049651); f_01002131(cpu); if cpu.returned_to != 0x1049651 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1049652; continue;
 }
 0x1049652 => {
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x168u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10f714bu32; cpu.ebx = v; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  pc = 0x104968a; continue;
 }
 0x104968a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104968b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104968b); }
 let mut pc: u32 = 0x104968b;
 loop { match pc {
 0x104968b => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if !cpu.zf { pc = 0x1049697; continue; }
  pc = 0x1049690; continue;
 }
 0x1049690 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x1049697; continue; }
  pc = 0x1049695; continue;
 }
 0x1049695 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049697 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { cpu.poll(); pc = 0x1049695; continue; }
  pc = 0x104969b; continue;
 }
 0x104969b => {
  cpu.push(0x10496a0); f_010499c1(cpu); if cpu.returned_to != 0x10496a0 { return; }
  if cpu.cf { pc = 0x1049728; continue; }
  pc = 0x10496a6; continue;
 }
 0x10496a6 => {
  cpu.push(0x10496ab); f_010497bf(cpu); if cpu.returned_to != 0x10496ab { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e40c0u32, v); }
  cpu.push(0x10496b9); f_01072d00(cpu); if cpu.returned_to != 0x10496b9 { return; }
  if cpu.cf { pc = 0x10496bb; continue; }
  pc = 0x10496bb; continue;
 }
 0x10496bb => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10496c5); f_01049a2e(cpu); if cpu.returned_to != 0x10496c5 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x1049695; continue; }
  pc = 0x10496cc; continue;
 }
 0x10496cc => {
  { let v: u8 = 0x42u8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.eax & cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx & cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.edx & cpu.edi; cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10496eb); f_01002131(cpu); if cpu.returned_to != 0x10496eb { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
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
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10f714bu32; cpu.ebx = v; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  pc = 0x1049728; continue;
 }
 0x1049728 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049729(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049729); }
 let mut pc: u32 = 0x1049729;
 loop { match pc {
 0x1049729 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if !cpu.zf { pc = 0x1049735; continue; }
  pc = 0x104972e; continue;
 }
 0x104972e => {
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x1049735; continue; }
  pc = 0x1049733; continue;
 }
 0x1049733 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049735 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { cpu.poll(); pc = 0x1049733; continue; }
  pc = 0x1049739; continue;
 }
 0x1049739 => {
  cpu.push(0x104973e); f_010499c1(cpu); if cpu.returned_to != 0x104973e { return; }
  if cpu.cf { pc = 0x10497be; continue; }
  pc = 0x1049740; continue;
 }
 0x1049740 => {
  cpu.push(0x1049745); f_010497bf(cpu); if cpu.returned_to != 0x1049745 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e40c0u32, v); }
  cpu.push(0x1049753); f_01072d00(cpu); if cpu.returned_to != 0x1049753 { return; }
  if cpu.cf { pc = 0x1049755; continue; }
  pc = 0x1049755; continue;
 }
 0x1049755 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x104975f); f_01049a2e(cpu); if cpu.returned_to != 0x104975f { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x1049733; continue; }
  pc = 0x1049766; continue;
 }
 0x1049766 => {
  { let v: u8 = 0x2u8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x10u8; cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.eax & cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx & cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.edx & cpu.edi; cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1049785); f_01002131(cpu); if cpu.returned_to != 0x1049785 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x168u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10f714bu32; cpu.ebx = v; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  pc = 0x10497be; continue;
 }
 0x10497be => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010497bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10497bf); }
 let mut pc: u32 = 0x10497bf;
 loop { match pc {
 0x10497bf => {
  { cpu.pushad(); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1129673u32; cpu.w32(0x10e404cu32, v); }
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  cpu.push(0x10497d9); f_01068dfc(cpu); if cpu.returned_to != 0x10497d9 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.cf { pc = 0x1049824; continue; }
  pc = 0x10497dc; continue;
 }
 0x10497dc => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1049824; continue; }
  pc = 0x10497e2; continue;
 }
 0x10497e2 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1049824; continue; }
  pc = 0x10497e8; continue;
 }
 0x10497e8 => {
  { cpu.sub8(cpu.r8(0x2u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1049824; continue; }
  pc = 0x10497ee; continue;
 }
 0x10497ee => {
  { cpu.sub8(cpu.r8(0x5cu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1049824; continue; }
  pc = 0x10497f4; continue;
 }
 0x10497f4 => {
  { let v: u8 = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5du32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x1049824; continue; }
  pc = 0x10497fc; continue;
 }
 0x10497fc => {
  { let v: u32 = 0x48u32; cpu.eax = v; }
  { let v = cpu.r8(0x5du32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x11757a4u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x3du32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1049824; continue; }
  pc = 0x104980f; continue;
 }
 0x104980f => {
  { cpu.sub8(cpu.r8(0x3fu32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x104981c; continue; }
  pc = 0x1049815; continue;
 }
 0x1049815 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x104981b); f_01049826(cpu); if cpu.returned_to != 0x104981b { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x104981c; continue;
 }
 0x104981c => {
  { let v: u8 = 0x44u8; cpu.w8(0x3fu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.eax)), 0xfeu8); cpu.w8(0x9u32.wrapping_add(cpu.eax), v); }
  pc = 0x1049824; continue;
 }
 0x1049824 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049826(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049826); }
 let mut pc: u32 = 0x1049826;
 loop { match pc {
 0x1049826 => {
  { let v = cpu.r32(0x10e3f84u32); cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e3f84u32, v); }
  { let v: u32 = 0x13u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  pc = 0x1049839; continue;
 }
 0x1049839 => {
  { let v: u32 = 0xb800u32; cpu.eax = v; }
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x104988a; continue; }
  pc = 0x1049843; continue;
 }
 0x1049843 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  cpu.push(0x104985a); f_0105f215(cpu); if cpu.returned_to != 0x104985a { return; }
  { let v: u32 = cpu.eax & 0x70u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x38u32); cpu.eax = v; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.imul8(v); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebx = v; }
  cpu.push(0x104986f); f_0105f215(cpu); if cpu.returned_to != 0x104986f { return; }
  { let v: u32 = cpu.eax & 0x70u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x38u32); cpu.eax = v; }
  { let v = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.imul8(v); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  cpu.push(0x1049883); f_01069fe6(cpu); if cpu.returned_to != 0x1049883 { return; }
  cpu.push(0x1049888); f_0106a04e(cpu); if cpu.returned_to != 0x1049888 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x104988a; continue;
 }
 0x104988a => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1049839; continue; }
  pc = 0x104988c; continue;
 }
 0x104988c => {
  { let v: u32 = cpu.pop(); cpu.w32(0x10e3f84u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049895(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049895); }
 let mut pc: u32 = 0x1049895;
 loop { match pc {
 0x1049895 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if !cpu.zf { pc = 0x10498a1; continue; }
  pc = 0x104989a; continue;
 }
 0x104989a => {
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x10498a1; continue; }
  pc = 0x104989f; continue;
 }
 0x104989f => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10498a1 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if cpu.zf { cpu.poll(); pc = 0x104989f; continue; }
  pc = 0x10498a5; continue;
 }
 0x10498a5 => {
  cpu.push(0x10498aa); f_010499c1(cpu); if cpu.returned_to != 0x10498aa { return; }
  if cpu.cf { pc = 0x10499c0; continue; }
  pc = 0x10498b0; continue;
 }
 0x10498b0 => {
  cpu.push(0x10498b5); f_010497bf(cpu); if cpu.returned_to != 0x10498b5 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e40c0u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10498c5); f_01072d00(cpu); if cpu.returned_to != 0x10498c5 { return; }
  if cpu.cf { pc = 0x10498fd; continue; }
  pc = 0x10498c7; continue;
 }
 0x10498c7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1128da1u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x10498fd; continue; }
  pc = 0x10498d2; continue;
 }
 0x10498d2 => {
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x10498fd; continue; }
  pc = 0x10498db; continue;
 }
 0x10498db => {
  cpu.push(0x10498e0); f_01012780(cpu); if cpu.returned_to != 0x10498e0 { return; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10498e6); f_010356fe(cpu); if cpu.returned_to != 0x10498e6 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1128da2u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if cpu.zf { pc = 0x10498fd; continue; }
  pc = 0x10498f4; continue;
 }
 0x10498f4 => {
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x10498fd; continue; }
  pc = 0x10498f8; continue;
 }
 0x10498f8 => {
  cpu.push(0x10498fd); f_01012b87(cpu); if cpu.returned_to != 0x10498fd { return; }
  pc = 0x10498fd; continue;
 }
 0x10498fd => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1049909); f_01049a2e(cpu); if cpu.returned_to != 0x1049909 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.cf { cpu.poll(); pc = 0x104989f; continue; }
  pc = 0x1049910; continue;
 }
 0x1049910 => {
  { let v: u8 = 0x0u8; cpu.w8(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x21u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x21u32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  { let v: u32 = 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.eax & cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx & cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.edx & cpu.edi; cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1049930); f_01002131(cpu); if cpu.returned_to != 0x1049930 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x168u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.ebp), v); }
  { cpu.popad(); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.edx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 7u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x400u16); }
  if (cpu.sf != cpu.of) { pc = 0x1049969; continue; }
  pc = 0x1049965; continue;
 }
 0x1049965 => {
  { let v: u16 = 0x3ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1049969; continue;
 }
 0x1049969 => {
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e4028u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x4u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1049983; continue; }
  pc = 0x104997e; continue;
 }
 0x104997e => {
  { let v: u32 = 0x4u32; cpu.esi = v; }
  pc = 0x1049983; continue;
 }
 0x1049983 => {
  { let v: u32 = cpu.r32(0x1128d58u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.ebx as u16); cpu.div16(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10f714bu32; cpu.ebx = v; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1fu32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  pc = 0x10499c0; continue;
 }
 0x10499c0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010499c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10499c1); }
 let mut pc: u32 = 0x10499c1;
 loop { match pc {
 0x10499c1 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010499c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10499c3); }
 let mut pc: u32 = 0x10499c3;
 loop { match pc {
 0x10499c3 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub((cpu.edx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.adc16((cpu.edx as u16), (cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x10499e9; continue; }
  pc = 0x10499e0; continue;
 }
 0x10499e0 => {
  { cpu.sub16((cpu.eax as u16), 0x100u16); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10499e9; continue; }
  pc = 0x10499e6; continue;
 }
 0x10499e6 => {
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10499e9 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010499ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10499ec); }
 let mut pc: u32 = 0x10499ec;
 loop { match pc {
 0x10499ec => {
  { let v: u16 = cpu.r16(0x10e3e9bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e9du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x21u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1049a2d; continue; }
  pc = 0x1049a0d; continue;
 }
 0x1049a0d => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.push(0x1049a15); f_0105f215(cpu); if cpu.returned_to != 0x1049a15 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x161u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let a = (cpu.ecx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x6u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42f0u32); cpu.push(0x1049a2d); cpu.call(t); if cpu.returned_to != 0x1049a2d { return; }
  pc = 0x1049a2d; continue;
 }
 0x1049a2d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049a2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049a2e); }
 let mut pc: u32 = 0x1049a2e;
 loop { match pc {
 0x1049a2e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shl8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) | ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1128d9bu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x20u8); }
  if cpu.zf { pc = 0x1049a70; continue; }
  pc = 0x1049a50; continue;
 }
 0x1049a50 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1049a70; continue; }
  pc = 0x1049a56; continue;
 }
 0x1049a56 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1049a5f; continue;
 }
 0x1049a5f => {
  { let v: u16 = (cpu.ebx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1049a6c); f_01048938(cpu); if cpu.returned_to != 0x1049a6c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if !cpu.cf { pc = 0x1049a84; continue; }
  pc = 0x1049a6f; continue;
 }
 0x1049a6f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049a70 => {
  { let v: u8 = cpu.r8(0x10e3f47u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1049a7f; continue; }
  pc = 0x1049a7c; continue;
 }
 0x1049a7c => {
  { let v: u8 = cpu.r8(0x63u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1049a7f; continue;
 }
 0x1049a7f => {
  { let v: u8 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x1049a5f; continue;
 }
 0x1049a84 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x17u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x16u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x1au32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x29u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x1128d9bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x10ee980u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1049b0c; continue; }
  pc = 0x1049ac3; continue;
 }
 0x1049ac3 => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1049ad2; continue; }
  pc = 0x1049acd; continue;
 }
 0x1049acd => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1049ad2; continue;
 }
 0x1049ad2 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x1049af3; continue; }
  pc = 0x1049ada; continue;
 }
 0x1049ada => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049af3 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049b0c => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1049b1e; continue; }
  pc = 0x1049b19; continue;
 }
 0x1049b19 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.neg8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1049b1e; continue;
 }
 0x1049b1e => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x1049b3f; continue; }
  pc = 0x1049b26; continue;
 }
 0x1049b26 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049b3f => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sar16((cpu.ebx as u16), 1u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x6u8; cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049b58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049b58); }
 let mut pc: u32 = 0x1049b58;
 loop { match pc {
 0x1049b58 => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1049b75; continue; }
  pc = 0x1049b70; continue;
 }
 0x1049b70 => {
  { cpu.sub8(cpu.r8(0x16u32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1049b78; continue; }
  pc = 0x1049b75; continue;
 }
 0x1049b75 => {
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1049b78; continue;
 }
 0x1049b78 => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x1049b82; continue; }
  pc = 0x1049b7f; continue;
 }
 0x1049b7f => {
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1049b82; continue;
 }
 0x1049b82 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x1049b8f; continue;
 }
 0x1049b8f => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.esi as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1049b9f; continue; }
  pc = 0x1049b94; continue;
 }
 0x1049b94 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x10e3dbdu32, v); }
  pc = 0x1049b9f; continue;
 }
 0x1049b9f => {
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1049b8f; continue; }
  pc = 0x1049ba3; continue;
 }
 0x1049ba3 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub((cpu.ecx as u16)); cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.ebp)), (cpu.ecx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049bb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049bb8); }
 let mut pc: u32 = 0x1049bb8;
 loop { match pc {
 0x1049bb8 => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1049bd5; continue; }
  pc = 0x1049bd0; continue;
 }
 0x1049bd0 => {
  { cpu.sub8(cpu.r8(0x16u32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1049bd8; continue; }
  pc = 0x1049bd5; continue;
 }
 0x1049bd5 => {
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1049bd8; continue;
 }
 0x1049bd8 => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x1049be2; continue; }
  pc = 0x1049bdf; continue;
 }
 0x1049bdf => {
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1049be2; continue;
 }
 0x1049be2 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x1049bef; continue;
 }
 0x1049bef => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.esi as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1049bfa; continue; }
  pc = 0x1049bf4; continue;
 }
 0x1049bf4 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1049bfa; continue;
 }
 0x1049bfa => {
  { let v: u16 = cpu.r16(0x10e3dbdu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x10e3dbdu32, v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1049bef; continue; }
  pc = 0x1049c03; continue;
 }
 0x1049c03 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub((cpu.ecx as u16)); cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.ebp)), (cpu.ecx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049c18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049c18); }
 let mut pc: u32 = 0x1049c18;
 loop { match pc {
 0x1049c18 => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1049c35; continue; }
  pc = 0x1049c30; continue;
 }
 0x1049c30 => {
  { cpu.sub8(cpu.r8(0x16u32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1049c38; continue; }
  pc = 0x1049c35; continue;
 }
 0x1049c35 => {
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1049c38; continue;
 }
 0x1049c38 => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x1049c42; continue; }
  pc = 0x1049c3f; continue;
 }
 0x1049c3f => {
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1049c42; continue;
 }
 0x1049c42 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x1049c4f; continue;
 }
 0x1049c4f => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.esi as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1049c5f; continue; }
  pc = 0x1049c54; continue;
 }
 0x1049c54 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x10e3dbdu32, v); }
  pc = 0x1049c5f; continue;
 }
 0x1049c5f => {
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1049c4f; continue; }
  pc = 0x1049c63; continue;
 }
 0x1049c63 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub((cpu.ecx as u16)); cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.ebp)), (cpu.ecx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049c78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049c78); }
 let mut pc: u32 = 0x1049c78;
 loop { match pc {
 0x1049c78 => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax as u8 as i8 as i16 as u16; cpu.set_ax(v); }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.edx as u8), 4u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1049c95; continue; }
  pc = 0x1049c90; continue;
 }
 0x1049c90 => {
  { cpu.sub8(cpu.r8(0x16u32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1049c98; continue; }
  pc = 0x1049c95; continue;
 }
 0x1049c95 => {
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1049c98; continue;
 }
 0x1049c98 => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x1049ca2; continue; }
  pc = 0x1049c9f; continue;
 }
 0x1049c9f => {
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1049ca2; continue;
 }
 0x1049ca2 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x1049caf; continue;
 }
 0x1049caf => {
  { let v: u16 = cpu.sub16((cpu.edx as u16), (cpu.esi as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1049cba; continue; }
  pc = 0x1049cb4; continue;
 }
 0x1049cb4 => {
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1049cba; continue;
 }
 0x1049cba => {
  { let v: u16 = cpu.r16(0x10e3dbdu32).wrapping_add((cpu.eax as u16)); cpu.w16(0x10e3dbdu32, v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1049caf; continue; }
  pc = 0x1049cc3; continue;
 }
 0x1049cc3 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.ebp)).wrapping_sub((cpu.ecx as u16)); cpu.w16(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.ebp)), (cpu.ecx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049cd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049cd8); }
 let mut pc: u32 = 0x1049cd8;
 loop { match pc {
 0x1049cd8 => {
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1049d28; continue; }
  pc = 0x1049ce9; continue;
 }
 0x1049ce9 => {
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1049cf4; continue; }
  pc = 0x1049cf1; continue;
 }
 0x1049cf1 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1049cf4; continue;
 }
 0x1049cf4 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x1049d12; continue; }
  pc = 0x1049cf9; continue;
 }
 0x1049cf9 => {
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1049d0b; continue; }
  pc = 0x1049d01; continue;
 }
 0x1049d01 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1049d70; continue; }
  pc = 0x1049d09; continue;
 }
 0x1049d09 => {
  pc = 0x1049d6d; continue;
 }
 0x1049d0b => {
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1049d73; continue; }
  pc = 0x1049d10; continue;
 }
 0x1049d10 => {
  pc = 0x1049d70; continue;
 }
 0x1049d12 => {
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1049d21; continue; }
  pc = 0x1049d17; continue;
 }
 0x1049d17 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x1049d6a; continue; }
  pc = 0x1049d1f; continue;
 }
 0x1049d1f => {
  pc = 0x1049d6d; continue;
 }
 0x1049d21 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x1049d76; continue; }
  pc = 0x1049d26; continue;
 }
 0x1049d26 => {
  pc = 0x1049d73; continue;
 }
 0x1049d28 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1049d36; continue; }
  pc = 0x1049d33; continue;
 }
 0x1049d33 => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1049d36; continue;
 }
 0x1049d36 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.cf { pc = 0x1049d54; continue; }
  pc = 0x1049d3b; continue;
 }
 0x1049d3b => {
  { let v: u16 = cpu.sar16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1049d4d; continue; }
  pc = 0x1049d43; continue;
 }
 0x1049d43 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1049d7c; continue; }
  pc = 0x1049d4b; continue;
 }
 0x1049d4b => {
  pc = 0x1049d7f; continue;
 }
 0x1049d4d => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1049d7c; continue; }
  pc = 0x1049d52; continue;
 }
 0x1049d52 => {
  pc = 0x1049d79; continue;
 }
 0x1049d54 => {
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1049d63; continue; }
  pc = 0x1049d59; continue;
 }
 0x1049d59 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x1049d6a; continue; }
  pc = 0x1049d61; continue;
 }
 0x1049d61 => {
  pc = 0x1049d7f; continue;
 }
 0x1049d63 => {
  { cpu.sub16((cpu.eax as u16), (cpu.ecx as u16)); }
  if (cpu.sf != cpu.of) { pc = 0x1049d76; continue; }
  pc = 0x1049d68; continue;
 }
 0x1049d68 => {
  pc = 0x1049d79; continue;
 }
 0x1049d6a => {
  { let v: u8 = cpu.sub8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049d6d => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049d70 => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049d73 => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049d76 => {
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049d79 => {
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049d7c => {
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049d7f => {
  { let v: u8 = 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049d82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049d82); }
 let mut pc: u32 = 0x1049d82;
 loop { match pc {
 0x1049d82 => {
  { let v: u32 = 0x1167ed4u32; cpu.edi = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  pc = 0x1049d8c; continue;
 }
 0x1049d8c => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if !cpu.zf { pc = 0x1049d94; continue; }
  pc = 0x1049d92; continue;
 }
 0x1049d92 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049d94 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1049d8c; continue; }
  pc = 0x1049d99; continue;
 }
 0x1049d99 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049d9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049d9b); }
 let mut pc: u32 = 0x1049d9b;
 loop { match pc {
 0x1049d9b => {
  { let v: u32 = 0x1167ed4u32; cpu.ebp = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1049da9; continue;
 }
 0x1049da9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.ebp, 0x8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1049da9; continue; }
  pc = 0x1049db2; continue;
 }
 0x1049db2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049db3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049db3); }
 let mut pc: u32 = 0x1049db3;
 loop { match pc {
 0x1049db3 => {
  cpu.push(0x1049db8); f_01049d82(cpu); if cpu.returned_to != 0x1049db8 { return; }
  if cpu.cf { pc = 0x1049dfd; continue; }
  pc = 0x1049dba; continue;
 }
 0x1049dba => {
  { let v: u8 = 0x0u8; cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1049dc5); f_0104f36d(cpu); if cpu.returned_to != 0x1049dc5 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1049deb; continue; }
  pc = 0x1049dd1; continue;
 }
 0x1049dd1 => {
  { cpu.sub8((cpu.eax as u8), 0x2au8); }
  if cpu.zf { pc = 0x1049deb; continue; }
  pc = 0x1049dd5; continue;
 }
 0x1049dd5 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x1049deb; continue; }
  pc = 0x1049dd9; continue;
 }
 0x1049dd9 => {
  { cpu.sub8((cpu.eax as u8), 0x28u8); }
  if cpu.zf { pc = 0x1049deb; continue; }
  pc = 0x1049ddd; continue;
 }
 0x1049ddd => {
  { cpu.sub8((cpu.eax as u8), 0x27u8); }
  if cpu.zf { pc = 0x1049deb; continue; }
  pc = 0x1049de1; continue;
 }
 0x1049de1 => {
  { cpu.sub8((cpu.eax as u8), 0x16u8); }
  if cpu.zf { pc = 0x1049def; continue; }
  pc = 0x1049de5; continue;
 }
 0x1049de5 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1049dfe; continue; }
  pc = 0x1049de9; continue;
 }
 0x1049de9 => {
  pc = 0x1049def; continue;
 }
 0x1049deb => {
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x1049def; continue;
 }
 0x1049def => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x1049dfd; continue;
 }
 0x1049dfd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049dfe => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049e01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049e01); }
 let mut pc: u32 = 0x1049e01;
 loop { match pc {
 0x1049e01 => {
  { let v: u32 = 0x1167ed4u32; cpu.ebp = v; }
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edi = v; }
  pc = 0x1049e11; continue;
 }
 0x1049e11 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x1049e21; continue; }
  pc = 0x1049e18; continue;
 }
 0x1049e18 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1049e1f); f_01049e2d(cpu); if cpu.returned_to != 0x1049e1f { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1049e21; continue;
 }
 0x1049e21 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1049e11; continue; }
  pc = 0x1049e26; continue;
 }
 0x1049e26 => {
  { let v: u32 = cpu.edi; cpu.w32(0x10e4010u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049e2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049e2d); }
 let mut pc: u32 = 0x1049e2d;
 loop { match pc {
 0x1049e2d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xfff8u16); }
  if (cpu.sf != cpu.of) { pc = 0x1049ec5; continue; }
  pc = 0x1049e42; continue;
 }
 0x1049e42 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1049ec5; continue; }
  pc = 0x1049e4b; continue;
 }
 0x1049e4b => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfff8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1049ec5; continue; }
  pc = 0x1049e5c; continue;
 }
 0x1049e5c => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1049ec5; continue; }
  pc = 0x1049e65; continue;
 }
 0x1049e65 => {
  { cpu.sub8(cpu.r8(0x11b4c6cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1049e86; continue; }
  pc = 0x1049e6e; continue;
 }
 0x1049e6e => {
  { cpu.sub32(cpu.edi, 0x10e55f8u32); }
  if (cpu.sf == cpu.of) { pc = 0x1049e86; continue; }
  pc = 0x1049e76; continue;
 }
 0x1049e76 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e401cu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x10e68bau32); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  if (cpu.sf != cpu.of) { pc = 0x1049e8f; continue; }
  pc = 0x1049e86; continue;
 }
 0x1049e86 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11b4c6cu32, v); }
  pc = 0x1049ec5; continue;
 }
 0x1049e8f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { cpu.stos(2, false); }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1049eb6; continue; }
  pc = 0x1049ead; continue;
 }
 0x1049ead => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x1123070u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1049ebd; continue;
 }
 0x1049eb6 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x1123082u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1049ebd; continue;
 }
 0x1049ebd => {
  { cpu.stos(2, false); }
  { let v: u16 = 0xffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x1049ec5; continue;
 }
 0x1049ec5 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { pc = 0x1049ef4; continue; }
  pc = 0x1049ece; continue;
 }
 0x1049ece => {
  { let v: u8 = cpu.inc8(cpu.r8(0x4u32.wrapping_add(cpu.ebp))); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1049ee1; continue; }
  pc = 0x1049ed8; continue;
 }
 0x1049ed8 => {
  { let v: u16 = cpu.r16(0x112307eu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1049ee8; continue;
 }
 0x1049ee1 => {
  { let v: u16 = cpu.r16(0x112307cu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1049ee8; continue;
 }
 0x1049ee8 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), (cpu.ecx as u8)); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x1049ef4; continue; }
  pc = 0x1049eee; continue;
 }
 0x1049eee => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1049ef4; continue;
 }
 0x1049ef4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049ef5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049ef5); }
 let mut pc: u32 = 0x1049ef5;
 loop { match pc {
 0x1049ef5 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x6u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0xau16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x2cu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), 0x1au16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x1049f19); f_0105381d(cpu); if cpu.returned_to != 0x1049f19 { return; }
  { let v: u8 = 0x50u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x1049f20); f_010504d6(cpu); if cpu.returned_to != 0x1049f20 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049f22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049f22); }
 let mut pc: u32 = 0x1049f22;
 loop { match pc {
 0x1049f22 => {
  { let v: u32 = 0x1167f74u32; cpu.edi = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1049f50; continue; }
  pc = 0x1049f35; continue;
 }
 0x1049f35 => {
  { cpu.sub8(cpu.r8(0x10ef20du32), 0x0u8); }
  if !cpu.zf { pc = 0x1049f50; continue; }
  pc = 0x1049f3e; continue;
 }
 0x1049f3e => {
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if cpu.zf { pc = 0x1049f50; continue; }
  pc = 0x1049f4a; continue;
 }
 0x1049f4a => {
  { let v: u32 = cpu.edi.wrapping_add(0x14a0u32); cpu.edi = v; }
  pc = 0x1049f50; continue;
 }
 0x1049f50 => {
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1049f54; continue;
 }
 0x1049f54 => {
  { cpu.scas(2, 0); }
  if cpu.zf { pc = 0x1049f5f; continue; }
  pc = 0x1049f58; continue;
 }
 0x1049f58 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x2au32); cpu.edi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1049f54; continue; }
  pc = 0x1049f5d; continue;
 }
 0x1049f5d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1049f5f => {
  { let v: u32 = cpu.sub32(cpu.edi, 0x2u32); cpu.edi = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049f64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049f64); }
 let mut pc: u32 = 0x1049f64;
 loop { match pc {
 0x1049f64 => {
  { let v: u32 = 0x1167f74u32; cpu.edi = v; }
  { let v: u32 = 0x14a0u32; cpu.ecx = v; }
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049f76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049f76); }
 let mut pc: u32 = 0x1049f76;
 loop { match pc {
 0x1049f76 => {
  let t = cpu.r32(0x10e436cu32); cpu.push(0x1049f7c); cpu.call(t); if cpu.returned_to != 0x1049f7c { return; }
  { let v: u32 = 0x116fd54u32; cpu.ebp = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x1049f86; continue;
 }
 0x1049f86 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x1049f97; continue; }
  pc = 0x1049f8e; continue;
 }
 0x1049f8e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1049f95); f_01049f9d(cpu); if cpu.returned_to != 0x1049f95 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1049f97; continue;
 }
 0x1049f97 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x1cu32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1049f86; continue; }
  pc = 0x1049f9c; continue;
 }
 0x1049f9c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01049f9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1049f9d); }
 let mut pc: u32 = 0x1049f9d;
 loop { match pc {
 0x1049f9d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbbu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e9bu32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3dbdu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e9du32, v); }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xfu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x1128d48u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.push(0x1049fca); cpu.call(t); if cpu.returned_to != 0x1049fca { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if !cpu.zf { pc = 0x104a0c9; continue; }
  pc = 0x1049fd1; continue;
 }
 0x1049fd1 => {
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1049fec); f_0104a2f7(cpu); if cpu.returned_to != 0x1049fec { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.ebp)).wrapping_add(0x10u16); cpu.w16(0x10u32.wrapping_add(cpu.ebp), v); }
  let t = cpu.r32(0x1128d48u32.wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.push(0x1049ffb); cpu.call(t); if cpu.returned_to != 0x1049ffb { return; }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x104a015); f_0104a2f7(cpu); if cpu.returned_to != 0x104a015 { return; }
  { cpu.popad(); }
  pc = 0x104a016; continue;
 }
 0x104a016 => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x4u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x104a031); f_01049db3(cpu); if cpu.returned_to != 0x104a031 { return; }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x9cu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x60u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0xa0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x5cu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x10ea910u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10f5e32u32; cpu.edi = v; }
  { let v: u32 = cpu.r16(0x10f78bcu32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x104a0c0; continue; }
  pc = 0x104a08c; continue;
 }
 0x104a08c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef32cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  cpu.push(0x104a0c0); f_0105c061(cpu); if cpu.returned_to != 0x104a0c0 { return; }
  pc = 0x104a0c0; continue;
 }
 0x104a0c0 => {
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x104a0c2; continue;
 }
 0x104a0c2 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104a0c9 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x1u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x104a0de); f_010771d7(cpu); if cpu.returned_to != 0x104a0de { return; }
  if !cpu.cf { pc = 0x104a0e7; continue; }
  pc = 0x104a0e0; continue;
 }
 0x104a0e0 => {
  cpu.push(0x104a0e5); f_0104a2e5(cpu); if cpu.returned_to != 0x104a0e5 { return; }
  cpu.poll(); pc = 0x104a0c2; continue;
 }
 0x104a0e7 => {
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.mul16(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x6u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.ebx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub16((cpu.ecx as u16), (cpu.edx as u16)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x104a131; continue; }
  pc = 0x104a122; continue;
 }
 0x104a122 => {
  if cpu.cf { cpu.poll(); pc = 0x104a016; continue; }
  pc = 0x104a128; continue;
 }
 0x104a128 => {
  { cpu.sub16((cpu.ebx as u16), (cpu.eax as u16)); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x104a016; continue; }
  pc = 0x104a131; continue;
 }
 0x104a131 => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x104a14b); f_0104a2f7(cpu); if cpu.returned_to != 0x104a14b { return; }
  if cpu.cf { cpu.poll(); pc = 0x104a0c2; continue; }
  pc = 0x104a151; continue;
 }
 0x104a151 => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x104a163); f_0104f36d(cpu); if cpu.returned_to != 0x104a163 { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10f714bu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x5u32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x104a179; continue; }
  pc = 0x104a177; continue;
 }
 0x104a177 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104a179 => {
  { let v: u16 = cpu.r16(0x10e3dbbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3dbdu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x1au32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x104a1d4; continue; }
  pc = 0x104a19a; continue;
 }
 0x104a19a => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e40fcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x104a1d3; continue; }
  pc = 0x104a1a3; continue;
 }
 0x104a1a3 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x104a1d3; continue; }
  pc = 0x104a1ac; continue;
 }
 0x104a1ac => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4100u32)); }
  if (cpu.sf != cpu.of) { pc = 0x104a1d3; continue; }
  pc = 0x104a1b5; continue;
 }
 0x104a1b5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x2u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x104a1d3; continue; }
  pc = 0x104a1c4; continue;
 }
 0x104a1c4 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xf0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.edx as u8), 0x17u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e4378u32); cpu.push(0x104a1d3); cpu.call(t); if cpu.returned_to != 0x104a1d3 { return; }
  pc = 0x104a1d3; continue;
 }
 0x104a1d3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104a1d4 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x104a1d3; continue; }
  pc = 0x104a1e5; continue;
 }
 0x104a1e5 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x104a1d3; continue; }
  pc = 0x104a1ee; continue;
 }
 0x104a1ee => {
  { let v: u16 = cpu.r16(0x10e40fcu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0x10u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x104a1d3; continue; }
  pc = 0x104a1fe; continue;
 }
 0x104a1fe => {
  { let v: u16 = cpu.r16(0x10e4100u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0x10u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), (cpu.edx as u16)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x104a1d3; continue; }
  pc = 0x104a20e; continue;
 }
 0x104a20e => {
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)) ^ 0x1u8; cpu.w8(0x19u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x554u32); cpu.ecx = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x10c052cu32, v); }
  { let v: u32 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x10e4130u32, v); }
  cpu.push(0x104a23c); f_010366a8(cpu); if cpu.returned_to != 0x104a23c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104a23d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104a23d); }
 let mut pc: u32 = 0x104a23d;
 loop { match pc {
 0x104a23d => {
  { cpu.pushad(); }
  { cpu.and32(cpu.ebx, 0x8u32); }
  if !cpu.zf { pc = 0x104a267; continue; }
  pc = 0x104a246; continue;
 }
 0x104a246 => {
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = 0x10ea910u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  pc = 0x104a256; continue;
 }
 0x104a256 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10f662fu32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { pc = 0x104a28c; continue; }
  pc = 0x104a264; continue;
 }
 0x104a264 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104a267 => {
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = 0x10ea910u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10f662fu32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x104a256; continue; }
  pc = 0x104a289; continue;
 }
 0x104a289 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104a28c => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xa0u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x5cu32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0xfau16; cpu.w16(0x10e3ebfu32, v); }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x104a2d1); f_0105a7f5(cpu); if cpu.returned_to != 0x104a2d1 { return; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104a2d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104a2d4); }
 let mut pc: u32 = 0x104a2d4;
 loop { match pc {
 0x104a2d4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0xdu32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x1152eeeu32.wrapping_add(cpu.eax))); cpu.w8(0x1152eeeu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104a2e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104a2e5); }
 let mut pc: u32 = 0x104a2e5;
 loop { match pc {
 0x104a2e5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0xdu32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1152eeeu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104a2f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104a2f7); }
 let mut pc: u32 = 0x104a2f7;
 loop { match pc {
 0x104a2f7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x104a308); f_0104a23d(cpu); if cpu.returned_to != 0x104a308 { return; }
  if !cpu.cf { pc = 0x104a311; continue; }
  pc = 0x104a30a; continue;
 }
 0x104a30a => {
  cpu.push(0x104a30f); f_0104a2d4(cpu); if cpu.returned_to != 0x104a30f { return; }
  pc = 0x104a358; continue;
 }
 0x104a311 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0x10e47b8u32; cpu.ecx = v; }
  pc = 0x104a327; continue;
 }
 0x104a327 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x100u32); }
  if cpu.cf { pc = 0x104a33b; continue; }
  pc = 0x104a337; continue;
 }
 0x104a337 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x104a34b; continue;
 }
 0x104a33b => {
  cpu.push(0x104a340); f_0104a35a(cpu); if cpu.returned_to != 0x104a340 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.cf { pc = 0x104a34b; continue; }
  pc = 0x104a344; continue;
 }
 0x104a344 => {
  cpu.push(0x104a349); f_0104a2e5(cpu); if cpu.returned_to != 0x104a349 { return; }
  pc = 0x104a358; continue;
 }
 0x104a34b => {
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x10e47cau32); }
  if cpu.cf { cpu.poll(); pc = 0x104a327; continue; }
  pc = 0x104a356; continue;
 }
 0x104a356 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104a358 => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0104a35a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x104a35a); }
 let mut pc: u32 = 0x104a35a;
 loop { match pc {
 0x104a35a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x11a5df0u32.wrapping_add(cpu.eax)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x104a3cd; continue; }
  pc = 0x104a366; continue;
 }
 0x104a366 => {
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = 0x11a3df0u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0x8u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebp as u16).wrapping_sub(0x8u16); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  pc = 0x104a385; continue;
 }
 0x104a385 => {
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1152e54u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x104a3ca; continue; }
  pc = 0x104a399; continue;
 }
 0x104a399 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ebx >> 8) as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x104a3ca; continue; }
  pc = 0x104a3a0; continue;
 }
 0x104a3a0 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x104a3d0; continue; }
  pc = 0x104a3a6; continue;
 }
 0x104a3a6 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x104a3b2; continue; }
  pc = 0x104a3af; continue;
 }
 0x104a3af => {
  { let v: u16 = cpu.neg16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x104a3b2; continue;
 }
 0x104a3b2 => {
  { cpu.sub16((cpu.edi as u16), 0x8u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x104a3ca; continue; }
  pc = 0x104a3b8; continue;
 }
 0x104a3b8 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.ebp as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x104a3c4; continue; }
  pc = 0x104a3c1; continue;
 }
 0x104a3c1 => {
  { let v: u16 = cpu.neg16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x104a3c4; continue;
 }
 0x104a3c4 => {
  { cpu.sub16((cpu.edi as u16), 0x8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x104a3f4; continue; }
  pc = 0x104a3ca; continue;
 }
 0x104a3ca => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x104a385; continue; }
  pc = 0x104a3cd; continue;
 }
 0x104a3cd => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104a3d0 => {
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.edx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x104a3dc; continue; }
  pc = 0x104a3d9; continue;
 }
 0x104a3d9 => {
  { let v: u16 = cpu.neg16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x104a3dc; continue;
 }
 0x104a3dc => {
  { cpu.sub16((cpu.edi as u16), 0x10u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x104a3ca; continue; }
  pc = 0x104a3e2; continue;
 }
 0x104a3e2 => {
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.edi as u16), (cpu.ebp as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x104a3ee; continue; }
  pc = 0x104a3eb; continue;
 }
 0x104a3eb => {
  { let v: u16 = cpu.neg16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x104a3ee; continue;
 }
 0x104a3ee => {
  { cpu.sub16((cpu.edi as u16), 0x10u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x104a3ca; continue; }
  pc = 0x104a3f4; continue;
 }
 0x104a3f4 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1128d9eu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4134u32, v); }
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x2u8); }
  if cpu.zf { pc = 0x104a44d; continue; }
  pc = 0x104a40a; continue;
 }
 0x104a40a => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x20u8); }
  if cpu.zf { pc = 0x104a4b9; continue; }
  pc = 0x104a416; continue;
 }
 0x104a416 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x24u8); }
  if cpu.zf { pc = 0x104a4b9; continue; }
  pc = 0x104a420; continue;
 }
 0x104a420 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x26u8); }
  if cpu.zf { pc = 0x104a4b9; continue; }
  pc = 0x104a42a; continue;
 }
 0x104a42a => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x104a4b9; continue; }
  pc = 0x104a434; continue;
 }
 0x104a434 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x22u8); }
  if cpu.zf { pc = 0x104a44d; continue; }
  pc = 0x104a43a; continue;
 }
 0x104a43a => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.eax)), 0x1bu8); }
  if cpu.zf { pc = 0x104a44d; continue; }
  pc = 0x104a440; continue;
 }
 0x104a440 => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x19u32.wrapping_add(cpu.eax)), 0x3u8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x104a4b9; continue; }
  pc = 0x104a44d; continue;
 }
 0x104a44d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0xb8u32; cpu.eax = v; }
  { let v = cpu.r8(0xdu32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x1152e54u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  cpu.push(0x104a464); f_0105f215(cpu); if cpu.returned_to != 0x104a464 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x1bu8); }
  if cpu.zf { pc = 0x104a475; continue; }
  pc = 0x104a46f; continue;
 }
 0x104a46f => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x18u8); }
  if !cpu.zf { pc = 0x104a48b; continue; }
  pc = 0x104a475; continue;
 }
 0x104a475 => {
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), 1u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x4u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x7u32; cpu.ecx = v; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x39u32.wrapping_add(cpu.ebx))); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.zf { pc = 0x104a48b; continue; }
  pc = 0x104a487; continue;
 }
 0x104a487 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x104a4b9; continue;
 }
 0x104a48b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = cpu.shr8(((cpu.eax >> 8) as u8), (cpu.ecx & 0xff)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if (cpu.sf == cpu.of) { pc = 0x104a4b9; continue; }
  pc = 0x104a493; continue;
 }
 0x104a493 => {
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x104a49d); f_010753bf(cpu); if cpu.returned_to != 0x104a49d { return; }
  if !cpu.sf { pc = 0x104a60e; continue; }
  pc = 0x104a4a3; continue;
 }
 0x104a4a3 => {
  { let v: u8 = 0x1u8; cpu.w8(0x1128d9cu32, v); }
  cpu.push(0x104a4af); f_0107479d(cpu); if cpu.returned_to != 0x104a4af { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x1128d9cu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x104a4b9 => {
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x104a4eb; continue; }
  pc = 0x104a4c2; continue;
 }
 0x104a4c2 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x3u8); }
  if cpu.zf { pc = 0x104a4eb; continue; }
  pc = 0x104a4c8; continue;
 }
 0x104a4c8 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0xfu8); }
  if cpu.zf { pc = 0x104a4eb; continue; }
  pc = 0x104a4ce; continue;
 }
 0x104a4ce => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.eax)), 0x10u8); }
  if cpu.zf { pc = 0x104a4eb; continue; }
  pc = 0x104a4d4; continue;
 }
 0x104a4d4 => {
  { let v: u8 = cpu.sub8(((cpu.edx >> 8) as u8), cpu.r8(0x10e3f49u32)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x104a4eb; continue; }
  pc = 0x104a4dc; continue;
 }
 0x104a4dc => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x104a4e2); f_0105f215(cpu); if cpu.returned_to != 0x104a4e2 { return; }
  { cpu.and32(cpu.eax, 0x200u32); }
  { let v: u8 = (!cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x104a4eb; continue;
 }
 0x104a4eb => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x104a4fb); f_01072d00(cpu); if cpu.returned_to != 0x104a4fb { return; }
  if cpu.cf { pc = 0x104a528; continue; }
  pc = 0x104a4fd; continue;
 }
 0x104a4fd => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xau32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x104a51d; continue; }
  pc = 0x104a513; continue;
 }
 0x104a513 => {
  cpu.push(0x104a518); f_01012780(cpu); if cpu.returned_to != 0x104a518 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x104a51d; continue;
 }
 0x104a51d => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1124471u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x104a528); f_010356fe(cpu); if cpu.returned_to != 0x104a528 { return; }
  pc = 0x104a528; continue;
 }
 0x104a528 => {
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x92u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x52u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3d47u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3d49u32, v); }
  { let v: u8 = 0x80u8; cpu.w8(0x10ef32fu32, v); }
  { let v: u8 = 0x4u8; cpu.w8(0x10ef32du32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef32eu32, v); }
  { let v: u16 = 0xfau16; cpu.w16(0x10e3ebfu32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e4134u32); cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x1u8); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  if cpu.zf { pc = 0x104a5a9; continue; }
  pc = 0x104a573; continue;
 }
 0x104a573 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x104a596; continue; }
  pc = 0x104a57f; continue;
 }
 0x104a57f => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x104a59b; continue; }
  pc = 0x104a588; continue;
 }
 0x104a588 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x104a5a0; continue; }
  pc = 0x104a591; continue;
 }
 0x104a591 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  pc = 0x104a5a3; continue;
 }
 0x104a596 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x1u32); cpu.eax = v; }
  pc = 0x104a5a3; continue;
 }
 0x104a59b => {
  { let v: u32 = cpu.add32(cpu.eax, 0x1u32); cpu.eax = v; }
  pc = 0x104a5a3; continue;
 }
 0x104a5a0 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  pc = 0x104a5a3; continue;
 }
 0x104a5a3 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x104a5dd; continue;
 }
 0x104a5a9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x0u8); }
  if cpu.zf { pc = 0x104a5cc; continue; }
  pc = 0x104a5b5; continue;
 }
 0x104a5b5 => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x1u8); }
  if cpu.zf { pc = 0x104a5d1; continue; }
  pc = 0x104a5be; continue;
 }
 0x104a5be => {
  { cpu.sub8(cpu.r8(0x10ef36du32), 0x2u8); }
  if cpu.zf { pc = 0x104a5d6; continue; }
  pc = 0x104a5c7; continue;
 }
 0x104a5c7 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x104a5d9; continue;
 }
 0x104a5cc => {
  { let v: u32 = cpu.add32(cpu.eax, 0x1u32); cpu.eax = v; }
  pc = 0x104a5d9; continue;
 }
 0x104a5d1 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  pc = 0x104a5d9; continue;
 }
 0x104a5d6 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x3u32); cpu.eax = v; }
  pc = 0x104a5d9; continue;
 }
 0x104a5d9 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x104a5dd; continue;
 }
 0x104a5dd => {
  cpu.push(0x104a5e2); f_0105c149(cpu); if cpu.returned_to != 0x104a5e2 { return; }
  { cpu.popad(); }
  cpu.push(0x104a5e8); f_01075344(cpu); if cpu.returned_to != 0x104a5e8 { return; }
  if !cpu.sf { pc = 0x104a60e; continue; }
  pc = 0x104a5ea; continue;
 }
 0x104a5ea => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x104a607; continue; }
  pc = 0x104a5f0; continue;
 }
 0x104a5f0 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x104a607; continue; }
  pc = 0x104a5f6; continue;
 }
 0x104a5f6 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0xfu8); }
  if cpu.zf { pc = 0x104a607; continue; }
  pc = 0x104a5fc; continue;
 }
 0x104a5fc => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x10u8); }
  if cpu.zf { pc = 0x104a607; continue; }
  pc = 0x104a602; continue;
 }
 0x104a602 => {
  cpu.push(0x104a607); f_010745fe(cpu); if cpu.returned_to != 0x104a607 { return; }
  pc = 0x104a607; continue;
 }
 0x104a607 => {
  { cpu.pushad(); }
  cpu.push(0x104a60d); f_010504cc(cpu); if cpu.returned_to != 0x104a60d { return; }
  { cpu.popad(); }
  pc = 0x104a60e; continue;
 }
 0x104a60e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
