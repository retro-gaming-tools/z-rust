#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_0049a940(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49a940); }
 let mut pc: u32 = 0x49a940;
 loop { match pc {
 0x49a940 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x45u32); }
  if (cpu.sf != cpu.of) { pc = 0x49a960; continue; }
  pc = 0x49a957; continue;
 }
 0x49a957 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x8bu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49a965; continue; }
  pc = 0x49a960; continue;
 }
 0x49a960 => {
  pc = 0x49ac40; continue;
 }
 0x49a965 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.edx)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49a977; continue; }
  pc = 0x49a96e; continue;
 }
 0x49a96e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.eax)), 0xbu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49a9d6; continue; }
  pc = 0x49a977; continue;
 }
 0x49a977 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49a9c2; continue; }
  pc = 0x49a9aa; continue;
 }
 0x49a9aa => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49a9c2; continue;
 }
 0x49a9c2 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x45u32); }
  if (cpu.sf != cpu.of) { pc = 0x49a9d1; continue; }
  pc = 0x49a9c8; continue;
 }
 0x49a9c8 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x8bu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49a9d6; continue; }
  pc = 0x49a9d1; continue;
 }
 0x49a9d1 => {
  pc = 0x49ac40; continue;
 }
 0x49a9d6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5dd798u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49aa02; continue; }
  pc = 0x49a9f0; continue;
 }
 0x49a9f0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ecx)), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49aa02; continue; }
  pc = 0x49a9f9; continue;
 }
 0x49a9f9 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49aa02; continue;
 }
 0x49aa02 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x46u32); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x16du32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = 0xffffffefu32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49aa4b; continue; }
  pc = 0x49aa3f; continue;
 }
 0x49aa3f => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49aa4b; continue; }
  pc = 0x49aa45; continue;
 }
 0x49aa45 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49aa5d; continue; }
  pc = 0x49aa4b; continue;
 }
 0x49aa4b => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49aa62; continue; }
  pc = 0x49aa51; continue;
 }
 0x49aa51 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49aa62; continue; }
  pc = 0x49aa57; continue;
 }
 0x49aa57 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49aa62; continue; }
  pc = 0x49aa5d; continue;
 }
 0x49aa5d => {
  pc = 0x49ac40; continue;
 }
 0x49aa62 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, 0x18u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49aa85; continue; }
  pc = 0x49aa71; continue;
 }
 0x49aa71 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x18u32); }
  { let v: u8 = (!cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49aa8c; continue;
 }
 0x49aa85 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49aa8c; continue;
 }
 0x49aa8c => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49aa97; continue; }
  pc = 0x49aa92; continue;
 }
 0x49aa92 => {
  pc = 0x49ac40; continue;
 }
 0x49aa97 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49aabb; continue; }
  pc = 0x49aaaf; continue;
 }
 0x49aaaf => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49aabb; continue; }
  pc = 0x49aab5; continue;
 }
 0x49aab5 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49aacd; continue; }
  pc = 0x49aabb; continue;
 }
 0x49aabb => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49aad2; continue; }
  pc = 0x49aac1; continue;
 }
 0x49aac1 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49aad2; continue; }
  pc = 0x49aac7; continue;
 }
 0x49aac7 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49aad2; continue; }
  pc = 0x49aacd; continue;
 }
 0x49aacd => {
  pc = 0x49ac40; continue;
 }
 0x49aad2 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49aaf5; continue; }
  pc = 0x49aae1; continue;
 }
 0x49aae1 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x3cu32); }
  { let v: u8 = (!cpu.zf) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49aafc; continue;
 }
 0x49aaf5 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49aafc; continue;
 }
 0x49aafc => {
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49ab07; continue; }
  pc = 0x49ab02; continue;
 }
 0x49ab02 => {
  pc = 0x49ac40; continue;
 }
 0x49ab07 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49ab2b; continue; }
  pc = 0x49ab1f; continue;
 }
 0x49ab1f => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49ab2b; continue; }
  pc = 0x49ab25; continue;
 }
 0x49ab25 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49ab3d; continue; }
  pc = 0x49ab2b; continue;
 }
 0x49ab2b => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49ab42; continue; }
  pc = 0x49ab31; continue;
 }
 0x49ab31 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49ab42; continue; }
  pc = 0x49ab37; continue;
 }
 0x49ab37 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49ab42; continue; }
  pc = 0x49ab3d; continue;
 }
 0x49ab3d => {
  pc = 0x49ac40; continue;
 }
 0x49ab42 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49ab65; continue; }
  pc = 0x49ab51; continue;
 }
 0x49ab51 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x3cu32); }
  { let v: u8 = (!cpu.zf) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ab6c; continue;
 }
 0x49ab65 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ab6c; continue;
 }
 0x49ab6c => {
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49ab77; continue; }
  pc = 0x49ab72; continue;
 }
 0x49ab72 => {
  pc = 0x49ac40; continue;
 }
 0x49ab77 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49ab9a; continue; }
  pc = 0x49ab8e; continue;
 }
 0x49ab8e => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49ab9a; continue; }
  pc = 0x49ab94; continue;
 }
 0x49ab94 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49abac; continue; }
  pc = 0x49ab9a; continue;
 }
 0x49ab9a => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49abb1; continue; }
  pc = 0x49aba0; continue;
 }
 0x49aba0 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49abb1; continue; }
  pc = 0x49aba6; continue;
 }
 0x49aba6 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49abb1; continue; }
  pc = 0x49abac; continue;
 }
 0x49abac => {
  pc = 0x49ac40; continue;
 }
 0x49abb1 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49ac17; continue; }
  pc = 0x49abb7; continue;
 }
 0x49abb7 => {
  cpu.push(0x49abbc); f_0049ac50(cpu); if cpu.returned_to != 0x49abbc { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x5dd5a8u32)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49abd1); f_0049dab0(cpu); if cpu.returned_to != 0x49abd1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49abdf; continue; }
  pc = 0x49abdd; continue;
 }
 0x49abdd => {
  pc = 0x49ac40; continue;
 }
 0x49abdf => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49abfa; continue; }
  pc = 0x49abe8; continue;
 }
 0x49abe8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.edx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49ac15; continue; }
  pc = 0x49abf1; continue;
 }
 0x49abf1 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49ac15; continue; }
  pc = 0x49abfa; continue;
 }
 0x49abfa => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x5dd5b0u32)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49ac0f); f_0049dab0(cpu); if cpu.returned_to != 0x49ac0f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ac15; continue;
 }
 0x49ac15 => {
  pc = 0x49ac2e; continue;
 }
 0x49ac17 => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49ac20); f_0049d8e0(cpu); if cpu.returned_to != 0x49ac20 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49ac2e; continue; }
  pc = 0x49ac2c; continue;
 }
 0x49ac2c => {
  pc = 0x49ac40; continue;
 }
 0x49ac2e => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49ac43; continue;
 }
 0x49ac40 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49ac43; continue;
 }
 0x49ac43 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ac50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ac50); }
 let mut pc: u32 = 0x49ac50;
 loop { match pc {
 0x49ac50 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x5dee78u32), 0x0u32); }
  if !cpu.zf { pc = 0x49ac77; continue; }
  pc = 0x49ac5c; continue;
 }
 0x49ac5c => {
  { cpu.sub32(cpu.r32(0x5dee78u32), 0x0u32); }
  if !cpu.zf { pc = 0x49ac77; continue; }
  pc = 0x49ac65; continue;
 }
 0x49ac65 => {
  cpu.push(0x49ac6a); f_0049ac80(cpu); if cpu.returned_to != 0x49ac6a { return; }
  { let v: u32 = cpu.r32(0x5dee78u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5dee78u32, v); }
  pc = 0x49ac77; continue;
 }
 0x49ac77 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ac80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ac80); }
 let mut pc: u32 = 0x49ac80;
 loop { match pc {
 0x49ac80 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5dedc0u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x5dd650u32, v); }
  { let v: u32 = cpu.r32(0x5dd650u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5dd640u32, v); }
  { let v = 0x4a26a0u32; cpu.push(v); }
  cpu.push(0x49acb5); f_0049e1f0(cpu); if cpu.returned_to != 0x49acb5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49ada2; continue; }
  pc = 0x49acc5; continue;
 }
 0x49acc5 => {
  { let v = 0x5dedc8u32; cpu.push(v); }
  let t = cpu.r32(0x4a10a4u32); cpu.push(0x49acd0); cpu.call(t); if cpu.returned_to != 0x49acd0 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x49ad9d; continue; }
  pc = 0x49acd9; continue;
 }
 0x49acd9 => {
  { let v: u32 = 0x1u32; cpu.w32(0x5dedc0u32, v); }
  { let v: u32 = cpu.r32(0x5dedc8u32); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, 0x3cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5dd5a8u32, v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5dee0eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49ad15; continue; }
  pc = 0x49acff; continue;
 }
 0x49acff => {
  { let v: u32 = cpu.r32(0x5dee1cu32); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5dd5a8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5dd5a8u32, v); }
  pc = 0x49ad15; continue;
 }
 0x49ad15 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5dee62u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49ad4a; continue; }
  pc = 0x49ad22; continue;
 }
 0x49ad22 => {
  { cpu.sub32(cpu.r32(0x5dee70u32), 0x0u32); }
  if cpu.zf { pc = 0x49ad4a; continue; }
  pc = 0x49ad2b; continue;
 }
 0x49ad2b => {
  { let v: u32 = 0x1u32; cpu.w32(0x5dd5acu32, v); }
  { let v: u32 = cpu.r32(0x5dee70u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5dee1cu32)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5dd5b0u32, v); }
  pc = 0x49ad5e; continue;
 }
 0x49ad4a => {
  { let v: u32 = 0x0u32; cpu.w32(0x5dd5acu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5dd5b0u32, v); }
  pc = 0x49ad5e; continue;
 }
 0x49ad5e => {
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0x5dedccu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dd638u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49ad71); f_0049dea0(cpu); if cpu.returned_to != 0x49ad71 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0x5dee20u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dd63cu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49ad87); f_0049dea0(cpu); if cpu.returned_to != 0x49ad87 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x5dd63cu32); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x3fu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x5dd638u32); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x3fu32.wrapping_add(cpu.ecx), v); }
  pc = 0x49ad9d; continue;
 }
 0x49ad9d => {
  pc = 0x49afa8; continue;
 }
 0x49ada2 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49adcc; continue; }
  pc = 0x49adac; continue;
 }
 0x49adac => {
  { cpu.sub32(cpu.r32(0x5dee74u32), 0x0u32); }
  if cpu.zf { pc = 0x49add1; continue; }
  pc = 0x49adb5; continue;
 }
 0x49adb5 => {
  { let v: u32 = cpu.r32(0x5dee74u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49adc5); f_0049de10(cpu); if cpu.returned_to != 0x49adc5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49add1; continue; }
  pc = 0x49adcc; continue;
 }
 0x49adcc => {
  pc = 0x49afa8; continue;
 }
 0x49add1 => {
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dee74u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49adde); f_0048ecf0(cpu); if cpu.returned_to != 0x49adde { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xecu32; cpu.push(v); }
  { let v = 0x4a2698u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49adf6); f_004936f0(cpu); if cpu.returned_to != 0x49adf6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49ae02); f_0048e380(cpu); if cpu.returned_to != 0x49ae02 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5dee74u32, v); }
  { cpu.sub32(cpu.r32(0x5dee74u32), 0x0u32); }
  if !cpu.zf { pc = 0x49ae18; continue; }
  pc = 0x49ae13; continue;
 }
 0x49ae13 => {
  pc = 0x49afa8; continue;
 }
 0x49ae18 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dee74u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49ae27); f_00493770(cpu); if cpu.returned_to != 0x49ae27 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dd638u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49ae3c); f_004935f0(cpu); if cpu.returned_to != 0x49ae3c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x5dd638u32); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x3u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x3u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x49ae6e; continue; }
  pc = 0x49ae5c; continue;
 }
 0x49ae5c => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ae6e; continue;
 }
 0x49ae6e => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49ae77); f_0049dd10(cpu); if cpu.returned_to != 0x49ae77 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xe10u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5dd5a8u32, v); }
  pc = 0x49ae85; continue;
 }
 0x49ae85 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x2bu32); }
  if cpu.zf { pc = 0x49aea6; continue; }
  pc = 0x49ae90; continue;
 }
 0x49ae90 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x49aeb1; continue; }
  pc = 0x49ae9b; continue;
 }
 0x49ae9b => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49aeb1; continue; }
  pc = 0x49aea6; continue;
 }
 0x49aea6 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49ae85; continue;
 }
 0x49aeb1 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3au32); }
  if !cpu.zf { pc = 0x49af56; continue; }
  pc = 0x49aec0; continue;
 }
 0x49aec0 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49aed2); f_0049dd10(cpu); if cpu.returned_to != 0x49aed2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5dd5a8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5dd5a8u32, v); }
  pc = 0x49aee6; continue;
 }
 0x49aee6 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x49af07; continue; }
  pc = 0x49aef1; continue;
 }
 0x49aef1 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49af07; continue; }
  pc = 0x49aefc; continue;
 }
 0x49aefc => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49aee6; continue;
 }
 0x49af07 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x49af56; continue; }
  pc = 0x49af12; continue;
 }
 0x49af12 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49af24); f_0049dd10(cpu); if cpu.returned_to != 0x49af24 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x5dd5a8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5dd5a8u32, v); }
  pc = 0x49af35; continue;
 }
 0x49af35 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x49af56; continue; }
  pc = 0x49af40; continue;
 }
 0x49af40 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49af56; continue; }
  pc = 0x49af4b; continue;
 }
 0x49af4b => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49af35; continue;
 }
 0x49af56 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49af6a; continue; }
  pc = 0x49af5c; continue;
 }
 0x49af5c => {
  { let v: u32 = cpu.r32(0x5dd5a8u32); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5dd5a8u32, v); }
  pc = 0x49af6a; continue;
 }
 0x49af6a => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5dd5acu32, v); }
  { cpu.sub32(cpu.r32(0x5dd5acu32), 0x0u32); }
  if cpu.zf { pc = 0x49af9f; continue; }
  pc = 0x49af7f; continue;
 }
 0x49af7f => {
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dd63cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49af90); f_004935f0(cpu); if cpu.returned_to != 0x49af90 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x5dd63cu32); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x3u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49afa8; continue;
 }
 0x49af9f => {
  { let v: u32 = cpu.r32(0x5dd63cu32); cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x49afa8; continue;
 }
 0x49afa8 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049afb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49afb0); }
 let mut pc: u32 = 0x49afb0;
 loop { match pc {
 0x49afb0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x5dd5acu32), 0x0u32); }
  if !cpu.zf { pc = 0x49afc4; continue; }
  pc = 0x49afbd; continue;
 }
 0x49afbd => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49b2a6; continue;
 }
 0x49afc4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x5dd640u32)); }
  if !cpu.zf { pc = 0x49afe4; continue; }
  pc = 0x49afd2; continue;
 }
 0x49afd2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5dd650u32)); }
  if cpu.zf { pc = 0x49b1ab; continue; }
  pc = 0x49afe4; continue;
 }
 0x49afe4 => {
  { cpu.sub32(cpu.r32(0x5dedc0u32), 0x0u32); }
  if cpu.zf { pc = 0x49b165; continue; }
  pc = 0x49aff1; continue;
 }
 0x49aff1 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x5dee60u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x49b059; continue; }
  pc = 0x49affe; continue;
 }
 0x49affe => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5dee6eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5dee6cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x5dee6au32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5dee68u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5dee64u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x5dee66u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5dee62u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x49b054); f_0049b2b0(cpu); if cpu.returned_to != 0x49b054 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  pc = 0x49b0aa; continue;
 }
 0x49b059 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5dee6eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5dee6cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x5dee6au32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5dee68u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5dee66u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x5dee62u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x49b0a7); f_0049b2b0(cpu); if cpu.returned_to != 0x49b0a7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  pc = 0x49b0aa; continue;
 }
 0x49b0aa => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x5dee0cu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x49b112; continue; }
  pc = 0x49b0b7; continue;
 }
 0x49b0b7 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5dee1au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5dee18u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x5dee16u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5dee14u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5dee10u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x5dee12u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5dee0eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x49b10d); f_0049b2b0(cpu); if cpu.returned_to != 0x49b10d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  pc = 0x49b163; continue;
 }
 0x49b112 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5dee1au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5dee18u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x5dee16u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5dee14u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5dee12u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x5dee0eu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x49b160); f_0049b2b0(cpu); if cpu.returned_to != 0x49b160 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  pc = 0x49b163; continue;
 }
 0x49b163 => {
  pc = 0x49b1ab; continue;
 }
 0x49b165 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x49b185); f_0049b2b0(cpu); if cpu.returned_to != 0x49b185 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x49b1a8); f_0049b2b0(cpu); if cpu.returned_to != 0x49b1a8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  pc = 0x49b1ab; continue;
 }
 0x49b1ab => {
  { let v: u32 = cpu.r32(0x5dd644u32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x5dd654u32)); }
  if (cpu.sf == cpu.of) { pc = 0x49b204; continue; }
  pc = 0x49b1b9; continue;
 }
 0x49b1b9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x5dd644u32)); }
  if (cpu.sf != cpu.of) { pc = 0x49b1d5; continue; }
  pc = 0x49b1c7; continue;
 }
 0x49b1c7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5dd654u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49b1dc; continue; }
  pc = 0x49b1d5; continue;
 }
 0x49b1d5 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49b2a6; continue;
 }
 0x49b1dc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x5dd644u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49b202; continue; }
  pc = 0x49b1ea; continue;
 }
 0x49b1ea => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x5dd654u32)); }
  if (cpu.sf == cpu.of) { pc = 0x49b202; continue; }
  pc = 0x49b1f8; continue;
 }
 0x49b1f8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49b2a6; continue;
 }
 0x49b202 => {
  pc = 0x49b247; continue;
 }
 0x49b204 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5dd654u32)); }
  if (cpu.sf != cpu.of) { pc = 0x49b220; continue; }
  pc = 0x49b212; continue;
 }
 0x49b212 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x5dd644u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49b227; continue; }
  pc = 0x49b220; continue;
 }
 0x49b220 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49b2a6; continue;
 }
 0x49b227 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x5dd654u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49b247; continue; }
  pc = 0x49b235; continue;
 }
 0x49b235 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5dd644u32)); }
  if (cpu.sf == cpu.of) { pc = 0x49b247; continue; }
  pc = 0x49b243; continue;
 }
 0x49b243 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49b2a6; continue;
 }
 0x49b247 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x3cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xe10u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, 0x3e8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x5dd644u32)); }
  if !cpu.zf { pc = 0x49b292; continue; }
  pc = 0x49b27c; continue;
 }
 0x49b27c => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5dd648u32)); }
  if (cpu.sf != cpu.of) { pc = 0x49b28e; continue; }
  pc = 0x49b287; continue;
 }
 0x49b287 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49b2a6; continue;
 }
 0x49b28e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49b2a6; continue;
 }
 0x49b292 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x5dd658u32)); }
  if (cpu.sf == cpu.of) { pc = 0x49b2a4; continue; }
  pc = 0x49b29d; continue;
 }
 0x49b29d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49b2a6; continue;
 }
 0x49b2a4 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49b2a6; continue;
 }
 0x49b2a6 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b2b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b2b0); }
 let mut pc: u32 = 0x49b2b0;
 loop { match pc {
 0x49b2b0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x1u32); }
  if !cpu.zf { pc = 0x49b38c; continue; }
  pc = 0x49b2c0; continue;
 }
 0x49b2c0 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49b2d9; continue; }
  pc = 0x49b2ca; continue;
 }
 0x49b2ca => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5dd75cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b2e6; continue;
 }
 0x49b2d9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5dd794u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b2e6; continue;
 }
 0x49b2e6 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x46u32); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x16du32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = 0xfffffff3u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x49b339; continue; }
  pc = 0x49b320; continue;
 }
 0x49b320 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, 0x7u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b34d; continue;
 }
 0x49b339 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x7u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b34d; continue;
 }
 0x49b34d => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebp)), 0x5u32); }
  if !cpu.zf { pc = 0x49b38a; continue; }
  pc = 0x49b353; continue;
 }
 0x49b353 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x49b36c; continue; }
  pc = 0x49b35d; continue;
 }
 0x49b35d => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5dd760u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b379; continue;
 }
 0x49b36c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5dd798u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b379; continue;
 }
 0x49b379 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49b38a; continue; }
  pc = 0x49b381; continue;
 }
 0x49b381 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x7u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b38a; continue;
 }
 0x49b38a => {
  pc = 0x49b3c1; continue;
 }
 0x49b38c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x3u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x49b3a5; continue; }
  pc = 0x49b396; continue;
 }
 0x49b396 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5dd75cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b3b2; continue;
 }
 0x49b3a5 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5dd794u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b3b2; continue;
 }
 0x49b3b2 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b3c1; continue;
 }
 0x49b3c1 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if !cpu.zf { pc = 0x49b401; continue; }
  pc = 0x49b3c7; continue;
 }
 0x49b3c7 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5dd644u32, v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, 0x3cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x3cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5dd648u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5dd640u32, v); }
  pc = 0x49b4a4; continue;
 }
 0x49b401 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5dd654u32, v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, 0x3cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x3cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x3e8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5dd658u32, v); }
  { let v: u32 = cpu.r32(0x5dd5b0u32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x3e8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5dd658u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5dd658u32, v); }
  { cpu.sub32(cpu.r32(0x5dd658u32), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49b471; continue; }
  pc = 0x49b44e; continue;
 }
 0x49b44e => {
  { let v: u32 = cpu.r32(0x5dd658u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x5265c00u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5dd658u32, v); }
  { let v: u32 = cpu.r32(0x5dd658u32); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, 0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5dd658u32, v); }
  pc = 0x49b49b; continue;
 }
 0x49b471 => {
  { cpu.sub32(cpu.r32(0x5dd658u32), 0x5265c00u32); }
  if (cpu.sf != cpu.of) { pc = 0x49b49b; continue; }
  pc = 0x49b47d; continue;
 }
 0x49b47d => {
  { let v: u32 = cpu.r32(0x5dd658u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x5265c00u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5dd658u32, v); }
  { let v: u32 = cpu.r32(0x5dd658u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5dd658u32, v); }
  pc = 0x49b49b; continue;
 }
 0x49b49b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5dd650u32, v); }
  pc = 0x49b4a4; continue;
 }
 0x49b4a4 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b4b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b4b0); }
 let mut pc: u32 = 0x49b4b0;
 loop { match pc {
 0x49b4b0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x5dee80u32), 0x0u32); }
  if !cpu.zf { pc = 0x49b50e; continue; }
  pc = 0x49b4bf; continue;
 }
 0x49b4bf => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4a26b4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  let t = cpu.r32(0x4a10a0u32); cpu.push(0x49b4d2); cpu.call(t); if cpu.returned_to != 0x49b4d2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49b4e2; continue; }
  pc = 0x49b4d6; continue;
 }
 0x49b4d6 => {
  { let v: u32 = 0x1u32; cpu.w32(0x5dee80u32, v); }
  pc = 0x49b50e; continue;
 }
 0x49b4e2 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4a26b0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x4a1098u32); cpu.push(0x49b4f7); cpu.call(t); if cpu.returned_to != 0x49b4f7 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49b507; continue; }
  pc = 0x49b4fb; continue;
 }
 0x49b4fb => {
  { let v: u32 = 0x2u32; cpu.w32(0x5dee80u32, v); }
  pc = 0x49b50e; continue;
 }
 0x49b507 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49b614; continue;
 }
 0x49b50e => {
  { cpu.sub32(cpu.r32(0x5dee80u32), 0x2u32); }
  if !cpu.zf { pc = 0x49b545; continue; }
  pc = 0x49b517; continue;
 }
 0x49b517 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49b526; continue; }
  pc = 0x49b51d; continue;
 }
 0x49b51d => {
  { let v: u32 = cpu.r32(0x5dee90u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b526; continue;
 }
 0x49b526 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a1098u32); cpu.push(0x49b540); cpu.call(t); if cpu.returned_to != 0x49b540 { return; }
  pc = 0x49b614; continue;
 }
 0x49b545 => {
  { cpu.sub32(cpu.r32(0x5dee80u32), 0x1u32); }
  if !cpu.zf { pc = 0x49b614; continue; }
  pc = 0x49b552; continue;
 }
 0x49b552 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49b56f; continue; }
  pc = 0x49b566; continue;
 }
 0x49b566 => {
  { let v: u32 = cpu.r32(0x5deea0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b56f; continue;
 }
 0x49b56f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10c8u32); cpu.push(0x49b593); cpu.call(t); if cpu.returned_to != 0x49b593 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49b59e; continue; }
  pc = 0x49b59c; continue;
 }
 0x49b59c => {
  pc = 0x49b603; continue;
 }
 0x49b59e => {
  { let v = 0x10eu32; cpu.push(v); }
  { let v = 0x4a26a4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x49b5b5); f_0048e720(cpu); if cpu.returned_to != 0x49b5b5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49b5c3; continue; }
  pc = 0x49b5c1; continue;
 }
 0x49b5c1 => {
  pc = 0x49b603; continue;
 }
 0x49b5c3 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10c8u32); cpu.push(0x49b5df); cpu.call(t); if cpu.returned_to != 0x49b5df { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49b5ea; continue; }
  pc = 0x49b5e8; continue;
 }
 0x49b5e8 => {
  pc = 0x49b603; continue;
 }
 0x49b5ea => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a10a0u32); cpu.push(0x49b600); cpu.call(t); if cpu.returned_to != 0x49b600 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b603; continue;
 }
 0x49b603 => {
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49b60e); f_0048ecf0(cpu); if cpu.returned_to != 0x49b60e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49b614; continue;
 }
 0x49b614 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b620); }
 let mut pc: u32 = 0x49b620;
 loop { match pc {
 0x49b620 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b62a; continue;
 }
 0x49b62a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49b641; continue; }
  pc = 0x49b63f; continue;
 }
 0x49b63f => {
  cpu.poll(); pc = 0x49b62a; continue;
 }
 0x49b641 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b650); }
 let mut pc: u32 = 0x49b650;
 loop { match pc {
 0x49b650 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x5deea8u32), 0x0u32); }
  if !cpu.zf { pc = 0x49b6b6; continue; }
  pc = 0x49b65f; continue;
 }
 0x49b65f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4a26b4u32; cpu.push(v); }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x4a109cu32); cpu.push(0x49b677); cpu.call(t); if cpu.returned_to != 0x49b677 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49b687; continue; }
  pc = 0x49b67b; continue;
 }
 0x49b67b => {
  { let v: u32 = 0x1u32; cpu.w32(0x5deea8u32, v); }
  pc = 0x49b6b6; continue;
 }
 0x49b687 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4a26b0u32; cpu.push(v); }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x4a10f4u32); cpu.push(0x49b69f); cpu.call(t); if cpu.returned_to != 0x49b69f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49b6af; continue; }
  pc = 0x49b6a3; continue;
 }
 0x49b6a3 => {
  { let v: u32 = 0x2u32; cpu.w32(0x5deea8u32, v); }
  pc = 0x49b6b6; continue;
 }
 0x49b6af => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49b910; continue;
 }
 0x49b6b6 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49b6cf; continue; }
  pc = 0x49b6bc; continue;
 }
 0x49b6bc => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49b6c9); f_0049b920(cpu); if cpu.returned_to != 0x49b6c9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b6cf; continue;
 }
 0x49b6cf => {
  { cpu.sub32(cpu.r32(0x5deea8u32), 0x2u32); }
  if !cpu.zf { pc = 0x49b6fb; continue; }
  pc = 0x49b6d8; continue;
 }
 0x49b6d8 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a10f4u32); cpu.push(0x49b6f6); cpu.call(t); if cpu.returned_to != 0x49b6f6 { return; }
  pc = 0x49b910; continue;
 }
 0x49b6fb => {
  { cpu.sub32(cpu.r32(0x5deea8u32), 0x1u32); }
  if !cpu.zf { pc = 0x49b910; continue; }
  pc = 0x49b708; continue;
 }
 0x49b708 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49b725; continue; }
  pc = 0x49b71c; continue;
 }
 0x49b71c => {
  { let v: u32 = cpu.r32(0x5deea0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b725; continue;
 }
 0x49b725 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10c8u32); cpu.push(0x49b749); cpu.call(t); if cpu.returned_to != 0x49b749 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49b759; continue; }
  pc = 0x49b752; continue;
 }
 0x49b752 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49b910; continue;
 }
 0x49b759 => {
  { let v = 0x174u32; cpu.push(v); }
  { let v = 0x4a26b8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49b770); f_0048e380(cpu); if cpu.returned_to != 0x49b770 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49b783; continue; }
  pc = 0x49b77c; continue;
 }
 0x49b77c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49b910; continue;
 }
 0x49b783 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10c8u32); cpu.push(0x49b79f); cpu.call(t); if cpu.returned_to != 0x49b79f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49b7a8; continue; }
  pc = 0x49b7a3; continue;
 }
 0x49b7a3 => {
  pc = 0x49b8f2; continue;
 }
 0x49b7a8 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a109cu32); cpu.push(0x49b7c2); cpu.call(t); if cpu.returned_to != 0x49b7c2 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49b7d0; continue; }
  pc = 0x49b7cb; continue;
 }
 0x49b7cb => {
  pc = 0x49b8f2; continue;
 }
 0x49b7d0 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x400u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49b81c; continue; }
  pc = 0x49b7dd; continue;
 }
 0x49b7dd => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49b817; continue; }
  pc = 0x49b7e3; continue;
 }
 0x49b7e3 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49b7f0; continue; }
  pc = 0x49b7eb; continue;
 }
 0x49b7eb => {
  pc = 0x49b8f2; continue;
 }
 0x49b7f0 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a109cu32); cpu.push(0x49b80e); cpu.call(t); if cpu.returned_to != 0x49b80e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49b817; continue; }
  pc = 0x49b812; continue;
 }
 0x49b812 => {
  pc = 0x49b8f2; continue;
 }
 0x49b817 => {
  pc = 0x49b8d1; continue;
 }
 0x49b81c => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v = 0x198u32; cpu.push(v); }
  { let v = 0x4a26b8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49b839); f_0048e380(cpu); if cpu.returned_to != 0x49b839 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49b84a; continue; }
  pc = 0x49b845; continue;
 }
 0x49b845 => {
  pc = 0x49b8f2; continue;
 }
 0x49b84a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a109cu32); cpu.push(0x49b868); cpu.call(t); if cpu.returned_to != 0x49b868 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49b871; continue; }
  pc = 0x49b86c; continue;
 }
 0x49b86c => {
  pc = 0x49b8f2; continue;
 }
 0x49b871 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49b8a3; continue; }
  pc = 0x49b877; continue;
 }
 0x49b877 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x220u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a10c4u32); cpu.push(0x49b896); cpu.call(t); if cpu.returned_to != 0x49b896 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49b8a1; continue; }
  pc = 0x49b89f; continue;
 }
 0x49b89f => {
  pc = 0x49b8f2; continue;
 }
 0x49b8a1 => {
  pc = 0x49b8d1; continue;
 }
 0x49b8a3 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x220u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a10c4u32); cpu.push(0x49b8c6); cpu.call(t); if cpu.returned_to != 0x49b8c6 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49b8d1; continue; }
  pc = 0x49b8cf; continue;
 }
 0x49b8cf => {
  pc = 0x49b8f2; continue;
 }
 0x49b8d1 => {
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49b8dc); f_0048ecf0(cpu); if cpu.returned_to != 0x49b8dc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49b8ea); f_0048ecf0(cpu); if cpu.returned_to != 0x49b8ea { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49b910; continue;
 }
 0x49b8f2 => {
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49b8fd); f_0048ecf0(cpu); if cpu.returned_to != 0x49b8fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49b90b); f_0048ecf0(cpu); if cpu.returned_to != 0x49b90b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49b910; continue;
 }
 0x49b910 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b920(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b920); }
 let mut pc: u32 = 0x49b920;
 loop { match pc {
 0x49b920 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49b932; continue;
 }
 0x49b932 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49b957; continue; }
  pc = 0x49b942; continue;
 }
 0x49b942 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49b957; continue; }
  pc = 0x49b94c; continue;
 }
 0x49b94c => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49b932; continue;
 }
 0x49b957 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x49b969; continue; }
  pc = 0x49b961; continue;
 }
 0x49b961 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  pc = 0x49b96c; continue;
 }
 0x49b969 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49b96c; continue;
 }
 0x49b96c => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049b970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49b970); }
 let mut pc: u32 = 0x49b970;
 loop { match pc {
 0x49b970 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = 0x1024u32; cpu.eax = v; }
  cpu.push(0x49b97d); f_00497dc0(cpu); if cpu.returned_to != 0x49b97d { return; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0xffffeffcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffefecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffefe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x5f2060u32)); }
  if !cpu.cf { pc = 0x49b9c0; continue; }
  pc = 0x49b9a1; continue;
 }
 0x49b9a1 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5f1f60u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(8))) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x49b9d2; continue; }
  pc = 0x49b9c0; continue;
 }
 0x49b9c0 => {
  { let v: u32 = 0x9u32; cpu.w32(0x5de9bcu32, v); }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49bbce; continue;
 }
 0x49b9d2 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49b9f9; continue; }
  pc = 0x49b9d8; continue;
 }
 0x49b9d8 => {
  { let v = 0x4a26d0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x81u32; cpu.push(v); }
  { let v = 0x4a26c4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x49b9f0); f_00490280(cpu); if cpu.returned_to != 0x49b9f0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x49b9f9; continue; }
  pc = 0x49b9f8; continue;
 }
 0x49b9f8 => {
  cpu.unimplemented(0x49b9f8, "Int3");
  pc = 0x49b9f9; continue;
 }
 0x49b9f9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x49b9d2; continue; }
  pc = 0x49b9ff; continue;
 }
 0x49b9ff => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49ba0c); f_0048fff0(cpu); if cpu.returned_to != 0x49ba0c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffeff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffeff4u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if cpu.zf { pc = 0x49ba3d; continue; }
  pc = 0x49ba1e; continue;
 }
 0x49ba1e => {
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49ba2b); f_0048fff0(cpu); if cpu.returned_to != 0x49ba2b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffeff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffeff0u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x49ba45; continue; }
  pc = 0x49ba3d; continue;
 }
 0x49ba3d => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49bbce; continue;
 }
 0x49ba45 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffeff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffefe8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffefe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49bb5a; continue; }
  pc = 0x49ba61; continue;
 }
 0x49ba61 => {
  { let v = 0x1000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffefecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49ba74); f_00494ec0(cpu); if cpu.returned_to != 0x49ba74 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x8000u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49ba85); f_0049e2b0(cpu); if cpu.returned_to != 0x49ba85 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ba8b; continue;
 }
 0x49ba8b => {
  { cpu.sub32(cpu.r32(0xffffefe8u32.wrapping_add(cpu.ebp)), 0x1000u32); }
  if (cpu.sf != cpu.of) { pc = 0x49baa3; continue; }
  pc = 0x49ba97; continue;
 }
 0x49ba97 => {
  { let v: u32 = 0x1000u32; cpu.w32(0xffffefe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49baaf; continue;
 }
 0x49baa3 => {
  { let v: u32 = cpu.r32(0xffffefe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffefe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49baaf; continue;
 }
 0x49baaf => {
  { let v: u32 = cpu.r32(0xffffefe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffeff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffefe8u32.wrapping_add(cpu.ebp)), 0x1000u32); }
  if (cpu.sf != cpu.of) { pc = 0x49bad3; continue; }
  pc = 0x49bac7; continue;
 }
 0x49bac7 => {
  { let v: u32 = 0x1000u32; cpu.w32(0xffffefdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49badf; continue;
 }
 0x49bad3 => {
  { let v: u32 = cpu.r32(0xffffefe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffefdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49badf; continue;
 }
 0x49badf => {
  { let v: u32 = cpu.r32(0xffffefdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffefecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49baf6); f_00490e50(cpu); if cpu.returned_to != 0x49baf6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffeff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffeff8u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x49bb29; continue; }
  pc = 0x49bb08; continue;
 }
 0x49bb08 => {
  { cpu.sub32(cpu.r32(0x5de9c0u32), 0x5u32); }
  if !cpu.zf { pc = 0x49bb1b; continue; }
  pc = 0x49bb11; continue;
 }
 0x49bb11 => {
  { let v: u32 = 0xdu32; cpu.w32(0x5de9bcu32, v); }
  pc = 0x49bb1b; continue;
 }
 0x49bb1b => {
  { let v: u32 = cpu.r32(0xffffeff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffefe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bb48; continue;
 }
 0x49bb29 => {
  { let v: u32 = cpu.r32(0xffffefe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0xffffeff8u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffefe8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffefe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x49ba8b; continue; }
  pc = 0x49bb48; continue;
 }
 0x49bb48 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49bb55); f_0049e2b0(cpu); if cpu.returned_to != 0x49bb55 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x49bbb3; continue;
 }
 0x49bb5a => {
  { cpu.sub32(cpu.r32(0xffffefe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49bbb3; continue; }
  pc = 0x49bb63; continue;
 }
 0x49bb63 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49bb72); f_0048fff0(cpu); if cpu.returned_to != 0x49bb72 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49bb7e); f_00497b60(cpu); if cpu.returned_to != 0x49bb7e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10dcu32); cpu.push(0x49bb88); cpu.call(t); if cpu.returned_to != 0x49bb88 { return; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffefe4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffefe4u32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x49bbb3; continue; }
  pc = 0x49bb9e; continue;
 }
 0x49bb9e => {
  { let v: u32 = 0xdu32; cpu.w32(0x5de9bcu32, v); }
  let t = cpu.r32(0x4a1170u32); cpu.push(0x49bbae); cpu.call(t); if cpu.returned_to != 0x49bbae { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de9c0u32, v); }
  pc = 0x49bbb3; continue;
 }
 0x49bbb3 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffeff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49bbc5); f_0048fff0(cpu); if cpu.returned_to != 0x49bbc5 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffefe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49bbce; continue;
 }
 0x49bbce => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bbe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bbe0); }
 let mut pc: u32 = 0x49bbe0;
 loop { match pc {
 0x49bbe0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x8u32.wrapping_add(cpu.ebp))); }
  if cpu.cf { pc = 0x49bc06; continue; }
  pc = 0x49bbfe; continue;
 }
 0x49bbfe => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x49bc0f; continue; }
  pc = 0x49bc06; continue;
 }
 0x49bc06 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bc0f; continue;
 }
 0x49bc0f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bc20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bc20); }
 let mut pc: u32 = 0x49bc20;
 loop { match pc {
 0x49bc20 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49bc3b); f_0049bbe0(cpu); if cpu.returned_to != 0x49bc3b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49bc77; continue; }
  pc = 0x49bc47; continue;
 }
 0x49bc47 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49bc5c); f_0049bbe0(cpu); if cpu.returned_to != 0x49bc5c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49bc77; continue; }
  pc = 0x49bc68; continue;
 }
 0x49bc68 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49bc77; continue;
 }
 0x49bc77 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49bc91); f_0049bbe0(cpu); if cpu.returned_to != 0x49bc91 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49bcac; continue; }
  pc = 0x49bc9d; continue;
 }
 0x49bc9d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x49bcac; continue;
 }
 0x49bcac => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49bcc6); f_0049bbe0(cpu); if cpu.returned_to != 0x49bcc6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bcd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bcd0); }
 let mut pc: u32 = 0x49bcd0;
 loop { match pc {
 0x49bcd0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x80000000u32; cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x80000000u32; cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bd30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bd30); }
 let mut pc: u32 = 0x49bd30;
 loop { match pc {
 0x49bd30 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x80000000u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x80000000u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bda0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bda0); }
 let mut pc: u32 = 0x49bda0;
 loop { match pc {
 0x49bda0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u16 = 0x404eu16; cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  pc = 0x49bddd; continue;
 }
 0x49bdcb => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bddd; continue;
 }
 0x49bddd => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x49be57; continue; }
  pc = 0x49bde3; continue;
 }
 0x49bde3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49be00); f_0049bcd0(cpu); if cpu.returned_to != 0x49be00 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49be0c); f_0049bcd0(cpu); if cpu.returned_to != 0x49be0c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49be1c); f_0049bc20(cpu); if cpu.returned_to != 0x49be1c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49be28); f_0049bcd0(cpu); if cpu.returned_to != 0x49be28 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49be4f); f_0049bc20(cpu); if cpu.returned_to != 0x49be4f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  cpu.poll(); pc = 0x49bdcb; continue;
 }
 0x49be57 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x49bea3; continue; }
  pc = 0x49be60; continue;
 }
 0x49be60 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx | cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x10u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49be57; continue;
 }
 0x49bea3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x8000u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49becc; continue; }
  pc = 0x49beb2; continue;
 }
 0x49beb2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49bebb); f_0049bcd0(cpu); if cpu.returned_to != 0x49bebb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0x1u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49bea3; continue;
 }
 0x49becc => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049bee0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49bee0); }
 let mut pc: u32 = 0x49bee0;
 loop { match pc {
 0x49bee0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xb4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bf59; continue;
 }
 0x49bf50 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49bf59; continue;
 }
 0x49bf59 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x20u32); }
  if cpu.zf { pc = 0x49bf85; continue; }
  pc = 0x49bf64; continue;
 }
 0x49bf64 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x9u32); }
  if cpu.zf { pc = 0x49bf85; continue; }
  pc = 0x49bf6f; continue;
 }
 0x49bf6f => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if cpu.zf { pc = 0x49bf85; continue; }
  pc = 0x49bf7a; continue;
 }
 0x49bf7a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xdu32); }
  if !cpu.zf { pc = 0x49bf87; continue; }
  pc = 0x49bf85; continue;
 }
 0x49bf85 => {
  cpu.poll(); pc = 0x49bf50; continue;
 }
 0x49bf87 => {
  { cpu.sub32(cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)), 0xau32); }
  if cpu.zf { pc = 0x49c6e8; continue; }
  pc = 0x49bf91; continue;
 }
 0x49bf91 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff80u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)), 0xbu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x49c6e3; continue; }
  pc = 0x49bfb2; continue;
 }
 0x49bfb2 => {
  { let v: u32 = cpu.r32(0xffffff80u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.poll(); pc = cpu.r32(0x49c8fau32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x49bfbc => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x31u32); }
  if (cpu.sf != cpu.of) { pc = 0x49bfe0; continue; }
  pc = 0x49bfc5; continue;
 }
 0x49bfc5 => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49bfe0; continue; }
  pc = 0x49bfce; continue;
 }
 0x49bfce => {
  { let v: u32 = 0x3u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c055; continue;
 }
 0x49bfe0 => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r8(0x5dd070u32) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.zf { pc = 0x49bff8; continue; }
  pc = 0x49bfef; continue;
 }
 0x49bfef => {
  { let v: u32 = 0x5u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c055; continue;
 }
 0x49bff8 => {
  { let v: u8 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xffffff7cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xffffff7cu32.wrapping_add(cpu.ebp)), 0x2bu8); }
  if cpu.zf { pc = 0x49c027; continue; }
  pc = 0x49c00a; continue;
 }
 0x49c00a => {
  { cpu.sub8(cpu.r8(0xffffff7cu32.wrapping_add(cpu.ebp)), 0x2du8); }
  if cpu.zf { pc = 0x49c036; continue; }
  pc = 0x49c013; continue;
 }
 0x49c013 => {
  { cpu.sub8(cpu.r8(0xffffff7cu32.wrapping_add(cpu.ebp)), 0x30u8); }
  if cpu.zf { pc = 0x49c01e; continue; }
  pc = 0x49c01c; continue;
 }
 0x49c01c => {
  pc = 0x49c045; continue;
 }
 0x49c01e => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c055; continue;
 }
 0x49c027 => {
  { let v: u32 = 0x2u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c055; continue;
 }
 0x49c036 => {
  { let v: u32 = 0x2u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x8000u16; cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c055; continue;
 }
 0x49c045 => {
  { let v: u32 = 0xau32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c055; continue;
 }
 0x49c055 => {
  pc = 0x49c6e3; continue;
 }
 0x49c05a => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x31u32); }
  if (cpu.sf != cpu.of) { pc = 0x49c088; continue; }
  pc = 0x49c06a; continue;
 }
 0x49c06a => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49c088; continue; }
  pc = 0x49c073; continue;
 }
 0x49c073 => {
  { let v: u32 = 0x3u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c10b; continue;
 }
 0x49c088 => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x5dd070u32) as i8 as i32 as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x49c0a0; continue; }
  pc = 0x49c097; continue;
 }
 0x49c097 => {
  { let v: u32 = 0x4u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c10b; continue;
 }
 0x49c0a0 => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff78u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff78u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2bu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff78u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff78u32.wrapping_add(cpu.ebp)), 0x3au32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x49c0fb; continue; }
  pc = 0x49c0c2; continue;
 }
 0x49c0c2 => {
  { let v: u32 = cpu.r32(0xffffff78u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x49c942u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = cpu.r32(0x49c92au32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x49c0d7 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c10b; continue;
 }
 0x49c0e0 => {
  { let v: u32 = 0x6u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c10b; continue;
 }
 0x49c0e9 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xbu32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c10b; continue;
 }
 0x49c0fb => {
  { let v: u32 = 0xau32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c10b; continue;
 }
 0x49c10b => {
  pc = 0x49c6e3; continue;
 }
 0x49c110 => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x31u32); }
  if (cpu.sf != cpu.of) { pc = 0x49c134; continue; }
  pc = 0x49c119; continue;
 }
 0x49c119 => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49c134; continue; }
  pc = 0x49c122; continue;
 }
 0x49c122 => {
  { let v: u32 = 0x3u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c176; continue;
 }
 0x49c134 => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r8(0x5dd070u32) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.zf { pc = 0x49c14c; continue; }
  pc = 0x49c143; continue;
 }
 0x49c143 => {
  { let v: u32 = 0x5u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c176; continue;
 }
 0x49c14c => {
  { let v: u8 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xffffff74u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xffffff74u32.wrapping_add(cpu.ebp)), 0x30u8); }
  if cpu.zf { pc = 0x49c160; continue; }
  pc = 0x49c15e; continue;
 }
 0x49c15e => {
  pc = 0x49c169; continue;
 }
 0x49c160 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c176; continue;
 }
 0x49c169 => {
  { let v: u32 = 0xau32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c176; continue;
 }
 0x49c176 => {
  pc = 0x49c6e3; continue;
 }
 0x49c17b => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c195; continue;
 }
 0x49c184 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c195; continue;
 }
 0x49c195 => {
  { cpu.sub32(cpu.r32(0x5dd06cu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49c1b9; continue; }
  pc = 0x49c19e; continue;
 }
 0x49c19e => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49c1ae); f_00496290(cpu); if cpu.returned_to != 0x49c1ae { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c1d7; continue;
 }
 0x49c1b9 => {
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5dce60u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff70u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c1d7; continue;
 }
 0x49c1d7 => {
  { cpu.sub32(cpu.r32(0xffffff70u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49c214; continue; }
  pc = 0x49c1e0; continue;
 }
 0x49c1e0 => {
  { cpu.sub32(cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)), 0x19u32); }
  if !cpu.cf { pc = 0x49c206; continue; }
  pc = 0x49c1e6; continue;
 }
 0x49c1e6 => {
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c20f; continue;
 }
 0x49c206 => {
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c20f; continue;
 }
 0x49c20f => {
  cpu.poll(); pc = 0x49c184; continue;
 }
 0x49c214 => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x5dd070u32) as i8 as i32 as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x49c22c; continue; }
  pc = 0x49c223; continue;
 }
 0x49c223 => {
  { let v: u32 = 0x4u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c28e; continue;
 }
 0x49c22c => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff6cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff6cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2bu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff6cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff6cu32.wrapping_add(cpu.ebp)), 0x3au32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x49c27e; continue; }
  pc = 0x49c24e; continue;
 }
 0x49c24e => {
  { let v: u32 = cpu.r32(0xffffff6cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x49c991u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = cpu.r32(0x49c97du32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x49c263 => {
  { let v: u32 = 0x6u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c28e; continue;
 }
 0x49c26c => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xbu32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c28e; continue;
 }
 0x49c27e => {
  { let v: u32 = 0xau32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c28e; continue;
 }
 0x49c28e => {
  pc = 0x49c6e3; continue;
 }
 0x49c293 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49c2ce; continue; }
  pc = 0x49c2a7; continue;
 }
 0x49c2a7 => {
  pc = 0x49c2ba; continue;
 }
 0x49c2a9 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c2ba; continue;
 }
 0x49c2ba => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if !cpu.zf { pc = 0x49c2ce; continue; }
  pc = 0x49c2c3; continue;
 }
 0x49c2c3 => {
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49c2a9; continue;
 }
 0x49c2ce => {
  pc = 0x49c2e1; continue;
 }
 0x49c2d0 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c2e1; continue;
 }
 0x49c2e1 => {
  { cpu.sub32(cpu.r32(0x5dd06cu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49c306; continue; }
  pc = 0x49c2ea; continue;
 }
 0x49c2ea => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49c2fb); f_00496290(cpu); if cpu.returned_to != 0x49c2fb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff68u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c323; continue;
 }
 0x49c306 => {
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5dce60u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff68u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c323; continue;
 }
 0x49c323 => {
  { cpu.sub32(cpu.r32(0xffffff68u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49c35e; continue; }
  pc = 0x49c32c; continue;
 }
 0x49c32c => {
  { cpu.sub32(cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)), 0x19u32); }
  if !cpu.cf { pc = 0x49c359; continue; }
  pc = 0x49c332; continue;
 }
 0x49c332 => {
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c359; continue;
 }
 0x49c359 => {
  cpu.poll(); pc = 0x49c2d0; continue;
 }
 0x49c35e => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff64u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff64u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x2bu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff64u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff64u32.wrapping_add(cpu.ebp)), 0x3au32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x49c3b0; continue; }
  pc = 0x49c380; continue;
 }
 0x49c380 => {
  { let v: u32 = cpu.r32(0xffffff64u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x49c9e0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = cpu.r32(0x49c9ccu32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x49c395 => {
  { let v: u32 = 0x6u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c3c0; continue;
 }
 0x49c39e => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xbu32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c3c0; continue;
 }
 0x49c3b0 => {
  { let v: u32 = 0xau32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c3c0; continue;
 }
 0x49c3c0 => {
  pc = 0x49c6e3; continue;
 }
 0x49c3c5 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x5dd06cu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49c3f1; continue; }
  pc = 0x49c3d5; continue;
 }
 0x49c3d5 => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49c3e6); f_00496290(cpu); if cpu.returned_to != 0x49c3e6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c40e; continue;
 }
 0x49c3f1 => {
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5dce60u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff60u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c40e; continue;
 }
 0x49c40e => {
  { cpu.sub32(cpu.r32(0xffffff60u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49c429; continue; }
  pc = 0x49c417; continue;
 }
 0x49c417 => {
  { let v: u32 = 0x4u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c436; continue;
 }
 0x49c429 => {
  { let v: u32 = 0xau32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c436; continue;
 }
 0x49c436 => {
  pc = 0x49c6e3; continue;
 }
 0x49c43b => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x31u32); }
  if (cpu.sf != cpu.of) { pc = 0x49c468; continue; }
  pc = 0x49c44d; continue;
 }
 0x49c44d => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49c468; continue; }
  pc = 0x49c456; continue;
 }
 0x49c456 => {
  { let v: u32 = 0x9u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c4bd; continue;
 }
 0x49c468 => {
  { let v: u8 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffff5cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xffffff5cu32.wrapping_add(cpu.ebp)), 0x2bu8); }
  if cpu.zf { pc = 0x49c4a7; continue; }
  pc = 0x49c47a; continue;
 }
 0x49c47a => {
  { cpu.sub8(cpu.r8(0xffffff5cu32.wrapping_add(cpu.ebp)), 0x2du8); }
  if cpu.zf { pc = 0x49c497; continue; }
  pc = 0x49c483; continue;
 }
 0x49c483 => {
  { cpu.sub8(cpu.r8(0xffffff5cu32.wrapping_add(cpu.ebp)), 0x30u8); }
  if cpu.zf { pc = 0x49c48e; continue; }
  pc = 0x49c48c; continue;
 }
 0x49c48c => {
  pc = 0x49c4b0; continue;
 }
 0x49c48e => {
  { let v: u32 = 0x8u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c4bd; continue;
 }
 0x49c497 => {
  { let v: u32 = 0x7u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c4bd; continue;
 }
 0x49c4a7 => {
  { let v: u32 = 0x7u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c4bd; continue;
 }
 0x49c4b0 => {
  { let v: u32 = 0xau32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c4bd; continue;
 }
 0x49c4bd => {
  pc = 0x49c6e3; continue;
 }
 0x49c4c2 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c4dc; continue;
 }
 0x49c4cb => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c4dc; continue;
 }
 0x49c4dc => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x30u32); }
  if !cpu.zf { pc = 0x49c4e7; continue; }
  pc = 0x49c4e5; continue;
 }
 0x49c4e5 => {
  cpu.poll(); pc = 0x49c4cb; continue;
 }
 0x49c4e7 => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x31u32); }
  if (cpu.sf != cpu.of) { pc = 0x49c50b; continue; }
  pc = 0x49c4f0; continue;
 }
 0x49c4f0 => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49c50b; continue; }
  pc = 0x49c4f9; continue;
 }
 0x49c4f9 => {
  { let v: u32 = 0x9u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c51b; continue;
 }
 0x49c50b => {
  { let v: u32 = 0xau32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c51b; continue;
 }
 0x49c51b => {
  pc = 0x49c6e3; continue;
 }
 0x49c520 => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x31u32); }
  if (cpu.sf != cpu.of) { pc = 0x49c544; continue; }
  pc = 0x49c529; continue;
 }
 0x49c529 => {
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49c544; continue; }
  pc = 0x49c532; continue;
 }
 0x49c532 => {
  { let v: u32 = 0x9u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c56e; continue;
 }
 0x49c544 => {
  { let v: u8 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffff58u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xffffff58u32.wrapping_add(cpu.ebp)), 0x30u8); }
  if cpu.zf { pc = 0x49c558; continue; }
  pc = 0x49c556; continue;
 }
 0x49c556 => {
  pc = 0x49c561; continue;
 }
 0x49c558 => {
  { let v: u32 = 0x8u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c56e; continue;
 }
 0x49c561 => {
  { let v: u32 = 0xau32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c56e; continue;
 }
 0x49c56e => {
  pc = 0x49c6e3; continue;
 }
 0x49c573 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffff84u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c594; continue;
 }
 0x49c583 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c594; continue;
 }
 0x49c594 => {
  { cpu.sub32(cpu.r32(0x5dd06cu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49c5b9; continue; }
  pc = 0x49c59d; continue;
 }
 0x49c59d => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49c5ae); f_00496290(cpu); if cpu.returned_to != 0x49c5ae { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff54u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c5d6; continue;
 }
 0x49c5b9 => {
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5dce60u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0x4u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff54u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c5d6; continue;
 }
 0x49c5d6 => {
  { cpu.sub32(cpu.r32(0xffffff54u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49c607; continue; }
  pc = 0x49c5df; continue;
 }
 0x49c5df => {
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xau32); cpu.eax = v; }
  { let v: u32 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff84u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)), 0x1450u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49c602; continue; }
  pc = 0x49c5f9; continue;
 }
 0x49c5f9 => {
  { let v: u32 = 0x1451u32; cpu.w32(0xffffff84u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c607; continue;
 }
 0x49c602 => {
  cpu.poll(); pc = 0x49c583; continue;
 }
 0x49c607 => {
  { let v: u32 = cpu.r32(0xffffff84u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c620; continue;
 }
 0x49c60f => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c620; continue;
 }
 0x49c620 => {
  { cpu.sub32(cpu.r32(0x5dd06cu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49c645; continue; }
  pc = 0x49c629; continue;
 }
 0x49c629 => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49c63a); f_00496290(cpu); if cpu.returned_to != 0x49c63a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff50u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c662; continue;
 }
 0x49c645 => {
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5dce60u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx & 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff50u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c662; continue;
 }
 0x49c662 => {
  { cpu.sub32(cpu.r32(0xffffff50u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49c66d; continue; }
  pc = 0x49c66b; continue;
 }
 0x49c66b => {
  cpu.poll(); pc = 0x49c60f; continue;
 }
 0x49c66d => {
  { let v: u32 = 0xau32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c6e3; continue;
 }
 0x49c67f => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49c6d3; continue; }
  pc = 0x49c685; continue;
 }
 0x49c685 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xffffff4cu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0xffffff4cu32.wrapping_add(cpu.ebp)), 0x2bu8); }
  if cpu.zf { pc = 0x49c6bb; continue; }
  pc = 0x49c6a0; continue;
 }
 0x49c6a0 => {
  { cpu.sub8(cpu.r8(0xffffff4cu32.wrapping_add(cpu.ebp)), 0x2du8); }
  if cpu.zf { pc = 0x49c6ab; continue; }
  pc = 0x49c6a9; continue;
 }
 0x49c6a9 => {
  pc = 0x49c6c4; continue;
 }
 0x49c6ab => {
  { let v: u32 = 0x7u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c6d1; continue;
 }
 0x49c6bb => {
  { let v: u32 = 0x7u32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c6d1; continue;
 }
 0x49c6c4 => {
  { let v: u32 = 0xau32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c6d1; continue;
 }
 0x49c6d1 => {
  pc = 0x49c6e3; continue;
 }
 0x49c6d3 => {
  { let v: u32 = 0xau32; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c6e3; continue;
 }
 0x49c6e3 => {
  cpu.poll(); pc = 0x49bf87; continue;
 }
 0x49c6e8 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49c837; continue; }
  pc = 0x49c6fa; continue;
 }
 0x49c6fa => {
  { cpu.sub32(cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49c837; continue; }
  pc = 0x49c704; continue;
 }
 0x49c704 => {
  { cpu.sub32(cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49c837; continue; }
  pc = 0x49c70e; continue;
 }
 0x49c70e => {
  { cpu.sub32(cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)), 0x18u32); }
  if (cpu.cf || cpu.zf) { pc = 0x49c73f; continue; }
  pc = 0x49c714; continue;
 }
 0x49c714 => {
  { let v: u32 = cpu.r8(0xffffffdfu32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x5u32); }
  if (cpu.sf != cpu.of) { pc = 0x49c726; continue; }
  pc = 0x49c71d; continue;
 }
 0x49c71d => {
  { let v: u8 = cpu.r8(0xffffffdfu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x1u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xffffffdfu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c726; continue;
 }
 0x49c726 => {
  { let v: u32 = 0x18u32; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c73f; continue;
 }
 0x49c73f => {
  { cpu.sub32(cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x49c81a; continue; }
  pc = 0x49c749; continue;
 }
 0x49c749 => {
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c75d; continue;
 }
 0x49c754 => {
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c75d; continue;
 }
 0x49c75d => {
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x49c77b; continue; }
  pc = 0x49c767; continue;
 }
 0x49c767 => {
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49c754; continue;
 }
 0x49c77b => {
  { let v: u32 = 0xffffffa0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49c78c); f_0049bda0(cpu); if cpu.returned_to != 0x49c78c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49c79d; continue; }
  pc = 0x49c795; continue;
 }
 0x49c795 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c79d; continue;
 }
 0x49c79d => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49c7b5; continue; }
  pc = 0x49c7ac; continue;
 }
 0x49c7ac => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c7b5; continue;
 }
 0x49c7b5 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49c7c4; continue; }
  pc = 0x49c7bb; continue;
 }
 0x49c7bb => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c7c4; continue;
 }
 0x49c7c4 => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1450u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49c7d6; continue; }
  pc = 0x49c7cd; continue;
 }
 0x49c7cd => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c818; continue;
 }
 0x49c7d6 => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0xffffebb0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49c7e8; continue; }
  pc = 0x49c7df; continue;
 }
 0x49c7df => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffff88u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c818; continue;
 }
 0x49c7e8 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffa0u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49c7f9); f_0049e800(cpu); if cpu.returned_to != 0x49c7f9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = cpu.r16(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa2u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffa6u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0xffffffaau32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c818; continue;
 }
 0x49c818 => {
  pc = 0x49c837; continue;
 }
 0x49c81a => {
  { let v: u16 = 0x0u16; cpu.w16(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c837; continue;
 }
 0x49c837 => {
  { cpu.sub32(cpu.r32(0xffffffacu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49c866; continue; }
  pc = 0x49c83d; continue;
 }
 0x49c83d => {
  { let v: u16 = 0x0u16; cpu.w16(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c8bd; continue;
 }
 0x49c866 => {
  { cpu.sub32(cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49c891; continue; }
  pc = 0x49c86c; continue;
 }
 0x49c86c => {
  { let v: u16 = 0x7fffu16; cpu.w16(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c8bd; continue;
 }
 0x49c891 => {
  { cpu.sub32(cpu.r32(0xffffff88u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49c8bd; continue; }
  pc = 0x49c897; continue;
 }
 0x49c897 => {
  { let v: u16 = 0x0u16; cpu.w16(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49c8bd; continue;
 }
 0x49c8bd => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x6u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffb0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ca20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ca20); }
 let mut pc: u32 = 0x49ca20;
 loop { match pc {
 0x49ca20 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x74u32); cpu.esp = v; }
  { let v: u16 = 0x4d10u16; cpu.w16(0xffffff9cu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x4du16; cpu.w16(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x9au16; cpu.w16(0xffffffa0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x134312f4u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xccu8; cpu.w8(0xffffffacu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xccu8; cpu.w8(0xffffffadu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xccu8; cpu.w8(0xffffffaeu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xccu8; cpu.w8(0xffffffafu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xccu8; cpu.w8(0xffffffb0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xccu8; cpu.w8(0xffffffb1u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xccu8; cpu.w8(0xffffffb2u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xccu8; cpu.w8(0xffffffb3u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xccu8; cpu.w8(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xccu8; cpu.w8(0xffffffb5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0xfbu8; cpu.w8(0xffffffb6u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x3fu8; cpu.w8(0xffffffb7u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x8000u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffff98u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffff90u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49cabe; continue; }
  pc = 0x49cab5; continue;
 }
 0x49cab5 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x49cac5; continue;
 }
 0x49cabe => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x2u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49cac5; continue;
 }
 0x49cac5 => {
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x49cb0c; continue; }
  pc = 0x49cad2; continue;
 }
 0x49cad2 => {
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49cb0c; continue; }
  pc = 0x49cad8; continue;
 }
 0x49cad8 => {
  { cpu.sub32(cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49cb0c; continue; }
  pc = 0x49cade; continue;
 }
 0x49cade => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x2u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x3u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49cf32; continue;
 }
 0x49cb0c => {
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if !cpu.zf { pc = 0x49cc06; continue; }
  pc = 0x49cb21; continue;
 }
 0x49cb21 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x80000000u32); }
  if !cpu.zf { pc = 0x49cb38; continue; }
  pc = 0x49cb32; continue;
 }
 0x49cb32 => {
  { cpu.sub32(cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49cb6c; continue; }
  pc = 0x49cb38; continue;
 }
 0x49cb38 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x40000000u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x49cb6c; continue; }
  pc = 0x49cb45; continue;
 }
 0x49cb45 => {
  { let v = 0x4a26f4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49cb56); f_00493770(cpu); if cpu.returned_to != 0x49cb56 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x6u8; cpu.w8(0x3u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cc01; continue;
 }
 0x49cb6c => {
  { let v: u32 = cpu.r32(0xffffff98u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49cbac; continue; }
  pc = 0x49cb79; continue;
 }
 0x49cb79 => {
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0xc0000000u32); }
  if !cpu.zf { pc = 0x49cbac; continue; }
  pc = 0x49cb82; continue;
 }
 0x49cb82 => {
  { cpu.sub32(cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49cbac; continue; }
  pc = 0x49cb88; continue;
 }
 0x49cb88 => {
  { let v = 0x4a26ecu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49cb99); f_00493770(cpu); if cpu.returned_to != 0x49cb99 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x5u8; cpu.w8(0x3u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cc01; continue;
 }
 0x49cbac => {
  { cpu.sub32(cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)), 0x80000000u32); }
  if !cpu.zf { pc = 0x49cbdf; continue; }
  pc = 0x49cbb5; continue;
 }
 0x49cbb5 => {
  { cpu.sub32(cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49cbdf; continue; }
  pc = 0x49cbbb; continue;
 }
 0x49cbbb => {
  { let v = 0x4a26e4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49cbcc); f_00493770(cpu); if cpu.returned_to != 0x49cbcc { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = 0x5u8; cpu.w8(0x3u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cc01; continue;
 }
 0x49cbdf => {
  { let v = 0x4a26dcu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49cbf0); f_00493770(cpu); if cpu.returned_to != 0x49cbf0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x6u8; cpu.w8(0x3u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cc01; continue;
 }
 0x49cc01 => {
  pc = 0x49cf2f; continue;
 }
 0x49cc06 => {
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffff94u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 24u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffff9cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffff94u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffa0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0xffffff90u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffd2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffceu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffcau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r16(0xffffffa4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49ccad); f_0049e800(cpu); if cpu.returned_to != 0x49ccad { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffd2u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x3fffu32); }
  if (cpu.sf != cpu.of) { pc = 0x49ccdd; continue; }
  pc = 0x49ccc1; continue;
 }
 0x49ccc1 => {
  { let v: u16 = cpu.r16(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x1u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffacu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49ccda); f_0049e3b0(cpu); if cpu.returned_to != 0x49ccda { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x49ccdd; continue;
 }
 0x49ccdd => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49cd31; continue; }
  pc = 0x49ccf1; continue;
 }
 0x49ccf1 => {
  { let v: u32 = cpu.r16(0xffffffa4u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49cd31; continue; }
  pc = 0x49cd03; continue;
 }
 0x49cd03 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x3u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49cf32; continue;
 }
 0x49cd31 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x15u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49cd3e; continue; }
  pc = 0x49cd37; continue;
 }
 0x49cd37 => {
  { let v: u32 = 0x15u32; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cd3e; continue;
 }
 0x49cd3e => {
  { let v: u32 = cpu.r32(0xffffffd2u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x3ffeu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffd2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cd66; continue;
 }
 0x49cd5d => {
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cd66; continue;
 }
 0x49cd66 => {
  { cpu.sub32(cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)), 0x8u32); }
  if (cpu.sf == cpu.of) { pc = 0x49cd7a; continue; }
  pc = 0x49cd6c; continue;
 }
 0x49cd6c => {
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49cd75); f_0049bcd0(cpu); if cpu.returned_to != 0x49cd75 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.poll(); pc = 0x49cd5d; continue;
 }
 0x49cd7a => {
  { cpu.sub32(cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49cdac; continue; }
  pc = 0x49cd80; continue;
 }
 0x49cd80 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cd98; continue;
 }
 0x49cd8f => {
  { let v: u32 = cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffff8cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cd98; continue;
 }
 0x49cd98 => {
  { cpu.sub32(cpu.r32(0xffffff8cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49cdac; continue; }
  pc = 0x49cd9e; continue;
 }
 0x49cd9e => {
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49cda7); f_0049bd30(cpu); if cpu.returned_to != 0x49cda7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.poll(); pc = 0x49cd8f; continue;
 }
 0x49cdac => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cdc9; continue;
 }
 0x49cdc0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cdc9; continue;
 }
 0x49cdc9 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49ce35; continue; }
  pc = 0x49cdcf; continue;
 }
 0x49cdcf => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49cdea); f_0049bcd0(cpu); if cpu.returned_to != 0x49cdea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49cdf6); f_0049bcd0(cpu); if cpu.returned_to != 0x49cdf6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49ce06); f_0049bc20(cpu); if cpu.returned_to != 0x49ce06 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0xffffffc8u32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49ce12); f_0049bcd0(cpu); if cpu.returned_to != 0x49ce12 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffd3u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xffffffd3u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49cdc0; continue;
 }
 0x49ce35 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r8(0xffffffd8u32.wrapping_add(cpu.ebp)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x35u32); }
  if (cpu.sf != cpu.of) { pc = 0x49ceb3; continue; }
  pc = 0x49ce58; continue;
 }
 0x49ce58 => {
  pc = 0x49ce63; continue;
 }
 0x49ce5a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ce63; continue;
 }
 0x49ce63 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), cpu.edx); }
  if cpu.cf { pc = 0x49ce81; continue; }
  pc = 0x49ce6e; continue;
 }
 0x49ce6e => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x39u32); }
  if !cpu.zf { pc = 0x49ce81; continue; }
  pc = 0x49ce79; continue;
 }
 0x49ce79 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.poll(); pc = 0x49ce5a; continue;
 }
 0x49ce81 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.cf { pc = 0x49cea5; continue; }
  pc = 0x49ce8c; continue;
 }
 0x49ce8c => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49cea5; continue;
 }
 0x49cea5 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49cf0c; continue;
 }
 0x49ceb3 => {
  pc = 0x49cebe; continue;
 }
 0x49ceb5 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49cebe; continue;
 }
 0x49cebe => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), cpu.eax); }
  if cpu.cf { pc = 0x49ced6; continue; }
  pc = 0x49cec9; continue;
 }
 0x49cec9 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if !cpu.zf { pc = 0x49ced6; continue; }
  pc = 0x49ced4; continue;
 }
 0x49ced4 => {
  cpu.poll(); pc = 0x49ceb5; continue;
 }
 0x49ced6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.cf { pc = 0x49cf0c; continue; }
  pc = 0x49cee1; continue;
 }
 0x49cee1 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x3u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x5u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49cf32; continue;
 }
 0x49cf0c => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x3u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x3u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49cf2f; continue;
 }
 0x49cf2f => {
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49cf32; continue;
 }
 0x49cf32 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049cf40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49cf40); }
 let mut pc: u32 = 0x49cf40;
 loop { match pc {
 0x49cf40 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x10u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49cf84; continue; }
  pc = 0x49cf6e; continue;
 }
 0x49cf6e => {
  { let v: u32 = 0xc000008fu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x49cf84; continue;
 }
 0x49cf84 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x2u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49cfa3; continue; }
  pc = 0x49cf8e; continue;
 }
 0x49cf8e => {
  { let v: u32 = 0xc0000093u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49cfa3; continue;
 }
 0x49cfa3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49cfc3; continue; }
  pc = 0x49cfad; continue;
 }
 0x49cfad => {
  { let v: u32 = 0xc0000091u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  pc = 0x49cfc3; continue;
 }
 0x49cfc3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x4u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49cfe3; continue; }
  pc = 0x49cfcd; continue;
 }
 0x49cfcd => {
  { let v: u32 = 0xc000008eu32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x8u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x49cfe3; continue;
 }
 0x49cfe3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x8u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49d002; continue; }
  pc = 0x49cfed; continue;
 }
 0x49cfed => {
  { let v: u32 = 0xc0000090u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x10u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49d002; continue;
 }
 0x49d002 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffffefu32; cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x4u32; cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffff7u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x8u32; cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x10u32; cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfffffffdu32; cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x20u32; cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffffeu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x49d0b6); f_0049d7e0(cpu); if cpu.returned_to != 0x49d0b6 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49d0d2; continue; }
  pc = 0x49d0c3; continue;
 }
 0x49d0c3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  pc = 0x49d0d2; continue;
 }
 0x49d0d2 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x4u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49d0ea; continue; }
  pc = 0x49d0dc; continue;
 }
 0x49d0dc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x49d0ea; continue;
 }
 0x49d0ea => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49d103; continue; }
  pc = 0x49d0f4; continue;
 }
 0x49d0f4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x49d103; continue;
 }
 0x49d103 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x10u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49d11c; continue; }
  pc = 0x49d10d; continue;
 }
 0x49d10d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  pc = 0x49d11c; continue;
 }
 0x49d11c => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x20u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49d134; continue; }
  pc = 0x49d126; continue;
 }
 0x49d126 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x49d134; continue;
 }
 0x49d134 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xc00u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x400u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x49d15b; continue; }
  pc = 0x49d14a; continue;
 }
 0x49d14a => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x400u32); }
  if cpu.zf { pc = 0x49d190; continue; }
  pc = 0x49d153; continue;
 }
 0x49d153 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49d1a2; continue; }
  pc = 0x49d159; continue;
 }
 0x49d159 => {
  pc = 0x49d1af; continue;
 }
 0x49d15b => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x800u32); }
  if cpu.zf { pc = 0x49d17e; continue; }
  pc = 0x49d164; continue;
 }
 0x49d164 => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0xc00u32); }
  if cpu.zf { pc = 0x49d16f; continue; }
  pc = 0x49d16d; continue;
 }
 0x49d16d => {
  pc = 0x49d1af; continue;
 }
 0x49d16f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x3u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x49d1af; continue;
 }
 0x49d17e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfffffffcu32; cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x49d1af; continue;
 }
 0x49d190 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfffffffcu32; cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x49d1af; continue;
 }
 0x49d1a2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfffffffcu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x49d1af; continue;
 }
 0x49d1af => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x300u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49d1f8; continue; }
  pc = 0x49d1c3; continue;
 }
 0x49d1c3 => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x200u32); }
  if cpu.zf { pc = 0x49d1e6; continue; }
  pc = 0x49d1cc; continue;
 }
 0x49d1cc => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x300u32); }
  if cpu.zf { pc = 0x49d1d7; continue; }
  pc = 0x49d1d5; continue;
 }
 0x49d1d5 => {
  pc = 0x49d208; continue;
 }
 0x49d1d7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffffe3u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x49d208; continue;
 }
 0x49d1e6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffffe3u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x49d208; continue;
 }
 0x49d1f8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffffe3u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x8u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x49d208; continue;
 }
 0x49d208 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfffe001fu32; cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffffe1u32; cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x50u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffffe1u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x50u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x40u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x49d28e); f_0049d800(cpu); if cpu.returned_to != 0x49d28e { return; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10e0u32); cpu.push(0x49d2a0); cpu.call(t); if cpu.returned_to != 0x49d2a0 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49d2bd; continue; }
  pc = 0x49d2b0; continue;
 }
 0x49d2b0 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffffeu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x49d2bd; continue;
 }
 0x49d2bd => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49d2d9; continue; }
  pc = 0x49d2cd; continue;
 }
 0x49d2cd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49d2d9; continue;
 }
 0x49d2d9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49d2f6; continue; }
  pc = 0x49d2e9; continue;
 }
 0x49d2e9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfffffff7u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x49d2f6; continue;
 }
 0x49d2f6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49d312; continue; }
  pc = 0x49d305; continue;
 }
 0x49d305 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffffefu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x49d312; continue;
 }
 0x49d312 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49d32b; continue; }
  pc = 0x49d31f; continue;
 }
 0x49d31f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xdfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49d32b; continue;
 }
 0x49d32b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x49d389; continue; }
  pc = 0x49d33c; continue;
 }
 0x49d33c => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.poll(); pc = cpu.r32(0x49d3f1u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x49d346 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xf3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0xcu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49d389; continue;
 }
 0x49d358 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xf3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49d389; continue;
 }
 0x49d36a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xf3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49d389; continue;
 }
 0x49d37c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xf3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49d389; continue;
 }
 0x49d389 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49d3ab; continue; }
  pc = 0x49d39d; continue;
 }
 0x49d39d => {
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x49d3bd; continue; }
  pc = 0x49d3a3; continue;
 }
 0x49d3a3 => {
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if cpu.zf { pc = 0x49d3cf; continue; }
  pc = 0x49d3a9; continue;
 }
 0x49d3a9 => {
  pc = 0x49d3dc; continue;
 }
 0x49d3ab => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0xf3u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x3u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x49d3dc; continue;
 }
 0x49d3bd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0xf3u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x2u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x49d3dc; continue;
 }
 0x49d3cf => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u8 = cpu.and8(((cpu.edx >> 8) as u8), 0xf3u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x49d3dc; continue;
 }
 0x49d3dc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d410); }
 let mut pc: u32 = 0x49d410;
 loop { match pc {
 0x49d410 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x8u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49d44a; continue; }
  pc = 0x49d429; continue;
 }
 0x49d429 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49d44a; continue; }
  pc = 0x49d433; continue;
 }
 0x49d433 => {
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x49d43a); f_0049d860(cpu); if cpu.returned_to != 0x49d43a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xf7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d74e; continue;
 }
 0x49d44a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x4u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49d475; continue; }
  pc = 0x49d454; continue;
 }
 0x49d454 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x4u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49d475; continue; }
  pc = 0x49d45e; continue;
 }
 0x49d45e => {
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x49d465); f_0049d860(cpu); if cpu.returned_to != 0x49d465 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d74e; continue;
 }
 0x49d475 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49d5f1; continue; }
  pc = 0x49d483; continue;
 }
 0x49d483 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49d5f1; continue; }
  pc = 0x49d491; continue;
 }
 0x49d491 => {
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x49d498); f_0049d860(cpu); if cpu.returned_to != 0x49d498 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xc00u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x400u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x49d4c7; continue; }
  pc = 0x49d4af; continue;
 }
 0x49d4af => {
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x400u32); }
  if cpu.zf { pc = 0x49d565; continue; }
  pc = 0x49d4bc; continue;
 }
 0x49d4bc => {
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49d4e2; continue; }
  pc = 0x49d4c2; continue;
 }
 0x49d4c2 => {
  pc = 0x49d5e3; continue;
 }
 0x49d4c7 => {
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x800u32); }
  if cpu.zf { pc = 0x49d525; continue; }
  pc = 0x49d4d0; continue;
 }
 0x49d4d0 => {
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0xc00u32); }
  if cpu.zf { pc = 0x49d5a5; continue; }
  pc = 0x49d4dd; continue;
 }
 0x49d4dd => {
  pc = 0x49d5e3; continue;
 }
 0x49d4e2 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4a27b0u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x49d507; continue; }
  pc = 0x49d4f4; continue;
 }
 0x49d4f4 => {
  { let v: u32 = cpu.r32(0x5dd580u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x5dd584u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d512; continue;
 }
 0x49d507 => {
  { let v = cpu.rf64(0x5dd580u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffd8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x49d512; continue;
 }
 0x49d512 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49d5e3; continue;
 }
 0x49d525 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4a27b0u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x49d54a; continue; }
  pc = 0x49d537; continue;
 }
 0x49d537 => {
  { let v: u32 = cpu.r32(0x5dd580u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x5dd584u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d555; continue;
 }
 0x49d54a => {
  { let v = cpu.rf64(0x5dd590u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffd0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x49d555; continue;
 }
 0x49d555 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49d5e3; continue;
 }
 0x49d565 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4a27b0u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x49d58a; continue; }
  pc = 0x49d577; continue;
 }
 0x49d577 => {
  { let v: u32 = cpu.r32(0x5dd590u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x5dd594u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d595; continue;
 }
 0x49d58a => {
  { let v = cpu.rf64(0x5dd580u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffc8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x49d595; continue;
 }
 0x49d595 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49d5e3; continue;
 }
 0x49d5a5 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4a27b0u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x41u8); }
  if !cpu.zf { pc = 0x49d5ca; continue; }
  pc = 0x49d5b7; continue;
 }
 0x49d5b7 => {
  { let v: u32 = cpu.r32(0x5dd590u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x5dd594u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d5d5; continue;
 }
 0x49d5ca => {
  { let v = cpu.rf64(0x5dd590u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffc0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x49d5d5; continue;
 }
 0x49d5d5 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49d5e3; continue;
 }
 0x49d5e3 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffffeu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d74e; continue;
 }
 0x49d5f1 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x2u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49d74e; continue; }
  pc = 0x49d5ff; continue;
 }
 0x49d5ff => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x10u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49d74e; continue; }
  pc = 0x49d60d; continue;
 }
 0x49d60d => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x10u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49d625; continue; }
  pc = 0x49d61e; continue;
 }
 0x49d61e => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d625; continue;
 }
 0x49d625 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4a27b0u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x40u8); }
  if !cpu.zf { pc = 0x49d72e; continue; }
  pc = 0x49d63b; continue;
 }
 0x49d63b => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49d64e); f_0049a6e0(cpu); if cpu.returned_to != 0x49d64e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x600u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0xfffffbceu32); }
  if (cpu.sf == cpu.of) { pc = 0x49d681; continue; }
  pc = 0x49d669; continue;
 }
 0x49d669 => {
  { let v = cpu.rf64(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x4a27b0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d71e; continue;
 }
 0x49d681 => {
  { let v = cpu.rf64(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x4a27b0u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x49d69a; continue; }
  pc = 0x49d691; continue;
 }
 0x49d691 => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d6a1; continue;
 }
 0x49d69a => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d6a1; continue;
 }
 0x49d6a1 => {
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0xfffffff6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xfu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0xfffffff6u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x10u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xfffffff6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d6c9; continue;
 }
 0x49d6c0 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d6c9; continue;
 }
 0x49d6c9 => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0xfffffc03u32); }
  if (cpu.sf == cpu.of) { pc = 0x49d710; continue; }
  pc = 0x49d6d2; continue;
 }
 0x49d6d2 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49d6e9; continue; }
  pc = 0x49d6dc; continue;
 }
 0x49d6dc => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49d6e9; continue; }
  pc = 0x49d6e2; continue;
 }
 0x49d6e2 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d6e9; continue;
 }
 0x49d6e9 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49d706; continue; }
  pc = 0x49d6fb; continue;
 }
 0x49d6fb => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x80000000u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d706; continue;
 }
 0x49d706 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49d6c0; continue;
 }
 0x49d710 => {
  { cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49d71e; continue; }
  pc = 0x49d716; continue;
 }
 0x49d716 => {
  { let v = cpu.rf64(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x49d71e; continue;
 }
 0x49d71e => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  pc = 0x49d735; continue;
 }
 0x49d72e => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d735; continue;
 }
 0x49d735 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49d745; continue; }
  pc = 0x49d73b; continue;
 }
 0x49d73b => {
  { let v = 0x10u32; cpu.push(v); }
  cpu.push(0x49d742); f_0049d860(cpu); if cpu.returned_to != 0x49d742 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x49d745; continue;
 }
 0x49d745 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xfffffffdu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d74e; continue;
 }
 0x49d74e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x10u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49d775; continue; }
  pc = 0x49d758; continue;
 }
 0x49d758 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x20u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49d775; continue; }
  pc = 0x49d762; continue;
 }
 0x49d762 => {
  { let v = 0x20u32; cpu.push(v); }
  cpu.push(0x49d769); f_0049d860(cpu); if cpu.returned_to != 0x49d769 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffffefu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d775; continue;
 }
 0x49d775 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d790(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d790); }
 let mut pc: u32 = 0x49d790;
 loop { match pc {
 0x49d790 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x49d7ae; continue; }
  pc = 0x49d7a0; continue;
 }
 0x49d7a0 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49d7c4; continue; }
  pc = 0x49d7a6; continue;
 }
 0x49d7a6 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x3u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49d7ba; continue; }
  pc = 0x49d7ac; continue;
 }
 0x49d7ac => {
  pc = 0x49d7c4; continue;
 }
 0x49d7ae => {
  { let v: u32 = 0x21u32; cpu.w32(0x5de9bcu32, v); }
  pc = 0x49d7c4; continue;
 }
 0x49d7ba => {
  { let v: u32 = 0x22u32; cpu.w32(0x5de9bcu32, v); }
  pc = 0x49d7c4; continue;
 }
 0x49d7c4 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d7d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d7d0); }
 let mut pc: u32 = 0x49d7d0;
 loop { match pc {
 0x49d7d0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d7e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d7e0); }
 let mut pc: u32 = 0x49d7e0;
 loop { match pc {
 0x49d7e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d800(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d800); }
 let mut pc: u32 = 0x49d800;
 loop { match pc {
 0x49d800 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.fstatus(); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d820); }
 let mut pc: u32 = 0x49d820;
 loop { match pc {
 0x49d820 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx & cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.fpu_control = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d860); }
 let mut pc: u32 = 0x49d860;
 loop { match pc {
 0x49d860 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49d87d; continue; }
  pc = 0x49d873; continue;
 }
 0x49d873 => {
  { let v = cpu.rf80(0x5dd740u32); cpu.fpush(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  {  }
  pc = 0x49d87d; continue;
 }
 0x49d87d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x8u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49d897; continue; }
  pc = 0x49d887; continue;
 }
 0x49d887 => {
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = cpu.rf80(0x5dd740u32); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  {  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  pc = 0x49d897; continue;
 }
 0x49d897 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49d8ab; continue; }
  pc = 0x49d8a1; continue;
 }
 0x49d8a1 => {
  { let v = cpu.rf80(0x5dd750u32); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  {  }
  pc = 0x49d8ab; continue;
 }
 0x49d8ab => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x4u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49d8be; continue; }
  pc = 0x49d8b5; continue;
 }
 0x49d8b5 => {
  { cpu.fpush(0.0); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  {  }
  pc = 0x49d8be; continue;
 }
 0x49d8be => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x20u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49d8ce; continue; }
  pc = 0x49d8c8; continue;
 }
 0x49d8c8 => {
  { cpu.fpush(std::f64::consts::PI); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  {  }
  pc = 0x49d8ce; continue;
 }
 0x49d8ce => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049d8e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49d8e0); }
 let mut pc: u32 = 0x49d8e0;
 loop { match pc {
 0x49d8e0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x5deeb0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49d909; continue; }
  pc = 0x49d902; continue;
 }
 0x49d902 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49da9d; continue;
 }
 0x49d909 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x7861f80u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x7861f80u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x46u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x1e13380u32); }
  if (cpu.sf != cpu.of) { pc = 0x49d999; continue; }
  pc = 0x49d93e; continue;
 }
 0x49d93e => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1e13380u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x1e13380u32); }
  if (cpu.sf != cpu.of) { pc = 0x49d999; continue; }
  pc = 0x49d95c; continue;
 }
 0x49d95c => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1e13380u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x1e28500u32); }
  if (cpu.sf != cpu.of) { pc = 0x49d990; continue; }
  pc = 0x49d979; continue;
 }
 0x49d979 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1e28500u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d999; continue;
 }
 0x49d990 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d999; continue;
 }
 0x49d999 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x15180u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, 0x15180u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49d9d6; continue; }
  pc = 0x49d9cd; continue;
 }
 0x49d9cd => {
  { let v: u32 = 0x5dd760u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d9dd; continue;
 }
 0x49d9d6 => {
  { let v: u32 = 0x5dd798u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d9dd; continue;
 }
 0x49d9dd => {
  { let v: u32 = 0x1u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d9ef; continue;
 }
 0x49d9e6 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49d9ef; continue;
 }
 0x49d9ef => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x1cu32.wrapping_add(cpu.eax))); }
  if (cpu.sf == cpu.of) { pc = 0x49da02; continue; }
  pc = 0x49da00; continue;
 }
 0x49da00 => {
  cpu.poll(); pc = 0x49d9e6; continue;
 }
 0x49da02 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x15180u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0xe10u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, 0xe10u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, 0x3cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49da9d; continue;
 }
 0x49da9d => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049dab0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49dab0); }
 let mut pc: u32 = 0x49dab0;
 loop { match pc {
 0x49dab0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49dac5; continue; }
  pc = 0x49dabe; continue;
 }
 0x49dabe => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49dd0c; continue;
 }
 0x49dac5 => {
  cpu.push(0x49daca); f_0049ac50(cpu); if cpu.returned_to != 0x49daca { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x3f480u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49db40; continue; }
  pc = 0x49dad5; continue;
 }
 0x49dad5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x7ffc0b7fu32); }
  if (cpu.sf == cpu.of) { pc = 0x49db40; continue; }
  pc = 0x49dae0; continue;
 }
 0x49dae0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x5dd5a8u32)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49daf7); f_0049d8e0(cpu); if cpu.returned_to != 0x49daf7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x5dd5acu32), 0x0u32); }
  if cpu.zf { pc = 0x49db3b; continue; }
  pc = 0x49db06; continue;
 }
 0x49db06 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49db0f); f_0049afb0(cpu); if cpu.returned_to != 0x49db0f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49db3b; continue; }
  pc = 0x49db16; continue;
 }
 0x49db16 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x5dd5b0u32)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49db2b); f_0049d8e0(cpu); if cpu.returned_to != 0x49db2b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  pc = 0x49db3b; continue;
 }
 0x49db3b => {
  pc = 0x49dd09; continue;
 }
 0x49db40 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49db49); f_0049d8e0(cpu); if cpu.returned_to != 0x49db49 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49db58); f_0049afb0(cpu); if cpu.returned_to != 0x49db58 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49db76; continue; }
  pc = 0x49db5f; continue;
 }
 0x49db5f => {
  { let v: u32 = cpu.r32(0x5dd5a8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5dd5b0u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49db84; continue;
 }
 0x49db76 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x5dd5a8u32)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49db84; continue;
 }
 0x49db84 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49dbb2; continue; }
  pc = 0x49db9c; continue;
 }
 0x49db9c => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49dbb2; continue;
 }
 0x49dbb2 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49dbfa; continue; }
  pc = 0x49dbe2; continue;
 }
 0x49dbe2 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49dbfa; continue;
 }
 0x49dbfa => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49dc42; continue; }
  pc = 0x49dc2a; continue;
 }
 0x49dc2a => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x18u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x18u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49dc42; continue;
 }
 0x49dc42 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49dc8d; continue; }
  pc = 0x49dc56; continue;
 }
 0x49dc56 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  pc = 0x49dd09; continue;
 }
 0x49dc8d => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49dd09; continue; }
  pc = 0x49dc93; continue;
 }
 0x49dc93 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x7u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.eax = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ecx)), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49dcfa; continue; }
  pc = 0x49dcc6; continue;
 }
 0x49dcc6 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1fu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x16cu32; cpu.w32(0x1cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0xbu32; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, 0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x49dd09; continue;
 }
 0x49dcfa => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  pc = 0x49dd09; continue;
 }
 0x49dd09 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49dd0c; continue;
 }
 0x49dd0c => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049dd10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49dd10); }
 let mut pc: u32 = 0x49dd10;
 loop { match pc {
 0x49dd10 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  pc = 0x49dd16; continue;
 }
 0x49dd16 => {
  { cpu.sub32(cpu.r32(0x5dd06cu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49dd36; continue; }
  pc = 0x49dd1f; continue;
 }
 0x49dd1f => {
  { let v = 0x8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49dd2e); f_00496290(cpu); if cpu.returned_to != 0x49dd2e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49dd4f; continue;
 }
 0x49dd36 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5dce60u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0x8u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49dd4f; continue;
 }
 0x49dd4f => {
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49dd60; continue; }
  pc = 0x49dd55; continue;
 }
 0x49dd55 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49dd16; continue;
 }
 0x49dd60 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x2du32); }
  if cpu.zf { pc = 0x49dd85; continue; }
  pc = 0x49dd7f; continue;
 }
 0x49dd7f => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x2bu32); }
  if !cpu.zf { pc = 0x49dd98; continue; }
  pc = 0x49dd85; continue;
 }
 0x49dd85 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49dd98; continue;
 }
 0x49dd98 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49dd9f; continue;
 }
 0x49dd9f => {
  { cpu.sub32(cpu.r32(0x5dd06cu32), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49ddbb; continue; }
  pc = 0x49dda8; continue;
 }
 0x49dda8 => {
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49ddb3); f_00496290(cpu); if cpu.returned_to != 0x49ddb3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ddd0; continue;
 }
 0x49ddbb => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5dce60u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0x4u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ddd0; continue;
 }
 0x49ddd0 => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49ddfb; continue; }
  pc = 0x49ddd6; continue;
 }
 0x49ddd6 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0xau32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49dd9f; continue;
 }
 0x49ddfb => {
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x2du32); }
  if !cpu.zf { pc = 0x49de08; continue; }
  pc = 0x49de01; continue;
 }
 0x49de01 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x49de0b; continue;
 }
 0x49de08 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49de0b; continue;
 }
 0x49de0b => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049de10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49de10); }
 let mut pc: u32 = 0x49de10;
 loop { match pc {
 0x49de10 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.edx, 0x3u32); }
  if !cpu.zf { pc = 0x49de5c; continue; }
  pc = 0x49de20; continue;
 }
 0x49de20 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { pc = 0x49de54; continue; }
  pc = 0x49de26; continue;
 }
 0x49de26 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x49de50; continue; }
  pc = 0x49de2a; continue;
 }
 0x49de2a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), cpu.r8(0x1u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { pc = 0x49de54; continue; }
  pc = 0x49de2f; continue;
 }
 0x49de2f => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x49de50; continue; }
  pc = 0x49de33; continue;
 }
 0x49de33 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x2u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { pc = 0x49de54; continue; }
  pc = 0x49de3b; continue;
 }
 0x49de3b => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x49de50; continue; }
  pc = 0x49de3f; continue;
 }
 0x49de3f => {
  { cpu.sub8(((cpu.eax >> 8) as u8), cpu.r8(0x3u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { pc = 0x49de54; continue; }
  pc = 0x49de44; continue;
 }
 0x49de44 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { cpu.poll(); pc = 0x49de20; continue; }
  pc = 0x49de4e; continue;
 }
 0x49de4e => {
  { let v: u32 = cpu.eax; cpu.eax = v; }
  pc = 0x49de50; continue;
 }
 0x49de50 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x49de54 => {
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x49de5c => {
  { cpu.and32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x49de78; continue; }
  pc = 0x49de64; continue;
 }
 0x49de64 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { cpu.poll(); pc = 0x49de54; continue; }
  pc = 0x49de6b; continue;
 }
 0x49de6b => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x49de50; continue; }
  pc = 0x49de70; continue;
 }
 0x49de70 => {
  { cpu.and32(cpu.edx, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x49de20; continue; }
  pc = 0x49de78; continue;
 }
 0x49de78 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { cpu.poll(); pc = 0x49de54; continue; }
  pc = 0x49de82; continue;
 }
 0x49de82 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { cpu.poll(); pc = 0x49de50; continue; }
  pc = 0x49de86; continue;
 }
 0x49de86 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), cpu.r8(0x1u32.wrapping_add(cpu.ecx))); }
  if !cpu.zf { cpu.poll(); pc = 0x49de54; continue; }
  pc = 0x49de8b; continue;
 }
 0x49de8b => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { cpu.poll(); pc = 0x49de50; continue; }
  pc = 0x49de8f; continue;
 }
 0x49de8f => {
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  cpu.poll(); pc = 0x49de20; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049dea0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x49dea0); } r_0049dea0(cpu, 0x49dea0); }
pub fn f_0049e000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x49e000); } r_0049dea0(cpu, 0x49e000); }
fn r_0049dea0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x49dea0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49deca; continue; }
  pc = 0x49debd; continue;
 }
 0x49debd => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49deca; continue; }
  pc = 0x49dec3; continue;
 }
 0x49dec3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49e179; continue;
 }
 0x49deca => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49deee; continue; }
  pc = 0x49ded0; continue;
 }
 0x49ded0 => {
  { let v = 0x4a27d4u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x7eu32; cpu.push(v); }
  { let v = 0x4a27c8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x49dee5); f_00490280(cpu); if cpu.returned_to != 0x49dee5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x49deee; continue; }
  pc = 0x49deed; continue;
 }
 0x49deed => {
  cpu.unimplemented(0x49deed, "Int3");
  pc = 0x49deee; continue;
 }
 0x49deee => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x49deca; continue; }
  pc = 0x49def4; continue;
 }
 0x49def4 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49e11c; continue; }
  pc = 0x49defe; continue;
 }
 0x49defe => {
  { cpu.sub32(cpu.r32(0x5dee90u32), 0x0u32); }
  if !cpu.zf { pc = 0x49df6d; continue; }
  pc = 0x49df07; continue;
 }
 0x49df07 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x49df65; continue; }
  pc = 0x49df0f; continue;
 }
 0x49df0f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0xffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49df30; continue; }
  pc = 0x49df1e; continue;
 }
 0x49df1e => {
  { let v: u32 = 0x2au32; cpu.w32(0x5de9bcu32, v); }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49e179; continue;
 }
 0x49df30 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x49df5a; continue; }
  pc = 0x49df52; continue;
 }
 0x49df52 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49e179; continue;
 }
 0x49df5a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49df07; continue;
 }
 0x49df65 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49e179; continue;
 }
 0x49df6d => {
  { cpu.sub32(cpu.r32(0x5dd06cu32), 0x1u32); }
  if !cpu.zf { pc = 0x49dffa; continue; }
  pc = 0x49df7a; continue;
 }
 0x49df7a => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.cf || cpu.zf) { pc = 0x49df93; continue; }
  pc = 0x49df80; continue;
 }
 0x49df80 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49df8d); f_0049e180(cpu); if cpu.returned_to != 0x49df8d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49df93; continue;
 }
 0x49df93 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x220u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5deea0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10c4u32); cpu.push(0x49dfba); cpu.call(t); if cpu.returned_to != 0x49dfba { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49dfe8; continue; }
  pc = 0x49dfc3; continue;
 }
 0x49dfc3 => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49dfe8; continue; }
  pc = 0x49dfc9; continue;
 }
 0x49dfc9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0xffffffffu32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x49dfe0; continue; }
  pc = 0x49dfd7; continue;
 }
 0x49dfd7 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49dfe0; continue;
 }
 0x49dfe0 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49e179; continue;
 }
 0x49dfe8 => {
  { let v: u32 = 0x2au32; cpu.w32(0x5de9bcu32, v); }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49e179; continue;
 }
 0x49dffa => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x49e000; continue;
 }
 0x49e000 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x220u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5deea0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a10c4u32); cpu.push(0x49e020); cpu.call(t); if cpu.returned_to != 0x49e020 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49e03a; continue; }
  pc = 0x49e029; continue;
 }
 0x49e029 => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49e03a; continue; }
  pc = 0x49e02f; continue;
 }
 0x49e02f => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x1u32); cpu.eax = v; }
  pc = 0x49e179; continue;
 }
 0x49e03a => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49e04b; continue; }
  pc = 0x49e040; continue;
 }
 0x49e040 => {
  let t = cpu.r32(0x4a1170u32); cpu.push(0x49e046); cpu.call(t); if cpu.returned_to != 0x49e046 { return; }
  { cpu.sub32(cpu.eax, 0x7au32); }
  if cpu.zf { pc = 0x49e05d; continue; }
  pc = 0x49e04b; continue;
 }
 0x49e04b => {
  { let v: u32 = 0x2au32; cpu.w32(0x5de9bcu32, v); }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49e179; continue;
 }
 0x49e05d => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x49e117; continue; }
  pc = 0x49e069; continue;
 }
 0x49e069 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dd06cu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5deea0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x4a10c4u32); cpu.push(0x49e08f); cpu.call(t); if cpu.returned_to != 0x49e08f { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49e09e; continue; }
  pc = 0x49e098; continue;
 }
 0x49e098 => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49e0b0; continue; }
  pc = 0x49e09e; continue;
 }
 0x49e09e => {
  { let v: u32 = 0x2au32; cpu.w32(0x5de9bcu32, v); }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49e179; continue;
 }
 0x49e0b0 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x49e0c3; continue; }
  pc = 0x49e0bb; continue;
 }
 0x49e0bb => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49e179; continue;
 }
 0x49e0c3 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e0de; continue;
 }
 0x49e0cc => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e0de; continue;
 }
 0x49e0de => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x49e109; continue; }
  pc = 0x49e0e6; continue;
 }
 0x49e0e6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0xfffffff4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x49e107; continue; }
  pc = 0x49e102; continue;
 }
 0x49e102 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49e179; continue;
 }
 0x49e107 => {
  cpu.poll(); pc = 0x49e0cc; continue;
 }
 0x49e109 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49e05d; continue;
 }
 0x49e117 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49e179; continue;
 }
 0x49e11c => {
  { cpu.sub32(cpu.r32(0x5dee90u32), 0x0u32); }
  if !cpu.zf { pc = 0x49e133; continue; }
  pc = 0x49e125; continue;
 }
 0x49e125 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49e12e); f_0049b620(cpu); if cpu.returned_to != 0x49e12e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x49e179; continue;
 }
 0x49e133 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x220u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5deea0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a10c4u32); cpu.push(0x49e155); cpu.call(t); if cpu.returned_to != 0x49e155 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49e164; continue; }
  pc = 0x49e15e; continue;
 }
 0x49e15e => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49e173; continue; }
  pc = 0x49e164; continue;
 }
 0x49e164 => {
  { let v: u32 = 0x2au32; cpu.w32(0x5de9bcu32, v); }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49e179; continue;
 }
 0x49e173 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x1u32); cpu.eax = v; }
  pc = 0x49e179; continue;
 }
 0x49e179 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e180(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e180); }
 let mut pc: u32 = 0x49e180;
 loop { match pc {
 0x49e180 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e195; continue;
 }
 0x49e195 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49e1bb; continue; }
  pc = 0x49e1a4; continue;
 }
 0x49e1a4 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49e1bb; continue; }
  pc = 0x49e1b0; continue;
 }
 0x49e1b0 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49e195; continue;
 }
 0x49e1bb => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49e1da; continue; }
  pc = 0x49e1c1; continue;
 }
 0x49e1c1 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x49e1da; continue; }
  pc = 0x49e1cd; continue;
 }
 0x49e1cd => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x1u32); cpu.eax = v; }
  pc = 0x49e1dd; continue;
 }
 0x49e1da => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49e1dd; continue;
 }
 0x49e1dd => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e1f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e1f0); }
 let mut pc: u32 = 0x49e1f0;
 loop { match pc {
 0x49e1f0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x5de9e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49e226; continue; }
  pc = 0x49e204; continue;
 }
 0x49e204 => {
  { cpu.sub32(cpu.r32(0x5de9ecu32), 0x0u32); }
  if cpu.zf { pc = 0x49e226; continue; }
  pc = 0x49e20d; continue;
 }
 0x49e20d => {
  cpu.push(0x49e212); f_0049e960(cpu); if cpu.returned_to != 0x49e212 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49e21d; continue; }
  pc = 0x49e216; continue;
 }
 0x49e216 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49e2a2; continue;
 }
 0x49e21d => {
  { let v: u32 = cpu.r32(0x5de9e4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e226; continue;
 }
 0x49e226 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49e2a0; continue; }
  pc = 0x49e22c; continue;
 }
 0x49e22c => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49e2a0; continue; }
  pc = 0x49e232; continue;
 }
 0x49e232 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49e23b); f_004936f0(cpu); if cpu.returned_to != 0x49e23b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e241; continue;
 }
 0x49e241 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x49e2a0; continue; }
  pc = 0x49e249; continue;
 }
 0x49e249 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49e254); f_004936f0(cpu); if cpu.returned_to != 0x49e254 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x49e295; continue; }
  pc = 0x49e25c; continue;
 }
 0x49e25c => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3du32); }
  if !cpu.zf { pc = 0x49e295; continue; }
  pc = 0x49e26d; continue;
 }
 0x49e26d => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49e280); f_0049e910(cpu); if cpu.returned_to != 0x49e280 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49e295; continue; }
  pc = 0x49e287; continue;
 }
 0x49e287 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x49e2a2; continue;
 }
 0x49e295 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49e241; continue;
 }
 0x49e2a0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49e2a2; continue;
 }
 0x49e2a2 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e2b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e2b0); }
 let mut pc: u32 = 0x49e2b0;
 loop { match pc {
 0x49e2b0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5f2060u32)); }
  if !cpu.cf { pc = 0x49e2de; continue; }
  pc = 0x49e2bf; continue;
 }
 0x49e2bf => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 5u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5f1f60u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(8))) as i8 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x49e2f0; continue; }
  pc = 0x49e2de; continue;
 }
 0x49e2de => {
  { let v: u32 = 0x9u32; cpu.w32(0x5de9bcu32, v); }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49e3a8; continue;
 }
 0x49e2f0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5f1f60u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(8))) as i8 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x80u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x8000u32); }
  if !cpu.zf { pc = 0x49e34c; continue; }
  pc = 0x49e31a; continue;
 }
 0x49e31a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5f1f60u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 5u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5f1f60u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(8)), v); }
  pc = 0x49e397; continue;
 }
 0x49e34c => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x4000u32); }
  if !cpu.zf { pc = 0x49e388; continue; }
  pc = 0x49e355; continue;
 }
 0x49e355 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5f1f60u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(8))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) | 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1fu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5f1f60u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(8)), v); }
  pc = 0x49e397; continue;
 }
 0x49e388 => {
  { let v: u32 = 0x16u32; cpu.w32(0x5de9bcu32, v); }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49e3a8; continue;
 }
 0x49e397 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffc000u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x8000u32); cpu.eax = v; }
  pc = 0x49e3a8; continue;
 }
 0x49e3a8 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e3b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e3b0); }
 let mut pc: u32 = 0x49e3b0;
 loop { match pc {
 0x49e3b0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4cu32); cpu.esp = v; }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x8000u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffffd0u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7fffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x7fffu32); }
  if (cpu.sf == cpu.of) { pc = 0x49e473; continue; }
  pc = 0x49e453; continue;
 }
 0x49e453 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (cpu.sf == cpu.of) { pc = 0x49e473; continue; }
  pc = 0x49e464; continue;
 }
 0x49e464 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xbffdu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49e4aa; continue; }
  pc = 0x49e473; continue;
 }
 0x49e473 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x80000000u32; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, 0x7fff8000u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49e7ef; continue;
 }
 0x49e4aa => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x3fbfu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49e4dd; continue; }
  pc = 0x49e4bb; continue;
 }
 0x49e4bb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x49e7ef; continue;
 }
 0x49e4dd => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49e523; continue; }
  pc = 0x49e4e9; continue;
 }
 0x49e4e9 => {
  { let v: u16 = cpu.r16(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x1u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffffffu32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49e523; continue; }
  pc = 0x49e504; continue;
 }
 0x49e504 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { pc = 0x49e523; continue; }
  pc = 0x49e50d; continue;
 }
 0x49e50d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x49e523; continue; }
  pc = 0x49e515; continue;
 }
 0x49e515 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  pc = 0x49e7ef; continue;
 }
 0x49e523 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x49e57f; continue; }
  pc = 0x49e530; continue;
 }
 0x49e530 => {
  { let v: u16 = cpu.r16(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x1u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x7fffffffu32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x49e57f; continue; }
  pc = 0x49e54c; continue;
 }
 0x49e54c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x49e57f; continue; }
  pc = 0x49e555; continue;
 }
 0x49e555 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x49e57f; continue; }
  pc = 0x49e55d; continue;
 }
 0x49e55d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x49e7ef; continue;
 }
 0x49e57f => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e598; continue;
 }
 0x49e58f => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e598; continue;
 }
 0x49e598 => {
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x5u32); }
  if (cpu.sf == cpu.of) { pc = 0x49e657; continue; }
  pc = 0x49e5a2; continue;
 }
 0x49e5a2 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x8u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x5u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e5c7; continue;
 }
 0x49e5be => {
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e5c7; continue;
 }
 0x49e5c7 => {
  { cpu.sub32(cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49e649; continue; }
  pc = 0x49e5cd; continue;
 }
 0x49e5cd => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffc4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49e612); f_0049bbe0(cpu); if cpu.returned_to != 0x49e612 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49e632; continue; }
  pc = 0x49e61e; continue;
 }
 0x49e61e => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0xffffffecu32.wrapping_add(cpu.ebp).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffecu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  pc = 0x49e632; continue;
 }
 0x49e632 => {
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49e5be; continue;
 }
 0x49e649 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49e58f; continue;
 }
 0x49e657 => {
  { let v: u16 = cpu.r16(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0x3ffeu16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e664; continue;
 }
 0x49e664 => {
  { let v: u32 = cpu.r16(0xffffffccu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49e693; continue; }
  pc = 0x49e66c; continue;
 }
 0x49e66c => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x80000000u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x49e693; continue; }
  pc = 0x49e679; continue;
 }
 0x49e679 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49e682); f_0049bcd0(cpu); if cpu.returned_to != 0x49e682 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = cpu.r16(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49e664; continue;
 }
 0x49e693 => {
  { let v: u32 = cpu.r16(0xffffffccu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49e6f3; continue; }
  pc = 0x49e69b; continue;
 }
 0x49e69b => {
  { let v: u16 = cpu.r16(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_sub(0x1u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e6a7; continue;
 }
 0x49e6a7 => {
  { let v: u32 = cpu.r16(0xffffffccu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x49e6e2; continue; }
  pc = 0x49e6af; continue;
 }
 0x49e6af => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49e6c8; continue; }
  pc = 0x49e6bf; continue;
 }
 0x49e6bf => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e6c8; continue;
 }
 0x49e6c8 => {
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49e6d1); f_0049bd30(cpu); if cpu.returned_to != 0x49e6d1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = cpu.r16(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x1u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49e6a7; continue;
 }
 0x49e6e2 => {
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49e6f3; continue; }
  pc = 0x49e6e8; continue;
 }
 0x49e6e8 => {
  { let v: u16 = cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8) | 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e6f3; continue;
 }
 0x49e6f3 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8000u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49e713; continue; }
  pc = 0x49e702; continue;
 }
 0x49e702 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1ffffu32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x18000u32); }
  if !cpu.zf { pc = 0x49e774; continue; }
  pc = 0x49e713; continue;
 }
 0x49e713 => {
  { cpu.sub32(cpu.r32(0xffffffeau32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x49e76b; continue; }
  pc = 0x49e719; continue;
 }
 0x49e719 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffeau32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffeeu32.wrapping_add(cpu.ebp)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x49e760; continue; }
  pc = 0x49e726; continue;
 }
 0x49e726 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffeeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff2u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffu32); }
  if !cpu.zf { pc = 0x49e752; continue; }
  pc = 0x49e73e; continue;
 }
 0x49e73e => {
  { let v: u16 = 0x8000u16; cpu.w16(0xfffffff2u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x1u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffccu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e75e; continue;
 }
 0x49e752 => {
  { let v: u16 = cpu.r16(0xfffffff2u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x1u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xfffffff2u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e75e; continue;
 }
 0x49e75e => {
  pc = 0x49e769; continue;
 }
 0x49e760 => {
  { let v: u32 = cpu.r32(0xffffffeeu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffeeu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e769; continue;
 }
 0x49e769 => {
  pc = 0x49e774; continue;
 }
 0x49e76b => {
  { let v: u32 = cpu.r32(0xffffffeau32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffeau32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e774; continue;
 }
 0x49e774 => {
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x7fffu32); }
  if (cpu.sf != cpu.of) { pc = 0x49e7b9; continue; }
  pc = 0x49e785; continue;
 }
 0x49e785 => {
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x80000000u32; cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x7fff8000u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x49e7ef; continue;
 }
 0x49e7b9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0xffffffeau32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x6u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xffffffccu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edx), v); }
  pc = 0x49e7ef; continue;
 }
 0x49e7ef => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e800(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e800); }
 let mut pc: u32 = 0x49e800;
 loop { match pc {
 0x49e800 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = 0x5dd7d0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x60u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49e81c; continue; }
  pc = 0x49e817; continue;
 }
 0x49e817 => {
  pc = 0x49e8bf; continue;
 }
 0x49e81c => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49e835; continue; }
  pc = 0x49e822; continue;
 }
 0x49e822 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x5dd930u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x60u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e835; continue;
 }
 0x49e835 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49e843; continue; }
  pc = 0x49e83b; continue;
 }
 0x49e83b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x49e843; continue;
 }
 0x49e843 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49e8bf; continue; }
  pc = 0x49e849; continue;
 }
 0x49e849 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x54u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49e86c; continue; }
  pc = 0x49e86a; continue;
 }
 0x49e86a => {
  cpu.poll(); pc = 0x49e843; continue;
 }
 0x49e86c => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, 0xcu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x49e8ad; continue; }
  pc = 0x49e88a; continue;
 }
 0x49e88a => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffeeu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffeeu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e8ad; continue;
 }
 0x49e8ad => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49e8ba); f_0049e3b0(cpu); if cpu.returned_to != 0x49e8ba { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  cpu.poll(); pc = 0x49e843; continue;
 }
 0x49e8bf => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e910); }
 let mut pc: u32 = 0x49e910;
 loop { match pc {
 0x49e910 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49e91e; continue; }
  pc = 0x49e91a; continue;
 }
 0x49e91a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49e95b; continue;
 }
 0x49e91e => {
  { let v: u32 = cpu.r32(0x5ded70u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5ded74u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49e942); f_0049ea10(cpu); if cpu.returned_to != 0x49e942 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49e955; continue; }
  pc = 0x49e94e; continue;
 }
 0x49e94e => {
  { let v: u32 = 0x7fffffffu32; cpu.eax = v; }
  pc = 0x49e95b; continue;
 }
 0x49e955 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x2u32); cpu.eax = v; }
  pc = 0x49e95b; continue;
 }
 0x49e95b => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049e960(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49e960); }
 let mut pc: u32 = 0x49e960;
 loop { match pc {
 0x49e960 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x5de9ecu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49e96e; continue;
 }
 0x49e96e => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if cpu.zf { pc = 0x49ea08; continue; }
  pc = 0x49e97a; continue;
 }
 0x49e97a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  let t = cpu.r32(0x4a10c4u32); cpu.push(0x49e994); cpu.call(t); if cpu.returned_to != 0x49e994 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49e9a2; continue; }
  pc = 0x49e99d; continue;
 }
 0x49e99d => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49ea0a; continue;
 }
 0x49e9a2 => {
  { let v = 0x3du32; cpu.push(v); }
  { let v = 0x4a27e4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49e9b4); f_0048e380(cpu); if cpu.returned_to != 0x49e9b4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49e9c5; continue; }
  pc = 0x49e9c0; continue;
 }
 0x49e9c0 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49ea0a; continue;
 }
 0x49e9c5 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  let t = cpu.r32(0x4a10c4u32); cpu.push(0x49e9e3); cpu.call(t); if cpu.returned_to != 0x49e9e3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49e9ec; continue; }
  pc = 0x49e9e7; continue;
 }
 0x49e9e7 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49ea0a; continue;
 }
 0x49e9ec => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49e9f7); f_0049edd0(cpu); if cpu.returned_to != 0x49e9f7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49e96e; continue;
 }
 0x49ea08 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49ea0a; continue;
 }
 0x49ea0a => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049ea10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49ea10); }
 let mut pc: u32 = 0x49ea10;
 loop { match pc {
 0x49ea10 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x5deed8u32), 0x0u32); }
  if !cpu.zf { pc = 0x49ea79; continue; }
  pc = 0x49ea22; continue;
 }
 0x49ea22 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4a26b4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4a26b4u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x4a10e8u32); cpu.push(0x49ea3a); cpu.call(t); if cpu.returned_to != 0x49ea3a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49ea4a; continue; }
  pc = 0x49ea3e; continue;
 }
 0x49ea3e => {
  { let v: u32 = 0x1u32; cpu.w32(0x5deed8u32, v); }
  pc = 0x49ea79; continue;
 }
 0x49ea4a => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4a26b0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x4a26b0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x4a10e4u32); cpu.push(0x49ea62); cpu.call(t); if cpu.returned_to != 0x49ea62 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49ea72; continue; }
  pc = 0x49ea66; continue;
 }
 0x49ea66 => {
  { let v: u32 = 0x2u32; cpu.w32(0x5deed8u32, v); }
  pc = 0x49ea79; continue;
 }
 0x49ea72 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49edc3; continue;
 }
 0x49ea79 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49ea92; continue; }
  pc = 0x49ea7f; continue;
 }
 0x49ea7f => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49ea8c); f_0049b920(cpu); if cpu.returned_to != 0x49ea8c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ea92; continue;
 }
 0x49ea92 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49eaab; continue; }
  pc = 0x49ea98; continue;
 }
 0x49ea98 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49eaa5); f_0049b920(cpu); if cpu.returned_to != 0x49eaa5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49eaab; continue;
 }
 0x49eaab => {
  { cpu.sub32(cpu.r32(0x5deed8u32), 0x2u32); }
  if !cpu.zf { pc = 0x49ead7; continue; }
  pc = 0x49eab4; continue;
 }
 0x49eab4 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10e4u32); cpu.push(0x49ead2); cpu.call(t); if cpu.returned_to != 0x49ead2 { return; }
  pc = 0x49edc3; continue;
 }
 0x49ead7 => {
  { cpu.sub32(cpu.r32(0x5deed8u32), 0x1u32); }
  if !cpu.zf { pc = 0x49edc3; continue; }
  pc = 0x49eae4; continue;
 }
 0x49eae4 => {
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49eb16; continue; }
  pc = 0x49eb0d; continue;
 }
 0x49eb0d => {
  { let v: u32 = cpu.r32(0x5deea0u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49eb16; continue;
 }
 0x49eb16 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49eb26; continue; }
  pc = 0x49eb1c; continue;
 }
 0x49eb1c => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49eca2; continue; }
  pc = 0x49eb26; continue;
 }
 0x49eb26 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x49eb38; continue; }
  pc = 0x49eb2e; continue;
 }
 0x49eb2e => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x49edc3; continue;
 }
 0x49eb38 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49eb48; continue; }
  pc = 0x49eb3e; continue;
 }
 0x49eb3e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49edc3; continue;
 }
 0x49eb48 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49eb58; continue; }
  pc = 0x49eb4e; continue;
 }
 0x49eb4e => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x49edc3; continue;
 }
 0x49eb58 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x4a1078u32); cpu.push(0x49eb66); cpu.call(t); if cpu.returned_to != 0x49eb66 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49eb71; continue; }
  pc = 0x49eb6a; continue;
 }
 0x49eb6a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49edc3; continue;
 }
 0x49eb71 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49eb7d; continue; }
  pc = 0x49eb77; continue;
 }
 0x49eb77 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x49ebaa; continue; }
  pc = 0x49eb7d; continue;
 }
 0x49eb7d => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if !cpu.zf { pc = 0x49eb89; continue; }
  pc = 0x49eb83; continue;
 }
 0x49eb83 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49ebaa; continue; }
  pc = 0x49eb89; continue;
 }
 0x49eb89 => {
  { let v = 0x4a27fcu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x163u32; cpu.push(v); }
  { let v = 0x4a27f0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x49eba1); f_00490280(cpu); if cpu.returned_to != 0x49eba1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x49ebaa; continue; }
  pc = 0x49eba9; continue;
 }
 0x49eba9 => {
  cpu.unimplemented(0x49eba9, "Int3");
  pc = 0x49ebaa; continue;
 }
 0x49ebaa => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x49eb71; continue; }
  pc = 0x49ebb0; continue;
 }
 0x49ebb0 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49ec29; continue; }
  pc = 0x49ebb6; continue;
 }
 0x49ebb6 => {
  { cpu.sub32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if !cpu.cf { pc = 0x49ebc6; continue; }
  pc = 0x49ebbc; continue;
 }
 0x49ebbc => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x49edc3; continue;
 }
 0x49ebc6 => {
  { let v: u32 = 0xffffffdeu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ebd7; continue;
 }
 0x49ebce => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ebd7; continue;
 }
 0x49ebd7 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49ec1f; continue; }
  pc = 0x49ebe2; continue;
 }
 0x49ebe2 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49ec1f; continue; }
  pc = 0x49ebee; continue;
 }
 0x49ebee => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x49ec1d; continue; }
  pc = 0x49ec00; continue;
 }
 0x49ec00 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49ec1d; continue; }
  pc = 0x49ec13; continue;
 }
 0x49ec13 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x49edc3; continue;
 }
 0x49ec1d => {
  cpu.poll(); pc = 0x49ebce; continue;
 }
 0x49ec1f => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x49edc3; continue;
 }
 0x49ec29 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x49eca2; continue; }
  pc = 0x49ec2f; continue;
 }
 0x49ec2f => {
  { cpu.sub32(cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if !cpu.cf { pc = 0x49ec3f; continue; }
  pc = 0x49ec35; continue;
 }
 0x49ec35 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49edc3; continue;
 }
 0x49ec3f => {
  { let v: u32 = 0xffffffdeu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ec50; continue;
 }
 0x49ec47 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ec50; continue;
 }
 0x49ec50 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49ec98; continue; }
  pc = 0x49ec5b; continue;
 }
 0x49ec5b => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49ec98; continue; }
  pc = 0x49ec67; continue;
 }
 0x49ec67 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x49ec96; continue; }
  pc = 0x49ec79; continue;
 }
 0x49ec79 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49ec96; continue; }
  pc = 0x49ec8c; continue;
 }
 0x49ec8c => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x49edc3; continue;
 }
 0x49ec96 => {
  cpu.poll(); pc = 0x49ec47; continue;
 }
 0x49ec98 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x49edc3; continue;
 }
 0x49eca2 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10c8u32); cpu.push(0x49ecba); cpu.call(t); if cpu.returned_to != 0x49ecba { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49ecca; continue; }
  pc = 0x49ecc3; continue;
 }
 0x49ecc3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49edc3; continue;
 }
 0x49ecca => {
  { let v = 0x18bu32; cpu.push(v); }
  { let v = 0x4a27f0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49ece1); f_0048e380(cpu); if cpu.returned_to != 0x49ece1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49ecf4; continue; }
  pc = 0x49eced; continue;
 }
 0x49eced => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49edc3; continue;
 }
 0x49ecf4 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10c8u32); cpu.push(0x49ed10); cpu.call(t); if cpu.returned_to != 0x49ed10 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49ed19; continue; }
  pc = 0x49ed14; continue;
 }
 0x49ed14 => {
  pc = 0x49eda4; continue;
 }
 0x49ed19 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10c8u32); cpu.push(0x49ed31); cpu.call(t); if cpu.returned_to != 0x49ed31 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49ed3c; continue; }
  pc = 0x49ed3a; continue;
 }
 0x49ed3a => {
  pc = 0x49eda4; continue;
 }
 0x49ed3c => {
  { let v = 0x19au32; cpu.push(v); }
  { let v = 0x4a27f0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49ed53); f_0048e380(cpu); if cpu.returned_to != 0x49ed53 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49ed61; continue; }
  pc = 0x49ed5f; continue;
 }
 0x49ed5f => {
  pc = 0x49eda4; continue;
 }
 0x49ed61 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10c8u32); cpu.push(0x49ed7d); cpu.call(t); if cpu.returned_to != 0x49ed7d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49ed83; continue; }
  pc = 0x49ed81; continue;
 }
 0x49ed81 => {
  pc = 0x49eda4; continue;
 }
 0x49ed83 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10e8u32); cpu.push(0x49eda1); cpu.call(t); if cpu.returned_to != 0x49eda1 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49eda4; continue;
 }
 0x49eda4 => {
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49edaf); f_0048ecf0(cpu); if cpu.returned_to != 0x49edaf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49edbd); f_0048ecf0(cpu); if cpu.returned_to != 0x49edbd { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49edc3; continue;
 }
 0x49edc3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049edd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49edd0); }
 let mut pc: u32 = 0x49edd0;
 loop { match pc {
 0x49edd0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49edfc; continue; }
  pc = 0x49eddd; continue;
 }
 0x49eddd => {
  { let v = 0x3du32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49ede8); f_0049f220(cpu); if cpu.returned_to != 0x49ede8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49edfc; continue; }
  pc = 0x49edf4; continue;
 }
 0x49edf4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x49ee04; continue; }
  pc = 0x49edfc; continue;
 }
 0x49edfc => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49f0ab; continue;
 }
 0x49ee04 => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.edx)) as i8 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x5de9e4u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x5de9e8u32)); }
  if !cpu.zf { pc = 0x49ee35; continue; }
  pc = 0x49ee21; continue;
 }
 0x49ee21 => {
  { let v: u32 = cpu.r32(0x5de9e4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49ee2d); f_0049f130(cpu); if cpu.returned_to != 0x49ee2d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de9e4u32, v); }
  pc = 0x49ee35; continue;
 }
 0x49ee35 => {
  { cpu.sub32(cpu.r32(0x5de9e4u32), 0x0u32); }
  if !cpu.zf { pc = 0x49eef5; continue; }
  pc = 0x49ee42; continue;
 }
 0x49ee42 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49ee67; continue; }
  pc = 0x49ee48; continue;
 }
 0x49ee48 => {
  { cpu.sub32(cpu.r32(0x5de9ecu32), 0x0u32); }
  if cpu.zf { pc = 0x49ee67; continue; }
  pc = 0x49ee51; continue;
 }
 0x49ee51 => {
  cpu.push(0x49ee56); f_0049e960(cpu); if cpu.returned_to != 0x49ee56 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49ee62; continue; }
  pc = 0x49ee5a; continue;
 }
 0x49ee5a => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49f0ab; continue;
 }
 0x49ee62 => {
  pc = 0x49eef5; continue;
 }
 0x49ee67 => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49ee74; continue; }
  pc = 0x49ee6d; continue;
 }
 0x49ee6d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49f0ab; continue;
 }
 0x49ee74 => {
  { cpu.sub32(cpu.r32(0x5de9e4u32), 0x0u32); }
  if !cpu.zf { pc = 0x49eeb4; continue; }
  pc = 0x49ee7d; continue;
 }
 0x49ee7d => {
  { let v = 0x87u32; cpu.push(v); }
  { let v = 0x4a283cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x49ee90); f_0048e380(cpu); if cpu.returned_to != 0x49ee90 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de9e4u32, v); }
  { cpu.sub32(cpu.r32(0x5de9e4u32), 0x0u32); }
  if !cpu.zf { pc = 0x49eea9; continue; }
  pc = 0x49eea1; continue;
 }
 0x49eea1 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49f0ab; continue;
 }
 0x49eea9 => {
  { let v: u32 = cpu.r32(0x5de9e4u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x49eeb4; continue;
 }
 0x49eeb4 => {
  { cpu.sub32(cpu.r32(0x5de9ecu32), 0x0u32); }
  if !cpu.zf { pc = 0x49eef5; continue; }
  pc = 0x49eebd; continue;
 }
 0x49eebd => {
  { let v = 0x8eu32; cpu.push(v); }
  { let v = 0x4a283cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x49eed0); f_0048e380(cpu); if cpu.returned_to != 0x49eed0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de9ecu32, v); }
  { cpu.sub32(cpu.r32(0x5de9ecu32), 0x0u32); }
  if !cpu.zf { pc = 0x49eee9; continue; }
  pc = 0x49eee1; continue;
 }
 0x49eee1 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49f0ab; continue;
 }
 0x49eee9 => {
  { let v: u32 = cpu.r32(0x5de9ecu32); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49eef5; continue;
 }
 0x49eef5 => {
  { let v: u32 = cpu.r32(0x5de9e4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49ef0e); f_0049f0b0(cpu); if cpu.returned_to != 0x49ef0e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x49efb1; continue; }
  pc = 0x49ef1e; continue;
 }
 0x49ef1e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if cpu.zf { pc = 0x49efb1; continue; }
  pc = 0x49ef2a; continue;
 }
 0x49ef2a => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49efa3; continue; }
  pc = 0x49ef30; continue;
 }
 0x49ef30 => {
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49ef41); f_0048ecf0(cpu); if cpu.returned_to != 0x49ef41 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x49ef4f; continue;
 }
 0x49ef46 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49ef4f; continue;
 }
 0x49ef4f => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x49ef70; continue; }
  pc = 0x49ef5b; continue;
 }
 0x49ef5b => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  cpu.poll(); pc = 0x49ef46; continue;
 }
 0x49ef70 => {
  { let v = 0xb9u32; cpu.push(v); }
  { let v = 0x4a283cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49ef8c); f_0048e780(cpu); if cpu.returned_to != 0x49ef8c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49efa1; continue; }
  pc = 0x49ef98; continue;
 }
 0x49ef98 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5de9e4u32, v); }
  pc = 0x49efa1; continue;
 }
 0x49efa1 => {
  pc = 0x49efaf; continue;
 }
 0x49efa3 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  pc = 0x49efaf; continue;
 }
 0x49efaf => {
  pc = 0x49f024; continue;
 }
 0x49efb1 => {
  { cpu.sub32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49f01d; continue; }
  pc = 0x49efb7; continue;
 }
 0x49efb7 => {
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x49efc5; continue; }
  pc = 0x49efbd; continue;
 }
 0x49efbd => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49efc5; continue;
 }
 0x49efc5 => {
  { let v = 0xceu32; cpu.push(v); }
  { let v = 0x4a283cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49efe5); f_0048e780(cpu); if cpu.returned_to != 0x49efe5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49eff9; continue; }
  pc = 0x49eff1; continue;
 }
 0x49eff1 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x49f0ab; continue;
 }
 0x49eff9 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5de9e4u32, v); }
  pc = 0x49f024; continue;
 }
 0x49f01d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49f0ab; continue;
 }
 0x49f024 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49f0a9; continue; }
  pc = 0x49f02a; continue;
 }
 0x49f02a => {
  { let v = 0xe5u32; cpu.push(v); }
  { let v = 0x4a283cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49f03f); f_004936f0(cpu); if cpu.returned_to != 0x49f03f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49f04b); f_0048e380(cpu); if cpu.returned_to != 0x49f04b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x49f0a9; continue; }
  pc = 0x49f057; continue;
 }
 0x49f057 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49f064); f_00493770(cpu); if cpu.returned_to != 0x49f064 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx & cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x4a10ecu32); cpu.push(0x49f09b); cpu.call(t); if cpu.returned_to != 0x49f09b { return; }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49f0a6); f_0048ecf0(cpu); if cpu.returned_to != 0x49f0a6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x49f0a9; continue;
 }
 0x49f0a9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49f0ab; continue;
 }
 0x49f0ab => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f0b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f0b0); }
 let mut pc: u32 = 0x49f0b0;
 loop { match pc {
 0x49f0b0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5de9e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f0c7; continue;
 }
 0x49f0be => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f0c7; continue;
 }
 0x49f0c7 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if cpu.zf { pc = 0x49f11a; continue; }
  pc = 0x49f0cf; continue;
 }
 0x49f0cf => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49f0e2); f_0049e910(cpu); if cpu.returned_to != 0x49f0e2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49f118; continue; }
  pc = 0x49f0e9; continue;
 }
 0x49f0e9 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))) as i8 as i32 as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x3du32); }
  if cpu.zf { pc = 0x49f10a; continue; }
  pc = 0x49f0fa; continue;
 }
 0x49f0fa => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x49f118; continue; }
  pc = 0x49f10a; continue;
 }
 0x49f10a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5de9e4u32)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  pc = 0x49f128; continue;
 }
 0x49f118 => {
  cpu.poll(); pc = 0x49f0be; continue;
 }
 0x49f11a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5de9e4u32)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x49f128; continue;
 }
 0x49f128 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f130); }
 let mut pc: u32 = 0x49f130;
 loop { match pc {
 0x49f130 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49f150; continue; }
  pc = 0x49f149; continue;
 }
 0x49f149 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49f21a; continue;
 }
 0x49f150 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49f16d; continue; }
  pc = 0x49f162; continue;
 }
 0x49f162 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49f150; continue;
 }
 0x49f16d => {
  { let v = 0x146u32; cpu.push(v); }
  { let v = 0x4a283cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49f189); f_0048e380(cpu); if cpu.returned_to != 0x49f189 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49f1a5; continue; }
  pc = 0x49f19b; continue;
 }
 0x49f19b => {
  { let v = 0x9u32; cpu.push(v); }
  cpu.push(0x49f1a2); f_0048ff90(cpu); if cpu.returned_to != 0x49f1a2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x49f1a5; continue;
 }
 0x49f1a5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f1ab; continue;
 }
 0x49f1ab => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x49f20e; continue; }
  pc = 0x49f1b3; continue;
 }
 0x49f1b3 => {
  { let v = 0x14fu32; cpu.push(v); }
  { let v = 0x4a283cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49f1ca); f_004936f0(cpu); if cpu.returned_to != 0x49f1ca { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49f1d6); f_0048e380(cpu); if cpu.returned_to != 0x49f1d6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if cpu.zf { pc = 0x49f1fa; continue; }
  pc = 0x49f1e6; continue;
 }
 0x49f1e6 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49f1f7); f_00493770(cpu); if cpu.returned_to != 0x49f1f7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x49f1fa; continue;
 }
 0x49f1fa => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x49f1ab; continue;
 }
 0x49f20e => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49f21a; continue;
 }
 0x49f21a => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f220(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f220); }
 let mut pc: u32 = 0x49f220;
 loop { match pc {
 0x49f220 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x5ded70u32), 0x0u32); }
  if !cpu.zf { pc = 0x49f242; continue; }
  pc = 0x49f22d; continue;
 }
 0x49f22d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49f23a); f_0048e2a0(cpu); if cpu.returned_to != 0x49f23a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x49f2e1; continue;
 }
 0x49f242 => {
  pc = 0x49f24d; continue;
 }
 0x49f244 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f24d; continue;
 }
 0x49f24d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x49f2cd; continue; }
  pc = 0x49f265; continue;
 }
 0x49f265 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x5deb69u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x4u32; cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x49f2b8; continue; }
  pc = 0x49f27c; continue;
 }
 0x49f27c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x49f294; continue; }
  pc = 0x49f290; continue;
 }
 0x49f290 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49f2e1; continue;
 }
 0x49f294 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx | cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), cpu.edx); }
  if !cpu.zf { pc = 0x49f2b6; continue; }
  pc = 0x49f2ae; continue;
 }
 0x49f2ae => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, 0x1u32); cpu.eax = v; }
  pc = 0x49f2e1; continue;
 }
 0x49f2b6 => {
  pc = 0x49f2c8; continue;
 }
 0x49f2b8 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), cpu.edx); }
  if !cpu.zf { pc = 0x49f2c8; continue; }
  pc = 0x49f2c6; continue;
 }
 0x49f2c6 => {
  pc = 0x49f2cd; continue;
 }
 0x49f2c8 => {
  cpu.poll(); pc = 0x49f244; continue;
 }
 0x49f2cd => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), cpu.eax); }
  if !cpu.zf { pc = 0x49f2df; continue; }
  pc = 0x49f2da; continue;
 }
 0x49f2da => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49f2e1; continue;
 }
 0x49f2df => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x49f2e1; continue;
 }
 0x49f2e1 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f2f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f2f0); }
 let mut pc: u32 = 0x49f2f0;
 loop { match pc {
 0x49f2f0 => {
  cpu.poll(); pc = cpu.r32(0x4a1188u32); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0049f300(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x49f300); }
 let mut pc: u32 = 0x49f300;
 loop { match pc {
 0x49f300 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x5dee90u32), 0x0u32); }
  if !cpu.zf { pc = 0x49f35d; continue; }
  pc = 0x49f316; continue;
 }
 0x49f316 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f327; continue;
 }
 0x49f31e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x49f327; continue;
 }
 0x49f327 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) as i8 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x49f355; continue; }
  pc = 0x49f331; continue;
 }
 0x49f331 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as i8 as i32 as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x49f353; continue; }
  pc = 0x49f33c; continue;
 }
 0x49f33c => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as i8 as i32 as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x49f353; continue; }
  pc = 0x49f347; continue;
 }
 0x49f347 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x49f353; continue;
 }
 0x49f353 => {
  cpu.poll(); pc = 0x49f31e; continue;
 }
 0x49f355 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49f3f6; continue;
 }
 0x49f35d => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x100u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dee90u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x49f37b); f_0049b650(cpu); if cpu.returned_to != 0x49f37b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49f389; continue; }
  pc = 0x49f387; continue;
 }
 0x49f387 => {
  pc = 0x49f3e5; continue;
 }
 0x49f389 => {
  { let v = 0x64u32; cpu.push(v); }
  { let v = 0x4a2848u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49f39b); f_0048e380(cpu); if cpu.returned_to != 0x49f39b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x49f3a9; continue; }
  pc = 0x49f3a7; continue;
 }
 0x49f3a7 => {
  pc = 0x49f3e5; continue;
 }
 0x49f3a9 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x100u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dee90u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49f3cc); f_0049b650(cpu); if cpu.returned_to != 0x49f3cc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x20u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x49f3d5; continue; }
  pc = 0x49f3d3; continue;
 }
 0x49f3d3 => {
  pc = 0x49f3e5; continue;
 }
 0x49f3d5 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x49f3e2); f_00493770(cpu); if cpu.returned_to != 0x49f3e2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x49f3e5; continue;
 }
 0x49f3e5 => {
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x49f3f0); f_0048ecf0(cpu); if cpu.returned_to != 0x49f3f0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x49f3f6; continue;
 }
 0x49f3f6 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a0000); } r_004a0000(cpu, 0x4a0000); }
