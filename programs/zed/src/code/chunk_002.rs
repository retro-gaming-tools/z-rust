#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_010111b3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10111b3); } r_010111b3(cpu, 0x10111b3); }
pub fn f_01011213(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011213); } r_010111b3(cpu, 0x1011213); }
pub fn f_01011263(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011263); } r_010111b3(cpu, 0x1011263); }
pub fn f_01011289(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011289); } r_010111b3(cpu, 0x1011289); }
pub fn f_01011296(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011296); } r_010111b3(cpu, 0x1011296); }
pub fn f_01011298(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011298); } r_010111b3(cpu, 0x1011298); }
pub fn f_010112c9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10112c9); } r_010111b3(cpu, 0x10112c9); }
pub fn f_010112d5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10112d5); } r_010111b3(cpu, 0x10112d5); }
pub fn f_01011572(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011572); } r_010111b3(cpu, 0x1011572); }
fn r_010111b3(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10111a9 => {
  { let v: u32 = cpu.and32(cpu.ecx, 0xfu32); cpu.ecx = v; }
  cpu.poll(); pc = cpu.r32(0x10111b3u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x10111b3 => {
  cpu.unimplemented(0x10111b3, "Hlt");
  { let v: u32 = cpu.adc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))).wrapping_add(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) ^ (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x12u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x12u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.unimplemented(0x10111cb, "Insb");
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.r32(0x94010112u32.wrapping_add(cpu.eax)), cpu.eax); cpu.w32(0x94010112u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.r32(0xbc010112u32.wrapping_add(cpu.eax)), cpu.ebp); cpu.w32(0xbc010112u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, cpu.esp); cpu.esp = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edi); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edx.wrapping_mul(1))).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) & (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  {  }
  pc = 0x10111f4; continue;
 }
 0x10111f4 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1011208 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1011213; continue;
 }
 0x1011213 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x101121c => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1011230 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1011244 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1011258 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1011263; continue;
 }
 0x1011263 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x101126c => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1011280 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1011289; continue;
 }
 0x1011289 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1011294 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1011296; continue;
 }
 0x1011296 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1011298; continue;
 }
 0x1011298 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10112a8 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10112bc => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10112c9 => {
  { let a = cpu.r32(0x1u32.wrapping_add(cpu.eax)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x1u32.wrapping_add(cpu.eax), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10112d0 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10112d5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10112e4 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10112f8 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x101130c => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1011320 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1011572 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x101157a; continue;
 }
 0x101157a => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x1011581; continue; }
  pc = 0x1011580; continue;
 }
 0x1011580 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x1011581; continue;
 }
 0x1011581 => {
  { let v: u8 = cpu.ror8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.esi, 0x0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x101157a; continue; }
  pc = 0x1011589; continue;
 }
 0x1011589 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x10115e5; continue; }
  pc = 0x101158f; continue;
 }
 0x101158f => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10115a6; continue; }
  pc = 0x1011594; continue;
 }
 0x1011594 => {
  cpu.push(0x1011599); f_01011139(cpu); if cpu.returned_to != 0x1011599 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10115a6 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x10115d1; continue; }
  pc = 0x10115ab; continue;
 }
 0x10115ab => {
  cpu.push(0x10115b0); f_0101116e(cpu); if cpu.returned_to != 0x10115b0 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10115ba; continue;
 }
 0x10115ba => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x10115c1; continue; }
  pc = 0x10115c0; continue;
 }
 0x10115c0 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10115c1; continue;
 }
 0x10115c1 => {
  { let v: u8 = cpu.ror8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.esi, 0x0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10115ba; continue; }
  pc = 0x10115c9; continue;
 }
 0x10115c9 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.poll(); pc = 0x10111a9; continue;
 }
 0x10115d1 => {
  cpu.push(0x10115d6); f_0101116e(cpu); if cpu.returned_to != 0x10115d6 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x10115de); f_0101116e(cpu); if cpu.returned_to != 0x10115de { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  pc = 0x10115e5; continue;
 }
 0x10115e5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011336(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011336); } r_01011336(cpu, 0x1011336); }
pub fn f_0101133e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101133e); } r_01011336(cpu, 0x101133e); }
pub fn f_0101133f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101133f); } r_01011336(cpu, 0x101133f); }
pub fn f_01011341(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011341); } r_01011336(cpu, 0x1011341); }
pub fn f_010115e6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10115e6); } r_01011336(cpu, 0x10115e6); }
fn r_01011336(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1011333 => {
  { let v: u32 = cpu.shld32(cpu.eax, cpu.ecx, 20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  pc = 0x101133a; continue;
 }
 0x1011336 => {
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x83u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1011349; continue; }
  pc = 0x101133a; continue;
 }
 0x101133a => {
  let t = cpu.r32(0x1011382u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.push(0x1011341); cpu.call(t); if cpu.returned_to != 0x1011341 { return; }
  pc = 0x1011341; continue;
 }
 0x101133e => {
  { let v: u32 = cpu.adc32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x140c7u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x140c7u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.movs(1, false); }
  pc = 0x1011349; continue;
 }
 0x101133f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1011341; continue;
 }
 0x1011341 => {
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  { let v: u32 = cpu.shld32(cpu.eax, cpu.ecx, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  let t = cpu.r32(0x1011382u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.push(0x1011355); cpu.call(t); if cpu.returned_to != 0x1011355 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  { let v: u32 = cpu.shld32(cpu.eax, cpu.ecx, 28u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  let t = cpu.r32(0x1011382u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.push(0x1011369); cpu.call(t); if cpu.returned_to != 0x1011369 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  let t = cpu.r32(0x1011382u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.push(0x101137b); cpu.call(t); if cpu.returned_to != 0x101137b { return; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x3c0u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1011349 => {
  cpu.unimplemented(0x1011349, "Enter");
  cpu.unimplemented(0x101134d, "Ud0");
 }
 0x10115e6 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10115ee; continue;
 }
 0x10115ee => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x10115f5; continue; }
  pc = 0x10115f4; continue;
 }
 0x10115f4 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10115f5; continue;
 }
 0x10115f5 => {
  { let v: u8 = cpu.ror8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.esi, 0x0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10115ee; continue; }
  pc = 0x10115fd; continue;
 }
 0x10115fd => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x101166c; continue; }
  pc = 0x1011603; continue;
 }
 0x1011603 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x101162e; continue; }
  pc = 0x1011608; continue;
 }
 0x1011608 => {
  cpu.push(0x101160d); f_01011139(cpu); if cpu.returned_to != 0x101160d { return; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x140u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x280u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3c0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x101162e => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x1011646; continue; }
  pc = 0x1011633; continue;
 }
 0x1011633 => {
  cpu.push(0x1011638); f_0101116e(cpu); if cpu.returned_to != 0x1011638 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x101163f); f_0101116e(cpu); if cpu.returned_to != 0x101163f { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.poll(); pc = 0x1011333; continue;
 }
 0x1011646 => {
  cpu.push(0x101164b); f_01011572(cpu); if cpu.returned_to != 0x101164b { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  cpu.push(0x1011653); f_01011572(cpu); if cpu.returned_to != 0x1011653 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x27eu32); cpu.edi = v; }
  cpu.push(0x101165e); f_01011572(cpu); if cpu.returned_to != 0x101165e { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  cpu.push(0x1011666); f_01011572(cpu); if cpu.returned_to != 0x1011666 { return; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x282u32); cpu.edi = v; }
  pc = 0x101166c; continue;
 }
 0x101166c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0101134f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101134f); }
 let mut pc: u32 = 0x101134f;
 loop { match pc {
 0x101134f => {
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x85u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x140c7u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x140c7u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.movs(1, false); }
  cpu.unimplemented(0x101135d, "Enter");
  cpu.unimplemented(0x1011361, "Ud0");
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011382(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011382); } r_01011382(cpu, 0x1011382); }
pub fn f_010113c4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10113c4); } r_01011382(cpu, 0x10113c4); }
fn r_01011382(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1011382 => {
  { let at = 0x0u32.wrapping_add(cpu.ebx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.unimplemented(0x1011386, "Aam");
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.port_in(0x13u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.unimplemented(0x101138e, "Hlt");
  { let v: u32 = cpu.adc32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x14u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) & 0x14u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) ^ 0x14u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x1u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))).wrapping_add(cpu.edx); cpu.w32(0x1u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x1u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))).wrapping_add(cpu.esp); cpu.w32(0x1u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x1u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))).wrapping_add(cpu.esi); cpu.w32(0x1u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x14940101u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))).wrapping_add(cpu.eax); cpu.w32(0x14940101u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.movs(1, false); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xc08b0101u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))).wrapping_add(cpu.esi); cpu.w32(0xc08b0101u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  pc = 0x10113c4; continue;
 }
 0x10113c4 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010113d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10113d4); }
 let mut pc: u32 = 0x10113d4;
 loop { match pc {
 0x10113d4 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010113e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10113e4); }
 let mut pc: u32 = 0x10113e4;
 loop { match pc {
 0x10113e4 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010113f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10113f4); }
 let mut pc: u32 = 0x10113f4;
 loop { match pc {
 0x10113f4 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011404(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011404); }
 let mut pc: u32 = 0x1011404;
 loop { match pc {
 0x1011404 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011414(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011414); }
 let mut pc: u32 = 0x1011414;
 loop { match pc {
 0x1011414 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011424(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011424); }
 let mut pc: u32 = 0x1011424;
 loop { match pc {
 0x1011424 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011434(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011434); }
 let mut pc: u32 = 0x1011434;
 loop { match pc {
 0x1011434 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011444(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011444); }
 let mut pc: u32 = 0x1011444;
 loop { match pc {
 0x1011444 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011454(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011454); }
 let mut pc: u32 = 0x1011454;
 loop { match pc {
 0x1011454 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011464(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011464); }
 let mut pc: u32 = 0x1011464;
 loop { match pc {
 0x1011464 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011474(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011474); }
 let mut pc: u32 = 0x1011474;
 loop { match pc {
 0x1011474 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011484(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011484); }
 let mut pc: u32 = 0x1011484;
 loop { match pc {
 0x1011484 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011494(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011494); }
 let mut pc: u32 = 0x1011494;
 loop { match pc {
 0x1011494 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010114a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10114a4); }
 let mut pc: u32 = 0x10114a4;
 loop { match pc {
 0x10114a4 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010114b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10114b4); }
 let mut pc: u32 = 0x10114b4;
 loop { match pc {
 0x10114b4 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011503(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011503); }
 let mut pc: u32 = 0x1011503;
 loop { match pc {
 0x1011503 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8((cpu.eax as u8), ((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x81000000u32); cpu.eax = v; }
  { let v: u32 = 0xae80000u32; cpu.w32(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x8c0efu32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x8c0efu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.eax = v; }
  f_01011520(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011520(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011520); }
 let mut pc: u32 = 0x1011520;
 loop { match pc {
 0x1011520 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x101152a; continue; }
  pc = 0x1011528; continue;
 }
 0x1011528 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x101152a; continue;
 }
 0x101152a => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x1011530; continue; }
  pc = 0x101152e; continue;
 }
 0x101152e => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1011530; continue;
 }
 0x1011530 => {
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x101153d; continue; }
  pc = 0x101153b; continue;
 }
 0x101153b => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x101153d; continue;
 }
 0x101153d => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x1011543; continue; }
  pc = 0x1011541; continue;
 }
 0x1011541 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1011543; continue;
 }
 0x1011543 => {
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x1011552; continue; }
  pc = 0x1011550; continue;
 }
 0x1011550 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1011552; continue;
 }
 0x1011552 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x1011558; continue; }
  pc = 0x1011556; continue;
 }
 0x1011556 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1011558; continue;
 }
 0x1011558 => {
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x1011565; continue; }
  pc = 0x1011563; continue;
 }
 0x1011563 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1011565; continue;
 }
 0x1011565 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x101156b; continue; }
  pc = 0x1011569; continue;
 }
 0x1011569 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x101156b; continue;
 }
 0x101156b => {
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0101166d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101166d); }
 let mut pc: u32 = 0x101166d;
 loop { match pc {
 0x10114c2 => {
  cpu.push(0x10114c7); f_01011520(cpu); if cpu.returned_to != 0x10114c7 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x10114d2); f_01011520(cpu); if cpu.returned_to != 0x10114d2 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x10114dd); f_01011520(cpu); if cpu.returned_to != 0x10114dd { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x10114e8); f_01011520(cpu); if cpu.returned_to != 0x10114e8 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  cpu.push(0x10114f5); f_01011520(cpu); if cpu.returned_to != 0x10114f5 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x1011500); f_01011520(cpu); if cpu.returned_to != 0x1011500 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x101150b); f_01011520(cpu); if cpu.returned_to != 0x101150b { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x1011516); f_01011520(cpu); if cpu.returned_to != 0x1011516 { return; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x8c0u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x101166d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1011675; continue;
 }
 0x1011675 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x101167c; continue; }
  pc = 0x101167b; continue;
 }
 0x101167b => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x101167c; continue;
 }
 0x101167c => {
  { let v: u8 = cpu.ror8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.esi, 0x0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1011675; continue; }
  pc = 0x1011684; continue;
 }
 0x1011684 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x1011743; continue; }
  pc = 0x101168e; continue;
 }
 0x101168e => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10116fe; continue; }
  pc = 0x1011693; continue;
 }
 0x1011693 => {
  cpu.push(0x1011698); f_01011139(cpu); if cpu.returned_to != 0x1011698 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x140u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x144u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x280u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x284u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3c0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3c4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x640u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x644u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x780u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x784u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8c0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8c4u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10116fe => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x101171d; continue; }
  pc = 0x1011703; continue;
 }
 0x1011703 => {
  cpu.push(0x1011708); f_0101116e(cpu); if cpu.returned_to != 0x1011708 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x101170f); f_01011182(cpu); if cpu.returned_to != 0x101170f { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x1011716); f_01011182(cpu); if cpu.returned_to != 0x1011716 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.poll(); pc = 0x10114c2; continue;
 }
 0x101171d => {
  cpu.push(0x1011722); f_010115e6(cpu); if cpu.returned_to != 0x1011722 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  cpu.push(0x101172a); f_010115e6(cpu); if cpu.returned_to != 0x101172a { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x4fcu32); cpu.edi = v; }
  cpu.push(0x1011735); f_010115e6(cpu); if cpu.returned_to != 0x1011735 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  cpu.push(0x101173d); f_010115e6(cpu); if cpu.returned_to != 0x101173d { return; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x504u32); cpu.edi = v; }
  pc = 0x1011743; continue;
 }
 0x1011743 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011793(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011793); } r_01011793(cpu, 0x1011793); }
pub fn f_010117d4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10117d4); } r_01011793(cpu, 0x10117d4); }
fn r_01011793(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1011793 => {
  cpu.unimplemented(0x1011793, "Aam");
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.port_in(0x17u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.unimplemented(0x101179b, "Hlt");
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x18u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x18u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) & 0x18u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) ^ 0x18u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u8 = cpu.sbb8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))).wrapping_add(cpu.edx); cpu.w32(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))).wrapping_add(cpu.esp); cpu.w32(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))).wrapping_add(cpu.esi); cpu.w32(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x18940101u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))).wrapping_add(cpu.eax); cpu.w32(0x18940101u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.movs(1, false); }
  { let v: u8 = cpu.sbb8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x18c40101u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))).wrapping_add(cpu.esi); cpu.w32(0x18c40101u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  {  }
  pc = 0x10117d4; continue;
 }
 0x10117d4 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010117e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10117e4); }
 let mut pc: u32 = 0x10117e4;
 loop { match pc {
 0x10117e4 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010117f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10117f4); }
 let mut pc: u32 = 0x10117f4;
 loop { match pc {
 0x10117f4 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011804(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011804); }
 let mut pc: u32 = 0x1011804;
 loop { match pc {
 0x1011804 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011814(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011814); }
 let mut pc: u32 = 0x1011814;
 loop { match pc {
 0x1011814 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011824(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011824); }
 let mut pc: u32 = 0x1011824;
 loop { match pc {
 0x1011824 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011834(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011834); }
 let mut pc: u32 = 0x1011834;
 loop { match pc {
 0x1011834 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011844(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011844); }
 let mut pc: u32 = 0x1011844;
 loop { match pc {
 0x1011844 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011854(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011854); }
 let mut pc: u32 = 0x1011854;
 loop { match pc {
 0x1011854 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011855(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011855); }
 let mut pc: u32 = 0x1011855;
 loop { match pc {
 0x1011855 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011864(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011864); }
 let mut pc: u32 = 0x1011864;
 loop { match pc {
 0x1011864 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011874(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011874); }
 let mut pc: u32 = 0x1011874;
 loop { match pc {
 0x1011874 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011884(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011884); }
 let mut pc: u32 = 0x1011884;
 loop { match pc {
 0x1011884 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011894(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011894); }
 let mut pc: u32 = 0x1011894;
 loop { match pc {
 0x1011894 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010118a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10118a4); }
 let mut pc: u32 = 0x10118a4;
 loop { match pc {
 0x10118a4 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010118b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10118b4); }
 let mut pc: u32 = 0x10118b4;
 loop { match pc {
 0x10118b4 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010118c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10118c4); }
 let mut pc: u32 = 0x10118c4;
 loop { match pc {
 0x10118c4 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011930(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011930); } r_01011930(cpu, 0x1011930); }
pub fn f_01011947(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011947); } r_01011930(cpu, 0x1011947); }
fn r_01011930(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1011930 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x101193a; continue; }
  pc = 0x1011938; continue;
 }
 0x1011938 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x101193a; continue;
 }
 0x101193a => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x1011940; continue; }
  pc = 0x101193e; continue;
 }
 0x101193e => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1011940; continue;
 }
 0x1011940 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1011947; continue;
 }
 0x1011947 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x101194d; continue; }
  pc = 0x101194b; continue;
 }
 0x101194b => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x101194d; continue;
 }
 0x101194d => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x1011953; continue; }
  pc = 0x1011951; continue;
 }
 0x1011951 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1011953; continue;
 }
 0x1011953 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x1011961; continue; }
  pc = 0x101195f; continue;
 }
 0x101195f => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1011961; continue;
 }
 0x1011961 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x1011967; continue; }
  pc = 0x1011965; continue;
 }
 0x1011965 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1011967; continue;
 }
 0x1011967 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x1011975; continue; }
  pc = 0x1011973; continue;
 }
 0x1011973 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1011975; continue;
 }
 0x1011975 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.cf { pc = 0x101197b; continue; }
  pc = 0x1011979; continue;
 }
 0x1011979 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x101197b; continue;
 }
 0x101197b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011980(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011980); }
 let mut pc: u32 = 0x1011980;
 loop { match pc {
 0x1011744 => {
  { let v: u32 = cpu.shld32(cpu.eax, cpu.ecx, 20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  let t = cpu.r32(0x1011793u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.push(0x1011752); cpu.call(t); if cpu.returned_to != 0x1011752 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  { let v: u32 = cpu.shld32(cpu.eax, cpu.ecx, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  let t = cpu.r32(0x1011793u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.push(0x1011766); cpu.call(t); if cpu.returned_to != 0x1011766 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  { let v: u32 = cpu.shld32(cpu.eax, cpu.ecx, 28u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  let t = cpu.r32(0x1011793u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.push(0x101177a); cpu.call(t); if cpu.returned_to != 0x101177a { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  let t = cpu.r32(0x1011793u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.push(0x101178c); cpu.call(t); if cpu.returned_to != 0x101178c { return; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x3c0u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1011980 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1011988; continue;
 }
 0x1011988 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x101198f; continue; }
  pc = 0x101198e; continue;
 }
 0x101198e => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x101198f; continue;
 }
 0x101198f => {
  { let v: u8 = cpu.ror8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.esi, 0x0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1011988; continue; }
  pc = 0x1011997; continue;
 }
 0x1011997 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x1011a19; continue; }
  pc = 0x101199d; continue;
 }
 0x101199d => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10119db; continue; }
  pc = 0x10119a2; continue;
 }
 0x10119a2 => {
  cpu.push(0x10119a7); f_01011139(cpu); if cpu.returned_to != 0x10119a7 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x142u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x280u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x282u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x3c0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x3c2u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10119db => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x10119f3; continue; }
  pc = 0x10119e0; continue;
 }
 0x10119e0 => {
  cpu.push(0x10119e5); f_0101116e(cpu); if cpu.returned_to != 0x10119e5 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x10119ec); f_0101116e(cpu); if cpu.returned_to != 0x10119ec { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.poll(); pc = 0x1011744; continue;
 }
 0x10119f3 => {
  cpu.push(0x10119f8); f_01011572(cpu); if cpu.returned_to != 0x10119f8 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  cpu.push(0x1011a00); f_01011572(cpu); if cpu.returned_to != 0x1011a00 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x27eu32); cpu.edi = v; }
  cpu.push(0x1011a0b); f_01011572(cpu); if cpu.returned_to != 0x1011a0b { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  cpu.push(0x1011a13); f_01011572(cpu); if cpu.returned_to != 0x1011a13 { return; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x282u32); cpu.edi = v; }
  pc = 0x1011a19; continue;
 }
 0x1011a19 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011a1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011a1a); }
 let mut pc: u32 = 0x1011a1a;
 loop { match pc {
 0x10118d4 => {
  cpu.push(0x10118d9); f_01011930(cpu); if cpu.returned_to != 0x10118d9 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x10118e4); f_01011930(cpu); if cpu.returned_to != 0x10118e4 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x10118ef); f_01011930(cpu); if cpu.returned_to != 0x10118ef { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x10118fa); f_01011930(cpu); if cpu.returned_to != 0x10118fa { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  cpu.push(0x1011907); f_01011930(cpu); if cpu.returned_to != 0x1011907 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x1011912); f_01011930(cpu); if cpu.returned_to != 0x1011912 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x101191d); f_01011930(cpu); if cpu.returned_to != 0x101191d { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x140u32); cpu.edi = v; }
  cpu.push(0x1011928); f_01011930(cpu); if cpu.returned_to != 0x1011928 { return; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x8c0u32); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1011a1a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1011a22; continue;
 }
 0x1011a22 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x1011a29; continue; }
  pc = 0x1011a28; continue;
 }
 0x1011a28 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x1011a29; continue;
 }
 0x1011a29 => {
  { let v: u8 = cpu.ror8((cpu.edx as u8), 1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.esi, 0x0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1011a22; continue; }
  pc = 0x1011a31; continue;
 }
 0x1011a31 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x1011b64; continue; }
  pc = 0x1011a3b; continue;
 }
 0x1011a3b => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x1011b1f; continue; }
  pc = 0x1011a44; continue;
 }
 0x1011a44 => {
  cpu.push(0x1011a49); f_01011139(cpu); if cpu.returned_to != 0x1011a49 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x140u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x142u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x144u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x146u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x280u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x282u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x284u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x286u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x3c0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x3c2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x3c4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x3c6u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x502u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x506u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x640u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x642u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x644u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x646u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x780u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x782u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x784u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x786u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8c0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8c2u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8c4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8c6u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1011b1f => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x1011b3e; continue; }
  pc = 0x1011b24; continue;
 }
 0x1011b24 => {
  cpu.push(0x1011b29); f_0101116e(cpu); if cpu.returned_to != 0x1011b29 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x1011b30); f_01011182(cpu); if cpu.returned_to != 0x1011b30 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.push(0x1011b37); f_01011182(cpu); if cpu.returned_to != 0x1011b37 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.poll(); pc = 0x10118d4; continue;
 }
 0x1011b3e => {
  cpu.push(0x1011b43); f_01011980(cpu); if cpu.returned_to != 0x1011b43 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  cpu.push(0x1011b4b); f_01011980(cpu); if cpu.returned_to != 0x1011b4b { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x4fcu32); cpu.edi = v; }
  cpu.push(0x1011b56); f_01011980(cpu); if cpu.returned_to != 0x1011b56 { return; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  cpu.push(0x1011b5e); f_01011980(cpu); if cpu.returned_to != 0x1011b5e { return; }
  { let v: u32 = cpu.sub32(cpu.edi, 0x504u32); cpu.edi = v; }
  pc = 0x1011b64; continue;
 }
 0x1011b64 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011b65(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011b65); }
 let mut pc: u32 = 0x1011b65;
 loop { match pc {
 0x1011b65 => {
  { cpu.pushad(); }
  { let v: u8 = 0x1u8; cpu.w8(0x110de50u32, v); }
  { let v: u32 = 0x12345678u32; cpu.eax = v; }
  { let v: u32 = 0xa0000u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1011b7b; continue;
 }
 0x1011b7b => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if cpu.zf { pc = 0x1011b86; continue; }
  pc = 0x1011b7f; continue;
 }
 0x1011b7f => {
  { let v: u8 = 0x0u8; cpu.w8(0x110de50u32, v); }
  pc = 0x1011b86; continue;
 }
 0x1011b86 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011b93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011b93); }
 let mut pc: u32 = 0x1011b93;
 loop { match pc {
 0x1011b93 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x110de50u32), 0x0u8); }
  if cpu.zf { pc = 0x1011bb3; continue; }
  pc = 0x1011bac; continue;
 }
 0x1011bac => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  pc = 0x1011bb8; continue;
 }
 0x1011bb3 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(2, true); }
  pc = 0x1011bb8; continue;
 }
 0x1011bb8 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011bba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011bba); }
 let mut pc: u32 = 0x1011bba;
 loop { match pc {
 0x1011bba => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x110de50u32), 0x0u8); }
  if cpu.zf { pc = 0x1011bd1; continue; }
  pc = 0x1011bca; continue;
 }
 0x1011bca => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x1011bd6; continue;
 }
 0x1011bd1 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.movs(2, true); }
  pc = 0x1011bd6; continue;
 }
 0x1011bd6 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011bd8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011bd8); } r_01011bd8(cpu, 0x1011bd8); }
pub fn f_01011c1c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011c1c); } r_01011bd8(cpu, 0x1011c1c); }
fn r_01011bd8(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1011bd8 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.ebx, 0x140u32); }
  if !cpu.zf { pc = 0x1011c39; continue; }
  pc = 0x1011be1; continue;
 }
 0x1011be1 => {
  { cpu.sub32(cpu.ecx, 0xc8u32); }
  if !cpu.zf { pc = 0x1011c39; continue; }
  pc = 0x1011be9; continue;
 }
 0x1011be9 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u8 = 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x19u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x110de50u32), 0x0u8); }
  if !cpu.zf { pc = 0x1011c1c; continue; }
  pc = 0x1011bfd; continue;
 }
 0x1011bfd => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x28u32; cpu.eax = v; }
  pc = 0x1011c03; continue;
 }
 0x1011c03 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1011c09); f_0101166d(cpu); if cpu.returned_to != 0x1011c09 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1011c03; continue; }
  pc = 0x1011c10; continue;
 }
 0x1011c10 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x8c0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1011bfd; continue; }
  pc = 0x1011c1a; continue;
 }
 0x1011c1a => {
  pc = 0x1011c39; continue;
 }
 0x1011c1c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x28u32; cpu.eax = v; }
  pc = 0x1011c22; continue;
 }
 0x1011c22 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1011c28); f_01011a1a(cpu); if cpu.returned_to != 0x1011c28 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1011c22; continue; }
  pc = 0x1011c2f; continue;
 }
 0x1011c2f => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x8c0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1011c1c; continue; }
  pc = 0x1011c39; continue;
 }
 0x1011c39 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011c3b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011c3b); } r_01011c3b(cpu, 0x1011c3b); }
pub fn f_01011c5b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011c5b); } r_01011c3b(cpu, 0x1011c5b); }
pub fn f_01011c5f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011c5f); } r_01011c3b(cpu, 0x1011c5f); }
pub fn f_01011c61(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011c61); } r_01011c3b(cpu, 0x1011c61); }
pub fn f_01011c62(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011c62); } r_01011c3b(cpu, 0x1011c62); }
fn r_01011c3b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1011c3b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011c50; continue;
 }
 0x1011c50 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x116u32); }
  if (cpu.sf != cpu.of) { pc = 0x1011c63; continue; }
  pc = 0x1011c59; continue;
 }
 0x1011c59 => {
  pc = 0x1011c85; continue;
 }
 0x1011c5b => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011c61; continue;
 }
 0x1011c5f => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.df = false; }
  pc = 0x1011c61; continue;
 }
 0x1011c61 => {
  cpu.poll(); pc = 0x1011c50; continue;
 }
 0x1011c62 => {
  { let v: u32 = cpu.port_in((cpu.edx as u16), 4) as u32; cpu.eax = v; }
  pc = 0x1011c63; continue;
 }
 0x1011c63 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d3efcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d3f00u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x1011c5b; continue;
 }
 0x1011c85 => {
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
pub fn f_01011c8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011c8e); }
 let mut pc: u32 = 0x1011c8e;
 loop { match pc {
 0x1011c8e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110de68u32, v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110de6cu32, v); }
  cpu.push(0x1011cb6); f_01011c3b(cpu); if cpu.returned_to != 0x1011cb6 { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01011cbe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011cbe); } r_01011cbe(cpu, 0x1011cbe); }
pub fn f_01011d0f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011d0f); } r_01011cbe(cpu, 0x1011d0f); }
fn r_01011cbe(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1011cbe => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011cda; continue;
 }
 0x1011cda => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x116u32); }
  if (cpu.sf != cpu.of) { pc = 0x1011cf0; continue; }
  pc = 0x1011ce3; continue;
 }
 0x1011ce3 => {
  pc = 0x1011d5b; continue;
 }
 0x1011ce8 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1011cda; continue;
 }
 0x1011cf0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d3efcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x1011d59; continue; }
  pc = 0x1011cff; continue;
 }
 0x1011cff => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d3efcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d3f00u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x115u32); }
  if cpu.zf { pc = 0x1011d34; continue; }
  pc = 0x1011d25; continue;
 }
 0x1011d0f => {
  { let v: u32 = cpu.sbb32(cpu.eax, 0x928b01u32); cpu.eax = v; }
  cpu.unimplemented(0x1011d14, "Aas");
  { let v: u32 = cpu.sbb32(cpu.eax, 0x89c20101u32); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.cf = false; }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x115u32); }
  if cpu.zf { pc = 0x1011d34; continue; }
  pc = 0x1011d25; continue;
 }
 0x1011d25 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d3f04u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x1011d47; continue; }
  pc = 0x1011d34; continue;
 }
 0x1011d34 => {
  { let v: u32 = cpu.r32(0x110de68u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x110de6cu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011d59; continue;
 }
 0x1011d47 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d3f04u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011d59; continue;
 }
 0x1011d59 => {
  cpu.poll(); pc = 0x1011ce8; continue;
 }
 0x1011d5b => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_01011d6d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011d6d); } r_01011d6d(cpu, 0x1011d6d); }
pub fn f_01011dc3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011dc3); } r_01011d6d(cpu, 0x1011dc3); }
pub fn f_01011eda(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011eda); } r_01011d6d(cpu, 0x1011eda); }
pub fn f_01011edb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1011edb); } r_01011d6d(cpu, 0x1011edb); }
fn r_01011d6d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1011d6d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1011d93; continue; }
  pc = 0x1011d87; continue;
 }
 0x1011d87 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011f07; continue;
 }
 0x1011d93 => {
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1011da5; continue; }
  pc = 0x1011d99; continue;
 }
 0x1011d99 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011f07; continue;
 }
 0x1011da5 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011dac; continue;
 }
 0x1011dac => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x116u32); }
  if (cpu.sf != cpu.of) { pc = 0x1011dbf; continue; }
  pc = 0x1011db5; continue;
 }
 0x1011db5 => {
  pc = 0x1011dce; continue;
 }
 0x1011db7 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1011dac; continue;
 }
 0x1011dbf => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  pc = 0x1011dc5; continue;
 }
 0x1011dc3 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x1011dc8; continue; }
  pc = 0x1011dc5; continue;
 }
 0x1011dc5 => {
  { cpu.sub32(cpu.r32(0x11d3efcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x1011db7; continue; }
  pc = 0x1011dce; continue;
 }
 0x1011dc8 => {
  { let v: u32 = cpu.sbb32(cpu.eax, 0xe9750001u32); cpu.eax = v; }
  pc = 0x1011dce; continue;
 }
 0x1011dce => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x116u32); }
  if !cpu.zf { pc = 0x1011de3; continue; }
  pc = 0x1011dd7; continue;
 }
 0x1011dd7 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011f07; continue;
 }
 0x1011de3 => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1011e0f; continue; }
  pc = 0x1011de9; continue;
 }
 0x1011de9 => {
  { let v: u32 = cpu.r32(0x110de68u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d3efcu32, v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d3f00u32, v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d3efcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011f07; continue;
 }
 0x1011e0f => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011e16; continue;
 }
 0x1011e16 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1011e2b; continue; }
  pc = 0x1011e1e; continue;
 }
 0x1011e1e => {
  pc = 0x1011f00; continue;
 }
 0x1011e23 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1011e16; continue;
 }
 0x1011e2b => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d3ef4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d3ef8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x1011e64; continue; }
  pc = 0x1011e50; continue;
 }
 0x1011e50 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d3efcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011e75; continue;
 }
 0x1011e64 => {
  { let v: u32 = cpu.r32(0x110de68u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x110de6cu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011e75; continue;
 }
 0x1011e75 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1011efb; continue; }
  pc = 0x1011e81; continue;
 }
 0x1011e81 => {
  { let v: u32 = 0x115u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011e88; continue;
 }
 0x1011e88 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1011e9a; continue; }
  pc = 0x1011e90; continue;
 }
 0x1011e90 => {
  pc = 0x1011ecc; continue;
 }
 0x1011e92 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1011e88; continue;
 }
 0x1011e9a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d3ef4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11d3efcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d3ef8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d3f00u32.wrapping_add(cpu.edx), v); }
  cpu.poll(); pc = 0x1011e92; continue;
 }
 0x1011ecc => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d3efcu32.wrapping_add(cpu.edx), v); }
  pc = 0x1011edb; continue;
 }
 0x1011eda => {
  { let v: u32 = cpu.r32(0xe2c1f055u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.ecx); cpu.w32(0xe2c1f055u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x8289e845u32.wrapping_add(cpu.ebx))); cpu.ecx = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), ((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xf0458b01u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d3efcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011f07; continue;
 }
 0x1011edb => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d3f00u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d3efcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011f07; continue;
 }
 0x1011efb => {
  cpu.poll(); pc = 0x1011e23; continue;
 }
 0x1011f00 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011f07; continue;
 }
 0x1011f07 => {
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
pub fn f_01011eb1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011eb1); }
 let mut pc: u32 = 0x1011eb1;
 loop { match pc {
 0x1011eb1 => {
  { let v: u32 = cpu.r32(0xe0c1ec45u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.ecx); cpu.w32(0xe0c1ec45u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xe2c1ec55u32.wrapping_add(cpu.ebx))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1d3ef880u32.wrapping_add(cpu.ebx))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1d3f0082u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.ecx); cpu.w32(0x1d3f0082u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  cpu.call(0x1011ecb); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01011f13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011f13); }
 let mut pc: u32 = 0x1011f13;
 loop { match pc {
 0x1011f13 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x1011fc1; continue; }
  pc = 0x1011f2e; continue;
 }
 0x1011f2e => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011f35; continue;
 }
 0x1011f35 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x116u32); }
  if (cpu.sf != cpu.of) { pc = 0x1011f48; continue; }
  pc = 0x1011f3e; continue;
 }
 0x1011f3e => {
  pc = 0x1011f59; continue;
 }
 0x1011f40 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1011f35; continue;
 }
 0x1011f48 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d3efcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { cpu.poll(); pc = 0x1011f40; continue; }
  pc = 0x1011f59; continue;
 }
 0x1011f59 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x116u32); }
  if cpu.zf { pc = 0x1011fc1; continue; }
  pc = 0x1011f62; continue;
 }
 0x1011f62 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1011f68; continue;
 }
 0x1011f68 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x115u32); }
  if (cpu.sf != cpu.of) { pc = 0x1011f7b; continue; }
  pc = 0x1011f71; continue;
 }
 0x1011f71 => {
  pc = 0x1011fad; continue;
 }
 0x1011f73 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1011f68; continue;
 }
 0x1011f7b => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d3f04u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11d3efcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d3f08u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d3f00u32.wrapping_add(cpu.edx), v); }
  cpu.poll(); pc = 0x1011f73; continue;
 }
 0x1011fad => {
  { let v: u32 = 0x0u32; cpu.w32(0x11d47a4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d47a8u32, v); }
  pc = 0x1011fc1; continue;
 }
 0x1011fc1 => {
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
pub fn f_01011fca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1011fca); }
 let mut pc: u32 = 0x1011fca;
 loop { match pc {
 0x1011fca => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136840u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11cea9cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136848u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff70u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1011fff); f_0108dece(cpu); if cpu.returned_to != 0x1011fff { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1012012); f_01001957(cpu); if cpu.returned_to != 0x1012012 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01012020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012020); }
 let mut pc: u32 = 0x1012020;
 loop { match pc {
 0x1012020 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x113684fu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1012040); f_01011fca(cpu); if cpu.returned_to != 0x1012040 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1012055; continue; }
  pc = 0x1012049; continue;
 }
 0x1012049 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10120ec; continue;
 }
 0x1012055 => {
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1012064); f_0108f08a(cpu); if cpu.returned_to != 0x1012064 { return; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x101206c); f_0108f186(cpu); if cpu.returned_to != 0x101206c { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x101207f); f_0108f08a(cpu); if cpu.returned_to != 0x101207f { return; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x101209a; continue; }
  pc = 0x1012087; continue;
 }
 0x1012087 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x1012091); f_01011d6d(cpu); if cpu.returned_to != 0x1012091 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x10120ab; continue; }
  pc = 0x101209a; continue;
 }
 0x101209a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10120a2); f_0108f1c8(cpu); if cpu.returned_to != 0x10120a2 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10120ec; continue;
 }
 0x10120ab => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10120c0); f_0108f30b(cpu); if cpu.returned_to != 0x10120c0 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10120cb); f_0108f1c8(cpu); if cpu.returned_to != 0x10120cb { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x10120dd; continue; }
  pc = 0x10120d5; continue;
 }
 0x10120d5 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10120ec; continue;
 }
 0x10120dd => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10120e5); f_01011f13(cpu); if cpu.returned_to != 0x10120e5 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10120ec; continue;
 }
 0x10120ec => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_010120f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10120f7); }
 let mut pc: u32 = 0x10120f7;
 loop { match pc {
 0x10120f7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x1012116; continue; }
  pc = 0x101210e; continue;
 }
 0x101210e => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1012116); f_01011f13(cpu); if cpu.returned_to != 0x1012116 { return; }
  pc = 0x1012116; continue;
 }
 0x1012116 => {
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
pub fn f_0101211f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101211f); }
 let mut pc: u32 = 0x101211f;
 loop { match pc {
 0x101211f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xc0u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x11d47acu32; cpu.eax = v; }
  cpu.push(0x101213e); f_0108f030(cpu); if cpu.returned_to != 0x101213e { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012145; continue;
 }
 0x1012145 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x10u32); }
  if (cpu.sf != cpu.of) { pc = 0x1012155; continue; }
  pc = 0x101214b; continue;
 }
 0x101214b => {
  pc = 0x1012173; continue;
 }
 0x101214d => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1012145; continue;
 }
 0x1012155 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d47acu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d47b0u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x101214d; continue;
 }
 0x1012173 => {
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
pub fn f_0101217c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101217c); }
 let mut pc: u32 = 0x101217c;
 loop { match pc {
 0x101217c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012191; continue;
 }
 0x1012191 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x8u32); }
  if (cpu.sf != cpu.of) { pc = 0x10121a1; continue; }
  pc = 0x1012197; continue;
 }
 0x1012197 => {
  pc = 0x10121b1; continue;
 }
 0x1012199 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1012191; continue;
 }
 0x10121a1 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d0fbcu32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x1012199; continue;
 }
 0x10121b1 => {
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
pub fn f_010121ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10121ba); }
 let mut pc: u32 = 0x10121ba;
 loop { match pc {
 0x10121ba => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if cpu.zf { pc = 0x1012235; continue; }
  pc = 0x10121d4; continue;
 }
 0x10121d4 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d1028u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x1012235; continue; }
  pc = 0x10121e1; continue;
 }
 0x10121e1 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d102cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x10121fd; continue; }
  pc = 0x10121ee; continue;
 }
 0x10121ee => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d1028u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x10121fd); f_010120f7(cpu); if cpu.returned_to != 0x10121fd { return; }
  pc = 0x10121fd; continue;
 }
 0x10121fd => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d1028u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d1030u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x11d1048u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d102cu32.wrapping_add(cpu.eax), v); }
  pc = 0x1012235; continue;
 }
 0x1012235 => {
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
pub fn f_01012200(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012200); }
 let mut pc: u32 = 0x1012200;
 loop { match pc {
 0x1012200 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x10u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.sbb32(cpu.eax, 0x1u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x45u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x45u32.wrapping_add(cpu.ebx), v); }
  { cpu.df = false; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x10u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.sbb32(cpu.eax, 0x1u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x45u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x45u32.wrapping_add(cpu.ebx), v); }
  { cpu.df = false; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)) | 0x1du8; cpu.w8(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  cpu.call(0x1012224); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0101223e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101223e); }
 let mut pc: u32 = 0x101223e;
 loop { match pc {
 0x101223e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if cpu.zf { pc = 0x1012279; continue; }
  pc = 0x1012255; continue;
 }
 0x1012255 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x101225c; continue;
 }
 0x101225c => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xfau32); }
  if (cpu.sf != cpu.of) { pc = 0x101226f; continue; }
  pc = 0x1012265; continue;
 }
 0x1012265 => {
  pc = 0x1012279; continue;
 }
 0x1012267 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x101225c; continue;
 }
 0x101226f => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1012277); f_010121ba(cpu); if cpu.returned_to != 0x1012277 { return; }
  cpu.poll(); pc = 0x1012267; continue;
 }
 0x1012279 => {
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
pub fn f_01012282(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1012282); } r_01012282(cpu, 0x1012282); }
pub fn f_01012444(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1012444); } r_01012282(cpu, 0x1012444); }
fn r_01012282(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1012282 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if !cpu.zf { pc = 0x10122ae; continue; }
  pc = 0x10122a2; continue;
 }
 0x10122a2 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012443; continue;
 }
 0x10122ae => {
  { cpu.sub32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x10122bd; continue; }
  pc = 0x10122b4; continue;
 }
 0x10122b4 => {
  { cpu.sub32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0xfau32); }
  if (cpu.sf != cpu.of) { pc = 0x10122c9; continue; }
  pc = 0x10122bd; continue;
 }
 0x10122bd => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012443; continue;
 }
 0x10122c9 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10122d1); f_010121ba(cpu); if cpu.returned_to != 0x10122d1 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v: u32 = 0x11d101cu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10122ea); f_0108f51c(cpu); if cpu.returned_to != 0x10122ea { return; }
  pc = 0x10122ea; continue;
 }
 0x10122ea => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v: u32 = 0x11d101cu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.push(0x10122fa); f_0108e46c(cpu); if cpu.returned_to != 0x10122fa { return; }
  { cpu.sub32(cpu.eax, 0x8u32); }
  if !cpu.cf { pc = 0x1012316; continue; }
  pc = 0x10122ff; continue;
 }
 0x10122ff => {
  { let v: u32 = 0x1136852u32; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v: u32 = 0x11d101cu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  cpu.push(0x1012314); f_0108ef8d(cpu); if cpu.returned_to != 0x1012314 { return; }
  cpu.poll(); pc = 0x10122ea; continue;
 }
 0x1012316 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x101231d; continue;
 }
 0x101231d => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xfau32); }
  if (cpu.sf != cpu.of) { pc = 0x1012330; continue; }
  pc = 0x1012326; continue;
 }
 0x1012326 => {
  pc = 0x1012371; continue;
 }
 0x1012328 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x101231d; continue;
 }
 0x1012330 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x1012345; continue; }
  pc = 0x1012338; continue;
 }
 0x1012338 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d1028u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x1012347; continue; }
  pc = 0x1012345; continue;
 }
 0x1012345 => {
  pc = 0x101236b; continue;
 }
 0x1012347 => {
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.edx = v; }
  { let v: u32 = 0x11d101cu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v: u32 = 0x11d101cu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x1012367); f_0108f5f5(cpu); if cpu.returned_to != 0x1012367 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x101236d; continue; }
  pc = 0x101236b; continue;
 }
 0x101236b => {
  pc = 0x101236f; continue;
 }
 0x101236d => {
  pc = 0x1012371; continue;
 }
 0x101236f => {
  cpu.poll(); pc = 0x1012328; continue;
 }
 0x1012371 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xfau32); }
  if !cpu.zf { pc = 0x10123c5; continue; }
  pc = 0x101237a; continue;
 }
 0x101237a => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1012385); f_01012020(cpu); if cpu.returned_to != 0x1012385 { return; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d1028u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d1028u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x10123a8; continue; }
  pc = 0x101239c; continue;
 }
 0x101239c => {
  { let v: u32 = 0xfffffffcu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012443; continue;
 }
 0x10123a8 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d1030u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d102cu32.wrapping_add(cpu.eax), v); }
  pc = 0x10123fb; continue;
 }
 0x10123c5 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d1028u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11d1028u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d1030u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d1030u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x11d102cu32.wrapping_add(cpu.eax), v); }
  pc = 0x10123fb; continue;
 }
 0x10123fb => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d1034u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d1038u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d103cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d1040u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d1044u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012443; continue;
 }
 0x1012443 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 0x1012444 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.df = false; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0101244e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101244e); } r_0101244e(cpu, 0x101244e); }
pub fn f_010125fb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10125fb); } r_0101244e(cpu, 0x10125fb); }
pub fn f_01012633(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1012633); } r_0101244e(cpu, 0x1012633); }
pub fn f_0101263c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101263c); } r_0101244e(cpu, 0x101263c); }
pub fn f_01012645(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1012645); } r_0101244e(cpu, 0x1012645); }
pub fn f_0101264e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101264e); } r_0101244e(cpu, 0x101264e); }
fn r_0101244e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x101244e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if !cpu.zf { pc = 0x1012474; continue; }
  pc = 0x1012468; continue;
 }
 0x1012468 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10126c8; continue;
 }
 0x1012474 => {
  { let v: u32 = 0x1136854u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1012481); f_01011fca(cpu); if cpu.returned_to != 0x1012481 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1012496; continue; }
  pc = 0x101248a; continue;
 }
 0x101248a => {
  { let v: u32 = 0xfffffffeu32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10126c8; continue;
 }
 0x1012496 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x2b11u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x7fu32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0x80u32; cpu.edx = v; }
  { let v: u32 = 0xfffffefcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10124cc); f_0108f61e(cpu); if cpu.returned_to != 0x10124cc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10126b9; continue; }
  pc = 0x10124d4; continue;
 }
 0x10124d4 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10124db; continue;
 }
 0x10124db => {
  { let v: u32 = 0xfffffefcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10124e6); f_0108e46c(cpu); if cpu.returned_to != 0x10124e6 { return; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10124f5; continue; }
  pc = 0x10124eb; continue;
 }
 0x10124eb => {
  pc = 0x1012511; continue;
 }
 0x10124ed => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10124db; continue;
 }
 0x10124f5 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0xfffffefcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))), 0x23u8); }
  if !cpu.zf { pc = 0x101250f; continue; }
  pc = 0x1012502; continue;
 }
 0x1012502 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0xfffffefcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x1012511; continue;
 }
 0x101250f => {
  cpu.poll(); pc = 0x10124ed; continue;
 }
 0x1012511 => {
  { cpu.sub8(cpu.r8(0xfffffefcu32.wrapping_add(cpu.ebp)), 0xau8); }
  if cpu.zf { pc = 0x1012523; continue; }
  pc = 0x101251a; continue;
 }
 0x101251a => {
  { cpu.sub8(cpu.r8(0xfffffefcu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1012528; continue; }
  pc = 0x1012523; continue;
 }
 0x1012523 => {
  cpu.poll(); pc = 0x1012496; continue;
 }
 0x1012528 => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffccu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136857u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xfffffefcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1012559); f_0108f6db(cpu); if cpu.returned_to != 0x1012559 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x2u32); }
  if (cpu.sf == cpu.of) { pc = 0x1012579; continue; }
  pc = 0x1012565; continue;
 }
 0x1012565 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x101256d); f_0108f1c8(cpu); if cpu.returned_to != 0x101256d { return; }
  { let v: u32 = 0xfffffffbu32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10126c8; continue;
 }
 0x1012579 => {
  { cpu.sub8(cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x53u8); }
  if cpu.zf { pc = 0x1012585; continue; }
  pc = 0x101257f; continue;
 }
 0x101257f => {
  { cpu.sub8(cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x73u8); }
  if !cpu.zf { pc = 0x1012591; continue; }
  pc = 0x1012585; continue;
 }
 0x1012585 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffef8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x101259b; continue;
 }
 0x1012591 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffef8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x101259b; continue;
 }
 0x101259b => {
  { let v: u32 = cpu.r32(0xfffffef8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x113686cu32; cpu.eax = v; }
  cpu.push(0x10125b9); f_0108f6fc(cpu); if cpu.returned_to != 0x10125b9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10125cd; continue; }
  pc = 0x10125bd; continue;
 }
 0x10125bd => {
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10125c8); f_0108e46c(cpu); if cpu.returned_to != 0x10125c8 { return; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if cpu.zf { pc = 0x10125f8; continue; }
  pc = 0x10125cd; continue;
 }
 0x10125cd => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1136870u32; cpu.eax = v; }
  cpu.push(0x10125e2); f_0108f6fc(cpu); if cpu.returned_to != 0x10125e2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10125f6; continue; }
  pc = 0x10125e6; continue;
 }
 0x10125e6 => {
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x10125f1); f_0108e46c(cpu); if cpu.returned_to != 0x10125f1 { return; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if cpu.zf { pc = 0x10125f8; continue; }
  pc = 0x10125f6; continue;
 }
 0x10125f6 => {
  pc = 0x1012655; continue;
 }
 0x10125f8 => {
  pc = 0x101260b; continue;
 }
 0x10125fb => {
  { let v: u32 = cpu.esp ^ cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x101260b; continue;
 }
 0x101260b => {
  { let v: u8 = cpu.r8(0x10fd44bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffef4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xfffffef4u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1012655; continue; }
  pc = 0x1012621; continue;
 }
 0x1012621 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfffffef4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x10125fbu32.wrapping_add(cpu.eax)); continue;
 }
 0x1012633 => {
  { let v: u8 = 0x66u8; cpu.w8(0xffffff7cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012655; continue;
 }
 0x101263c => {
  { let v: u8 = 0x67u8; cpu.w8(0xffffff7cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012655; continue;
 }
 0x1012645 => {
  { let v: u8 = 0x73u8; cpu.w8(0xffffff7cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012655; continue;
 }
 0x101264e => {
  { let v: u8 = 0x69u8; cpu.w8(0xffffff7cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012655; continue;
 }
 0x1012655 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1012675); f_01012282(cpu); if cpu.returned_to != 0x1012675 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x101268e; continue; }
  pc = 0x101267e; continue;
 }
 0x101267e => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1012686); f_0108f1c8(cpu); if cpu.returned_to != 0x1012686 { return; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10126c8; continue;
 }
 0x101268e => {
  { cpu.sub32(cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)), 0x80u32); }
  if (cpu.sf != cpu.of) { pc = 0x10126b4; continue; }
  pc = 0x1012697; continue;
 }
 0x1012697 => {
  cpu.push(0x101269c); f_01011cbe(cpu); if cpu.returned_to != 0x101269c { return; }
  { cpu.sub32(cpu.eax, 0x186a0u32); }
  if (cpu.sf == cpu.of) { pc = 0x10126b4; continue; }
  pc = 0x10126a3; continue;
 }
 0x10126a3 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10126ab); f_0108f1c8(cpu); if cpu.returned_to != 0x10126ab { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10126c8; continue;
 }
 0x10126b4 => {
  cpu.poll(); pc = 0x1012496; continue;
 }
 0x10126b9 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10126c1); f_0108f1c8(cpu); if cpu.returned_to != 0x10126c1 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10126c8; continue;
 }
 0x10126c8 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_010126d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10126d4); }
 let mut pc: u32 = 0x10126d4;
 loop { match pc {
 0x10126d4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
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
pub fn f_010126e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10126e9); }
 let mut pc: u32 = 0x10126e9;
 loop { match pc {
 0x10126e9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012701; continue;
 }
 0x1012701 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x110de82u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1012716; continue; }
  pc = 0x101270c; continue;
 }
 0x101270c => {
  pc = 0x101272c; continue;
 }
 0x101270e => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1012701; continue;
 }
 0x1012716 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x101271e); f_01012b1e(cpu); if cpu.returned_to != 0x101271e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x101272a; continue; }
  pc = 0x1012722; continue;
 }
 0x1012722 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012774; continue;
 }
 0x101272a => {
  cpu.poll(); pc = 0x101270e; continue;
 }
 0x101272c => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012733; continue;
 }
 0x1012733 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x110de82u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1012748; continue; }
  pc = 0x101273e; continue;
 }
 0x101273e => {
  pc = 0x101276d; continue;
 }
 0x1012740 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1012733; continue;
 }
 0x1012748 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1012750); f_01012b1e(cpu); if cpu.returned_to != 0x1012750 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1012763; continue; }
  pc = 0x1012754; continue;
 }
 0x1012754 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d47b0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x101276b; continue; }
  pc = 0x1012763; continue;
 }
 0x1012763 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012774; continue;
 }
 0x101276b => {
  cpu.poll(); pc = 0x1012740; continue;
 }
 0x101276d => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012774; continue;
 }
 0x1012774 => {
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
pub fn f_01012780(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012780); }
 let mut pc: u32 = 0x1012780;
 loop { match pc {
 0x1012780 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if !cpu.zf { pc = 0x10127a6; continue; }
  pc = 0x101279a; continue;
 }
 0x101279a => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012924; continue;
 }
 0x10127a6 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x10127b5; continue; }
  pc = 0x10127ac; continue;
 }
 0x10127ac => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0xfau32); }
  if (cpu.sf != cpu.of) { pc = 0x10127b7; continue; }
  pc = 0x10127b5; continue;
 }
 0x10127b5 => {
  pc = 0x10127c4; continue;
 }
 0x10127b7 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d1028u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x10127d0; continue; }
  pc = 0x10127c4; continue;
 }
 0x10127c4 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012924; continue;
 }
 0x10127d0 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d1034u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x10127df); f_010126e9(cpu); if cpu.returned_to != 0x10127df { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x10127f4; continue; }
  pc = 0x10127e8; continue;
 }
 0x10127e8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012924; continue;
 }
 0x10127f4 => {
  { let v: u32 = 0x1000u32; cpu.w32(0x10fcc06u32, v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x101280e; continue; }
  pc = 0x1012804; continue;
 }
 0x1012804 => {
  { let v: u32 = 0x10u32; cpu.w32(0x10fcc06u32, v); }
  pc = 0x101280e; continue;
 }
 0x101280e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1012816); f_01012930(cpu); if cpu.returned_to != 0x1012816 { return; }
  { let v = cpu.r32(0x11d4888u32); cpu.push(v); }
  cpu.push(0x1012821); f_01090e8d(cpu); if cpu.returned_to != 0x1012821 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1012837; continue; }
  pc = 0x101282d; continue;
 }
 0x101282d => {
  { let v: u32 = 0xfffffff8u32; cpu.eax = v; }
  cpu.push(0x1012837); f_0101364c(cpu); if cpu.returned_to != 0x1012837 { return; }
  pc = 0x1012837; continue;
 }
 0x1012837 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1012840); f_010910de(cpu); if cpu.returned_to != 0x1012840 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d47acu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d1034u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x11d47b0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v = cpu.r32(0x11d1030u32.wrapping_add(cpu.eax)); cpu.push(v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v = cpu.r32(0x11d1028u32.wrapping_add(cpu.eax)); cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1012881); f_01091251(cpu); if cpu.returned_to != 0x1012881 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101288f); f_010915f5(cpu); if cpu.returned_to != 0x101288f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v = cpu.r32(0x11d1044u32.wrapping_add(cpu.eax)); cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10128a5); f_0109157f(cpu); if cpu.returned_to != 0x10128a5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v = cpu.r32(0x11d1040u32.wrapping_add(cpu.eax)); cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10128bb); f_01091509(cpu); if cpu.returned_to != 0x10128bb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d1038u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x10128dd; continue; }
  pc = 0x10128cb; continue;
 }
 0x10128cb => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10128d8); f_010912d3(cpu); if cpu.returned_to != 0x10128d8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10128ed; continue;
 }
 0x10128dd => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10128ea); f_010912d3(cpu); if cpu.returned_to != 0x10128ea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10128ed; continue;
 }
 0x10128ed => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d103cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x1012912; continue; }
  pc = 0x10128fa; continue;
 }
 0x10128fa => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d103cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101290f); f_0109166b(cpu); if cpu.returned_to != 0x101290f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x1012912; continue;
 }
 0x1012912 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101291b); f_01091355(cpu); if cpu.returned_to != 0x101291b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012924; continue;
 }
 0x1012924 => {
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
pub fn f_01012930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012930); }
 let mut pc: u32 = 0x1012930;
 loop { match pc {
 0x1012930 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if cpu.zf { pc = 0x10129ac; continue; }
  pc = 0x101294a; continue;
 }
 0x101294a => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x101295b; continue; }
  pc = 0x1012950; continue;
 }
 0x1012950 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x110de82u32)); }
  if (cpu.sf != cpu.of) { pc = 0x101295d; continue; }
  pc = 0x101295b; continue;
 }
 0x101295b => {
  pc = 0x101296a; continue;
 }
 0x101295d => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d47acu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x101296c; continue; }
  pc = 0x101296a; continue;
 }
 0x101296a => {
  pc = 0x10129ac; continue;
 }
 0x101296c => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v = cpu.r32(0x11d47acu32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x101297b); f_0109149c(cpu); if cpu.returned_to != 0x101297b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v = cpu.r32(0x11d47acu32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x101298d); f_01091071(cpu); if cpu.returned_to != 0x101298d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d47acu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d47b0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10129ac; continue;
 }
 0x10129ac => {
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
pub fn f_010129b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10129b5); }
 let mut pc: u32 = 0x10129b5;
 loop { match pc {
 0x10129b5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if cpu.zf { pc = 0x1012a0c; continue; }
  pc = 0x10129cc; continue;
 }
 0x10129cc => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10129d3; continue;
 }
 0x10129d3 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x110de82u32)); }
  if (cpu.sf != cpu.of) { pc = 0x10129e8; continue; }
  pc = 0x10129de; continue;
 }
 0x10129de => {
  pc = 0x1012a0c; continue;
 }
 0x10129e0 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10129d3; continue;
 }
 0x10129e8 => {
  { let v: u32 = 0x1100u32; cpu.w32(0x10fcc06u32, v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1012a02; continue; }
  pc = 0x10129f8; continue;
 }
 0x10129f8 => {
  { let v: u32 = 0x11u32; cpu.w32(0x10fcc06u32, v); }
  pc = 0x1012a02; continue;
 }
 0x1012a02 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1012a0a); f_01012930(cpu); if cpu.returned_to != 0x1012a0a { return; }
  cpu.poll(); pc = 0x10129e0; continue;
 }
 0x1012a0c => {
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
pub fn f_01012a15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012a15); }
 let mut pc: u32 = 0x1012a15;
 loop { match pc {
 0x1012a15 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if !cpu.zf { pc = 0x1012a35; continue; }
  pc = 0x1012a2c; continue;
 }
 0x1012a2c => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012a80; continue;
 }
 0x1012a35 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012a43; continue;
 }
 0x1012a43 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x110de82u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1012a58; continue; }
  pc = 0x1012a4e; continue;
 }
 0x1012a4e => {
  pc = 0x1012a65; continue;
 }
 0x1012a50 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1012a43; continue;
 }
 0x1012a58 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1012a60); f_01012b1e(cpu); if cpu.returned_to != 0x1012a60 { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1012a50; continue;
 }
 0x1012a65 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x110de82u32)); }
  if !cpu.zf { pc = 0x1012a79; continue; }
  pc = 0x1012a70; continue;
 }
 0x1012a70 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012a80; continue;
 }
 0x1012a79 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012a80; continue;
 }
 0x1012a80 => {
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
pub fn f_01012a8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012a8c); }
 let mut pc: u32 = 0x1012a8c;
 loop { match pc {
 0x1012a8c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if !cpu.zf { pc = 0x1012aaf; continue; }
  pc = 0x1012aa6; continue;
 }
 0x1012aa6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012ade; continue;
 }
 0x1012aaf => {
  cpu.push(0x1012ab4); f_010129b5(cpu); if cpu.returned_to != 0x1012ab4 { return; }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1012ac1; continue; }
  pc = 0x1012aba; continue;
 }
 0x1012aba => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012ac1; continue;
 }
 0x1012ac1 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1012ace; continue; }
  pc = 0x1012ac7; continue;
 }
 0x1012ac7 => {
  { let v: u32 = 0x10u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012ace; continue;
 }
 0x1012ace => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110de82u32, v); }
  { let v: u32 = cpu.r32(0x110de82u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012ade; continue;
 }
 0x1012ade => {
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
pub fn f_01012aea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012aea); }
 let mut pc: u32 = 0x1012aea;
 loop { match pc {
 0x1012aea => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if !cpu.zf { pc = 0x1012b0a; continue; }
  pc = 0x1012b01; continue;
 }
 0x1012b01 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012b12; continue;
 }
 0x1012b0a => {
  { let v: u32 = cpu.r32(0x110de82u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012b12; continue;
 }
 0x1012b12 => {
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
pub fn f_01012b1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012b1e); }
 let mut pc: u32 = 0x1012b1e;
 loop { match pc {
 0x1012b1e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if !cpu.zf { pc = 0x1012b41; continue; }
  pc = 0x1012b38; continue;
 }
 0x1012b38 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012b7b; continue;
 }
 0x1012b41 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d47acu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x1012b65; continue; }
  pc = 0x1012b4e; continue;
 }
 0x1012b4e => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v = cpu.r32(0x11d47acu32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x1012b5d); f_010916e1(cpu); if cpu.returned_to != 0x1012b5d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x1012b6e; continue; }
  pc = 0x1012b65; continue;
 }
 0x1012b65 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012b75; continue;
 }
 0x1012b6e => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012b75; continue;
 }
 0x1012b75 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012b7b; continue;
 }
 0x1012b7b => {
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
pub fn f_01012b87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012b87); }
 let mut pc: u32 = 0x1012b87;
 loop { match pc {
 0x1012b87 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if cpu.zf { pc = 0x1012bb0; continue; }
  pc = 0x1012ba3; continue;
 }
 0x1012ba3 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d47acu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x1012bb2; continue; }
  pc = 0x1012bb0; continue;
 }
 0x1012bb0 => {
  pc = 0x1012bc8; continue;
 }
 0x1012bb2 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v = cpu.r32(0x11d47acu32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x1012bc5); f_0109157f(cpu); if cpu.returned_to != 0x1012bc5 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x1012bc8; continue;
 }
 0x1012bc8 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01012bd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012bd0); }
 let mut pc: u32 = 0x1012bd0;
 loop { match pc {
 0x1012bd0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if cpu.zf { pc = 0x1012bf9; continue; }
  pc = 0x1012bec; continue;
 }
 0x1012bec => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d47acu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x1012bfb; continue; }
  pc = 0x1012bf9; continue;
 }
 0x1012bf9 => {
  pc = 0x1012c14; continue;
 }
 0x1012bfb => {
  { let v: u32 = cpu.sar32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 1u32); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v = cpu.r32(0x11d47acu32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x1012c11); f_010915f5(cpu); if cpu.returned_to != 0x1012c11 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x1012c14; continue;
 }
 0x1012c14 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01012c1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012c1c); }
 let mut pc: u32 = 0x1012c1c;
 loop { match pc {
 0x1012c1c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if cpu.zf { pc = 0x1012c48; continue; }
  pc = 0x1012c36; continue;
 }
 0x1012c36 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x11d4888u32); cpu.push(v); }
  cpu.push(0x1012c45); f_01091b5a(cpu); if cpu.returned_to != 0x1012c45 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x1012c48; continue;
 }
 0x1012c48 => {
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
pub fn f_01012c51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012c51); }
 let mut pc: u32 = 0x1012c51;
 loop { match pc {
 0x1012c51 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x110de8au32)); cpu.w32(0x110de8au32, v); }
  { let v = 0x4000u32; cpu.push(v); }
  { let v = cpu.r32(0x11d4878u32); cpu.push(v); }
  { let v = cpu.r32(0x11d4870u32); cpu.push(v); }
  { let v = cpu.r32(0x110de86u32); cpu.push(v); }
  cpu.push(0x1012c86); f_01091f8c(cpu); if cpu.returned_to != 0x1012c86 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11d4880u32).wrapping_add(0x4000u32); cpu.w32(0x11d4880u32, v); }
  { let v: u32 = cpu.r32(0x11d4878u32).wrapping_add(0x4000u32); cpu.w32(0x11d4878u32, v); }
  { let v: u32 = cpu.r32(0x11d4878u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11d487cu32)); }
  if !cpu.zf { pc = 0x1012cb4; continue; }
  pc = 0x1012caa; continue;
 }
 0x1012caa => {
  { let v: u32 = cpu.r32(0x11d4874u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d4878u32, v); }
  pc = 0x1012cb4; continue;
 }
 0x1012cb4 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x11d4870u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d4870u32, v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01012ccb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012ccb); }
 let mut pc: u32 = 0x1012ccb;
 loop { match pc {
 0x1012ccb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if !cpu.zf { pc = 0x1012cf0; continue; }
  pc = 0x1012ce7; continue;
 }
 0x1012ce7 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012d16; continue;
 }
 0x1012cf0 => {
  cpu.push(0x1012cf5); f_01012e61(cpu); if cpu.returned_to != 0x1012cf5 { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d4884u32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d486cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d4870u32, v); }
  { let v: u32 = 0x4000u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012d16; continue;
 }
 0x1012d16 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01012d21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012d21); }
 let mut pc: u32 = 0x1012d21;
 loop { match pc {
 0x1012d21 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if cpu.zf { pc = 0x1012d43; continue; }
  pc = 0x1012d3d; continue;
 }
 0x1012d3d => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1012d45; continue; }
  pc = 0x1012d43; continue;
 }
 0x1012d43 => {
  pc = 0x1012d4b; continue;
 }
 0x1012d45 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1012d50; continue; }
  pc = 0x1012d4b; continue;
 }
 0x1012d4b => {
  pc = 0x1012e59; continue;
 }
 0x1012d50 => {
  { let v = cpu.r32(0x11d4888u32); cpu.push(v); }
  cpu.push(0x1012d5b); f_01090e8d(cpu); if cpu.returned_to != 0x1012d5b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110de86u32, v); }
  { cpu.sub32(cpu.r32(0x110de86u32), 0x0u32); }
  if !cpu.zf { pc = 0x1012d76; continue; }
  pc = 0x1012d6c; continue;
 }
 0x1012d6c => {
  { let v: u32 = 0xfffffff8u32; cpu.eax = v; }
  cpu.push(0x1012d76); f_0101364c(cpu); if cpu.returned_to != 0x1012d76 { return; }
  pc = 0x1012d76; continue;
 }
 0x1012d76 => {
  { let v = cpu.r32(0x110de86u32); cpu.push(v); }
  cpu.push(0x1012d81); f_010910de(cpu); if cpu.returned_to != 0x1012d81 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x11d486cu32), 0x0u32); }
  if !cpu.zf { pc = 0x1012da1; continue; }
  pc = 0x1012d8d; continue;
 }
 0x1012d8d => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.r32(0x110de86u32); cpu.push(v); }
  cpu.push(0x1012d9c); f_010912d3(cpu); if cpu.returned_to != 0x1012d9c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1012db3; continue;
 }
 0x1012da1 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.r32(0x110de86u32); cpu.push(v); }
  cpu.push(0x1012db0); f_010912d3(cpu); if cpu.returned_to != 0x1012db0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x1012db3; continue;
 }
 0x1012db3 => {
  { let v = cpu.r32(0x11d4884u32); cpu.push(v); }
  { let v = cpu.r32(0x110de86u32); cpu.push(v); }
  cpu.push(0x1012dc4); f_01091509(cpu); if cpu.returned_to != 0x1012dc4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = cpu.r32(0x110de86u32); cpu.push(v); }
  cpu.push(0x1012dd4); f_010915f5(cpu); if cpu.returned_to != 0x1012dd4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x7fu32; cpu.push(v); }
  { let v = cpu.r32(0x110de86u32); cpu.push(v); }
  cpu.push(0x1012de4); f_0109157f(cpu); if cpu.returned_to != 0x1012de4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1012c51u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x110de86u32); cpu.push(v); }
  cpu.push(0x1012df8); f_01092263(cpu); if cpu.returned_to != 0x1012df8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d4874u32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d4878u32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d487cu32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x11d4870u32, v); }
  pc = 0x1012e1d; continue;
 }
 0x1012e1d => {
  cpu.push(0x1012e22); f_01012ebc(cpu); if cpu.returned_to != 0x1012e22 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x1012e1d; continue; }
  pc = 0x1012e26; continue;
 }
 0x1012e26 => {
  { let v = 0x4000u32; cpu.push(v); }
  { let v = cpu.r32(0x11d4878u32); cpu.push(v); }
  { let v = cpu.r32(0x11d4870u32); cpu.push(v); }
  { let v = cpu.r32(0x110de86u32); cpu.push(v); }
  cpu.push(0x1012e42); f_01091f8c(cpu); if cpu.returned_to != 0x1012e42 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d4880u32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x11d4878u32), 0x4000u32); cpu.w32(0x11d4878u32, v); }
  pc = 0x1012e59; continue;
 }
 0x1012e59 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01012e61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012e61); }
 let mut pc: u32 = 0x1012e61;
 loop { match pc {
 0x1012e61 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x0u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if cpu.zf { pc = 0x1012e81; continue; }
  pc = 0x1012e78; continue;
 }
 0x1012e78 => {
  { cpu.sub32(cpu.r32(0x110de86u32), 0x0u32); }
  if !cpu.zf { pc = 0x1012e83; continue; }
  pc = 0x1012e81; continue;
 }
 0x1012e81 => {
  pc = 0x1012eb5; continue;
 }
 0x1012e83 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.r32(0x110de86u32); cpu.push(v); }
  cpu.push(0x1012e90); f_01092263(cpu); if cpu.returned_to != 0x1012e90 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.r32(0x110de86u32); cpu.push(v); }
  cpu.push(0x1012e9e); f_01091071(cpu); if cpu.returned_to != 0x1012e9e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x110de86u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x11d4870u32, v); }
  pc = 0x1012eb5; continue;
 }
 0x1012eb5 => {
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
pub fn f_01012ebc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012ebc); }
 let mut pc: u32 = 0x1012ebc;
 loop { match pc {
 0x1012ebc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if cpu.zf { pc = 0x1012edc; continue; }
  pc = 0x1012ed3; continue;
 }
 0x1012ed3 => {
  { cpu.sub32(cpu.r32(0x110de86u32), 0x0u32); }
  if !cpu.zf { pc = 0x1012ee5; continue; }
  pc = 0x1012edc; continue;
 }
 0x1012edc => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012f2c; continue;
 }
 0x1012ee5 => {
  { cpu.sub32(cpu.r32(0x11d4870u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x1012ef7; continue; }
  pc = 0x1012eee; continue;
 }
 0x1012eee => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012f2c; continue;
 }
 0x1012ef7 => {
  { let v = cpu.r32(0x110de86u32); cpu.push(v); }
  cpu.push(0x1012f02); f_01091ea7(cpu); if cpu.returned_to != 0x1012f02 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d4870u32, v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if cpu.zf { pc = 0x1012f1f; continue; }
  pc = 0x1012f16; continue;
 }
 0x1012f16 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012f26; continue;
 }
 0x1012f1f => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012f26; continue;
 }
 0x1012f26 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012f2c; continue;
 }
 0x1012f2c => {
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
pub fn f_01012f38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012f38); }
 let mut pc: u32 = 0x1012f38;
 loop { match pc {
 0x1012f38 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.r32(0x110de86u32); cpu.push(v); }
  cpu.push(0x1012f51); f_010916e1(cpu); if cpu.returned_to != 0x1012f51 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.zf { pc = 0x1012f62; continue; }
  pc = 0x1012f59; continue;
 }
 0x1012f59 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012f69; continue;
 }
 0x1012f62 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1012f69; continue;
 }
 0x1012f69 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_01012f7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012f7b); }
 let mut pc: u32 = 0x1012f7b;
 loop { match pc {
 0x1012f7b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.r32(0x110de86u32); cpu.push(v); }
  cpu.push(0x1012f94); f_01092091(cpu); if cpu.returned_to != 0x1012f94 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x11d4880u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_01012fae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1012fae); }
 let mut pc: u32 = 0x1012fae;
 loop { match pc {
 0x1012fae => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de5cu32), 0x0u32); }
  if cpu.zf { pc = 0x1012fd5; continue; }
  pc = 0x1012fc8; continue;
 }
 0x1012fc8 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d0fbcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x1012fd7; continue; }
  pc = 0x1012fd5; continue;
 }
 0x1012fd5 => {
  pc = 0x1013026; continue;
 }
 0x1012fd7 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v = cpu.r32(0x11d0fc4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x1012fe6); f_01092d4f(cpu); if cpu.returned_to != 0x1012fe6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v = cpu.r32(0x11d0fc4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x1012ff8); f_01092aa2(cpu); if cpu.returned_to != 0x1012ff8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d0fbcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x101300a); f_010120f7(cpu); if cpu.returned_to != 0x101300a { return; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d0fbcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x11d0fc0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1013026; continue;
 }
 0x1013026 => {
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
pub fn f_0101302f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101302f); }
 let mut pc: u32 = 0x101302f;
 loop { match pc {
 0x101302f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013044; continue;
 }
 0x1013044 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x8u32); }
  if (cpu.sf != cpu.of) { pc = 0x1013054; continue; }
  pc = 0x101304a; continue;
 }
 0x101304a => {
  pc = 0x101305e; continue;
 }
 0x101304c => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1013044; continue;
 }
 0x1013054 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x101305c); f_01012fae(cpu); if cpu.returned_to != 0x101305c { return; }
  cpu.poll(); pc = 0x101304c; continue;
 }
 0x101305e => {
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
pub fn f_01013067(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1013067); }
 let mut pc: u32 = 0x1013067;
 loop { match pc {
 0x1013067 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x101307f); f_01011cbe(cpu); if cpu.returned_to != 0x101307f { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de5cu32), 0x0u32); }
  if !cpu.zf { pc = 0x1013097; continue; }
  pc = 0x101308b; continue;
 }
 0x101308b => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013143; continue;
 }
 0x1013097 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d0fbcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x10130b4; continue; }
  pc = 0x10130a4; continue;
 }
 0x10130a4 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10130ac); f_0101329b(cpu); if cpu.returned_to != 0x10130ac { return; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10130b4); f_01012fae(cpu); if cpu.returned_to != 0x10130b4 { return; }
  pc = 0x10130b4; continue;
 }
 0x10130b4 => {
  cpu.push(0x10130b9); f_010139f8(cpu); if cpu.returned_to != 0x10130b9 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v: u32 = 0x11d0fbcu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10130d6); f_01012020(cpu); if cpu.returned_to != 0x10130d6 { return; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0fbcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d0fbcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x10130f6; continue; }
  pc = 0x10130ed; continue;
 }
 0x10130ed => {
  { let v: u32 = 0xfffffffeu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013143; continue;
 }
 0x10130f6 => {
  { let v = cpu.r32(0x11d4890u32); cpu.push(v); }
  cpu.push(0x1013101); f_010929bd(cpu); if cpu.returned_to != 0x1013101 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1013117; continue; }
  pc = 0x101310d; continue;
 }
 0x101310d => {
  { let v: u32 = 0xfffffff8u32; cpu.eax = v; }
  cpu.push(0x1013117); f_0101364c(cpu); if cpu.returned_to != 0x1013117 { return; }
  pc = 0x1013117; continue;
 }
 0x1013117 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v = cpu.r32(0x11d0fbcu32.wrapping_add(cpu.eax)); cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101312c); f_01092b0f(cpu); if cpu.returned_to != 0x101312c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d0fc4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013143; continue;
 }
 0x1013143 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_0101314e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101314e); }
 let mut pc: u32 = 0x101314e;
 loop { match pc {
 0x101314e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x0u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x110de8eu32, v); }
  { cpu.sub32(cpu.r32(0x110de7cu32), 0x0u32); }
  if cpu.zf { pc = 0x101318d; continue; }
  pc = 0x101316d; continue;
 }
 0x101316d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11d489au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101317e); f_01093d39(cpu); if cpu.returned_to != 0x101317e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x110de7cu32, v); }
  pc = 0x10131a3; continue;
 }
 0x101318d => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x10131a3; continue; }
  pc = 0x1013193; continue;
 }
 0x1013193 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10131a0); f_01093d39(cpu); if cpu.returned_to != 0x10131a0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x10131a3; continue;
 }
 0x10131a3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010131a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10131a8); }
 let mut pc: u32 = 0x10131a8;
 loop { match pc {
 0x10131a8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de5cu32), 0x0u32); }
  if cpu.zf { pc = 0x10131d3; continue; }
  pc = 0x10131c6; continue;
 }
 0x10131c6 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d0fbcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x10131d5; continue; }
  pc = 0x10131d3; continue;
 }
 0x10131d3 => {
  pc = 0x101321d; continue;
 }
 0x10131d5 => {
  { let v: u32 = 0x0u32; cpu.w32(0x110de7cu32, v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x10131eb; continue; }
  pc = 0x10131e5; continue;
 }
 0x10131e5 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10131ed; continue; }
  pc = 0x10131eb; continue;
 }
 0x10131eb => {
  pc = 0x101321d; continue;
 }
 0x10131ed => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x101320b; continue; }
  pc = 0x10131f3; continue;
 }
 0x10131f3 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v = cpu.r32(0x11d0fc4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x1013206); f_01093d39(cpu); if cpu.returned_to != 0x1013206 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x101321d; continue;
 }
 0x101320b => {
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11d489au32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x110de7cu32, v); }
  pc = 0x101321d; continue;
 }
 0x101321d => {
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
pub fn f_01013224(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1013224); }
 let mut pc: u32 = 0x1013224;
 loop { match pc {
 0x1013224 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de5cu32), 0x0u32); }
  if cpu.zf { pc = 0x101324b; continue; }
  pc = 0x101323e; continue;
 }
 0x101323e => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d0fbcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x1013254; continue; }
  pc = 0x101324b; continue;
 }
 0x101324b => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x101328f; continue;
 }
 0x1013254 => {
  { let v: u32 = 0x0u32; cpu.w32(0x110de7cu32, v); }
  { let v: u32 = 0x101314eu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v = cpu.r32(0x11d0fc4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x1013273); f_01093e9c(cpu); if cpu.returned_to != 0x1013273 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v = cpu.r32(0x11d0fc4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x1013285); f_01092c08(cpu); if cpu.returned_to != 0x1013285 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x101328f; continue;
 }
 0x101328f => {
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
pub fn f_0101329b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101329b); }
 let mut pc: u32 = 0x101329b;
 loop { match pc {
 0x101329b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de5cu32), 0x0u32); }
  if cpu.zf { pc = 0x10132c2; continue; }
  pc = 0x10132b5; continue;
 }
 0x10132b5 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d0fbcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x10132c4; continue; }
  pc = 0x10132c2; continue;
 }
 0x10132c2 => {
  pc = 0x10132e4; continue;
 }
 0x10132c4 => {
  { cpu.sub32(cpu.r32(0x110de64u32), 0x0u32); }
  if cpu.zf { pc = 0x10132d2; continue; }
  pc = 0x10132cd; continue;
 }
 0x10132cd => {
  cpu.push(0x10132d2); f_010129b5(cpu); if cpu.returned_to != 0x10132d2 { return; }
  pc = 0x10132d2; continue;
 }
 0x10132d2 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v = cpu.r32(0x11d0fc4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x10132e1); f_01092c75(cpu); if cpu.returned_to != 0x10132e1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10132e4; continue;
 }
 0x10132e4 => {
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
pub fn f_010132ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10132ed); }
 let mut pc: u32 = 0x10132ed;
 loop { match pc {
 0x10132ed => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013302; continue;
 }
 0x1013302 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x8u32); }
  if (cpu.sf != cpu.of) { pc = 0x1013312; continue; }
  pc = 0x1013308; continue;
 }
 0x1013308 => {
  pc = 0x101331c; continue;
 }
 0x101330a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1013302; continue;
 }
 0x1013312 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x101331a); f_0101329b(cpu); if cpu.returned_to != 0x101331a { return; }
  cpu.poll(); pc = 0x101330a; continue;
 }
 0x101331c => {
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
pub fn f_01013325(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1013325); }
 let mut pc: u32 = 0x1013325;
 loop { match pc {
 0x1013325 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de5cu32), 0x0u32); }
  if cpu.zf { pc = 0x101334c; continue; }
  pc = 0x101333f; continue;
 }
 0x101333f => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d0fbcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x1013355; continue; }
  pc = 0x101334c; continue;
 }
 0x101334c => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013382; continue;
 }
 0x1013355 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v = cpu.r32(0x11d0fc4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x1013364); f_01092f36(cpu); if cpu.returned_to != 0x1013364 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.zf { pc = 0x1013375; continue; }
  pc = 0x101336c; continue;
 }
 0x101336c => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x101337c; continue;
 }
 0x1013375 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x101337c; continue;
 }
 0x101337c => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013382; continue;
 }
 0x1013382 => {
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
pub fn f_010133d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10133d9); }
 let mut pc: u32 = 0x10133d9;
 loop { match pc {
 0x10133d9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de5cu32), 0x0u32); }
  if cpu.zf { pc = 0x1013405; continue; }
  pc = 0x10133f3; continue;
 }
 0x10133f3 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x11d4890u32); cpu.push(v); }
  cpu.push(0x1013402); f_010932ca(cpu); if cpu.returned_to != 0x1013402 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x1013405; continue;
 }
 0x1013405 => {
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
pub fn f_0101340e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101340e); }
 let mut pc: u32 = 0x101340e;
 loop { match pc {
 0x101340e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de5cu32), 0x0u32); }
  if cpu.zf { pc = 0x1013437; continue; }
  pc = 0x101342a; continue;
 }
 0x101342a => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d0fbcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x1013439; continue; }
  pc = 0x1013437; continue;
 }
 0x1013437 => {
  pc = 0x1013455; continue;
 }
 0x1013439 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x3e8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xcu32); cpu.eax = v; }
  { let v = cpu.r32(0x11d0fc4u32.wrapping_add(cpu.eax)); cpu.push(v); }
  cpu.push(0x1013452); f_01092e3e(cpu); if cpu.returned_to != 0x1013452 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x1013455; continue;
 }
 0x1013455 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_0101345d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101345d); }
 let mut pc: u32 = 0x101345d;
 loop { match pc {
 0x101345d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de68u32), 0x0u32); }
  if !cpu.zf { pc = 0x1013487; continue; }
  pc = 0x101347b; continue;
 }
 0x101347b => {
  { let v: u32 = 0xfffffffdu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10135e5; continue;
 }
 0x1013487 => {
  cpu.push(0x101348c); f_01011cbe(cpu); if cpu.returned_to != 0x101348c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110de80u32, v); }
  cpu.push(0x101349d); f_010013e0(cpu); if cpu.returned_to != 0x101349d { return; }
  cpu.push(0x10134a2); f_0101211f(cpu); if cpu.returned_to != 0x10134a2 { return; }
  cpu.push(0x10134a7); f_0101217c(cpu); if cpu.returned_to != 0x10134a7 { return; }
  cpu.push(0x10134ac); f_0108f909(cpu); if cpu.returned_to != 0x10134ac { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x110de58u32, v); }
  { let v: u32 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10134c2); f_0108fb18(cpu); if cpu.returned_to != 0x10134c2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  cpu.push(0x10134ce); f_0108fb18(cpu); if cpu.returned_to != 0x10134ce { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x11d4888u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10134dc); f_01090c42(cpu); if cpu.returned_to != 0x10134dc { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10134f2; continue; }
  pc = 0x10134e8; continue;
 }
 0x10134e8 => {
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x1013505; continue; }
  pc = 0x10134ee; continue;
 }
 0x10134ee => {
  pc = 0x1013518; continue;
 }
 0x10134f2 => {
  { let v: u32 = 0x1u32; cpu.w32(0x110de54u32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1105e5bu32, v); }
  pc = 0x1013533; continue;
 }
 0x1013505 => {
  { let v: u32 = 0x0u32; cpu.w32(0x110de54u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1105e5bu32, v); }
  pc = 0x1013533; continue;
 }
 0x1013518 => {
  cpu.push(0x101351d); f_0108fa85(cpu); if cpu.returned_to != 0x101351d { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x110de58u32, v); }
  { let v: u32 = 0xfffffffau32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10135e5; continue;
 }
 0x1013533 => {
  { let v: u32 = 0x11d4890u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101353e); f_0109277e(cpu); if cpu.returned_to != 0x101353e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x1013554; continue; }
  pc = 0x101354a; continue;
 }
 0x101354a => {
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x1013567; continue; }
  pc = 0x1013550; continue;
 }
 0x1013550 => {
  pc = 0x101357a; continue;
 }
 0x1013554 => {
  { let v: u32 = 0x1u32; cpu.w32(0x110de5cu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x1105e5cu32, v); }
  pc = 0x1013592; continue;
 }
 0x1013567 => {
  { let v: u32 = 0x0u32; cpu.w32(0x110de5cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x1105e5cu32, v); }
  pc = 0x1013592; continue;
 }
 0x101357a => {
  cpu.push(0x101357f); f_0108fa85(cpu); if cpu.returned_to != 0x101357f { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x110de58u32, v); }
  { let v: u32 = 0xfffffff9u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10135e5; continue;
 }
 0x1013592 => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x10135de; continue; }
  pc = 0x1013598; continue;
 }
 0x1013598 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10135a1); f_01090372(cpu); if cpu.returned_to != 0x10135a1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d488cu32, v); }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x11d488cu32); cpu.push(v); }
  cpu.push(0x10135b9); f_010905c6(cpu); if cpu.returned_to != 0x10135b9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.r32(0x11d488cu32); cpu.push(v); }
  cpu.push(0x10135c7); f_01090790(cpu); if cpu.returned_to != 0x10135c7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x110de60u32, v); }
  cpu.push(0x10135d9); f_010906b2(cpu); if cpu.returned_to != 0x10135d9 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d4894u32, v); }
  pc = 0x10135de; continue;
 }
 0x10135de => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10135e5; continue;
 }
 0x10135e5 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_010135ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10135ef); }
 let mut pc: u32 = 0x10135ef;
 loop { match pc {
 0x10135ef => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x0u32); cpu.esp = v; }
  cpu.push(0x1013602); f_010132ed(cpu); if cpu.returned_to != 0x1013602 { return; }
  cpu.push(0x1013607); f_010129b5(cpu); if cpu.returned_to != 0x1013607 { return; }
  { cpu.sub32(cpu.r32(0x110de60u32), 0x0u32); }
  if cpu.zf { pc = 0x1013628; continue; }
  pc = 0x1013610; continue;
 }
 0x1013610 => {
  { let v = cpu.r32(0x11d488cu32); cpu.push(v); }
  cpu.push(0x101361b); f_0109092e(cpu); if cpu.returned_to != 0x101361b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x110de60u32, v); }
  pc = 0x1013628; continue;
 }
 0x1013628 => {
  { cpu.sub32(cpu.r32(0x110de58u32), 0x1u32); }
  if !cpu.zf { pc = 0x1013636; continue; }
  pc = 0x1013631; continue;
 }
 0x1013631 => {
  cpu.push(0x1013636); f_0108fa85(cpu); if cpu.returned_to != 0x1013636 { return; }
  pc = 0x1013636; continue;
 }
 0x1013636 => {
  cpu.push(0x101363b); f_0101302f(cpu); if cpu.returned_to != 0x101363b { return; }
  cpu.push(0x1013640); f_0101223e(cpu); if cpu.returned_to != 0x1013640 { return; }
  cpu.push(0x1013645); f_01011c3b(cpu); if cpu.returned_to != 0x1013645 { return; }
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
pub fn f_0101364c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101364c); } r_0101364c(cpu, 0x101364c); }
pub fn f_0101367b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101367b); } r_0101364c(cpu, 0x101367b); }
pub fn f_010136c5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10136c5); } r_0101364c(cpu, 0x10136c5); }
pub fn f_010136d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10136d8); } r_0101364c(cpu, 0x10136d8); }
pub fn f_010136e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10136e8); } r_0101364c(cpu, 0x10136e8); }
pub fn f_010136f8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10136f8); } r_0101364c(cpu, 0x10136f8); }
pub fn f_01013708(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1013708); } r_0101364c(cpu, 0x1013708); }
pub fn f_01013718(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1013718); } r_0101364c(cpu, 0x1013718); }
pub fn f_01013728(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1013728); } r_0101364c(cpu, 0x1013728); }
fn r_0101364c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x101364c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x1013697; continue; }
  pc = 0x1013663; continue;
 }
 0x1013663 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136875u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x101366d); cpu.push(0x1013672); f_0108cf85(cpu); if cpu.returned_to != 0x1013672 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x1013746; continue;
 }
 0x101367b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.sbb8(cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), ((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])) | ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { cpu.cf = false; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.flat_ds(0x101368b); cpu.push(0x1013690); cpu.call(0xd90237c6); if cpu.returned_to != 0x1013690 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let at = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.esi = v; cpu.set_segment(3, selector); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1013697; continue;
 }
 0x1013697 => {
  { let v: u32 = 0x1136881u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x101369d); cpu.push(0x10136a2); f_0108cf85(cpu); if cpu.returned_to != 0x10136a2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x7u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x6u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1013738; continue; }
  pc = 0x10136b8; continue;
 }
 0x10136b8 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x101367bu32.wrapping_add(cpu.eax)); continue;
 }
 0x10136c5 => {
  { let v: u32 = 0x1136893u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x10136cb); cpu.push(0x10136d0); f_0108cf85(cpu); if cpu.returned_to != 0x10136d0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1013746; continue;
 }
 0x10136d8 => {
  { let v: u32 = 0x11368a5u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x10136de); cpu.push(0x10136e3); f_0108cf85(cpu); if cpu.returned_to != 0x10136e3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1013746; continue;
 }
 0x10136e8 => {
  { let v: u32 = 0x11368b4u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x10136ee); cpu.push(0x10136f3); f_0108cf85(cpu); if cpu.returned_to != 0x10136f3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1013746; continue;
 }
 0x10136f8 => {
  { let v: u32 = 0x11368beu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x10136fe); cpu.push(0x1013703); f_0108cf85(cpu); if cpu.returned_to != 0x1013703 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1013746; continue;
 }
 0x1013708 => {
  { let v: u32 = 0x11368ceu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x101370e); cpu.push(0x1013713); f_0108cf85(cpu); if cpu.returned_to != 0x1013713 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1013746; continue;
 }
 0x1013718 => {
  { let v: u32 = 0x11368deu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x101371e); cpu.push(0x1013723); f_0108cf85(cpu); if cpu.returned_to != 0x1013723 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1013746; continue;
 }
 0x1013728 => {
  { let v: u32 = 0x11368efu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x101372e); cpu.push(0x1013733); f_0108cf85(cpu); if cpu.returned_to != 0x1013733 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1013746; continue;
 }
 0x1013738 => {
  { let v: u32 = 0x1136900u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x101373e); cpu.push(0x1013743); f_0108cf85(cpu); if cpu.returned_to != 0x1013743 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x1013746; continue;
 }
 0x1013746 => {
  cpu.flat_ds(0x1013746); cpu.push(0x101374b); f_010135ef(cpu); if cpu.returned_to != 0x101374b { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.flat_ds(0x1013750); cpu.push(0x1013755); f_0108dd4b(cpu); if cpu.returned_to != 0x1013755 { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x101375d); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0101375e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101375e); } r_0101375e(cpu, 0x101375e); }
pub fn f_010137f9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10137f9); } r_0101375e(cpu, 0x10137f9); }
pub fn f_01013811(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1013811); } r_0101375e(cpu, 0x1013811); }
pub fn f_01013829(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1013829); } r_0101375e(cpu, 0x1013829); }
pub fn f_01013841(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1013841); } r_0101375e(cpu, 0x1013841); }
fn r_0101375e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x101375e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if !cpu.zf { pc = 0x1013786; continue; }
  pc = 0x101377a; continue;
 }
 0x101377a => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10138a8; continue;
 }
 0x1013786 => {
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if !cpu.zf { pc = 0x101379b; continue; }
  pc = 0x101378f; continue;
 }
 0x101378f => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10138a8; continue;
 }
 0x101379b => {
  { cpu.sub32(cpu.r32(0x11d1028u32), 0x0u32); }
  if cpu.zf { pc = 0x10137ae; continue; }
  pc = 0x10137a4; continue;
 }
 0x10137a4 => {
  { let v: u32 = cpu.r32(0x110de70u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x10137b0; continue; }
  pc = 0x10137ae; continue;
 }
 0x10137ae => {
  pc = 0x10137be; continue;
 }
 0x10137b0 => {
  { let v: u32 = cpu.r32(0x110de74u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x101389a; continue; }
  pc = 0x10137be; continue;
 }
 0x10137be => {
  pc = 0x10137d3; continue;
 }
 0x10137d3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x10137da); f_010121ba(cpu); if cpu.returned_to != 0x10137da { return; }
  { let v: u8 = cpu.r8(0x10fd44bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1013859; continue; }
  pc = 0x10137ea; continue;
 }
 0x10137ea => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x10137c3u32.wrapping_add(cpu.eax)); continue;
 }
 0x10137f9 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136908u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101380c); f_0108dece(cpu); if cpu.returned_to != 0x101380c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x101386f; continue;
 }
 0x1013811 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136915u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1013824); f_0108dece(cpu); if cpu.returned_to != 0x1013824 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x101386f; continue;
 }
 0x1013829 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136922u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101383c); f_0108dece(cpu); if cpu.returned_to != 0x101383c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x101386f; continue;
 }
 0x1013841 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x113692fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1013854); f_0108dece(cpu); if cpu.returned_to != 0x1013854 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x101386f; continue;
 }
 0x1013859 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x113693cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101386c); f_0108dece(cpu); if cpu.returned_to != 0x101386c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x101386f; continue;
 }
 0x101386f => {
  { let v = 0x7fu32; cpu.push(v); }
  { let v = 0x2b11u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xffu32; cpu.ebx = v; }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x1013889); f_01012282(cpu); if cpu.returned_to != 0x1013889 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x101389a; continue; }
  pc = 0x1013892; continue;
 }
 0x1013892 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10138a8; continue;
 }
 0x101389a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x10138a1); f_01012780(cpu); if cpu.returned_to != 0x10138a1 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10138a8; continue;
 }
 0x10138a8 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_010137c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10137c3); }
 let mut pc: u32 = 0x10137c3;
 loop { match pc {
 0x10137c3 => {
  { cpu.cf = true; }
  cpu.unimplemented(0x10137c4, "Aaa");
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.edi); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_sub(cpu.edi); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), (cpu.eax as u8)); }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 224u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x4ca237db); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010138b3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10138b3); } r_010138b3(cpu, 0x10138b3); }
pub fn f_01013907(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1013907); } r_010138b3(cpu, 0x1013907); }
pub fn f_0101393d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101393d); } r_010138b3(cpu, 0x101393d); }
pub fn f_01013955(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1013955); } r_010138b3(cpu, 0x1013955); }
pub fn f_0101396d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101396d); } r_010138b3(cpu, 0x101396d); }
pub fn f_01013985(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1013985); } r_010138b3(cpu, 0x1013985); }
fn r_010138b3(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10138b3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x110de54u32), 0x0u32); }
  if !cpu.zf { pc = 0x10138d9; continue; }
  pc = 0x10138cd; continue;
 }
 0x10138cd => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10139ec; continue;
 }
 0x10138d9 => {
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if !cpu.zf { pc = 0x10138ee; continue; }
  pc = 0x10138e2; continue;
 }
 0x10138e2 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10139ec; continue;
 }
 0x10138ee => {
  { cpu.sub32(cpu.r32(0x11d1028u32), 0x0u32); }
  if cpu.zf { pc = 0x1013905; continue; }
  pc = 0x10138f7; continue;
 }
 0x10138f7 => {
  { let v: u32 = cpu.r32(0x110de78u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x10139de; continue; }
  pc = 0x1013905; continue;
 }
 0x1013905 => {
  pc = 0x1013917; continue;
 }
 0x1013907 => {
  { cpu.sub32(cpu.eax, 0x55010139u32); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); }
  { let v: u32 = cpu.r32(0x39u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.ebp); cpu.w32(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.edi); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x1013917; continue;
 }
 0x1013917 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x101391e); f_010121ba(cpu); if cpu.returned_to != 0x101391e { return; }
  { let v: u8 = cpu.r8(0x10fd44bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x101399d; continue; }
  pc = 0x101392e; continue;
 }
 0x101392e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x1013907u32.wrapping_add(cpu.eax)); continue;
 }
 0x101393d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136949u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1013950); f_0108dece(cpu); if cpu.returned_to != 0x1013950 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10139b3; continue;
 }
 0x1013955 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136957u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1013968); f_0108dece(cpu); if cpu.returned_to != 0x1013968 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10139b3; continue;
 }
 0x101396d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136965u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1013980); f_0108dece(cpu); if cpu.returned_to != 0x1013980 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10139b3; continue;
 }
 0x1013985 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136973u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1013998); f_0108dece(cpu); if cpu.returned_to != 0x1013998 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10139b3; continue;
 }
 0x101399d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136981u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10139b0); f_0108dece(cpu); if cpu.returned_to != 0x10139b0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10139b3; continue;
 }
 0x10139b3 => {
  { let v = 0x7fu32; cpu.push(v); }
  { let v = 0x2b11u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0xffu32; cpu.ebx = v; }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x10139cd); f_01012282(cpu); if cpu.returned_to != 0x10139cd { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x10139de; continue; }
  pc = 0x10139d6; continue;
 }
 0x10139d6 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10139ec; continue;
 }
 0x10139de => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x10139e5); f_01012780(cpu); if cpu.returned_to != 0x10139e5 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10139ec; continue;
 }
 0x10139ec => {
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
pub fn f_010139f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10139f8); }
 let mut pc: u32 = 0x10139f8;
 loop { match pc {
 0x10139f8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.r32(0x11d4890u32); cpu.push(v); }
  cpu.push(0x1013a11); f_0109351e(cpu); if cpu.returned_to != 0x1013a11 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if !cpu.zf { pc = 0x1013a21; continue; }
  pc = 0x1013a1d; continue;
 }
 0x1013a1d => {
  { let v: u8 = 0x5u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013a21; continue;
 }
 0x1013a21 => {
  { cpu.sub8(cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1013a2f; continue; }
  pc = 0x1013a27; continue;
 }
 0x1013a27 => {
  cpu.push(0x1013a2c); f_01013a68(cpu); if cpu.returned_to != 0x1013a2c { return; }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u8)); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013a2f; continue;
 }
 0x1013a2f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.eax, 0x41u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
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
pub fn f_01013a46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1013a46); }
 let mut pc: u32 = 0x1013a46;
 loop { match pc {
 0x1013a46 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x110de54u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
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
pub fn f_01013a68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1013a68); }
 let mut pc: u32 = 0x1013a68;
 loop { match pc {
 0x1013a68 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x19cu32); cpu.esp = v; }
  { let v: u32 = 0x11cea9cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x113698fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1013a8e); f_0108dece(cpu); if cpu.returned_to != 0x1013a8e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0xffffff7cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xfffffe64u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1013aa4); f_01090019(cpu); if cpu.returned_to != 0x1013aa4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1013b93; continue; }
  pc = 0x1013aaf; continue;
 }
 0x1013aaf => {
  { cpu.sub8(cpu.r8(0xfffffe64u32.wrapping_add(cpu.ebp)), 0x63u8); }
  if cpu.zf { pc = 0x1013ac1; continue; }
  pc = 0x1013ab8; continue;
 }
 0x1013ab8 => {
  { cpu.sub8(cpu.r8(0xfffffe64u32.wrapping_add(cpu.ebp)), 0x43u8); }
  if !cpu.zf { pc = 0x1013acd; continue; }
  pc = 0x1013ac1; continue;
 }
 0x1013ac1 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013b9a; continue;
 }
 0x1013acd => {
  { cpu.sub8(cpu.r8(0xfffffe64u32.wrapping_add(cpu.ebp)), 0x79u8); }
  if cpu.zf { pc = 0x1013adf; continue; }
  pc = 0x1013ad6; continue;
 }
 0x1013ad6 => {
  { cpu.sub8(cpu.r8(0xfffffe64u32.wrapping_add(cpu.ebp)), 0x59u8); }
  if !cpu.zf { pc = 0x1013aeb; continue; }
  pc = 0x1013adf; continue;
 }
 0x1013adf => {
  { let v: u32 = 0x7u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013b9a; continue;
 }
 0x1013aeb => {
  { cpu.sub8(cpu.r8(0xfffffe64u32.wrapping_add(cpu.ebp)), 0x47u8); }
  if cpu.zf { pc = 0x1013afd; continue; }
  pc = 0x1013af4; continue;
 }
 0x1013af4 => {
  { cpu.sub8(cpu.r8(0xfffffe64u32.wrapping_add(cpu.ebp)), 0x67u8); }
  if !cpu.zf { pc = 0x1013b39; continue; }
  pc = 0x1013afd; continue;
 }
 0x1013afd => {
  { cpu.sub8(cpu.r8(0xfffffe65u32.wrapping_add(cpu.ebp)), 0x52u8); }
  if cpu.zf { pc = 0x1013b0f; continue; }
  pc = 0x1013b06; continue;
 }
 0x1013b06 => {
  { cpu.sub8(cpu.r8(0xfffffe65u32.wrapping_add(cpu.ebp)), 0x72u8); }
  if !cpu.zf { pc = 0x1013b1b; continue; }
  pc = 0x1013b0f; continue;
 }
 0x1013b0f => {
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013b9a; continue;
 }
 0x1013b1b => {
  { cpu.sub8(cpu.r8(0xfffffe65u32.wrapping_add(cpu.ebp)), 0x45u8); }
  if cpu.zf { pc = 0x1013b2d; continue; }
  pc = 0x1013b24; continue;
 }
 0x1013b24 => {
  { cpu.sub8(cpu.r8(0xfffffe65u32.wrapping_add(cpu.ebp)), 0x65u8); }
  if !cpu.zf { pc = 0x1013b39; continue; }
  pc = 0x1013b2d; continue;
 }
 0x1013b2d => {
  { let v: u32 = 0x4u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013b9a; continue;
 }
 0x1013b39 => {
  { cpu.sub8(cpu.r8(0xfffffe64u32.wrapping_add(cpu.ebp)), 0x45u8); }
  if cpu.zf { pc = 0x1013b4b; continue; }
  pc = 0x1013b42; continue;
 }
 0x1013b42 => {
  { cpu.sub8(cpu.r8(0xfffffe64u32.wrapping_add(cpu.ebp)), 0x65u8); }
  if !cpu.zf { pc = 0x1013b54; continue; }
  pc = 0x1013b4b; continue;
 }
 0x1013b4b => {
  { let v: u32 = 0x5u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013b9a; continue;
 }
 0x1013b54 => {
  { cpu.sub8(cpu.r8(0xfffffe64u32.wrapping_add(cpu.ebp)), 0x52u8); }
  if cpu.zf { pc = 0x1013b66; continue; }
  pc = 0x1013b5d; continue;
 }
 0x1013b5d => {
  { cpu.sub8(cpu.r8(0xfffffe64u32.wrapping_add(cpu.ebp)), 0x72u8); }
  if !cpu.zf { pc = 0x1013b93; continue; }
  pc = 0x1013b66; continue;
 }
 0x1013b66 => {
  { cpu.sub8(cpu.r8(0xfffffee4u32.wrapping_add(cpu.ebp)), 0x4du8); }
  if cpu.zf { pc = 0x1013b78; continue; }
  pc = 0x1013b6f; continue;
 }
 0x1013b6f => {
  { cpu.sub8(cpu.r8(0xfffffee4u32.wrapping_add(cpu.ebp)), 0x6du8); }
  if !cpu.zf { pc = 0x1013b93; continue; }
  pc = 0x1013b78; continue;
 }
 0x1013b78 => {
  { cpu.sub8(cpu.r8(0xfffffee5u32.wrapping_add(cpu.ebp)), 0x54u8); }
  if cpu.zf { pc = 0x1013b8a; continue; }
  pc = 0x1013b81; continue;
 }
 0x1013b81 => {
  { cpu.sub8(cpu.r8(0xfffffee5u32.wrapping_add(cpu.ebp)), 0x74u8); }
  if !cpu.zf { pc = 0x1013b93; continue; }
  pc = 0x1013b8a; continue;
 }
 0x1013b8a => {
  { let v: u32 = 0x6u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013b9a; continue;
 }
 0x1013b93 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1013b9a; continue;
 }
 0x1013b9a => {
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
pub fn f_01013ba6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1013ba6); }
 let mut pc: u32 = 0x1013ba6;
 loop { match pc {
 0x1013ba6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1013bc6); f_01013dec(cpu); if cpu.returned_to != 0x1013bc6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01013bd1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1013bd1); }
 let mut pc: u32 = 0x1013bd1;
 loop { match pc {
 0x1013bd1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1013bf1); f_01013e03(cpu); if cpu.returned_to != 0x1013bf1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01013bfc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1013bfc); } r_01013bfc(cpu, 0x1013bfc); }
pub fn f_01013c00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1013c00); } r_01013bfc(cpu, 0x1013c00); }
fn r_01013bfc(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1013bfc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) & ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1013c00; continue;
 }
 0x1013c00 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) & ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  f_01013c04(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01013c04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1013c04); }
 let mut pc: u32 = 0x1013c04;
 loop { match pc {
 0x1013c04 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x18u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x1013c2f; continue; }
  pc = 0x1013c21; continue;
 }
 0x1013c21 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x1013c2d); f_0108ef05(cpu); if cpu.returned_to != 0x1013c2d { return; }
  pc = 0x1013c45; continue;
 }
 0x1013c2f => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x1013c3a); f_01094bff(cpu); if cpu.returned_to != 0x1013c3a { return; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x1013c45); f_0108e170(cpu); if cpu.returned_to != 0x1013c45 { return; }
  pc = 0x1013c45; continue;
 }
 0x1013c45 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01013c4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1013c4b); }
 let mut pc: u32 = 0x1013c4b;
 loop { match pc {
 0x1013c4b => {
  { let v: u32 = cpu.r32(0x110de98u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x110de98u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01013c6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1013c6b); }
 let mut pc: u32 = 0x1013c6b;
 loop { match pc {
 0x1013c6b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  let t = cpu.r32(0x110de98u32); cpu.push(0x1013c78); cpu.call(t); if cpu.returned_to != 0x1013c78 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1013c88; continue; }
  pc = 0x1013c7e; continue;
 }
 0x1013c7e => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1013c85); f_01013dec(cpu); if cpu.returned_to != 0x1013c85 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x1013c88; continue;
 }
 0x1013c88 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01013c8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1013c8c); }
 let mut pc: u32 = 0x1013c8c;
 loop { match pc {
 0x1013c8c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1013cab; continue; }
  pc = 0x1013c95; continue;
 }
 0x1013c95 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1013ca0); f_01013e03(cpu); if cpu.returned_to != 0x1013ca0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  let t = cpu.r32(0x110de9cu32); cpu.push(0x1013cab); cpu.call(t); if cpu.returned_to != 0x1013cab { return; }
  pc = 0x1013cab; continue;
 }
 0x1013cab => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01013cad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1013cad); }
 let mut pc: u32 = 0x1013cad;
 loop { match pc {
 0x1013cad => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x100u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x1013cd4); f_0108ef70(cpu); if cpu.returned_to != 0x1013cd4 { return; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x1013cdf; continue; }
  pc = 0x1013cdb; continue;
 }
 0x1013cdb => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x1013d0e; continue;
 }
 0x1013cdf => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x1013d0e; continue;
 }
 0x1013d0e => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x1013d2b; continue; }
  pc = 0x1013d12; continue;
 }
 0x1013d12 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1013d28); f_01013d5e(cpu); if cpu.returned_to != 0x1013d28 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x1013d2b; continue;
 }
 0x1013d2b => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01013d32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1013d32); }
 let mut pc: u32 = 0x1013d32;
 loop { match pc {
 0x1013d32 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = 0x101u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x1013d5a); f_0108ef70(cpu); if cpu.returned_to != 0x1013d5a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01013d5e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1013d5e); } r_01013d5e(cpu, 0x1013d5e); }
pub fn f_01013dc7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1013dc7); } r_01013d5e(cpu, 0x1013dc7); }
fn r_01013d5e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1013d5e => {
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if !cpu.cf { pc = 0x1013d71; continue; }
  pc = 0x1013d6d; continue;
 }
 0x1013d6d => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x1013d73; continue;
 }
 0x1013d71 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x1013d73; continue;
 }
 0x1013d73 => {
  if !cpu.cf { pc = 0x1013d77; continue; }
  pc = 0x1013d75; continue;
 }
 0x1013d75 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  pc = 0x1013d77; continue;
 }
 0x1013d77 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x600u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x1013d81; continue;
 }
 0x1013d81 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x1013db6); f_0108ef70(cpu); if cpu.returned_to != 0x1013db6 { return; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1013dc7 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if !cpu.cf { pc = 0x1013dda; continue; }
  pc = 0x1013dd6; continue;
 }
 0x1013dd6 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x1013ddc; continue;
 }
 0x1013dda => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x1013ddc; continue;
 }
 0x1013ddc => {
  if !cpu.cf { pc = 0x1013de0; continue; }
  pc = 0x1013dde; continue;
 }
 0x1013dde => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  pc = 0x1013de0; continue;
 }
 0x1013de0 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x601u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x1013d81; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01013dec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1013dec); }
 let mut pc: u32 = 0x1013dec;
 loop { match pc {
 0x1013dec => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1013dff); f_01013d5e(cpu); if cpu.returned_to != 0x1013dff { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01013e03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1013e03); }
 let mut pc: u32 = 0x1013e03;
 loop { match pc {
 0x1013e03 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1013e16); f_01013dc7(cpu); if cpu.returned_to != 0x1013e16 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0101404c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101404c); } r_0101404c(cpu, 0x101404c); }
pub fn f_010140c1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10140c1); } r_0101404c(cpu, 0x10140c1); }
fn r_0101404c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1014034 => {
  { cpu.and8(cpu.r8(0x2d4u32.wrapping_add(cpu.esp)), 0x1u8); }
  if !cpu.zf { pc = 0x1014045; continue; }
  pc = 0x101403e; continue;
 }
 0x101403e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x1014045); f_0108e26b(cpu); if cpu.returned_to != 0x1014045 { return; }
  pc = 0x1014045; continue;
 }
 0x1014045 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10143c7; continue;
 }
 0x101404c => {
  { let v = 0xacu32; cpu.push(v); }
  { let v: u32 = 0x194u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x290u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1014068); f_01013c04(cpu); if cpu.returned_to != 0x1014068 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x284u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1d0u32.wrapping_add(cpu.esp))); cpu.ecx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x10141db; continue;
 }
 0x1014080 => {
  { let v = 0x18u32; cpu.push(v); }
  { let v: u32 = 0x268u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2dcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1014099); f_01013c04(cpu); if cpu.returned_to != 0x1014099 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x29cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x284u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1d8u32.wrapping_add(cpu.esp))); cpu.ecx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x10141c6; continue;
 }
 0x10140b8 => {
  { let v = 0x8u32; cpu.push(v); }
  { let v: u32 = 0x280u32.wrapping_add(cpu.esp); cpu.eax = v; }
  pc = 0x10140c1; continue;
 }
 0x10140c1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2dcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10140d1); f_01013c04(cpu); if cpu.returned_to != 0x10140d1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x101411f; continue; }
  pc = 0x10140da; continue;
 }
 0x10140da => {
  { let v: u8 = cpu.r8(0x26cu32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x2u8); }
  if cpu.zf { pc = 0x1014111; continue; }
  pc = 0x10140e6; continue;
 }
 0x10140e6 => {
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x1014111; continue; }
  pc = 0x10140eb; continue;
 }
 0x10140eb => {
  { cpu.and8(cpu.r8(0x298u32.wrapping_add(cpu.esp)), 0xfu8); }
  if cpu.zf { pc = 0x1014111; continue; }
  pc = 0x10140f5; continue;
 }
 0x10140f5 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x1014111; continue; }
  pc = 0x10140f9; continue;
 }
 0x10140f9 => {
  { let v: u32 = cpu.r32(0x2d8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x298u32.wrapping_add(cpu.esp)).wrapping_add(cpu.edx); cpu.w32(0x298u32.wrapping_add(cpu.esp), v); }
  pc = 0x1014111; continue;
 }
 0x1014111 => {
  { let v: u32 = cpu.r32(0x298u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x240u32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(4)), v); }
  pc = 0x101411f; continue;
 }
 0x101411f => {
  { let v: u32 = cpu.r32(0x2a4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x298u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x294u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2a4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x240u32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x264u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x280u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.cf { pc = 0x101416e; continue; }
  pc = 0x1014165; continue;
 }
 0x1014165 => {
  { let v: u32 = cpu.edx; cpu.w32(0x2a8u32.wrapping_add(cpu.esp), v); }
  pc = 0x1014175; continue;
 }
 0x101416e => {
  { let v: u32 = cpu.ecx; cpu.w32(0x2a8u32.wrapping_add(cpu.esp), v); }
  pc = 0x1014175; continue;
 }
 0x1014175 => {
  { let v: u8 = cpu.r8(0x1bcu32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x27cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x210u32.wrapping_add(cpu.esp))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2a8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x29cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2dcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10141ad); f_01013c04(cpu); if cpu.returned_to != 0x10141ad { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2a8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x298u32.wrapping_add(cpu.esp)).wrapping_add(cpu.eax); cpu.w32(0x298u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x294u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x10141c6; continue;
 }
 0x10141c6 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x274u32.wrapping_add(cpu.esp))); }
  if cpu.cf { cpu.poll(); pc = 0x10140b8; continue; }
  pc = 0x10141d3; continue;
 }
 0x10141d3 => {
  { let v: u32 = cpu.r32(0x29cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x10141db; continue;
 }
 0x10141db => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1d4u32.wrapping_add(cpu.esp))); }
  if cpu.cf { cpu.poll(); pc = 0x1014080; continue; }
  pc = 0x10141e8; continue;
 }
 0x10141e8 => {
  { let v: u32 = cpu.r32(0x284u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1f8u32.wrapping_add(cpu.esp))); cpu.ecx = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x290u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2dcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x101420f); f_01013c04(cpu); if cpu.returned_to != 0x101420f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x10143a2; continue;
 }
 0x101421b => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x28cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2dcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1014234); f_01013c04(cpu); if cpu.returned_to != 0x1014234 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x28cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x288u32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x1014393; continue; }
  pc = 0x101424f; continue;
 }
 0x101424f => {
  { let v: u32 = cpu.edx; cpu.w32(0x2a0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x284u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1fcu32.wrapping_add(cpu.esp))); cpu.ecx = v; }
  pc = 0x1014267; continue;
 }
 0x1014267 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x2b8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2dcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1014280); f_01013c04(cpu); if cpu.returned_to != 0x1014280 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x2c0u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2dcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x101429c); f_01013c04(cpu); if cpu.returned_to != 0x101429c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x2b4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x1014034; continue; }
  pc = 0x10142b5; continue;
 }
 0x10142b5 => {
  { let v: u8 = cpu.r8(0x2bcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10142d9; continue; }
  pc = 0x10142c5; continue;
 }
 0x10142c5 => {
  { let v: u8 = cpu.r8(0x2bcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x1014034; continue; }
  pc = 0x10142d9; continue;
 }
 0x10142d9 => {
  { let v: u8 = cpu.r8(0x2b4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1014034; continue; }
  pc = 0x10142ed; continue;
 }
 0x10142ed => {
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x2b0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2dcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1014306); f_01013c04(cpu); if cpu.returned_to != 0x1014306 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x2bcu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x101431b); f_01013c04(cpu); if cpu.returned_to != 0x101431b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x2b4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x1014330); f_01013c04(cpu); if cpu.returned_to != 0x1014330 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2acu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.r32(0x1b8u32.wrapping_add(cpu.esp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x101436f; continue; }
  pc = 0x1014348; continue;
 }
 0x1014348 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x2b8u32.wrapping_add(cpu.esp)) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2b0u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x23cu32.wrapping_add(cpu.esp).wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x101436f; continue;
 }
 0x101436f => {
  { let v: u32 = cpu.r32(0x288u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x284u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1fcu32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if cpu.cf { cpu.poll(); pc = 0x1014267; continue; }
  pc = 0x101438c; continue;
 }
 0x101438c => {
  { let v: u32 = cpu.r32(0x2a0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x1014393; continue;
 }
 0x1014393 => {
  { let v: u32 = cpu.r32(0x288u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x10143a2; continue;
 }
 0x10143a2 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1a4u32.wrapping_add(cpu.esp))); }
  if cpu.cf { cpu.poll(); pc = 0x101421b; continue; }
  pc = 0x10143af; continue;
 }
 0x10143af => {
  { cpu.and8(cpu.r8(0x2d4u32.wrapping_add(cpu.esp)), 0x1u8); }
  if !cpu.zf { pc = 0x10143c0; continue; }
  pc = 0x10143b9; continue;
 }
 0x10143b9 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10143c0); f_0108e26b(cpu); if cpu.returned_to != 0x10143c0 { return; }
  pc = 0x10143c0; continue;
 }
 0x10143c0 => {
  { let v: u32 = cpu.r32(0x2d8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x10143c7; continue;
 }
 0x10143c7 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x2c0u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010140ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10140ff); }
 let mut pc: u32 = 0x10140ff;
 loop { match pc {
 0x10140ff => {
  { let v: u8 = cpu.r8(0x10ba0fe0u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x10ba0fe0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(37893);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010143d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10143d7); }
 let mut pc: u32 = 0x10143d7;
 loop { match pc {
 0x10143d7 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x110de94u32, v); }
  { let v = 0x200u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10143ee); f_0108defe(cpu); if cpu.returned_to != 0x10143ee { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1014403; continue; }
  pc = 0x10143f8; continue;
 }
 0x10143f8 => {
  { let v: u32 = 0x3u32; cpu.w32(0x110de94u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1014403 => {
  cpu.push(0x1014408); f_0108e959(cpu); if cpu.returned_to != 0x1014408 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1014419; continue; }
  pc = 0x101440f; continue;
 }
 0x101440f => {
  { let v: u32 = 0x5u32; cpu.w32(0x110de94u32, v); }
  pc = 0x1014419; continue;
 }
 0x1014419 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x1014420); f_0108e26b(cpu); if cpu.returned_to != 0x1014420 { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01014423(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1014423); }
 let mut pc: u32 = 0x1014423;
 loop { match pc {
 0x1014423 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x110de94u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1014437); f_010143d7(cpu); if cpu.returned_to != 0x1014437 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1014450; continue; }
  pc = 0x1014441; continue;
 }
 0x1014441 => {
  { let v: u32 = 0x3u32; cpu.w32(0x110de94u32, v); }
  pc = 0x101444b; continue;
 }
 0x101444b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1014450 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1014460; continue; }
  pc = 0x1014458; continue;
 }
 0x1014458 => {
  let t = cpu.r32(0x110de98u32); cpu.push(0x101445e); cpu.call(t); if cpu.returned_to != 0x101445e { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x1014460; continue;
 }
 0x1014460 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1014470; continue; }
  pc = 0x1014464; continue;
 }
 0x1014464 => {
  { let v: u32 = 0x2u32; cpu.w32(0x110de94u32, v); }
  cpu.poll(); pc = 0x101444b; continue;
 }
 0x1014470 => {
  { let v = 0x200u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x101447f); f_0108defe(cpu); if cpu.returned_to != 0x101447f { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x1014493; continue; }
  pc = 0x1014489; continue;
 }
 0x1014489 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  let t = cpu.r32(0x110de9cu32); cpu.push(0x1014491); cpu.call(t); if cpu.returned_to != 0x1014491 { return; }
  cpu.poll(); pc = 0x1014441; continue;
 }
 0x1014493 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  cpu.push(0x101449c); f_0108e170(cpu); if cpu.returned_to != 0x101449c { return; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x10144b4; continue; }
  pc = 0x10144a0; continue;
 }
 0x10144a0 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  let t = cpu.r32(0x110de9cu32); cpu.push(0x10144a8); cpu.call(t); if cpu.returned_to != 0x10144a8 { return; }
  { let v: u32 = 0x5u32; cpu.w32(0x110de94u32, v); }
  cpu.poll(); pc = 0x101444b; continue;
 }
 0x10144b4 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10144bb); f_0108e26b(cpu); if cpu.returned_to != 0x10144bb { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010144c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10144c0); }
 let mut pc: u32 = 0x10144c0;
 loop { match pc {
 0x10144c0 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x110de94u32, v); }
  { let v = 0x180u32; cpu.push(v); }
  { let v = 0x262u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10144dc); f_0108defe(cpu); if cpu.returned_to != 0x10144dc { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10144f3; continue; }
  pc = 0x10144e6; continue;
 }
 0x10144e6 => {
  { let v: u32 = 0x4u32; cpu.w32(0x110de94u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10144f3 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  cpu.push(0x1014500); f_0108e996(cpu); if cpu.returned_to != 0x1014500 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { cpu.poll(); pc = 0x10144e6; continue; }
  pc = 0x1014505; continue;
 }
 0x1014505 => {
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.esp))); }
  if cpu.zf { pc = 0x1014518; continue; }
  pc = 0x101450b; continue;
 }
 0x101450b => {
  { let v: u32 = 0x6u32; cpu.w32(0x110de94u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1014518 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x101451f); f_0108e26b(cpu); if cpu.returned_to != 0x101451f { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01014842(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1014842); }
 let mut pc: u32 = 0x1014842;
 loop { match pc {
 0x1014842 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.sar8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0u32); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x8000003fu32.wrapping_add(cpu.eax)), ((cpu.eax >> 8) as u8)); cpu.w8(0x8000003fu32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x101484d, "Aas");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.unimplemented(0x1014851, "Aas");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), ((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  f_01014856(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01014856(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1014856); }
 let mut pc: u32 = 0x1014856;
 loop { match pc {
 0x1014856 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = 0x1014842u32; cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x1014881; continue; }
  pc = 0x101487a; continue;
 }
 0x101487a => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014881; continue;
 }
 0x1014881 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x4u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x101488e; continue; }
  pc = 0x1014887; continue;
 }
 0x1014887 => {
  { let v: u32 = 0x4u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x101488e; continue;
 }
 0x101488e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10148a2; continue;
 }
 0x10148a2 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x10148b2; continue; }
  pc = 0x10148a8; continue;
 }
 0x10148a8 => {
  pc = 0x1014918; continue;
 }
 0x10148aa => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10148a2; continue;
 }
 0x10148b2 => {
  { let v = cpu.rf32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = (cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf64(0x11369a0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10148cf); f_01094e48(cpu); if cpu.returned_to != 0x10148cf { return; }
  { let v = cpu.st(0); cpu.wf32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; let h = cpu.eflags_high; let i = cpu.interrupts_off; cpu.set_flags(v); cpu.df = d; cpu.of = o; cpu.eflags_high = h; cpu.interrupts_off = i; }
  if (cpu.cf || cpu.zf) { pc = 0x10148e3; continue; }
  pc = 0x10148dc; continue;
 }
 0x10148dc => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10148e3; continue;
 }
 0x10148e3 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x3f800000u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10148f3; continue; }
  pc = 0x10148ec; continue;
 }
 0x10148ec => {
  { let v: u32 = 0x3f800000u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10148f3; continue;
 }
 0x10148f3 => {
  { let v = cpu.rf32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x11369a8u32); cpu.set_st(0, v); }
  cpu.push(0x1014901); f_0108de9e(cpu); if cpu.returned_to != 0x1014901 { return; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u8 = cpu.r8(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11d509cu32.wrapping_add(cpu.edx), v); }
  cpu.poll(); pc = 0x10148aa; continue;
 }
 0x1014918 => {
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
pub fn f_01014921(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1014921); }
 let mut pc: u32 = 0x1014921;
 loop { match pc {
 0x1014921 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x101493d; continue;
 }
 0x101493d => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1014952; continue; }
  pc = 0x1014948; continue;
 }
 0x1014948 => {
  pc = 0x101496f; continue;
 }
 0x101494a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x101493d; continue;
 }
 0x1014952 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x11d509cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.poll(); pc = 0x101494a; continue;
 }
 0x101496f => {
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
pub fn f_01014a00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1014a00); }
 let mut pc: u32 = 0x1014a00;
 loop { match pc {
 0x1014a00 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x4u8); }
  if !cpu.zf { pc = 0x1014a27; continue; }
  pc = 0x1014a16; continue;
 }
 0x1014a16 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1014a3a; continue; }
  pc = 0x1014a27; continue;
 }
 0x1014a27 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x1014a35); f_01094e58(cpu); if cpu.returned_to != 0x1014a35 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014a80; continue;
 }
 0x1014a3a => {
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if !cpu.zf { pc = 0x1014a5c; continue; }
  pc = 0x1014a4a; continue;
 }
 0x1014a4a => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x1014a57); f_01094e58(cpu); if cpu.returned_to != 0x1014a57 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014a7a; continue;
 }
 0x1014a5c => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)) | 0x10u8; cpu.w8(0xdu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.eax))); cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014a7a; continue;
 }
 0x1014a7a => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014a80; continue;
 }
 0x1014a80 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01014a88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1014a88); }
 let mut pc: u32 = 0x1014a88;
 loop { match pc {
 0x1014a88 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x4u8); }
  if !cpu.zf { pc = 0x1014ab5; continue; }
  pc = 0x1014aa4; continue;
 }
 0x1014aa4 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1014acd; continue; }
  pc = 0x1014ab5; continue;
 }
 0x1014ab5 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  cpu.push(0x1014ac8); f_01094e58(cpu); if cpu.returned_to != 0x1014ac8 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014b1d; continue;
 }
 0x1014acd => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if !cpu.zf { pc = 0x1014af9; continue; }
  pc = 0x1014ae7; continue;
 }
 0x1014ae7 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x1014af4); f_01094e58(cpu); if cpu.returned_to != 0x1014af4 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014b17; continue;
 }
 0x1014af9 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)) | 0x10u8; cpu.w8(0xdu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.eax))); cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014b17; continue;
 }
 0x1014b17 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014b1d; continue;
 }
 0x1014b1d => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x4u8); }
  if !cpu.zf { pc = 0x1014b37; continue; }
  pc = 0x1014b26; continue;
 }
 0x1014b26 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1014b4f; continue; }
  pc = 0x1014b37; continue;
 }
 0x1014b37 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  cpu.push(0x1014b4a); f_01094e58(cpu); if cpu.returned_to != 0x1014b4a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014b9f; continue;
 }
 0x1014b4f => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if !cpu.zf { pc = 0x1014b7b; continue; }
  pc = 0x1014b69; continue;
 }
 0x1014b69 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x1014b76); f_01094e58(cpu); if cpu.returned_to != 0x1014b76 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014b99; continue;
 }
 0x1014b7b => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)) | 0x10u8; cpu.w8(0xdu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.eax))); cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014b99; continue;
 }
 0x1014b99 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014b9f; continue;
 }
 0x1014b9f => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x4u8); }
  if !cpu.zf { pc = 0x1014bb9; continue; }
  pc = 0x1014ba8; continue;
 }
 0x1014ba8 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1014bd1; continue; }
  pc = 0x1014bb9; continue;
 }
 0x1014bb9 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  cpu.push(0x1014bcc); f_01094e58(cpu); if cpu.returned_to != 0x1014bcc { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014c21; continue;
 }
 0x1014bd1 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if !cpu.zf { pc = 0x1014bfd; continue; }
  pc = 0x1014beb; continue;
 }
 0x1014beb => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x1014bf8); f_01094e58(cpu); if cpu.returned_to != 0x1014bf8 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014c1b; continue;
 }
 0x1014bfd => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)) | 0x10u8; cpu.w8(0xdu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.eax))); cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014c1b; continue;
 }
 0x1014c1b => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014c21; continue;
 }
 0x1014c21 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x4u8); }
  if !cpu.zf { pc = 0x1014c3b; continue; }
  pc = 0x1014c2a; continue;
 }
 0x1014c2a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1014c50; continue; }
  pc = 0x1014c3b; continue;
 }
 0x1014c3b => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.push(0x1014c4b); f_01094e58(cpu); if cpu.returned_to != 0x1014c4b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014c96; continue;
 }
 0x1014c50 => {
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if !cpu.zf { pc = 0x1014c72; continue; }
  pc = 0x1014c60; continue;
 }
 0x1014c60 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x1014c6d); f_01094e58(cpu); if cpu.returned_to != 0x1014c6d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014c90; continue;
 }
 0x1014c72 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)) | 0x10u8; cpu.w8(0xdu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.eax))); cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014c90; continue;
 }
 0x1014c90 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1014c96; continue;
 }
 0x1014c96 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01015124(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015124); }
 let mut pc: u32 = 0x1015124;
 loop { match pc {
 0x1015124 => {
  { let v: u8 = cpu.and8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0xfeu8); cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1015130); f_01047466(cpu); if cpu.returned_to != 0x1015130 { return; }
  { let v: u8 = 0x3au8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015135(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015135); }
 let mut pc: u32 = 0x1015135;
 loop { match pc {
 0x1015135 => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x101513e); f_01047466(cpu); if cpu.returned_to != 0x101513e { return; }
  { let v: u8 = 0x37u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015143(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015143); }
 let mut pc: u32 = 0x1015143;
 loop { match pc {
 0x1015143 => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x101514c); f_01047466(cpu); if cpu.returned_to != 0x101514c { return; }
  { let v: u8 = 0x35u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015151(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015151); }
 let mut pc: u32 = 0x1015151;
 loop { match pc {
 0x1015151 => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x101515a); f_01047466(cpu); if cpu.returned_to != 0x101515a { return; }
  { let v: u8 = 0x36u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0101515f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101515f); }
 let mut pc: u32 = 0x101515f;
 loop { match pc {
 0x101515f => {
  { let v: u32 = cpu.ebp; cpu.w32(0x110dea0u32, v); }
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x83u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10151e9; continue; }
  pc = 0x101516f; continue;
 }
 0x101516f => {
  { cpu.sub8(cpu.r8(0x83u32.wrapping_add(cpu.ebp)), 0xfeu8); }
  if cpu.zf { pc = 0x10151e9; continue; }
  pc = 0x1015178; continue;
 }
 0x1015178 => {
  { let v: u16 = cpu.r16(0x8eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x90u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r8(0x83u32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x110dea6u32, v); }
  { let v: u16 = (cpu.ecx as u16) & 0x3fu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x92u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x10151a8; continue; }
  pc = 0x10151a7; continue;
 }
 0x10151a7 => {
  { cpu.cf = true; }
  pc = 0x10151a8; continue;
 }
 0x10151a8 => {
  cpu.push(0x10151ad); f_010156c0(cpu); if cpu.returned_to != 0x10151ad { return; }
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { cpu.and8(cpu.r8(0x83u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x10151be; continue; }
  pc = 0x10151b8; continue;
 }
 0x10151b8 => {
  { let v = cpu.r16(cpu.esp) as u32; cpu.esp = cpu.esp.wrapping_add(2); let h = cpu.eflags_high; cpu.set_flags(v); cpu.eflags_high = h; }
  if cpu.cf { pc = 0x10151e9; continue; }
  pc = 0x10151bc; continue;
 }
 0x10151bc => {
  pc = 0x10151e3; continue;
 }
 0x10151be => {
  { let v = cpu.r16(cpu.esp) as u32; cpu.esp = cpu.esp.wrapping_add(2); let h = cpu.eflags_high; cpu.set_flags(v); cpu.eflags_high = h; }
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { pc = 0x10151e9; continue; }
  pc = 0x10151c9; continue;
 }
 0x10151c9 => {
  { let v: u8 = cpu.r8(0x83u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x10151d9; continue; }
  pc = 0x10151d7; continue;
 }
 0x10151d7 => {
  { let v: u8 = 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10151d9; continue;
 }
 0x10151d9 => {
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10151e9; continue;
 }
 0x10151e3 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x83u32.wrapping_add(cpu.ebp))); cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10151e9; continue;
 }
 0x10151e9 => {
  { cpu.popad(); }
  { let v: u16 = cpu.r8(0x92u32.wrapping_add(cpu.ebp)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 4u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8) & 0xfu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.push(0x10151ff); f_0105f178(cpu); if cpu.returned_to != 0x10151ff { return; }
  { let v: u16 = (cpu.eax as u16) & 0x3fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x4u16); }
  if cpu.cf { pc = 0x1015211; continue; }
  pc = 0x1015209; continue;
 }
 0x1015209 => {
  { let v: u8 = cpu.inc8(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0x1u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.ebx >> 8) as u8), 4u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1015211; continue;
 }
 0x1015211 => {
  { cpu.sub8(cpu.r8(0x10ee92eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1015251; continue; }
  pc = 0x101521a; continue;
 }
 0x101521a => {
  { cpu.sub8((cpu.ebx as u8), 0x4u8); }
  if (cpu.sf != cpu.of) { pc = 0x1015223; continue; }
  pc = 0x101521f; continue;
 }
 0x101521f => {
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1015251; continue;
 }
 0x1015223 => {
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x7u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x1015251; continue; }
  pc = 0x101522d; continue;
 }
 0x101522d => {
  if cpu.zf { pc = 0x101523e; continue; }
  pc = 0x101522f; continue;
 }
 0x101522f => {
  cpu.push(0x1015234); f_0105f178(cpu); if cpu.returned_to != 0x1015234 { return; }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x1015251; continue; }
  pc = 0x101523a; continue;
 }
 0x101523a => {
  { let v: u8 = 0x5u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1015251; continue;
 }
 0x101523e => {
  cpu.push(0x1015243); f_0105f178(cpu); if cpu.returned_to != 0x1015243 { return; }
  { let v: u16 = (cpu.eax as u16) & 0x7u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.zf { pc = 0x1015251; continue; }
  pc = 0x101524d; continue;
 }
 0x101524d => {
  { let v: u8 = 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1015251; continue;
 }
 0x1015251 => {
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x92u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1015262); f_01047466(cpu); if cpu.returned_to != 0x1015262 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x15u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x3bu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0xfeu8); }
  if !cpu.zf { pc = 0x101528e; continue; }
  pc = 0x101527c; continue;
 }
 0x101527c => {
  cpu.push(0x1015281); f_0106a395(cpu); if cpu.returned_to != 0x1015281 { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x101549b; continue; }
  pc = 0x1015289; continue;
 }
 0x1015289 => {
  pc = 0x1015681; continue;
 }
 0x101528e => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10152e0; continue; }
  pc = 0x1015297; continue;
 }
 0x1015297 => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x10152e0; continue; }
  pc = 0x10152a0; continue;
 }
 0x10152a0 => {
  { cpu.and8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x10152c1; continue; }
  pc = 0x10152a9; continue;
 }
 0x10152a9 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x96u32.wrapping_add(cpu.ebp))); cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if !cpu.zf { pc = 0x10152e0; continue; }
  pc = 0x10152b8; continue;
 }
 0x10152b8 => {
  { let v: u8 = cpu.r8(0x96u32.wrapping_add(cpu.ebp)) | 0x40u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10152e0; continue;
 }
 0x10152c1 => {
  { let v: u8 = cpu.r8(0x96u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10152d9; continue; }
  pc = 0x10152cf; continue;
 }
 0x10152cf => {
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10152e0; continue;
 }
 0x10152d9 => {
  { let v: u8 = 0x1u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10152e0; continue;
 }
 0x10152e0 => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1015681; continue; }
  pc = 0x10152ed; continue;
 }
 0x10152ed => {
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1015681; continue; }
  pc = 0x10152fa; continue;
 }
 0x10152fa => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if cpu.zf { pc = 0x1015681; continue; }
  pc = 0x1015306; continue;
 }
 0x1015306 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u16 = cpu.r8(0x6u32.wrapping_add(cpu.ebx)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x1015404; continue; }
  pc = 0x101531f; continue;
 }
 0x101531f => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if cpu.cf { pc = 0x1015681; continue; }
  pc = 0x1015331; continue;
 }
 0x1015331 => {
  { cpu.sub8(cpu.r8(0xabu32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x101538e; continue; }
  pc = 0x101533a; continue;
 }
 0x101533a => {
  { cpu.sub8(cpu.r8(0xabu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x10153ab; continue; }
  pc = 0x1015343; continue;
 }
 0x1015343 => {
  { cpu.sub8(cpu.r8(0xabu32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x1015369; continue; }
  pc = 0x101534c; continue;
 }
 0x101534c => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x30u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e4154u32, v); }
  pc = 0x10153d0; continue;
 }
 0x1015369 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x30u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x8u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e4154u32, v); }
  pc = 0x10153d0; continue;
 }
 0x101538e => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x30u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e4154u32, v); }
  pc = 0x10153d0; continue;
 }
 0x10153ab => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x10e4154u32, v); }
  pc = 0x10153d0; continue;
 }
 0x10153d0 => {
  cpu.push(0x10153d5); f_010507b9(cpu); if cpu.returned_to != 0x10153d5 { return; }
  if cpu.cf { pc = 0x1015663; continue; }
  pc = 0x10153db; continue;
 }
 0x10153db => {
  { let v: u8 = cpu.or8(cpu.r8(0x98u32.wrapping_add(cpu.ebp)), 0x2u8); cpu.w8(0x98u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10153e7); f_0106a2d3(cpu); if cpu.returned_to != 0x10153e7 { return; }
  if cpu.cf { pc = 0x1015663; continue; }
  pc = 0x10153ed; continue;
 }
 0x10153ed => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xfeu8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1015681; continue;
 }
 0x1015404 => {
  { let v: u16 = cpu.r16(0x39u32.wrapping_add(cpu.edi)).wrapping_add((cpu.eax as u16)); cpu.w16(0x39u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.dec8(cpu.r8(0x7fu32.wrapping_add(cpu.ebp))); cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { pc = 0x101542b; continue; }
  pc = 0x101540d; continue;
 }
 0x101540d => {
  { let v: u8 = cpu.r8(0x84u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7fu32.wrapping_add(cpu.ebp), v); }
  { cpu.pushad(); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1015423); f_01015682(cpu); if cpu.returned_to != 0x1015423 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.push(0x101542a); f_01015682(cpu); if cpu.returned_to != 0x101542a { return; }
  { cpu.popad(); }
  pc = 0x101542b; continue;
 }
 0x101542b => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x39u32.wrapping_add(cpu.edi)), (cpu.eax as u16)); }
  if cpu.cf { pc = 0x1015681; continue; }
  pc = 0x1015439; continue;
 }
 0x1015439 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x10c6120u32); cpu.eax = v; }
  { let v: u16 = cpu.r8(0x31u32.wrapping_add(cpu.eax)) as u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r8(0x30u32.wrapping_add(cpu.eax)) as u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 3u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x8u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1015473); f_010507b9(cpu); if cpu.returned_to != 0x1015473 { return; }
  if cpu.cf { pc = 0x1015663; continue; }
  pc = 0x1015479; continue;
 }
 0x1015479 => {
  cpu.push(0x101547e); f_0106a2d3(cpu); if cpu.returned_to != 0x101547e { return; }
  if cpu.cf { pc = 0x1015663; continue; }
  pc = 0x1015484; continue;
 }
 0x1015484 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x17u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x17u32.wrapping_add(cpu.edi)), 0xfdu8); cpu.w8(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xfeu8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1015681; continue;
 }
 0x101549b => {
  { let v: u8 = 0xffu8; cpu.w8(0x83u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x92u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x40u8); }
  if !cpu.zf { pc = 0x10154fa; continue; }
  pc = 0x10154b5; continue;
 }
 0x10154b5 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10154bb); f_01015954(cpu); if cpu.returned_to != 0x10154bb { return; }
  if cpu.cf { pc = 0x1015663; continue; }
  pc = 0x10154c1; continue;
 }
 0x10154c1 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x2eu32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xau32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xcu32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x110dea0u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x1015663; continue;
 }
 0x10154fa => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.edi)), 0x0u16); }
  if !cpu.zf { pc = 0x10155b5; continue; }
  pc = 0x1015507; continue;
 }
 0x1015507 => {
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
  cpu.push(0x101552c); f_0101ae01(cpu); if cpu.returned_to != 0x101552c { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1015537); f_0105f215(cpu); if cpu.returned_to != 0x1015537 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x101553e; continue;
 }
 0x101553e => {
  cpu.push(0x1015543); f_01015954(cpu); if cpu.returned_to != 0x1015543 { return; }
  if cpu.cf { pc = 0x101565f; continue; }
  pc = 0x1015549; continue;
 }
 0x1015549 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3e93u32)); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.r16(0x10e3e93u32) & 0x3u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10fcaf0u32; cpu.esi = v; }
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
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x110dea0u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110dea8u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), 0x10u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x101553e; continue; }
  pc = 0x10155b0; continue;
 }
 0x10155b0 => {
  pc = 0x101565f; continue;
 }
 0x10155b5 => {
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
  cpu.push(0x10155df); f_0101ae01(cpu); if cpu.returned_to != 0x10155df { return; }
  { cpu.popad(); }
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10155ea); f_0105f215(cpu); if cpu.returned_to != 0x10155ea { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10155f1; continue;
 }
 0x10155f1 => {
  cpu.push(0x10155f6); f_01015954(cpu); if cpu.returned_to != 0x10155f6 { return; }
  if cpu.cf { pc = 0x101565f; continue; }
  pc = 0x10155f8; continue;
 }
 0x10155f8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3e93u32)); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.r16(0x10e3e93u32) & 0x3u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10fcae0u32; cpu.esi = v; }
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
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x110dea0u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x10u32.wrapping_add(cpu.ebp))); cpu.w8(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110deb0u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x10u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10155f1; continue; }
  pc = 0x101565f; continue;
 }
 0x101565f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x1015663; continue;
 }
 0x1015663 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x96u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.ebx = v; }
  { let v: u8 = cpu.and8(cpu.r8(0xb1u32.wrapping_add(cpu.ebp)), 0xfeu8); cpu.w8(0xb1u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x1015681); f_01059da3(cpu); if cpu.returned_to != 0x1015681 { return; }
  pc = 0x1015681; continue;
 }
 0x1015681 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015682(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015682); }
 let mut pc: u32 = 0x1015682;
 loop { match pc {
 0x1015682 => {
  { cpu.pushad(); }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x1015688; continue;
 }
 0x1015688 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.edi)), 0xffffu16); }
  if cpu.zf { pc = 0x10156be; continue; }
  pc = 0x101568f; continue;
 }
 0x101568f => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.and8(cpu.r8(0xeu32.wrapping_add(cpu.edi)), 0x4u8); }
  if !cpu.zf { pc = 0x10156b6; continue; }
  pc = 0x1015696; continue;
 }
 0x1015696 => {
  { cpu.and8(cpu.r8(0xeu32.wrapping_add(cpu.edi)), 0x2u8); }
  if cpu.zf { pc = 0x10156b6; continue; }
  pc = 0x101569c; continue;
 }
 0x101569c => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xfffeu16); }
  if !cpu.zf { pc = 0x10156b6; continue; }
  pc = 0x10156a6; continue;
 }
 0x10156a6 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if !cpu.sf { pc = 0x10156af; continue; }
  pc = 0x10156ac; continue;
 }
 0x10156ac => {
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10156af; continue;
 }
 0x10156af => {
  { let v: u8 = cpu.or8(cpu.r8(0xeu32.wrapping_add(cpu.edi)), 0x4u8); cpu.w8(0xeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x10156be; continue;
 }
 0x10156b6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x12u32); cpu.edi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x1015688; continue; }
  pc = 0x10156be; continue;
 }
 0x10156be => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010156c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10156c0); }
 let mut pc: u32 = 0x10156c0;
 loop { match pc {
 0x10156c0 => {
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110dea4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.eax as u8), 4u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110dea5u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e99u32, v); }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = 0x110ded8u32; cpu.esi = v; }
  { let v = cpu.r16(cpu.esp) as u32; cpu.esp = cpu.esp.wrapping_add(2); let h = cpu.eflags_high; cpu.set_flags(v); cpu.eflags_high = h; }
  if !cpu.cf { pc = 0x1015713; continue; }
  pc = 0x1015705; continue;
 }
 0x1015705 => {
  { let v: u32 = cpu.r8(0xabu32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x110def4u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.esi = v; }
  pc = 0x1015713; continue;
 }
 0x1015713 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x1015801; continue; }
  pc = 0x101571e; continue;
 }
 0x101571e => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e99u32, v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x1722u16); }
  if !cpu.zf { pc = 0x1015751; continue; }
  pc = 0x1015743; continue;
 }
 0x1015743 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x110dea5u32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1015764; continue;
 }
 0x1015751 => {
  { cpu.sub16((cpu.ecx as u16), 0x1724u16); }
  if !cpu.zf { pc = 0x1015764; continue; }
  pc = 0x1015758; continue;
 }
 0x1015758 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r8(0x110dea4u32) as u32; cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1015764; continue;
 }
 0x1015764 => {
  { cpu.and8(cpu.r8(0x110dea6u32), 0x40u8); }
  if cpu.zf { pc = 0x1015785; continue; }
  pc = 0x101576d; continue;
 }
 0x101576d => {
  { cpu.sub16((cpu.ecx as u16), 0x172au16); }
  if !cpu.zf { pc = 0x101577a; continue; }
  pc = 0x1015774; continue;
 }
 0x1015774 => {
  { let v: u16 = 0x172cu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1015785; continue;
 }
 0x101577a => {
  { cpu.sub16((cpu.ecx as u16), 0x172cu16); }
  if !cpu.zf { pc = 0x1015785; continue; }
  pc = 0x1015781; continue;
 }
 0x1015781 => {
  { let v: u16 = 0x172au16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x1015785; continue;
 }
 0x1015785 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3e93u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3e95u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.sf { pc = 0x10157f8; continue; }
  pc = 0x10157a7; continue;
 }
 0x10157a7 => {
  { cpu.sub16((cpu.ebx as u16), 0x0u16); }
  if cpu.sf { pc = 0x10157f8; continue; }
  pc = 0x10157ad; continue;
 }
 0x10157ad => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x10157f8; continue; }
  pc = 0x10157b6; continue;
 }
 0x10157b6 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x10157f8; continue; }
  pc = 0x10157bf; continue;
 }
 0x10157bf => {
  { cpu.pushad(); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x62u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.and16((cpu.edx as u16), 0x3u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x10157cf); f_0106b882(cpu); if cpu.returned_to != 0x10157cf { return; }
  { cpu.popad(); }
  cpu.push(0x10157d5); f_01072d00(cpu); if cpu.returned_to != 0x10157d5 { return; }
  if cpu.cf { pc = 0x10157f8; continue; }
  pc = 0x10157d7; continue;
 }
 0x10157d7 => {
  { let v: u8 = 0xffu8; cpu.w8(0x1124471u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xcu32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x112446bu32), 0x0u8); }
  if cpu.zf { pc = 0x10157f7; continue; }
  pc = 0x10157ed; continue;
 }
 0x10157ed => {
  cpu.push(0x10157f2); f_01012780(cpu); if cpu.returned_to != 0x10157f2 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1124471u32, v); }
  pc = 0x10157f7; continue;
 }
 0x10157f7 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10157f8; continue;
 }
 0x10157f8 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x1015713; continue;
 }
 0x1015801 => {
  { cpu.sub16(cpu.r16(0x10e3e99u32), 0xffffu16); }
  if !cpu.zf { pc = 0x1015835; continue; }
  pc = 0x101580b; continue;
 }
 0x101580b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x101581a); f_01072d00(cpu); if cpu.returned_to != 0x101581a { return; }
  if cpu.cf { pc = 0x1015831; continue; }
  pc = 0x101581c; continue;
 }
 0x101581c => {
  { let v: u8 = 0x6u8; cpu.w8(0x10ef35du32, v); }
  { let v: u16 = cpu.r16(0x112c96eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x110d732u32, v); }
  pc = 0x1015831; continue;
 }
 0x1015831 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1015835 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015837(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015837); }
 let mut pc: u32 = 0x1015837;
 loop { match pc {
 0x1015837 => {
  { cpu.pushad(); }
  { cpu.pushad(); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = ((cpu.eax >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x10c6120u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x11u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x12u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x13u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x15u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2u8; cpu.w8(0x33u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x39u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x34u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x4bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x6u32.wrapping_add(cpu.ebp)), 0xf2u8); cpu.w8(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x16u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x101589c); f_0107281d(cpu); if cpu.returned_to != 0x101589c { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  let t = cpu.r32(0x36u32.wrapping_add(cpu.esi)); cpu.push(0x10158a0); cpu.call(t); if cpu.returned_to != 0x10158a0 { return; }
  { cpu.popad(); }
  cpu.push(0x10158a6); f_0102f745(cpu); if cpu.returned_to != 0x10158a6 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010158a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10158a8); }
 let mut pc: u32 = 0x10158a8;
 loop { match pc {
 0x10158a8 => {
  { let v: u8 = 0x9u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10158b1); f_01047466(cpu); if cpu.returned_to != 0x10158b1 { return; }
  { let v: u8 = 0x3cu8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(cpu.r8(0x87u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x101592e; continue; }
  pc = 0x10158be; continue;
 }
 0x10158be => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.inc8(cpu.r8(0x15u32.wrapping_add(cpu.edi))); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)) & 0x7u8; cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10158da); f_0104872b(cpu); if cpu.returned_to != 0x10158da { return; }
  cpu.push(0x10158df); f_010487ef(cpu); if cpu.returned_to != 0x10158df { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x110debau32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110deb8u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add((cpu.ebx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.pushad(); }
  cpu.push(0x1015907); f_0104968b(cpu); if cpu.returned_to != 0x1015907 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  cpu.push(0x101590e); f_0104968b(cpu); if cpu.returned_to != 0x101590e { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x8eu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.sf { pc = 0x1015928; continue; }
  pc = 0x101591f; continue;
 }
 0x101591f => {
  { let v: u8 = cpu.r8(0x87u32.wrapping_add(cpu.ebp)) & 0xfdu8; cpu.w8(0x87u32.wrapping_add(cpu.ebp), v); }
  pc = 0x101592e; continue;
 }
 0x1015928 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x8eu32.wrapping_add(cpu.ebp))); cpu.w8(0x8eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x101592e; continue;
 }
 0x101592e => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x2084u32); }
  if !cpu.zf { pc = 0x101593b; continue; }
  pc = 0x1015939; continue;
 }
 0x1015939 => {
  pc = 0x1015953; continue;
 }
 0x101593b => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x1cu32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x1015953; continue; }
  pc = 0x1015945; continue;
 }
 0x1015945 => {
  { let v: u32 = cpu.r32(0x10e3f84u32); cpu.eax = v; }
  cpu.push(0x101594f); f_01047379(cpu); if cpu.returned_to != 0x101594f { return; }
  { let v: u8 = cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0xefu8); cpu.w8(0x31u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1015953; continue;
 }
 0x1015953 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015954(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015954); }
 let mut pc: u32 = 0x1015954;
 loop { match pc {
 0x1015954 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x11b4104u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x101595f; continue;
 }
 0x101595f => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x101596b; continue; }
  pc = 0x1015964; continue;
 }
 0x1015964 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x18u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x101595f; continue; }
  pc = 0x1015969; continue;
 }
 0x1015969 => {
  pc = 0x101596e; continue;
 }
 0x101596b => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x101596e => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015974(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015974); }
 let mut pc: u32 = 0x1015974;
 loop { match pc {
 0x1015974 => {
  { cpu.sub32(cpu.r32(0x110eab4u32), 0x0u32); }
  if cpu.zf { pc = 0x1015add; continue; }
  pc = 0x1015981; continue;
 }
 0x1015981 => {
  { cpu.sub8(cpu.r8(0x110ee01u32), 0x0u8); }
  if cpu.zf { pc = 0x1015995; continue; }
  pc = 0x101598a; continue;
 }
 0x101598a => {
  { let v: u8 = cpu.dec8(cpu.r8(0x110ee01u32)); cpu.w8(0x110ee01u32, v); }
  pc = 0x1015add; continue;
 }
 0x1015995 => {
  { cpu.sub32(cpu.r32(0x11d50e4u32), 0x0u32); }
  if cpu.zf { pc = 0x10159ba; continue; }
  pc = 0x101599e; continue;
 }
 0x101599e => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11d50e4u32)); cpu.w32(0x11d50e4u32, v); }
  if !cpu.zf { pc = 0x10159ba; continue; }
  pc = 0x10159a6; continue;
 }
 0x10159a6 => {
  { let v: u32 = 0x0u32; cpu.w32(0x110eab4u32, v); }
  cpu.push(0x10159b5); f_010863ff(cpu); if cpu.returned_to != 0x10159b5 { return; }
  pc = 0x1015add; continue;
 }
 0x10159ba => {
  cpu.push(0x10159bf); f_01015b47(cpu); if cpu.returned_to != 0x10159bf { return; }
  if cpu.cf { pc = 0x1015add; continue; }
  pc = 0x10159c5; continue;
 }
 0x10159c5 => {
  { let v: u32 = cpu.r32(0x110eab4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if cpu.zf { pc = 0x10159ed; continue; }
  pc = 0x10159d9; continue;
 }
 0x10159d9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x110eab4u32, v); }
  cpu.push(0x10159e8); f_010863ff(cpu); if cpu.returned_to != 0x10159e8 { return; }
  pc = 0x1015add; continue;
 }
 0x10159ed => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110ed94u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110ed95u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1015a15; continue; }
  pc = 0x1015a05; continue;
 }
 0x1015a05 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1015a15; continue; }
  pc = 0x1015a0b; continue;
 }
 0x1015a0b => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x1015add; continue; }
  pc = 0x1015a15; continue;
 }
 0x1015a15 => {
  cpu.push(0x1015a1a); f_01015b06(cpu); if cpu.returned_to != 0x1015a1a { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1015a22); f_010338d0(cpu); if cpu.returned_to != 0x1015a22 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1015a2a); f_01016c53(cpu); if cpu.returned_to != 0x1015a2a { return; }
  cpu.push(0x1015a2f); f_01016b71(cpu); if cpu.returned_to != 0x1015a2f { return; }
  cpu.push(0x1015a34); f_01016c91(cpu); if cpu.returned_to != 0x1015a34 { return; }
  cpu.push(0x1015a39); f_01016dbd(cpu); if cpu.returned_to != 0x1015a39 { return; }
  cpu.push(0x1015a3e); f_01016bae(cpu); if cpu.returned_to != 0x1015a3e { return; }
  cpu.push(0x1015a43); f_0101644b(cpu); if cpu.returned_to != 0x1015a43 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1015a4b); f_010338d0(cpu); if cpu.returned_to != 0x1015a4b { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1015a53); f_01016ad0(cpu); if cpu.returned_to != 0x1015a53 { return; }
  cpu.push(0x1015a58); f_01016a1d(cpu); if cpu.returned_to != 0x1015a58 { return; }
  cpu.push(0x1015a5d); f_010163bf(cpu); if cpu.returned_to != 0x1015a5d { return; }
  cpu.push(0x1015a62); f_01015b88(cpu); if cpu.returned_to != 0x1015a62 { return; }
  cpu.push(0x1015a67); f_0104f3c9(cpu); if cpu.returned_to != 0x1015a67 { return; }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1015a95; continue; }
  pc = 0x1015a6d; continue;
 }
 0x1015a6d => {
  { let v: u8 = cpu.r8(0x110ed99u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1015a77); f_010176bf(cpu); if cpu.returned_to != 0x1015a77 { return; }
  if cpu.cf { pc = 0x1015a90; continue; }
  pc = 0x1015a79; continue;
 }
 0x1015a79 => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x1015a89; continue; }
  pc = 0x1015a7d; continue;
 }
 0x1015a7d => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x1015a89; continue; }
  pc = 0x1015a81; continue;
 }
 0x1015a81 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x1015a89; continue; }
  pc = 0x1015a85; continue;
 }
 0x1015a85 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if !cpu.zf { pc = 0x1015a95; continue; }
  pc = 0x1015a89; continue;
 }
 0x1015a89 => {
  cpu.push(0x1015a8e); f_01017748(cpu); if cpu.returned_to != 0x1015a8e { return; }
  if !cpu.cf { pc = 0x1015a95; continue; }
  pc = 0x1015a90; continue;
 }
 0x1015a90 => {
  cpu.push(0x1015a95); f_01015ade(cpu); if cpu.returned_to != 0x1015a95 { return; }
  pc = 0x1015a95; continue;
 }
 0x1015a95 => {
  cpu.push(0x1015a9a); f_010176bf(cpu); if cpu.returned_to != 0x1015a9a { return; }
  if !cpu.cf { pc = 0x1015aa1; continue; }
  pc = 0x1015a9c; continue;
 }
 0x1015a9c => {
  cpu.push(0x1015aa1); f_01016d64(cpu); if cpu.returned_to != 0x1015aa1 { return; }
  pc = 0x1015aa1; continue;
 }
 0x1015aa1 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x110ed7du32)); cpu.w8(0x110ed7du32, v); }
  if !cpu.sf { pc = 0x1015ab0; continue; }
  pc = 0x1015aa9; continue;
 }
 0x1015aa9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110ed7du32, v); }
  pc = 0x1015ab0; continue;
 }
 0x1015ab0 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x110ed7eu32)); cpu.w8(0x110ed7eu32, v); }
  if !cpu.sf { pc = 0x1015abf; continue; }
  pc = 0x1015ab8; continue;
 }
 0x1015ab8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110ed7eu32, v); }
  pc = 0x1015abf; continue;
 }
 0x1015abf => {
  { let v: u8 = cpu.dec8(cpu.r8(0x110ed83u32)); cpu.w8(0x110ed83u32, v); }
  if !cpu.sf { pc = 0x1015ace; continue; }
  pc = 0x1015ac7; continue;
 }
 0x1015ac7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110ed83u32, v); }
  pc = 0x1015ace; continue;
 }
 0x1015ace => {
  { let v: u8 = cpu.dec8(cpu.r8(0x110ed82u32)); cpu.w8(0x110ed82u32, v); }
  if !cpu.sf { pc = 0x1015add; continue; }
  pc = 0x1015ad6; continue;
 }
 0x1015ad6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110ed82u32, v); }
  pc = 0x1015add; continue;
 }
 0x1015add => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015ade(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015ade); }
 let mut pc: u32 = 0x1015ade;
 loop { match pc {
 0x1015ade => {
  { cpu.pushad(); }
  { cpu.sub8(cpu.r8(0x110edc0u32), 0x0u8); }
  if !cpu.zf { pc = 0x1015b04; continue; }
  pc = 0x1015ae8; continue;
 }
 0x1015ae8 => {
  { let v: u32 = cpu.r32(0x11d50dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d50e0u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x22u32); cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1015b04); cpu.call(t); if cpu.returned_to != 0x1015b04 { return; }
  pc = 0x1015b04; continue;
 }
 0x1015b04 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015b06(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015b06); }
 let mut pc: u32 = 0x1015b06;
 loop { match pc {
 0x1015b06 => {
  { let v: u32 = cpu.r32(0x11d50f8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d50fcu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11d50e8u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x11d50ecu32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x11d50f0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x11d50f4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d50dcu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11d50e0u32, v); }
  { cpu.sub32(cpu.ebx, 0xffffffd8u32); }
  if !cpu.zf { pc = 0x1015b46; continue; }
  pc = 0x1015b45; continue;
 }
 0x1015b45 => {
  {  }
  pc = 0x1015b46; continue;
 }
 0x1015b46 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015b47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015b47); }
 let mut pc: u32 = 0x1015b47;
 loop { match pc {
 0x1015b47 => {
  { let v: u32 = cpu.r32(0x11d50dcu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4104u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1015b74; continue; }
  pc = 0x1015b54; continue;
 }
 0x1015b54 => {
  { cpu.sub32(cpu.eax, 0xffffff90u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1015b74; continue; }
  pc = 0x1015b59; continue;
 }
 0x1015b59 => {
  { let v: u32 = cpu.r32(0x11d50e0u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1015b74; continue; }
  pc = 0x1015b66; continue;
 }
 0x1015b66 => {
  { cpu.sub32(cpu.eax, 0xffffffb0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1015b74; continue; }
  pc = 0x1015b6b; continue;
 }
 0x1015b6b => {
  { let v: u8 = 0x0u8; cpu.w8(0x110ed89u32, v); }
  pc = 0x1015b72; continue;
 }
 0x1015b72 => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1015b74 => {
  { let v: u8 = cpu.inc8(cpu.r8(0x110ed89u32)); cpu.w8(0x110ed89u32, v); }
  { cpu.sub8(cpu.r8(0x110ed89u32), 0x11u8); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1015b72; continue; }
  pc = 0x1015b83; continue;
 }
 0x1015b83 => {
  f_01016950(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015b88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015b88); }
 let mut pc: u32 = 0x1015b88;
 loop { match pc {
 0x1015b88 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0xfa0u32); }
  if (cpu.sf != cpu.of) { pc = 0x1015ba8; continue; }
  pc = 0x1015b91; continue;
 }
 0x1015b91 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4fu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xau32); cpu.ebx = v; }
  { let v: u32 = 0x1271u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1015ba5); cpu.call(t); if cpu.returned_to != 0x1015ba5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1015ba8; continue;
 }
 0x1015ba8 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0xbb8u32); }
  if (cpu.sf != cpu.of) { pc = 0x1015bc8; continue; }
  pc = 0x1015bb1; continue;
 }
 0x1015bb1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x47u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xau32); cpu.ebx = v; }
  { let v: u32 = 0x1271u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1015bc5); cpu.call(t); if cpu.returned_to != 0x1015bc5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1015bc8; continue;
 }
 0x1015bc8 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x7d0u32); }
  if (cpu.sf != cpu.of) { pc = 0x1015be8; continue; }
  pc = 0x1015bd1; continue;
 }
 0x1015bd1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4fu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x5u32); cpu.ebx = v; }
  { let v: u32 = 0x1271u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1015be5); cpu.call(t); if cpu.returned_to != 0x1015be5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1015be8; continue;
 }
 0x1015be8 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x3e8u32); }
  if (cpu.sf != cpu.of) { pc = 0x1015c08; continue; }
  pc = 0x1015bf1; continue;
 }
 0x1015bf1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x47u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x5u32); cpu.ebx = v; }
  { let v: u32 = 0x1271u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1015c05); cpu.call(t); if cpu.returned_to != 0x1015c05 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1015c08; continue;
 }
 0x1015c08 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x1388u32); }
  if (cpu.sf != cpu.of) { pc = 0x1015c28; continue; }
  pc = 0x1015c11; continue;
 }
 0x1015c11 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4bu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x7u32); cpu.ebx = v; }
  { let v: u32 = 0x1271u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1015c25); cpu.call(t); if cpu.returned_to != 0x1015c25 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1015c28; continue;
 }
 0x1015c28 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015c29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015c29); }
 let mut pc: u32 = 0x1015c29;
 loop { match pc {
 0x1015c29 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110ed9au32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110ee01u32, v); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1015c48; continue; }
  pc = 0x1015c41; continue;
 }
 0x1015c41 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110ee01u32, v); }
  pc = 0x1015c48; continue;
 }
 0x1015c48 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110ed89u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d50e4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d50e8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d50ecu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d50f0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d50f4u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x110ed81u32, v); }
  { cpu.sub32(cpu.ebp, 0x12345678u32); }
  if cpu.zf { pc = 0x1015c9c; continue; }
  pc = 0x1015c90; continue;
 }
 0x1015c90 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1015c9c; continue;
 }
 0x1015c9c => {
  { let v: u32 = cpu.eax.wrapping_sub(0x60u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x30u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d50f8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11d50fcu32, v); }
  cpu.push(0x1015cb2); f_01015b06(cpu); if cpu.returned_to != 0x1015cb2 { return; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1015cd5; continue; }
  pc = 0x1015cb8; continue;
 }
 0x1015cb8 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1015d43; continue; }
  pc = 0x1015cc2; continue;
 }
 0x1015cc2 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if !cpu.zf { pc = 0x1015d42; continue; }
  pc = 0x1015ccc; continue;
 }
 0x1015ccc => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110ed7fu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1015cd5 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110ed7fu32, v); }
  cpu.push(0x1015ce2); f_0101cc10(cpu); if cpu.returned_to != 0x1015ce2 { return; }
  { let v: u8 = cpu.r8(0x110ed7fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x52u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x2cu32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x10e3f94u32)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x23u32.wrapping_add(cpu.edx)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1119dc5u32, v); }
  { let v: u32 = cpu.r8(0x10f7dc7u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1119dc9u32, v); }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = 0x5eu32; cpu.ebx = v; }
  cpu.push(0x1015d3d); f_0101cebe(cpu); if cpu.returned_to != 0x1015d3d { return; }
  cpu.push(0x1015d42); f_0101cbf7(cpu); if cpu.returned_to != 0x1015d42 { return; }
  pc = 0x1015d42; continue;
 }
 0x1015d42 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1015d43 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110ed7fu32, v); }
  cpu.push(0x1015d50); f_0101cc10(cpu); if cpu.returned_to != 0x1015d50 { return; }
  { let v: u8 = cpu.r8(0x110ed7fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x52u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x2cu32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x10e3f94u32)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x23u32.wrapping_add(cpu.edx)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1119dc5u32, v); }
  { let v: u32 = cpu.r8(0x10f7dc7u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1119dc9u32, v); }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = 0x5eu32; cpu.ebx = v; }
  cpu.push(0x1015dab); f_0101cebe(cpu); if cpu.returned_to != 0x1015dab { return; }
  cpu.push(0x1015db0); f_0101cbf7(cpu); if cpu.returned_to != 0x1015db0 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015db1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015db1); }
 let mut pc: u32 = 0x1015db1;
 loop { match pc {
 0x1015db1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110ed7fu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110ed85u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015dc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015dc0); }
 let mut pc: u32 = 0x1015dc0;
 loop { match pc {
 0x1015dc0 => {
  cpu.push(0x1015dc5); f_0108643d(cpu); if cpu.returned_to != 0x1015dc5 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x110ed7fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x1015dd5; continue; }
  pc = 0x1015dd3; continue;
 }
 0x1015dd3 => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1015dd5; continue;
 }
 0x1015dd5 => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u32 = 0x110edc5u32; cpu.edi = v; }
  { cpu.scas(1, 2); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1015dea; continue; }
  pc = 0x1015de5; continue;
 }
 0x1015de5 => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  pc = 0x1015dea; continue;
 }
 0x1015dea => {
  { let v: u32 = 0x14u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1015e02; continue; }
  pc = 0x1015dfb; continue;
 }
 0x1015dfb => {
  { cpu.sub32(cpu.eax, 0x10u32); }
  if cpu.cf { pc = 0x1015e09; continue; }
  pc = 0x1015e00; continue;
 }
 0x1015e00 => {
  pc = 0x1015e07; continue;
 }
 0x1015e02 => {
  { cpu.sub32(cpu.eax, 0x11u32); }
  if cpu.cf { pc = 0x1015e09; continue; }
  pc = 0x1015e07; continue;
 }
 0x1015e07 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x1015e09; continue;
 }
 0x1015e09 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x110edd9u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110566eu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x110edc5u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110ed7fu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015e28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015e28); }
 let mut pc: u32 = 0x1015e28;
 loop { match pc {
 0x1015e28 => {
  cpu.push(0x1015e2d); f_01086474(cpu); if cpu.returned_to != 0x1015e2d { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x110ed7fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x1015e3d; continue; }
  pc = 0x1015e3b; continue;
 }
 0x1015e3b => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1015e3d; continue;
 }
 0x1015e3d => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u32 = 0x110edc5u32; cpu.edi = v; }
  { cpu.scas(1, 2); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1015e52; continue; }
  pc = 0x1015e4d; continue;
 }
 0x1015e4d => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  pc = 0x1015e52; continue;
 }
 0x1015e52 => {
  { let v: u32 = 0x14u32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.sf { pc = 0x1015e6f; continue; }
  pc = 0x1015e5c; continue;
 }
 0x1015e5c => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1015e6f; continue; }
  pc = 0x1015e6a; continue;
 }
 0x1015e6a => {
  { let v: u32 = 0xfu32; cpu.eax = v; }
  pc = 0x1015e6f; continue;
 }
 0x1015e6f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x110edd9u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110566eu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x110edc5u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110ed7fu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015e8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015e8e); }
 let mut pc: u32 = 0x1015e8e;
 loop { match pc {
 0x1015e8e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x1015e97; continue; }
  pc = 0x1015e95; continue;
 }
 0x1015e95 => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1015e97; continue;
 }
 0x1015e97 => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u32 = 0x110edc5u32; cpu.edi = v; }
  { cpu.scas(1, 2); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1015eac; continue; }
  pc = 0x1015ea7; continue;
 }
 0x1015ea7 => {
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  pc = 0x1015eac; continue;
 }
 0x1015eac => {
  { let v: u32 = 0x14u32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x110edd9u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110566eu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015ec5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015ec5); }
 let mut pc: u32 = 0x1015ec5;
 loop { match pc {
 0x1015ec5 => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x1015eca; continue;
 }
 0x1015eca => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1015ed0); f_01015dc0(cpu); if cpu.returned_to != 0x1015ed0 { return; }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x110ed7fu32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x49u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1015f79; continue; }
  pc = 0x1015ef3; continue;
 }
 0x1015ef3 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1015f79; continue; }
  pc = 0x1015f02; continue;
 }
 0x1015f02 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x1015f79; continue; }
  pc = 0x1015f0b; continue;
 }
 0x1015f0b => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x1015f79; continue; }
  pc = 0x1015f17; continue;
 }
 0x1015f17 => {
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x1015f78; continue; }
  pc = 0x1015f45; continue;
 }
 0x1015f45 => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x1015f78; continue; }
  pc = 0x1015f49; continue;
 }
 0x1015f49 => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x1015f78; continue; }
  pc = 0x1015f4d; continue;
 }
 0x1015f4d => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x1015f78; continue; }
  pc = 0x1015f51; continue;
 }
 0x1015f51 => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x1015f78; continue; }
  pc = 0x1015f55; continue;
 }
 0x1015f55 => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1015f78; continue; }
  pc = 0x1015f59; continue;
 }
 0x1015f59 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x1015f78; continue; }
  pc = 0x1015f5d; continue;
 }
 0x1015f5d => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x1015f78; continue; }
  pc = 0x1015f61; continue;
 }
 0x1015f61 => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if cpu.zf { pc = 0x1015f78; continue; }
  pc = 0x1015f65; continue;
 }
 0x1015f65 => {
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.zf { pc = 0x1015f78; continue; }
  pc = 0x1015f69; continue;
 }
 0x1015f69 => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if cpu.zf { pc = 0x1015f78; continue; }
  pc = 0x1015f6d; continue;
 }
 0x1015f6d => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x1015f78; continue; }
  pc = 0x1015f71; continue;
 }
 0x1015f71 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1015eca; continue; }
  pc = 0x1015f78; continue;
 }
 0x1015f78 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1015f79 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1015eca; continue; }
  pc = 0x1015f81; continue;
 }
 0x1015f81 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01015f82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1015f82); }
 let mut pc: u32 = 0x1015f82;
 loop { match pc {
 0x1015f82 => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x1015f87; continue;
 }
 0x1015f87 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1015f8d); f_01015e28(cpu); if cpu.returned_to != 0x1015f8d { return; }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x110ed7fu32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x49u32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x101603a; continue; }
  pc = 0x1015fb0; continue;
 }
 0x1015fb0 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x101603a; continue; }
  pc = 0x1015fbf; continue;
 }
 0x1015fbf => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x101603a; continue; }
  pc = 0x1015fc8; continue;
 }
 0x1015fc8 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x101603a; continue; }
  pc = 0x1015fd4; continue;
 }
 0x1015fd4 => {
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x1016039; continue; }
  pc = 0x1016002; continue;
 }
 0x1016002 => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x1016039; continue; }
  pc = 0x1016006; continue;
 }
 0x1016006 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x1016039; continue; }
  pc = 0x101600a; continue;
 }
 0x101600a => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x1016039; continue; }
  pc = 0x101600e; continue;
 }
 0x101600e => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x1016039; continue; }
  pc = 0x1016012; continue;
 }
 0x1016012 => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x1016039; continue; }
  pc = 0x1016016; continue;
 }
 0x1016016 => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if cpu.zf { pc = 0x1016039; continue; }
  pc = 0x101601a; continue;
 }
 0x101601a => {
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.zf { pc = 0x1016039; continue; }
  pc = 0x101601e; continue;
 }
 0x101601e => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x1016039; continue; }
  pc = 0x1016022; continue;
 }
 0x1016022 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x1016039; continue; }
  pc = 0x1016026; continue;
 }
 0x1016026 => {
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.zf { pc = 0x1016039; continue; }
  pc = 0x101602a; continue;
 }
 0x101602a => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if cpu.zf { pc = 0x1016039; continue; }
  pc = 0x101602e; continue;
 }
 0x101602e => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x1016039; continue; }
  pc = 0x1016032; continue;
 }
 0x1016032 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1015f87; continue; }
  pc = 0x1016039; continue;
 }
 0x1016039 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x101603a => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1015f87; continue; }
  pc = 0x1016042; continue;
 }
 0x1016042 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016043(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1016043); }
 let mut pc: u32 = 0x1016043;
 loop { match pc {
 0x1016043 => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x1016048; continue;
 }
 0x1016048 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x101604e); f_01015dc0(cpu); if cpu.returned_to != 0x101604e { return; }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x110ed7fu32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x4au32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10160ef; continue; }
  pc = 0x1016071; continue;
 }
 0x1016071 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10160ef; continue; }
  pc = 0x1016080; continue;
 }
 0x1016080 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x10160ef; continue; }
  pc = 0x1016089; continue;
 }
 0x1016089 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x10160ef; continue; }
  pc = 0x1016095; continue;
 }
 0x1016095 => {
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10160ee; continue; }
  pc = 0x10160c3; continue;
 }
 0x10160c3 => {
  { cpu.sub8((cpu.eax as u8), 0x11u8); }
  if cpu.zf { pc = 0x10160ee; continue; }
  pc = 0x10160c7; continue;
 }
 0x10160c7 => {
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x10160ee; continue; }
  pc = 0x10160cb; continue;
 }
 0x10160cb => {
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x10160ee; continue; }
  pc = 0x10160cf; continue;
 }
 0x10160cf => {
  { cpu.sub8((cpu.eax as u8), 0xfu8); }
  if cpu.zf { pc = 0x10160ee; continue; }
  pc = 0x10160d3; continue;
 }
 0x10160d3 => {
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x10160ee; continue; }
  pc = 0x10160d7; continue;
 }
 0x10160d7 => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x10160ee; continue; }
  pc = 0x10160db; continue;
 }
 0x10160db => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x10160ee; continue; }
  pc = 0x10160df; continue;
 }
 0x10160df => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x10160ee; continue; }
  pc = 0x10160e3; continue;
 }
 0x10160e3 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x10160ee; continue; }
  pc = 0x10160e7; continue;
 }
 0x10160e7 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1016048; continue; }
  pc = 0x10160ee; continue;
 }
 0x10160ee => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10160ef => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1016048; continue; }
  pc = 0x10160f7; continue;
 }
 0x10160f7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010160f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10160f8); }
 let mut pc: u32 = 0x10160f8;
 loop { match pc {
 0x10160f8 => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x10160fd; continue;
 }
 0x10160fd => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x1016103); f_01015e28(cpu); if cpu.returned_to != 0x1016103 { return; }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x110ed7fu32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x4au32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10161a4; continue; }
  pc = 0x1016126; continue;
 }
 0x1016126 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10161a4; continue; }
  pc = 0x1016135; continue;
 }
 0x1016135 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x10161a4; continue; }
  pc = 0x101613e; continue;
 }
 0x101613e => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x10161a4; continue; }
  pc = 0x101614a; continue;
 }
 0x101614a => {
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8((cpu.eax as u8), 0x11u8); }
  if cpu.zf { pc = 0x10161a3; continue; }
  pc = 0x1016178; continue;
 }
 0x1016178 => {
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x10161a3; continue; }
  pc = 0x101617c; continue;
 }
 0x101617c => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10161a3; continue; }
  pc = 0x1016180; continue;
 }
 0x1016180 => {
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x10161a3; continue; }
  pc = 0x1016184; continue;
 }
 0x1016184 => {
  { cpu.sub8((cpu.eax as u8), 0xfu8); }
  if cpu.zf { pc = 0x10161a3; continue; }
  pc = 0x1016188; continue;
 }
 0x1016188 => {
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x10161a3; continue; }
  pc = 0x101618c; continue;
 }
 0x101618c => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x10161a3; continue; }
  pc = 0x1016190; continue;
 }
 0x1016190 => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x10161a3; continue; }
  pc = 0x1016194; continue;
 }
 0x1016194 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x10161a3; continue; }
  pc = 0x1016198; continue;
 }
 0x1016198 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x10161a3; continue; }
  pc = 0x101619c; continue;
 }
 0x101619c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10160fd; continue; }
  pc = 0x10161a3; continue;
 }
 0x10161a3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10161a4 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10160fd; continue; }
  pc = 0x10161ac; continue;
 }
 0x10161ac => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010161ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10161ad); }
 let mut pc: u32 = 0x10161ad;
 loop { match pc {
 0x10161ad => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x10161b2; continue;
 }
 0x10161b2 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10161b8); f_01015e28(cpu); if cpu.returned_to != 0x10161b8 { return; }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x110ed7fu32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x4bu32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10162ad; continue; }
  pc = 0x10161db; continue;
 }
 0x10161db => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10162ad; continue; }
  pc = 0x10161ea; continue;
 }
 0x10161ea => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x10162ad; continue; }
  pc = 0x10161f3; continue;
 }
 0x10161f3 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x10162ad; continue; }
  pc = 0x10161ff; continue;
 }
 0x10161ff => {
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8((cpu.eax as u8), 0x11u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016231; continue;
 }
 0x1016231 => {
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016239; continue;
 }
 0x1016239 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016241; continue;
 }
 0x1016241 => {
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016249; continue;
 }
 0x1016249 => {
  { cpu.sub8((cpu.eax as u8), 0xfu8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016251; continue;
 }
 0x1016251 => {
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016259; continue;
 }
 0x1016259 => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016261; continue;
 }
 0x1016261 => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016269; continue;
 }
 0x1016269 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016271; continue;
 }
 0x1016271 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016275; continue;
 }
 0x1016275 => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016279; continue;
 }
 0x1016279 => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x101627d; continue;
 }
 0x101627d => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016281; continue;
 }
 0x1016281 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016285; continue;
 }
 0x1016285 => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016289; continue;
 }
 0x1016289 => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x101628d; continue;
 }
 0x101628d => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016291; continue;
 }
 0x1016291 => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016295; continue;
 }
 0x1016295 => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x1016299; continue;
 }
 0x1016299 => {
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x101629d; continue;
 }
 0x101629d => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x10162a1; continue;
 }
 0x10162a1 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x10162ac; continue; }
  pc = 0x10162a5; continue;
 }
 0x10162a5 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10161b2; continue; }
  pc = 0x10162ac; continue;
 }
 0x10162ac => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10162ad => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10161b2; continue; }
  pc = 0x10162b5; continue;
 }
 0x10162b5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010162b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10162b6); }
 let mut pc: u32 = 0x10162b6;
 loop { match pc {
 0x10162b6 => {
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x10162bb; continue;
 }
 0x10162bb => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10162c1); f_01015dc0(cpu); if cpu.returned_to != 0x10162c1 { return; }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x110ed7fu32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r8(0x4bu32.wrapping_add(cpu.ebx)) as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10163b6; continue; }
  pc = 0x10162e4; continue;
 }
 0x10162e4 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10163b6; continue; }
  pc = 0x10162f3; continue;
 }
 0x10162f3 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x10163b6; continue; }
  pc = 0x10162fc; continue;
 }
 0x10162fc => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x10163b6; continue; }
  pc = 0x1016308; continue;
 }
 0x1016308 => {
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x101633a; continue;
 }
 0x101633a => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x1016342; continue;
 }
 0x1016342 => {
  { cpu.sub8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x101634a; continue;
 }
 0x101634a => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x1016352; continue;
 }
 0x1016352 => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x101635a; continue;
 }
 0x101635a => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x1016362; continue;
 }
 0x1016362 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x101636a; continue;
 }
 0x101636a => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x1016372; continue;
 }
 0x1016372 => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x101637a; continue;
 }
 0x101637a => {
  { cpu.sub8((cpu.eax as u8), 0xau8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x101637e; continue;
 }
 0x101637e => {
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x1016382; continue;
 }
 0x1016382 => {
  { cpu.sub8((cpu.eax as u8), 0xdu8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x1016386; continue;
 }
 0x1016386 => {
  { cpu.sub8((cpu.eax as u8), 0x11u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x101638a; continue;
 }
 0x101638a => {
  { cpu.sub8((cpu.eax as u8), 0x12u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x101638e; continue;
 }
 0x101638e => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x1016392; continue;
 }
 0x1016392 => {
  { cpu.sub8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x1016396; continue;
 }
 0x1016396 => {
  { cpu.sub8((cpu.eax as u8), 0xfu8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x101639a; continue;
 }
 0x101639a => {
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x101639e; continue;
 }
 0x101639e => {
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x10163a2; continue;
 }
 0x10163a2 => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x10163a6; continue;
 }
 0x10163a6 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x10163aa; continue;
 }
 0x10163aa => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x10163b5; continue; }
  pc = 0x10163ae; continue;
 }
 0x10163ae => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10162bb; continue; }
  pc = 0x10163b5; continue;
 }
 0x10163b5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10163b6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10162bb; continue; }
  pc = 0x10163be; continue;
 }
 0x10163be => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010163bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10163bf); }
 let mut pc: u32 = 0x10163bf;
 loop { match pc {
 0x10163bf => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4bu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if !cpu.zf { pc = 0x10163d8; continue; }
  pc = 0x10163ca; continue;
 }
 0x10163ca => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if !cpu.zf { pc = 0x10163d8; continue; }
  pc = 0x10163d2; continue;
 }
 0x10163d2 => {
  { let v: u8 = cpu.r8(0x110ed7fu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10163d8; continue;
 }
 0x10163d8 => {
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x52u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4650u32); cpu.edx = v; }
  { let v: u32 = 0x14u32; cpu.w32(0x110d6dcu32, v); }
  { let v: u32 = 0x2u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x6d616c46u32); }
  if !cpu.zf { pc = 0x1016418; continue; }
  pc = 0x1016402; continue;
 }
 0x1016402 => {
  { cpu.sub16(cpu.r16(0x4u32.wrapping_add(cpu.esi)), 0x7265u16); }
  if !cpu.zf { pc = 0x1016418; continue; }
  pc = 0x101640a; continue;
 }
 0x101640a => {
  { let v: u32 = 0x6f727950u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x73u8; cpu.w8(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x5u32.wrapping_add(cpu.esi), v); }
  pc = 0x1016418; continue;
 }
 0x1016418 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x110ed7fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x110edc0u32), 0x0u8); }
  if cpu.zf { pc = 0x101642d; continue; }
  pc = 0x101642a; continue;
 }
 0x101642a => {
  { let v: u8 = cpu.r8(0x4bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x101642d; continue;
 }
 0x101642d => {
  cpu.push(0x1016432); f_01015e8e(cpu); if cpu.returned_to != 0x1016432 { return; }
  { let v: u16 = cpu.r16(0x110566eu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x3eu32); cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1016447); cpu.call(t); if cpu.returned_to != 0x1016447 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0101644b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101644b); }
 let mut pc: u32 = 0x101644b;
 loop { match pc {
 0x101644b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x4bu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x1016486; continue; }
  pc = 0x1016454; continue;
 }
 0x1016454 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x1016486; continue; }
  pc = 0x101645b; continue;
 }
 0x101645b => {
  { let v: u8 = cpu.r8(0x110ed7fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { pc = 0x1016486; continue; }
  pc = 0x1016464; continue;
 }
 0x1016464 => {
  { let v: u16 = cpu.r16(0x10e3d79u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x101647c; continue; }
  pc = 0x1016470; continue;
 }
 0x1016470 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x101647c; continue; }
  pc = 0x1016476; continue;
 }
 0x1016476 => {
  { let v: u16 = cpu.r16(0x10e3d7bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x101647c; continue;
 }
 0x101647c => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110ed7fu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110ed9au32, v); }
  pc = 0x1016486; continue;
 }
 0x1016486 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x52u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x110ed80u32, v); }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110ed99u32, v); }
  { cpu.sub32(cpu.r32(0x110ead8u32.wrapping_add(cpu.eax.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x10164f4; continue; }
  pc = 0x10164e6; continue;
 }
 0x10164e6 => {
  { let v: u8 = cpu.r8(0x110ed9bu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x10164f3); f_010170e6(cpu); if cpu.returned_to != 0x10164f3 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10164f4 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x110ed80u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x1016501); f_01016569(cpu); if cpu.returned_to != 0x1016501 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10e413cu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1cu32); cpu.ebx = v; }
  { let v: u32 = 0x15u32; cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x1016513; continue;
 }
 0x1016513 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebx)), 0xffffu16); }
  if cpu.zf { pc = 0x101651c; continue; }
  pc = 0x101651a; continue;
 }
 0x101651a => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x101651c; continue;
 }
 0x101651c => {
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1016513; continue; }
  pc = 0x1016522; continue;
 }
 0x1016522 => {
  { let v: u32 = (cpu.eax as u8) as u32; cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1016530); f_01059cfe(cpu); if cpu.returned_to != 0x1016530 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x10ef397u32), 0x30u8); }
  if !cpu.zf { pc = 0x101654b; continue; }
  pc = 0x101653a; continue;
 }
 0x101653a => {
  { let v: u8 = cpu.r8(0x10ef398u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10ef397u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef398u32, v); }
  pc = 0x101654b; continue;
 }
 0x101654b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x18u32); cpu.ebx = v; }
  { let v: u32 = 0x10ef397u32; cpu.esi = v; }
  { let v: u32 = 0x1272u32; cpu.ecx = v; }
  cpu.push(0x1016565); f_0104bcd6(cpu); if cpu.returned_to != 0x1016565 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016569(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1016569); }
 let mut pc: u32 = 0x1016569;
 loop { match pc {
 0x1016569 => {
  { cpu.pushad(); }
  cpu.push(0x101656f); f_0101cc10(cpu); if cpu.returned_to != 0x101656f { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x2cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x23u32.wrapping_add(cpu.edx)) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1119dc5u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r8(0x10f7dc7u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1119dc9u32, v); }
  cpu.push(0x101659d); f_0104f3b7(cpu); if cpu.returned_to != 0x101659d { return; }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10165a8; continue; }
  pc = 0x10165a3; continue;
 }
 0x10165a3 => {
  cpu.push(0x10165a8); f_010338d0(cpu); if cpu.returned_to != 0x10165a8 { return; }
  pc = 0x10165a8; continue;
 }
 0x10165a8 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10165b3); f_0104f7bb(cpu); if cpu.returned_to != 0x10165b3 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x2au32; cpu.ecx = v; }
  { let v = cpu.r32(0x118cd68u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.push(v); }
  { let v: u32 = 0x11de5acu32; cpu.w32(0x118cd68u32.wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x15u32); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10165dd); cpu.call(t); if cpu.returned_to != 0x10165dd { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x118cd68u32.wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x32u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x14u32); cpu.ebx = v; }
  { let v: u32 = 0x126du32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10165fc); cpu.call(t); if cpu.returned_to != 0x10165fc { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x1016607); f_0101cbf7(cpu); if cpu.returned_to != 0x1016607 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0101660b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101660b); }
 let mut pc: u32 = 0x101660b;
 loop { match pc {
 0x101660b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x1016632; continue; }
  pc = 0x1016625; continue;
 }
 0x1016625 => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1016632; continue;
 }
 0x1016632 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x11d50dcu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x11d50e0u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = 0x43u32; cpu.eax = v; }
  { let v: u32 = 0x3eu32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x110ed95u32), 0x3e8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1016661; continue; }
  pc = 0x1016658; continue;
 }
 0x1016658 => {
  { cpu.sub8(cpu.r8(0x110ed94u32), 0x1u8); }
  if cpu.zf { pc = 0x1016661; continue; }
  pc = 0x1016661; continue;
 }
 0x1016661 => {
  { let v: u32 = 0x26u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xeu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x101666c); f_01018ffd(cpu); if cpu.returned_to != 0x101666c { return; }
  if !cpu.cf { pc = 0x1016674; continue; }
  pc = 0x101666e; continue;
 }
 0x101666e => {
  cpu.push(0x1016673); f_010167d1(cpu); if cpu.returned_to != 0x1016673 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016674 => {
  { let v: u32 = 0x43u32; cpu.eax = v; }
  { let v: u32 = 0x2fu32; cpu.ebx = v; }
  { let v: u32 = 0x26u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xeu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x1016689); f_01018ffd(cpu); if cpu.returned_to != 0x1016689 { return; }
  if !cpu.cf { pc = 0x1016691; continue; }
  pc = 0x101668b; continue;
 }
 0x101668b => {
  cpu.push(0x1016690); f_010168f3(cpu); if cpu.returned_to != 0x1016690 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016691 => {
  { let v: u32 = 0x35u32; cpu.eax = v; }
  { let v: u32 = 0x15u32; cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x7u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x10166a6); f_01018ffd(cpu); if cpu.returned_to != 0x10166a6 { return; }
  if !cpu.cf { pc = 0x10166ae; continue; }
  pc = 0x10166a8; continue;
 }
 0x10166a8 => {
  cpu.push(0x10166ad); f_0101697f(cpu); if cpu.returned_to != 0x10166ad { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10166ae => {
  { let v: u32 = 0x35u32; cpu.eax = v; }
  { let v: u32 = 0x41u32; cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x7u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x10166c3); f_01018ffd(cpu); if cpu.returned_to != 0x10166c3 { return; }
  if !cpu.cf { pc = 0x10166cb; continue; }
  pc = 0x10166c5; continue;
 }
 0x10166c5 => {
  cpu.push(0x10166ca); f_010169ce(cpu); if cpu.returned_to != 0x10166ca { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10166cb => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { let v: u32 = 0x16u32; cpu.ebx = v; }
  { let v: u32 = 0x2bu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0x25u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x10166e0); f_01018ffd(cpu); if cpu.returned_to != 0x10166e0 { return; }
  if !cpu.cf { pc = 0x10166e8; continue; }
  pc = 0x10166e2; continue;
 }
 0x10166e2 => {
  cpu.push(0x10166e7); f_010167d1(cpu); if cpu.returned_to != 0x10166e7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10166e8 => {
  cpu.push(0x10166ed); f_01016708(cpu); if cpu.returned_to != 0x10166ed { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010166ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10166ee); }
 let mut pc: u32 = 0x10166ee;
 loop { match pc {
 0x10166ee => {
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if cpu.zf { pc = 0x1016707; continue; }
  pc = 0x10166f7; continue;
 }
 0x10166f7 => {
  { cpu.and8(cpu.r8(0x10ee968u32), 0x1u8); }
  if !cpu.zf { pc = 0x1016707; continue; }
  pc = 0x1016700; continue;
 }
 0x1016700 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110ed81u32, v); }
  pc = 0x1016707; continue;
 }
 0x1016707 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016708(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1016708); }
 let mut pc: u32 = 0x1016708;
 loop { match pc {
 0x1016708 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10ee97du32, v); }
  { cpu.sub8(cpu.r8(0x110ed81u32), 0x0u8); }
  if cpu.zf { pc = 0x1016757; continue; }
  pc = 0x1016718; continue;
 }
 0x1016718 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x101673f; continue; }
  pc = 0x1016732; continue;
 }
 0x1016732 => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x101673f; continue;
 }
 0x101673f => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d50e8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11d50ecu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016757 => {
  { let v: u32 = cpu.r32(0x11d50f0u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x11d50f4u32)); cpu.eax = v; }
  if cpu.zf { pc = 0x1016780; continue; }
  pc = 0x1016764; continue;
 }
 0x1016764 => {
  cpu.push(0x1016769); f_01015b06(cpu); if cpu.returned_to != 0x1016769 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d50f8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11d50fcu32, v); }
  pc = 0x1016780; continue;
 }
 0x1016780 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x10167a7; continue; }
  pc = 0x101679a; continue;
 }
 0x101679a => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10167a7; continue;
 }
 0x10167a7 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f60u32)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10e3f64u32)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d50f0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11d50f4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11d50e8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11d50ecu32, v); }
  { let v: u8 = 0x1u8; cpu.w8(0x110ed81u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010167d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10167d1); }
 let mut pc: u32 = 0x10167d1;
 loop { match pc {
 0x10167d1 => {
  { cpu.sub32(cpu.r32(0x11d50e4u32), 0x0u32); }
  if cpu.zf { pc = 0x10167db; continue; }
  pc = 0x10167da; continue;
 }
 0x10167da => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10167db => {
  { cpu.sub8(cpu.r8(0x110ed82u32), 0x0u8); }
  if cpu.zf { pc = 0x10167e5; continue; }
  pc = 0x10167e4; continue;
 }
 0x10167e4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10167e5 => {
  { let v: u8 = 0x3u8; cpu.w8(0x110ed83u32, v); }
  f_010167ec(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010167ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10167ec); }
 let mut pc: u32 = 0x10167ec;
 loop { match pc {
 0x10167ec => {
  { let v: u32 = cpu.r32(0x110eab4u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1016888; continue; }
  pc = 0x10167fc; continue;
 }
 0x10167fc => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1016816; continue; }
  pc = 0x1016802; continue;
 }
 0x1016802 => {
  { cpu.sub32(cpu.r32(0x11d50e4u32), 0x0u32); }
  if !cpu.zf { pc = 0x1016815; continue; }
  pc = 0x101680b; continue;
 }
 0x101680b => {
  { let v: u32 = 0x4u32; cpu.w32(0x11d50e4u32, v); }
  pc = 0x1016815; continue;
 }
 0x1016815 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016816 => {
  { let v: u8 = cpu.r8(0x110ed7fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1016820); f_01017741(cpu); if cpu.returned_to != 0x1016820 { return; }
  if cpu.cf { pc = 0x101687f; continue; }
  pc = 0x1016826; continue;
 }
 0x1016826 => {
  { let v: u32 = 0x16u32; cpu.w32(0x11d50e4u32, v); }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1016840; continue; }
  pc = 0x1016839; continue;
 }
 0x1016839 => {
  { let v: u32 = cpu.r32(0x11d50e4u32).wrapping_add(0xau32); cpu.w32(0x11d50e4u32, v); }
  pc = 0x1016840; continue;
 }
 0x1016840 => {
  { let v: u8 = cpu.r8(0x110ed7fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x101684a); f_010168c9(cpu); if cpu.returned_to != 0x101684a { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x1016855); f_010363b2(cpu); if cpu.returned_to != 0x1016855 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1016880; continue; }
  pc = 0x101685c; continue;
 }
 0x101685c => {
  cpu.push(0x1016861); f_0101cc10(cpu); if cpu.returned_to != 0x1016861 { return; }
  cpu.push(0x1016866); f_0105f178(cpu); if cpu.returned_to != 0x1016866 { return; }
  { let v: u32 = cpu.and32(cpu.eax, 0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110eb28u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x101687a); f_0101cebe(cpu); if cpu.returned_to != 0x101687a { return; }
  cpu.push(0x101687f); f_0101cbf7(cpu); if cpu.returned_to != 0x101687f { return; }
  pc = 0x101687f; continue;
 }
 0x101687f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016880 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x11d50e4u32), 0x12u32); cpu.w32(0x11d50e4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016888 => {
  cpu.push(0x101688d); f_010168a4(cpu); if cpu.returned_to != 0x101688d { return; }
  { let v: u8 = 0x2u8; cpu.w8(0x110ed93u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x110eab4u32, v); }
  cpu.push(0x10168a3); f_010863ff(cpu); if cpu.returned_to != 0x10168a3 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010168a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10168a4); }
 let mut pc: u32 = 0x10168a4;
 loop { match pc {
 0x10168a4 => {
  { cpu.pushad(); }
  { let v: u8 = 0xf7u8; cpu.w8(0x10ef210u32, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x116c0d8u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3e0fu32, v); }
  cpu.push(0x10168c7); f_01036a75(cpu); if cpu.returned_to != 0x10168c7 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010168c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10168c9); }
 let mut pc: u32 = 0x10168c9;
 loop { match pc {
 0x10168c9 => {
  { cpu.pushad(); }
  { let v: u8 = 0xffu8; cpu.w8(0x10ef210u32, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x116c0d8u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4048u32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3e0fu32, v); }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3e11u32, v); }
  cpu.push(0x10168f1); f_01036a75(cpu); if cpu.returned_to != 0x10168f1 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010168f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10168f3); }
 let mut pc: u32 = 0x10168f3;
 loop { match pc {
 0x10168f3 => {
  { cpu.sub32(cpu.r32(0x11d50e4u32), 0x0u32); }
  if cpu.zf { pc = 0x10168fd; continue; }
  pc = 0x10168fc; continue;
 }
 0x10168fc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10168fd => {
  { cpu.sub8(cpu.r8(0x110ed83u32), 0x0u8); }
  if cpu.zf { pc = 0x1016907; continue; }
  pc = 0x1016906; continue;
 }
 0x1016906 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016907 => {
  { let v: u8 = 0x3u8; cpu.w8(0x110ed82u32, v); }
  { let v: u32 = cpu.r32(0x110eab4u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1016934; continue; }
  pc = 0x101691a; continue;
 }
 0x101691a => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1016934; continue; }
  pc = 0x1016920; continue;
 }
 0x1016920 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.push(0x101692b); f_010363b2(cpu); if cpu.returned_to != 0x101692b { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1016933); f_010168c9(cpu); if cpu.returned_to != 0x1016933 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016934 => {
  { let v: u32 = 0x4u32; cpu.w32(0x11d50e4u32, v); }
  f_01016950(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016940(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1016940); }
 let mut pc: u32 = 0x1016940;
 loop { match pc {
 0x1016940 => {
  { cpu.sub32(cpu.r32(0x110eab4u32), 0x0u32); }
  if !cpu.zf { pc = 0x101694a; continue; }
  pc = 0x1016949; continue;
 }
 0x1016949 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x101694a => {
  cpu.push(0x101694f); f_01016950(cpu); if cpu.returned_to != 0x101694f { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016950(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1016950); }
 let mut pc: u32 = 0x1016950;
 loop { match pc {
 0x1016950 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x110ed9au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x101696d; continue; }
  pc = 0x101695a; continue;
 }
 0x101695a => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x110ed7fu32, v); }
  { cpu.sub8(cpu.r8(0x110edc0u32), 0x0u8); }
  if !cpu.zf { pc = 0x101696d; continue; }
  pc = 0x1016968; continue;
 }
 0x1016968 => {
  cpu.push(0x101696d); f_010167ec(cpu); if cpu.returned_to != 0x101696d { return; }
  pc = 0x101696d; continue;
 }
 0x101696d => {
  { let v: u32 = 0x0u32; cpu.w32(0x110eab4u32, v); }
  cpu.push(0x101697c); f_010863ff(cpu); if cpu.returned_to != 0x101697c { return; }
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0101697f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101697f); }
 let mut pc: u32 = 0x101697f;
 loop { match pc {
 0x101697f => {
  { cpu.sub32(cpu.r32(0x11d50e4u32), 0x0u32); }
  if cpu.zf { pc = 0x1016989; continue; }
  pc = 0x1016988; continue;
 }
 0x1016988 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016989 => {
  { cpu.sub8(cpu.r8(0x110ed82u32), 0x0u8); }
  if cpu.zf { pc = 0x1016993; continue; }
  pc = 0x1016992; continue;
 }
 0x1016992 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016993 => {
  { cpu.sub8(cpu.r8(0x110ed83u32), 0x0u8); }
  if cpu.zf { pc = 0x101699d; continue; }
  pc = 0x101699c; continue;
 }
 0x101699c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x101699d => {
  { let v: u32 = cpu.r32(0x110eab4u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x10169cd; continue; }
  pc = 0x10169a9; continue;
 }
 0x10169a9 => {
  { let v: u8 = 0x3u8; cpu.w8(0x110ed7du32, v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x10169c2; continue; }
  pc = 0x10169b6; continue;
 }
 0x10169b6 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x10169c8; continue; }
  pc = 0x10169bc; continue;
 }
 0x10169bc => {
  cpu.push(0x10169c1); f_01016043(cpu); if cpu.returned_to != 0x10169c1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10169c2 => {
  cpu.push(0x10169c7); f_01015ec5(cpu); if cpu.returned_to != 0x10169c7 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10169c8 => {
  cpu.push(0x10169cd); f_010162b6(cpu); if cpu.returned_to != 0x10169cd { return; }
  pc = 0x10169cd; continue;
 }
 0x10169cd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010169ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10169ce); }
 let mut pc: u32 = 0x10169ce;
 loop { match pc {
 0x10169ce => {
  { cpu.sub32(cpu.r32(0x11d50e4u32), 0x0u32); }
  if cpu.zf { pc = 0x10169d8; continue; }
  pc = 0x10169d7; continue;
 }
 0x10169d7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10169d8 => {
  { cpu.sub8(cpu.r8(0x110ed82u32), 0x0u8); }
  if cpu.zf { pc = 0x10169e2; continue; }
  pc = 0x10169e1; continue;
 }
 0x10169e1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10169e2 => {
  { cpu.sub8(cpu.r8(0x110ed83u32), 0x0u8); }
  if cpu.zf { pc = 0x10169ec; continue; }
  pc = 0x10169eb; continue;
 }
 0x10169eb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10169ec => {
  { let v: u32 = cpu.r32(0x110eab4u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1016a1c; continue; }
  pc = 0x10169f8; continue;
 }
 0x10169f8 => {
  { let v: u8 = 0x3u8; cpu.w8(0x110ed7eu32, v); }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1016a11; continue; }
  pc = 0x1016a05; continue;
 }
 0x1016a05 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1016a17; continue; }
  pc = 0x1016a0b; continue;
 }
 0x1016a0b => {
  cpu.push(0x1016a10); f_010160f8(cpu); if cpu.returned_to != 0x1016a10 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016a11 => {
  cpu.push(0x1016a16); f_01015f82(cpu); if cpu.returned_to != 0x1016a16 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016a17 => {
  cpu.push(0x1016a1c); f_010161ad(cpu); if cpu.returned_to != 0x1016a1c { return; }
  pc = 0x1016a1c; continue;
 }
 0x1016a1c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016a1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1016a1d); }
 let mut pc: u32 = 0x1016a1d;
 loop { match pc {
 0x1016a1d => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1016acd; continue; }
  pc = 0x1016a27; continue;
 }
 0x1016a27 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x35u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x15u32); cpu.ebx = v; }
  { let v: u32 = 0x1266u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1016a3b); cpu.call(t); if cpu.returned_to != 0x1016a3b { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1016acd; continue; }
  pc = 0x1016a48; continue;
 }
 0x1016a48 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x25u32); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  cpu.push(0x1016a6f); f_01087028(cpu); if cpu.returned_to != 0x1016a6f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0xddu32); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x10e3d81u32); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x35u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x10e4108u32)); }
  if (cpu.sf == cpu.of) { pc = 0x1016aca; continue; }
  pc = 0x1016aa6; continue;
 }
 0x1016aa6 => {
  { cpu.sub32(cpu.ecx, 0x0u32); }
  if cpu.sf { pc = 0x1016aca; continue; }
  pc = 0x1016aab; continue;
 }
 0x1016aab => {
  if cpu.zf { pc = 0x1016aca; continue; }
  pc = 0x1016aad; continue;
 }
 0x1016aad => {
  { let v = cpu.r32(0x10e4108u32); cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10e4108u32, v); }
  { let v: u32 = 0x1267u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1016ac4); cpu.call(t); if cpu.returned_to != 0x1016ac4 { return; }
  { let v: u32 = cpu.pop(); cpu.w32(0x10e4108u32, v); }
  pc = 0x1016aca; continue;
 }
 0x1016aca => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1016acd; continue;
 }
 0x1016acd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016ace(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1016ace); }
 let mut pc: u32 = 0x1016ace;
 loop { match pc {
 0x1016ace => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016ad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1016ad0); }
 let mut pc: u32 = 0x1016ad0;
 loop { match pc {
 0x1016ad0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x43u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2fu32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x110ed82u32), 0x0u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x125eu32); cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1016af4); cpu.call(t); if cpu.returned_to != 0x1016af4 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x43u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x3eu32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x126eu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1016b0f; continue; }
  pc = 0x1016b0a; continue;
 }
 0x1016b0a => {
  { let v: u32 = 0x1260u32; cpu.ecx = v; }
  pc = 0x1016b0f; continue;
 }
 0x1016b0f => {
  { cpu.sub8(cpu.r8(0x110ed83u32), 0x0u8); }
  if cpu.zf { pc = 0x1016b19; continue; }
  pc = 0x1016b18; continue;
 }
 0x1016b18 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x1016b19; continue;
 }
 0x1016b19 => {
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1016b1f); cpu.call(t); if cpu.returned_to != 0x1016b1f { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1016b70; continue; }
  pc = 0x1016b28; continue;
 }
 0x1016b28 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x110ed7du32), 0x0u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1262u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x32u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x15u32); cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1016b49); cpu.call(t); if cpu.returned_to != 0x1016b49 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x110ed7eu32), 0x0u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1264u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x32u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x40u32); cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1016b6d); cpu.call(t); if cpu.returned_to != 0x1016b6d { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1016b70; continue;
 }
 0x1016b70 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016b71(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1016b71); }
 let mut pc: u32 = 0x1016b71;
 loop { match pc {
 0x1016b71 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.ebp, 0x12345678u32); }
  if cpu.zf { pc = 0x1016ba7; continue; }
  pc = 0x1016b7c; continue;
 }
 0x1016b7c => {
  { let v: u32 = 0x1257u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if cpu.zf { pc = 0x1016b97; continue; }
  pc = 0x1016b87; continue;
 }
 0x1016b87 => {
  { let v: u32 = 0x1258u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1016b97; continue; }
  pc = 0x1016b92; continue;
 }
 0x1016b92 => {
  { let v: u32 = 0x1259u32; cpu.ecx = v; }
  pc = 0x1016b97; continue;
 }
 0x1016b97 => {
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x6u32); cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1016ba3); cpu.call(t); if cpu.returned_to != 0x1016ba3 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016ba7 => {
  { let v: u32 = 0x125au32; cpu.ecx = v; }
  cpu.poll(); pc = 0x1016b97; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016bae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1016bae); }
 let mut pc: u32 = 0x1016bae;
 loop { match pc {
 0x1016bae => {
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1au32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x1016c37; continue; }
  pc = 0x1016bbc; continue;
 }
 0x1016bbc => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = (cpu.edx as u16) as u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x64u32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u8 = 0xau8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u8 = 0xau8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x58u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x5u32); cpu.ebx = v; }
  { let v: u32 = 0x13u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xfu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0x79u32; cpu.ebp = v; }
  cpu.push(0x1016bf6); f_0106faa0(cpu); if cpu.returned_to != 0x1016bf6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1016c0b); f_01059cfe(cpu); if cpu.returned_to != 0x1016c0b { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x25u8; cpu.w8(0x10ef399u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10ef39au32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5bu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x6u32); cpu.ebx = v; }
  { let v: u32 = 0x1272u32; cpu.ecx = v; }
  { let v: u32 = 0x10ef397u32; cpu.esi = v; }
  cpu.push(0x1016c33); f_0104bcd6(cpu); if cpu.returned_to != 0x1016c33 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016c37 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x110ed78u32; cpu.esi = v; }
  { let v: u32 = 0x1272u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x59u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x6u32); cpu.ebx = v; }
  cpu.push(0x1016c4f); f_0104bcd6(cpu); if cpu.returned_to != 0x1016c4f { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016c53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1016c53); }
 let mut pc: u32 = 0x1016c53;
 loop { match pc {
 0x1016c53 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1256u32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1016c61); cpu.call(t); if cpu.returned_to != 0x1016c61 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xf0u32; cpu.eax = v; }
  { let v = cpu.r8(0x10e3d63u32); cpu.mul8(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10c2402u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x761u32; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x6u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x16u32); cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1016c8d); cpu.call(t); if cpu.returned_to != 0x1016c8d { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016c91(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1016c91); } r_01016c91(cpu, 0x1016c91); }
pub fn f_01016d33(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1016d33); } r_01016c91(cpu, 0x1016d33); }
fn r_01016c91(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1016c91 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110edc0u32, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.ebp, 0x12345678u32); }
  if cpu.zf { pc = 0x1016d0d; continue; }
  pc = 0x1016ca7; continue;
 }
 0x1016ca7 => {
  { let v: u32 = 0x125du32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1016cc2; continue; }
  pc = 0x1016cb2; continue;
 }
 0x1016cb2 => {
  { let v: u32 = 0x125bu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1016cc2; continue; }
  pc = 0x1016cbd; continue;
 }
 0x1016cbd => {
  { let v: u32 = 0x125cu32; cpu.ecx = v; }
  pc = 0x1016cc2; continue;
 }
 0x1016cc2 => {
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x13u32); cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1016cce); cpu.call(t); if cpu.returned_to != 0x1016cce { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x1016cd6); f_01016da8(cpu); if cpu.returned_to != 0x1016cd6 { return; }
  { cpu.and8(cpu.r8(0x10e3d07u32), 0x2u8); }
  if cpu.zf { pc = 0x1016d0c; continue; }
  pc = 0x1016cdf; continue;
 }
 0x1016cdf => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x126au32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1016cfd; continue; }
  pc = 0x1016ced; continue;
 }
 0x1016ced => {
  { let v: u32 = 0x1268u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1016cfd; continue; }
  pc = 0x1016cf8; continue;
 }
 0x1016cf8 => {
  { let v: u32 = 0x1269u32; cpu.ecx = v; }
  pc = 0x1016cfd; continue;
 }
 0x1016cfd => {
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x13u32); cpu.ebx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1016d09); cpu.call(t); if cpu.returned_to != 0x1016d09 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x1016d0c; continue;
 }
 0x1016d0c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016d0d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x125bu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x110ed88u32), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x1016cc2; continue; }
  pc = 0x1016d1e; continue;
 }
 0x1016d1e => {
  { let v: u32 = 0x125cu32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x110ed86u32), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x1016cc2; continue; }
  pc = 0x1016d2c; continue;
 }
 0x1016d2c => {
  { let v: u32 = 0x125du32; cpu.ecx = v; }
  cpu.poll(); pc = 0x1016cc2; continue;
 }
 0x1016d33 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1268u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x110ed88u32), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x1016cc2; continue; }
  pc = 0x1016d48; continue;
 }
 0x1016d48 => {
  { let v: u32 = 0x1269u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x110ed86u32), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x1016cc2; continue; }
  pc = 0x1016d5a; continue;
 }
 0x1016d5a => {
  { let v: u32 = 0x126au32; cpu.ecx = v; }
  cpu.poll(); pc = 0x1016cc2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016d64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1016d64); }
 let mut pc: u32 = 0x1016d64;
 loop { match pc {
 0x1016d64 => {
  { cpu.sub8(cpu.r8(0x110edc0u32), 0x0u8); }
  if !cpu.zf { pc = 0x1016da7; continue; }
  pc = 0x1016d6d; continue;
 }
 0x1016d6d => {
  { let v: u32 = cpu.r32(0x11d50dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d50e0u32); cpu.ebx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x13u32); cpu.ebx = v; }
  { let v: u32 = 0x126cu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1016d8a); cpu.call(t); if cpu.returned_to != 0x1016d8a { return; }
  { cpu.popad(); }
  { cpu.and8(cpu.r8(0x10e3d07u32), 0x2u8); }
  if cpu.zf { pc = 0x1016da7; continue; }
  pc = 0x1016d94; continue;
 }
 0x1016d94 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_add(0x40u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x13u32); cpu.ebx = v; }
  { let v: u32 = 0x126bu32; cpu.ecx = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1016da6); cpu.call(t); if cpu.returned_to != 0x1016da6 { return; }
  { cpu.popad(); }
  pc = 0x1016da7; continue;
 }
 0x1016da7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016da8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1016da8); }
 let mut pc: u32 = 0x1016da8;
 loop { match pc {
 0x1016da8 => {
  { let v: u8 = 0x1u8; cpu.w8(0x110edc0u32, v); }
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1016dbc; continue; }
  pc = 0x1016db5; continue;
 }
 0x1016db5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110edc0u32, v); }
  pc = 0x1016dbc; continue;
 }
 0x1016dbc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016dbd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1016dbd); }
 let mut pc: u32 = 0x1016dbd;
 loop { match pc {
 0x1016dbd => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.ebp, 0x12345678u32); }
  if !cpu.zf { pc = 0x1016dd1; continue; }
  pc = 0x1016dc8; continue;
 }
 0x1016dc8 => {
  cpu.push(0x1016dcd); f_01016f05(cpu); if cpu.returned_to != 0x1016dcd { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016dd1 => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1016ed4; continue; }
  pc = 0x1016ddb; continue;
 }
 0x1016ddb => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1016e65; continue; }
  pc = 0x1016de5; continue;
 }
 0x1016de5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1016dec); f_01031c81(cpu); if cpu.returned_to != 0x1016dec { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1016e2e; continue; }
  pc = 0x1016df2; continue;
 }
 0x1016df2 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x112d78fu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x112d793u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x23u32); cpu.ebx = v; }
  { let v: u32 = 0x1272u32; cpu.ecx = v; }
  cpu.push(0x1016e2a); f_01017020(cpu); if cpu.returned_to != 0x1016e2a { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016e2e => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x58u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x23u32); cpu.ebx = v; }
  { let v: u32 = 0x1272u32; cpu.ecx = v; }
  { let v: u32 = 0x110edbbu32; cpu.esi = v; }
  cpu.push(0x1016e45); f_0104bcd6(cpu); if cpu.returned_to != 0x1016e45 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016e49 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x58u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x23u32); cpu.ebx = v; }
  { let v: u32 = 0x1272u32; cpu.ecx = v; }
  { let v: u32 = 0x110edbbu32; cpu.esi = v; }
  cpu.push(0x1016e61); f_0104bcd6(cpu); if cpu.returned_to != 0x1016e61 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016e65 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x52u32; cpu.eax = v; }
  { let v = cpu.r8(0x110ed7fu32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  cpu.push(0x1016e83); f_01087028(cpu); if cpu.returned_to != 0x1016e83 { return; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xddu32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x10e3d81u32); cpu.div32(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1016e97); f_01031c81(cpu); if cpu.returned_to != 0x1016e97 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { cpu.poll(); pc = 0x1016e49; continue; }
  pc = 0x1016ea0; continue;
 }
 0x1016ea0 => {
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x112d78fu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x112d793u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x23u32); cpu.ebx = v; }
  { let v: u32 = 0x1272u32; cpu.ecx = v; }
  cpu.push(0x1016ed0); f_01017020(cpu); if cpu.returned_to != 0x1016ed0 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016ed4 => {
  { cpu.and8(cpu.r8(0x10e3d07u32), 0x2u8); }
  if cpu.zf { pc = 0x1016f01; continue; }
  pc = 0x1016edd; continue;
 }
 0x1016edd => {
  { let v: u32 = 0x0u32; cpu.w32(0x112d78fu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x112d793u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x23u32); cpu.ebx = v; }
  { let v: u32 = 0x1272u32; cpu.ecx = v; }
  cpu.push(0x1016f01); f_01017020(cpu); if cpu.returned_to != 0x1016f01 { return; }
  pc = 0x1016f01; continue;
 }
 0x1016f01 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016f05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1016f05); }
 let mut pc: u32 = 0x1016f05;
 loop { match pc {
 0x1016f05 => {
  { cpu.sub8(cpu.r8(0x110ed87u32), 0xffu8); }
  if !cpu.zf { pc = 0x1016fef; continue; }
  pc = 0x1016f12; continue;
 }
 0x1016f12 => {
  { cpu.sub8(cpu.r8(0x110ed88u32), 0xffu8); }
  if cpu.zf { pc = 0x1016f80; continue; }
  pc = 0x1016f1f; continue;
 }
 0x1016f1f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x1016f26); f_01031c81(cpu); if cpu.returned_to != 0x1016f26 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1016f49; continue; }
  pc = 0x1016f2c; continue;
 }
 0x1016f2c => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x110ead4u32); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  pc = 0x1016f49; continue;
 }
 0x1016f49 => {
  { let v: u32 = cpu.eax; cpu.w32(0x112d78fu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x112d793u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x23u32); cpu.ebx = v; }
  { let v: u32 = 0x1272u32; cpu.ecx = v; }
  cpu.push(0x1016f66); f_01017020(cpu); if cpu.returned_to != 0x1016f66 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016f80 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1016fef => {
  { cpu.and8(cpu.r8(0x10e3d07u32), 0x2u8); }
  if cpu.zf { pc = 0x101701c; continue; }
  pc = 0x1016ff8; continue;
 }
 0x1016ff8 => {
  { let v: u32 = 0x0u32; cpu.w32(0x112d78fu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x112d793u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x23u32); cpu.ebx = v; }
  { let v: u32 = 0x1272u32; cpu.ecx = v; }
  cpu.push(0x101701c); f_01017020(cpu); if cpu.returned_to != 0x101701c { return; }
  pc = 0x101701c; continue;
 }
 0x101701c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01016f6a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1016f6a); } r_01016f6a(cpu, 0x1016f6a); }
pub fn f_01016f81(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1016f81); } r_01016f6a(cpu, 0x1016f81); }
fn r_01016f6a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1016f6a => {
  { let v: u32 = 0x0u32; cpu.w32(0x112d78fu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x112d793u32, v); }
  pc = 0x1016fd8; continue;
 }
 0x1016f81 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x4au32; cpu.eax = v; }
  { let v = cpu.r8(0x110ed7fu32); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x10c6120u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4650u32); cpu.eax = v; }
  cpu.push(0x1016f9e); f_01087028(cpu); if cpu.returned_to != 0x1016f9e { return; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xddu32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x10e3d81u32); cpu.div32(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1016fb2); f_01031c81(cpu); if cpu.returned_to != 0x1016fb2 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { cpu.poll(); pc = 0x1016f6a; continue; }
  pc = 0x1016fbb; continue;
 }
 0x1016fbb => {
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x112d78fu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x112d793u32, v); }
  pc = 0x1016fd8; continue;
 }
 0x1016fd8 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x23u32); cpu.ebx = v; }
  { let v: u32 = 0x1272u32; cpu.ecx = v; }
  cpu.push(0x1016feb); f_01017020(cpu); if cpu.returned_to != 0x1016feb { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017020); }
 let mut pc: u32 = 0x1017020;
 loop { match pc {
 0x1017020 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x112d78fu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xau32); }
  if cpu.cf { pc = 0x101703d; continue; }
  pc = 0x101702e; continue;
 }
 0x101702e => {
  { let v: u32 = 0x9u32; cpu.edx = v; }
  { let v: u32 = 0x3bu32; cpu.w32(0x112d793u32, v); }
  pc = 0x101703d; continue;
 }
 0x101703d => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x1017047); f_01059cfe(cpu); if cpu.returned_to != 0x1017047 { return; }
  { let v: u32 = 0x1194208u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10ef397u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x112d793u32); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  cpu.push(0x1017068); f_01059cfe(cpu); if cpu.returned_to != 0x1017068 { return; }
  { let v: u16 = cpu.r16(0x10ef397u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x1194208u32; cpu.esi = v; }
  cpu.push(0x1017082); f_0104bcd6(cpu); if cpu.returned_to != 0x1017082 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017083(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017083); }
 let mut pc: u32 = 0x1017083;
 loop { match pc {
 0x1017083 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x112d78fu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xau32); }
  if cpu.cf { pc = 0x10170a0; continue; }
  pc = 0x1017091; continue;
 }
 0x1017091 => {
  { let v: u32 = 0x9u32; cpu.edx = v; }
  { let v: u32 = 0x3bu32; cpu.w32(0x112d793u32, v); }
  pc = 0x10170a0; continue;
 }
 0x10170a0 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x10170aa); f_01059cfe(cpu); if cpu.returned_to != 0x10170aa { return; }
  { let v: u32 = 0x1194208u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10ef397u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x112d793u32); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  cpu.push(0x10170cb); f_01059cfe(cpu); if cpu.returned_to != 0x10170cb { return; }
  { let v: u16 = cpu.r16(0x10ef397u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x1194208u32; cpu.esi = v; }
  cpu.push(0x10170e5); f_0104b988(cpu); if cpu.returned_to != 0x10170e5 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010170e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10170e6); }
 let mut pc: u32 = 0x10170e6;
 loop { match pc {
 0x10170e6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x10fcc16u32, v); }
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x10170fd; continue; }
  pc = 0x10170f6; continue;
 }
 0x10170f6 => {
  { let v: u8 = 0x1u8; cpu.w8(0x10fcc16u32, v); }
  pc = 0x10170fd; continue;
 }
 0x10170fd => {
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x110eb38u32.wrapping_add(cpu.edx.wrapping_mul(8)))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x110eb3cu32.wrapping_add(cpu.edx.wrapping_mul(8)))); cpu.ebx = v; }
  { let v: u32 = 0x10e4978u32; cpu.w32(0x10e4010u32, v); }
  { let v: u32 = 0x10e55fau32; cpu.w32(0x10e401cu32, v); }
  { let v: u32 = 0x11d5798u32; cpu.esi = v; }
  { let v: u32 = 0x11d5850u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x30u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x62u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x7u8; cpu.w8(0x94u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x13u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1du32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10e4010u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edx), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x5u8); }
  if cpu.zf { pc = 0x1017263; continue; }
  pc = 0x1017165; continue;
 }
 0x1017165 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x15u8); }
  if cpu.zf { pc = 0x1017263; continue; }
  pc = 0x101716f; continue;
 }
 0x101716f => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x16u8); }
  if cpu.zf { pc = 0x1017263; continue; }
  pc = 0x1017179; continue;
 }
 0x1017179 => {
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x17u8); }
  if cpu.zf { pc = 0x1017263; continue; }
  pc = 0x1017183; continue;
 }
 0x1017183 => {
  { let v: u8 = 0x6u8; cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.inc8(cpu.r8(0x32u32.wrapping_add(cpu.esi))); cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x32u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.cf { pc = 0x1017194; continue; }
  pc = 0x1017190; continue;
 }
 0x1017190 => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.esi), v); }
  pc = 0x1017194; continue;
 }
 0x1017194 => {
  { let v: u16 = 0x64u16; cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x64u16; cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.esi)), 0x24u8); }
  if cpu.zf { pc = 0x10171c5; continue; }
  pc = 0x10171a6; continue;
 }
 0x10171a6 => {
  { cpu.sub8(cpu.r8(0x10e3d07u32), 0x0u8); }
  if !cpu.zf { pc = 0x101721b; continue; }
  pc = 0x10171b3; continue;
 }
 0x10171b3 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { pc = 0x1017214; continue; }
  pc = 0x10171c0; continue;
 }
 0x10171c0 => {
  pc = 0x101721b; continue;
 }
 0x10171c5 => {
  { let v: u8 = 0x2u8; cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { pc = 0x101721b; continue; }
  pc = 0x10171d2; continue;
 }
 0x10171d2 => {
  { cpu.and8(cpu.r8(0x96u32.wrapping_add(cpu.esi)), 0x40u8); }
  if !cpu.zf { pc = 0x10171f3; continue; }
  pc = 0x10171db; continue;
 }
 0x10171db => {
  { let v: u8 = cpu.inc8(cpu.r8(0x96u32.wrapping_add(cpu.esi))); cpu.w8(0x96u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x96u32.wrapping_add(cpu.esi)), 0x7u8); }
  if !cpu.zf { pc = 0x101721b; continue; }
  pc = 0x10171ea; continue;
 }
 0x10171ea => {
  { let v: u8 = cpu.or8(cpu.r8(0x96u32.wrapping_add(cpu.esi)), 0x40u8); cpu.w8(0x96u32.wrapping_add(cpu.esi), v); }
  pc = 0x101721b; continue;
 }
 0x10171f3 => {
  { let v: u8 = cpu.r8(0x96u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x101720b; continue; }
  pc = 0x1017201; continue;
 }
 0x1017201 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x40u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x96u32.wrapping_add(cpu.esi), v); }
  pc = 0x101721b; continue;
 }
 0x101720b => {
  { let v: u8 = 0x1u8; cpu.w8(0x96u32.wrapping_add(cpu.esi), v); }
  pc = 0x101721b; continue;
 }
 0x1017214 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x15u32.wrapping_add(cpu.edi))); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x15u32.wrapping_add(cpu.edi)), 0x7u8); cpu.w8(0x15u32.wrapping_add(cpu.edi), v); }
  pc = 0x101721b; continue;
 }
 0x101721b => {
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  cpu.push(0x101723f); f_0106c8df(cpu); if cpu.returned_to != 0x101723f { return; }
  cpu.push(0x1017244); f_0106c057(cpu); if cpu.returned_to != 0x1017244 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10fcc16u32, v); }
  { let v: u32 = 0x10e4978u32; cpu.w32(0x10e4010u32, v); }
  { let v: u32 = 0x10e55fau32; cpu.w32(0x10e401cu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1017263 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3f60u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3f64u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x10e3d07u32), 0x0u8); }
  if !cpu.zf { pc = 0x1017292; continue; }
  pc = 0x1017282; continue;
 }
 0x1017282 => {
  { cpu.sub8(cpu.r8(0x10ee91bu32), 0x0u8); }
  if !cpu.zf { pc = 0x1017292; continue; }
  pc = 0x101728b; continue;
 }
 0x101728b => {
  { let v: u8 = cpu.dec8(cpu.r8(0x39u32.wrapping_add(cpu.esi))); cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x39u32.wrapping_add(cpu.esi)) & 0x7u8; cpu.w8(0x39u32.wrapping_add(cpu.esi), v); }
  pc = 0x1017292; continue;
 }
 0x1017292 => {
  { let v: u8 = 0x2u8; cpu.w8(0x79u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e97u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x67u32.wrapping_add(cpu.ebp)) | 0x4u8; cpu.w8(0x67u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x110ed84u32; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x73u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x4u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x81u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x5u8); }
  if cpu.zf { pc = 0x10172fc; continue; }
  pc = 0x10172d5; continue;
 }
 0x10172d5 => {
  { let v: u8 = 0x32u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x81u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x15u8); }
  if cpu.zf { pc = 0x10172fc; continue; }
  pc = 0x10172e8; continue;
 }
 0x10172e8 => {
  { let v: u8 = 0x33u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x16u8); }
  if cpu.zf { pc = 0x10172fc; continue; }
  pc = 0x10172f2; continue;
 }
 0x10172f2 => {
  { let v: u8 = 0x34u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x30u32.wrapping_add(cpu.ebp)), 0x17u8); }
  if cpu.zf { pc = 0x10172fc; continue; }
  pc = 0x10172fc; continue;
 }
 0x10172fc => {
  { let v: u8 = 0x1u8; cpu.w8(0x110ed8au32, v); }
  cpu.push(0x1017308); f_0106b8cb(cpu); if cpu.returned_to != 0x1017308 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x110ed8au32, v); }
  cpu.push(0x1017314); f_0106c057(cpu); if cpu.returned_to != 0x1017314 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x10fcc16u32, v); }
  { let v: u32 = 0x10e4978u32; cpu.w32(0x10e4010u32, v); }
  { let v: u32 = 0x10e55fau32; cpu.w32(0x10e401cu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017333(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017333); }
 let mut pc: u32 = 0x1017333;
 loop { match pc {
 0x1017333 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x101733d; continue; }
  pc = 0x101733c; continue;
 }
 0x101733c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x101733d => {
  { cpu.sub32(cpu.r32(0x110eab8u32), 0x0u32); }
  if cpu.zf { pc = 0x10174a5; continue; }
  pc = 0x101734a; continue;
 }
 0x101734a => {
  { cpu.sub8(cpu.r8(0x110d63au32), 0x0u8); }
  if !cpu.zf { pc = 0x10174a5; continue; }
  pc = 0x1017357; continue;
 }
 0x1017357 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if !cpu.zf { pc = 0x10174a5; continue; }
  pc = 0x1017364; continue;
 }
 0x1017364 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd43du32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x10174a5; continue; }
  pc = 0x1017378; continue;
 }
 0x1017378 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x110eab8u32)); cpu.w32(0x110eab8u32, v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x110eab8u32), 0xaau32); }
  if (cpu.sf != cpu.of) { pc = 0x1017399; continue; }
  pc = 0x101738c; continue;
 }
 0x101738c => {
  { cpu.and8(cpu.r8(0x10e3d07u32), 0x2u8); }
  if !cpu.zf { pc = 0x10174a5; continue; }
  pc = 0x1017399; continue;
 }
 0x1017399 => {
  { let v: u16 = 0x56u16; cpu.w16(0x110ed8bu32, v); }
  { let v: u16 = 0xd6u16; cpu.w16(0x110ed8du32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x110ed8fu32, v); }
  { let v: u16 = 0x16u16; cpu.w16(0x110ed91u32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x10173ea; continue; }
  pc = 0x10173c6; continue;
 }
 0x10173c6 => {
  { let v: u16 = 0xf6u16; cpu.w16(0x110ed8bu32, v); }
  { let v: u16 = 0x172u16; cpu.w16(0x110ed8du32, v); }
  { let v: u16 = 0x8u16; cpu.w16(0x110ed8fu32, v); }
  { let v: u16 = 0x16u16; cpu.w16(0x110ed91u32, v); }
  pc = 0x10173ea; continue;
 }
 0x10173ea => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x1017411; continue; }
  pc = 0x1017404; continue;
 }
 0x1017404 => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1017411; continue;
 }
 0x1017411 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x110ed8bu32)); }
  if (cpu.sf != cpu.of) { pc = 0x101743a; continue; }
  pc = 0x101741a; continue;
 }
 0x101741a => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x110ed8du32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x101743a; continue; }
  pc = 0x1017423; continue;
 }
 0x1017423 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x110ed8fu32)); }
  if (cpu.sf != cpu.of) { pc = 0x101743a; continue; }
  pc = 0x101742c; continue;
 }
 0x101742c => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x110ed91u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x101743a; continue; }
  pc = 0x1017435; continue;
 }
 0x1017435 => {
  { let v: u32 = 0x8u32; cpu.ebp = v; }
  pc = 0x101743a; continue;
 }
 0x101743a => {
  { let v: u32 = cpu.r32(0x110eabcu32); cpu.esi = v; }
  { let v: u32 = 0x12c7u32; cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0x110edaau32); }
  if cpu.zf { pc = 0x101746c; continue; }
  pc = 0x101744d; continue;
 }
 0x101744d => {
  { let v: u32 = 0x12c9u32; cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0x110edaeu32); }
  if cpu.zf { pc = 0x101746c; continue; }
  pc = 0x101745a; continue;
 }
 0x101745a => {
  { let v: u32 = 0x12cdu32; cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0x110edb7u32); }
  if cpu.zf { pc = 0x101746c; continue; }
  pc = 0x1017467; continue;
 }
 0x1017467 => {
  { let v: u32 = 0x12cbu32; cpu.ecx = v; }
  pc = 0x101746c; continue;
 }
 0x101746c => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x110ed8bu32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x110ed8fu32) as u32; cpu.ebx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10ee950u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x10e44dcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x1017492); cpu.call(t); if cpu.returned_to != 0x1017492 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x101749a); cpu.call(t); if cpu.returned_to != 0x101749a { return; }
  { cpu.popad(); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x78u32); cpu.eax = v; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10174a5); cpu.call(t); if cpu.returned_to != 0x10174a5 { return; }
  pc = 0x10174a5; continue;
 }
 0x10174a5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010174a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10174a6); }
 let mut pc: u32 = 0x10174a6;
 loop { match pc {
 0x10174a6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x10ee91du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10fd43du32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x10175b6; continue; }
  pc = 0x10174ba; continue;
 }
 0x10174ba => {
  cpu.push(0x10174bf); f_010176af(cpu); if cpu.returned_to != 0x10174bf { return; }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x11d5104u32, v); }
  { let v: u32 = 0x8u32; cpu.w32(0x11d5100u32, v); }
  { let v: u32 = 0x8u32; cpu.w32(0x11056aau32, v); }
  { let v: u32 = 0x8u32; cpu.w32(0x11056aeu32, v); }
  { let v: u32 = 0xc8u32; cpu.w32(0x11056b2u32, v); }
  { cpu.sub8(cpu.r8(0x10ef352u32), 0x0u8); }
  if cpu.zf { pc = 0x101750e; continue; }
  pc = 0x1017504; continue;
 }
 0x1017504 => {
  { let v: u32 = cpu.r32(0x10e412cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11056b2u32, v); }
  pc = 0x101750e; continue;
 }
 0x101750e => {
  { let v: u32 = cpu.r32(0x11056b2u32).wrapping_sub(0x20u32); cpu.w32(0x11056b2u32, v); }
  { let v: u32 = 0x11d5108u32; cpu.edi = v; }
  pc = 0x101751a; continue;
 }
 0x101751a => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x10175ac; continue; }
  pc = 0x1017525; continue;
 }
 0x1017525 => {
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e93u32, v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x10ee91du32)); }
  if !cpu.zf { pc = 0x10175ac; continue; }
  pc = 0x1017542; continue;
 }
 0x1017542 => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10175ac; continue; }
  pc = 0x101754c; continue;
 }
 0x101754c => {
  cpu.push(0x1017551); f_01017603(cpu); if cpu.returned_to != 0x1017551 { return; }
  { cpu.sub8(cpu.r8(0x4cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1017578; continue; }
  pc = 0x1017557; continue;
 }
 0x1017557 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3e93u32)); cpu.w16(0x10e3e93u32, v); }
  { cpu.sub8(cpu.r8(0x4du32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1017578; continue; }
  pc = 0x1017564; continue;
 }
 0x1017564 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3e93u32)); cpu.w16(0x10e3e93u32, v); }
  { cpu.sub8(cpu.r8(0x4eu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1017578; continue; }
  pc = 0x1017571; continue;
 }
 0x1017571 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3e93u32)); cpu.w16(0x10e3e93u32, v); }
  pc = 0x1017578; continue;
 }
 0x1017578 => {
  cpu.push(0x101757d); f_010175b7(cpu); if cpu.returned_to != 0x101757d { return; }
  { let v: u32 = cpu.r32(0x11d5100u32).wrapping_add(0x10u32); cpu.w32(0x11d5100u32, v); }
  { let v: u32 = cpu.r32(0x11056b2u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x11d5100u32), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x10175a2; continue; }
  pc = 0x1017591; continue;
 }
 0x1017591 => {
  { let v: u32 = cpu.r32(0x11056aeu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d5100u32, v); }
  { let v: u32 = cpu.r32(0x11056aau32).wrapping_add(0x20u32); cpu.w32(0x11056aau32, v); }
  pc = 0x10175a2; continue;
 }
 0x10175a2 => {
  { let v: u32 = cpu.r32(0x11056aau32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d5104u32, v); }
  pc = 0x10175ac; continue;
 }
 0x10175ac => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x101751a; continue; }
  pc = 0x10175b6; continue;
 }
 0x10175b6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010175b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10175b7); }
 let mut pc: u32 = 0x10175b7;
 loop { match pc {
 0x10175b7 => {
  { cpu.sub16(cpu.r16(0x10e3e93u32), 0x0u16); }
  if !cpu.zf { pc = 0x10175c2; continue; }
  pc = 0x10175c1; continue;
 }
 0x10175c1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10175c2 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x11d5104u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d5100u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x3u32); cpu.ebx = v; }
  { let v: u16 = 0x12aau16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x10175de); cpu.call(t); if cpu.returned_to != 0x10175de { return; }
  { let v: u32 = cpu.r32(0x11d5104u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d5100u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x9u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x3u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0x1283u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1017601); cpu.call(t); if cpu.returned_to != 0x1017601 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017603(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017603); }
 let mut pc: u32 = 0x1017603;
 loop { match pc {
 0x1017603 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x11d5104u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d5100u32); cpu.ebx = v; }
  { let v: u16 = 0x12d7u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x10e42ecu32); cpu.push(0x1017628); cpu.call(t); if cpu.returned_to != 0x1017628 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x11d5104u32), 0x10u32); cpu.w32(0x11d5104u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017635(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017635); }
 let mut pc: u32 = 0x1017635;
 loop { match pc {
 0x1017635 => {
  { cpu.sub32(cpu.r32(0x10e3f68u32), 0x0u32); }
  if cpu.zf { pc = 0x101763f; continue; }
  pc = 0x101763e; continue;
 }
 0x101763e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x101763f => {
  { cpu.sub16(cpu.r16(0x10e3ce1u32), 0x0u16); }
  if !cpu.zf { pc = 0x10176ad; continue; }
  pc = 0x101764d; continue;
 }
 0x101764d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x10e3cdbu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3cddu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee97bu32), 0x0u8); }
  if cpu.zf { pc = 0x1017674; continue; }
  pc = 0x1017667; continue;
 }
 0x1017667 => {
  { let v: u16 = cpu.r16(0x10e3d51u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d53u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1017674; continue;
 }
 0x1017674 => {
  { let v: u32 = 0x11d5108u32; cpu.esi = v; }
  { let v: u32 = 0x8cu32; cpu.ecx = v; }
  pc = 0x101767e; continue;
 }
 0x101767e => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x10176ad; continue; }
  pc = 0x1017686; continue;
 }
 0x1017686 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.cf { pc = 0x10176a7; continue; }
  pc = 0x101768a; continue;
 }
 0x101768a => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if cpu.cf { pc = 0x10176a7; continue; }
  pc = 0x101768f; continue;
 }
 0x101768f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10176a7; continue; }
  pc = 0x1017698; continue;
 }
 0x1017698 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10176a7; continue; }
  pc = 0x10176a2; continue;
 }
 0x10176a2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10176a7 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x101767e; continue; }
  pc = 0x10176ad; continue;
 }
 0x10176ad => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010176af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10176af); }
 let mut pc: u32 = 0x10176af;
 loop { match pc {
 0x10176af => {
  { let v: u32 = 0x11d5108u32; cpu.edi = v; }
  { let v: u32 = 0x1a4u32; cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010176bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10176bf); }
 let mut pc: u32 = 0x10176bf;
 loop { match pc {
 0x10176bf => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x10176ec; continue; }
  pc = 0x10176cf; continue;
 }
 0x10176cf => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x10ee934u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x10176e0); f_01017985(cpu); if cpu.returned_to != 0x10176e0 { return; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x116e730u32)); }
  if cpu.cf { pc = 0x1017708; continue; }
  pc = 0x10176ea; continue;
 }
 0x10176ea => {
  pc = 0x1017701; continue;
 }
 0x10176ec => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10176f0; continue;
 }
 0x10176f0 => {
  cpu.push(0x10176f5); f_01017731(cpu); if cpu.returned_to != 0x10176f5 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10176f0; continue; }
  pc = 0x10176fa; continue;
 }
 0x10176fa => {
  { cpu.sub8((cpu.edx as u8), 0x63u8); }
  if cpu.cf { pc = 0x1017708; continue; }
  pc = 0x10176ff; continue;
 }
 0x10176ff => {
  pc = 0x1017701; continue;
 }
 0x1017701 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1017708 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  pc = 0x1017714; continue;
 }
 0x1017714 => {
  { cpu.and8(cpu.r8(0x31u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x1017720; continue; }
  pc = 0x101771a; continue;
 }
 0x101771a => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7u32); }
  if cpu.zf { pc = 0x101772a; continue; }
  pc = 0x1017720; continue;
 }
 0x1017720 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xb8u32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1017714; continue; }
  pc = 0x1017728; continue;
 }
 0x1017728 => {
  cpu.poll(); pc = 0x1017701; continue;
 }
 0x101772a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017731(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017731); }
 let mut pc: u32 = 0x1017731;
 loop { match pc {
 0x1017731 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x10ee934u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x101773d); f_01017985(cpu); if cpu.returned_to != 0x101773d { return; }
  { let v: u8 = cpu.add8((cpu.edx as u8), (cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017741(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1017741); } r_01017741(cpu, 0x1017741); }
pub fn f_01017748(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1017748); } r_01017741(cpu, 0x1017748); }
fn r_01017741(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1017741 => {
  cpu.push(0x1017746); f_01017922(cpu); if cpu.returned_to != 0x1017746 { return; }
  if !cpu.cf { pc = 0x101776d; continue; }
  pc = 0x1017748; continue;
 }
 0x1017748 => {
  { cpu.pushad(); }
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xffu8; cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x101775f); f_01017816(cpu); if cpu.returned_to != 0x101775f { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf != cpu.of) { pc = 0x101776c; continue; }
  pc = 0x1017769; continue;
 }
 0x1017769 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x101776c => {
  { cpu.popad(); }
  pc = 0x101776d; continue;
 }
 0x101776d => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0101776f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101776f); }
 let mut pc: u32 = 0x101776f;
 loop { match pc {
 0x101776f => {
  cpu.push(0x1017774); f_010176bf(cpu); if cpu.returned_to != 0x1017774 { return; }
  if cpu.cf { pc = 0x1017778; continue; }
  pc = 0x1017776; continue;
 }
 0x1017776 => {
  f_01017741(cpu); return;
 }
 0x1017778 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017779(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017779); }
 let mut pc: u32 = 0x1017779;
 loop { match pc {
 0x1017779 => {
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1017785; continue;
 }
 0x1017785 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x10177b8; continue; }
  pc = 0x101778d; continue;
 }
 0x101778d => {
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10177b8; continue; }
  pc = 0x1017794; continue;
 }
 0x1017794 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x10177b8; continue; }
  pc = 0x101779a; continue;
 }
 0x101779a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x10177ac); f_0108c1c5(cpu); if cpu.returned_to != 0x10177ac { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x10177b8; continue; }
  pc = 0x10177b7; continue;
 }
 0x10177b7 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10177b8; continue;
 }
 0x10177b8 => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1017785; continue; }
  pc = 0x10177c1; continue;
 }
 0x10177c1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3e93u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x10177d7; continue;
 }
 0x10177d7 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x101780a; continue; }
  pc = 0x10177dd; continue;
 }
 0x10177dd => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x101780a; continue; }
  pc = 0x10177e5; continue;
 }
 0x10177e5 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x101780a; continue; }
  pc = 0x10177ec; continue;
 }
 0x10177ec => {
  { let v: u8 = cpu.r8(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x101780a; continue; }
  pc = 0x10177f6; continue;
 }
 0x10177f6 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x101780a; continue; }
  pc = 0x10177fd; continue;
 }
 0x10177fd => {
  cpu.push(0x1017802); f_01017922(cpu); if cpu.returned_to != 0x1017802 { return; }
  if !cpu.cf { pc = 0x101780a; continue; }
  pc = 0x1017804; continue;
 }
 0x1017804 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10e4138u32)); cpu.w32(0x10e4138u32, v); }
  pc = 0x101780a; continue;
 }
 0x101780a => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10177d7; continue; }
  pc = 0x1017810; continue;
 }
 0x1017810 => {
  { let v: u32 = cpu.r32(0x10e4138u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017816(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017816); }
 let mut pc: u32 = 0x1017816;
 loop { match pc {
 0x1017816 => {
  { let v: u32 = 0x1152e54u32; cpu.ebp = v; }
  { let v: u32 = 0xf0u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x1017822; continue;
 }
 0x1017822 => {
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x1017855; continue; }
  pc = 0x101782a; continue;
 }
 0x101782a => {
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x1017855; continue; }
  pc = 0x1017831; continue;
 }
 0x1017831 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if !cpu.zf { pc = 0x1017855; continue; }
  pc = 0x1017837; continue;
 }
 0x1017837 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1017849); f_0108c1c5(cpu); if cpu.returned_to != 0x1017849 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if !cpu.zf { pc = 0x1017855; continue; }
  pc = 0x1017854; continue;
 }
 0x1017854 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x1017855; continue;
 }
 0x1017855 => {
  { let v: u32 = cpu.ebp.wrapping_add(0xb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1017822; continue; }
  pc = 0x101785e; continue;
 }
 0x101785e => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10e3e93u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e4138u32, v); }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x1017874; continue;
 }
 0x1017874 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10178e9; continue; }
  pc = 0x101787e; continue;
 }
 0x101787e => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x10178e9; continue; }
  pc = 0x101788a; continue;
 }
 0x101788a => {
  { cpu.sub32(cpu.r32(0x110edc1u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x101789e; continue; }
  pc = 0x1017896; continue;
 }
 0x1017896 => {
  { cpu.sub32(cpu.r32(0x110edc1u32), cpu.ebp); }
  if cpu.zf { pc = 0x10178e9; continue; }
  pc = 0x101789e; continue;
 }
 0x101789e => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x10178e9; continue; }
  pc = 0x10178a5; continue;
 }
 0x10178a5 => {
  { let v: u8 = cpu.r8(0x7u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x10178b9; continue; }
  pc = 0x10178ac; continue;
 }
 0x10178ac => {
  cpu.push(0x10178b1); f_01017922(cpu); if cpu.returned_to != 0x10178b1 { return; }
  if !cpu.cf { pc = 0x10178b9; continue; }
  pc = 0x10178b3; continue;
 }
 0x10178b3 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10e4138u32)); cpu.w32(0x10e4138u32, v); }
  pc = 0x10178b9; continue;
 }
 0x10178b9 => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10178c5; continue; }
  pc = 0x10178bf; continue;
 }
 0x10178bf => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10e4138u32)); cpu.w32(0x10e4138u32, v); }
  pc = 0x10178c5; continue;
 }
 0x10178c5 => {
  { cpu.sub8(cpu.r8(0x4cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10178d1; continue; }
  pc = 0x10178cb; continue;
 }
 0x10178cb => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10e4138u32)); cpu.w32(0x10e4138u32, v); }
  pc = 0x10178d1; continue;
 }
 0x10178d1 => {
  { cpu.sub8(cpu.r8(0x4du32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10178dd; continue; }
  pc = 0x10178d7; continue;
 }
 0x10178d7 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10e4138u32)); cpu.w32(0x10e4138u32, v); }
  pc = 0x10178dd; continue;
 }
 0x10178dd => {
  { cpu.sub8(cpu.r8(0x4eu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10178e9; continue; }
  pc = 0x10178e3; continue;
 }
 0x10178e3 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10e4138u32)); cpu.w32(0x10e4138u32, v); }
  pc = 0x10178e9; continue;
 }
 0x10178e9 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1017874; continue; }
  pc = 0x10178ef; continue;
 }
 0x10178ef => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x5u32; cpu.ecx = v; }
  pc = 0x10178f9; continue;
 }
 0x10178f9 => {
  { cpu.sub8(cpu.r8(0x10ef37du32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.sf { pc = 0x1017913; continue; }
  pc = 0x1017902; continue;
 }
 0x1017902 => {
  { let v: u32 = cpu.r32(0x110eac4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x1017913; continue; }
  pc = 0x101790d; continue;
 }
 0x101790d => {
  { let v: u32 = cpu.inc32(cpu.r32(0x10e4138u32)); cpu.w32(0x10e4138u32, v); }
  pc = 0x1017913; continue;
 }
 0x1017913 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10178f9; continue; }
  pc = 0x1017916; continue;
 }
 0x1017916 => {
  cpu.push(0x101791b); f_010393d8(cpu); if cpu.returned_to != 0x101791b { return; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x10e4138u32)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017922(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017922); }
 let mut pc: u32 = 0x1017922;
 loop { match pc {
 0x1017922 => {
  { cpu.pushad(); }
  { cpu.sub8((cpu.eax as u8), 0xffu8); }
  if cpu.zf { pc = 0x101797f; continue; }
  pc = 0x1017927; continue;
 }
 0x1017927 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x52u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x4650u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0x2cu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10e413cu32, v); }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.ecx as u8); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x2au32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x22u32.wrapping_add(cpu.ebx)); cpu.mul8(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10e3f94u32)); cpu.eax = v; }
  { let v: u32 = 0x55f0u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x1017982; continue; }
  pc = 0x1017973; continue;
 }
 0x1017973 => {
  { cpu.sub8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x1017982; continue; }
  pc = 0x1017977; continue;
 }
 0x1017977 => {
  { cpu.sub8((cpu.eax as u8), 0x5u8); }
  if cpu.zf { pc = 0x1017982; continue; }
  pc = 0x101797b; continue;
 }
 0x101797b => {
  { cpu.sub8((cpu.eax as u8), 0x6u8); }
  if cpu.zf { pc = 0x1017982; continue; }
  pc = 0x101797f; continue;
 }
 0x101797f => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1017982 => {
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017985(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017985); }
 let mut pc: u32 = 0x1017985;
 loop { match pc {
 0x1017985 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x1017998; continue;
 }
 0x1017998 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10179ca; continue; }
  pc = 0x101799e; continue;
 }
 0x101799e => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x10179ca; continue; }
  pc = 0x10179a6; continue;
 }
 0x10179a6 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x8u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x10179ca; continue; }
  pc = 0x10179ae; continue;
 }
 0x10179ae => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10179b5; continue; }
  pc = 0x10179b4; continue;
 }
 0x10179b4 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10179b5; continue;
 }
 0x10179b5 => {
  { cpu.sub8(cpu.r8(0x4cu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10179bc; continue; }
  pc = 0x10179bb; continue;
 }
 0x10179bb => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10179bc; continue;
 }
 0x10179bc => {
  { cpu.sub8(cpu.r8(0x4du32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10179c3; continue; }
  pc = 0x10179c2; continue;
 }
 0x10179c2 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10179c3; continue;
 }
 0x10179c3 => {
  { cpu.sub8(cpu.r8(0x4eu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x10179ca; continue; }
  pc = 0x10179c9; continue;
 }
 0x10179c9 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10179ca; continue;
 }
 0x10179ca => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1017998; continue; }
  pc = 0x10179d0; continue;
 }
 0x10179d0 => {
  { cpu.sub8(cpu.r8(0x10ef37du32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.sf { pc = 0x10179da; continue; }
  pc = 0x10179d9; continue;
 }
 0x10179d9 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10179da; continue;
 }
 0x10179da => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010179e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10179e0); }
 let mut pc: u32 = 0x10179e0;
 loop { match pc {
 0x10179e0 => {
  { cpu.pushad(); }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10179ee); f_01017985(cpu); if cpu.returned_to != 0x10179ee { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x8u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10179fa); f_01017985(cpu); if cpu.returned_to != 0x10179fa { return; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = cpu.sub8((cpu.ecx as u8), (cpu.edx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1017a8a; continue; }
  pc = 0x1017a07; continue;
 }
 0x1017a07 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x10ee934u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if !cpu.zf { pc = 0x1017a20; continue; }
  pc = 0x1017a16; continue;
 }
 0x1017a16 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x116e730u32)); }
  if cpu.cf { pc = 0x1017a2b; continue; }
  pc = 0x1017a1e; continue;
 }
 0x1017a1e => {
  pc = 0x1017a27; continue;
 }
 0x1017a20 => {
  { cpu.sub8((cpu.edx as u8), 0x63u8); }
  if cpu.cf { pc = 0x1017a2b; continue; }
  pc = 0x1017a25; continue;
 }
 0x1017a25 => {
  pc = 0x1017a27; continue;
 }
 0x1017a27 => {
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1017a44; continue;
 }
 0x1017a2b => {
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ecx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x116e730u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x10ee91eu32), 0x0u8); }
  if cpu.zf { pc = 0x1017a3e; continue; }
  pc = 0x1017a3c; continue;
 }
 0x1017a3c => {
  { let v: u8 = 0x63u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1017a3e; continue;
 }
 0x1017a3e => {
  { let v: u8 = cpu.sub8((cpu.edx as u8), (cpu.ecx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.sf { pc = 0x1017a8a; continue; }
  pc = 0x1017a42; continue;
 }
 0x1017a42 => {
  if cpu.zf { pc = 0x1017a8a; continue; }
  pc = 0x1017a44; continue;
 }
 0x1017a44 => {
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x1017a4e; continue;
 }
 0x1017a4e => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1017a84; continue; }
  pc = 0x1017a54; continue;
 }
 0x1017a54 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1017a84; continue; }
  pc = 0x1017a5c; continue;
 }
 0x1017a5c => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x1017a84; continue; }
  pc = 0x1017a63; continue;
 }
 0x1017a63 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x8u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x1017a84; continue; }
  pc = 0x1017a68; continue;
 }
 0x1017a68 => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1017a84; continue; }
  pc = 0x1017a6e; continue;
 }
 0x1017a6e => {
  { let v: u16 = cpu.r16(0x4cu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x4bu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x4eu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4du32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xffu8; cpu.w8(0x4eu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x1017a8a; continue; }
  pc = 0x1017a84; continue;
 }
 0x1017a84 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1017a4e; continue; }
  pc = 0x1017a8a; continue;
 }
 0x1017a8a => {
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x1017a94; continue;
 }
 0x1017a94 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1017b04; continue; }
  pc = 0x1017a9e; continue;
 }
 0x1017a9e => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1017b04; continue; }
  pc = 0x1017aaa; continue;
 }
 0x1017aaa => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x1017b04; continue; }
  pc = 0x1017ab5; continue;
 }
 0x1017ab5 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x8u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x1017b04; continue; }
  pc = 0x1017aba; continue;
 }
 0x1017aba => {
  { cpu.sub8(cpu.r8(0x4bu32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1017b04; continue; }
  pc = 0x1017ac0; continue;
 }
 0x1017ac0 => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if !cpu.zf { pc = 0x1017b04; continue; }
  pc = 0x1017ac6; continue;
 }
 0x1017ac6 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x1017aef; continue; }
  pc = 0x1017acc; continue;
 }
 0x1017acc => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1017ad8; continue; }
  pc = 0x1017ad2; continue;
 }
 0x1017ad2 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x1017b04; continue; }
  pc = 0x1017ad8; continue;
 }
 0x1017ad8 => {
  { let v: u16 = cpu.r16(0x10e3d79u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1017ae6); f_01031710(cpu); if cpu.returned_to != 0x1017ae6 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x2au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1017b04; continue;
 }
 0x1017aef => {
  { let v: u16 = cpu.r16(0x10e3d7bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1017afd); f_010316a1(cpu); if cpu.returned_to != 0x1017afd { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x2au32.wrapping_add(cpu.ebp), v); }
  pc = 0x1017b04; continue;
 }
 0x1017b04 => {
  { let v: u32 = cpu.add32(cpu.ebp, 0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1017a94; continue; }
  pc = 0x1017b0e; continue;
 }
 0x1017b0e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017b10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017b10); }
 let mut pc: u32 = 0x1017b10;
 loop { match pc {
 0x1017b10 => {
  { let v: u8 = 0x4u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1017b19); f_0106674c(cpu); if cpu.returned_to != 0x1017b19 { return; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x1017b23; continue; }
  pc = 0x1017b1f; continue;
 }
 0x1017b1f => {
  { let v: u8 = 0x32u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1017b23; continue;
 }
 0x1017b23 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017b24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017b24); }
 let mut pc: u32 = 0x1017b24;
 loop { match pc {
 0x1017b24 => {
  { let v: u8 = 0x4u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1017b2d); f_0106674c(cpu); if cpu.returned_to != 0x1017b2d { return; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x1017b4e; continue; }
  pc = 0x1017b33; continue;
 }
 0x1017b33 => {
  { let v: u8 = 0x33u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub(cpu.r8(0x20u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1017b4a; continue; }
  pc = 0x1017b44; continue;
 }
 0x1017b44 => {
  { let v: u8 = 0x1u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1017b4e; continue;
 }
 0x1017b4a => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1017b4e; continue;
 }
 0x1017b4e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017b4f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017b4f); }
 let mut pc: u32 = 0x1017b4f;
 loop { match pc {
 0x1017b4f => {
  { let v: u8 = 0x4u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x1017b58); f_0106674c(cpu); if cpu.returned_to != 0x1017b58 { return; }
  { cpu.sub8(cpu.r8(0x35u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if !cpu.zf { pc = 0x1017b79; continue; }
  pc = 0x1017b5e; continue;
 }
 0x1017b5e => {
  { let v: u8 = 0x34u8; cpu.w8(0x35u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub(cpu.r8(0x20u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x78u32.wrapping_add(cpu.ebp)), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x1017b75; continue; }
  pc = 0x1017b6f; continue;
 }
 0x1017b6f => {
  { let v: u8 = 0x1u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1017b79; continue;
 }
 0x1017b75 => {
  { let v: u8 = 0x0u8; cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1017b79; continue;
 }
 0x1017b79 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017b7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017b7c); }
 let mut pc: u32 = 0x1017b7c;
 loop { match pc {
 0x1017b7c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d5880u32, v); }
  pc = 0x1017b8c; continue;
 }
 0x1017b8c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if cpu.zf { pc = 0x1017bc2; continue; }
  pc = 0x1017baa; continue;
 }
 0x1017baa => {
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if cpu.zf { pc = 0x1017bc2; continue; }
  pc = 0x1017bb2; continue;
 }
 0x1017bb2 => {
  cpu.push(0x1017bb7); f_01017bce(cpu); if cpu.returned_to != 0x1017bb7 { return; }
  { let v: u32 = cpu.r32(0x11d5880u32).wrapping_add(cpu.eax); cpu.w32(0x11d5880u32, v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x1017b8c; continue;
 }
 0x1017bc2 => {
  { let v: u32 = cpu.r32(0x11d5880u32); cpu.eax = v; }
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
pub fn f_01017bce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017bce); }
 let mut pc: u32 = 0x1017bce;
 loop { match pc {
 0x1017bce => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e8bu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3e91u32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e87u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e89u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e8du32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e8fu32, v); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.sf { pc = 0x1017ccc; continue; }
  pc = 0x1017c0a; continue;
 }
 0x1017c0a => {
  { cpu.and16((cpu.ebx as u16), (cpu.ebx as u16)); }
  if cpu.sf { pc = 0x1017ccc; continue; }
  pc = 0x1017c13; continue;
 }
 0x1017c13 => {
  { cpu.and16((cpu.ecx as u16), (cpu.ecx as u16)); }
  if cpu.sf { pc = 0x1017ccc; continue; }
  pc = 0x1017c1c; continue;
 }
 0x1017c1c => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.sf { pc = 0x1017ccc; continue; }
  pc = 0x1017c25; continue;
 }
 0x1017c25 => {
  cpu.push(0x1017c2a); f_01017cd9(cpu); if cpu.returned_to != 0x1017c2a { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u32 = cpu.r16(0x10e3e9fu32) as u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x1017c53; continue; }
  pc = 0x1017c49; continue;
 }
 0x1017c49 => {
  { cpu.cf = false; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1017c53 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r16(0x10e3e87u32) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x10e3e89u32) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10ea910u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x7fu32; cpu.ecx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x34u8); }
  if cpu.zf { pc = 0x1017c7a; continue; }
  pc = 0x1017c75; continue;
 }
 0x1017c75 => {
  { cpu.sub8((cpu.ecx as u8), 0x28u8); }
  if !cpu.zf { pc = 0x1017c80; continue; }
  pc = 0x1017c7a; continue;
 }
 0x1017c7a => {
  { let v: u8 = cpu.inc8(cpu.r8(0x11d5885u32)); cpu.w8(0x11d5885u32, v); }
  pc = 0x1017c80; continue;
 }
 0x1017c80 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x10e41e8u32)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as u32; cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x11c5bb5u32), 0x0u8); }
  if !cpu.zf { pc = 0x1017c92; continue; }
  pc = 0x1017c92; continue;
 }
 0x1017c92 => {
  { cpu.sub8((cpu.ecx as u8), 0xau8); }
  if cpu.cf { pc = 0x1017cb1; continue; }
  pc = 0x1017c97; continue;
 }
 0x1017c97 => {
  { cpu.sub8(cpu.r8(0x11d5884u32), 0x0u8); }
  if cpu.zf { pc = 0x1017ca2; continue; }
  pc = 0x1017ca0; continue;
 }
 0x1017ca0 => {
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x1017ca2; continue;
 }
 0x1017ca2 => {
  { let v: u8 = 0x1u8; cpu.w8(0x11d5884u32, v); }
  { let v: u32 = cpu.r32(0x11d587cu32).wrapping_add(cpu.ecx); cpu.w32(0x11d587cu32, v); }
  pc = 0x1017cbe; continue;
 }
 0x1017cb1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x11d5884u32, v); }
  { let v: u32 = cpu.r32(0x11d587cu32).wrapping_add(cpu.ecx); cpu.w32(0x11d587cu32, v); }
  pc = 0x1017cbe; continue;
 }
 0x1017cbe => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  let t = cpu.r32(0x10e4070u32); cpu.push(0x1017cc5); cpu.call(t); if cpu.returned_to != 0x1017cc5 { return; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1017c53; continue; }
  pc = 0x1017cc8; continue;
 }
 0x1017cc8 => {
  { cpu.cf = false; }
  pc = 0x1017ccd; continue;
 }
 0x1017ccc => {
  { cpu.cf = true; }
  pc = 0x1017ccd; continue;
 }
 0x1017ccd => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11d587cu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017cd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017cd9); }
 let mut pc: u32 = 0x1017cd9;
 loop { match pc {
 0x1017cd9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x11d587cu32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x11d5884u32, v); }
  { cpu.sub16(cpu.r16(0x10e3e8bu32), 0x0u16); }
  if cpu.zf { pc = 0x1017cfb; continue; }
  pc = 0x1017cf4; continue;
 }
 0x1017cf4 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3e8bu32)); cpu.w16(0x10e3e8bu32, v); }
  pc = 0x1017cfb; continue;
 }
 0x1017cfb => {
  { cpu.sub16(cpu.r16(0x10e3e91u32), 0x0u16); }
  if cpu.zf { pc = 0x1017d0c; continue; }
  pc = 0x1017d05; continue;
 }
 0x1017d05 => {
  { let v: u16 = cpu.inc16(cpu.r16(0x10e3e91u32)); cpu.w16(0x10e3e91u32, v); }
  pc = 0x1017d0c; continue;
 }
 0x1017d0c => {
  { let v: u16 = cpu.shr16(cpu.r16(0x10e3e87u32), 4u32); cpu.w16(0x10e3e87u32, v); }
  { let v: u16 = cpu.shr16(cpu.r16(0x10e3e89u32), 4u32); cpu.w16(0x10e3e89u32, v); }
  { let v: u16 = cpu.shr16(cpu.r16(0x10e3e8du32), 4u32); cpu.w16(0x10e3e8du32, v); }
  { let v: u16 = cpu.shr16(cpu.r16(0x10e3e8fu32), 4u32); cpu.w16(0x10e3e8fu32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3ea1u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3ea3u32, v); }
  { let v: u16 = 0x1u16; cpu.w16(0x10e3ea5u32, v); }
  { let v: u16 = cpu.r16(0x10e3e8du32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e8fu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e91u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e87u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1017d8d; continue; }
  pc = 0x1017d64; continue;
 }
 0x1017d64 => {
  { let a = cpu.r16(0x10e3e87u32); let b = (cpu.eax as u16); { let v: u16 = b; cpu.w16(0x10e3e87u32, v); } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e8du32, v); }
  { let a = cpu.r16(0x10e3e89u32); let b = (cpu.ebx as u16); { let v: u16 = b; cpu.w16(0x10e3e89u32, v); } { let v: u16 = a; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e8fu32, v); }
  { let a = cpu.r16(0x10e3e8bu32); let b = (cpu.ecx as u16); { let v: u16 = b; cpu.w16(0x10e3e8bu32, v); } { let v: u16 = a; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e91u32, v); }
  pc = 0x1017d8d; continue;
 }
 0x1017d8d => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x10e3e87u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3e89u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1017da9; continue; }
  pc = 0x1017d9d; continue;
 }
 0x1017d9d => {
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3ea3u32, v); }
  { let v: u16 = cpu.neg16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1017da9; continue;
 }
 0x1017da9 => {
  { let v: u16 = cpu.sub16((cpu.ecx as u16), cpu.r16(0x10e3e8bu32)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1017dbe; continue; }
  pc = 0x1017db2; continue;
 }
 0x1017db2 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xffffu16; cpu.w16(0x10e3ea5u32, v); }
  pc = 0x1017dbe; continue;
 }
 0x1017dbe => {
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x1017df6; continue; }
  pc = 0x1017dc3; continue;
 }
 0x1017dc3 => {
  { cpu.sub16((cpu.ecx as u16), (cpu.eax as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1017e2a; continue; }
  pc = 0x1017dc8; continue;
 }
 0x1017dc8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e9fu32, v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3ea9u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3eabu32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3ea7u32, v); }
  { let v: u32 = 0x1017e59u32; cpu.w32(0x10e4070u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1017df6 => {
  { cpu.sub16((cpu.ecx as u16), (cpu.ebx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1017e2a; continue; }
  pc = 0x1017dfb; continue;
 }
 0x1017dfb => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e9fu32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3eabu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3ea7u32, v); }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3ea9u32, v); }
  { let v: u32 = 0x1017ed3u32; cpu.w32(0x10e4070u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1017e2a => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e9fu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shr16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x10e3eabu32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3ea7u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3ea9u32, v); }
  { let v: u32 = 0x1017f4du32; cpu.w32(0x10e4070u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017e59(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017e59); }
 let mut pc: u32 = 0x1017e59;
 loop { match pc {
 0x1017e59 => {
  { let v: u16 = cpu.r16(0x10e3e87u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e89u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e8bu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3ea9u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3eabu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3ea1u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.esi as u16), cpu.r16(0x10e3e95u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1017e99; continue; }
  pc = 0x1017e8b; continue;
 }
 0x1017e8b => {
  { let v: u16 = (cpu.esi as u16).wrapping_add(cpu.r16(0x10e3e93u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3ea3u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1017e99; continue;
 }
 0x1017e99 => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), cpu.r16(0x10e3e97u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1017eb0; continue; }
  pc = 0x1017ea2; continue;
 }
 0x1017ea2 => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x10e3e93u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), cpu.r16(0x10e3ea5u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1017eb0; continue;
 }
 0x1017eb0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e87u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e89u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e8bu32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x10e3ea9u32, v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x10e3eabu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017ed3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1017ed3); } r_01017ed3(cpu, 0x1017ed3); }
pub fn f_01017f01(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1017f01); } r_01017ed3(cpu, 0x1017f01); }
fn r_01017ed3(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1017ed3 => {
  { let v: u16 = cpu.r16(0x10e3e87u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e89u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e8bu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3ea7u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3eabu32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x10e3ea3u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.esi as u16), cpu.r16(0x10e3e93u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1017f13; continue; }
  pc = 0x1017f05; continue;
 }
 0x1017f01 => {
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.ecx)).wrapping_add(cpu.edi); cpu.w32(0xeu32.wrapping_add(cpu.ecx), v); }
  pc = 0x1017f05; continue;
 }
 0x1017f05 => {
  { let v: u16 = (cpu.esi as u16).wrapping_add(cpu.r16(0x10e3e95u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3ea1u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1017f13; continue;
 }
 0x1017f13 => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), cpu.r16(0x10e3e97u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1017f2a; continue; }
  pc = 0x1017f1c; continue;
 }
 0x1017f1c => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x10e3e95u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), cpu.r16(0x10e3ea5u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1017f2a; continue;
 }
 0x1017f2a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e87u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e89u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e8bu32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x10e3ea7u32, v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x10e3eabu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01017f4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1017f4d); }
 let mut pc: u32 = 0x1017f4d;
 loop { match pc {
 0x1017f4d => {
  { let v: u16 = cpu.r16(0x10e3e87u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e89u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e8bu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3ea7u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3ea9u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x10e3ea5u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.esi as u16), cpu.r16(0x10e3e93u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1017f8d; continue; }
  pc = 0x1017f7f; continue;
 }
 0x1017f7f => {
  { let v: u16 = (cpu.esi as u16).wrapping_add(cpu.r16(0x10e3e97u32)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(cpu.r16(0x10e3ea1u32)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1017f8d; continue;
 }
 0x1017f8d => {
  { let v: u16 = cpu.sub16((cpu.edi as u16), cpu.r16(0x10e3e95u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x1017fa4; continue; }
  pc = 0x1017f96; continue;
 }
 0x1017f96 => {
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x10e3e97u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), cpu.r16(0x10e3ea3u32)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x1017fa4; continue;
 }
 0x1017fa4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e87u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e89u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10e3e8bu32, v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x10e3ea7u32, v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x10e3ea9u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0101809a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101809a); }
 let mut pc: u32 = 0x101809a;
 loop { match pc {
 0x101809a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e40a8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x33u32; cpu.push(v); }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11d588cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d5888u32); cpu.eax = v; }
  cpu.push(0x10180cd); f_01018334(cpu); if cpu.returned_to != 0x10180cd { return; }
  { let v: u32 = cpu.r32(0x11d589cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x11d5890u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d5898u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x11d5894u32)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x1018103; continue; }
  pc = 0x10180e9; continue;
 }
 0x10180e9 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11d5890u32)); cpu.w32(0x11d5890u32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x11d5894u32)); cpu.w32(0x11d5894u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11d589cu32)); cpu.w32(0x11d589cu32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11d5898u32)); cpu.w32(0x11d5898u32, v); }
  pc = 0x101811b; continue;
 }
 0x1018103 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11d5894u32)); cpu.w32(0x11d5894u32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x11d5890u32)); cpu.w32(0x11d5890u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11d5898u32)); cpu.w32(0x11d5898u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11d589cu32)); cpu.w32(0x11d589cu32, v); }
  pc = 0x101811b; continue;
 }
 0x101811b => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018122; continue;
 }
 0x1018122 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x60u32); }
  if (cpu.sf != cpu.of) { pc = 0x1018132; continue; }
  pc = 0x1018128; continue;
 }
 0x1018128 => {
  pc = 0x1018199; continue;
 }
 0x101812a => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1018122; continue;
 }
 0x1018132 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x0u16); }
  if !cpu.zf { pc = 0x101818d; continue; }
  pc = 0x101813b; continue;
 }
 0x101813b => {
  { let v: u16 = cpu.r16(0x11d5890u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = cpu.r16(0x11d5894u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = cpu.r16(0x11d589cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = cpu.r16(0x11d5898u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1018182); f_010181a9(cpu); if cpu.returned_to != 0x1018182 { return; }
  cpu.push(0x1018187); f_01034285(cpu); if cpu.returned_to != 0x1018187 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x101819d; continue;
 }
 0x101818d => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(0x88u32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x101812a; continue;
 }
 0x1018199 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x101819d; continue;
 }
 0x101819d => {
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_010181a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10181a9); }
 let mut pc: u32 = 0x10181a9;
 loop { match pc {
 0x10181a9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10e6910u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x1018240); f_010188b5(cpu); if cpu.returned_to != 0x1018240 { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.edx, 0x4000u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
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
pub fn f_0101825b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101825b); }
 let mut pc: u32 = 0x101825b;
 loop { match pc {
 0x101825b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e40a8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x17u32; cpu.push(v); }
  { let v: u32 = 0x17u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11d588cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d5888u32); cpu.eax = v; }
  cpu.push(0x101828e); f_01018334(cpu); if cpu.returned_to != 0x101828e { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x11d5890u32)); cpu.w32(0x11d5890u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11d589cu32)); cpu.w32(0x11d589cu32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x11d5894u32)); cpu.w32(0x11d5894u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11d5898u32)); cpu.w32(0x11d5898u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10182ad; continue;
 }
 0x10182ad => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x60u32); }
  if (cpu.sf != cpu.of) { pc = 0x10182bd; continue; }
  pc = 0x10182b3; continue;
 }
 0x10182b3 => {
  pc = 0x1018324; continue;
 }
 0x10182b5 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10182ad; continue;
 }
 0x10182bd => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x0u16); }
  if !cpu.zf { pc = 0x1018318; continue; }
  pc = 0x10182c6; continue;
 }
 0x10182c6 => {
  { let v: u16 = cpu.r16(0x11d5890u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = cpu.r16(0x11d5894u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = cpu.r16(0x11d589cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = cpu.r16(0x11d5898u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x101830d); f_010181a9(cpu); if cpu.returned_to != 0x101830d { return; }
  cpu.push(0x1018312); f_01034285(cpu); if cpu.returned_to != 0x1018312 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018328; continue;
 }
 0x1018318 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(0x88u32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10182b5; continue;
 }
 0x1018324 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018328; continue;
 }
 0x1018328 => {
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_01018334(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018334); }
 let mut pc: u32 = 0x1018334;
 loop { match pc {
 0x1018334 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d5890u32, v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d5894u32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018361; continue;
 }
 0x1018361 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x1018385; continue; }
  pc = 0x1018373; continue;
 }
 0x1018373 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x101839a; continue; }
  pc = 0x1018385; continue;
 }
 0x1018385 => {
  { let v: u32 = cpu.r32(0x11d5894u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d5890u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.push(0x1018396); f_010187dd(cpu); if cpu.returned_to != 0x1018396 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x101839c; continue; }
  pc = 0x101839a; continue;
 }
 0x101839a => {
  pc = 0x10183aa; continue;
 }
 0x101839c => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11d5890u32)); cpu.w32(0x11d5890u32, v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1018361; continue;
 }
 0x10183aa => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10183b0; continue;
 }
 0x10183b0 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffff80u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x10183d4; continue; }
  pc = 0x10183c2; continue;
 }
 0x10183c2 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xffffff80u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x10183e9; continue; }
  pc = 0x10183d4; continue;
 }
 0x10183d4 => {
  { let v: u32 = cpu.r32(0x11d5894u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d5890u32); cpu.eax = v; }
  cpu.push(0x10183e5); f_010187dd(cpu); if cpu.returned_to != 0x10183e5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10183eb; continue; }
  pc = 0x10183e9; continue;
 }
 0x10183e9 => {
  pc = 0x10183f7; continue;
 }
 0x10183eb => {
  { let v: u32 = cpu.dec32(cpu.r32(0x11d5894u32)); cpu.w32(0x11d5894u32, v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(0xffffff80u32); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10183b0; continue;
 }
 0x10183f7 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d589cu32, v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11d5898u32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x101840d; continue;
 }
 0x101840d => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x1018431; continue; }
  pc = 0x101841f; continue;
 }
 0x101841f => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x1018446; continue; }
  pc = 0x1018431; continue;
 }
 0x1018431 => {
  { let v: u32 = cpu.r32(0x11d5898u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d589cu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x1018442); f_010187dd(cpu); if cpu.returned_to != 0x1018442 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x1018448; continue; }
  pc = 0x1018446; continue;
 }
 0x1018446 => {
  pc = 0x1018456; continue;
 }
 0x1018448 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x11d589cu32)); cpu.w32(0x11d589cu32, v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x101840d; continue;
 }
 0x1018456 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x101845c; continue;
 }
 0x101845c => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x1018486; continue; }
  pc = 0x1018471; continue;
 }
 0x1018471 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x80u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x101849b; continue; }
  pc = 0x1018486; continue;
 }
 0x1018486 => {
  { let v: u32 = cpu.r32(0x11d5898u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d589cu32); cpu.eax = v; }
  cpu.push(0x1018497); f_010187dd(cpu); if cpu.returned_to != 0x1018497 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x101849d; continue; }
  pc = 0x101849b; continue;
 }
 0x101849b => {
  pc = 0x10184ac; continue;
 }
 0x101849d => {
  { let v: u32 = cpu.inc32(cpu.r32(0x11d5898u32)); cpu.w32(0x11d5898u32, v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(0x80u32); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x101845c; continue;
 }
 0x10184ac => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010184b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10184b4); }
 let mut pc: u32 = 0x10184b4;
 loop { match pc {
 0x10184b4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)).wrapping_add(0xfffffffeu32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11d5888u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10184d4; continue;
 }
 0x10184d4 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11d5888u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10184e9; continue; }
  pc = 0x10184df; continue;
 }
 0x10184df => {
  pc = 0x1018539; continue;
 }
 0x10184e1 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10184d4; continue;
 }
 0x10184e9 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10184f0; continue;
 }
 0x10184f0 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x9u32); }
  if (cpu.sf != cpu.of) { pc = 0x1018500; continue; }
  pc = 0x10184f6; continue;
 }
 0x10184f6 => {
  pc = 0x1018531; continue;
 }
 0x10184f8 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10184f0; continue;
 }
 0x1018500 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110f044u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1018514); f_01018621(cpu); if cpu.returned_to != 0x1018514 { return; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x101852f; continue; }
  pc = 0x1018518; continue;
 }
 0x1018518 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11d588cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x1018529); f_01018549(cpu); if cpu.returned_to != 0x1018529 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x101853d; continue;
 }
 0x101852f => {
  cpu.poll(); pc = 0x10184f8; continue;
 }
 0x1018531 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10184e1; continue;
 }
 0x1018539 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x101853d; continue;
 }
 0x101853d => {
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_01018549(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018549); }
 let mut pc: u32 = 0x1018549;
 loop { match pc {
 0x1018549 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e40a8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110f068u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110f06cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x101858b; continue;
 }
 0x101858b => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x60u32); }
  if (cpu.sf != cpu.of) { pc = 0x101859e; continue; }
  pc = 0x1018591; continue;
 }
 0x1018591 => {
  pc = 0x1018613; continue;
 }
 0x1018596 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x101858b; continue;
 }
 0x101859e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x0u16); }
  if !cpu.zf { pc = 0x1018607; continue; }
  pc = 0x10185a7; continue;
 }
 0x10185a7 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x10u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x10u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x5u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x10u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10185fc); f_010181a9(cpu); if cpu.returned_to != 0x10185fc { return; }
  cpu.push(0x1018601); f_01034285(cpu); if cpu.returned_to != 0x1018601 { return; }
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018617; continue;
 }
 0x1018607 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(0x88u32); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1018596; continue;
 }
 0x1018613 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018617; continue;
 }
 0x1018617 => {
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
pub fn f_01018621(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018621); }
 let mut pc: u32 = 0x1018621;
 loop { match pc {
 0x1018621 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x101863b; continue;
 }
 0x101863b => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x8u32); }
  if (cpu.sf != cpu.of) { pc = 0x101864b; continue; }
  pc = 0x1018641; continue;
 }
 0x1018641 => {
  pc = 0x101868b; continue;
 }
 0x1018643 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x101863b; continue;
 }
 0x101864b => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018652; continue;
 }
 0x1018652 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x8u32); }
  if (cpu.sf != cpu.of) { pc = 0x1018662; continue; }
  pc = 0x1018658; continue;
 }
 0x1018658 => {
  pc = 0x1018685; continue;
 }
 0x101865a => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1018652; continue;
 }
 0x1018662 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x101866d); f_0101869a(cpu); if cpu.returned_to != 0x101866d { return; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x1018677; continue; }
  pc = 0x1018671; continue;
 }
 0x1018671 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x101868f; continue;
 }
 0x1018677 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x101865a; continue;
 }
 0x1018685 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)).wrapping_add(0x78u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1018643; continue;
 }
 0x101868b => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x101868f; continue;
 }
 0x101868f => {
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_0101869a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101869a); }
 let mut pc: u32 = 0x101869a;
 loop { match pc {
 0x101869a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10e6910u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)) & 0x7fu32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10186d9); f_010187dd(cpu); if cpu.returned_to != 0x10186d9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10186e3; continue; }
  pc = 0x10186dd; continue;
 }
 0x10186dd => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018727; continue;
 }
 0x10186e3 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x2u8); }
  if !cpu.zf { pc = 0x10186f1; continue; }
  pc = 0x10186eb; continue;
 }
 0x10186eb => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018727; continue;
 }
 0x10186f1 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x7u8); }
  if !cpu.zf { pc = 0x1018715; continue; }
  pc = 0x1018701; continue;
 }
 0x1018701 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x101870f; continue; }
  pc = 0x1018709; continue;
 }
 0x1018709 => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018727; continue;
 }
 0x101870f => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018727; continue;
 }
 0x1018715 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x1018723; continue; }
  pc = 0x101871d; continue;
 }
 0x101871d => {
  { let v: u8 = 0x1u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018727; continue;
 }
 0x1018723 => {
  { let v: u8 = 0x0u8; cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018727; continue;
 }
 0x1018727 => {
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_01018732(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018732); }
 let mut pc: u32 = 0x1018732;
 loop { match pc {
 0x1018732 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018745; continue;
 }
 0x1018745 => {
  { let v: u32 = cpu.r32(0x11d588cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d5888u32); cpu.eax = v; }
  cpu.push(0x1018755); f_010188b5(cpu); if cpu.returned_to != 0x1018755 { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x1018770; continue; }
  pc = 0x1018766; continue;
 }
 0x1018766 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x1018799; continue; }
  pc = 0x1018770; continue;
 }
 0x1018770 => {
  { let v: u32 = cpu.r32(0x11d588cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d5888u32); cpu.eax = v; }
  cpu.push(0x1018780); f_010187dd(cpu); if cpu.returned_to != 0x1018780 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x1018799; continue; }
  pc = 0x1018784; continue;
 }
 0x1018784 => {
  { let v: u32 = cpu.r32(0x11d588cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x11d5888u32); cpu.eax = v; }
  cpu.push(0x1018794); f_010188b5(cpu); if cpu.returned_to != 0x1018794 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10187d2; continue;
 }
 0x1018799 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x11d5888u32)); cpu.w32(0x11d5888u32, v); }
  { cpu.sub32(cpu.r32(0x11d5888u32), 0x80u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1018745; continue; }
  pc = 0x10187ab; continue;
 }
 0x10187ab => {
  { let v: u32 = 0x0u32; cpu.w32(0x11d5888u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x11d588cu32)); cpu.w32(0x11d588cu32, v); }
  { cpu.sub32(cpu.r32(0x11d588cu32), 0x80u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x1018745; continue; }
  pc = 0x10187cb; continue;
 }
 0x10187cb => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10187d2; continue;
 }
 0x10187d2 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_010187dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10187dd); }
 let mut pc: u32 = 0x10187dd;
 loop { match pc {
 0x10187dd => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e40a8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10e40a8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018800; continue;
 }
 0x1018800 => {
  { let v: u32 = cpu.r32(0x10e40a8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3300u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x1018820; continue; }
  pc = 0x101880f; continue;
 }
 0x101880f => {
  pc = 0x10188a3; continue;
 }
 0x1018814 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(0x88u32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1018800; continue;
 }
 0x1018820 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax)), 0x0u16); }
  if cpu.zf { pc = 0x101889e; continue; }
  pc = 0x101882d; continue;
 }
 0x101882d => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x101889e; continue; }
  pc = 0x1018845; continue;
 }
 0x1018845 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x101889e; continue; }
  pc = 0x1018860; continue;
 }
 0x1018860 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x101889e; continue; }
  pc = 0x101887a; continue;
 }
 0x101887a => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x101889e; continue; }
  pc = 0x1018895; continue;
 }
 0x1018895 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10188aa; continue;
 }
 0x101889e => {
  cpu.poll(); pc = 0x1018814; continue;
 }
 0x10188a3 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10188aa; continue;
 }
 0x10188aa => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_010188b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10188b5); }
 let mut pc: u32 = 0x10188b5;
 loop { match pc {
 0x10188b5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10ea910u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_010188e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10188e8); }
 let mut pc: u32 = 0x10188e8;
 loop { match pc {
 0x10188e8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10e40a8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1018905; continue;
 }
 0x1018905 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x60u32); }
  if (cpu.sf != cpu.of) { pc = 0x1018915; continue; }
  pc = 0x101890b; continue;
 }
 0x101890b => {
  pc = 0x101892d; continue;
 }
 0x101890d => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x1018905; continue;
 }
 0x1018915 => {
  { let v: u32 = 0x88u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(0x88u32); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x101892b); f_0108f030(cpu); if cpu.returned_to != 0x101892b { return; }
  cpu.poll(); pc = 0x101890d; continue;
 }
 0x101892d => {
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
pub fn f_01018938(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018938); }
 let mut pc: u32 = 0x1018938;
 loop { match pc {
 0x1018938 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x11d58a8u32, v); }
  { let v: u32 = 0x8u32; cpu.w32(0x11d58acu32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x1018967; continue; }
  pc = 0x1018957; continue;
 }
 0x1018957 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x1018967; continue; }
  pc = 0x101895d; continue;
 }
 0x101895d => {
  { let v: u32 = 0xcu32; cpu.w32(0x11d58a8u32, v); }
  pc = 0x1018967; continue;
 }
 0x1018967 => {
  { let v: u32 = 0x8u32; cpu.w32(0x11d58b0u32, v); }
  { let v: u32 = 0x8u32; cpu.w32(0x11d58b4u32, v); }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x1018991; continue; }
  pc = 0x1018981; continue;
 }
 0x1018981 => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.esi)), 0x3u8); }
  if cpu.zf { pc = 0x1018991; continue; }
  pc = 0x1018987; continue;
 }
 0x1018987 => {
  { let v: u32 = 0xcu32; cpu.w32(0x11d58b0u32, v); }
  pc = 0x1018991; continue;
 }
 0x1018991 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x71u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x10189a8; continue; }
  pc = 0x101899c; continue;
 }
 0x101899c => {
  { cpu.sub8(cpu.r8(0x79u32.wrapping_add(cpu.ebp)), 0x3u8); }
  if cpu.zf { pc = 0x10189a8; continue; }
  pc = 0x10189a2; continue;
 }
 0x10189a2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10189a8; continue;
 }
 0x10189a8 => {
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2cu32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11d58a8u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x11d58b0u32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(cpu.r16(0x2eu32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x11d58acu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x11d58b4u32)); cpu.eax = v; }
  { let v = (cpu.eax as u16); cpu.imul16(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if !cpu.cf { pc = 0x10189fd; continue; }
  pc = 0x10189fa; continue;
 }
 0x10189fa => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10189fd => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01018a00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018a00); }
 let mut pc: u32 = 0x1018a00;
 loop { match pc {
 0x1018a00 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.w32(0x110f0b0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x110f0b4u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x110f0b8u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x110f0bcu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x1105770u32, v); }
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  pc = 0x1018a28; continue;
 }
 0x1018a28 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1018a7e; continue; }
  pc = 0x1018a33; continue;
 }
 0x1018a33 => {
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
  if cpu.zf { pc = 0x1018a7e; continue; }
  pc = 0x1018a4f; continue;
 }
 0x1018a4f => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ebx); cpu.edx = v; }
  cpu.push(0x1018a5b); f_01019011(cpu); if cpu.returned_to != 0x1018a5b { return; }
  if !cpu.cf { pc = 0x1018a7e; continue; }
  pc = 0x1018a5d; continue;
 }
 0x1018a5d => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x10c5974u32); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = (cpu.ebx as u8); cpu.div8(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1105770u32), 0xffffffffu32); }
  if cpu.zf { pc = 0x1018a8a; continue; }
  pc = 0x1018a76; continue;
 }
 0x1018a76 => {
  { cpu.sub32(cpu.r32(0x1105770u32), cpu.eax); }
  if cpu.zf { pc = 0x1018a9c; continue; }
  pc = 0x1018a7e; continue;
 }
 0x1018a7e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1018a28; continue; }
  pc = 0x1018a87; continue;
 }
 0x1018a87 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1018a8a => {
  { let v: u32 = cpu.eax; cpu.w32(0x1105770u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x1105770u32); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1018a9c => {
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
pub fn f_01018aa3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018aa3); }
 let mut pc: u32 = 0x1018aa3;
 loop { match pc {
 0x1018aa3 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  cpu.push(0x1018ab6); f_01018b47(cpu); if cpu.returned_to != 0x1018ab6 { return; }
  if cpu.cf { pc = 0x1018abb; continue; }
  pc = 0x1018ab8; continue;
 }
 0x1018ab8 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1018abb => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x11a188cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10e40c0u32, v); }
  { cpu.popad(); }
  { let v: u16 = cpu.r16(0x1105729u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x110572bu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x1018ae6); f_01018af4(cpu); if cpu.returned_to != 0x1018ae6 { return; }
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10e40c0u32); cpu.edi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01018af4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018af4); }
 let mut pc: u32 = 0x1018af4;
 loop { match pc {
 0x1018af4 => {
  { cpu.pushad(); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = 0x1172f54u32; cpu.esi = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  pc = 0x1018b17; continue;
 }
 0x1018b17 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1018b40; continue; }
  pc = 0x1018b1e; continue;
 }
 0x1018b1e => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.esi)), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x1018b40; continue; }
  pc = 0x1018b24; continue;
 }
 0x1018b24 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), (cpu.ebx as u16)); }
  if !cpu.zf { pc = 0x1018b40; continue; }
  pc = 0x1018b2a; continue;
 }
 0x1018b2a => {
  { let v: u16 = 0xffffu16; cpu.w16(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x35u32.wrapping_add(cpu.esi)) & 0x7eu8; cpu.w8(0x35u32.wrapping_add(cpu.esi), v); }
  { cpu.sub16(cpu.r16(0x27u32.wrapping_add(cpu.esi)), 0x0u16); }
  if cpu.zf { pc = 0x1018b40; continue; }
  pc = 0x1018b3b; continue;
 }
 0x1018b3b => {
  { let v: u16 = 0xffffu16; cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1018b40; continue;
 }
 0x1018b40 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x40u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1018b17; continue; }
  pc = 0x1018b45; continue;
 }
 0x1018b45 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01018b47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018b47); }
 let mut pc: u32 = 0x1018b47;
 loop { match pc {
 0x1018b47 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1174854u32; cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  pc = 0x1018b55; continue;
 }
 0x1018b55 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebp)), 0xffffu16); }
  if cpu.zf { pc = 0x1018b8a; continue; }
  pc = 0x1018b5d; continue;
 }
 0x1018b5d => {
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
  if (!cpu.cf && !cpu.zf) { pc = 0x1018b8a; continue; }
  pc = 0x1018b86; continue;
 }
 0x1018b86 => {
  pc = 0x1018b95; continue;
 }
 0x1018b8a => {
  { let v: u32 = cpu.add32(cpu.ebp, 0xau32); cpu.ebp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1018b55; continue; }
  pc = 0x1018b8f; continue;
 }
 0x1018b8f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1018b95 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1105729u32, v); }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x110572bu32, v); }
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
pub fn f_01018baf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018baf); }
 let mut pc: u32 = 0x1018baf;
 loop { match pc {
 0x1018baf => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x1018bc7; continue;
 }
 0x1018bc7 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1018cb4; continue; }
  pc = 0x1018bd1; continue;
 }
 0x1018bd1 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1018cb4; continue; }
  pc = 0x1018bdd; continue;
 }
 0x1018bdd => {
  { cpu.sub32(cpu.r32(0x22u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x1018cb4; continue; }
  pc = 0x1018bea; continue;
 }
 0x1018bea => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x10e3e93u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1018cb2; continue; }
  pc = 0x1018c0a; continue;
 }
 0x1018c0a => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16).wrapping_add(cpu.r16(0x10e3e97u32)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edi as u16), cpu.r16(0x10e3e95u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1018cb2; continue; }
  pc = 0x1018c22; continue;
 }
 0x1018c22 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x10c6120u32; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e97u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x2u32.wrapping_add(cpu.ebp))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e93u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1018cb2; continue; }
  pc = 0x1018c4f; continue;
 }
 0x1018c4f => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3e95u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1018cb2; continue; }
  pc = 0x1018c58; continue;
 }
 0x1018c58 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x10e3e97u32, v); }
  { let v: u32 = 0x4au32; cpu.eax = v; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = 0x10c6120u32.wrapping_add(cpu.eax); cpu.ebp = v; }
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
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { let v: u16 = cpu.r16(0x10e3e97u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1018cb2 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1018cb4; continue;
 }
 0x1018cb4 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1018bc7; continue; }
  pc = 0x1018cbe; continue;
 }
 0x1018cbe => {
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01018cce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018cce); }
 let mut pc: u32 = 0x1018cce;
 loop { match pc {
 0x1018cce => {
  { cpu.pushad(); }
  cpu.push(0x1018cd4); f_01018e6e(cpu); if cpu.returned_to != 0x1018cd4 { return; }
  if !cpu.cf { pc = 0x1018ce5; continue; }
  pc = 0x1018cd6; continue;
 }
 0x1018cd6 => {
  { let v: u32 = cpu.r32(0x10e4154u32); cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1018ce5; continue; }
  pc = 0x1018ce2; continue;
 }
 0x1018ce2 => {
  { cpu.popad(); }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1018ce5 => {
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01018ce8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018ce8); }
 let mut pc: u32 = 0x1018ce8;
 loop { match pc {
 0x1018ce8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x1018d00; continue;
 }
 0x1018d00 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1018d8c; continue; }
  pc = 0x1018d0a; continue;
 }
 0x1018d0a => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1018d8c; continue; }
  pc = 0x1018d12; continue;
 }
 0x1018d12 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x10e3e93u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1018d8a; continue; }
  pc = 0x1018d21; continue;
 }
 0x1018d21 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edi as u16), cpu.r16(0x10e3e95u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1018d8a; continue; }
  pc = 0x1018d2e; continue;
 }
 0x1018d2e => {
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x10c6120u32); cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1018d53; continue; }
  pc = 0x1018d50; continue;
 }
 0x1018d50 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1018d53; continue;
 }
 0x1018d53 => {
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e93u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1018d8a; continue; }
  pc = 0x1018d66; continue;
 }
 0x1018d66 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3e95u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1018d8a; continue; }
  pc = 0x1018d6f; continue;
 }
 0x1018d6f => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1018d8a => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1018d8c; continue;
 }
 0x1018d8c => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1018d00; continue; }
  pc = 0x1018d96; continue;
 }
 0x1018d96 => {
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01018da6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018da6); }
 let mut pc: u32 = 0x1018da6;
 loop { match pc {
 0x1018da6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x1018dbe; continue;
 }
 0x1018dbe => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1018e54; continue; }
  pc = 0x1018dc8; continue;
 }
 0x1018dc8 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1018e54; continue; }
  pc = 0x1018dd4; continue;
 }
 0x1018dd4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = cpu.r8(0x38u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x10fd445u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x10e3e93u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1018e52; continue; }
  pc = 0x1018ded; continue;
 }
 0x1018ded => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edi as u16), cpu.r16(0x10e3e95u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1018e52; continue; }
  pc = 0x1018dfa; continue;
 }
 0x1018dfa => {
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x10c6120u32); cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x10fd445u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1018e1f; continue; }
  pc = 0x1018e1f; continue;
 }
 0x1018e1f => {
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e93u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1018e52; continue; }
  pc = 0x1018e2e; continue;
 }
 0x1018e2e => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3e95u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1018e52; continue; }
  pc = 0x1018e37; continue;
 }
 0x1018e37 => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1018e52 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1018e54; continue;
 }
 0x1018e54 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1018dbe; continue; }
  pc = 0x1018e5e; continue;
 }
 0x1018e5e => {
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01018e6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018e6e); }
 let mut pc: u32 = 0x1018e6e;
 loop { match pc {
 0x1018e6e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x1018e86; continue;
 }
 0x1018e86 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1018f25; continue; }
  pc = 0x1018e90; continue;
 }
 0x1018e90 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x1018f25; continue; }
  pc = 0x1018e9c; continue;
 }
 0x1018e9c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x10e3e93u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1018f23; continue; }
  pc = 0x1018eab; continue;
 }
 0x1018eab => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edi as u16), cpu.r16(0x10e3e95u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1018f23; continue; }
  pc = 0x1018eb8; continue;
 }
 0x1018eb8 => {
  { cpu.sub8(cpu.r8(0x112ad0cu32), 0x0u8); }
  if !cpu.zf { pc = 0x1018ec7; continue; }
  pc = 0x1018ec1; continue;
 }
 0x1018ec1 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x2u8); }
  if cpu.zf { pc = 0x1018f23; continue; }
  pc = 0x1018ec7; continue;
 }
 0x1018ec7 => {
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x10c6120u32); cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1018eec; continue; }
  pc = 0x1018ee9; continue;
 }
 0x1018ee9 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1018eec; continue;
 }
 0x1018eec => {
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e93u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1018f23; continue; }
  pc = 0x1018eff; continue;
 }
 0x1018eff => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3e95u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1018f23; continue; }
  pc = 0x1018f08; continue;
 }
 0x1018f08 => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1018f23 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1018f25; continue;
 }
 0x1018f25 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1018e86; continue; }
  pc = 0x1018f2f; continue;
 }
 0x1018f2f => {
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01018f3f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018f3f); }
 let mut pc: u32 = 0x1018f3f;
 loop { match pc {
 0x1018f3f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10e3e93u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10e3e95u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x116c0d8u32; cpu.ebp = v; }
  { let v: u32 = 0x23u32; cpu.ecx = v; }
  pc = 0x1018f57; continue;
 }
 0x1018f57 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0xffu8); }
  if cpu.zf { pc = 0x1018fe3; continue; }
  pc = 0x1018f61; continue;
 }
 0x1018f61 => {
  { cpu.sub16(cpu.r16(0x1cu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if !cpu.zf { pc = 0x1018fe3; continue; }
  pc = 0x1018f69; continue;
 }
 0x1018f69 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x10e3e93u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1018fe1; continue; }
  pc = 0x1018f78; continue;
 }
 0x1018f78 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edi as u16), cpu.r16(0x10e3e95u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1018fe1; continue; }
  pc = 0x1018f85; continue;
 }
 0x1018f85 => {
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.mul8(v); }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x10c6120u32); cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if !cpu.zf { pc = 0x1018faa; continue; }
  pc = 0x1018fa7; continue;
 }
 0x1018fa7 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x1018faa; continue;
 }
 0x1018faa => {
  { let v: u16 = cpu.shl16((cpu.ebx as u16), 4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add((cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x10e3e93u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1018fe1; continue; }
  pc = 0x1018fbd; continue;
 }
 0x1018fbd => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x10e3e95u32)); }
  if (cpu.sf != cpu.of) { pc = 0x1018fe1; continue; }
  pc = 0x1018fc6; continue;
 }
 0x1018fc6 => {
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x33u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16).wrapping_add((cpu.esi as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.edi as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x10e4154u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1018fe1 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x1018fe3; continue;
 }
 0x1018fe3 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x50u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1018f57; continue; }
  pc = 0x1018fed; continue;
 }
 0x1018fed => {
  { let v: u16 = cpu.r16(0x10e3e93u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3e95u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01018ffd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1018ffd); }
 let mut pc: u32 = 0x1018ffd;
 loop { match pc {
 0x1018ffd => {
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x101900f; continue; }
  pc = 0x1019001; continue;
 }
 0x1019001 => {
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x101900f; continue; }
  pc = 0x1019005; continue;
 }
 0x1019005 => {
  { cpu.sub32(cpu.edi, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x101900f; continue; }
  pc = 0x1019009; continue;
 }
 0x1019009 => {
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x101900f; continue; }
  pc = 0x101900d; continue;
 }
 0x101900d => {
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x101900f => {
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01019011(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1019011); }
 let mut pc: u32 = 0x1019011;
 loop { match pc {
 0x1019011 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x110f0b0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x110f0b4u32); cpu.edi = v; }
  cpu.push(0x1019023); f_01018ffd(cpu); if cpu.returned_to != 0x1019023 { return; }
  if cpu.cf { pc = 0x101908e; continue; }
  pc = 0x1019025; continue;
 }
 0x1019025 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x110f0b8u32)); cpu.esi = v; }
  cpu.push(0x1019030); f_01018ffd(cpu); if cpu.returned_to != 0x1019030 { return; }
  if cpu.cf { pc = 0x101908e; continue; }
  pc = 0x1019032; continue;
 }
 0x1019032 => {
  { let v: u32 = cpu.r32(0x110f0b0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x110f0b4u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x110f0bcu32)); cpu.edi = v; }
  cpu.push(0x1019049); f_01018ffd(cpu); if cpu.returned_to != 0x1019049 { return; }
  if cpu.cf { pc = 0x101908e; continue; }
  pc = 0x101904b; continue;
 }
 0x101904b => {
  { let v: u32 = cpu.r32(0x110f0b0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x110f0b4u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x110f0b8u32)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x110f0bcu32)); cpu.edi = v; }
  cpu.push(0x1019068); f_01018ffd(cpu); if cpu.returned_to != 0x1019068 { return; }
  if cpu.cf { pc = 0x101908e; continue; }
  pc = 0x101906a; continue;
 }
 0x101906a => {
  { let v: u32 = cpu.r32(0x110f0b8u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x110f0bcu32); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 1u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x110f0b0u32)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x110f0b4u32)); cpu.edi = v; }
  cpu.push(0x101908b); f_01018ffd(cpu); if cpu.returned_to != 0x101908b { return; }
  if cpu.cf { pc = 0x101908e; continue; }
  pc = 0x101908d; continue;
 }
 0x101908d => {
  { cpu.cf = false; }
  pc = 0x101908e; continue;
 }
 0x101908e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01019090(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1019090); }
 let mut pc: u32 = 0x1019090;
 loop { match pc {
 0x1019090 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x0u8); }
  if cpu.zf { pc = 0x10190dd; continue; }
  pc = 0x101909e; continue;
 }
 0x101909e => {
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x2u8); }
  if cpu.zf { pc = 0x10190ce; continue; }
  pc = 0x10190a7; continue;
 }
 0x10190a7 => {
  { cpu.sub8(cpu.r8(0x10ee91du32), 0x3u8); }
  if cpu.zf { pc = 0x10190bf; continue; }
  pc = 0x10190b0; continue;
 }
 0x10190b0 => {
  { let v: u16 = cpu.r16(0x10e3d1bu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d1du32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10190ea; continue;
 }
 0x10190bf => {
  { let v: u16 = cpu.r16(0x10e3d27u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d29u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10190ea; continue;
 }
 0x10190ce => {
  { let v: u16 = cpu.r16(0x10e3d23u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d25u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10190ea; continue;
 }
 0x10190dd => {
  { let v: u16 = cpu.r16(0x10e3d1fu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x10e3d21u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x10190ea; continue;
 }
 0x10190ea => {
  { let v: u32 = 0xa0u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = 0xa0u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  cpu.push(0x10190fb); f_01018ffd(cpu); if cpu.returned_to != 0x10190fb { return; }
  if !cpu.cf { pc = 0x1019131; continue; }
  pc = 0x10190fd; continue;
 }
 0x10190fd => {
  cpu.push(0x1019102); f_010576b2(cpu); if cpu.returned_to != 0x1019102 { return; }
  { let v: u8 = cpu.r8(0x31u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110597du32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.push(0x101911a); f_0104f36d(cpu); if cpu.returned_to != 0x101911a { return; }
  { let v: u8 = cpu.r8(0x4000u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fu32; cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x1019134; continue; }
  pc = 0x1019127; continue;
 }
 0x1019127 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if cpu.zf { pc = 0x1019134; continue; }
  pc = 0x1019131; continue;
 }
 0x1019131 => {
  { cpu.cf = false; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1019134 => {
  { cpu.cf = true; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01019137(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1019137); }
 let mut pc: u32 = 0x1019137;
 loop { match pc {
 0x1019137 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax & 0xfff0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xfff0u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110f0b0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x110f0b4u32, v); }
  { let v: u32 = 0x20u32; cpu.w32(0x110f0b8u32, v); }
  { let v: u32 = 0x20u32; cpu.w32(0x110f0bcu32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  pc = 0x1019178; continue;
 }
 0x1019178 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x10191c4; continue; }
  pc = 0x1019182; continue;
 }
 0x1019182 => {
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
  cpu.push(0x10191a3); f_01019011(cpu); if cpu.returned_to != 0x10191a3 { return; }
  if !cpu.cf { pc = 0x10191c4; continue; }
  pc = 0x10191a5; continue;
 }
 0x10191a5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x110f0b0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x110f0b4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x110f0b0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x110f0b4u32); cpu.edi = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10191c4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x28u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1019178; continue; }
  pc = 0x10191cd; continue;
 }
 0x10191cd => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.popad(); }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010191d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10191d1); }
 let mut pc: u32 = 0x10191d1;
 loop { match pc {
 0x10191d1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0xc8u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  pc = 0x10191e7; continue;
 }
 0x10191e7 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x1019246; continue; }
  pc = 0x10191ee; continue;
 }
 0x10191ee => {
  { cpu.sub8(cpu.r8(0x110d6f0u32), 0x0u8); }
  if !cpu.zf { pc = 0x10191fd; continue; }
  pc = 0x10191f7; continue;
 }
 0x10191f7 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x50u8); }
  if cpu.zf { pc = 0x1019246; continue; }
  pc = 0x10191fd; continue;
 }
 0x10191fd => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x1019246; continue; }
  pc = 0x1019202; continue;
 }
 0x1019202 => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x1019246; continue; }
  pc = 0x1019208; continue;
 }
 0x1019208 => {
  { cpu.sub8(cpu.r8(0x110d769u32), 0x0u8); }
  if !cpu.zf { pc = 0x1019221; continue; }
  pc = 0x1019211; continue;
 }
 0x1019211 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 3u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1019229; continue;
 }
 0x1019221 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x1019229; continue;
 }
 0x1019229 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1019246; continue; }
  pc = 0x1019233; continue;
 }
 0x1019233 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1019246; continue; }
  pc = 0x101923c; continue;
 }
 0x101923c => {
  { let v: u8 = 0x0u8; cpu.w8(0x110d769u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1019246 => {
  { let v: u32 = cpu.add32(cpu.esi, 0x28u32); cpu.esi = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x10191e7; continue; }
  pc = 0x101924b; continue;
 }
 0x101924b => {
  { let v: u8 = 0x0u8; cpu.w8(0x110d769u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01019255(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1019255); }
 let mut pc: u32 = 0x1019255;
 loop { match pc {
 0x1019255 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x11a188cu32; cpu.esi = v; }
  { let v: u32 = 0x28u32; cpu.eax = v; }
  { let v = ((cpu.edx >> 8) as u8); cpu.mul8(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.esi)), 0xffffu16); }
  if cpu.zf { pc = 0x10192d0; continue; }
  pc = 0x1019278; continue;
 }
 0x1019278 => {
  { cpu.sub8(cpu.r8(0x110d6f0u32), 0x0u8); }
  if !cpu.zf { pc = 0x1019287; continue; }
  pc = 0x1019281; continue;
 }
 0x1019281 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.esi)), 0x50u8); }
  if cpu.zf { pc = 0x10192d0; continue; }
  pc = 0x1019287; continue;
 }
 0x1019287 => {
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x10192d0; continue; }
  pc = 0x101928c; continue;
 }
 0x101928c => {
  { cpu.sub16((cpu.ebx as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x10192d0; continue; }
  pc = 0x1019292; continue;
 }
 0x1019292 => {
  { cpu.sub8(cpu.r8(0x110d769u32), 0x0u8); }
  if !cpu.zf { pc = 0x10192ab; continue; }
  pc = 0x101929b; continue;
 }
 0x101929b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x12u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), 3u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10192b3; continue;
 }
 0x10192ab => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10192b3; continue;
 }
 0x10192b3 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10192d0; continue; }
  pc = 0x10192bd; continue;
 }
 0x10192bd => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub16((cpu.ebp as u16), cpu.r16(0x2u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10192d0; continue; }
  pc = 0x10192c6; continue;
 }
 0x10192c6 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110d769u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = true; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10192d0 => {
  { let v: u8 = 0x0u8; cpu.w8(0x110d769u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.cf = false; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010192da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10192da); }
 let mut pc: u32 = 0x10192da;
 loop { match pc {
 0x10192da => {
  f_010191d1(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010192df(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10192df); } r_010192df(cpu, 0x10192df); }
pub fn f_010194a6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10194a6); } r_010192df(cpu, 0x10194a6); }
fn r_010192df(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10192df => {
  { let v = cpu.edi; cpu.push(v); }
  if cpu.cf { pc = 0x101934b; continue; }
  pc = 0x10192e2; continue;
 }
 0x10192e2 => {
  if cpu.zf { pc = 0x1019349; continue; }
  pc = 0x10192e4; continue;
 }
 0x10192e4 => {
  { let v: u8 = cpu.and8(cpu.r8(0x72u32.wrapping_add(cpu.eax)), ((cpu.edx >> 8) as u8)); cpu.w8(0x72u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x10192e7, "Outsd");
  if cpu.zf { pc = 0x101934f; continue; }
  pc = 0x10192ea; continue;
 }
 0x10192ea => {
  cpu.unimplemented(0x10192ea, "Arpl");
  if cpu.cf { pc = 0x1019362; continue; }
  pc = 0x10192f0; continue;
 }
 0x10192f0 => {
  cpu.unimplemented(0x10192f0, "Outsd");
  if cpu.cf { pc = 0x10192f3; continue; }
  pc = 0x10192f3; continue;
 }
 0x10192f3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x6eu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.edx as u8)); cpu.w8(0x6eu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6fu32.wrapping_add(cpu.esi)), 0x77u32); cpu.ebp = v; }
  cpu.unimplemented(0x1019308, "Outsb");
  { let v: u8 = cpu.r8(0x6eu32.wrapping_add(cpu.ebp)) & ((cpu.edx >> 8) as u8); cpu.w8(0x6eu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))), 0x0u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x69u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(2))), (cpu.eax as u8)); cpu.w8(0x69u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(2)), v); }
  if (cpu.cf || cpu.zf) { pc = 0x101938f; continue; }
  pc = 0x101932a; continue;
 }
 0x101932a => {
  { let v: u8 = cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.esi)), ((cpu.ecx >> 8) as u8)); cpu.w8(0x6fu32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x101934f; continue; }
  pc = 0x101932f; continue;
 }
 0x101932f => {
  if cpu.cf { pc = 0x1019396; continue; }
  pc = 0x1019331; continue;
 }
 0x1019331 => {
  { cpu.popad(); }
  if !cpu.sf { pc = 0x1019335; continue; }
  pc = 0x1019335; continue;
 }
 0x1019335 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x6eu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.edx as u8)); cpu.w8(0x6eu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6fu32.wrapping_add(cpu.esi)), 0x77u32); cpu.ebp = v; }
  cpu.unimplemented(0x101934e, "Outsb");
  pc = 0x101934f; continue;
 }
 0x1019349 => {
  cpu.unimplemented(0x1019349, "Outsb");
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6fu32.wrapping_add(cpu.esi)), 0x77u32); cpu.ebp = v; }
  cpu.unimplemented(0x101934e, "Outsb");
  pc = 0x101934f; continue;
 }
 0x101934b => {
  cpu.unimplemented(0x101934b, "Outsb");
  cpu.unimplemented(0x101934c, "Outsd");
  if (!cpu.cf && !cpu.zf) { pc = 0x10193bd; continue; }
  pc = 0x101934f; continue;
 }
 0x101934f => {
  { let v: u8 = cpu.r8(0x6fu32.wrapping_add(cpu.ebx)) & ((cpu.eax >> 8) as u8); cpu.w8(0x6fu32.wrapping_add(cpu.ebx), v); }
  cpu.unimplemented(0x1019352, "Insd");
  cpu.unimplemented(0x1019353, "Insd");
  { cpu.popad(); }
  cpu.unimplemented(0x1019355, "Outsb");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[4])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[4]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x72u32.wrapping_add(cpu.ebp)), ((cpu.eax >> 8) as u8)); cpu.w8(0x72u32.wrapping_add(cpu.ebp), v); }
  if cpu.cf { pc = 0x10193e2; continue; }
  pc = 0x1019373; continue;
 }
 0x1019362 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x52u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x52u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.and8(cpu.r8(0x72u32.wrapping_add(cpu.ebp)), ((cpu.eax >> 8) as u8)); cpu.w8(0x72u32.wrapping_add(cpu.ebp), v); }
  if cpu.cf { pc = 0x10193e2; continue; }
  pc = 0x1019373; continue;
 }
 0x1019373 => {
  if cpu.cf { pc = 0x1019375; continue; }
  pc = 0x1019375; continue;
 }
 0x1019375 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x61u32.wrapping_add(cpu.edx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x61u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.and8(cpu.r8(0x69u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(2)).wrapping_add(cpu.bases[4])), ((cpu.eax >> 8) as u8)); cpu.w8(0x69u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(2)).wrapping_add(cpu.bases[4]), v); }
  if (cpu.cf || cpu.zf) { pc = 0x10193fc; continue; }
  pc = 0x1019397; continue;
 }
 0x101938f => {
  { cpu.popad(); }
  { let v: u8 = cpu.and8(cpu.r8(0x69u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(2)).wrapping_add(cpu.bases[4])), ((cpu.eax >> 8) as u8)); cpu.w8(0x69u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(2)).wrapping_add(cpu.bases[4]), v); }
  if (cpu.cf || cpu.zf) { pc = 0x10193fc; continue; }
  pc = 0x1019397; continue;
 }
 0x1019396 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x75716572u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x1019413; continue; }
  pc = 0x101939f; continue;
 }
 0x1019397 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x75716572u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x1019413; continue; }
  pc = 0x101939f; continue;
 }
 0x101939f => {
  { let v: u8 = cpu.and8(cpu.r8(0x74u32.wrapping_add(cpu.ebx)), ((cpu.edx >> 8) as u8)); cpu.w8(0x74u32.wrapping_add(cpu.ebx), v); }
  if cpu.cf { pc = 0x1019419; continue; }
  pc = 0x10193a4; continue;
 }
 0x10193a4 => {
  cpu.unimplemented(0x10193a4, "Arpl");
  { let v: u8 = cpu.and8(cpu.r8(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[5])), ((cpu.ecx >> 8) as u8)); cpu.w8(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[5]), v); }
  if cpu.zf { pc = 0x1019418; continue; }
  pc = 0x10193b0; continue;
 }
 0x10193b0 => {
  { let v: u8 = cpu.r8(0x65u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x65u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])), 0x65u32); cpu.esp = v; }
  if cpu.cf { pc = 0x101942b; continue; }
  pc = 0x10193b9; continue;
 }
 0x10193b9 => {
  cpu.unimplemented(0x10193b9, "Outsd");
  if cpu.cf { pc = 0x10193bc; continue; }
  pc = 0x10193bc; continue;
 }
 0x10193bc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.unimplemented(0x10193d5, "Outsb");
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6fu32.wrapping_add(cpu.esi)), 0x77u32); cpu.ebp = v; }
  cpu.unimplemented(0x10193da, "Outsb");
  { let v: u8 = cpu.r8(0x65u32.wrapping_add(cpu.ebp)) & ((cpu.ecx >> 8) as u8); cpu.w8(0x65u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x20u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[4])), 0x65707974u32); cpu.esp = v; }
  pc = 0x10193e6; continue;
 }
 0x10193bd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x6eu32.wrapping_add(cpu.ebp)).wrapping_add((cpu.edx as u8)); cpu.w8(0x6eu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6fu32.wrapping_add(cpu.esi)), 0x77u32); cpu.ebp = v; }
  cpu.unimplemented(0x10193da, "Outsb");
  { let v: u8 = cpu.r8(0x65u32.wrapping_add(cpu.ebp)) & ((cpu.ecx >> 8) as u8); cpu.w8(0x65u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x20u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[4])), 0x65707974u32); cpu.esp = v; }
  pc = 0x10193e6; continue;
 }
 0x10193e2 => {
  if cpu.zf { pc = 0x101945d; continue; }
  pc = 0x10193e4; continue;
 }
 0x10193e4 => {
  if cpu.of { pc = 0x101944b; continue; }
  pc = 0x10193e6; continue;
 }
 0x10193e6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x65u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x65u32.wrapping_add(cpu.ebx), v); }
  cpu.unimplemented(0x10193f9, "Arpl");
  { let v: u8 = cpu.and8(cpu.r8(0x6fu32.wrapping_add(cpu.esi)), ((cpu.ecx >> 8) as u8)); cpu.w8(0x6fu32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x1019422; continue; }
  pc = 0x1019402; continue;
 }
 0x10193fc => {
  if cpu.cf { pc = 0x101941e; continue; }
  pc = 0x10193fe; continue;
 }
 0x10193fe => {
  cpu.unimplemented(0x10193fe, "Outsb");
  cpu.unimplemented(0x10193ff, "Outsd");
  if cpu.zf { pc = 0x1019422; continue; }
  pc = 0x1019402; continue;
 }
 0x1019402 => {
  cpu.unimplemented(0x1019402, "Outsw");
  if !cpu.zf { pc = 0x1019474; continue; }
  pc = 0x1019406; continue;
 }
 0x1019406 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[4])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[4]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1019413; continue;
 }
 0x1019413 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1019419; continue;
 }
 0x1019418 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  if cpu.cf { pc = 0x1019486; continue; }
  pc = 0x101941d; continue;
 }
 0x1019419 => {
  { let v: u8 = cpu.r8(0x72u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x72u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x74u32.wrapping_add(cpu.esi)), 0x6e207265u32); cpu.ebp = v; }
  cpu.unimplemented(0x1019423, "Outsd");
  if cpu.zf { pc = 0x1019446; continue; }
  pc = 0x1019426; continue;
 }
 0x101941d => {
  cpu.unimplemented(0x101941d, "Outsb");
  pc = 0x101941e; continue;
 }
 0x101941e => {
  if cpu.zf { pc = 0x1019485; continue; }
  pc = 0x1019420; continue;
 }
 0x1019420 => {
  if cpu.cf { pc = 0x1019442; continue; }
  pc = 0x1019422; continue;
 }
 0x1019422 => {
  cpu.unimplemented(0x1019422, "Outsb");
  cpu.unimplemented(0x1019423, "Outsd");
  if cpu.zf { pc = 0x1019446; continue; }
  pc = 0x1019426; continue;
 }
 0x1019426 => {
  if cpu.cf { pc = 0x101948d; continue; }
  pc = 0x1019428; continue;
 }
 0x1019428 => {
  { cpu.popad(); }
  if !cpu.sf { pc = 0x101942c; continue; }
  pc = 0x101942c; continue;
 }
 0x101942b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edi; cpu.push(v); }
  if cpu.cf { pc = 0x10194a9; continue; }
  pc = 0x1019440; continue;
 }
 0x101942c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x72u32.wrapping_add(cpu.edi)).wrapping_add((cpu.edx as u8)); cpu.w8(0x72u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x20u32.wrapping_add(cpu.ebp)), 0x6c756166u32); cpu.esi = v; }
  pc = 0x1019447; continue;
 }
 0x1019440 => {
  if cpu.zf { pc = 0x10194a7; continue; }
  pc = 0x1019442; continue;
 }
 0x1019442 => {
  { let v: u8 = cpu.and8(cpu.r8(0x61u32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0x61u32.wrapping_add(cpu.esi), v); }
  if !cpu.zf { pc = 0x10194b3; continue; }
  pc = 0x1019447; continue;
 }
 0x1019446 => {
  cpu.unimplemented(0x1019446, "Insb");
  pc = 0x1019447; continue;
 }
 0x1019447 => {
  if cpu.zf { pc = 0x1019449; continue; }
  pc = 0x1019449; continue;
 }
 0x1019449 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x101944b; continue;
 }
 0x101944b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x101945d; continue;
 }
 0x101945d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x65u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x65u32.wrapping_add(cpu.edx), v); }
  { cpu.popad(); }
  { let v: u8 = cpu.and8(cpu.r8(0x61u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[4])), ((cpu.eax >> 8) as u8)); cpu.w8(0x61u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[4]), v); }
  if !cpu.zf { pc = 0x10194d5; continue; }
  pc = 0x1019469; continue;
 }
 0x1019469 => {
  if cpu.zf { pc = 0x101946b; continue; }
  pc = 0x101946b; continue;
 }
 0x101946b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.unimplemented(0x1019484, "Outsb");
  pc = 0x1019486; continue;
 }
 0x1019474 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x65u32.wrapping_add(cpu.edi)), (cpu.eax as u8)); cpu.w8(0x65u32.wrapping_add(cpu.edi), v); }
  pc = 0x1019485; continue;
 }
 0x1019485 => {
  cpu.unimplemented(0x1019485, "Outsb");
  pc = 0x1019486; continue;
 }
 0x1019486 => {
  if cpu.cf { pc = 0x10194ea; continue; }
  pc = 0x1019489; continue;
 }
 0x1019489 => {
  cpu.unimplemented(0x1019489, "Insb");
  { let v: u8 = cpu.r8(0x61u32.wrapping_add(cpu.esi)) & ((cpu.eax >> 8) as u8); cpu.w8(0x61u32.wrapping_add(cpu.esi), v); }
  pc = 0x101948d; continue;
 }
 0x101948d => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x72u32.wrapping_add(cpu.ebp).wrapping_add(cpu.esi.wrapping_mul(2))), 0x65u32); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x57u32.wrapping_add(cpu.esi)).wrapping_add((cpu.edx as u8)); cpu.w8(0x57u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x10194a9; continue;
 }
 0x10194a6 => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x10194a7; continue;
 }
 0x10194a7 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x10194a9; continue;
 }
 0x10194a9 => {
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10194be; continue;
 }
 0x10194b3 => {
  cpu.unimplemented(0x10194b3, "Hlt");
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10194be; continue;
 }
 0x10194be => {
  { let v: u32 = 0x11369b0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10194c9); f_0108dd9d(cpu); if cpu.returned_to != 0x10194c9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x23u32); cpu.eax = v; }
  { let v: u32 = 0x10192dfu32; cpu.edx = v; }
  pc = 0x10194d5; continue;
 }
 0x10194d5 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10194dd); f_0108dd9d(cpu); if cpu.returned_to != 0x10194dd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x11369bcu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10194eb); f_0108dd9d(cpu); if cpu.returned_to != 0x10194eb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.push(0x10194f3); f_0108de0a(cpu); if cpu.returned_to != 0x10194f3 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x52u32); }
  if cpu.cf { pc = 0x101953d; continue; }
  pc = 0x10194fc; continue;
 }
 0x10194ea => {
  { let v: u8 = cpu.r8(0x17e804c4u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x17e804c4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x7d83f845u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x7d83f845u32.wrapping_add(cpu.ecx), v); }
  { cpu.cf = false; }
  { let v = cpu.edx; cpu.push(v); }
  if cpu.cf { pc = 0x101953d; continue; }
  pc = 0x10194fc; continue;
 }
 0x10194fc => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x52u32); }
  if (cpu.cf || cpu.zf) { pc = 0x1019574; continue; }
  pc = 0x1019506; continue;
 }
 0x1019506 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x66u32); }
  if cpu.cf { pc = 0x1019532; continue; }
  pc = 0x101950c; continue;
 }
 0x101950c => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x66u32); }
  if (cpu.cf || cpu.zf) { pc = 0x101958f; continue; }
  pc = 0x1019516; continue;
 }
 0x1019516 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x69u32); }
  if cpu.cf { pc = 0x101952d; continue; }
  pc = 0x101951c; continue;
 }
 0x101951c => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x69u32); }
  if (cpu.cf || cpu.zf) { pc = 0x1019559; continue; }
  pc = 0x1019522; continue;
 }
 0x1019522 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x72u32); }
  if cpu.zf { pc = 0x101956b; continue; }
  pc = 0x1019528; continue;
 }
 0x1019528 => {
  pc = 0x10195a1; continue;
 }
 0x101952d => {
  pc = 0x10195a1; continue;
 }
 0x1019532 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x61u32); }
  if cpu.zf { pc = 0x101957d; continue; }
  pc = 0x1019538; continue;
 }
 0x1019538 => {
  pc = 0x10195a1; continue;
 }
 0x101953d => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x46u32); }
  if cpu.cf { pc = 0x1019551; continue; }
  pc = 0x1019543; continue;
 }
 0x1019543 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x46u32); }
  if (cpu.cf || cpu.zf) { pc = 0x1019598; continue; }
  pc = 0x1019549; continue;
 }
 0x1019549 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x49u32); }
  if cpu.zf { pc = 0x1019562; continue; }
  pc = 0x101954f; continue;
 }
 0x101954f => {
  pc = 0x10195a1; continue;
 }
 0x1019551 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x41u32); }
  if cpu.zf { pc = 0x1019586; continue; }
  pc = 0x1019557; continue;
 }
 0x1019557 => {
  pc = 0x10195a1; continue;
 }
 0x1019559 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10195a6; continue;
 }
 0x1019562 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10195a6; continue;
 }
 0x101956b => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10195a6; continue;
 }
 0x1019574 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10195a6; continue;
 }
 0x101957d => {
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10195a6; continue;
 }
 0x1019586 => {
  { let v: u32 = 0x2u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10195a6; continue;
 }
 0x101958f => {
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10195a6; continue;
 }
 0x1019598 => {
  { let v: u32 = 0x3u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10195a6; continue;
 }
 0x10195a1 => {
  cpu.poll(); pc = 0x10194be; continue;
 }
 0x10195a6 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01019600(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1019600); }
 let mut pc: u32 = 0x1019600;
 loop { match pc {
 0x1019600 => {
  { let a = cpu.ebp; let b = cpu.eax; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = 0xc4830007u32; cpu.ebx = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x5e5f5decu32.wrapping_add(cpu.ecx)), (cpu.ecx as u8)); cpu.w8(0x5e5f5decu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0101960e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101960e); }
 let mut pc: u32 = 0x101960e;
 loop { match pc {
 0x101960e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x11369e8u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101963f); f_01095199(cpu); if cpu.returned_to != 0x101963f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(0x3u32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x101964d; continue;
 }
 0x101964d => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x100u32); }
  if (cpu.sf != cpu.of) { pc = 0x1019663; continue; }
  pc = 0x1019656; continue;
 }
 0x1019656 => {
  pc = 0x101999b; continue;
 }
 0x101965b => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x101964d; continue;
 }
 0x1019663 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x101998f; continue; }
  pc = 0x101966f; continue;
 }
 0x101966f => {
  { let v: u32 = cpu.r32(0x10e3f3du32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x11u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x101969c; continue; }
  pc = 0x1019681; continue;
 }
 0x1019681 => {
  { let v: u32 = 0x11369e6u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1019692); f_01095199(cpu); if cpu.returned_to != 0x1019692 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x101969c; continue;
 }
 0x101969c => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x2au32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1133380u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x11d58b8u32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136a0eu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10196ec); f_01095199(cpu); if cpu.returned_to != 0x10196ec { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1136a22u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1019706); f_01095199(cpu); if cpu.returned_to != 0x1019706 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x101971c); f_0101a081(cpu); if cpu.returned_to != 0x101971c { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136a30u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101973a); f_01095199(cpu); if cpu.returned_to != 0x101973a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136a44u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101975d); f_01095199(cpu); if cpu.returned_to != 0x101975d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1019774); f_010199a3(cpu); if cpu.returned_to != 0x1019774 { return; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136a57u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1019797); f_01095199(cpu); if cpu.returned_to != 0x1019797 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1du32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10197af); f_010199a3(cpu); if cpu.returned_to != 0x10197af { return; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136a6au32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10197d2); f_01095199(cpu); if cpu.returned_to != 0x10197d2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1eu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  cpu.push(0x10197ed); f_010199a3(cpu); if cpu.returned_to != 0x10197ed { return; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1fu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136a7du32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1019810); f_01095199(cpu); if cpu.returned_to != 0x1019810 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1fu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1019828); f_010199a3(cpu); if cpu.returned_to != 0x1019828 { return; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136a90u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101984b); f_01095199(cpu); if cpu.returned_to != 0x101984b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x1019863); f_010199a3(cpu); if cpu.returned_to != 0x1019863 { return; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x21u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136aa3u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1019886); f_01095199(cpu); if cpu.returned_to != 0x1019886 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x21u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x101989e); f_010199a3(cpu); if cpu.returned_to != 0x101989e { return; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x22u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136ab6u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10198c1); f_01095199(cpu); if cpu.returned_to != 0x10198c1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x23u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136acau32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10198e4); f_01095199(cpu); if cpu.returned_to != 0x10198e4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x24u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136adeu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1019907); f_01095199(cpu); if cpu.returned_to != 0x1019907 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x24u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  cpu.push(0x1019922); f_010199a3(cpu); if cpu.returned_to != 0x1019922 { return; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.r32(0x11d58b8u32); cpu.push(v); }
  { let v: u32 = 0x1136af1u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1019942); f_01095199(cpu); if cpu.returned_to != 0x1019942 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x11d58b8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11d58b8u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136b1au32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101996e); f_01095199(cpu); if cpu.returned_to != 0x101996e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1136b43u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1019988); f_01095199(cpu); if cpu.returned_to != 0x1019988 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)).wrapping_add(0x2u32); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x101998f; continue;
 }
 0x101998f => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)).wrapping_add(0x2cu32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x101965b; continue;
 }
 0x101999b => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_010199a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10199a3); }
 let mut pc: u32 = 0x10199a3;
 loop { match pc {
 0x10199a3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0xffu32); }
  if cpu.zf { pc = 0x10199f9; continue; }
  pc = 0x10199bf; continue;
 }
 0x10199bf => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1136b8cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10199e0); f_01095199(cpu); if cpu.returned_to != 0x10199e0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x11d58b8u32), cpu.eax); cpu.w32(0x11d58b8u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1019a14; continue;
 }
 0x10199f9 => {
  { let v: u32 = 0x1136b9fu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x1019a0a); f_01095199(cpu); if cpu.returned_to != 0x1019a0a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x1019a14; continue;
 }
 0x1019a14 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_0101a081(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101a081); } r_0101a081(cpu, 0x101a081); }
pub fn f_0101a09a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101a09a); } r_0101a081(cpu, 0x101a09a); }
pub fn f_0101a0ca(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101a0ca); } r_0101a081(cpu, 0x101a0ca); }
pub fn f_0101a0e0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101a0e0); } r_0101a081(cpu, 0x101a0e0); }
pub fn f_0101a0f6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101a0f6); } r_0101a081(cpu, 0x101a0f6); }
pub fn f_0101a10c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101a10c); } r_0101a081(cpu, 0x101a10c); }
pub fn f_0101a122(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x101a122); } r_0101a081(cpu, 0x101a122); }
fn r_0101a081(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x101a081 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x101a0ab; continue;
 }
 0x101a09a => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xa0f60101u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | 0xa1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & cpu.r8(0x458a0101u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.df = false; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x101a136; continue; }
  pc = 0x101a0bb; continue;
 }
 0x101a0ab => {
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x4u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x101a136; continue; }
  pc = 0x101a0bb; continue;
 }
 0x101a0bb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x101a097u32.wrapping_add(cpu.eax)); continue;
 }
 0x101a0ca => {
  { let v: u32 = 0x1136eb4u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101a0db); f_01095199(cpu); if cpu.returned_to != 0x101a0db { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x101a136; continue;
 }
 0x101a0e0 => {
  { let v: u32 = 0x1136ebbu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101a0f1); f_01095199(cpu); if cpu.returned_to != 0x101a0f1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x101a136; continue;
 }
 0x101a0f6 => {
  { let v: u32 = 0x1136ec9u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101a107); f_01095199(cpu); if cpu.returned_to != 0x101a107 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x101a136; continue;
 }
 0x101a10c => {
  { let v: u32 = 0x1136ed6u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101a11d); f_01095199(cpu); if cpu.returned_to != 0x101a11d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x101a136; continue;
 }
 0x101a122 => {
  { let v: u32 = 0x1136edeu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1133ca0u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x101a133); f_01095199(cpu); if cpu.returned_to != 0x101a133 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x101a136; continue;
 }
 0x101a136 => {
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
pub fn f_0101a097(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101a097); }
 let mut pc: u32 = 0x101a097;
 loop { match pc {
 0x101a097 => {
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(416); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