pub fn f_004a0001(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a0001); } r_004a0000(cpu, 0x4a0001); }
pub fn f_004a0003(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a0003); } r_004a0000(cpu, 0x4a0003); }
pub fn f_004a0004(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a0004); } r_004a0000(cpu, 0x4a0004); }
pub fn f_004a0010(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a0010); } r_004a0000(cpu, 0x4a0010); }
fn r_004a0000(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4a0000 => {
  { cpu.pushad(); }
  pc = 0x4a0001; continue;
 }
 0x4a0001 => {
  cpu.push(0x4a0006); f_00402f70(cpu); if cpu.returned_to != 0x4a0006 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5d31b4u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r8(0x5d31b4u32) as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4a0003 => {
  cpu.unimplemented(0x4a0003, "Das");
  pc = 0x4a0004; continue;
 }
 0x4a0004 => {
  { let v = ((cpu.ebx >> 8) as u8); cpu.idiv8(v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5d31b4u32, v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r8(0x5d31b4u32) as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x4a0010 => {
  { let v: u32 = cpu.xor32(cpu.r32(0x0u32.wrapping_add(cpu.ebp)), cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0012(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4a0012); }
 let mut pc: u32 = 0x4a0012;
 loop { match pc {
 0x4a0012 => {
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x5au32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a0020(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a0020); } r_004a0020(cpu, 0x4a0020); }
pub fn f_004a0026(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a0026); } r_004a0020(cpu, 0x4a0026); }
pub fn f_004a0036(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a0036); } r_004a0020(cpu, 0x4a0036); }
pub fn f_004a006e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a006e); } r_004a0020(cpu, 0x4a006e); }
pub fn f_004a007c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a007c); } r_004a0020(cpu, 0x4a007c); }
pub fn f_004a0080(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a0080); } r_004a0020(cpu, 0x4a0080); }
pub fn f_004a009e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a009e); } r_004a0020(cpu, 0x4a009e); }
fn r_004a0020(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4a0020 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a0026; continue;
 }
 0x4a0026 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a0036; continue;
 }
 0x4a0036 => {
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
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a006e; continue;
 }
 0x4a006e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a007c; continue;
 }
 0x4a007c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a0080; continue;
 }
 0x4a0080 => {
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
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a009e; continue;
 }
 0x4a009e => {
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
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.call(0x4a0200); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004a002d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a002d); } r_004a002d(cpu, 0x4a002d); }
pub fn f_004a004b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a004b); } r_004a002d(cpu, 0x4a004b); }
pub fn f_004a005b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a005b); } r_004a002d(cpu, 0x4a005b); }
pub fn f_004a0069(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a0069); } r_004a002d(cpu, 0x4a0069); }
pub fn f_004a0075(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a0075); } r_004a002d(cpu, 0x4a0075); }
pub fn f_004a0079(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a0079); } r_004a002d(cpu, 0x4a0079); }
pub fn f_004a007f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a007f); } r_004a002d(cpu, 0x4a007f); }
pub fn f_004a0091(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a0091); } r_004a002d(cpu, 0x4a0091); }
pub fn f_004a00b5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a00b5); } r_004a002d(cpu, 0x4a00b5); }
pub fn f_004a0119(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x4a0119); } r_004a002d(cpu, 0x4a0119); }
fn r_004a002d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x4a002d => {
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
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a004b; continue;
 }
 0x4a004b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a005b; continue;
 }
 0x4a005b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a0069; continue;
 }
 0x4a0069 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a0075; continue;
 }
 0x4a0075 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a0079; continue;
 }
 0x4a0079 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a007f; continue;
 }
 0x4a007f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a0091; continue;
 }
 0x4a0091 => {
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
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a00b5; continue;
 }
 0x4a00b5 => {
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
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x4a0119; continue;
 }
 0x4a0119 => {
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
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.call(0x4a0201); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004769d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4769d0); }
 let mut pc: u32 = 0x4769d0;
 loop { match pc {
 0x4769d0 => {
  { let v: u16 = cpu.r16(0x4d68eau32).wrapping_add((cpu.esi as u16)); cpu.w16(0x4d68eau32, v); }
  { let v: u16 = cpu.sub16((cpu.ebp as u16), (cpu.ecx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x4769e6; continue; }
  pc = 0x4769dc; continue;
 }
 0x4769dc => {
  { let v: u16 = cpu.r16(0x4d68e8u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d68e8u32, v); }
  { let v: u16 = cpu.add16((cpu.ebp as u16), (cpu.edx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  pc = 0x4769e6; continue;
 }
 0x4769e6 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4d6860u32)); cpu.w8(0x4d6860u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x4769d0; continue; }
  pc = 0x4769ee; continue;
 }
 0x4769ee => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_004769ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4769ef); }
 let mut pc: u32 = 0x4769ef;
 loop { match pc {
 0x4769ef => {
  { let v: u16 = cpu.r16(0x4d68e8u32).wrapping_add((cpu.eax as u16)); cpu.w16(0x4d68e8u32, v); }
  { let v: u16 = cpu.sub16((cpu.ebp as u16), (cpu.edx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x476a05; continue; }
  pc = 0x4769fb; continue;
 }
 0x4769fb => {
  { let v: u16 = (cpu.ebp as u16).wrapping_add((cpu.ecx as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16(cpu.r16(0x4d68eau32), (cpu.esi as u16)); cpu.w16(0x4d68eau32, v); }
  pc = 0x476a05; continue;
 }
 0x476a05 => {
  { let v: u8 = cpu.dec8(cpu.r8(0x4d6860u32)); cpu.w8(0x4d6860u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x4769ef; continue; }
  pc = 0x476a0d; continue;
 }
 0x476a0d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
